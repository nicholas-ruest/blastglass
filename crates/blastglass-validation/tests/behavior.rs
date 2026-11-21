use blastglass_kernel::RehearsalRequest;
use blastglass_validation::validate_window;
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
    let r = validate_window(&request()).unwrap();
    assert_eq!(r.stage, "validation");
    assert!(!r.evidence.is_empty());
}
