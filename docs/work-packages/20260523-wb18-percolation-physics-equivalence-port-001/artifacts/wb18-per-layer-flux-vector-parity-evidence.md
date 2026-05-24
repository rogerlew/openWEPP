# WB18 Per Layer Flux Vector Parity Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Vector Basis
Contract-derived nominal vector from
`tests/integration/wb18_percolation_physics_kernel_contract.rs`:
- `nsl = 2`
- `theta = [5.0, 5.0]`
- `fc = [4.0, 4.0]`
- `ul = [8.0, 8.0]`
- `ssc = [2.0e-6, 1.0e-5]`

## Observed WB18 Outputs
From executed WB18 nominal test:
- `wb18_perc_pei_0001 = 0.07184232735651037`
- `wb18_perc_pei_0002 = 0.54`
- `wb18_perc_theta_0001 = 4.92815767264349`
- `wb18_perc_theta_0002 = 4.53184232735651`
- `D = 0.54`
- `Pe = 0.54`
- `wb11_soil_water = 9.46`

## Closure Notes
- Deep percolation loss closes as `D == Pe` in the vector.
- Per-layer state updates remain finite and non-negative.
- Vector demonstrates layerwise flux authority is exercised (non-zero top and
  bottom fluxes) instead of scalar-only WB11 surrogate behavior.
