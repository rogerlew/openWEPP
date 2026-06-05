# Unit Provenance Audit

Ran:

- Checked every HPHYS0299 ledger row for canonical `hrsnow` source mapping.
- Required openWEPP field: `snow_hourly_snowfall_depth_sum_m`.
- Rejected field for canonical parity: `snow_hourly_snowfall_water_equiv_sum_m`.

## Result

- Status: `PASS`; all `hrsnow` provenance rows use snowfall depth.

## Provenance

- Baseline partition call: `/workdir/wepp-forest_260430_baseline/src/winter.for:296-300`
- Baseline `stmtim` equation: `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`
- Equation: `hrsnow(hour) = rain / wntdur * 10.0`
- Comparison: `depth-vs-depth`
