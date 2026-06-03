Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/package.md`
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
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

Files:
- `docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `Cargo.toml`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
- `tests/integration/hphys0256_wb19_latqcc_lane_branch_contract.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/erod13_wave1_core_kernel_contract.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `tests/integration/irrig10_irrigation_runtime_kernel_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb16_peak_runoff_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`

Task: execute package objective end-to-end for declared WB19 `latqcc` lateral
lane-branch scope.

Constraints: contract-first sequencing; canonical SC authority; pinned
baseline provenance at `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no heuristic lateral-flux damping or storage compensation.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including review artifacts, verification artifacts, targeted H1/H7/H39
diagnostics, full `H1..H39` metrics, and HOLD/GO disposition.
