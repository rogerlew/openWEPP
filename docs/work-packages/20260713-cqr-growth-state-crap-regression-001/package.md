# Growth-State CRAP Regression Closure

Status: `COMPLETE`

Package id: `20260713-cqr-growth-state-crap-regression-001`

Date: `2026-07-13`

Execution mode: `package-end-to-end`

## Objective

Restore the completed CQR campaign's empty actionable CRAP set after the new
adjudicated closure gate found
`DirectGrowthInputs::compute_equation_growth_state` at CRAP `31.01620054282776`.
Reduce the function to CRAP at most 30 through a behavior-preserving cohesive
helper extraction while retaining the exact SC-PLANT-001 perennial root-cap
ordering and floating-point operation order.

## Authority

- Explicit user direction on 2026-07-13 to implement the adjudicated CRAP
  closure gate and disposition its findings.
- `docs/decisions/0021-module-coverage-closure-thresholds.md`: production CRAP
  must be at most 30 unless an exact accepted adjudication applies.
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`, especially
  algorithm steps 12-13 and `INV-PLANT-027`.
- `docs/work-packages/20260701-kernel-boundary-cqr-row6-growth-decomposition-001/`:
  prior behavior-preserving growth CQR precedent.
- The fresh full-workspace artifact at
  `/tmp/openwepp-acrap-live-20260713/workspace-crap.json`.

This is a user-directed follow-on package, not a nightly batch package. No Git
commit or branch operation is authorized; the complete scaffold is established
before the production edit and the final diff remains available for human
review.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`;
- one private helper extraction for the root mass/root depth candidate block;
- existing focused characterization tests for zero-cap and ordinary annual and
  perennial growth behavior;
- package-local evidence, dual independent review and verification, and work
  package catalog updates.

Out of scope:

- Any equation, branch-condition, validation, error, threshold, tolerance,
  public API, serialization, or science-contract change;
- New CRAP exception or reclassification;
- Test-only coverage inflation or deletion of assertions;
- Opportunistic cleanup outside the root-update block;
- Any preexisting dirty file outside this package's intended write set.

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260713-cqr-growth-state-crap-regression-001/**`

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes the already
dispatched `comparator_suite_runner` and two independent reviewer/verifier
agents. The runner may write only `artifacts/heavy-run.md`. Reviewer A may write
only `artifacts/review-a.md` and `artifacts/verification-a.md`; Reviewer B may
write only `artifacts/review-b.md` and `artifacts/verification-b.md`. Reviewers
must not read each other's work before submitting their initial findings.

The runner is required for fresh full-workspace CRAP and full closure gates.
Both reviewers are required to assess behavior identity, contract fidelity,
metric closure, gate integration, and evidence truthfulness.

## Plan

1. Preserve the fresh failing CRAP row and current-source provenance.
2. Confirm existing tests characterize the affected root-cap and ordinary
   growth branches.
3. Move the root mass/root depth candidate block unchanged into one private
   helper, preserving statement and expression order.
4. Run formatting and focused growth tests.
5. Delegate fresh full-workspace CRAP and required Rust closure gates.
6. Complete dual independent review and verification, disposition every
   finding, and rerun any affected gate.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `CQR-GR-001` | The exact raw row and source provenance are recorded. |
| `CQR-GR-002` | Existing characterization covers zero-cap perennial, positive-cap perennial, and annual root paths; no test is weakened or deleted. |
| `CQR-GR-003` | The extraction preserves branch order, expression grouping, error order, and all state outputs. |
| `CQR-GR-004` | Fresh full-workspace CRAP reports zero actionable rows and the target function is at most 30. |
| `CQR-GR-005` | Focused growth tests and required workspace Rust gates pass on terminal source. |
| `CQR-GR-006` | Dual review and dual verification complete, with every finding dispositioned and accepted fixes reverified. |
| `CQR-GR-007` | The touched Rust file remains below the 2,000-line warning threshold. |

Every criterion must be `PASS` before this package may close.

## Progress

- [x] (2026-07-13) Fresh gate found one actionable row at CRAP `31.0162`.
- [x] (2026-07-13) Package scaffold and authority map completed before the
  production edit.
- [x] (2026-07-13) Implement behavior-preserving helper extraction; focused
  crate verification passes 405/405.
- [x] (2026-07-13) Terminal heavy verification passes: CRAP `27.015625`,
  workspace `2/2/0`, and full Nextest `1,960/1,960`.
- [x] (2026-07-13) Both independent reviews accept the extraction without an
  implementation finding.
- [x] (2026-07-14) Complete shared gate-finding post-disposition verification;
  both independent reviewers return `PASS` and lift their HOLDs.

## Decision Log

- Decision: do not adjudicate or waive the row.
  Rationale: it is ordinary hand-authored scientific control flow and therefore
  remains eligible under ADR-0021.
  Date/Author: 2026-07-13 / Codex.
- Decision: use a private cohesive helper extraction rather than alter coverage.
  Rationale: fresh coverage is already `97.43589743589743%`; the excess is
  cyclomatic complexity introduced by the required root-cap branch.
  Date/Author: 2026-07-13 / Codex.

## Recovery And Idempotence

The implementation is one extraction. If focused behavior changes, restore the
block to its original in-function location without touching unrelated dirty
files and leave this package in `HOLD` with the failed evidence.

## Outcomes And Retrospective

The package closes `PASS`. One private cohesive extraction reduced
`compute_equation_growth_state` from CRAP `31.0162` to `27.015625`; its helper
is CRAP `5` at full coverage. The source remains `1,668` lines and preserves
SC-PLANT-001 `INV-PLANT-027`, branch and error order, expression grouping, and
all published state. Focused crate tests pass `405/405`, the binding full
Nextest lane passes `1,960/1,960`, format/Clippy/deny pass, the final workspace
census is `2/2/0`, and both independent reviewers return `PASS`.
