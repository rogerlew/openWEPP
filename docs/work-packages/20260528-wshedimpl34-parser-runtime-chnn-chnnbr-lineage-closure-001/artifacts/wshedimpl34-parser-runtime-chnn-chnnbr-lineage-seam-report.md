# WSHEDIMPL34 Parser/Runtime `chnn/chnnbr` Lineage Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser authority enforces watershed-channel Manning relation
  `chnn >= chnnbr` (`CHN-E-005` when violated).
- Runtime seed closure:
  - WS10 channel seed path now enforces explicit `chnn >= chnnbr` relation
    before runtime symbol publication,
  - violated relation fails closed with
    `WatershedRuntimeInputError::ChannelSymbolOutOfDomain` on
    `ws10_channel_{id}_chnn`.
- Lineage result:
  - watershed channel parser relation authority and WS10 runtime seed authority
    now align on explicit `chnn >= chnnbr` semantics.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_chnn_less_than_chnnbr` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_chnn_less_than_chnnbr` -> pass
