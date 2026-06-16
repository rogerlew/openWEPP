# CQR28 CRAP Before

Ran: before metrics were captured with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/lcov_before.info`
- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/crap_before.json`

Target file:
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`.

Before LCOV for target file:

- Lines: `790/1149`, `68.76%`
- Functions: `10/14`, `71.43%`

Live before rows above CRAP `30` in the target file:

| Function | Line | CC | Coverage | CRAP | Scope |
|---|---:|---:|---:|---:|---|
| `Wb11HydrologyKernel::run_percolation` | 734 | 91.0 | 71.5430861723447 | 281.82979375564685 | CQR28 target |
| `Wb11HydrologyKernel::resolve_effective_wb18_frozen_depth` | 390 | 16.0 | 10.638297872340425 | 198.68137117979634 | Out of scope |
| `Wb11HydrologyKernel::run_plant_root_uptake` | 14 | 53.0 | 88.38951310861424 | 57.39645909305284 | Out of scope |

Ran: `cargo crap` emitted 126 LCOV source-map warnings. The target file was
present in LCOV and in `crap_before.json`; the warning is recorded as a tool
configuration warning, not a target-metric blocker.
