# Snow Carry Source-Line Parity Source Lineage

Status: complete

Evidence mode: static

Static:

- `/workdir/wepp-forest_260430_baseline/src/winter.for:193` copies
  `snodpy` into `snodpt` at winter day start.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53` initializes
  hourly `snodep`/`snodpt`/`densgy`/`densgt` from carried
  `snodpy`/`densg`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:122-139` applies
  density-settling equations using `wdayct` and the density cap.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312` writes
  updated `snodpt`, `snodpy`, and `densg` after each hour.
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1361,1466` and
  `/workdir/wepp-forest_260430_baseline/src/inidat.for:383` ground
  initial `snodpy`/`densg` provenance.
- `SC-INFILE-MANAGEMENT-001` maps canonical `snodpy` to
  `management.initial[i].params.snodpy_m`; snow sidecar docs confirm
  initial snow depth is management-owned, not `snow.txt`-owned.
- openWEPP runtime aliases are `snow.runtime_swe`,
  `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, and
  `snow.runtime_settle_day_count`.

## Verified Source Requirements

- `03_kernel_support_00_support_helpers.rs:3690-3790`: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3691, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3746, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3747, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3748`
- `03_kernel_support_00_support_helpers.rs:3872-3912`: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3872, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3901, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3911`
- `03_kernel_support_00_support_helpers.rs:4218-4227`: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4223, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4224, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4225, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4226`
- `03_kernel_support_01_kernel_phases.rs:4216-4235`: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:4218, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:4225, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:4231`
- `SC-INFILE-MANAGEMENT-001:201`: `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:201`
- `infile.for:1361,1466`: `/workdir/wepp-forest_260430_baseline/src/infile.for:1361, /workdir/wepp-forest_260430_baseline/src/infile.for:1466`
- `inidat.for:383`: `/workdir/wepp-forest_260430_baseline/src/inidat.for:383`
- `runtime_inputs/04_snow_frost_irrigation.rs:663-691`: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs:663, crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs:672, crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs:681, crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs:690`
- `snowd.for:122-139`: `/workdir/wepp-forest_260430_baseline/src/snowd.for:122, /workdir/wepp-forest_260430_baseline/src/snowd.for:125, /workdir/wepp-forest_260430_baseline/src/snowd.for:129, /workdir/wepp-forest_260430_baseline/src/snowd.for:131, /workdir/wepp-forest_260430_baseline/src/snowd.for:135, /workdir/wepp-forest_260430_baseline/src/snowd.for:139`
- `snowd.for:303-312`: `/workdir/wepp-forest_260430_baseline/src/snowd.for:310, /workdir/wepp-forest_260430_baseline/src/snowd.for:311, /workdir/wepp-forest_260430_baseline/src/snowd.for:312`
- `snowd.for:50-53`: `/workdir/wepp-forest_260430_baseline/src/snowd.for:50, /workdir/wepp-forest_260430_baseline/src/snowd.for:51, /workdir/wepp-forest_260430_baseline/src/snowd.for:52, /workdir/wepp-forest_260430_baseline/src/snowd.for:53`
- `winter.for:193`: `/workdir/wepp-forest_260430_baseline/src/winter.for:193`
