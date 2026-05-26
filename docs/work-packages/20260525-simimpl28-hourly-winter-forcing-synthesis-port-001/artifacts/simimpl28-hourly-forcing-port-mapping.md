# SIMIMPL28 Hourly Forcing Port Mapping

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Baseline lineage mapped into runtime seam synthesis helpers in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`:
  - `aspect.for` lineage -> `simimpl28_aspect_geometry`
  - `sunmap.for` lineage -> `simimpl28_sunmap`
  - `radcur.for` lineage -> `simimpl28_radcur`
  - `hr_tmp.for` / `hrtmp.for` lineage -> `simimpl28_hr_tmp_hour` /
    `simimpl28_hrtmp`
  - `stmtim.for` lineage -> `simimpl28_stmtim_hourly_partition`
- Emission occurs only when active winter context is detected via
  `snow.options.snow_file_present` or `frost.options.frost_file_present`.
- Required context symbols (`avgslp`, `azm`, `rst`) are enforced with typed
  runtime errors, not silent defaults.

## Ran
- `rg -n "simimpl28_aspect_geometry|simimpl28_sunmap|simimpl28_radcur|simimpl28_hr_tmp_hour|simimpl28_stmtim_hourly_partition|build_simimpl28_hourly_winter_forcing_symbols" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `rg -n "MissingRuntimeContextSymbol|RuntimeContextSymbolOutOfRange|InvalidCalendarDate" crates/openwepp-climate-runtime-adapter/src/lib.rs`
