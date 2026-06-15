# Public API Surface Parity Report

Static: production changes add only private items in `watershed_wat.rs`:

- `WatBatchColumns`
- `WatIdentityColumns`
- `WatValueColumns`
- `read_wat_file_row`
- `positive_area_m2`
- `day_key_from_columns`
- `wat_values_from_columns`

Static: the public entry points and error type remain unchanged:

- `pub fn build_watershed_daily_rows_from_wat<I, P>(...)`
- `pub enum WatershedWatPublicationError`

Static: no dependency, crate manifest, public module export, parquet writer
schema, or science-contract authority file was changed.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace` passed after the refactor.

Disposition: public API parity preserved.
