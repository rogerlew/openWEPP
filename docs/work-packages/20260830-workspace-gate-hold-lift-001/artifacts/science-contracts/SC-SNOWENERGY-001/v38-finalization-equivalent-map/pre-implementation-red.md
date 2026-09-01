# V38 finalization-equivalent map pre-implementation red

Static authority:

- Retained canonical r95 log: `/tmp/wghl_001d_v37_64m_r95_result_audit.log`.
- SHA-256: `a69e6d16b176cdf29015d55c01834f41a32def624774cf736f8f182884b5571c`.
- Exact support `1800..1860 s`: solver budget 21, scaled merit
  `3.485183697193861e-3`, derived merit `0`, `R_z=[0]`; stabilized replay
  budget 26, scaled merit `3.1996e-3`, derived merit `0`, `R_z=[0]`, one
  independent replay.
- Authentic finalization then remained `LSE=true`, `Stage3=false`,
  `boundary=true` and changed lane-1 thickness bits
  `4569208177783694401 -> 4569208162027237604`.
- This proves the v37 residual/replay map and authentic finalization map use
  different operand tuples. It disproves the omitted-derived-depth hypothesis
  without relaxing any closure rule.

Ran:

- Contract authority test
  `v38_contract_binds_finalization_equivalent_charged_map`: PASS, nextest run
  `9dcfbba0-ae2c-453d-a57d-2dc2be8c5c29`.
- Source/behavior obligation
  `v38_finalization_equivalent_production_seams_are_required`: expected RED,
  nextest run `5213ca85-f4b4-431a-a43f-d0f788818d83`.
- The expected red reports all four production seams and all five behavior
  obligations absent. No production source was changed before this red.

## Multi-layer exact-carry clarification

Static: before the first canonical V38 rerun, inspection of the canonical
fixture established three ordered soil layers per OFE while the initial V38
consumer required exactly one. Sending all layers through the full-coordinate
projection would also reround and zero exact carries for deeper layers that are
not reduced solve coordinates. `INV-SNOWENERGY-062` and
`OBL-SNOWENERGY-C-030` now bind first-node-only projection plus bit-exact
preservation of every deeper layer.

Ran: amended contract authority PASS, nextest run
`c8f2bcd5-daa6-4c59-b39e-9b857e79c402`. The amended source obligation is
expected RED, nextest run `4c93ebfe-81d3-4b25-92b5-86fa16359aa5`, solely
because the charged map does not yet call
`project_soil_thermal_unpublished_top_layer_coordinates_v2`. This red precedes
the partial-projection consumer edit.

Required implementation:

- one finalization-equivalent endpoint map per shared-budget charge;
- proposed terminal snow and exact-carry soil operands reconstructed before
  carrier evaluation;
- non-provisional carrier/LSE/open-covered/precipitation/CN posture identical
  to authentic finalization;
- exactly one Stage 3 physical map per charged evaluation;
- exact v35 receipt stabilization and same-input replay;
- independent authentic-finalization exact image equality before publication;
- typed rollback on provisional-map, extra-map, repair, receipt, replay,
  finalization, budget, or private-publication poison.
