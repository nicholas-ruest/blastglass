use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_nats::{publish_event, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf nats_ack".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert_eq!(publish_event(&c, &r).unwrap().stage, "upstream-nats");
}
