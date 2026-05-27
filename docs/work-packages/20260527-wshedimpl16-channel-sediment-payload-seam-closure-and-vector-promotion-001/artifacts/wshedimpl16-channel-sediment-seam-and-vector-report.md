# WSHEDIMPL16 Channel Sediment Seam and Vector Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Closed seam:
  - contributor `particle_diameter_m[npart]` now persists from HBP parse
    surfaces into WS10 runtime contributor payload symbols.
- Guard closure:
  - WS10 now fails closed on missing/non-finite/non-positive
    `hs{ID}_particle_diameter_m_{class:04}`.
- Vector promotion:
  - WS11 sediment vector now validates current production branch-equation
    closure for `qsed` and `tc` instead of publication presence only.
- Residual open scope:
  - Full `chnero/chnrt/detach` segment process migration is still unresolved and
    remains explicitly non-promotable (`GAP-SYSTEM-008`, `GAP-ROUTE-009`,
    `GAP-SED-006`).

## Ran
- `cargo test --workspace --test ws11_channel_routing_physics_equivalence_contract` passed.
- `cargo test --workspace --test watershed_cli_behavior_contract` passed,
  including baseline-authoritative comparator-lane test
  `wshedimpl14_baseline_authoritative_cli_lane_replays_baseline_ebe_signature`.
