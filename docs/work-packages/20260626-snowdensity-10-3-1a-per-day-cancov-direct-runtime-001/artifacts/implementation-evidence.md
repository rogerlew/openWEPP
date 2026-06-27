# Implementation Evidence

Evidence class: Static + Ran.

## Contract Authority

`SC-SNOWFREEZE-001` is amended to v90 with:

- `cancov_daily_series` variable authority.
- `INV-SNOWFREEZE-063`, requiring snowbench/CoE melt replay to consume
  direct-production per-day canopy when available.
- `OBL-SNOWFREEZE-P-038`, requiring low-canopy/mixed/deciduous adjudication
  packages to publish source, row count, date alignment, summary statistics,
  and fail-closed validation.
- `SNOWDENSITY-10.3.1a Per-Day Cancov Direct-Runtime Addendum`.

## Runtime Surface

`DirectPublicationDayInput` now carries:

```rust
pub canopy_cover_fraction: Option<f64>
```

`DirectFrameExecutor::apply_publication_day_input` validates this value when
present:

- finite;
- in `[0, 1]`;
- typed `DirectDomainViolation` on violation.

The value is evidence only at the executor boundary. It does not alter executor
state because the snow partition has already consumed the same canopy value in
the day-input builder.

## Direct Production Source

`DirectProductionDayInputBuilder::build` sets:

```rust
day_input.canopy_cover_fraction =
    Some(growth_state_for_publication.canopy_cover_fraction);
```

This is the same `growth_state_for_publication` used immediately before:

- `snow_liquid_partition(..., growth_state_for_publication.canopy_cover_fraction, ...)`;
- direct canopy interception inputs.

No separate diagnostic canopy model was introduced.

## Snowbench Export

`export_pysnobal_inputs` now uses `build_static_hillslope_runtime_setup`, then
runs a direct-production publication capture with
`run_publication_capture_with_interleaved_day_inputs` to record lane-0
day-input canopy values.

Export sidecar:

```text
canopy_series.csv
date,day_index,canopy_cover_fraction,source
```

Report additions:

- `canopy_source`
- `canopy_series_path`
- `canopy_series_summary.day_count`
- `canopy_series_summary.min`
- `canopy_series_summary.max`
- `canopy_series_summary.mean`
- `canopy_series_summary.first`
- `canopy_series_summary.last`
- `canopy_series_summary.dynamic`

`primary_canopy_cover_fraction` remains as a backward-compatible scalar
summary/initial runtime-surface value, not as seasonal-canopy authority.

## CoE Melt Replay

`openwepp-snowbench coe-melt` now reads `canopy_series.csv` and groups forcing
by date with the matched daily canopy value.

Fail-closed validation covers:

- missing header;
- unexpected header;
- wrong column count;
- non-numeric day index;
- non-contiguous day index;
- non-finite canopy;
- canopy outside `[0, 1]`;
- duplicated date;
- empty sidecar;
- length mismatch against forcing dates;
- missing date during CoE grouping.

`CoeMeltReport.constants.canopy_cover_fraction` is now the series mean for
backward-compatible summary use, and the report publishes the full sidecar path
and summary.

## Schema Compatibility

The package intentionally did not change:

- PySnobal `forcing.csv` header;
- CoE `coe_melt_snow.csv` header;
- production WAT/HBP/PASS/loss/manifest schemas.

`snowdensity06b_coe_bound_density_replay` passed in the workspace gate,
proving the CoE-bound density replay still reads `coe_melt_snow.csv`.

## Tests

Added `tests/integration/snowdensity10_3_1a_per_day_cancov.rs` and registered it
in `Cargo.toml`.

Updated:

- `tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs`
- `tests/integration/snowdensity05g_harness_fidelity_rerun.rs`
- snow-density contract-version guards from v89 to v90.

Ran:

- focused package and G0 export tests;
- full workspace tests.
