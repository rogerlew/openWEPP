# HPHYS0204 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Integrated residual-family gap register
| Gap ID | Residual family | Evidence | Confidence tier | Status |
| --- | --- | --- | --- | --- |
| `HP204-GAP-001` | `ProfileFCStore` residual closure incomplete (`27/39`) after depth-authority stabilization. | Ran: `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`; Static: HPHYS0207 disposition evidence and HPHYS0202-0207 lineage notes. | single-hillslope daily lane (higher-confidence comparator lane; residual remains investigation signal) | open |
| `HP204-GAP-002` | Subsurface/soil-water aggregate residual families remain saturated (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`: `39/39` fail hillslopes), with shared kernel dependence on FC/WP threshold lineage (`thetfc_####`/`thetdr_####`). | Ran: recomputed from HPHYS0207 semantic reports and summarized in HPHYS0204 artifacts; Static: HPHYS0207 reviewer lineage coupling evidence. | single-hillslope daily lane (higher-confidence comparator lane; residual remains investigation signal) | open |
| `HP204-GAP-004` | `ProfileWPStore` is near-closed (`1/39`) and should be adjudicated separately from FC residual posture. | Ran: recomputed from HPHYS0207 semantic reports (`ProfileWPStore` fail count + mean-abs). | single-hillslope daily lane (higher-confidence comparator lane; residual remains investigation signal) | open (near-closed) |
| `HP204-GAP-003` | Profile geometry/capacity surfaces are effectively closed (`ProfileDepth 0/39`, `ProfilePorosityCap 0/39`). | Ran: recomputed targeted fail counts and mean-abs summaries from HPHYS0207 semantic reports. | single-hillslope daily lane (higher-confidence acceptance signal) | closed |

## Root-cause family classification
1. `HP204-GAP-001` is a FC storage residual lane that remains open after
   normalized-profile depth-authority stabilization.
2. `HP204-GAP-002` remains in percolation/lateral/soil-water process-lineage
   families but shares FC/WP threshold input lineage on the kernel side; this
   family is not independent of FC/WP threshold-state migration closure.
3. `HP204-GAP-004` is a near-closed WP storage lane and should not be grouped
   at the same severity as FC residuals.
4. `HP204-GAP-003` indicates profile depth/capacity publication authority is
   stable under current contract and implementation posture.
