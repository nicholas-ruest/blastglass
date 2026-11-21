use blastglass_kernel::RehearsalRequest;
use blastglass_validation::validate_window;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 200,
        operator: "sre@example.com".into(),
    };
    assert!(validate_window(&r).is_err());
}
