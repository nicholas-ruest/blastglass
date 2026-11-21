use blastglass_kernel::RehearsalRequest;
use blastglass_scenario::compile_plan;
fn request() -> RehearsalRequest {
    RehearsalRequest {
        service: "checkout".into(),
        scenario: "regional failover exercise".into(),
        blast_radius: 25,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    }
}
#[test]
fn produces_domain_evidence() {
    let r = compile_plan(&request()).unwrap();
    assert_eq!(r.stage, "scenario");
    assert!(!r.evidence.is_empty());
}
