# WB17 Contract Test Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Contract-Derived Test Surfaces Added
- Added new integration suite:
  - `tests/integration/wb17_et_physics_kernel_contract.rs`
- Registered suite in workspace test targets:
  - `Cargo.toml` (`[[test]] name = "wb17_et_physics_kernel_contract"`)

## Implemented WB17 Contract Vectors
1. WB17 ET partition vector validates component outputs (`Ep`, `Es`, `Er`) and
   aggregate outputs (`ET`, `Ws`) under soil-water-limited conditions.
2. Missing WB17 required ET symbol vector validates typed missing-input failure
   posture for `wb17_residue_interception`.
3. Non-finite WB17 ET input vector validates typed non-finite failure posture
   for `lai`.
4. Domain-invalid WB17 ET input vector validates typed domain failure posture
   for negative `wb17_residue_interception`.

## Pre-Implementation Expectation
These vectors were authored from updated canonical SC authority and expected to
fail against the existing WB11 surrogate ET runtime before production WB17 code
implementation.
