# Batch 4 Coupling-Surface Adjudication

Evidence mode: Static
Status: partial map + narrower HOLD; no relocation

## Outcome

Batch 4 did not relocate narrative. CLIM05's signed snow-coupled WB12 storage closure rows map precisely to `INV-WATBAL-013`, but they were retained in core with active test-vector context. CLIM06, WB14, WB15, and IRRIG10 carry live coupling formulas, required surfaces, guard codes, and/or contract-test obligations that are not fully exposed by current WATBAL invariant rows.

## Row dispositions

| Entry group | Outcome | Binding IDs | Next gate |
|---|---|---|---|
| CLIM05 snow-coupled reconciliation rows | map-to-existing-INV, retained in core | `INV-WATBAL-013` | none for BEI; later structural cleanup may move only after adjacent test-vector context is explicitly indexed. |
| CLIM06 frozen-soil infiltration rows | narrower science-HOLD | none | `SCSTRUCT03-CLIM06-BEI-PROMOTION` |
| WB14 infiltration/hyetograph rows | narrower science-HOLD | none | `SCSTRUCT03-WB14-BEI-PROMOTION` |
| WB15 canopy interception rows | narrower science-HOLD | none | `SCSTRUCT03-WB15-BEI-PROMOTION` |
| IRRIG10 irrigation storage-coupling rows | narrower science-HOLD | none | `SCSTRUCT03-IRRIG10-BEI-PROMOTION` |

## Conservation

No `INV-*` or `OBL-*` rows were added, removed, or weakened. No narrative moved to the sidecar.
