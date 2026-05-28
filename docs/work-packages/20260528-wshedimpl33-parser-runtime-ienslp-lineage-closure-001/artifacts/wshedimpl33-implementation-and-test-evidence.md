# WSHEDIMPL33 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime projection update in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - added explicit WS10 channel seed guard for `ienslp` domain `1..=2`,
  - added typed `ChannelSymbolOutOfDomain` failure path for
    `ws10_channel_{id}_ienslp` when parser/runtime seam receives out-of-domain
    values.
- Parser contract surface was already authoritative for `ienslp in [1,2]`;
  WSHEDIMPL33 adds explicit parser vector coverage and runtime-seed symmetry.
- Test/fixture updates:
  - `tests/integration/infile_watershed_channel_parser_contract.rs`
  - `tests/fixtures/infile/watershed_channel/strict_ienslp_out_of_domain.chn`
  - runtime seam test in
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_ienslp_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ienslp` -> pass
