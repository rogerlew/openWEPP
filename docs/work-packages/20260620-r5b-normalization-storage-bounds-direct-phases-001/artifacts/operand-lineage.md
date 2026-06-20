# Operand Lineage

Static:

| Operand | R5B source | Downstream use |
|---|---|---|
| precipitation | `DirectDayForcing.precipitation_m` | input accounting, storage input |
| effective temperature | `DirectDayForcing.effective_temperature_c` | normalization context only |
| surface carry | sum of `DirectTransferBuffers.surface_carry_m` | transfer input accounting |
| lateral carry | sum of `DirectTransferBuffers.lateral_carry_m` | transfer input accounting |
| upstream flow | `DirectTransferBuffers.upstream_flow_m` | transfer input accounting |
| subsurface input | `DirectTransferBuffers.subsurface_input_m` | transfer input accounting |
| initial storage | `DirectWaterState.soil_water_m` | storage input and storage bounds |
| closure tolerance | `DirectStorageReconciliationInputs.closure_tolerance_m` | storage bounds |

R5B does not publish any public WB13/WAT/PASS/loss operand.
