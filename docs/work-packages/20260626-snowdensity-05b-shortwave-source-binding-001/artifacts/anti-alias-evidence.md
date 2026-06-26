# Anti-Alias Evidence

Evidence mode: Static.

## Rejected Aliases

Rejected states for SNOWDENSITY-05B:

- Raw `Ly d^-1` values published under `winter.hourly.rad_mj_m2_####`.
- Already-`MJ m^-2 d^-1` daily radiation treated as `radly`.
- Double conversion from MJ back to Langleys and then to MJ again.
- Silent clipping of high daily or hourly radiation instead of typed failure.
- A fitted radiation scalar used to improve snowmelt residuals.
- A snow-only radiation scalar that bypasses ET/shared hydrology forcing.
- Site-specific radiation tuning against SNOTEL or frost-site observations.

## Existing Guards

Static source inspection found:

- `SC-CLIMATE-001#INV-CLIMATE-013` rejects Langley/MJ aliasing, double
  conversion, and clipping.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  uses `radly`, `radmj = radly * 0.04184`, `sunmap`, `radcur`, and `hr_tmp`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/climate.rs`
  contains focused checks for single conversion, near-isothermal `radmj/24`,
  and fail-closed high radiation.

SNOWDENSITY-05B extends those guards to the melt-modernization governance
surface without changing runtime code.
