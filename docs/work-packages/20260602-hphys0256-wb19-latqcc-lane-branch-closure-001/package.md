# 20260602-hphys0256-wb19-latqcc-lane-branch-closure-001

Status: completed/HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0254 closed WB11 normalized seed projection enough to remove seed-state
ambiguity from the H1/H7/H39 day-1 residuals. The remaining targeted residuals
are dominated by WB19 `latqcc` (`H1 +0.595319 mm`, `H7 +1.469954 mm`,
`H39 +8.733643 mm`). This package executes the HPHYS0254 continuation by
making WB19 lateral-transfer lane authority explicit and correcting any proven
daily-vs-hourly lateral branch mismatch.

## Objective

Diagnose, correct, and validate WB19 lateral-transfer `latqcc` lineage after
corrected WB11 seed projection. The primary closure target is baseline
authority for daily `watbal.for` lateral behavior versus hourly
`watbal_hourly.for` behavior, including frozen-adjusted availability and
conductivity weighting semantics.

## Rationale

HPHYS0247/HPHYS0252 intentionally migrated hourly WB19 lateral capacity and
frozen-adjusted storage authority from `watbal_hourly.for`. HPHYS0254 then
showed remaining day-1 H1/H7/H39 deficits are post-seed process terms,
especially `latqcc`. Static baseline review indicates openWEPP may apply
hourly `meblfc`/`drfc` lateral selection to daily lanes, while baseline
`watbal.for` uses a different daily law with `fzdrfc`, `fzul`, and `hk`
weighting. That mismatch can materially alter daily lateral withdrawals.

## Included Scope

- Amend canonical `SC-SUBHYD-001` and `SC-WATBAL-001` for proven WB19
  daily/hourly lateral lane semantics.
- Add contract-derived tests before production edits.
- Diagnose pinned baseline lateral blocks in
  `/workdir/wepp-forest_260430_baseline/src/watbal.for` and
  `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`.
- Correct only baseline-authoritative WB19 lateral-transfer behavior needed for
  daily-vs-hourly lane parity.
- Run targeted H1/H7/H39 diagnostics and a fresh full `H1..H39` semantic metric
  snapshot.
- Record review, verification, gates, and final disposition.

## Excluded Scope

- No heuristic `latqcc` damping or storage compensation.
- No WB17 Ep/root uptake, snow/runoff timing, WB18 Dp/Pe, WB12, or WB13
  publication changes unless a proven WB19 lateral contract requires a narrow
  diagnostic hook.
- No per-OFE dynamic hydrology-state migration.
- No watershed rerun unless separately authorized.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Canonical contract amendments for WB19 lateral lane authority.
3. Contract-derived tests that fail before the production correction and pass
   after it.
4. Minimal production correction for WB19 lateral daily/hourly branch behavior,
   if diagnosis proves a defect.
5. Targeted H1/H7/H39 `latqcc` diagnostics.
6. Full `H1..H39` semantic metrics and continuation disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/targeted-h1-h7-h39-diagnostics.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepppy/.venv/bin/python`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

## Intended Write Set

- `docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `Cargo.toml`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
- `tests/integration/hphys0256_wb19_latqcc_lane_branch_contract.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/erod13_wave1_core_kernel_contract.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `tests/integration/irrig10_irrigation_runtime_kernel_contract.rs`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb15_canopy_interception_kernel_contract.rs`
- `tests/integration/wb16_peak_runoff_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No production code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — Baseline Lateral Diagnosis

Read the required contracts, HPHYS0254 evidence, and pinned baseline
`watbal.for`/`watbal_hourly.for` lateral blocks. Record daily and hourly WB19
lateral branch semantics in `artifacts/wb19-latqcc-lane-branch-diagnosis.md`.

### Phase B — Contract and Test Gate

Amend canonical contracts for proven lane-specific semantics. Add
contract-derived tests that demonstrate the daily/hourly distinction. Run the
targeted test before production edits and record the expected failing evidence
in `artifacts/pre-implementation-contract-gate.md`.

### Phase C — Production Correction

Apply the minimal WB19 lateral branch correction. Preserve typed fail-closed
guards; do not introduce silent defaults or heuristic flux scaling. Run
targeted tests and update `artifacts/implementation-test-evidence.md`.

### Phase D — Metrics, Review, and Disposition

Run targeted H1/H7/H39 diagnostics and the full `H1..H39` semantic suite.
Record metrics, review artifacts, verification artifacts, gate results, and
final `HOLD`/`GO` disposition.

## Exit Criteria

- Contracts and tests cover implemented behavior.
- Production changes trace to pinned baseline source, not heuristic math.
- H1/H7/H39 `latqcc` evidence is recorded.
- Full `H1..H39` semantic metrics exist or not-run rationale is explicit.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- Full Rust gates are run or explicitly marked not-run with rationale.
- Disposition is truthful; unresolved semantic parity remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-02) Scaffolded HPHYS0256 from HPHYS0254 continuation.
- [x] (2026-06-02) Completed WB19 daily/hourly lateral diagnosis.
- [x] (2026-06-02) Amended contracts and added red contract-derived tests.
- [x] (2026-06-02) Implemented production correction authorized by diagnosis.
- [x] (2026-06-02) Ran targeted and full-suite metric snapshots.
- [x] (2026-06-02) Recorded review, verification, disposition, and handoff.

## Surprises & Discoveries

- Static baseline review before scaffolding indicates daily `watbal.for` and
  hourly `watbal_hourly.for` do not use the same lateral conductivity law.
- Ran: full H1..H39 diagnostics selected the hourly lane
  (`selected_lane: hourly`, `substep_count: 24`), so daily-lane correction did
  not change the continuation metrics.
- Static: several older integration fixtures were implicitly asserting hourly
  WB19 behavior through the previous daily default; those fixtures now either
  set `wb19_lateral_drain_lane_substeps=24` or use corrected daily storage
  expectations.

## Decision Log

- Decision: Scope HPHYS0256 to WB19 lateral lane authority.
  Rationale: HPHYS0254 closed seed projection and named `latqcc` as the next
  dominant targeted residual.
  Date/Author: 2026-06-02 / Codex.
- Decision: Keep disposition in HOLD after implementation.
  Rationale: daily lane authority is corrected and gated, but the H1/H7/H39
  and full-suite evidence exercised hourly mode and retained the HPHYS0254
  residuals; closing overall `latqcc` parity would be false.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- Completed contract-first implementation of baseline-authoritative daily
  WB19 lateral branch behavior while preserving hourly WB19 branch behavior.
- Full repository gates pass. `cargo deny check` exits 0 with existing duplicate
  crate and unmatched-license allowance warnings.
- Disposition is `completed/HOLD`: next continuation should focus hourly WB19
  lateral residual lineage because the authoritative H1/H7/H39 suite remains
  on the hourly lane and metrics are unchanged.
