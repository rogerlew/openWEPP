# Gate Results

Status: passed
Evidence mode: Ran

Ran:
- `cargo fmt --check`
  - Result: passed.
- `cargo test --test hphys0295_cumulative_storage_budget_contract -- --nocapture`
  - Result: passed, `3 passed; 0 failed`.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing warnings for duplicate crates and unmatched
    license allowances; advisories, bans, licenses, and sources reported `ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - Result: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - Result: passed, `2 passed; 0 failed`.
- `wctl doc-lint --path docs/work-packages/README.md`
  - Result: passed, `1 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001`
  - Result: passed, `0 files validated, 0 errors, 0 warnings`.
