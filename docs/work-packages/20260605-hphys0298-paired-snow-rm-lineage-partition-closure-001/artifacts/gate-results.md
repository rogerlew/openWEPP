# Gate Results

Status: queued

Evidence mode: not-run

Ran: pending.

Required gates:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
- `wctl doc-lint --path docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001`
