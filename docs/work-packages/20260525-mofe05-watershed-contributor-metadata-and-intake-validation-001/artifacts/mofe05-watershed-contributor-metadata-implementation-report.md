# MOFE05 Watershed Contributor Metadata Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Implemented watershed contributor metadata intake closure in:
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`

Key behavior implemented:
- Extended `inputs.hillslopes_block[]` with additive optional
  `manifest_file` path surface.
- Added intake validation policy for contributor MOFE metadata:
  - multi-OFE contributor (`hbp.nofe > 1`) requires `manifest_file`.
  - manifest schema must be `openwepp-hillslope-run-manifest-v1`.
  - required `wb13_publication` fields must exist with valid types/values:
    - `publication_ofe_policy`
    - `contributor_ofe_count`
    - `area_policy`
    - `publication_area_m2`.
- Added consistency checks:
  - `contributor_ofe_count == hbp.nofe`.
  - `publication_ofe_policy` and `area_policy` must match MOFE04 canonical
    values.
  - `publication_area_m2` must be finite and > 0.
- Added typed intake guard IDs:
  - `CLIWAT-E-036` for missing/unreadable required metadata source.
  - `CLIWAT-E-037` for malformed/inconsistent metadata payload.

## Ran
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract mofe05 -- --nocapture`
