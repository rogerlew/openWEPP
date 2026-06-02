# 20260601-hphys0240-hourly-runoff-carryover-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: COMPLETE_GROUP_B_CARRYOVER_CLOSED_HPHYS_HOLD_FOR_GROUPS_C_D

## Objective
Close the HPHYS0239 Dispatch-Group-B residual by implementing full hourly
runoff carryover authority for the WB19 -> WB14/WB12 tail, including
baseline-authoritative same-pass runoff carryover semantics and any scheduler
dependency reconciliation required for that carryover path.

## Why This Package Exists
HPHYS0239 closed WB19 -> WB12 -> WB13 handoff ordering and WB13 anti-shadow
publication for `Q`, `Ep`, `Es`, and `Er`, but explicitly left the HPHYS stream
in `HOLD` for remaining hourly runoff carryover and cadence work. HPHYS0237
identified surface drainage/runoff carryover and runoff assembly placement as
partial/not-migrated hourly gaps. This package is the next required package in
that chain.

## Scope
### Included
- Contract amendments in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- Contract-derived tests in:
  - `tests/integration/wb11_hydrology_kernel_contract.rs`
  - `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
  - `tests/integration/wb12_reconciliation_kernel_contract.rs`
- Production implementation in:
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
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
- MOFE hourly carry-array runtime surfaces (`Dispatch Group C`).
- Full WB14/WB12 cadence and ET/infiltration observation closure beyond runoff
  carryover (`Dispatch Group D`).
- Erosion or watershed routing changes not required to preserve runoff
  carryover surfaces.

## Closure Measures (Required)
1. `MEASURE-HP240-001`: canonical contracts encode baseline-authoritative
   hourly runoff carryover symbols, ordering, units, and guard posture.
2. `MEASURE-HP240-002`: contract-derived tests prove same-pass WB19/WB14/WB12
   runoff carryover consumption under hourly lane mode.
3. `MEASURE-HP240-003`: production code publishes and consumes carryover
   surfaces without stale state reuse or silent fallback/defaulting.
4. `MEASURE-HP240-004`: scheduler dependencies preserve the contract-derived
   carryover order.
5. `MEASURE-HP240-005`: required workspace gates pass and are recorded with
   truthful evidence labels.

## Deliverables
1. `artifacts/hphys0240-contract-implementation-evidence.md`
2. `artifacts/hphys0240-contract-test-implementation-evidence.md`
3. `artifacts/hphys0240-preimplementation-contract-gate.md`
4. `artifacts/hphys0240-implementation-and-test-evidence.md`
5. `artifacts/hphys0240-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0240_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Amend canonical contracts for hourly runoff carryover authority.
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
- Do not invent or approximate runoff carryover physics; every equation,
  constant, guard, and invariant must trace to canonical contract text plus
  provenance citations.
- Preserve legacy WEPP variable naming continuity for carryover symbols and
  record explicit alias mappings where openWEPP boundary names differ.
- If baseline-authoritative carryover closure is not completed, keep
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/hphys0237-hourly-routine-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/artifacts/hphys0239_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0240-hourly-runoff-carryover-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Contract authority amendment
- Amend canonical contracts for hourly runoff carryover symbols, ordering,
  units, and fail-closed guard posture.

### Phase B - Contract-derived tests + pre-implementation gate
- Add tests for carryover order, stale-surface rejection, and scheduler
  dependency closure; record the pre-implementation gate before code edits.

### Phase C - Production implementation + gates
- Implement baseline-authoritative carryover publication/consumption and run
  required workspace gates.

### Phase D - Disposition + handoff
- Publish closure evidence, dual review/verification artifacts, disposition,
  and next handoff for Dispatch Group C.

## Exit Criteria
- `MEASURE-HP240-001..005` satisfied and evidenced.
- Disposition explicitly records whether Dispatch Group B residual is closed
  and whether HPHYS stream remains `HOLD` for Groups C/D.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/tests/docs edits only; no credentials/network writes.
