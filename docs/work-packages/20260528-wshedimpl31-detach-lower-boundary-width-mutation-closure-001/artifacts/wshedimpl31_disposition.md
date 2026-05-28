# WSHEDIMPL31 Disposition

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Decision: HOLD
- Scope completion: complete for declared WS31 seam.
- Closed in this package:
  - migrated baseline `detach.for` lower-boundary rectangular width mutation
    semantics (`wera(i) -> wida(i)` when `flagc=2` and `wera>wfl`) into
    WS23/WS24 closure paths.
  - added WS10 `wida_{point:04}` state writeback publication for WS20 opt-in
    routing outcomes.
  - amended contract/index rows for WS31 traceability.
- Remaining blockers:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- Validation gates recorded in `gate-results.md`.
