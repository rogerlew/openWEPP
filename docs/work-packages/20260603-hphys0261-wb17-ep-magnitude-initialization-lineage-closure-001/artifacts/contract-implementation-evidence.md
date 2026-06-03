# Contract Implementation Evidence

Status: completed

Evidence mode: static

## Completed Contract Amendments

Static: Added `SC-EVAP-001#INV-EVAP-019` for WB17 `Ep`
magnitude/initialization trace evidence across `evap` partition state, raw and
effective `pltol`, WB18 `ul(i)`, stress thresholds, layer `UPi`/`Ui`, final
`Ep`, and legacy `evap`/`swu` call-order provenance.

Static: Added the `INV-EVAP-019` guard-map row plus EVAP trace alias and
producer-obligation text for the HPHYS0261 opt-in diagnostics.

Static: Added `SC-WATBAL-001#INV-WATBAL-047` requiring WB13 `Ep`/storage
residual claims to consume `SC-EVAP-001#INV-EVAP-019` before assigning the
stable day-1 `Ep +0.235294 mm` split to physics or publication compensation.

Static: Added the `INV-WATBAL-047` guard-map row plus WATBAL trace alias text
for the HPHYS0261 opt-in diagnostics.

## Legacy Provenance

Static: `/workdir/wepp-forest_260430_baseline/src/evap.for:583-586` seeds
`ep` from current `lai` and `eo`.

Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:557-559`
calls `evap` before daily plant-growth update calls.

Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:943-981`
calls `ptgrp`/`ptgra`, then `swu`.

Static: `/workdir/wepp-forest_260430_baseline/src/swu.for:122-191` applies
effective `pltol`, `ul(i)`, `st(i)`, `UPi`, and `Ui`.
