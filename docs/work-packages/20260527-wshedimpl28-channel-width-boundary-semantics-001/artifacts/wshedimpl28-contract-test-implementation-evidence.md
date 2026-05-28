# WSHEDIMPL28 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Contract-derived WS11 vector added in:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths`
    - enables WS20+WS21 routing lanes with required `crfrac` projection,
    - forces rectangular hydraulic lane (`ws10_channel_1_ishape = 2`) so
      boundary-width symbols are behaviorally active,
    - perturbs lower-boundary `wida` segment widths (`0002`, `0003`),
    - asserts channel routing outputs (`qsed` and/or `tc`) respond.
- Pre-runtime probe note:
  - initial probe execution (before the rectangular-lane forcing refinement)
    produced no output shift and failed the assertion; that probe result was
    used to tighten the vector so it targets the authoritative seam directly.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths -- --exact`
  - Result: fail in initial probe run before vector forcing refinement.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths -- --exact`
  - Result: pass after runtime migration + vector forcing refinement.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract`
  - Result: pass (28/28), includes WS28 vector.
