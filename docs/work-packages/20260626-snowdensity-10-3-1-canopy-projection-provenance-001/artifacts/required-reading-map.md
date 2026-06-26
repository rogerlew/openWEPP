# Required Reading Map

Status: complete.

Evidence class: Static.

Read:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `tests/fixtures/cancov_forest/README.md`
- `/home/workdir/wepppy/docs/work-packages/20260626_deciduous_mixed_forest_managements/package.md`
- `/home/workdir/wepppy/docs/work-packages/20260626_deciduous_mixed_forest_managements/artifacts/parameterization-research.md`
- `/home/workdir/wepppy/docs/work-packages/20260626_deciduous_mixed_forest_managements/artifacts/winter-cancov-validation.md`

Relevant source references:

- `crates/openwepp-runner/src/hillslope/snowbench.rs`: `export_pysnobal_inputs`
  extracts `primary_canopy_cover_fraction` from
  `static_parts.runtime_surface.state_surface["cancov"]`.
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`:
  `run_coe_melt_snowbench` calls `export_pysnobal_inputs` and records
  `canopy_source = "generated_openwepp_runtime_surface.cancov"`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`:
  strict runtime projection seeds `cancov` from `initial_data.base_line[1]`.

