# WSHEDIMPL31 Detach Lower-Boundary Width Mutation Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Baseline authority (`detach.for`):
  - `flagc = flagct`
  - `if (flagc.eq.3.and.depa(ichan,i).eq.0.0) flagc = 2`
  - `if (flagc.eq.2.and.wera(ichan,i).gt.wfl) wida(ichan,i)=wera(ichan,i)`
- WSHEDIMPL31 migration mapping:
  - WS23/WS24 closure now returns `werod_ft` (`wera`-equivalent detach width).
  - WS20 segment loop applies baseline lower-boundary mutation rule under
    rectangular-lane semantics (`lower_flagc == 2`).
  - Updated `wida` point family is carried into WS10 state writeback via
    `wida_{point:04}` publication.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_ws24_rectangular_detach_wida_mutation_projects_to_state -- --exact` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl31_contract_non_rectangular_lane_does_not_apply_wida_mutation -- --exact` -> pass
