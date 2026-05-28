# WSHEDIMPL36 Disposition

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Decision: HOLD
- Scope completion: complete for declared WSHEDIMPL36 parser/runtime
  rating-curve control-lineage seam.
- Closed in this package:
  - aligned watershed channel parser authority and WS10 runtime seed validation
    on explicit `icntrl==4` rating-curve payload-shape and domain semantics,
  - added explicit WS10 runtime seed projection and fail-closed guard continuity
    for `ws10_channel_{id}_{rccoef,rcexp,rcoset}`,
  - added parser/runtime seam vectors and fixtures verifying out-of-domain and
    payload-shape fail-closed behavior,
  - amended contract/index rows for WSHEDIMPL36 traceability.
- Remaining blockers:
  - `GAP-ROUTE-008`
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- Validation gates recorded in `gate-results.md`.
