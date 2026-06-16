# CQR31 CRAP Before

Ran: `cargo llvm-cov clean --workspace`

Ran: `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/lcov_before.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/crap_before.json`

Target file:
`crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`

Before metrics:

- `build_simulation_owned_wb13_row_for_ofe`
- Line: `883`
- Cyclomatic complexity: `76.0`
- Coverage: `68.78787878787878`
- CRAP: `251.62932776803854`

LCOV summary:

- `FNF: 119`
- `FNH: 76`
- `LF: 1625`
- `LH: 1090`

Line counts before:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  `1875`
- `docs/work-packages/README.md`: `669`
- `docs/work-packages/cqr-burndown-execplan.md`: `754`

Warning: `cargo crap` reported `126` source files with no matching LCOV entry,
the same source-map warning class observed on prior CQR rows.
