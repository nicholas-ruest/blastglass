# Crate-to-backend production call paths

The `blastglass-service` binary constructs the router and evidence store. The create handler calls identity validation and `blastglass-orchestration::run_domain`, whose ordered path reaches scenario, identity, authorization, validation, correlation, deadline, observability, risk, telemetry and scheduler. It then calls each of the seven upstream adapter crates and aggregates their typed results.

`blastglass-api-schema` converts the transport command into `blastglass-kernel::RehearsalRequest`. The service reconstructs `blastglass-audit::AuditChain`, whose seal/verify operations use `blastglass-evidence`, and commits through `blastglass-persistence`. GET by correlation calls `blastglass-replay`, which re-verifies stored evidence. The service therefore reaches all 25 packages through a production route rather than development-only features.

Behavior and failure tests live next to every package. Reciprocal mapping is enforced by the composition report and semantic review: a package without a unique responsibility, caller and negative boundary is not counted.
