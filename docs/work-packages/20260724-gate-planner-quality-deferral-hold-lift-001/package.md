# Close Gate-Planner Quality-Deferral Regressions And Lift Order 3

This Defect-Closure ExecPlan is a living document governed by
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`. Keep
`Progress`, `Surprises & Discoveries`, `Decision Log`, and
`Outcomes & Retrospective` current throughout execution.

Package ID: `20260724-gate-planner-quality-deferral-hold-lift-001`

Status: `ACTIVE / SCAFFOLD`

## Purpose / Big Picture

Close defect `QOBS-HOLD-LIFT-01`: seven gate-planner tests retained by the
quality observatory's corrected full regression no longer agree with the
Order-2 `DEFERRED_TO_QUALITY_CI` schema and exact committed-checkout contract.
After correction, the full correctness inventory must pass and
`20260724-quality-observatory-merged-coverage-001` must resume from HOLD,
execute both coverage profiles, publish compact observational evidence, and
pass independent terminal verification.

## Progress

- [x] 2026-07-24: Retain the exact seven-test failure set and durable attempt-3
  evidence.
- [x] 2026-07-24: Scaffold this prerequisite before gate-policy or Rust edits.
- [ ] Reproduce and attribute all seven failures to named mechanisms.
- [ ] Correct policy fixtures/tests without weakening production guards.
- [ ] Pass focused, owning-crate, review, and full-workspace gates.
- [ ] Lift Order 3, run fresh merged coverage, and verify publication twice.
- [ ] Reconcile and close both packages.

## Objective

Align every affected executor, planner, pre-heavy, and verifier fixture with the
ratified Order-2 quality deferral and exact-checkout behavior; retain negative
coverage for retired quality nodes and dirty committed execution; then produce
the previously blocked Order-3 quality evidence.

## Correction Authority Envelope

Defect ID: `QOBS-HOLD-LIFT-01`.

Observed violation: attempt 3 at
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt3-LuBliP`
ran all 2,279 full-profile tests, passed 2,272, and failed exactly seven
`openwepp-gate-planner` tests. Two fixtures supplied prohibited retired quality
definitions, three fixture repositories violated the exact committed-checkout
contract, one mutation test never reached its expected command, and one
terminal-reconciliation test observed a lower-level checkout change instead of
its intended semantic delta.

In-scope correction surfaces:

- `gate-policy/v1/**`
- `crates/openwepp-gate-planner/**`
- package-local tests and fixtures on those surfaces
- this package tree and catalog entry
- hold-lift state/evidence under
  `docs/work-packages/20260724-quality-observatory-merged-coverage-001/**`

Allowed edit classes:

- remove or replace retired quality gate definitions in synthetic fixtures;
- make fixture repositories clean and exact at the commit each test names;
- update assertions so each test reaches and proves its intended contract;
- correct production validation only if reproduction proves a real
  Order-2 implementation inconsistency rather than stale fixture setup;
- add focused regression/source-contract coverage for the named mechanisms.

Acceptance:

- all seven exact failures pass without ignores or relaxed assertions;
- retired quality definitions remain schema-invalid and cannot enter an
  ordinary plan or receipt;
- dirty or nonexact committed checkouts remain fail-closed;
- the owning gate-planner crate and selected full workspace pass;
- a fresh Order-3 transition executes `full` and `science-manual`, merges
  coverage, publishes the exact compact artifact set, and passes two
  independent terminal verifications.

Protected boundaries:

- do not reintroduce affected/global/combined coverage or CRAP nodes into
  TESTGATE;
- do not weaken `GATE-COMMITTED-CHECKOUT-NOT-EXACT`, source-mutation,
  schema, receipt, or verifier fail-closed behavior;
- do not change CRAP thresholds, science behavior, kernel physics, workflow
  dispatch, or external systems;
- do not treat a focused pass as authority to skip selected terminal gates.

## Conversion Rule And HOLD Legitimacy

The seven failures are reproduced, testable, and owned by this envelope.
Therefore this package must diagnose, correct, validate, review, and disposition
them end-to-end. It may not HOLD for implementation effort or intermediate
diagnosis. A HOLD is legitimate only if required evidence becomes unavailable
or the mechanism is proven outside every declared gate-policy, planner, test,
and Order-3 hold-lift surface; any HOLD must include the audit required by
`docs/defect_closure_execplans.md`.

## Declared Write Set

- `gate-policy/v1/**`
- `crates/openwepp-gate-planner/**`
- `docs/work-packages/20260724-gate-planner-quality-deferral-hold-lift-001/**`
- `docs/work-packages/20260724-quality-observatory-merged-coverage-001/**`
- `docs/work-packages/README.md`

## Dependencies

- `20260724-testgate-quality-deferral-001` completed Order 2.
- `20260724-quality-observatory-merged-coverage-001` is committed at HOLD with
  exact attempt-3 evidence.
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- ADR-0041 and `docs/standards/testing-and-gate-strategy.md`

## Plan Of Work

First reproduce the seven tests individually and group failures only by proven
mechanism. Correct the shared fixture constructors and the smallest test-local
setup needed to preserve each intended assertion. Run the seven exact tests,
the complete gate-planner crate, formatting, warnings-denied Clippy, schema and
anti-evasion checks selected by the terminal diff, and dual independent review.

After a clean prerequisite correctness result, run the selected full-workspace
regression through `comparator_suite_runner`. Then change the Order-3 finding
disposition from the named predecessor blocker to fixed, return its package to
ACTIVE, create a fresh durable attempt, and delegate its one-process
`quality_observatory.py transition` run. Dispatch two read-only terminal
verifiers against the published evidence. Only then may both packages close.

## Validation And Acceptance

The pre-implementation intent plan must authenticate the exact terminal gates.
At minimum execution must retain:

- exact runs for all seven failing test identities;
- `cargo nextest run -p openwepp-gate-planner`;
- warnings-denied Clippy for the owning crate;
- Rustfmt, schema/source guards, documentation lint, diff/write-set checks, and
  line-count governance;
- delegated full-workspace correctness;
- delegated fresh Order-3 merged-coverage collection;
- two independent implementation/security reviews and two independent
  terminal verifications.

Every gate is `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`. Any non-PASS required
gate prevents complete disposition.

## Security Impact

This correction changes trust-policy fixtures. Review must prove that retired
quality execution remains prohibited and that source, Git, plan, receipt, and
verification exactness are not weakened. Run the external-authority
anti-evasion guards if the terminal diff touches authority-suite posture or
required-case bindings.

## Line-Count Governance

Every changed `.rs` file is measured. Files at or above 2,000 lines require a
WARN disposition and split intent; a nonexempt file at or above 3,000 lines
blocks closure unless refactored.

## Delegation

Subagent requirement: REQUIRED. This package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation/security
reviewers, `comparator_suite_runner` for every selected heavy/full-workspace and
quality-observatory run, and two independent read-only terminal verifiers.
Expected outputs are compact findings, test metrics, evidence identities, and
durable log paths. Repository write access for all subagents is read-only.

## Surprises & Discoveries

- None beyond the retained attempt-3 failure set at scaffold time.

## Decision Log

- Decision: keep the seven symptoms in one DC package.
  Rationale: they share the Order-2 gate-policy authority, gate-planner crate,
  fixture constructors, validation surface, and Order-3 hold-lift acceptance.
  Date/Author: 2026-07-24 / Codex.
- Decision: include the complete Order-3 rerun in this package's acceptance.
  Rationale: passing stale fixtures alone does not lift the consumer package's
  failed full-regression and publication gates.
  Date/Author: 2026-07-24 / Codex.

## Outcomes & Retrospective

Pending execution.

## Idempotence And Recovery

Focused tests and validation are repeatable. Every heavy run uses a new durable
attempt root and is never resumed after a terminal result. Failed heavy evidence
is retained; no unchanged self-retry is allowed.

## Defect-Shaped Handoff

First actionable item if this package cannot close: close defect
`QOBS-HOLD-LIFT-01` at the exact proven boundary; never relay only a next
inspection step.

Revision note: scaffolded on 2026-07-24 from the retained Order-3 attempt-3
failure evidence so implementation and hold-lift work share one authenticated
authority envelope.
