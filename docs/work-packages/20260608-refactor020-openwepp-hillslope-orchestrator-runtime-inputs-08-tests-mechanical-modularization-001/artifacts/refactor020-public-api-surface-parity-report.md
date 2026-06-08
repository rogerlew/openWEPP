# REFACTOR020 Public API Surface Parity Report

Status: complete
Evidence mode: Static/Ran

Static:
- Baseline: single `#[cfg(test)] mod tests` implementation in
  `src/runtime_inputs/08_tests.rs`.
- Post-modularization: thin facade module in `08_tests.rs` that includes
  `common`, `soil`, `slope`, `management`, and `climate` test modules.
- Public symbols are unchanged for crate consumers; only test helper/test organization
  changed.
- Fixture include paths were corrected for nested module depth.

Ran:
- 2026-06-08T23:13:29Z: `cargo test -p openwepp-hillslope-orchestrator --tests` (107 passed, 0 failed), confirming test set and names remained discoverable under `runtime_inputs::tests` and `runtime_inputs::tests_mod`.

## Checks
- Preserved module exports by keeping all shared imports and helper constants/functions in
  `08_tests/common.rs`.
- Maintained test module naming and visibility consistent with pre-refactor behavior.
