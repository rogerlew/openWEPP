# WSHEDIMPL37 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime migration updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - added explicit WS11 runon partition lineage publication
    (`rvolat`, `rvotop`, `rvolon`) and duration-max lineage publication
    (`durlat`, `durtop`, `durrunon`, `durchan`, `durirrig`, `watdur`),
  - added explicit runoff-case lineage publication
    (`ws11_runoff_case`, `ws11_qci`, `ws11_qcf`, `ws11_runvol`, `tl`, `rofc`),
  - preserved explicit `ipeak` branch continuity for
    Rational/CREAMS/KinematicWave/MuskingumCunge lanes with threshold handling
    and routed incoming-hydrograph behavior when local runoff is absent.
- WS11 conformance vectors now assert route-chain closure surfaces and
  branch-specific behaviors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`.
- Canonical contracts/index updated for WSHEDIMPL37 traceability and
  `GAP-ROUTE-008` closure.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl37_` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
