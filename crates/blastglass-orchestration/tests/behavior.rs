use blastglass_kernel::RehearsalRequest;
use blastglass_orchestration::{aggregate_score, run_domain};
#[test]
fn composes_domain_stages() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "regional failover".into(),
        blast_radius: 25,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    let s = run_domain(&r).unwrap();
    assert_eq!(s.len(), 10);
    assert!(aggregate_score(&s).unwrap() > 0.0);
}
