# Gate Results

Status: complete
Evidence mode: Ran

## Rust Gates

Ran:
- `cargo fmt --check`
  - Result: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing dependency duplicate/license-not-encountered warnings; final status: `advisories ok, bans ok, licenses ok, sources ok`.

## Contract And Runtime Gates

Ran:
- Pre-implementation contract gate: failed as expected before production edits.
- Focused HPHYS0285 contract tests: passed after production edits.
- Adjacent HPHYS0283/HPHYS0284 snowmelt tests: passed.
- Claude review remediation gate: `cargo test --test hphys0284_negative_melt_snowpack_state_contract --test hphys0285_spring_soil_storage_retention_contract -- --nocapture` passed, `6 passed`.
- Claude review adjacent snow gate: `cargo test --test clim05_snow_runtime_kernel_contract --test hphys0283_snowmelt_infiltration_partition_contract -- --nocapture` passed, `10 passed`.
- Post-review H1..H39 release runtime rerun after `CLAUDE-0285-001` remediation: passed, `39/39`; root `/tmp/hphys0285_review_remediation_20260604T203602Z`.
- Release H1 smoke: passed.
- Full H1..H39 release runtime: passed, `39/39`.
- Full H1..H39 semantic suite: completed, `0/39` semantic pass.
