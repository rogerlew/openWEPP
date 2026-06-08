# REFACTOR007 gate results

Status: complete  
Evidence mode: ran  
Date: 2026-06-08

## Command log bundle
- `artifacts/gates-20260608T014949Z`

## Scope
Validation gates executed:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p openwepp-runner --tests`
4. `cargo test --workspace`
5. `cargo deny check`

## Results
- `cargo fmt --check`: pass (`exit_code=0`)
- `cargo clippy --workspace --all-targets -- -D warnings`: pass (`exit_code=0`)
- `cargo test -p openwepp-runner --tests`: pass (`exit_code=0`)
- `cargo test --workspace`: pass (`exit_code=0`)
- `cargo deny check`: pass (`exit_code=0`)

## Warning capture
- `cargo deny check` emitted non-fatal warnings:
  - `warning[duplicate]` for `getrandom`, `hashbrown`, `twox-hash`
  - `warning[license-not-encountered]` for `ISC`, `Unicode-DFS-2016`
