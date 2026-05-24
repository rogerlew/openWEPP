# WB19 Lateral Drainage Hydraulic Vector Parity Evidence

Status: `completed`
Evidence mode: `Ran`

## Command Evidence
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract`
- Result: `4 passed; 0 failed`

## Nominal Hydraulic Vector
From `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
(`TOL = 1.0e-9`):
- Lateral phase (`HKERNEL-WB11-LAT-OK-001`)
  - `q = 0.1221880517890351`
  - `wb18_perc_theta_0001 = 4.377811948210965`
  - `wb18_perc_theta_0002 = 6.0`
  - `wb11_drainable_storage = 2.377811948210965`
- Drainage phase (`HKERNEL-WB11-DRAIN-OK-001`)
  - `Qdd = 0.1`
  - `Qd = 0.2221880517890351`
  - `wb18_perc_theta_0002 = 5.9`
  - `wb11_drainable_storage = 2.277811948210965`

## Interpretation
The WB19 nominal vector confirms deterministic, layer-aware lateral and
drainage writeback behavior with explicit cap handling and aggregate
`Qd = q + Qdd` coupling.
