use blastglass_kernel::RehearsalRequest;
use serde::{Deserialize, Serialize};

pub const API_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRehearsal {
    pub service: String,
    pub scenario: String,
    pub blast_radius: u8,
    pub recovery_minutes: u32,
}

impl CreateRehearsal {
    pub fn into_domain(self, operator: String) -> RehearsalRequest {
        RehearsalRequest {
            service: self.service,
            scenario: self.scenario,
            blast_radius: self.blast_radius,
            recovery_minutes: self.recovery_minutes,
            operator,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RehearsalResponse {
    pub id: String,
    pub correlation_id: String,
    pub status: String,
    pub risk_score: f64,
    pub stages: Vec<String>,
    pub evidence_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}

pub fn schema_fingerprint() -> String {
    "blastglass-api-v1-2026-11-20".into()
}
