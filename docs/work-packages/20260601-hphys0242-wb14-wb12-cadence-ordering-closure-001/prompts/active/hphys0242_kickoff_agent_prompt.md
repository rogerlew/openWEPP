Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001/package.md`
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

Files:
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

Task: execute HPHYS0242 objective end-to-end by closing Dispatch Group D
WB14/WB12 cadence and infiltration/ET/runoff/storage observation ordering.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent
defaults/clamping for domain violations; no heuristic/proxy process-physics
substitutions.
Do not modify production code until canonical contracts, contract-derived tests,
and the pre-implementation contract gate are complete and recorded.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
