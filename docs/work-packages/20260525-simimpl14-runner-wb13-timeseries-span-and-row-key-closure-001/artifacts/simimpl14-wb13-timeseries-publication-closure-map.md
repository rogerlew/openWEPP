# simimpl14-wb13-timeseries-publication-closure-map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- WB13 publication now emits full-run row span, not single-day projection:
- `build_h5_wat_output` accepts full row slice and appends each row into `Wb13DailyWaterBalanceSurface`.
- `build_hillslope_wat_rows` writes one parquet row per executed day.
- Added publication provenance closure fields:
- `row_count`
- `sim_day_index_monotonic`
- `first_row_key`
- `last_row_key`
- Added typed guards for empty row spans and invalid/non-positive `sim_day_index` in publication surfaces.

## Ran
- Command: `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract -- --nocapture`
- Result: `3 passed; 0 failed`.
- Includes passing SIMIMPL14 span test asserting:
- 2 WB13 numeric rows in `H5.hbp` for 2 climate days.
- manifest `/wb13_publication/row_count == 2`.
- manifest monotonic `sim_day_index` and first/last key assertions.
