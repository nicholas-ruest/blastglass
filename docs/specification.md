# Blastglass product specification

Blastglass helps site reliability engineers rehearse operational recovery decisions without executing a real destructive change. An operator defines a service, disruption scenario, blast radius and recovery target. The system authenticates and authorizes the command, validates the domain intent, executes ten Rust domain stages and seven source-pinned upstream responsibilities, computes a transparent composite score, and seals the completed outcome into a restart-safe audit chain.

## Acceptance outcomes

1. An authorized operator receives a completed result with seventeen named stages, correlation ID, score and evidence seal.
2. An observer or over-boundary request is denied before any upstream call or durable write; correcting the role completes the same authored scenario.
3. A saved result remains visible after browser refresh and backend process restart, with the same correlation and digest.

The product is local-first and deployable by configuration only. It does not provision cloud resources, trigger production failures, merge its own PR, or claim that its prototype score is a probability. Source integrity, complete evidence and human review are release constraints rather than optional hardening.
