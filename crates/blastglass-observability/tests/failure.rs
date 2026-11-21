use blastglass_kernel::RehearsalRequest;
use blastglass_observability::instrument;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service:
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                .into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(instrument(&r).is_err());
}
