# HPHYS0203 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP203-GAP-001` | Canonical contracts lacked explicit physics-robustness vectors for WB13 publication families spanning profile, soil-water aggregate, and subsurface-loss surfaces. | Static: HPHYS0203 addenda in `SC-WATBAL-001`, `SC-SOIL-001`, `SC-SUBHYD-001`, `SC-SYSTEM-001`, plus registry updates in `science-contracts/index.md`. | closed |
| `HP203-GAP-002` | Robustness coverage for domain guards/closure/order invariants was not codified as dedicated contract-derived tests for HPHYS follow-on lanes. | Static: new integration suite `tests/integration/hphys0203_physics_robustness_contract.rs`; new WB13 direct guard probes in `openwepp-runner` tests; `Cargo.toml` test registration. | closed |
| `HP203-GAP-003` | Deterministic perturbation stability and regression-fixture closure checks were incomplete for FC/WP profile-storage lineage. | Static + Ran: perturbation vectors landed in HPHYS0203 integration/runner tests and passed in `cargo test --workspace`. | closed |

## Residual risk after closure
- Ran: diagnostic comparator residual remains non-zero for several targeted
  columns (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`,
  `ProfileWPStore`) in the 39-hillslope lane.
- Static: HPHYS0203 defines robustness closure and diagnostics, not comparator
  zero-delta closure; downstream integrated disposition remains queued in
  `hphys0204`.
