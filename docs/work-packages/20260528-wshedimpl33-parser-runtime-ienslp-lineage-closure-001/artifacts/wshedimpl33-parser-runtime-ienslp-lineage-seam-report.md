# WSHEDIMPL33 Parser/Runtime `ienslp` Lineage Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser authority is strict `ienslp in [1,2]` (`CHN-E-004` on out-of-domain).
- Runtime seed closure:
  - WS10 channel seed path now enforces explicit `ienslp` domain guard
    (`1..=2`) before runtime symbol publication,
  - out-of-domain values fail closed with
    `WatershedRuntimeInputError::ChannelSymbolOutOfDomain` on
    `ws10_channel_{id}_ienslp`.
- Lineage result:
  - watershed channel parser domain authority and WS10 runtime seed authority
    now align on explicit `ienslp` domain semantics (`1..=2`).

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_ienslp_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ienslp` -> pass
