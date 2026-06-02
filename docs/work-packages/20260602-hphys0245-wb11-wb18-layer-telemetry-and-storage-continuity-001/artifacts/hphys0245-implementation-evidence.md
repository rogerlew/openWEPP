# HPHYS0245 Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Static
- Added diagnostics-only HPHYS0245 JSONL telemetry to the hillslope runner at
  `crates/openwepp-runner/src/hillslope/mod.rs`.
- The sidecar is disabled by default and only writes when
  `OPENWEPP_HPHYS0245_TRACE_PATH` is set.
- `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS` limits emitted simulation days; invalid
  values fail through the existing typed `HillslopeCliError` runtime-surface
  path.
- Trace rows use schema
  `openwepp-hphys0245-wb11-wb18-trace-v1` and include:
  - run/day identity: `run_name`, `sim_day_index`, `simulation_year`,
    `calendar_year`, `julian_day`
  - boundary identity: `boundary`, `phase`
  - WB11 aggregate storage: `wb11_soil_water_m`,
    `wb11_soil_water_mm`
  - WB18 layers: `wb18_theta_sum_m`, `wb18_theta_layers_m`,
    `wb18_pei_sum_m`, `wb18_pei_layers_m`
  - WB18/WB13 flux/publication fields: `d_m`, `pe_m`, `wb13_dp_mm`,
    `wb13_total_soil_mm`, `wb13_soil_water_total_mm`
  - reconciliation helper: `wb11_minus_theta_sum_m`
- Source evidence:
  - `crates/openwepp-runner/src/hillslope/mod.rs:250` carries the daily result
    trace rows.
  - `crates/openwepp-runner/src/hillslope/mod.rs:254` defines
    `SchedulerLifecycleContext`.
  - `crates/openwepp-runner/src/hillslope/mod.rs:266` defines
    `Hphys0245TraceConfig`.
  - `crates/openwepp-runner/src/hillslope/mod.rs:280` defines
    `Hphys0245TraceRow`.
  - `crates/openwepp-runner/src/hillslope/mod.rs:303` defines the
    phase-wrapping `Hphys0245TelemetryKernel`.
  - `crates/openwepp-runner/src/hillslope/mod.rs:360` defines the trace schema
    and environment gates.
  - `crates/openwepp-runner/src/hillslope/mod.rs:922` reads the telemetry
    configuration once per hillslope run.
  - `crates/openwepp-runner/src/hillslope/mod.rs:1072` writes the JSONL sidecar
    only when the gate is enabled.
  - `crates/openwepp-runner/src/hillslope/mod.rs:3072` gates per-day trace
    capture.
  - `crates/openwepp-runner/src/hillslope/mod.rs:3252` parses environment
    configuration.
  - `crates/openwepp-runner/src/hillslope/mod.rs:3301` writes JSONL rows.
  - `crates/openwepp-runner/src/hillslope/mod.rs:3345` builds trace rows from
    the runtime surface.
- Focused tests:
  - `hphys0245_trace_config_limits_requested_days`
  - `hphys0245_trace_row_captures_storage_and_percolation_symbols`
  - `hphys0245_trace_writer_serializes_jsonl_rows`

## Ran
- `cargo fmt --check`: pass.
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner`: pass.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.

## Production Behavior
- No production process-physics equations were changed.
- No science contracts were amended.
- Default hillslope output remains silent for HPHYS0245 telemetry unless the
  explicit environment variable is set.
