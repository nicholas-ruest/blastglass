use blastglass_api_schema::{CreateRehearsal, API_VERSION};
#[test]
fn maps_typed_contract() {
    let d = CreateRehearsal {
        service: "checkout".into(),
        scenario: "failover".into(),
        blast_radius: 20,
        recovery_minutes: 30,
    }
    .into_domain("sre@example.com".into());
    assert_eq!(d.service, "checkout");
    assert_eq!(API_VERSION, "v1");
}
