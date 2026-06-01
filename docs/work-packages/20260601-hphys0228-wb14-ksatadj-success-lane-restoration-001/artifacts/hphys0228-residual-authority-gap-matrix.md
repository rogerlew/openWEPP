# HPHYS0228 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap Matrix

| Gap ID | Description | Status | Evidence |
| --- | --- | --- | --- |
| `HP228-GAP-001` | WB14 `ksatadj` regime tests (`9001/9002/9003`) were downgraded to forced domain-failure signatures and no longer exercised active successful-lane behavior. | closed | Ran: restored successful-lane equivalence assertions in `wb14_infiltration_hyetograph_kernel_contract.rs`, targeted test pass. |
| `HP228-GAP-002` | Active WB14 `ksatadj` vectors were not seeded with WB19-indexed FC/WP-consistent layer values, causing early guard failures unrelated to intended regime-law assertions. | closed | Static + Ran: added ksatadj-only seed normalization helper and kept baseline vectors intact; full WB14 suite pass. |
| `HP228-GAP-003` | Integrated HPHYS residual-family closure remains incomplete (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`). | open | Static: this package was scoped only to WB14 `ksatadj` successful-lane restoration. |
