use blastglass_kernel::RehearsalRequest;
use blastglass_telemetry::ingest;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "NaN".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(ingest(&r).is_err());
}
