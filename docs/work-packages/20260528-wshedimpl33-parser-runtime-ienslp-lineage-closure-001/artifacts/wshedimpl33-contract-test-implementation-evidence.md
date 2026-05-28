# WSHEDIMPL33 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Added parser contract vector in
  `tests/integration/infile_watershed_channel_parser_contract.rs`:
  - `strict_mode_rejects_ienslp_out_of_domain`
- Added parser fixture:
  - `tests/fixtures/infile/watershed_channel/strict_ienslp_out_of_domain.chn`
- Added runtime seam vector in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` tests:
  - `watershed_channel_runtime_seed_rejects_out_of_domain_ienslp`

## Ran
- `cargo test --test infile_watershed_channel_parser_contract` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ienslp` -> pass
