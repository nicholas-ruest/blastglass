use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_midstream::{aggregate_stream, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf safety_window".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert_eq!(
        aggregate_stream(&c, &r).unwrap().stage,
        "upstream-midstream"
    );
}
