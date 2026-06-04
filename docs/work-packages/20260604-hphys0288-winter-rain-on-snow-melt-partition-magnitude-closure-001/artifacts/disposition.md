# Disposition

Status: executed-hold
Evidence mode: Static + Ran

## Summary

HPHYS0288 is executed and held for continuation. The package ported the baseline-authoritative residual rain-on-snow release seam from `snowd.for -> winter.for -> wmelt` into openWEPP's snow/runoff coupling, expanded contract tests, fixed review-identified duplication and evidence gaps, and passed final Rust/governance gates.

## Accepted Implementation

Static:
- Residual rain-on-snow release is tracked as `snow.hourly.rain_released_m_####`.
- Released rain is added to routed hourly melt after daily signed-melt redistribution, preserving baseline order.
- Released rain is excluded from physical snowpack storage loss and subtracted from direct-rain input to avoid double counting.
- WB12/WB14 partition terms are centralized in `resolve_snow_partition_terms`.
- HPHYS0245 trace schema v13 emits `snow_hourly_rain_released_sum_m` and `wb12_infiltration_m` for target trace diagnosis.

## Validation

Ran:
- Focused HPHYS0288 contract tests: pass, 3 tests.
- Focused runner trace tests: pass.
- Authority anti-evasion and AUTH11 guards: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with existing duplicate/unmatched-license warnings.
- Full H1..H39 semantic suite: runtime 39/39, semantic 39/39, semantic pass `0/39`.

## Metrics

Ran:
- Final full-suite root: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z`.
- Selected metric deltas versus HPHYS0287:
  - `Ep`: mean abs diff improved by `-0.015602`; fail count `-974`.
  - `Total-Soil` / `SoilWaterTotal`: mean abs diff improved by `-4.046006`; fail count `-2363`.
  - `latqcc`: mean abs diff improved by `-0.037288`; fail count `-573`.
  - `Dp`: mean abs diff worsened by `+0.000717`; fail count `+116`.
  - `Q`: effectively unchanged.
  - `RM`: unchanged.
  - `Snow-Water`: unchanged.

## Hold Rationale

Static:
- Semantic parity remains `0/39`; this package does not close HPHYS water-balance parity.
- The correction moved storage/ET/lateral metrics but did not move `Q`, `RM`, or `Snow-Water`.
- The next package should target WB13/RM publication and winter runoff/snowpack forcing lineage on the material 2014/2016 residual days, not compensate with WB17 ET tuning.
