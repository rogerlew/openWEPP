# WSHEDIMPL42 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Added Contract-Derived Regression Test
- File: `crates/openwepp-runner/src/hillslope/mod.rs`
- Test: `wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten`
- Assertion intent:
  - In breakpoint mode (`ibrkpt=1`) with stale `ninten=2` and active-day
    `nbrkpt=3`, seeding must use 3-point breakpoint cardinality.
  - `ninten` and `nbrkpt` are aligned to `3.0`.
  - `wb12_rainfall_input` preserves full active-day rainfall (`0.003` m).

## Execution Evidence
- Ran:
  - `cargo test -p openwepp-runner wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten`
- Result:
  - pass (`1 passed; 0 failed`).
