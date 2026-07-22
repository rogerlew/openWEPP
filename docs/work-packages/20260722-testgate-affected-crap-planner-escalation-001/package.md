# TESTGATE Affected CRAP Planner Escalation

Package ID: `20260722-testgate-affected-crap-planner-escalation-001`

Queue ID: `TESTGATE-AFFECTED-CRAP-PLANNER-ESCALATION-01`

Status: `ACTIVE / SCAFFOLD`

## Progress

- [x] Scaffold commit `cb6eda5e` predates planner edits.
- [x] Separate measurement packages from production-owning quality packages.
- [x] Escalate direct measurement-only changes to critical/global quality.
- [x] Focused root/production/repository regressions and target Clippy pass.
- [ ] Commit the correction and rerun the clean-checkout planner target.
- [ ] Complete package audit, dual review, RTR-035 closure, and qualification.

## Objective

Close the planner half of RTR-035 by escalating any bounded selection containing
a measurement-only Cargo package to critical/global quality before node
construction. A root integration-test change must select global adjudicated CRAP
and must never schedule affected CRAP with package `openwepp`.

## Correction Authority Envelope

- Defect: RTR-035,
  `GATE-AFFECTED-CRAP-ROOT-PACKAGE-ADMISSION-MISMATCH`.
- Observed failure: receipt `71d1081c...7587`; affected coverage passed
  1,091/1,091 before root package admission failed.
- Authority: Cargo workspace ownership from the reconstructed base/head graph,
  plus the reviewed checker/driver prerequisite at `c79bf202`.
- Protected boundaries: do not map root measurement to dependency closure, do
  not weaken affected preflight, do not rerun retained coverage, and do not
  alter science, execution, receipt, retry, or comparator semantics.

## Declared Write Set

- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/repository.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `docs/work-packages/20260722-testgate-affected-crap-planner-escalation-001/**`
- `docs/work-packages/20260722-testgate-affected-crap-root-scope-recovery-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before planner edits.
2. Classify production-owning versus measurement-only workspace packages from
   the reconstructed Cargo graph.
3. Escalate selections containing a measurement-only package to critical risk
   with an explicit reason before gate definitions are instantiated.
4. Prove a root integration-test change selects global CRAP, not affected CRAP;
   preserve ordinary production-package planning behavior.
5. Run focused planner tests, formatting, Clippy, package audit, and dual review.
6. Close RTR-035 only at the exact reviewed correction commit, rebuild the
   release planner, and delegate exactly one changed-head qualification.

## Exit Criteria

- Root `openwepp` is never passed to affected adjudicated CRAP.
- Measurement-only selection produces critical risk, global quality scope, an
  `adjudicated-crap-v1` node, no `affected-adjudicated-crap-v1` node, and a typed
  reason code.
- Existing bounded production-package coverage remains characterized.
- Focused tests, formatting, Clippy, package admission, and dual review pass.
- RTR-035 closes durably before one delegated changed-head qualification.

## Security Impact

Fail-closed strengthening only. No secrets, network authority, deployment,
branch change, retry expansion, or gate bypass is introduced.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for exact qualification, and two independent read-only
terminal verifiers. Expected outputs are package-local reviews/verifications and
retained external qualification evidence. Write access is read-only except for
the comparator's ignored evidence root. Do not push, deploy, switch branches,
manually dispatch TESTGATE, run HEAVY on the parent, or repeat unchanged gates.
