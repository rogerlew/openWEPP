# HPHYS0225 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap Matrix

| Gap ID | Description | Status | Evidence |
| --- | --- | --- | --- |
| `HP225-GAP-001` | WB19 lateral/drainage runtime source used legacy max-reconciliation available-pool expressions (`max(layer_pool, legacy_term)`). | closed | Static: parent source capture showed both expressions at lines `1087` and `1262`; production source now uses `let available_pool = layer_pool;` in both phases. |
| `HP225-GAP-002` | No dedicated required Level-4 suite guarded WB19 layer-pool available-cap authority. | closed | Static: added suite `cas_l4_subhyd_layer_pool_withdrawal_cap_001` with fixture lock/provenance + registry linkage. Ran: `cargo test --test hphys0225_wb19_layer_pool_withdrawal_cap_contract` passed. |
| `HP225-GAP-003` | Canonical contract language did not explicitly prohibit legacy max-reconciliation expansion. | closed | Static: `SC-SUBHYD-001` adds `INV-SUBHYD-017` + HPHYS0225 addendum; `SC-WATBAL-001` adds HPHYS0225 addendum linked to new suite. |
| `HP225-GAP-004` | Integrated residual families (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`) remain unresolved outside this scoped closure. | open | Static + Ran: package scope intentionally excludes full cohort rerun; integrated HPHYS stream remains `HOLD`. |
