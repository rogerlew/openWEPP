# Pre-Implementation Contract Gate

Status: complete
Evidence mode: Ran

## Evidence

Ran:
- Command before production-code implementation: `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture`
- Result: failed as expected.
- Failure signal:
  - `hphys0286_contract_post_et_lower_layer_excess_moves_upward`: top-layer storage remained unchanged instead of receiving lower-layer excess.
  - `hphys0286_contract_same_pass_water_uses_frozen_adjusted_upper_cap`: lower layer remained above frozen-adjusted cap.

Static:
- This is the contract-first red gate proving the pre-existing runtime lacked the WB17 post-ET upper-limit redistribution seam.
