# Operand Lineage

Status: pre-implementation.

Static: All R4N operands are direct-runtime shadow-only and use meters as the
water-depth unit unless noted.

| Operand | Unit | Direct Source | Authority | Downstream Role |
|---|---:|---|---|---|
| `Es` | m | R4N surface ET soil extraction from upper 0.10 m | `SC-EVAP-001` | component diagnostic and aggregate ET seed |
| `Er` | m | R4N residue interception evaporation | `SC-EVAP-001` | component diagnostic and aggregate ET seed |
| `Etp` | m | R4N LAI-scaled transpiration demand | `SC-EVAP-001` | root uptake demand |
| layer storage after ET | m | R4N surface ET mutation | `SC-EVAP-001`, `SC-WATBAL-001` | R4O layer input |
| `UPi` / `Ui` | m | R4N SWU potential/actual uptake | `SC-EVAP-001` | root uptake diagnostics |
| `Ep` | m | sum of R4N post-WB19 actual uptake | `SC-EVAP-001` | component ET diagnostic |
| `Ws` | ratio | `Ep / Etp`, bounded by canonical zero-demand rules | `SC-EVAP-001` | stress diagnostic |
| aggregate `ET` | m | `Es + Er + Ep` after root uptake | `SC-EVAP-001`, `SC-WATBAL-001` | R4B `evapotranspiration_m` |

Static: Public WB13/WAT/PASS ET fields are out of scope. R4N may compare to
publication-side values in tests, but it must not make them authoritative.
