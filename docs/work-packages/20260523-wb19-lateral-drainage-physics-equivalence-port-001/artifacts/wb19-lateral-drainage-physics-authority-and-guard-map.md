# WB19 Lateral Drainage Physics Authority And Guard Map

Status: `completed`
Evidence mode: `Static`

## Scope
Record canonical WB19 lateral/drainage physics authority and the corresponding
runtime guard/failure map used by production kernels.

## Canonical Authority
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (v7)
  - WB19 algorithm state surfaces and execution spec sections define
    layer-aware lateral and drainage equations, withdrawal order, and `Qd = q + Qdd`.
  - WB19 branch/guard table defines typed failure IDs
    `HKERNEL-WB11-LAT-E-001..003` and `HKERNEL-WB11-DRAIN-E-001..003`.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (v23)
  - Hydrology lane authority updated to WB17 ET + WB18 percolation + WB19
    lateral/drainage execution and typed guard continuity.
- `docs/specifications/science-contracts/index.md`
  - Registry notes updated to advertise WB19 authority as active posture.

## Runtime Guard Map
| Kernel phase class | Required WB19 family | Guard behavior | Typed failures |
| --- | --- | --- | --- |
| `hydrology_lateral_transfer` | `nsl`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ssc_####`, `dg_####`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`, `Pe` | Missing symbol hard-fail, non-finite hard-fail, out-of-range/domain hard-fail | `HKERNEL-WB11-LAT-E-001..003` |
| `hydrology_drainage` | Lateral family + `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient`, `q` | Missing symbol hard-fail, non-finite hard-fail, out-of-range/domain hard-fail | `HKERNEL-WB11-DRAIN-E-001..003` |

## Production Implementation Anchor
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - WB19 symbol constants: lines 140-146.
  - Layer load + finite/domain guards: lines 2091-2171.
  - Top-down lateral withdrawal: lines 2181-2196.
  - Tile-layer-to-surface drainage withdrawal: lines 2198-2221.
  - WB19 lateral execution path: lines 4363-4526.
  - WB19 drainage execution path: lines 4529-4848.

## No-Silent-Default Posture
WB19 production paths reject missing/non-finite/out-of-range domains with typed
status IDs; no silent clamping/defaulting paths were introduced for WB19
lateral/drainage required symbols.
