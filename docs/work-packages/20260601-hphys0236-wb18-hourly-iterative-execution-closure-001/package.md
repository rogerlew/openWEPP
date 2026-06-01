# 20260601-hphys0236-wb18-hourly-iterative-execution-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0235 by implementing
baseline-authoritative WB18 hourly iterative percolation execution semantics
(`24` substeps/day) in production kernel code, then rerunning `unpalatable-rind`
(`H1..H39`) to re-adjudicate monitored residual families.

## Why This Package Exists
HPHYS0235 isolated the persistent `Dp` mismatch root cause to hourly lane
execution shape: openWEPP used divisor-only single-pass behavior while baseline
`ui_run=1` is iterative via `watbal_hourly` + `purk`. This package lands the
first production migration slice for that gap.

## Scope
### Included
- WB18 kernel implementation update in:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- Contract-derived integration test update in:
  - `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- Full workspace gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- `unpalatable-rind` `H1..H39` rerun + semantic comparator readjudication
  artifacts.
- Package artifacts/disposition/handoff updates.

### Explicitly Out of Scope
- Full hourly forcing distribution migration across non-WB18 families
  (for example WB14/WB12 substep forcing slicing).
- Watershed routing and non-hydrology workstreams.
- Heuristic/proxy process-physics substitutions.

## Closure Measures (Required)
1. `MEASURE-HP236-001`: WB18 hourly lane executes explicit iterative substeps
   with per-substep recomputation and accumulated daily `D`/`Pe` publication.
2. `MEASURE-HP236-002`: contract-derived test coverage rejects divisor-only
   single-pass regression for hourly lane.
3. `MEASURE-HP236-003`: required workspace gates pass.
4. `MEASURE-HP236-004`: `H1..H39` rerun + semantic comparison coverage is
   complete (`39/39` execution, `39/39` semantic reports).
5. `MEASURE-HP236-005`: disposition explicitly adjudicates monitored-family
   deltas and records next implementation slice.

## Deliverables
1. `artifacts/hphys0236-contract-implementation-evidence.md`
2. `artifacts/hphys0236-contract-test-implementation-evidence.md`
3. `artifacts/hphys0236-preimplementation-contract-gate.md`
4. `artifacts/hphys0236-implementation-and-test-evidence.md`
5. `artifacts/hphys0236-residual-authority-gap-matrix.md`
6. `artifacts/hphys0236-h1-transient-lane-diagnostic.md`
7. `artifacts/hphys0236-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0236_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Confirm canonical contract authority surfaces are sufficient (`SC-PERC-001`
   v19 / `SC-WATBAL-001` v66).
2. Implement contract-derived test updates.
3. Record pre-implementation contract gate.
4. Modify production kernel code.
5. Run workspace gates and rerun/readjudication evidence.
6. Publish disposition and next-action handoff.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/artifacts/hphys0235_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`

## Phase Plan
### Phase A - Intake and authority gate
- Confirm scope from HPHYS0235 handoff.
- Verify canonical `SC-*` authority already encodes required hourly iterative
  behavior.

### Phase B - Contract-derived tests
- Replace divisor-only hourly assertion with iterative recompute regression
  vector.
- Record pre-implementation contract gate.

### Phase C - Production implementation + gates
- Implement iterative hourly WB18 substep loop.
- Run required workspace gates.

### Phase D - Rerun/readjudication + disposition
- Execute `H1..H39` rerun and semantic comparator.
- Publish monitored-family delta matrix and stream disposition with next
  actions.

## Exit Criteria
- `MEASURE-HP236-001..005` satisfied and evidenced.
- Stream-level `HOLD`/`GO` decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs changes only; no credentials/network writes.
