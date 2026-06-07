# Implementation and Test Evidence

Status: complete

Evidence mode: Static + Ran.

## Implementation Summary

- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Seeds scheduler-facing `year` and Julian `day` symbols before scheduler
    execution.
  - Captures the PL activation sentinel before annual pre-plant filtering.
  - Restores the sentinel after scheduler writeback so later days can activate
    annual slots.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - Removed the invalid `WB15_VDMT_MAX` plant-state cap.
  - Added `WB15_INTERCEPT_BIOMASS_MAX_KG_HA = 8000.0`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Allows finite non-negative `vdmt`.
  - Applies the pinned-baseline `8000 kg ha^-1` cap only to the interception
    equation input.

## Focused Runs

Ran:

- `cargo test -p openwepp-runner fq3dc_ -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap -- --nocapture`: passed.
- `cargo test -p openwepp-runner hphys0250_pl_activation_keeps_zero_date_perennial_slots_active -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator annual_growth_phase_emits_typed_growth_context -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator pl16_annual_growth_accepts_zero_gddmax_sentinel_for_summer_branch -- --nocapture`: passed.

## Notes

Temporary diagnostic prints used during localization were removed before final
validation. Static search found no `OPENWEPP_DIAG` remnants in the touched code.
