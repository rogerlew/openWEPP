# WSHEDIMPL36 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Added parser contract vectors in
  `tests/integration/infile_watershed_channel_parser_contract.rs`:
  - `strict_mode_rejects_rating_curve_rccoef_non_positive`
  - `strict_mode_rejects_rating_curve_rcoset_negative`
- Added parser fixtures:
  - `tests/fixtures/infile/watershed_channel/strict_rating_curve_rccoef_non_positive.chn`
  - `tests/fixtures/infile/watershed_channel/strict_rating_curve_rcoset_negative.chn`
- Added/expanded runtime seam vectors in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` tests:
  - `watershed_channel_runtime_seed_projects_ws10_symbols` now asserts
    `ws10_channel_{id}_{rccoef,rcexp,rcoset}` projection,
  - `watershed_channel_runtime_seed_rejects_missing_rating_curve_payload_for_icntrl4`
  - `watershed_channel_runtime_seed_rejects_rating_curve_payload_when_icntrl_not4`
  - `watershed_channel_runtime_seed_rejects_out_of_domain_rccoef`
  - `watershed_channel_runtime_seed_rejects_out_of_domain_rcexp`
  - `watershed_channel_runtime_seed_rejects_out_of_domain_rcoset`

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rccoef_non_positive` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rcoset_negative` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_` -> pass
