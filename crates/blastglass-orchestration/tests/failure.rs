use blastglass_kernel::RehearsalRequest;
use blastglass_orchestration::run_domain;
#[test]
fn denial_stops_composition() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 90,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(run_domain(&r).is_err());
}
