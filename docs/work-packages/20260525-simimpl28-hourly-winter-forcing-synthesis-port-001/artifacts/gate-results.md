# SIMIMPL28 Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required package gates executed for SIMIMPL28 scope.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crate versions and
  unmatched allowed licenses in `deny.toml`; no failing advisories/bans/licenses/sources)
