use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};

pub fn authorize_radius(request: &RehearsalRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.blast_radius > 80 {
        return Err(DomainError::Denied("blast radius exceeds policy".into()));
    }
    let score = (100.0 - request.blast_radius as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "authorization".into(),
        score,
        message: "authorized bounded blast radius".into(),
        evidence: vec![format!(
            "authorization:{}:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
