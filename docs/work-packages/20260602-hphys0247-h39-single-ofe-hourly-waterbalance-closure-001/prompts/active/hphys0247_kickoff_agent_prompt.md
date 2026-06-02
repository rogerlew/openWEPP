# HPHYS0247 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/package.md`
sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/`
- `docs/work-packages/20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001/`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

Files:

- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/01_phase_routing.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/wb13_daily_water_balance_output_surface_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
- `tests/fixtures/constitutive/**`

Task: execute HPHYS0247 end-to-end for H39 single-OFE hourly hillslope
water-balance closure. Reproduce the current H39 hourly residual, amend
canonical `SC-*` contract authority, add contract-derived tests, record the
pre-implementation contract gate, implement baseline-authoritative production
fixes, rerun H39 hourly validation, and update all required artifacts through
disposition.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance from `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no heuristic/proxy process-physics substitutions; no comparator-tolerance
tuning as a substitute for physics migration; no production hydrology code
edits before contract and contract-test gates for the touched surface.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, package progress, gate evidence, review and
verification posture, worker handoff, and final disposition for all completed
phases.
