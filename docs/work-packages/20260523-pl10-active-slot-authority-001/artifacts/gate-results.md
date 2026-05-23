# PL10 Gate Results

Status: `complete`
Evidence mode: `Ran`

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Results

| gate | command | result | notes |
|---|---|---|---|
| format | `cargo fmt --check` | `pass` | formatted source checked clean |
| lint | `cargo clippy --workspace --all-targets -- -D warnings` | `pass` | no warnings remaining |
| tests | `cargo test --workspace` | `pass` | workspace unit/integration/doc tests passed |
| dependency-policy | `cargo deny check` | `pass` | advisories/bans/licenses/sources all `ok`; unmatched allowlist warnings present in `deny.toml` |
