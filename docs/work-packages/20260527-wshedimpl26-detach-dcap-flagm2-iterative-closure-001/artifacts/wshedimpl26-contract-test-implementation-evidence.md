# WSHEDIMPL26 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Contract-derived WS11 vector added in:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `wshedimpl26_contract_ws21_case4_iterative_closure_stress_vector_remains_resolved`
    - enables WS20+WS21 opt-in with `crfrac`,
    - applies elevated channel erodibility (`chnk`) forcing,
    - asserts successful routing and zero unresolved-detachment diagnostics.
- Kernel unit vector added in:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs` test module
  - `wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe`
    - verifies `flagm=2` behavior caps detachment rate at `maxe`,
      while `flagm=1` remains uncapped.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract`
  - Result: pass.
- `cargo test -p openwepp-watershed-orchestrator wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe`
  - Result: pass.
