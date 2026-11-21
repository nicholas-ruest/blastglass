use blastglass_api_schema::CreateRehearsal;
#[test]
fn domain_validation_rejects_empty_service() {
    let d = CreateRehearsal {
        service: "".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
    }
    .into_domain("sre@example.com".into());
    assert!(d.validate_identity().is_err());
}
