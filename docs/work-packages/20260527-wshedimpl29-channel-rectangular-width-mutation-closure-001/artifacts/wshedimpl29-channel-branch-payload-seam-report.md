# WSHEDIMPL29 Channel Branch Payload Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Seam target:
  - Baseline `chnrt.for` detachment lane mutates rectangular bottom width:
    `if flagc.eq.2.and.werb(ichan,i-1).gt.wfu) widb(ichan,i-1)=werb(ichan,i-1)`.
- WS29 seam closure:
  - runtime `dcap` now exposes eroded-width geometry (`werod`) to WS20 lane
    logic,
  - WS20 applies the rectangular mutation gate (`werod > wfu`) to
    `widb(i-1)`,
  - mutated `widb` points are written back to runtime state symbols
    (`ws10_channel_{id}_widb_{point:04}`).

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl29_contract_ws20_rectangular_widb_mutation_projects_to_state -- --exact` -> pass
