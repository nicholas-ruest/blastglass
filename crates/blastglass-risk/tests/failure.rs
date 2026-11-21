use blastglass_kernel::RehearsalRequest;
use blastglass_risk::score_risk;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 0,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(score_risk(&r).is_err());
}
