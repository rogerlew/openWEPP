# Contract-Test Implementation Evidence

Status: completed

Evidence mode: Static + Ran

Static:
- Added contract-derived tests in
  `tests/integration/wb18_percolation_physics_kernel_contract.rs`:
  - `wb18_contract_conformance_hourly_restrictive_bottom_uses_bedrock_thickness_weighting`
  - `wb18_contract_conformance_hourly_restrictive_bottom_requires_ui_bdrkth`
  - `wb18_contract_conformance_hourly_restrictive_bottom_rejects_non_finite_ui_bdrkth`
  - `wb18_contract_conformance_hourly_restrictive_bottom_rejects_non_positive_ui_bdrkth`
- Review fixes removed a duplicated test oracle loop and asserted final
  `wb18_perc_theta_0001` plus `wb11_soil_water` state mutation.

Ran:
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  before production edits: expected fail.
- Failing vectors:
  - `uses_bedrock_thickness_weighting`: expected `D=0.00026396333842521876`,
    observed `D=0.17280000000000006`.
  - `requires_ui_bdrkth`: expected `HKERNEL-WB11-PERC-E-001`, observed
    `HKERNEL-WB11-PERC-OK-001`.
- After the first production patch and H39 run, updated the first vector to
  cover the additional baseline `meblfc` bottom-layer `fx=1` branch.
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  then failed as expected before the second production patch:
  - expected `D=0.00026396333842521876`,
    observed `D=0.00000418081710894704`.
- Final targeted run passed: `15/15`.
