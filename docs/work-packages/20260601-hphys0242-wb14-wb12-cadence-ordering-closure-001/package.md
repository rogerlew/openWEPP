# 20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: GO_HPHYS0239_FOLLOWUP_DISPATCH_GROUPS_B_C_D_CLOSED

## Objective
Close HPHYS0237 Dispatch Group D by reconciling WB14/WB12 cadence and
infiltration/ET/runoff/storage observation ordering under hourly lane mode,
using baseline-authoritative physics and explicit contract-derived tests.

## Why This Package Exists
HPHYS0237 identified WB14 runoff reconciliation cadence, WB12 storage
reconciliation cadence, and ET/read-order coupling to infiltration as remaining
hourly migration gaps. HPHYS0239 closed a handoff slice but left the stream in
`HOLD` for Groups B/C/D. After HPHYS0240 and HPHYS0241 close runoff carryover
and MOFE carry arrays, this package completes the cadence/ordering closure for
the remaining WB14/WB12 tail.

## Scope
### Included
- Contract amendments in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- Contract-derived tests in:
  - `tests/integration/wb11_hydrology_kernel_contract.rs`
  - `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
  - `tests/integration/wb12_reconciliation_kernel_contract.rs`
  - `tests/integration/wb17_et_physics_kernel_contract.rs`
  - `tests/integration/wb18_percolation_physics_kernel_contract.rs`
  - `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- Production implementation in:
  - `crates/openwepp-hillslope-orchestrator/src/phase.rs`
  - `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Workspace validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Explicitly Out of Scope
- HPHYS0240 hourly runoff carryover closure.
- HPHYS0241 MOFE hourly carry-array closure.
- Watershed/channel/erosion routing physics unrelated to WB14/WB12 cadence and
  observation ordering.
- Subhourly lane enablement beyond explicit daily/hourly lane semantics.

## Closure Measures (Required)
1. `MEASURE-HP242-001`: canonical contracts encode WB14/WB12 hourly cadence,
   same-pass observation ordering, and fail-closed guard posture.
2. `MEASURE-HP242-002`: contract-derived tests prove ET observes authoritative
   infiltration lineage and WB14/WB12 observe hourly-mutated state/flux lineage.
3. `MEASURE-HP242-003`: production code implements cadence/ordering closure
   without surrogate formulas, stale-surface reuse, or silent defaults.
4. `MEASURE-HP242-004`: scheduler dependencies and phase order reflect the
   accepted baseline-authoritative cadence contract.
5. `MEASURE-HP242-005`: required workspace gates pass and are recorded with
   truthful evidence labels.

## Deliverables
1. `artifacts/hphys0242-contract-implementation-evidence.md`
2. `artifacts/hphys0242-contract-test-implementation-evidence.md`
3. `artifacts/hphys0242-preimplementation-contract-gate.md`
4. `artifacts/hphys0242-implementation-and-test-evidence.md`
5. `artifacts/hphys0242-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0242_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Amend canonical contracts for WB14/WB12 cadence and ordering authority.
2. Add/adjust contract-derived tests.
3. Record pre-implementation contract gate.
4. Modify production code.
5. Run required workspace gates.
6. Publish disposition and worker handoff.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Each artifact must label evidence class (`Static:` vs `Ran:`).

## Physics Authority Requirements
- Canonical `SC-*` contracts are the only authority for new/changed process
  physics; package-local notes are evidence, not authority replacement.
- Physics/equation authority defaults to `/workdir/wepp-forest_260430_baseline`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Do not invent or approximate WB14/WB12 cadence or observation-ordering
  physics; every equation, constant, guard, and invariant must trace to
  canonical contract text plus provenance citations.
- Preserve legacy WEPP variable naming continuity for touched WB14/WB12/WB17
  symbols and record explicit alias mappings where openWEPP boundary names
  differ.
- If baseline-authoritative cadence/order closure is not completed, keep
  disposition in `HOLD` and open a follow-on package rather than merging
  placeholder/proxy physics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/hphys0237-hourly-routine-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/artifacts/hphys0239_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0240-hourly-runoff-carryover-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0241-mofe-hourly-carry-arrays-routing-continuity-001/package.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/evap.for`
- `/workdir/wepp-forest_260430_baseline/src/evappm.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Contract authority amendment
- Amend canonical contracts for WB14/WB12 hourly cadence, ET/infiltration
  observation ordering, and fail-closed guard requirements.

### Phase B - Contract-derived tests + pre-implementation gate
- Add tests that prove ordering/cadence closure and stale-surface rejection;
  record the pre-implementation gate before production edits.

### Phase C - Production implementation + gates
- Implement cadence/ordering closure in scheduler and hydrology runtime paths,
  then run required workspace gates.

### Phase D - Disposition + handoff
- Publish evidence, reviews, verification, disposition, and final HPHYS stream
  HOLD/GO posture for the HPHYS0239 follow-up chain.

## Exit Criteria
- `MEASURE-HP242-001..005` satisfied and evidenced.
- Disposition explicitly records whether Dispatch Group D is closed and whether
  the HPHYS0239 follow-up queue can move from `HOLD` to `GO` or must remain in
  `HOLD` with named residual blockers.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/tests/docs edits only; no credentials/network writes.
