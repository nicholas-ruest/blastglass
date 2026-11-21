use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};

pub mod topology;

pub fn compile_plan(request: &RehearsalRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.scenario.len() < 6 {
        return Err(DomainError::Denied("scenario is too vague".into()));
    }
    let analysis = topology::analyze_request(request)?;
    let vocabulary = request.scenario.split_whitespace().count() as f64 * 4.0;
    let topology_exposure = analysis.maximum_weighted_impact * 0.45;
    let score = (vocabulary + topology_exposure).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "scenario".into(),
        score,
        message: format!(
            "compiled recovery scenario across {} affected components",
            analysis.affected_components
        ),
        evidence: vec![
            format!("scenario:{}:{:.2}", request.stable_key(), score),
            format!(
                "topology:depth={}:burden={}",
                analysis.longest_path, analysis.recovery_burden_minutes
            ),
        ],
    })
}
