# Unit Governance Gap Analysis

Status: completed/HOLD
Evidence mode: Static + Ran

Static: HPHYS0276 closes the immediate governance gap that allowed raw
conversion literals in the HPHYS0272/HPHYS0269 winter radiation and snowpack
lineage. It does not close all raw conversion literals in production.

## Closed First-Wave Gaps

- HPHYS0272 radiation conversion now routes through
  `langleys_per_day_to_megajoules_per_square_meter_per_day`.
- SIMIMPL28 uniform daily-to-hourly fallback now uses a named helper.
- SIMIMPL29 melt wind, rain-inch, inch-to-meter, snow density/depth, and
  density-unit conversions now use named helpers.
- WB19 drainage `m s^-1 -> cm h^-1`, `m -> cm`, and `cm -> m` conversions now
  use named helpers.
- Release guard fails on unauthorized raw literals in first-wave enforced files.

## Remaining HOLD Gaps

Ran:
- `tools/release/check_raw_unit_conversions.py --inventory-all-production`
  produced 73 candidate findings after excluding test-only files.

Static: remaining candidate clusters are primarily:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  soil depth, conductivity, density, and percent/fraction conversions.
- `crates/openwepp-runner/src/hillslope/mod.rs` publication and ET projection
  depth conversions.
- `crates/openwepp-climate-runtime-adapter/src/lib.rs` climate
  `hours <-> seconds` and `mm -> m` parser/runtime conversions.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs` constants that need
  classification as true conversions, thresholds, or sentinels.

Disposition: follow-up package should migrate/allowlist these clusters before
expanding the guard to all production paths.
