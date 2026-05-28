# WSHEDIMPL29 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Runtime implementation updates:
  - File: `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - `ws26_dcap` now returns projected geometry outcomes (`df`, `depmid`,
    `werod`) instead of only detachment flux.
  - `ws20_route_case12_segment_family` now:
    - validates and consumes segment `depb` inputs for `dcap`,
    - applies baseline rectangular mutation gate:
      `if flagc == 2 && werod > wfu { widb(i-1) = werod }`,
    - carries updated `widb` point values in routing result payload.
  - channel node writeback now projects `widb` point symbols:
    - `ws10_channel_{id}_widb_{point:04}`.
- Contract-derived test updates:
  - File: `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - Added
    `wshedimpl29_contract_ws20_rectangular_widb_mutation_projects_to_state`.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl29_contract_ws20_rectangular_widb_mutation_projects_to_state -- --exact` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass (29/29)
