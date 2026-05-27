# WSHEDIMPL21 Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHEDIMPL21 scope is complete:
  - WS21 opt-in control intake added:
    - `ws10_channel_{id}_ws21_case34_enable`
  - WS21 diagnostics publication is active:
    - `ws21_case3_segment_count`
    - `ws21_case4_segment_count`
    - `ws21_enddet_segment_count`
    - `ws21_detach_unmigrated_segment_count`
  - WS21 case3/case4 scaffolding is wired in the unresolved-detachment branch,
    keeping detach/dcap unresolved behavior explicit and fail-visible.
  - WS11 vectors enforce WS21 default-off and WS20+WS21 opt-in continuity.
- Canonical contract/index posture updated for WS21 diagnostics scope.
- Residual blockers remain open and non-promotable:
  - `GAP-SYSTEM-008`
  - `GAP-ROUTE-009`
  - `GAP-SED-006`

### Immediate next actions
- Prepare and execute follow-on package for baseline-authoritative detachment
  closure:
  - implement production `detach/dcap` math lineage in watershed runtime path
    (no surrogate fallback),
  - replace WS21 unresolved-detachment scaffold with authoritative segment
    routing for case34/enddet closure,
  - retain typed guard behavior and explicit domain-failure handling.
- After detach/dcap closure, rerun watershed baseline-authoritative comparator
  lane and publish updated GO/HOLD disposition for:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed.
- Full validation gates in `gate-results.md` passed.
