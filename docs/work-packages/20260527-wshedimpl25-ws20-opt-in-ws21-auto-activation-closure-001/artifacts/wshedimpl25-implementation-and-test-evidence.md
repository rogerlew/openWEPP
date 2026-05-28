# WSHEDIMPL25 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Runtime implementation update:
  - File: `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - Function: `assemble_incoming_sediment_load_and_capacity(...)`
  - Change:
    - Introduced `ws21_case34_opt_in` (explicit WS21 toggle read).
    - Set `ws21_case34_enabled = ws20_case12_enabled || ws21_case34_opt_in`.
  - Effect:
    - WS20-only opt-in lanes now execute through WS21 migrated branch families.
    - Residual WS20 unresolved-detachment fallback lane is closed.
    - WS20-only opt-in lanes remain fail-closed on missing `crfrac`.
- Contract-derived tests updated in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  for WS25 fail-closed and success vectors.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
- `cargo test --workspace -q` -> pass
- `cargo deny check` -> pass (non-failing duplicate/license-not-encountered
  warnings only)
