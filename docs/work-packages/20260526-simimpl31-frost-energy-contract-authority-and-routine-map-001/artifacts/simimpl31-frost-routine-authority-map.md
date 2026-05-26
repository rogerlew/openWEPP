# SIMIMPL31 Frost Routine Authority Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL31 ratifies canonical frost routine-chain authority from
  `/workdir/wepp-forest_260430_baseline` into `SC-SNOWFREEZE-001`.
- Authority map:

| Baseline routine | Baseline call-map responsibility | Contract alias/invariant ownership |
|---|---|---|
| `winter` (`winter.for`) | Dispatches hourly frost processing under active winter/frost trigger conditions. | Dispatch closure for `INV-SNOWFREEZE-012`; boundary families `winter.hourly.*`, `snow.hourly.*`, downstream `frost.runtime_*`. |
| `frostN` (`frostn.for`) | Main frost driver; performs `frwatc(1)` ingress, freeze/thaw branch dispatch, and `frwatc(0)` egress at hour-24 or thaw-complete. | Branch/handoff closure for `INV-SNOWFREEZE-012`; required payload family `frost.runtime_*` and reserved `frost.hourly.*`. |
| `frzng` (`frzng.for`) | Freeze-active depth extension and latent-heat branch execution; calls `frznw` as needed. | Freeze lineage closure for `INV-SNOWFREEZE-012`; depth/water bookkeeping linkage to `frost.runtime_dfrost` and `frost.runtime_ws_frz`. |
| `frznw` (`frznw.for`) | Fine-layer freeze helper used by `frzng`. | Energy/time bounded freeze update semantics under `INV-SNOWFREEZE-012`. |
| `frwatc` (`frwatc.for`) | Bidirectional water exchange bridge (`wbtofs=1` water-balance -> frost; `wbtofs=0` frost -> water-balance). | Handoff seam closure under `INV-SNOWFREEZE-012`; cross-contract soil/watbal coupling continuity. |
| `frsoil` (`frsoil.for`) | Frost-active saturated conductivity adjustment across fine layers. | Conductivity seam closure under `INV-SNOWFREEZE-013`; runtime coupling to `frost.runtime_infcap_frz`. |
| `getFreezeCond` (`getfreezecond.for`) | Land-use/plant-based frozen-soil coefficient selector consumed by `frsoil`. | Coefficient provenance in `INV-SNOWFREEZE-013` and CLIM06 `kfactor*` coupling posture. |
| `winthd` (`winthd.for`) | Winter diagnostics output/reporting path. | Governance-only reporting authority; not a runtime-seam replacement. |

## Ran
- `rg -n "call frostN\(hour\)|frdp\(iplane\)\.ge\.0\.001|hrtemp\.lt\.0\.0|slsic\(1,1,iplane\)\.ge\.0\.00001" /workdir/wepp-forest_260430_baseline/src/winter.for`
- `rg -n "call frwatc\(1\)|call frzng\(hour\)|call frwatc\(0\)|frzflg" /workdir/wepp-forest_260430_baseline/src/frostn.for`
- `rg -n "subroutine frsoil|getFreezeCond\(|subroutine frwatc|wbtofs" /workdir/wepp-forest_260430_baseline/src/frsoil.for /workdir/wepp-forest_260430_baseline/src/getfreezecond.for /workdir/wepp-forest_260430_baseline/src/frwatc.for`
