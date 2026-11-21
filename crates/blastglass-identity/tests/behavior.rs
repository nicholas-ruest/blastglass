use blastglass_identity::verify_operator;
use blastglass_kernel::RehearsalRequest;
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
    let r = verify_operator(&request()).unwrap();
    assert_eq!(r.stage, "identity");
    assert!(!r.evidence.is_empty());
}
