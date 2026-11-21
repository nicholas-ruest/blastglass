use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_ruvector::{retrieve_neighbors, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf similarity_set".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert_eq!(
        retrieve_neighbors(&c, &r).unwrap().stage,
        "upstream-ruvector"
    );
}
