# TESTGATE-PLAN-01 Implementation Handoff

Evidence class: `Static`

TESTGATE-ALIGN-01 establishes schema authority only. The production impact map
remains `SCHEMA_ONLY_NONBLOCKING`; no selector, executor, trusted receipt,
campaign certificate, affected-coverage runner, CI cutover, or assurance
mutation path exists yet.

## First Actionable Package

`TESTGATE-PLAN-01` is the next eligible test/gate package. It must consume
`gate-policy/v1/` in shadow mode and implement:

- canonical Git change sets and Cargo reverse-dependency/feature expansion;
- versioned non-Cargo impact edges with critical unknown fallback;
- intent and terminal plan reconciliation, stable topological DAGs, and verified
  zero-work plans;
- RFC 8785 identity derivation, transitive input manifests, and root
  reconstruction;
- executor confinement for typed argument arrays and versioned legacy adapters;
- exact planned/executed inventory comparison and canonical outcome reduction;
- unsigned receipt verification, typed attestation subject equality,
  signature/issuer/revocation checks, and hermetic reuse classes; and
- append-only campaign and assurance folds, compare-and-swap ancestry, anchored
  backstop/certification checks, and deterministic target-bound currency.

## Acceptance Boundary

The planner remains nonblocking until retained packages/campaigns replay in
shadow mode and the fixed scorecard in the canonical testing/gate strategy is
met. Any missing input, unknown path, empty unverified inventory, identity
drift, invalid event transition, unsafe reuse, or ambiguous subject fails
closed. The conservative full runner remains the implementation fallback.

`TESTGATE-CI-01` and `TESTGATE-ASSURE-01` remain later packages; PLAN must not
claim their executor, CI publication, or assurance-transfer behavior.
