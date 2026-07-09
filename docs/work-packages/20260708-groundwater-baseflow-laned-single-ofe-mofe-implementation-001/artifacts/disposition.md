# Disposition

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

| Item | Disposition | Evidence |
|---|---|---|
| Use `GwcoeffFile` parser state, not ad hoc parsing | accepted | Runner sidecar resolution now carries `HillslopeSidecarResolution.gwcoeff`; direct authority conversion consumes parsed fields. |
| Missing `gwcoeff.txt` disables process | accepted | `lr_bf=0` maps to `DirectGroundwaterAuthority::Disabled`; no coefficient defaults were added. |
| Implement Srivastava linear recurrence | accepted | `DirectGroundwaterRunState::run_day`; tested by `gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports`. |
| MOFE recharge aggregation | accepted | `run_groundwater_day_from_lane_frames` sums `deep_percolation_m * area_m2`; tested by `gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation`. |
| Active router source exclusion | accepted | `laned_active_lane_source` unchanged; no groundwater fields are read by the surface source builder. |
| Generated baseflow consumer proof | accepted | WAT `Base` producer added; watershed WAT reader already consumes `Base` into `baseflow_mm`/`channel_baseflow_m3`; runner projection test added. |
| Generated deep seepage consumer proof | deferred | `gwdsv` computed/recorded but no real downstream consumer implemented. |
| `bftharea` threshold branch | deferred | coefficient carried, but watershed/channel threshold behavior is outside this hillslope package. |
