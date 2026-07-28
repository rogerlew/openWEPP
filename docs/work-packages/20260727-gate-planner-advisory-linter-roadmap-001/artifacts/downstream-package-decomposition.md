# Downstream Package Decomposition

No package below is scaffolded by Order 0. Each requires separate user
authorization.

## Order 1 — Governance Authority Alignment

Objective: apply the accepted conflict-matrix patches so operative instructions
describe direct agent execution and a non-authoritative linter.

Write boundary: relevant `AGENTS.md`, testing/gate standards, tool guidance,
decision/catalog cross-references, the two registered TESTGATE source/schema
guard tests and their `Cargo.toml` registrations, frozen package status
overlays, impact-policy authority rows, and a literal historical-policy
identity registry. No planner executable or CAL result.

Acceptance: no prospective planner/TESTGATE authority remains; every preserved
correctness, quality, calibration, and Harvard obligation has a direct owner.
The increment lands under ADR-0043 through direct manual checks and does not
require a final planner-admitted run. Historical policy verification uses the
pinned old object, and no frozen package can be resumed accidentally. Rollback:
revert the governance/guard/status increment as one unit.

## Order 2 — CAL Legacy-Integration Removal

Objective: remove CAL-04B dependence on general gate-planner transactions while
preserving direct package commands, immediate primary-failure recording, and
the protected Harvard barrier.

Write boundary: CAL package-local executor/control artifacts and the minimum
custody mechanism. No model-domain or frozen calibration-design change.

Acceptance: fresh direct pre-Harvard execution can proceed without planner
state; failures are durably recorded before cleanup; the separate custody owner
proves every invariant listed in ADR-0043 Decision 10. Rollback: retain the old
adapter read-only until the replacement proof passes; Harvard remains sealed.

## Order 3 — Advisory Linter Thin Slice

Objective: implement the frozen interface using only read-only static analysis.

Write boundary: a new neutral tool surface, focused fixtures/tests, and user
documentation. It must not import executor, receipt, ledger, CI, publication,
recovery, CAL, or custody modules.

Acceptance: all modes and schema fields work deterministically; allowlist and
no-write/no-execute tests pass; injected failures demonstrate manual
continuation. Rollback: delete the thin slice; manual route remains complete.

## Order 4 — Legacy Execution And CI Retirement

Objective: delete or historical-quarantine the planner control plane after
consumer inventory and replacement proofs close.

Write boundary: legacy Rust/Python planner surfaces, schemas, workflows,
policies, docs, and consumers named by the migration map.

Acceptance: live consumers are zero; named historical verification remains
read-only; no CI invokes the linter; direct canonical commands still work.
Rollback: restore only a specific deleted consumer path, never prospective
authority.

## Order 5 — Agent-Friction Qualification

Objective: compare the thin slice with the manual route on the frozen cohort.

Write boundary: package-local fixtures, measurements, and findings; only small
tool corrections explicitly authorized by the package.

Acceptance: every metric and stop-loss in
`friction-baseline-and-success-metrics.md` is satisfied. Otherwise disable or
delete the linter and retain the manual route. No CI qualification is allowed.
