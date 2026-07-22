# TESTGATE Affected CRAP Root-Scope Recovery

Package ID: `20260722-testgate-affected-crap-root-scope-recovery-001`

Queue ID: `TESTGATE-AFFECTED-CRAP-ROOT-SCOPE-RECOVERY-01`

Status: `ACTIVE / SCAFFOLD`

## Objective

Close RTR-035 by making affected CRAP distinguish measurement packages from
production source packages. A workspace-root/test-aggregation package must map
fail-closed to its local production dependency closure, and that mapping must
be validated before expensive coverage acquisition.

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
   packages map to themselves; measurement-only packages map to their local
   transitive production dependency closure; unknown or empty mappings fail.
3. Invoke the mapping preflight before coverage acquisition and retain its exact
   JSON in the CRAP output envelope.
4. Add positive root-aggregation, ordinary production, unknown, and empty-map
   regressions; update source contracts and the exact adapter digest.
5. Run focused checks, dual review, durable RTR-035 closure, then one delegated
   changed-head qualification.

## Exit Criteria

- Root `openwepp` resolves to its canonical local production dependency closure.
- Invalid measurement names and empty production mappings fail before coverage.
- Fresh adjudication filters by resolved production packages and retains both
  requested measurement and resolved production identities.
- Focused Python/integration tests, shell syntax, policy checks, and package
  admission pass.
- Dual independent review passes and RTR-035 is durably closed.
- One delegated changed-head qualification passes, followed by dual terminal
  verification without rerunning HEAVY.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for exact qualification, and two independent read-only
terminal verifiers. Outputs are package-local reviews/verifications and retained
external qualification evidence. Write access is read-only except for the
comparator's ignored evidence root. Do not push, deploy, switch branches,
manually dispatch TESTGATE, run HEAVY on the parent, or repeat unchanged gates.
