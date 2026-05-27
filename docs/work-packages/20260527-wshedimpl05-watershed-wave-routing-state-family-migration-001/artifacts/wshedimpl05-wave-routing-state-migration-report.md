# WSHEDIMPL05 Wave-Routing State Migration Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented WS11 `ipeak > 2` wave-routing state-family publication in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - `ws10_channel_{id}_q1`
  - `ws10_channel_{id}_qin`
  - `ws10_channel_{id}_qlat`
  - `ws10_channel_{id}_c0`
  - `ws10_channel_{id}_c1`
  - `ws10_channel_{id}_c2`
  - `ws10_channel_{id}_c3`
  - `ws10_channel_{id}_c4`
- Added explicit branch-local helper routines for KW/MC wave-state
  computation with typed non-finite/domain failure behavior.
- Promoted WSHED03 WS11 wave-state vector to active conformance by removing
  ignore posture in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`.
- Synchronized contract gap posture in `SC-ROUTE-001` and registry note in
  `docs/specifications/science-contracts/index.md`.

## Ran
- `cargo fmt`
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
