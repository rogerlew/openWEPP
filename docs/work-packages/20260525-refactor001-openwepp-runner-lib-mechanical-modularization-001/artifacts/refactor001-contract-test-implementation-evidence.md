# REFACTOR001 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Layout-coupled contract test was updated to remain contract-accurate after module split.

Implemented test update:
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - replaced brittle single-file coupling (`include_str!(".../src/lib.rs")` expectation) with module-tree aware source scan helpers:
    - `runner_src_tree_contains(needle: &str) -> bool`
    - `source_tree_contains_rs(root: &Path, needle: &str) -> bool`
  - assertion `cli03_runner_crate_wires_output_surface_dependency` now verifies runner source-tree wiring independent of `lib.rs` monolith residency.

Contract intent preserved:
- test still enforces existence of output-surface runner wiring,
- test no longer encodes an invalid architecture assumption that all code must remain in one file.

## Ran
1. `cargo test -p openwepp-runner --tests`
   - result: pass
2. `cargo test --workspace`
   - result: pass
