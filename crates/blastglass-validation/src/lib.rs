use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};

pub fn validate_window(request: &RehearsalRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.recovery_minutes > 120 {
        return Err(DomainError::Denied(
            "recovery window exceeds two hours".into(),
        ));
    }
    let score = (120.0 - request.recovery_minutes.min(120) as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "validation".into(),
        score,
        message: "validated recovery objective".into(),
        evidence: vec![format!("validation:{}:{:.2}", request.stable_key(), score)],
    })
}
