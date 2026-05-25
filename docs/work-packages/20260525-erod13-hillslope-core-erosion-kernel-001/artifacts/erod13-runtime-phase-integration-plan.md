# Erod13 runtime phase integration plan

Status: completed
Evidence mode: mixed

## Static
- Runtime integration point: hillslope `closure_diagnostics` scheduler phase.
- Dispatched phase class remains `hydrology_peak_runoff`.
- EROD13 activation gate: `erod13_core_enabled`.
  - absent or `0`: EROD13 path disabled, WB16-only behavior retained.
  - `1`: EROD13 Wave-1 core executes after WB16 peak/duration calculations.
- Production implementation location:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - function `run_peak_runoff` now calls `run_erod13_wave1_core`.

## Ran
- Verified by integration tests that EROD13 executes on enabled path and preserves WB16 behavior when disabled:
  - `cargo test --test erod13_wave1_core_kernel_contract`
  - `cargo test --test wb16_peak_runoff_kernel_contract`
