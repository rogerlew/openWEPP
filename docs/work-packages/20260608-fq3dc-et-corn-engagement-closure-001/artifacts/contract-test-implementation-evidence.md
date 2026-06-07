# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Static + Ran.

## Added Tests

- `crates/openwepp-runner/src/hillslope/mod.rs`
  - `fq3dc_annual_preplant_skip_preserves_pl_sentinel_for_later_activation`
  - `fq3dc_scheduler_calendar_day_symbol_uses_julian_day_for_pl_activation`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap`

## Updated Contract-Version Tests

- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
  now expects `SC-WATBAL-001` `contract_version: 147`.
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
  now expects `SC-WATBAL-001` `contract_version: 147`.

## Ran

- `cargo test -p openwepp-hillslope-orchestrator fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap -- --nocapture`
- `cargo test -p openwepp-runner fq3dc_ -- --nocapture`
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract`
- Final full gate includes `cargo test --workspace`.

All listed tests passed.
