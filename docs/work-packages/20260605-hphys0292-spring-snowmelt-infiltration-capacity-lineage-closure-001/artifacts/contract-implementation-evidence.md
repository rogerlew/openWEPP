# Contract Implementation Evidence

Status: executed
Evidence mode: Static

Static:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-025` for H1/H7/H39 spring producer-partition localization across snow state, retained/released rain, raw/routed melt, WB12 infiltration, `Q`, WB13 `RM`, and storage.
- Added `SC-RUNOFFPART-001#INV-RUNOFFPART-022` for WB12/WB14 routed-melt capacity classification; amended during execution to require producer hourly melt timing while conserving the daily routed-melt scalar.
- Added `SC-WATBAL-001#INV-WATBAL-067` for spring snowmelt/infiltration capacity lineage before downstream storage/ET ownership is assigned.
- Added guard-map/proof-obligation rows `OBL-SNOWFREEZE-P-013`, `OBL-RUNOFFPART-P-008`, and `INV-WATBAL-067` references in canonical `SC-*` files.

Baseline provenance:

- `/workdir/wepp-forest_260430_baseline/src/disag.for` lines 141-174: snowmelt branch computes positive melt duration and melt forcing.
- `/workdir/wepp-forest_260430_baseline/src/grna.for` lines 267-269: `smrate = wmelt(iplane) / dur`.
