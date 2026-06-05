# Gate Results

Status: passed
Evidence mode: Static + Ran

Ran:
- Final validation:
  - `cargo fmt --check`
    - Initial result: failed due formatting in the new HPHYS0297 test.
  - `cargo fmt`
    - Result: applied formatting.
  - `cargo fmt --check`
    - Result: passed.
  - `cargo test --test hphys0297_snow_rm_defect_ledger_contract -- --nocapture`
    - Result: passed, `3 passed; 0 failed`.
  - `cargo clippy --workspace --all-targets -- -D warnings`
    - Result: passed.
  - `cargo test --workspace`
    - Result: passed.
  - `cargo deny check`
    - Result: passed with existing duplicate-crate and unmatched-license
      warnings; advisories, bans, licenses, and sources reported `ok`.
  - `bash tools/release/check_authority_suite_antievasion.sh`
    - Result: passed.
  - `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
    - Result: passed, `2 passed; 0 failed`.
  - `wctl doc-lint --path docs/work-packages/README.md`
    - Result: passed, `1 files validated, 0 errors, 0 warnings`.
  - `wctl doc-lint --path docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001`
    - Result: passed, `0 files validated, 0 errors, 0 warnings`.
- `cargo test --test hphys0297_snow_rm_defect_ledger_contract -- --nocapture`
  - Initial result: failed due package wording/assertion wrapping mismatch.
- `cargo test --test hphys0297_snow_rm_defect_ledger_contract -- --nocapture`
  - Result: passed, `3 passed; 0 failed`.
- `.venv/bin/python docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/hphys0297_defect_ledger.py --run-root /tmp/hphys0297_full_20260605T000000Z --trace-max-days 1800`
  - Result: passed.
