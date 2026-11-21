use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RehearsalRequest {
    pub service: String,
    pub scenario: String,
    pub blast_radius: u8,
    pub recovery_minutes: u32,
    pub operator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageResult {
    pub stage: String,
    pub score: f64,
    pub message: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("value out of range: {0}")]
    Range(&'static str),
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("upstream unavailable: {0}")]
    Upstream(String),
    #[error("evidence integrity failure: {0}")]
    Integrity(String),
    #[error("storage failure: {0}")]
    Storage(String),
}

impl RehearsalRequest {
    pub fn validate_identity(&self) -> Result<(), DomainError> {
        if self.service.trim().is_empty() {
            return Err(DomainError::Missing("service"));
        }
        if self.scenario.trim().is_empty() {
            return Err(DomainError::Missing("scenario"));
        }
        if self.operator.trim().is_empty() {
            return Err(DomainError::Missing("operator"));
        }
        if self.blast_radius > 100 {
            return Err(DomainError::Range("blast_radius"));
        }
        if self.recovery_minutes == 0 || self.recovery_minutes > 1440 {
            return Err(DomainError::Range("recovery_minutes"));
        }
        Ok(())
    }

    pub fn stable_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.service.trim().to_lowercase(),
            self.scenario.trim().to_lowercase(),
            self.operator.trim().to_lowercase()
        )
    }
}

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, request: &RehearsalRequest) -> Result<StageResult, DomainError>;
}
