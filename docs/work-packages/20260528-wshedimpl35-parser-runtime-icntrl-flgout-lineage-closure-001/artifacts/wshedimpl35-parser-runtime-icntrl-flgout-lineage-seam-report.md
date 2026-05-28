# WSHEDIMPL35 Parser/Runtime `icntrl/flgout` Lineage Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser authority enforces watershed-channel control domains:
  - `icntrl in [0,4]` (`CHN-E-004` when violated),
  - `flgout in [0,1]` (`CHN-E-004` when violated).
- Runtime seed closure:
  - WS10 channel seed path now enforces explicit `icntrl/flgout` domains
    before runtime symbol publication,
  - out-of-domain values fail closed with
    `WatershedRuntimeInputError::ChannelSymbolOutOfDomain` on
    `ws10_channel_{id}_icntrl` or `ws10_channel_{id}_flgout`.
- Lineage result:
  - watershed channel parser control authority and WS10 runtime seed authority
    now align on explicit `icntrl/flgout` domain semantics.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_icntrl_out_of_domain` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_flgout_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_icntrl` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_flgout` -> pass
