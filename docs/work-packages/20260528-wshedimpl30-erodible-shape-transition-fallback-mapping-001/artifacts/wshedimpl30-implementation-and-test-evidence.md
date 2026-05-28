# WSHEDIMPL30 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime updates in `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - Added `WS30_ERODIBLE_RECTANGULAR_DEPTH_THRESHOLD_FT = 1.0e-4`.
  - Expanded WS15 control guard to accept `ishape` domain `1..=3`.
  - Added `ws30_shape_flag_from_ishape` and
    `ws30_apply_erodible_rectangular_fallback`.
  - WS20 routing now:
    - carries both `depa` and `depb` point vectors,
    - resolves `flagct` from `ishape`,
    - applies `depb(i-1)` fallback to rectangular (`flagc=2`) at upper
      boundary calls (`hydchn`, `dcap`),
    - applies `depa(i)` fallback to rectangular (`flagc=2`) at lower boundary
      calls (`hydchn`, WS23/WS24 detach closures).
  - WS10 terminal `tc` hydraulics now resolve `ishape` through the same WS30
    mapping and use terminal `depb` fallback when required.
- Added two WS30 vectors to WS11 integration contract suite.

## Ran
- Pre-implementation gate failures recorded in
  `wshedimpl30-preimplementation-contract-gate.md`.
- Post-implementation exact vectors:
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl30_contract_ws20_ishape3_erodible_lane_vector_executes -- --exact` -> pass
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl30_contract_ws20_ishape3_depa_depb_fallback_mapping_affects_outputs -- --exact` -> pass
