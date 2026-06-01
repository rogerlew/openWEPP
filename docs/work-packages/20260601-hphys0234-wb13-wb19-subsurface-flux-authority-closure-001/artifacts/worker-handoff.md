# HPHYS0234 Worker Handoff

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Immediate Next Actions

1. Open follow-on package to reconcile remaining coupled residual families:
   - `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`.
2. Prioritize WB18/WB19 coupled physics lanes over WB13 publication lanes:
   HPHYS0234 proved publication anti-shadow fixes are not the remaining
   dominant residual source.
3. Preserve WB13 flux-preferred subsurface publication lineage as locked:
   - `D`, `q`, `Qdd`, `Qd` resolve via `*_prefer_flux(...)` under conflicts.
4. Rerun `H1..H39` in follow-on and publish monitored-column deltas relative to
   HPHYS0234 summary.
5. Use this run root as evidence anchor:
   - `/tmp/hphys0234_20260601T215019Z/parity/`.
