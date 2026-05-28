# WSHEDIMPL32 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser updates in
  `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`:
  - strict-mode `ishape` domain now accepts `1..=3`,
  - compatibility mode normalizes out-of-range positive classes to
    naturally eroded class (`ishape=3`) with explicit warning continuity.
- Runtime projection updates in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - added explicit WS10 `ishape` domain guard at seed boundary
    (`1..=3` only),
  - preserved typed `ChannelSymbolOutOfDomain` failure path for out-of-domain
    runtime projection.
- Refactored repeated symbol projection in
  `seed_watershed_runtime_surface_from_watershed_channel` into a table-driven
  loop to satisfy `clippy -D warnings` (`too_many_lines`) without changing
  symbol coverage.
- Test/fixture updates:
  - parser integration contract vector updates in
    `tests/integration/infile_watershed_channel_parser_contract.rs`,
  - runtime seam vectors added in
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`,
  - fixture updates in
    `tests/fixtures/infile/watershed_channel/compat_ishape_normalized.chn` and
    `tests/fixtures/infile/watershed_channel/strict_ishape_naturally_eroded.chn`.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_projects_naturally_eroded_ishape` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ishape` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl3` -> pass
