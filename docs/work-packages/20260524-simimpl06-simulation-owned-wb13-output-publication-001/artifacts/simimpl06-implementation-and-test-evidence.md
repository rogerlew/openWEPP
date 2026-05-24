# simimpl06 implementation and test evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- `execute_daily_scheduler_kernel_lifecycle(...)` now returns a composed
  execution result that includes:
  - SIMPIPE execution provenance,
  - SIMOUT WB13 publication provenance,
  - a simulation-owned WB13 row assembled from executed writeback surfaces.
- WB13/H.wat production publication path now consumes simulation-owned row data:
  - `build_h5_wat_output(&SimulationOwnedWb13Row)`
  - `build_hillslope_wat_rows(&SimulationOwnedWb13Row)`
- Projection-first helper path was removed from WB13 publication flow.
- Typed SIMOUT failure routing added for missing/non-finite/domain-invalid
  runtime symbol requirements at WB13 publication assembly boundary:
  - surface: `wb13_publication`
  - guard id: `HS-SIMOUT-E-001`
- Parquet schema field was aligned with canonical WB13 naming:
  - `Total-Soil Water` -> `Total-Soil`

## Ran
- `cargo fmt --all`
- `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wb13_publication_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract -- --ignored`

## Outcomes
- Initial clippy pass surfaced style-level issues (`too_many_lines`,
  `uninlined_format_args`, conversion lint); addressed in-code and re-ran
  clippy to pass.
- SIMOUT WB13 contract test is now active and passing.
- Workspace gates are passing; `cargo deny check` completed with non-blocking
  duplicate/unmatched-license warnings only.
- Deferred SIMMODE vector remains expected-fail when forced with `--ignored`.
