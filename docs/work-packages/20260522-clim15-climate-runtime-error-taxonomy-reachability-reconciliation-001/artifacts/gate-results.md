# CLIM15 Gate Results

Evidence mode: `Ran`
Status: `pass`

Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass

Notes:
- `cargo deny check` emitted `license-not-encountered` allowlist warnings; final advisory/bans/licenses/sources checks were `ok`.
