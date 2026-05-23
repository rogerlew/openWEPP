# WB11 Kernel Algorithm Guard Map

Status: `completed`
Evidence mode: `Static`

## Production Kernel
- Type: `Wb11HydrologyKernel`
- File: `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- Boundary: `openwepp_kernel_contract::HillslopeKernel`

## Phase Algorithms
- `hydrology_evapotranspiration`
  - Inputs: `wb11_soil_water`, `wb11_et_demand`
  - Outputs: state `wb11_soil_water`; flux `ET`, `Ws`
  - Deterministic rule: `ET = min(soil_water, et_demand)`; `Ws = 1` when demand near zero else `ET/et_demand`
- `hydrology_percolation_deep_seepage`
  - Inputs: `wb11_soil_water`, `wb11_field_capacity`, `wb11_perc_fraction`
  - Outputs: state `wb11_soil_water`; flux `D`, `Pe`
  - Deterministic rule: `excess = max(soil_water - field_capacity, 0)`; `D = excess * perc_fraction`; `Pe = D`
- `hydrology_lateral_transfer`
  - Inputs: `wb11_drainable_storage`, `wb11_lateral_fraction`, prior `Pe`
  - Outputs: state `wb11_drainable_storage`; flux `q`
  - Deterministic rule: `available = drainable_storage + Pe`; `q = available * lateral_fraction`
- `hydrology_drainage`
  - Inputs: `wb11_drainable_storage`, `wb11_drainage_fraction`, `wb11_drainage_coefficient`, prior `q`
  - Outputs: state `wb11_drainable_storage`; flux `Qdd`, `Qd`
  - Deterministic rule: `Qdd = min(drainable_storage * drainage_fraction, drainage_coefficient)`; `Qd = q + Qdd`

## Typed Guard Classes
- Missing required symbol: `*-E-001` (`BoundaryClass::MissingRequiredInput`)
- Non-finite symbol: `*-E-002` (`BoundaryClass::NonFinite`)
- Domain/range violation: `*-E-003` (`BoundaryClass::DomainViolation`)

## Guard Code Families
- ET: `HKERNEL-WB11-ET-E-001..003`
- Percolation: `HKERNEL-WB11-PERC-E-001..003`
- Lateral: `HKERNEL-WB11-LAT-E-001..003`
- Drainage: `HKERNEL-WB11-DRAIN-E-001..003`
