use blastglass_authorization::authorize_radius;
use blastglass_kernel::RehearsalRequest;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 90,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(authorize_radius(&r).is_err());
}
