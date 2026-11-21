use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_opa::{evaluate_policy, AdapterConfig};
#[test]
fn propagates_upstream_failure() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "exit 17".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(evaluate_policy(&c, &r).is_err());
}
