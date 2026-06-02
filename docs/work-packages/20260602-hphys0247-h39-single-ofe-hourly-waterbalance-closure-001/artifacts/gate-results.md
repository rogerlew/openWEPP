# Gate Results

Status: queued

Evidence mode: not-run

Static:
- Queued artifact for final gate commands and results.

Ran:
- Not run.

Required final gates:
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo deny check`
- [ ] `bash tools/release/check_authority_suite_antievasion.sh` if required
- [ ] `cargo test --test auth11_required_suite_obligation_guards_contract` if
  required
