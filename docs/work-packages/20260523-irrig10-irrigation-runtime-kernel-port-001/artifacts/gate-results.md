# Gate Results

Status: `completed`
Evidence mode: `Ran`

## Required Gate Commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

## Results

- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`)

## Notes

`cargo deny check` reported only `license-not-encountered` warnings for
unmatched allow-list entries in `deny.toml`; these did not fail the gate.
