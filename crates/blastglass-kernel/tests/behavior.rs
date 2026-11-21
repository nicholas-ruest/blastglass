use blastglass_kernel::RehearsalRequest;
#[test]
fn accepts_well_formed_rehearsal() {
    let r = RehearsalRequest {
        service: "checkout".into(),
        scenario: "regional failover".into(),
        blast_radius: 25,
        recovery_minutes: 30,
        operator: "sre@example.com".into(),
    };
    assert!(r.validate_identity().is_ok());
    assert_eq!(r.stable_key(), "checkout:regional failover:sre@example.com");
}
