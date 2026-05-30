# HPHYS0204 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Integrated residual-family gap register
| Gap ID | Residual family | Evidence | Confidence tier | Status |
| --- | --- | --- | --- | --- |
| `HP204-GAP-001` | `ProfileFCStore` / `ProfileWPStore` residual closure incomplete (`27/39`, `1/39`) after depth-authority stabilization. | Ran: `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`; Static: HPHYS0207 disposition evidence. | single-hillslope daily lane (higher-confidence comparator lane; residual remains investigation signal) | open |
| `HP204-GAP-002` | Subsurface/soil-water aggregate residual families remain saturated (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`: `39/39` fail hillslopes). | Ran: recomputed from HPHYS0207 semantic reports and summarized in HPHYS0204 artifacts. | single-hillslope daily lane (higher-confidence comparator lane; residual remains investigation signal) | open |
| `HP204-GAP-003` | Profile geometry/capacity surfaces are effectively closed (`ProfileDepth 0/39`, `ProfilePorosityCap 0/39`). | Ran: recomputed targeted fail counts and mean-abs summaries from HPHYS0207 semantic reports. | single-hillslope daily lane (higher-confidence acceptance signal) | closed |

## Root-cause family classification
1. `HP204-GAP-001` likely in FC/WP storage-lineage and threshold-policy lanes.
2. `HP204-GAP-002` likely in percolation/lateral/soil-water process-lineage
   migrations (not resolved by FC/WP depth-authority package).
3. `HP204-GAP-003` indicates profile depth/capacity publication authority is
   stable under current contract and implementation posture.
