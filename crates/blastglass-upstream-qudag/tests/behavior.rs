use blastglass_kernel::RehearsalRequest;
use blastglass_upstream_qudag::{seal_envelope, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf kem_ciphertext".into()]);
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert_eq!(seal_envelope(&c, &r).unwrap().stage, "upstream-qudag");
}
