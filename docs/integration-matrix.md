# Integration matrix

| Project and revision | Product responsibility | Rust call site | Success marker | Failure proof |
|---|---|---|---|---|
| RuVector `979c31a` | similar-incident retrieval | upstream-ruvector | `similarity_set` | corrupt index load |
| ruv-FANN `02431fd` | calibrated recovery risk | upstream-fann | `calibrated_interval` | invalid quantile/config |
| midstream `0934fd3` | streaming safety window | upstream-midstream | `safety_window` | constraint violation |
| QuDAG `b534668` | protected evidence envelope | upstream-qudag | `kem_ciphertext` | security level mismatch |
| federated-mcp `1caf7ac` | federated lifecycle receipt | upstream-federation | `federated_receipt` | rejected write |
| OPA `ba39415` | independent policy decision | upstream-opa | `policy_allow` | denied input |
| NATS `2a9066e` | rehearsal event round trip | upstream-nats | `nats_ack` | bad credentials |

All wrappers verify a 40-character checkout HEAD before execution. Adapter tests additionally prove non-zero exit and missing-marker behavior. Coordination acknowledgments are never included as product integrations.
