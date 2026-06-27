# Verification Agent B

Evidence class: Static + Ran.

## Static Checks

- `git diff --check`: pass.
- `rg -n "qwet|frzftp" crates || true`: no hits.
- Source scan confirms `snowbench_coe_melt.rs` reads `canopy_series.csv` through
  `read_canopy_series`.
- Source scan confirms the stale replay call
  `group_daily_forcing(hourly, export_report.primary_canopy_cover_fraction)` is
  absent.

## Contract Checks

`tests/integration/snowdensity10_3_1a_per_day_cancov.rs` verifies:

- `contract_version: 90`;
- `cancov_daily_series`;
- `INV-SNOWFREEZE-063`;
- `OBL-SNOWFREEZE-P-038`;
- package scope markers.

## Verdict

Verified complete.
