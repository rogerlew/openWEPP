# WSHEDIMPL23 Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHEDIMPL23 landed:
  - Baseline-authoritative WS21 `case4 -> detach` iterative closure
    (`nt < cnpart`) migrated into runtime path.
  - Residual WS21 unresolved-detachment fallback for this branch removed.
  - WS23 WS11 vector asserts
    `ws10_channel_1_ws21_detach_unmigrated_segment_count == 0.0` for migrated
    branch execution.
  - WS22 fail-closed `crfrac` seam and WS21 `dcap/case34/enddet` lineage
    remain active.
- Residual blockers remain out-of-scope:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`
  - Remaining full `chnero/chnrt` parity families and validation closure.

### Immediate next actions
- Prepare and execute follow-on package for remaining `chnero/chnrt` parity
  closure families and comparator evidence promotion:
  - map remaining baseline route/erosion branch families to canonical `SC-*`
    contract rows,
  - add contract-derived vectors for uncovered branch families,
  - migrate remaining production branch logic with typed-guard posture,
  - rerun comparator lane(s) and update GO/HOLD disposition for:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed.
