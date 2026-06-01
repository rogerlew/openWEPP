# HPHYS0233 Worker Handoff

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Immediate Next Actions

1. Open follow-on package to reconcile remaining coupled residual families:
   - `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`.
2. Prioritize WB18/WB19 coupling lane where `latqcc` regressed in HPHYS0233:
   - audit lateral withdrawal and soil-water continuity interaction after
     restrictive conductivity changes,
   - isolate whether daily `D` reduction is now over-coupling into lateral
     availability and downstream publication.
3. Carry forward strict guard posture added in HPHYS0233:
   - no silent defaults for `slflag` domain violations,
   - no non-positive `kslast` acceptance when `slflag=1`,
   - preserve WB13 flux-preferred `Dp` publication lineage.
4. Rerun `H1..H39` and publish same monitored-column summary table with
   explicit deltas vs HPHYS0233 baseline.
5. Use this run root as evidence anchor:
   - `/tmp/hphys0233_20260601T211306Z/parity/`.
