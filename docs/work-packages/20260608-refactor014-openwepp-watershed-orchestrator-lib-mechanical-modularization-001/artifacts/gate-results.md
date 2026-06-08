# REFACTOR014 gate results

Status: complete
Evidence mode: Ran

## Required gate command outcomes
- Static: `cargo fmt --check` (pass with exit `0`).
- Static: `cargo clippy --workspace --all-targets -- -D warnings` (pass with exit `0`).
- Ran: `cargo fmt --check` — exit `0`
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` — exit `0`
- Ran: `cargo test -p openwepp-watershed-orchestrator --tests` — exit `0`
  - Test result: `43` passed, `0` failed.
- Ran: `cargo test --workspace` — exit `101`
  - Failing test: `adr0017_is_accepted_and_registered_as_ratified_governance` in `adr0017_comparator_distrust_ratification_contract.rs`.
  - Failure text: `work_packages.contains("20260605-adr0017-comparator-distrust-ratification-001")`
  - Scope assessed as external package-index compliance requirement, pre-existing to this package.
- Ran: `cargo deny check` — exit `0`
  - Warnings only: duplicate `getrandom`, `hashbrown`, `twox-hash`, and unmatched license allowances for `ISC`/`Unicode-DFS-2016` in policy file.
  - No failed advisories/bans/licenses/sources checks.
