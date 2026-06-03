# Gate Results

Status: completed

Evidence mode: ran

## Gates

- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Ran: `cargo test --workspace` passed.
- Ran: `cargo deny check` passed with existing duplicate/unmatched-license
  warnings; advisories, bans, licenses, and sources were ok.
- Ran: `bash tools/release/check_authority_suite_antievasion.sh` passed.
- Ran: `cargo test --test auth11_required_suite_obligation_guards_contract`
  passed.
