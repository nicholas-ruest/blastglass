use blastglass_correlation::correlate;
use blastglass_kernel::RehearsalRequest;
#[test]
fn rejects_domain_boundary() {
    let r = RehearsalRequest {
        service: "".into(),
        scenario: "x".into(),
        blast_radius: 20,
        recovery_minutes: 30,
        operator: "x".into(),
    };
    assert!(correlate(&r).is_err());
}
