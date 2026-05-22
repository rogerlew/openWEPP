# Gate Results

Status: `complete`
Evidence mode: `Ran`

Required gate commands:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

## Results

Execution context: `/home/workdir/openWEPP`
Log directory: `docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/gate-logs/`

1. `cargo fmt --check`
- result: `fail`
- log: `gate-logs/01-cargo-fmt-check.log`
- failing surface: `tests/integration/infile_hbp_parser_contract.rs` (format drift)

2. `cargo clippy --workspace --all-targets -- -D warnings`
- result: `pass`
- log: `gate-logs/02-cargo-clippy-workspace.log`

3. `cargo test --workspace`
- result: `pass`
- log: `gate-logs/03-cargo-test-workspace.log`

4. `cargo deny check`
- result: `pass-with-warnings`
- log: `gate-logs/04-cargo-deny-check.log`
- warnings: `license-not-encountered` unmatched allow-list entries; terminal summary reports `advisories ok, bans ok, licenses ok, sources ok`.

## Ratification Outcome

Full required gate set is **not all-pass** due to `cargo fmt --check` failure.
