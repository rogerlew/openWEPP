# WSHEDIMPL18 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented/updated contract-derived vectors:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`:
    - Updated `wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family`
      to assert:
      - `qsed` mass conservation remains authoritative,
      - `tc` is finite/non-negative and not collapsed to surrogate identity.
    - Added
      `wshedimpl18_contract_channel_transport_capacity_responds_to_particle_diameter`
      to enforce `tc` sensitivity to class-diameter perturbation while `qsed`
      remains unchanged.

## Ran
- `cargo test --workspace` passed, including WS11 vectors for
  transport-capacity behavior.
