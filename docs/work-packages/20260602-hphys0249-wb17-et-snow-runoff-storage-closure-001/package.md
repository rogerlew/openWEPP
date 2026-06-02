# HPHYS0249 WB17 ET, Snow/Runoff, and Storage Closure

Status: HOLD

This ExecPlan-style work package is a living autonomous execution document.
It follows `/workdir/openWEPP/docs/codex_exec_plans.md` and must remain
self-contained enough for a new agent to execute from kickoff through
disposition without user intervention.

## Objective

Diagnose, correct, and validate the next HPHYS residual focus: WB17
evapotranspiration component lineage (`Ep`/`Es`), snow/runoff timing
(`Snow-Water`, `RM`, `Q`), and aggregate storage (`Total-Soil`,
`SoilWaterTotal`) for the 39 single-OFE hillslope suite.

## Rationale

HPHYS0248 materially reduced the H39 early-season WB18 `Dp`/`Pe` defect but
held disposition because full-suite semantic parity remained `0/39`. The
largest remaining coupled families are WB17 `Ep`/`Es`, snowmelt/runoff timing,
and aggregate soil-water storage. Static inspection shows openWEPP WB17
currently subtracts ET from `wb11_soil_water` as a scalar aggregate while the
pinned WEPP baseline mutates legacy per-layer `st(i)` storage through
`evap.for` and `swu.for`, then recomputes `watcon = Σsoilw(i)`. This package
must close that lineage first and then rerun the full suite to quantify the
remaining snow/runoff/storage path.

## Included Scope

- Amend canonical `SC-*` authority for WB17 layer-storage ET lineage and
  coupled WB13 aggregate storage publication.
- Add contract-derived tests proving WB17 mutates per-layer storage before
  aggregate writeback for soil evaporation and root uptake.
- Diagnose H39 and full-suite `Ep`, `Es`, `Snow-Water`, `RM`, `Q`,
  `Total-Soil`, and `SoilWaterTotal` residuals before/after targeted fixes.
- Modify WB17 production code and directly coupled aggregate storage surfaces
  needed to satisfy canonical contract authority.
- Run targeted WB17 tests and the full `H1..H39` hillslope semantic suite.
- Dispatch two independent review agents, resolve actionable findings, and
  record disposition.

## Excluded Scope

- Heuristic or empirical tuning to compensate for snow/runoff timing.
- Placeholder/proxy process-physics math in production kernel/runtime
  publication paths.
- WB19 lateral-flow magnitude correction unless directly caused by WB17
  storage inputs.
- Broader management/growth migration beyond WB17 ET surfaces required for
  baseline-authoritative `evap`/`swu` execution.
- Commit, branch creation, or push operations; the user must request those
  separately.

## Deliverables

1. Canonical contract amendments for WB17 layer-storage ET lineage and WB13
   aggregate storage coupling.
2. Contract-derived Rust tests that fail before production edits and pass
   after the WB17 correction.
3. Baseline lineage map for `evap.for`, `swu.for`, and `watbal_hourly.for`.
4. Targeted production WB17 correction if contract/test evidence proves a
   defect.
5. H39 and full `H1..H39` semantic metrics for continuation planning.
6. Snow/runoff timing diagnosis with explicit HOLD follow-on scope if no
   baseline-authoritative correction is landed.
7. Dual review artifacts, recorded fixes, verification artifacts, gate results,
   owned-file manifest, worker handoff, and final disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0248-wb18-h39-dp-pe-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260602-hphys0248-wb18-h39-dp-pe-lineage-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/evap.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/snow.for`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Phase Plan

1. Scaffold the package, artifact placeholders, and kickoff prompt.
2. Map pinned baseline WB17 `evap`/`swu`/hourly ordering authority.
3. Amend canonical contracts before production edits.
4. Add contract-derived tests before production edits.
5. Record pre-implementation contract gate evidence with truthfulness labels.
6. Implement minimal baseline-authoritative WB17 layer-storage correction.
7. Run targeted Rust tests and H39 semantic validation.
8. Run the full `H1..H39` hillslope semantic suite.
9. Diagnose snow/runoff timing residuals and aggregate-storage impacts.
10. Dispatch dual review, resolve actionable findings, and update artifacts.
11. Run feasible final gates and record disposition.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Kernel production code edits before steps 1-3 are invalid package execution.

## Exit Criteria

- Canonical `SC-*` contracts cite pinned baseline lineages for WB17 `evap`,
  `swu`, layer `st(i)` mutation, and aggregate `watcon` recomputation.
- Contract-derived tests fail before the WB17 production fix and pass after it.
- H39 semantic metrics are recorded before/after for `Ep`, `Es`,
  `Snow-Water`, `RM`, `Q`, `Total-Soil`, and `SoilWaterTotal`.
- Full `H1..H39` semantic suite completes with aggregated continuation
  metrics.
- Dual review findings are dispositioned and actionable fixes are resolved.
- `cargo fmt --check`, targeted WB17 tests, workspace tests/clippy/deny when
  feasible, and `git diff --check` are recorded truthfully.
- Disposition is `GO` only if no known contract/profile violations remain;
  otherwise disposition is `HOLD` with focused continuation scope.

## Security-Impact Gate

No security-sensitive behavior is expected. This package changes deterministic
local hydrology math and documentation only. New file I/O must stay inside
explicit run/artifact paths and must not introduce shell interpolation.

## Evidence Truthfulness

All artifacts must label evidence with `Static:` when based on source or
document inspection and `Ran:` when based on an executed command. Do not claim a
workflow ran when only a build/check/static read was performed.

## Autonomy

This package is intended for no-intervention execution. Agents must execute the
phase plan end-to-end and update artifacts through disposition without asking
the user for next steps unless hard-blocked by missing required authority,
missing baseline data, or repeated tool failures.

## Final Disposition Snapshot

Status: HOLD as of 2026-06-02.

HPHYS0249 completed contract-first WB17 layer-storage correction, dual review,
review-fix resolution, final gates, and full `H1..H39` semantic validation.
The package remains in `HOLD` because semantic parity is still `0/39` and the
remaining residual families (`Ep`, snow/runoff timing, aggregate storage) are
not closed.
