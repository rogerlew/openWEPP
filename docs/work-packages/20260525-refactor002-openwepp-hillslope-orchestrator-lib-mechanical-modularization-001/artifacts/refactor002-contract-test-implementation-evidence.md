# REFACTOR002 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Layout-coupled contract test was updated to remain contract-accurate after module split.

Implemented test update:
- `tests/integration/arch22_typed_state_surface_contract.rs`
  - replaced direct single-file hillslope orchestrator source read with module-tree aware `.rs` source scan helper:
    - `source_tree_contains_rs(root: &Path, needle: &str) -> bool`
  - `arch22_hillslope_guard_accessor_signature_is_typed` now validates typed guard accessor signatures across `crates/openwepp-hillslope-orchestrator/src/**/*.rs`.

Contract intent preserved:
- test still enforces typed guard accessor signature expectations,
- test no longer assumes all implementation resides in one file.

## Ran
1. `cargo test --test arch22_typed_state_surface_contract`
   - result: pass (`6 passed; 0 failed`)
2. `cargo test --workspace`
   - result: pass
