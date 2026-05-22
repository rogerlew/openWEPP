# CLIM12 Gate Results

Status: `complete`
Evidence mode: `Ran`

Ran:
1. `cargo fmt --check`
- exit: `0`
- note: initially failed on formatting, then `cargo fmt` applied, and re-run passed.

2. `cargo clippy --workspace --all-targets -- -D warnings`
- exit: `0`
- note: initially reported unused imports and `needless_pass_by_value`; patched and re-ran successfully.

3. `cargo test --workspace`
- exit: `0`
- note: full workspace suite passed, including new integration parity test.

4. `cargo deny check`
- exit: `0`
- note: completed with `license-not-encountered` warnings in `deny.toml` allowlist; advisories/bans/licenses/sources all reported `ok`.
