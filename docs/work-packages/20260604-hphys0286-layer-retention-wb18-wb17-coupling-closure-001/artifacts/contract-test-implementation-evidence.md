# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

## Evidence

Static:
- Added `tests/integration/hphys0286_layer_retention_wb18_wb17_contract.rs`.
- Registered the test target in `Cargo.toml`.
- Test `hphys0286_contract_post_et_lower_layer_excess_moves_upward` proves no-outside-water lower-layer excess is capped at `wb18_perc_ul` and moved upward without aggregate loss.
- Test `hphys0286_contract_same_pass_water_uses_frozen_adjusted_upper_cap` proves same-pass outside water activates the frozen-adjusted cap `ul - frzw`.

Ran:
- `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture`
  - Result after implementation: passed, `2 passed`.
