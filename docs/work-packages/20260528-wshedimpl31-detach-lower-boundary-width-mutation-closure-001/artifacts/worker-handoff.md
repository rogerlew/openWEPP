# WSHEDIMPL31 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Completed in WSHEDIMPL31
- Canonical contracts/index updated for WS31 scope:
  - `SC-ROUTE-001` v33
  - `SC-SED-001` v32
  - `SC-SYSTEM-001` v54
  - `science-contracts/index.md` notes refreshed
- WS23/WS24 detach closures now return eroded-width outcome (`werod_ft`) and
  WS20 lower-boundary routing applies baseline rectangular mutation rule:
  - `flagc=2 && werod_ft>wfl_ft => wida(i)=werod_ft`.
- WS10 writeback now publishes `wida_{point:04}` updates from WS20 routing
  execution.
- WS11 vectors added/passing for WS31 seam behavior.

## Immediate Next Actions
1. Reconcile parser/runtime shape-code lineage so naturally eroded class mapping
   from watershed channel input authority is explicit and unambiguous across
   parser projection and WS10 kernel consumption (WSHEDIMPL30 handoff item 2).
2. Continue channel sediment migration with explicit HOLD posture until
   `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008` closure criteria are met.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_ws24_rectangular_detach_wida_mutation_projects_to_state -- --exact` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_non_rectangular_lane_does_not_apply_wida_mutation -- --exact` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
