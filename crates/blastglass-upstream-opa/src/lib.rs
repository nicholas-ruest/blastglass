use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub command: String,
    pub args: Vec<String>,
    pub required_marker: String,
}
impl AdapterConfig {
    pub fn pinned(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            command: command.into(),
            args,
            required_marker: "policy_allow".into(),
        }
    }
}

pub fn evaluate_policy(
    config: &AdapterConfig,
    request: &RehearsalRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("BLASTGLASS_SERVICE", &request.service)
        .env("BLASTGLASS_SCENARIO", &request.scenario)
        .output()
        .map_err(|e| DomainError::Upstream(format!("upstream-opa: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "upstream-opa exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "upstream-opa missing marker {}",
            config.required_marker
        )));
    }
    Ok(StageResult {
        stage: "upstream-opa".into(),
        score: 100.0,
        message: "evaluate authorization policy".into(),
        evidence: vec![text.trim().into()],
    })
}
