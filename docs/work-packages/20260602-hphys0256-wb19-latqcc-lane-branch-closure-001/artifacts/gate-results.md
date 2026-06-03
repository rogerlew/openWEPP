# Gate Results

Status: completed

Evidence mode: ran

- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Ran: `cargo test --workspace` passed.
- Ran: `bash tools/release/check_authority_suite_antievasion.sh` passed.
- Ran: `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` passed.
- Ran: `cargo test --test auth06_fixture_provenance_hash_enforcement_contract -- --nocapture` passed as part of workspace and earlier focused authority checks.
- Ran: `cargo deny check` passed with existing duplicate crate warnings
  (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowance
  warnings (`ISC`, `Unicode-DFS-2016`).
