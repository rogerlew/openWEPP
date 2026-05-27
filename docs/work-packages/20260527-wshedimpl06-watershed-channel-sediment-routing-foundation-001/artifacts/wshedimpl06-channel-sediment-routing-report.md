# WSHEDIMPL06 Channel Sediment Routing Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented WS11 channel sediment publication-family closure in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - `ws10_channel_{id}_qsed`
  - `ws10_channel_{id}_tc`
- Added deterministic helper
  `assemble_incoming_sediment_load_and_capacity` to compute contributor
  sediment load and publication-capacity state with fail-closed typed
  non-finite/domain guards.
- Updated WS11 expected-failure coverage posture by promoting
  `wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family`
  to active conformance in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`.
- Synchronized contract gap posture for WSHED06 scope in:
  - `SC-ROUTE-001` (`GAP-ROUTE-009` narrowed),
  - `SC-SED-001` (`GAP-SED-006` impact text normalized),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008` impact text normalized),
  - `docs/specifications/science-contracts/index.md`.

## Ran
- `cargo fmt`
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
