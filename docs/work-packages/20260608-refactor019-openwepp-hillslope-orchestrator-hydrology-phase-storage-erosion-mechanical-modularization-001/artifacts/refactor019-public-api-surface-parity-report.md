# REFACTOR019 Public API Surface Parity Report

Status: complete
Evidence mode: Static/Ran

Static:
- Baseline target file initially exported these `pub(crate)` kernel methods:
  - `run_storage_reconciliation`
  - `run_erod13_wave1_core`
  - `run_erod14_wave2`
  - `run_erod19_route_segment_migration`
  - helper helpers `erod19_shear`, `erod19_root`, `erod19_xcrit_classification`, `erod19_depc`, `erod19_depend`
  - `run_peak_runoff`
- Public surface expected to remain unchanged for this package because behavior is non-functional in intent.

Ran:
- 2026-06-08T22:50:27Z: post-refactor symbol scan confirms the same `pub(crate)` method set exists under module split (same function names retained).
- 2026-06-08T22:50:27Z: `cargo test --workspace` and `cargo test -p openwepp-hillslope-orchestrator --tests` passed, indicating import and call-site compatibility.
