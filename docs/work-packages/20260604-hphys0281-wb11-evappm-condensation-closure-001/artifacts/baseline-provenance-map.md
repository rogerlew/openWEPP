# Baseline Provenance Map

Status: completed
Evidence mode: static

| openWEPP surface | Baseline authority | Mapping |
|---|---|---|
| `pmet.es_m` | `/workdir/wepp-forest_260430_baseline/src/evappm.for:435-456` | Migrated EVAPPM raw soil/residue evaporation result; production publishes non-negative value for WB17 seam. |
| `pmet.es_storage_return_m` | `/workdir/wepp-forest_260430_baseline/src/evappm.for:461-472` | Positive storage-return magnitude when baseline `xx = es - resint` is negative. |
| `wb18_perc_theta_0001` | `/workdir/wepp-forest_260430_baseline/src/evappm.for:469-472` | openWEPP top-layer alias for baseline `st(1,iplane)` receiving `-xx`. |
| WB17 PMET guard | `SC-EVAP-001#INV-EVAP-022` | Material-negative `pmet.es_m` remains invalid; producer must not publish it. |
