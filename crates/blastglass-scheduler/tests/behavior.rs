use blastglass_kernel::RehearsalRequest;
use blastglass_scheduler::schedule;
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
    let r = schedule(&request()).unwrap();
    assert_eq!(r.stage, "scheduler");
    assert!(!r.evidence.is_empty());
}
