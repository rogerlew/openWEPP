# WSHEDIMPL35 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Added parser contract vector in
  `tests/integration/infile_watershed_channel_parser_contract.rs`:
  - `strict_mode_rejects_icntrl_out_of_domain`
  - `strict_mode_rejects_flgout_out_of_domain`
- Added parser fixtures:
  - `tests/fixtures/infile/watershed_channel/strict_icntrl_out_of_domain.chn`
  - `tests/fixtures/infile/watershed_channel/strict_flgout_out_of_domain.chn`
- Added runtime seam vector in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` tests:
  - `watershed_channel_runtime_seed_rejects_out_of_domain_icntrl`
  - `watershed_channel_runtime_seed_rejects_out_of_domain_flgout`

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_icntrl_out_of_domain` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_flgout_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_icntrl` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_flgout` -> pass
