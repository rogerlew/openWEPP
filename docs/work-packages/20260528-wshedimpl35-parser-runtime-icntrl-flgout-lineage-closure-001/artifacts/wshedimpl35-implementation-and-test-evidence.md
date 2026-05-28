# WSHEDIMPL35 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime projection update in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - added explicit WS10 channel seed domain guards for `icntrl` and `flgout`,
  - added projection of `ws10_channel_{id}_icntrl` and
    `ws10_channel_{id}_flgout`,
  - added typed `ChannelSymbolOutOfDomain` failure paths when parser/runtime
    seam receives out-of-domain `icntrl` or `flgout`.
- Parser contract surface was already authoritative for `icntrl` and `flgout`
  domain constraints; WSHEDIMPL35 adds explicit parser vector coverage and
  runtime-seed symmetry.
- Test/fixture updates:
  - `tests/integration/infile_watershed_channel_parser_contract.rs`
  - `tests/fixtures/infile/watershed_channel/strict_icntrl_out_of_domain.chn`
  - `tests/fixtures/infile/watershed_channel/strict_flgout_out_of_domain.chn`
  - runtime seam test in
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_icntrl_out_of_domain` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_flgout_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_icntrl` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_flgout` -> pass
