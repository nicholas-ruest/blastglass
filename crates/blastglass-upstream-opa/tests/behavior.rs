use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_opa::{evaluate_policy, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf policy_allow".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert_eq!(evaluate_policy(&c, &r).unwrap().stage, "upstream-opa");
}
