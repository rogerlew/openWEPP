# Gate Results

Status: passed
Evidence mode: Ran

Ran:
- `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`
  - Result: passed, `3 passed; 0 failed`.
- `.venv/bin/python docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/hphys0296_diagnostics.py --run-root /tmp/hphys0296_full_20260605T070000Z --trace-max-days 1800`
  - Result: passed.
- `cargo fmt --check`
  - Result: initially failed on formatting; `cargo fmt` applied formatting.
- `cargo fmt --check`
  - Result: passed after formatting.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing duplicate-crate and unmatched-license-allowance
    warnings; advisories, bans, licenses, and sources reported `ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - Result: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - Result: passed, `2 passed; 0 failed`.
- `wctl doc-lint --path docs/work-packages/README.md`
  - Result: passed, `1 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001`
  - Result: passed, `0 files validated, 0 errors, 0 warnings`.
