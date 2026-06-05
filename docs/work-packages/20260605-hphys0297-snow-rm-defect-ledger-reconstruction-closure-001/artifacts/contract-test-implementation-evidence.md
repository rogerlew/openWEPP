# Contract-Test Implementation Evidence

Status: executed
Evidence mode: Static + Ran

Static:
- Added `tests/integration/hphys0297_snow_rm_defect_ledger_contract.rs`.
- Registered the test in `Cargo.toml`.
- The test verifies:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-028`.
  - `SC-RUNOFFPART-001#INV-RUNOFFPART-025`.
  - `SC-WATBAL-001#INV-WATBAL-072`.
  - HPHYS0297 package framing as a defect ledger, not an acceptance bucket.
  - Corrected openWEPP negative-melt helper lineage remains present.

Ran:
- `cargo test --test hphys0297_snow_rm_defect_ledger_contract -- --nocapture`
  - Initial result: failed due package wording/assertion wrapping mismatch.
  - Fix: made the static assertion robust to Markdown line wrapping.
- `cargo test --test hphys0297_snow_rm_defect_ledger_contract -- --nocapture`
  - Result: passed, `3 passed; 0 failed`.
