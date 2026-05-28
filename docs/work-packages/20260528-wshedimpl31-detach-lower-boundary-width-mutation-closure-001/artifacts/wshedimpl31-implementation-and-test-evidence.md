# WSHEDIMPL31 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime updates in `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - Added `Ws23DetachClosureOutcome` to carry both routed sediment flux and
    lower-boundary eroded-width outcome (`werod_ft`) from WS23/WS24 closure.
  - Updated WS23 and WS24 closure functions to return the detach outcome
    structure.
  - Added baseline-authoritative lower-boundary width mutation projection in
    WS20 routing:
    - if `lower_flagc == 2` and `werod_ft > wfl_ft`, update
      `wida(i)` (`width_a_points_ft[segment_index]`) with `werod_ft`.
  - Extended WS20 routing result and WS19 publication payload with
    `wida_points_ft` / `ws31_wida_points_ft`.
  - Added WS10 state writeback publication for `wida_{point:04}`.
- Added two WS31 vectors to WS11 integration contract suite:
  - rectangular-lane mutation path,
  - non-rectangular control path.

## Ran
- Exact WS31 vectors:
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_ws24_rectangular_detach_wida_mutation_projects_to_state -- --exact` -> pass
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_non_rectangular_lane_does_not_apply_wida_mutation -- --exact` -> pass
- WS30+WS31 focused regression subset:
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl3` -> pass
