# Consumer Path Proof

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

Producer-only evidence cannot close export/publication. This package closes the
generated baseflow (`gwbfv`) consumer leg and records the remaining generated
deep-seepage/channel threshold hold.

## `gwbfv` Closed Path

1. Parser:
   `openwepp_input_contract::parsers::gwcoeff::parse_gwcoeff_from_path` returns
   `GwcoeffFile`.
2. Runner intake:
   `HillslopeSidecarResolution.gwcoeff` carries the parser result in legacy
   discovery and runfile modes.
3. Runtime authority:
   `direct_groundwater_authority_from_gwcoeff` maps `lr_bf=0` to
   `DirectGroundwaterAuthority::Disabled` and `lr_bf=1` to
   `DirectGroundwaterAuthority::LinearReservoir`.
4. Runtime state:
   `DirectRunFrame.configure_groundwater` initializes `DirectGroundwaterRunState`
   from total Lane D area and `igwstrd`.
5. Recharge input:
   `DirectRunFrame.run_groundwater_day_from_lane_frames` sums
   `hydrology_projection.deep_percolation_m * lane.area_m2` over all lanes after
   per-lane hydrology.
6. Recurrence:
   `DirectGroundwaterRunState::run_day` computes `S_i`, `gwbfv`, and `gwdsv`
   and fails closed on non-finite, negative, or outflow-over-storage states.
7. Active-router negative proof:
   `laned_active_lane_source` remains unchanged and reads only surface runoff
   source terms (`wb14_hourly_excess`, `ui_SCrunf`-lineage carry, routed melt);
   no groundwater output fields are consumed by the surface router.
8. Publication:
   terminal direct publication rows carry `groundwater_baseflow_mm` and
   `groundwater_baseflow_m3`; non-terminal rows remain zero so watershed WAT
   aggregation reconstructs the generated m3 exactly from the terminal row.
9. Real downstream consumer:
   direct WAT now emits nullable `Base`; `crates/openwepp-runner/src/watershed_wat.rs`
   already reads optional `Base` into `baseflow_mm` and
   `channel_baseflow_m3`.
10. Ran:
    `cargo test -p openwepp-runner r6a_direct_projection_consumers_read_publication_frame_operands -- --nocapture`
    proves direct publication `groundwater_baseflow_mm` maps to WAT `Base`.

## Held Path

`gwdsv` is computed and recorded in direct runtime state and active summary, but
no real HBP/pass/watershed consumer was implemented in this package. `bftharea`
is parsed and carried as authority, but the watershed/channel threshold branch
is also not consumed by the current hillslope Lane D implementation.

Required follow-on:

- add a real generated groundwater deep-seepage consumer or explicitly
  authorized publication surface; and
- implement or adjudicate the watershed/channel `bftharea` threshold branch.
