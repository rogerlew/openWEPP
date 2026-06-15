# Public API Surface Parity Report

Status: complete
Evidence mode: Static

Checked files:

- `crates/openwepp-input-contract/src/parsers/hbp/mod.rs`
- `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`

Result: no public HBP parser API change.

Stable public entrypoints remain in `hbp/mod.rs`:

- `parse_hbp_from_bytes`
- `parse_hbp_from_bytes_with_latest_event_payload`
- `parse_hbp_from_path`
- `parse_hbp_from_path_with_latest_event_payload`

`parse_layout` remains crate-private to the HBP parser module as `pub(super) fn parse_layout(data: &[u8]) -> Result<Layout, HbpParseError>`.
