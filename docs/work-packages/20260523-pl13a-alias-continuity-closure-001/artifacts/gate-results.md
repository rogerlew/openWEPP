# PL13A Gate Results

Status: `complete`
Evidence mode: `Ran`

## Alias Continuity Verification Gate

```bash
cargo test --test sim_contract_symbol_alias_registry -- --nocapture
```

Result:
- `ok` (`13 passed`, `0 failed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: `license-not-encountered` warnings were reported for unmatched
  allowlist entries in `deny.toml`; command exit status remained success.
