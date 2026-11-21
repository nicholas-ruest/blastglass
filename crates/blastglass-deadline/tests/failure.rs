use blastglass_deadline::budget;
use blastglass_kernel::RehearsalRequest;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 900,
        operator: "sre@example.com".into(),
    };
    assert!(budget(&r).is_err());
}
