# WSHEDIMPL10 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Added fixture:
  - `tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp`
- Added parser contract-derived vector:
  - `strict_mode_parses_active_structure_payload_exports` in
    `tests/integration/infile_watershed_impoundment_parser_contract.rs`
  - Validates typed export surfaces for drop spillway, culverts, rockfill,
    emergency spillway, filter barrier, and perforated riser payloads.
- Updated runtime seam unit test to consume active fixture and confirm
  fail-closed behavior for unimplemented active coefficient projection:
  - `watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap`

## Ran
- `cargo test -p openwepp --test infile_watershed_impoundment_parser_contract strict_mode_parses_active_structure_payload_exports`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap`
