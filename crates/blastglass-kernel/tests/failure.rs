use blastglass_kernel::{DomainError, RehearsalRequest};
#[test]
fn rejects_impossible_recovery_window() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 25,
        recovery_minutes: 0,
        operator: "sre".into(),
    };
    assert_eq!(
        r.validate_identity(),
        Err(DomainError::Range("recovery_minutes"))
    );
}
