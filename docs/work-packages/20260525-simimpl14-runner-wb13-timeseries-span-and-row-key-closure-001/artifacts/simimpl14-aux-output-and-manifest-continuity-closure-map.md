# simimpl14-aux-output-and-manifest-continuity-closure-map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Manifest continuity additions in `[crates/openwepp-runner/src/lib.rs]`:
- `/execution_provenance/climate_day_count`
- `/execution_provenance/executed_day_count`
- `/wb13_publication/row_count`
- `/wb13_publication/sim_day_index_monotonic`
- `/wb13_publication/first_row_key/*`
- `/wb13_publication/last_row_key/*`
- Loss output continuity additions:
- `climate_day_count`, `executed_day_count`, `first_day_year`, `first_day_julian`, `last_day_year`, `last_day_julian`.
- Optional output payload now publishes first/last day and climate/executed day counts for span-truthful auxiliary evidence.

## Ran
- Command: `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract -- --nocapture`
- Result: pass.
- Verified loss assertions:
- `/climate_day_count == 2`
- `/executed_day_count == 2`
- `/first_day_julian == 1`
- `/last_day_julian == 2`
