# WSHEDIMPL06 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Updated hillslope sediment payload validator in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` so
  `read_hillslope_sediment_payload` returns net sediment mass
  (`max(total_detachment - total_deposition, 0.0)`) while preserving full
  payload-family guard checks.
- Added channel helper:
  `assemble_incoming_sediment_load_and_capacity`:
  - sums contributor sediment mass,
  - computes `qsed = mass / event_duration`,
  - publishes `tc = qsed`,
  - hard-fails on non-finite/domain-invalid states.
- WS10 channel writeback now publishes:
  - `ws10_channel_{id}_qsed`
  - `ws10_channel_{id}_tc`
- Promoted WS11 WSHED03 sediment expected-failure vector to active conformance.
- Updated `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`, and
  `science-contracts/index.md` to reflect WSHED06 publication-family closure
  and retained residual process-parity blockers.

## Ran
- `cargo fmt`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
- `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract`
- `cargo test --workspace`
  - result: failed on existing unrelated lane:
    `erod13_registry_updates_reference_wave1_authority`
- `cargo deny check`
  - result: pass with existing duplicate/unmatched-license warnings
