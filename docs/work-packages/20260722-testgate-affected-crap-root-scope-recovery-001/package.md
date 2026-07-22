# TESTGATE Affected CRAP Root-Scope Recovery

Package ID: `20260722-testgate-affected-crap-root-scope-recovery-001`

Queue ID: `TESTGATE-AFFECTED-CRAP-ROOT-SCOPE-RECOVERY-01`

Status: `ACTIVE / REVIEW`

## Progress

- [x] Scaffold commit `ddb00d41` predates tooling/test edits.
- [x] Validate affected measurement package ownership before acquisition.
- [x] Reject root `openwepp` as measurement-only and require global quality.
- [x] Retain exact admitted scope for valid production packages.
- [x] Update all three CRAP adapter digests.
- [x] Focused Python, shell, integration, policy, formatting, and Clippy checks
  pass.
- [ ] Complete dual review and disposition this package at the planner boundary.
- [ ] Scaffold the fresh planner-escalation prerequisite; RTR-035 remains open.

## Objective

Close the checker/driver half of RTR-035 by making affected CRAP distinguish
measurement packages from production source packages before acquisition. A
workspace-root/test-aggregation package must fail closed and require global
quality; only packages owning `crates/*/src` may enter affected acquisition.

## Observed Failure

Receipt `71d1081c...7587` records affected CRAP failure after its instrumented
Nextest traversal passed 1,091/1,091. The planner correctly selected root package
`openwepp` to execute the changed integration-test surface, but adjudication
rejected it because canonical production sources live under `crates/*/src`.
The incompatibility was checked only after the 689.242-second traversal.

## Declared Write Set

- `tools/release/check_adjudicated_crap.py`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/README.md`
- `tests/python/test_adjudicated_crap_gate.py`
- `tests/integration/testgate_ci_executor_contract.rs`
- `gate-policy/v1/gate-definitions.json`
- `docs/work-packages/20260722-testgate-affected-crap-root-scope-recovery-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before tooling/test edits.
2. Resolve requested measurement packages from locked Cargo metadata. Production
   packages map to themselves; measurement-only or unknown packages fail and
   require global quality.
3. Invoke the mapping preflight before coverage acquisition and retain its exact
   JSON in the CRAP output envelope.
4. Add positive root-aggregation, ordinary production, unknown, and empty-map
   regressions; update source contracts and the exact adapter digest.
5. Run focused checks and dual review. Hold at the immutable write-set boundary
   while a fresh planner package owns global escalation; close RTR-035 only
   after both halves pass review.

## Exit Criteria

- Root `openwepp` fails before coverage and requires global quality.
- Valid production packages map to themselves; invalid names fail closed.
- Fresh adjudication filters by resolved production packages and retains both
  requested measurement and resolved production identities.
- Focused Python/integration tests, shell syntax, policy checks, and package
  admission pass.
- Dual independent review passes for this early-admission half.
- A fresh planner package owns global escalation and final RTR-035 closure
  before one delegated changed-head qualification.

Planner escalation is outside this immutable package write set and must be
scaffolded as a fresh prerequisite before qualification.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for exact qualification, and two independent read-only
terminal verifiers. Outputs are package-local reviews/verifications and retained
external qualification evidence. Write access is read-only except for the
comparator's ignored evidence root. Do not push, deploy, switch branches,
manually dispatch TESTGATE, run HEAVY on the parent, or repeat unchanged gates.
