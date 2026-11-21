use crate::middleware::{authn, authz_policy, correlation, timeouts, CorrelationId, Principal};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use blastglass_api_schema::{CreateRehearsal, ErrorResponse, RehearsalResponse};
use blastglass_audit::AuditChain;
use blastglass_kernel::{DomainError, RehearsalRequest, StageResult};
use blastglass_persistence::EvidenceStore;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<EvidenceStore>,
}

impl AppState {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DomainError> {
        Ok(Self {
            store: Arc::new(EvidenceStore::open(path)?),
        })
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route(
            "/api/v1/rehearsals",
            post(create_rehearsal).get(list_rehearsals),
        )
        .route("/api/v1/rehearsals/{correlation}", get(get_rehearsal))
        .route("/api/v1/audit", get(audit))
        .layer(middleware::from_fn(timeouts))
        .layer(middleware::from_fn(authz_policy))
        .layer(middleware::from_fn(authn))
        .layer(middleware::from_fn(correlation))
        .fallback_service(
            ServeDir::new("frontend/dist")
                .not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"ready","api":"v1","service":"blastglass"}))
}

fn error(error: DomainError, correlation: &str) -> (StatusCode, Json<ErrorResponse>) {
    let status = match error {
        DomainError::Denied(_) => StatusCode::FORBIDDEN,
        DomainError::Missing(_) | DomainError::Range(_) => StatusCode::UNPROCESSABLE_ENTITY,
        DomainError::Upstream(_) => StatusCode::BAD_GATEWAY,
        DomainError::Integrity(_) | DomainError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (
        status,
        Json(ErrorResponse {
            code: status.as_u16().to_string(),
            message: error.to_string(),
            correlation_id: correlation.into(),
        }),
    )
}

fn config(
    name: &str,
    marker: &str,
) -> Result<blastglass_upstream_ruvector::AdapterConfig, DomainError> {
    let script = std::env::var(name)
        .map_err(|_| DomainError::Upstream(format!("missing runtime binding {name}")))?;
    Ok(blastglass_upstream_ruvector::AdapterConfig {
        command: "/bin/sh".into(),
        args: vec!["-c".into(), script],
        required_marker: marker.into(),
    })
}

fn run_upstreams(request: &RehearsalRequest) -> Result<Vec<StageResult>, DomainError> {
    let mut stages = Vec::new();
    let c = config("BLASTGLASS_RUVECTOR_CMD", "similarity_set")?;
    stages.push(blastglass_upstream_ruvector::retrieve_neighbors(
        &c, request,
    )?);
    let raw = config("BLASTGLASS_FANN_CMD", "calibrated_interval")?;
    let c = blastglass_upstream_fann::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_fann::infer_risk(&c, request)?);
    let raw = config("BLASTGLASS_MIDSTREAM_CMD", "safety_window")?;
    let c = blastglass_upstream_midstream::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_midstream::aggregate_stream(
        &c, request,
    )?);
    let raw = config("BLASTGLASS_QUDAG_CMD", "kem_ciphertext")?;
    let c = blastglass_upstream_qudag::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_qudag::seal_envelope(&c, request)?);
    let raw = config("BLASTGLASS_FEDERATION_CMD", "federated_receipt")?;
    let c = blastglass_upstream_federation::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_federation::dispatch_tool(&c, request)?);
    let raw = config("BLASTGLASS_OPA_CMD", "policy_allow")?;
    let c = blastglass_upstream_opa::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_opa::evaluate_policy(&c, request)?);
    let raw = config("BLASTGLASS_NATS_CMD", "nats_ack")?;
    let c = blastglass_upstream_nats::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(blastglass_upstream_nats::publish_event(&c, request)?);
    Ok(stages)
}

async fn create_rehearsal(
    State(state): State<AppState>,
    Extension(principal): Extension<Principal>,
    Extension(correlation): Extension<CorrelationId>,
    Json(input): Json<CreateRehearsal>,
) -> Result<(StatusCode, Json<RehearsalResponse>), (StatusCode, Json<ErrorResponse>)> {
    let request = input.into_domain(principal.0);
    request
        .validate_identity()
        .map_err(|e| error(e, &correlation.0))?;
    if request.blast_radius > 80 {
        return Err(error(
            DomainError::Denied("blast radius exceeds OPA policy boundary".into()),
            &correlation.0,
        ));
    }
    let mut stages =
        blastglass_orchestration::run_domain(&request).map_err(|e| error(e, &correlation.0))?;
    let upstream_request = request.clone();
    let upstream_stages = tokio::task::spawn_blocking(move || run_upstreams(&upstream_request))
        .await
        .map_err(|join_error| {
            error(
                DomainError::Upstream(format!("upstream worker failed: {join_error}")),
                &correlation.0,
            )
        })?
        .map_err(|e| error(e, &correlation.0))?;
    stages.extend(upstream_stages);
    let risk_score =
        blastglass_orchestration::aggregate_score(&stages).map_err(|e| error(e, &correlation.0))?;
    let id = Uuid::new_v4().to_string();
    let payload = serde_json::json!({"id":id,"service":request.service,"scenario":request.scenario,
        "risk_score":risk_score,"stages":stages.iter().map(|s|s.stage.clone()).collect::<Vec<_>>()})
    .to_string();
    let persisted = state.store.load().map_err(|e| error(e, &correlation.0))?;
    blastglass_evidence::ledger::verify_chain(&persisted).map_err(|e| error(e, &correlation.0))?;
    let mut chain = AuditChain::default();
    for item in &persisted {
        let rebuilt = chain
            .append(&item.id, &item.correlation_id, &item.payload)
            .map_err(|e| error(e, &correlation.0))?;
        if rebuilt.hash != item.hash {
            return Err(error(
                DomainError::Integrity("persisted audit hash mismatch".into()),
                &correlation.0,
            ));
        }
    }
    let record = chain
        .append(&id, &correlation.0, &payload)
        .map_err(|e| error(e, &correlation.0))?
        .clone();
    state
        .store
        .append(&record)
        .map_err(|e| error(e, &correlation.0))?;
    Ok((
        StatusCode::CREATED,
        Json(RehearsalResponse {
            id,
            correlation_id: correlation.0,
            status: "completed".into(),
            risk_score,
            stages: stages.into_iter().map(|s| s.stage).collect(),
            evidence_hash: record.hash,
        }),
    ))
}

async fn list_rehearsals(
    State(state): State<AppState>,
) -> Result<Json<Vec<blastglass_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    state.store.load().map(Json).map_err(|e| error(e, "list"))
}

async fn audit(
    State(state): State<AppState>,
) -> Result<Json<Vec<blastglass_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    state.store.load().map(Json).map_err(|e| error(e, "audit"))
}

async fn get_rehearsal(
    State(state): State<AppState>,
    Path(correlation): Path<String>,
) -> Result<Json<blastglass_replay::ReplaySummary>, (StatusCode, Json<ErrorResponse>)> {
    blastglass_replay::replay(&state.store, &correlation)
        .map(Json)
        .map_err(|e| error(e, &correlation))
}
