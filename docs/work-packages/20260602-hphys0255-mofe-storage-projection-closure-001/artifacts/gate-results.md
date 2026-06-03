# Gate Results

Status: complete
Evidence mode: ran

Ran: targeted tests

- `cargo test --test wb11_storage_projection_kernel_contract hphys0255 -- --nocapture`
  - Result: pass.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area -- --nocapture`
  - Result: pass after production patch.

Ran: full Rust gates

- Command:
  `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`
- Result: pass.
- `cargo deny check`: advisories, bans, licenses, and sources ok.
- Warnings retained from existing dependency/license posture:
  duplicate `getrandom`, `hashbrown`, `twox-hash`; unmatched license
  allowances `ISC`, `Unicode-DFS-2016`.
