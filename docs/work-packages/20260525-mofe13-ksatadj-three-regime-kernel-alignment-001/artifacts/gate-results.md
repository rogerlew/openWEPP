# Gate Results

Status: complete
Evidence mode: Ran

## Required Gates
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only)

Additional parity gates:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill ... output_mofe13 ... --policy compat` -> pass
- semantic comparator canonical baseline run -> fail (`no baseline rows parsed`)
- semantic comparator normalized 25-column baseline run -> pass (execution),
  report indicates row-key mismatch with `semantic_pass=false`.
