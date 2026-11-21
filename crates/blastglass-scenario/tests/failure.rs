use blastglass_kernel::RehearsalRequest;
use blastglass_scenario::compile_plan;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "x".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(compile_plan(&r).is_err());
}
