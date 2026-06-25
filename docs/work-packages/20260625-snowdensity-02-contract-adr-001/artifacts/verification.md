# Verification

Ran:

- `cargo fmt --check`
- `cargo test --test snowdensity02_contract_adr_guard`
- `git diff --check`

Result:

- PASS.

Iteration notes:

- First focused test attempt did not find the test target because `Cargo.toml`
  requires explicit `[[test]]` registration.
- After registration, the guard failed once on the package handoff wording for
  the no-site-constants rule.
- Both issues were corrected; the final focused run passed `3` tests.

Not run:

- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`

Attempted but not counted:

- `wctl doc-lint --path docs/work-packages/20260625-snowdensity-02-contract-adr-001`
- `wctl doc-lint --path docs/decisions/0027-opt-in-physics-bulk-snow-model.md`
- `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

These returned `0 files validated`; they are not treated as validation evidence.

Rationale:

- This package changed contract/ADR/planning documentation and added one
  focused integration guard. It did not change production Rust code or
  mechanical-refactor scope. The package acceptance criteria require focused
  guard validation, formatting, and whitespace checks.
