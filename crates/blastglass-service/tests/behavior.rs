use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blastglass_service::app::{router, AppState};
use tower::ServiceExt;

fn configure() {
    for (name, marker) in [
        ("BLASTGLASS_RUVECTOR_CMD", "similarity_set"),
        ("BLASTGLASS_FANN_CMD", "calibrated_interval"),
        ("BLASTGLASS_MIDSTREAM_CMD", "safety_window"),
        ("BLASTGLASS_QUDAG_CMD", "kem_ciphertext"),
        ("BLASTGLASS_FEDERATION_CMD", "federated_receipt"),
        ("BLASTGLASS_OPA_CMD", "policy_allow"),
        ("BLASTGLASS_NATS_CMD", "nats_ack"),
    ] {
        std::env::set_var(name, format!("printf {marker}"));
    }
}

#[tokio::test]
async fn browser_to_durable_result_slice() {
    configure();
    let path =
        std::env::temp_dir().join(format!("blastglass-service-{}.jsonl", std::process::id()));
    let app = router(AppState::open(&path).unwrap());
    let body = r#"{"service":"checkout","scenario":"regional failover","blast_radius":25,"recovery_minutes":30}"#;
    let response = app
        .oneshot(
            Request::post("/api/v1/rehearsals")
                .header("authorization", "Bearer blastglass-operator")
                .header("x-role", "rehearsal-operator")
                .header("x-operator", "sre@example.com")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        AppState::open(&path).unwrap().store.load().unwrap().len(),
        1
    );
    let _ = std::fs::remove_file(path);
}
