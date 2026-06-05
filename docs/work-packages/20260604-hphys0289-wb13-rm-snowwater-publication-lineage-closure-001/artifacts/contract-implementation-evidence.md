# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Contract Amendments

- `SC-WATBAL-001` updated to `contract_version: 108` with `REF-WATBAL-LEGACY-WB13-RM-SNOW`, `RM` variable coverage, `INV-WATBAL-064`, guard-map coverage, and HPARITY01 `RM` lineage register correction.
- `SC-RUNOFFPART-001` updated to `contract_version: 32` with `REF-RUNOFFPART-LEGACY-WB13-RM`, `INV-RUNOFFPART-019`, and guard-map coverage for routed daily `wmelt` publication.
- `SC-SNOWFREEZE-001` updated to `contract_version: 24` with `REF-SNOWFREEZE-LEGACY-WB13-RM-SNOW`, `wmelt` variable coverage, `INV-SNOWFREEZE-022`, and guard-map coverage.

## Baseline Authority

Static:

- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106` computes `rm=(rain(iplane)+wmelt(iplane)+irdept(iplane)+iraplo(iplane))*1000.` and `Snow-Water=snodpy(iplane)*densg(iplane)`.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142` mirrors the daily WB13 RM/Snow-Water publication formula.
- `/workdir/wepp-forest_260430_baseline/src/outfil.for:621-630` labels WB13 `RM=rainfall+irrigation+snowmelt` and `Snow Water=Water in surface snow`.
- `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880` clears `rain(iplane)` after winter processing except the warm-rain/no-snow restoration branch.
- `/workdir/wepp-forest_260430_baseline/src/winter.for:456-464` adds residual rain-on-snow release into `hrmlt`/`wmelt`.
