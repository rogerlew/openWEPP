# Erod14 runtime phase integration plan

Status: completed
Evidence mode: mixed

## Static
- Runtime integration point: hillslope `closure_diagnostics` scheduler phase.
- Dispatched phase class remains `hydrology_peak_runoff`.
- Wave-2 activation gate: `erod14_wave2_enabled`.
  - absent or `0`: Wave-2 lane disabled.
  - `1`: Wave-2 executes after EROD13 Wave-1 path in `run_peak_runoff`.
- Production integration location:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - `run_peak_runoff` now calls `run_erod14_wave2`.

## Ran
- Verified by integration tests:
  - `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract`
  - `cargo test --workspace`
