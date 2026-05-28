# WSHEDIMPL24 Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHEDIMPL24 landed:
  - Baseline-authoritative `case12.for` transition continuation
    (`xdemax < x(i)` into `detach.for`) migrated into WS20 segment routing.
  - Runtime now publishes
    `ws10_channel_{id}_ws24_case2_detach_segment_count`.
  - WS24 vectors verify both missing-`crfrac` fail-closed behavior and
    successful routed transition path with `crfrac` projection.
  - WS22 fail-closed `crfrac` seam and WS21/WS23 migrated detach-capacity
    lineage remain active.
- Residual blockers remain out-of-scope:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`
  - Remaining full `chnero/chnrt` parity families and validation closure.

### Immediate next actions
- Prepare follow-on package to migrate remaining baseline-authoritative
  `chnero/chnrt/detach` branch families not yet represented in WS20/WS21/WS24:
- Map uncovered branch families in `chnrt.for` to canonical `SC-*` rows.
- Add contract-derived WS11 vectors for each uncovered branch family.
- Migrate remaining production branch logic with typed-guard posture.
- Preserve explicit HOLD posture until residual blockers are closed:
  `GAP-ROUTE-009`, `GAP-SED-006`, `GAP-SYSTEM-008`.

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed
  (`24 passed; 0 failed`).
