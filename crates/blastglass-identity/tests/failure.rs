use blastglass_identity::verify_operator;
use blastglass_kernel::RehearsalRequest;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "invalid".into(),
    };
    assert!(verify_operator(&r).is_err());
}
