# Identity and Access bounded context

## Purpose and language

Principal represents the authenticated operator; OperatorRole expresses authority to initiate a controlled failure rehearsal.

## Aggregate boundary and invariants

Authentication establishes a principal before authorization. Only rehearsal-operator may mutate state; readers may inspect health and audit evidence.

## Commands, events and failure semantics

AuthenticationRequired and PolicyDenied terminate the command before domain/upstream calls. AccessRestored begins a new authorized attempt. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `blastglass-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
