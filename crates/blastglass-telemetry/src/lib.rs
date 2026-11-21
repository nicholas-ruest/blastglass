use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};

pub fn ingest(request: &RehearsalRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.scenario.contains("NaN") {
        return Err(DomainError::Denied(
            "telemetry contains invalid sample".into(),
        ));
    }
    let score = ((request.blast_radius as f64).sqrt() * 10.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "telemetry".into(),
        score,
        message: "ingested normalized rehearsal telemetry".into(),
        evidence: vec![format!("telemetry:{}:{:.2}", request.stable_key(), score)],
    })
}
