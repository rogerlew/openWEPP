# HPHYS0208 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP208-GAP-001` | Runtime-input projection omitted coupled WB11 threshold lineage surfaces required by downstream WB11/WB18/WB13 publication consumers (`sat`, `por_####`, `cpm_####`). | Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` now projects `sat`, `por_####`, `cpm_####` (plus primary aliases) from corrected-layer lineage. | closed |
| `HP208-GAP-002` | WB11 seed initialization consumed incomplete threshold lineage and allowed surrogate initialization paths that violated HPHYS0208 coupling intent. | Static: `crates/openwepp-runner/src/hillslope/mod.rs` now requires coupled `sat`/`por_####`/`cpm_####`/`thetfc_####`/`thetdr_####`/`dg_####` inputs with typed fail-closed guards and lane-aware saturation policy. | closed |
| `HP208-GAP-003` | WB14 `ksatadj` top-two-layer metric derivation assumed legacy FC/WP layout and failed against HPHYS0208 seed semantics. | Static: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` now supports HPHYS0208 `thetdr_####` lineage with legacy fallback for compatibility lanes. | closed |
| `HP208-GAP-004` | Coupled residual families remain open after implementation (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`). | Ran: `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json` reports fail-hillslope counts `27`, `39`, `39`, `39`, `39` respectively. | open |

## Residual risk after execution
- Ran: fail-hillslope counts did not improve versus HPHYS0207 for all monitored
  columns.
- Ran: residual magnitude regressed on two coupled columns:
  - `Dp` mean abs diff avg: `0.1870 -> 40.1559`
  - `latqcc` mean abs diff avg: `83.5557 -> 173.2285`
- Ran: `Total-Soil` and `SoilWaterTotal` improved modestly in mean abs diff,
  but fail-hillslope counts remained saturated at `39/39`.
