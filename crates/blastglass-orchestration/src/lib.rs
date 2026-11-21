use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};

pub fn run_domain(request: &RehearsalRequest) -> Result<Vec<StageResult>, DomainError> {
    let stages = vec![
        blastglass_scenario::compile_plan(request)?,
        blastglass_identity::verify_operator(request)?,
        blastglass_authorization::authorize_radius(request)?,
        blastglass_validation::validate_window(request)?,
        blastglass_correlation::correlate(request)?,
        blastglass_deadline::budget(request)?,
        blastglass_observability::instrument(request)?,
        blastglass_risk::score_risk(request)?,
        blastglass_telemetry::ingest(request)?,
        blastglass_scheduler::schedule(request)?,
    ];
    if stages.iter().any(|stage| stage.evidence.is_empty()) {
        return Err(DomainError::Integrity(
            "domain stage omitted evidence".into(),
        ));
    }
    Ok(stages)
}

pub fn aggregate_score(stages: &[StageResult]) -> Result<f64, DomainError> {
    if stages.is_empty() {
        return Err(DomainError::Missing("stages"));
    }
    Ok(stages.iter().map(|s| s.score).sum::<f64>() / stages.len() as f64)
}
