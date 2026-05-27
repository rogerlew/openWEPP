# WSHEDIMPL22 Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHEDIMPL22 landed:
  - WS21 opt-in required seam for class-fraction projection:
    - `ws10_channel_{id}_crfrac_{class:04}`
  - WS21 opt-in detachment-capacity lineage (`dcap` helper) in production path.
  - WS21 opt-in `case34` and `enddet` runtime branch execution.
  - Explicit fail-closed behavior when `crfrac` projection is absent.
- Residual blocker remains:
  - WS21 `case4 -> detach` iterative closure branch (`nt < cnpart`) is still
    unresolved and remains explicitly non-promotable in gap posture.
- Package-governance note:
  - Contract-first sequencing variance is documented in WS22 artifacts; follow-on
    package execution should enforce strict contract-first ordering.

### Immediate next actions
- Prepare and execute follow-on package for remaining iterative branch closure:
  - implement baseline-authoritative `detach.for` iterative closure path for
    WS21 `case4` rows where `nt < cnpart`,
  - remove residual WS21 unresolved-detachment diagnostics from that branch,
  - preserve typed guard and no-surrogate posture.
- Rerun watershed baseline-authoritative comparator lane after iterative branch
  closure and publish updated GO/HOLD disposition for:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed.
