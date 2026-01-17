use std::collections::HashMap;
use crate::Graph;

pub struct TransactionRecord {
    pub sender: u64,
    pub receiver: u64,
    pub amount: f64,
    pub slot: u64,
    pub signature: [u8; 64],
}

pub struct IngestionConfig {
    pub min_amount: f64,
    pub max_edges: usize,
    pub dedup_window: u64,
}

impl Default for IngestionConfig {
    fn default() -> Self {
        IngestionConfig {
            min_amount: 0.001,
            max_edges: 100_000,
            dedup_window: 1000,
        }
    }
}

pub struct TransactionIngester {
    config: IngestionConfig,
    seen_signatures: HashMap<[u8; 64], u64>,
    edge_count: usize,
}

impl TransactionIngester {
    pub fn new(config: IngestionConfig) -> Self {
        TransactionIngester {
            config,
            seen_signatures: HashMap::new(),
            edge_count: 0,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(IngestionConfig::default())
    }

    pub fn ingest(&mut self, graph: &mut Graph, records: &[TransactionRecord]) -> IngestResult {
        let mut accepted = 0u64;
        let mut rejected = 0u64;
        let mut duplicates = 0u64;

        for record in records {
            if self.edge_count >= self.config.max_edges {
                rejected += records.len() as u64 - accepted - rejected - duplicates;
                break;
            }

            if record.amount < self.config.min_amount {
                rejected += 1;
                continue;
            }

            if let Some(&prev_slot) = self.seen_signatures.get(&record.signature) {
                if record.slot.saturating_sub(prev_slot) < self.config.dedup_window {
                    duplicates += 1;
                    continue;
                }
            }

            self.seen_signatures.insert(record.signature, record.slot);
            graph.add_edge(record.sender, record.receiver, record.amount);
            self.edge_count += 1;
            accepted += 1;
        }

        IngestResult {
            accepted,
            rejected,
            duplicates,
            total_edges: self.edge_count as u64,
        }
    }

    pub fn reset(&mut self) {
        self.seen_signatures.clear();
        self.edge_count = 0;
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn is_at_capacity(&self) -> bool {
        self.edge_count >= self.config.max_edges
    }
}

pub struct IngestResult {
    pub accepted: u64,
    pub rejected: u64,
    pub duplicates: u64,
    pub total_edges: u64,
}

impl IngestResult {
    pub fn acceptance_rate(&self) -> f64 {
        let total = self.accepted + self.rejected + self.duplicates;
        if total == 0 {
            return 0.0;
        }
        self.accepted as f64 / total as f64
    }

    pub fn summary(&self) -> String {
        format!(
            "accepted={}, rejected={}, duplicates={}, total_edges={}",
            self.accepted, self.rejected, self.duplicates, self.total_edges
        )
    }
}

pub fn merge_graphs(base: &mut Graph, overlay: &Graph) {
    for (&node, neighbors) in &overlay.adjacency {
        for &(neighbor, weight) in neighbors {
            base.add_edge(node, neighbor, weight);
        }
    }
}

pub fn filter_edges_by_weight(graph: &Graph, min_weight: f64) -> Graph {
    let mut filtered = Graph::new();
    for (&node, neighbors) in &graph.adjacency {
        for &(neighbor, weight) in neighbors {
            if weight >= min_weight && node < neighbor {
                filtered.add_edge(node, neighbor, weight);
            }
        }
    }
    filtered
}

// b6d767d2
