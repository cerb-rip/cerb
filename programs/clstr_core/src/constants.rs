use anchor_lang::prelude::*;

pub const MAX_RISK_SCORE: u64 = 10000;
pub const ZK_HASH_ROUNDS: u32 = 256;
pub const DISCRIMINATOR_SIZE: usize = 8;
pub const FLAG_SEED: &[u8] = b"flag";
pub const CONFIG_SEED: &[u8] = b"config";
pub const PROGRAM_VERSION: &str = "0.4.3";

pub fn risk_score_to_basis_points(score: u64) -> u64 {
    score.min(MAX_RISK_SCORE)
}

// 9bf31c7f
