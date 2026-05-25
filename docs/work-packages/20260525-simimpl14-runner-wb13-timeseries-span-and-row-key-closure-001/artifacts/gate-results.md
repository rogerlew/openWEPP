# gate-results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required gate set executed for SIMIMPL14 disposition.

## Ran
- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> fail.
- Failure surface: `crates/openwepp-watershed-output/src/writers.rs` (`clippy::uninlined_format_args`, `clippy::too_many_lines`) outside SIMIMPL14 scoped write set.
- User-directed acceptance: lint closure ownership explicitly transferred to the active writer work-package; SIMIMPL14 closure is approved without reopening SIMIMPL14 scope.
- `cargo test --workspace` -> pass.
- `cargo deny check` -> pass (warnings only).
