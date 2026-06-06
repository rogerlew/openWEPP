# OpenWEPP Stmtim Trace Instrumentation

Status: complete

Evidence mode: Static

Static:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  now returns `Simimpl28StmtimHourlyPartition` from
  `simimpl28_stmtim_hourly_partition`.
- SIMIMPL28 publishes these per-hour diagnostic aliases:
  `snow.hourly.stmtim.rain_m_####`,
  `snow.hourly.stmtim.stmdur_s_####`,
  `snow.hourly.stmtim.wntdur_h_####`,
  `snow.hourly.stmtim.wnttim_h_####`,
  `snow.hourly.stmtim.hrtemp_c_####`,
  `snow.hourly.stmtim.rst_c_####`,
  `snow.hourly.stmtim.hrrain_m_####`,
  `snow.hourly.stmtim.hrsnow_m_####`,
  `snow.hourly.stmtim.active_interval_####`,
  `snow.hourly.stmtim.rain_branch_####`, and
  `snow.hourly.stmtim.snow_branch_####`.
- Existing `snow.hourly.rain_m_####` and `snow.hourly.snowfall_m_####` values
  are sourced from the same `partition.hrrain_m` and `partition.hrsnow_m`
  fields, so HPHYS0318 is diagnostic instrumentation only.
- `crates/openwepp-runner/src/hillslope/mod.rs` bumped the HPHYS0245 trace
  schema to `v17` and serializes matching `snow_hourly_stmtim_*` maps.
