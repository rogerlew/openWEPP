# openWEPP Gate Policy Contracts v1

Status: schema authority with a shadow planner; nonblocking until executor
cutover

Decision authority: `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`

Operational authority: `docs/standards/testing-and-gate-strategy.md`

This directory contains the machine-readable contracts for deterministic
gate planning and evidence. `schemas/` defines strict JSON Schema Draft 2020-12
documents. `fixtures/valid/` demonstrates admitted payloads;
`fixtures/invalid/` contains one-mutation descriptors that derive each negative
case from its positive counterpart and bind the intended instance/schema error
path. `impact-map.json` seeds non-Cargo ownership for the gate-policy authority
itself. `gate-definitions.json` supplies the shadow planner's closed typed
argument-vector registry; it never contains shell source.

TESTGATE-PLAN-01 consumes these contracts to implement nonblocking selection
and verification. It does not execute gates, publish evidence, certify a
campaign, or mutate assurance. `TESTGATE-CI-01` owns
executor and cutover behavior, and `TESTGATE-ASSURE-01` owns assurance planner
integration.

Identity-bearing JSON uses I-JSON constraints, RFC 8785 canonicalization, and
SHA-256 as required by the operational standard. Derived ID fields are excluded
from their own digest payload; referenced predecessor and input identities
remain included. The v1 schemas constrain shape and closed vocabularies. The
shadow verifier independently enforces canonicalization, digest equality,
DAG acyclicity/topological order, transition folds, transitive-root
reconstruction, attestation trust, and hermetic reuse.

The schemas nevertheless carry every input required for those future checks:
planner promotion/ownership and execution-context identities, typed assurance
axes, governed zero-work evidence, complete gate-node receipt snapshots,
canonical authority outcomes, a single typed receipt attestation subject,
discriminated campaign events, anchored backstop/certification state, and
immutable assurance-impact events. Schema conditionals reject locally
expressible contradictions; the source-level semantic guard additionally
rejects cross-field inventory, attempt, outcome, mutation-digest,
certification-head, receipt, and authorization inconsistencies. The shadow
verifier implements those same semantic checks over arbitrary records.

In v1, `policy_id` is `ADR-0039` and `policy_sha256` binds the exact bytes of
`docs/standards/testing-and-gate-strategy.md`, the living operational policy
adopted by that ADR.

The v1 execution matrix is deliberately limited to the exact `rustc` host and
the default feature set. Any other target or declared non-default feature fails
closed until a later versioned matrix explicitly admits it.
