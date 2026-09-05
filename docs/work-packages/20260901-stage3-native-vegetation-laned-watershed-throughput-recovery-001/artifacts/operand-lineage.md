# Conservation operand lineage

Status: `TERMINAL AUTHORITY RETAINED — PACKAGE QUALIFICATION FAILED`

Evidence mode: `Static`

| Operand | Units/basis | Authoritative producer | Consumer / reconstruction | Rejected alias |
| --- | --- | --- | --- | --- |
| solid/liquid precipitation parcels | `kg m^-2 tile-ground`, destination-weighted to OFE ground | sealed precipitation parcel set | Stage 3 water/advected heat ledgers | raw daily precipitation or CoE partition |
| beginning/ending ice and retained liquid | `kg m^-2 OFE-ground` | accepted Stage 3 owner | independent support/day mass delta | candidate/private solver state |
| melt, refreeze, deposition, sublimation, routed liquid | `kg m^-2 OFE-ground` | accepted Stage 3 physical result | solid and liquid identities | adjacent diagnostic opportunity columns |
| ordered surface/CN/soil heat | `J m^-2 OFE-ground` | accepted LSE/Stage 3/soil receipts | equal/opposite snow-soil and complete energy closure | receipt digest or provisional Q coordinate |
| cold-content and terminal energy delta | `J m^-2 OFE-ground` | accepted beginning/ending Stage 3 owners | independent energy-state difference | private root residual |
| native transpiration/evaporation | `kg m^-2 tile-ground s^-1` and integrated mass | accepted vegetation occupancy solver | destination-weighted ET ledger | PMET-only/daily legacy ET |
| Lane D soil release | `q_runoff_m`, metres over `DirectDayFrame.area_m2`; volume is exactly `q_runoff_m * area_m2` | accepted `DirectRunoffShadowProjection.q_runoff_m` in the committed day frame | `laned_active_lane_source`, then 24 hourly `LanedActiveLaneSource.depths_m` whose sum reconstructs `q_runoff_m` | hillslope daily aggregate, `liquid_input_m`, or surface-liquid ingress trace |
| Lane D mesh source | hourly metres over lane area, converted to mesh depth by `area_m2 / (slplen_m * width_m)` and to `m s^-1` by the recorded `/3600` helper | `LanedActiveLaneSource.depths_m` plus geometry | `route_single_ofe_with_step_trace`; injected volume is solver `rainfall_excess_m2 * width_m` | raw rainfall intensity or unscaled lane-area depth |
| Lane D upstream handoff | solver `UpstreamHandoff` discharge/volume state in the kinematic-wave module's declared SI basis | immediately preceding OFE router result in lane-index order | next OFE call to `route_single_ofe_with_step_trace` | cloned local source, daily aggregate, or reordered OFE |
| Lane D sent/received and outlet books | `m3` per OFE/day; `source_m3=sum(depths_m)*area_m2`, `soil_release_m3=q_runoff_m*area_m2`, `outlet_m3=outflow_m2*width_m`, `mesh_storage_m3=storage_change_m2*width_m` | active Lane D result/books in `direct_runtime/laned_active.rs` | adjacent handoff cancellation, router mass identity, then hillslope closure before row publication | depth without area, width/area cross-pair, cloned source, or legacy interchange alias |
| WAT/PASS/HBP publication values | each field's schema-declared depth or volume basis, paired with the exact accepted row `area_m2` and OFE identity | accepted `DirectPublicationDayRow` after Lane D closure and Stage 3 day commit | field-specific reconstruction from accepted row plus per-OFE Lane D books; no grouped unit assumption | internal-state area, another row's area, legacy interchange column, or producer residual |

Acceptance fixtures must make every rejected alias numerically different from
the authoritative operand. In particular, the 10+-OFE fixture uses unequal
OFE areas, unequal precipitation/ET/runoff, nonzero upstream transfers, snow
and snow-free lanes, and nonzero CN heat so that an aggregate, adjacent column,
old daily builder, or same-formula self-check cannot pass accidentally.

Independent reconstruction reads produced accepted outputs/owner snapshots,
not the producer's residual vector. It reconstructs support and daily snow
mass/energy, exact snow-soil cancellation, ET water loss, each adjacent Lane D
handoff, lane-area-to-mesh conversion, router source/storage/outlet identity,
hillslope internal-transfer cancellation, and outlet volume/magnitude.
Metadata units and area basis must match this table. A factor-`1000` depth
poison, a lane-area/mesh-area swap, and a previous/next-OFE area swap are
mandatory separating vectors.
