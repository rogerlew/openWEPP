# CQR30 CRAP Before

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/lcov_before.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/crap_before.json`

Target file:
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`

Before metrics:

- `Wb11HydrologyKernel::run_erod13_wave1_core`
- Line: `6`
- Cyclomatic complexity: `81.0`
- Coverage: `69.60352422907489`
- CRAP: `265.2636791582994`

LCOV summary:

- `FNF: 1`
- `FNH: 1`
- `LF: 227`
- `LH: 158`

Warning: `cargo crap` reported `126` source files with no matching LCOV entry,
the same source-map warning class observed on prior CQR rows.
