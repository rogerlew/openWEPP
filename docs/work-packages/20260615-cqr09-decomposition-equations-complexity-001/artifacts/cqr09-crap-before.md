# CQR09 CRAP Before

Ran: `cargo crap --workspace --lcov
docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_before.info
--min 0 --format json --output
docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/crap_before.json`
exited `0`.

Static: live target identity from `crap_before.json`:

- file:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
- function: `build_annual_decomposition_control`
- line: `712`
- cyclomatic complexity: `79.0`
- coverage: `38.97849462365591`
- CRAP: `1497.0871919084125`

Static: other pre-existing target-file CRAP rows above `30` before this
package:

- `build_perennial_decomposition_control`: line `1167`, CC `47.0`,
  coverage `61.35593220338983`, CRAP `174.48063613125004`.
- `compute_equation_decomposition_seed_surface`: line `12`, CC `27.0`,
  coverage `63.58381502890174`, CRAP `62.20552842111687`.

Static: `cargo-crap` emitted duplicate rows for the same target-file functions
because the workspace report includes multiple crate path entries. Closure uses
unique `(file, function, line)` rows.
