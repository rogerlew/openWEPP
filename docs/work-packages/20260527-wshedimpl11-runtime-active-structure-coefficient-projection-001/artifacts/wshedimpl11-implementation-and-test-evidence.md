# WSHEDIMPL11 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented runtime active-structure projection in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - removed active projection-gap fail-closed branch,
  - added deterministic active payload projection helpers for drop, culvert,
    rockfill, emergency, filter-barrier, and perforated-riser families,
  - preserved typed domain/non-finite guards for projected coefficients and
    intermediate derivation surfaces.
- Promoted runtime unit vector to require active coefficient projection success
  in `runtime_inputs` tests.
- Added active WS12 integration vector in
  `tests/integration/ws12_impoundment_physics_equivalence_contract.rs` and
  refactored fixture seeding helper to support active/inactive impoundment
  fixtures.

## Ran
- `cargo fmt`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_projects_active_structure_coefficients`
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract wshed11_contract_ws12_vector_projects_active_structure_payloads`
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract`
- `cargo test -p openwepp-watershed-orchestrator`
- `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract`
