# WSHEDIMPL34 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime projection update in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - added explicit WS10 channel seed relation guard for
    `chnn >= chnnbr`,
  - added typed `ChannelSymbolOutOfDomain` failure path on
    `ws10_channel_{id}_chnn` when parser/runtime seam receives
    `chnn < chnnbr`.
- Parser contract surface was already authoritative for `chnn >= chnnbr`;
  WSHEDIMPL34 adds explicit parser vector coverage and runtime-seed symmetry.
- Test/fixture updates:
  - `tests/integration/infile_watershed_channel_parser_contract.rs`
  - `tests/fixtures/infile/watershed_channel/strict_chnn_less_than_chnnbr.chn`
  - runtime seam test in
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_chnn_less_than_chnnbr` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_chnn_less_than_chnnbr` -> pass
