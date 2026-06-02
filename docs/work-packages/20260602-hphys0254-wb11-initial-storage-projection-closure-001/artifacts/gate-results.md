# Gate Results

Status: complete

Evidence mode: ran

Ran:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Result:

- All required gates passed.
- `cargo deny check` reported existing warnings for duplicate crate versions (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances (`ISC`, `Unicode-DFS-2016`), then completed with `advisories ok, bans ok, licenses ok, sources ok`.

Additional focused gates:

- `cargo test -p openwepp-hillslope-orchestrator --lib -- --nocapture`: passed after alias-specific unit-test updates.
- `cargo test -p openwepp-runner --lib -- --nocapture`: passed after runner test fixture alias updates.
