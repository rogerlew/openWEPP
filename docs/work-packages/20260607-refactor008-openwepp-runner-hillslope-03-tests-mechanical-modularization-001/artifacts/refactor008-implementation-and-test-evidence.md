# REFACTOR008 refactor008 implementation and test evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
Execution evidence for the mechanical extraction of `03_tests.rs`.

## Static
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - test functions removed: `0`
  - line count reduced: `3942 -> 518`
  - module declarations added: `simimpl`, `publication`, `trace`
- `crates/openwepp-runner/src/hillslope/tests03/simimpl.rs`
  - tests moved: `5`
  - line count: `130`
- `crates/openwepp-runner/src/hillslope/tests03/publication.rs`
  - tests moved: `49`
  - line count: `2079`
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`
  - tests moved: `14`
  - line count: `1228`

## Ran
- Required package gates were executed and passed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test -p openwepp-runner --tests`
  - `cargo test --workspace`
  - `cargo deny check`
- Post-edit `#[test]` inventory was counted via static scan and remains preserved:
  - pre: `68`
  - post: `68`
