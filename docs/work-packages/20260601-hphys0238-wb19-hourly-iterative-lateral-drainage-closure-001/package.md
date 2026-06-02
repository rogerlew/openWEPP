# 20260601-hphys0238-wb19-hourly-iterative-lateral-drainage-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Implement baseline-authoritative WB19 hourly iterative substep execution for
lateral and drainage routines in openWEPP, including lane-authoritative runtime
seeding, contract-derived regression guards, and workspace gate evidence.

## Why This Package Exists
HPHYS0237 completed the bulk hourly routine inventory and identified WB19
`run_lateral_transfer` and `run_drainage` as the first required production
migration slice after WB18 iterative closure.

## Scope
### Included
- Contract amendments for WB19 hourly iterative lane authority in:
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- Contract-derived test updates in:
  - `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` (lane-seeding tests)
- Production implementation in:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Workspace gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`

### Explicitly Out of Scope
- Scheduler ordering reconciliation (`LateralTransfer`/`Drainage`/`RunoffReconciliation`).
- WB14/WB12 hourly cadence migration.
- MOFE hourly carry-array routing publication surfaces.

## Closure Measures (Required)
1. `MEASURE-HP238-001`: WB19 lateral execution uses iterative lane substeps
   with per-substep recomputation and accumulated daily `q`.
2. `MEASURE-HP238-002`: WB19 drainage execution uses iterative lane substeps
   with per-substep recomputation and accumulated daily `Qdd` under cumulative
   daily capacity cap.
3. `MEASURE-HP238-003`: Runner WB11 seed publishes WB19 lane-substep symbol
   for daily/hourly lanes.
4. `MEASURE-HP238-004`: Contract-derived tests reject non-iterative/single-pass
   regression for WB19 hourly lane behavior.
5. `MEASURE-HP238-005`: Required workspace gates pass and artifacts capture
   results truthfully.

## Deliverables
1. `artifacts/hphys0238-contract-implementation-evidence.md`
2. `artifacts/hphys0238-contract-test-implementation-evidence.md`
3. `artifacts/hphys0238-preimplementation-contract-gate.md`
4. `artifacts/hphys0238-implementation-and-test-evidence.md`
5. `artifacts/hphys0238-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0238_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Amend canonical contracts for WB19 hourly iterative lane authority.
2. Add/adjust contract-derived tests for WB19 lane behavior.
3. Record pre-implementation contract gate.
4. Modify production code.
5. Run required workspace gates.
6. Publish disposition and worker handoff.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Each artifact must label evidence class (`Static:` vs `Ran:`).

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/hphys0237_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0238-wb19-hourly-iterative-lateral-drainage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`

## Phase Plan
### Phase A - Contract authority amendment
- Amend `SC-SUBHYD-001` and `SC-WATBAL-001` for WB19 hourly iterative lane
  authority and symbol lineage.

### Phase B - Contract-derived tests + pre-implementation gate
- Add lane-iterative WB19 contract regression vectors.
- Add runner WB11 lane-seed coverage for WB19 lane symbol.
- Record contract gate prior to code edits.

### Phase C - Production implementation + gates
- Implement WB19 iterative lane loops and cumulative daily publication.
- Run required workspace gates.

### Phase D - Disposition + handoff
- Publish closure evidence and remaining HOLD follow-on queue.

## Exit Criteria
- `MEASURE-HP238-001..005` satisfied and evidenced.
- Disposition explicitly records HOLD/GO for remaining hourly migration queue.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs changes only; no credentials/network writes.
