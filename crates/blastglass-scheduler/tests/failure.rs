use blastglass_kernel::RehearsalRequest;
use blastglass_scheduler::schedule;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 1,
        operator: "sre@example.com".into(),
    };
    assert!(schedule(&r).is_err());
}
