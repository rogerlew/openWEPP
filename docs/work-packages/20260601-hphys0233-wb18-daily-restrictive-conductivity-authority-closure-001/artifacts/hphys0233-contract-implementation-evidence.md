# HPHYS0233 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Static

Updated canonical contract authority in:
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`

Amendments applied:
1. `contract_version` advanced `17 -> 18`.
2. Added WB18 daily restrictive-layer conductivity authority:
   - when `slflag=1`, bottom-layer seepage conductivity uses harmonic effective
     `K` from `Ksi` and `kslast`,
   - branch scoped to daily lane (`wb18_perc_lane_substeps == 1`) and bottom
     layer only.
3. Added strict domain posture:
   - `slflag` must be finite binary domain (`0/1`),
   - `kslast` must be finite and strictly positive when `slflag=1`.
4. Added WB13 publication lineage anti-shadow requirement:
   - `Dp` publication must consume flux-authoritative `D` and not a stale state
     surface when both exist.
5. Updated alias mapping, required inputs, test-vector obligations, and
   revision history for HPHYS0233.

Authority anchors cited in-contract:
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
