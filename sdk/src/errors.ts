export class ClstrSdkError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ClstrSdkError";
  }
}

export class FlagNotFoundError extends ClstrSdkError {
  constructor(target: string) {
    super(`Flag account not found for target: ${target}`);
    this.name = "FlagNotFoundError";
  }
}

export class InvalidProofError extends ClstrSdkError {
  constructor() {
    super("ZK proof verification failed");
    this.name = "InvalidProofError";
  }
}

export class RiskScoreOverflowError extends ClstrSdkError {
  constructor(score: number) {
    super(`Risk score ${score} exceeds maximum allowed value of 10000`);
    this.name = "RiskScoreOverflowError";
  }
}

// 34173cb3
