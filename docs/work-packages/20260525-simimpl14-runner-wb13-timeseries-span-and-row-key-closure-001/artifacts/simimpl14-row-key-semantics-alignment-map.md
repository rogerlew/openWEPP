# simimpl14-row-key-semantics-alignment-map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Row-key semantics aligned to simulation-year authority for replay overlap:
- Added simulation-year mapping helper and wired mapping into per-day row construction.
- `build_simulation_owned_wb13_row` now writes WB13 `Y` from simulation-year mapping (not calendar year).
- Added row-key provenance surface (`year`, `julian_day`, `ofe`, `sim_day_index`) for first and last emitted rows.
- `HillslopeWatRow.year` and WB13/H5 year now share simulation-year semantics.

## Ran
- Command: `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract -- --nocapture`
- Result: pass.
- Verified assertions:
- WB13 `Y` token equals `1` for both emitted fixture rows in same calendar year.
- `/wb13_publication/first_row_key/year == 1`.
