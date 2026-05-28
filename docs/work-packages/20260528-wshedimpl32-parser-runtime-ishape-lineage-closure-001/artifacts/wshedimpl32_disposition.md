# WSHEDIMPL32 Disposition

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Decision: HOLD
- Scope completion: complete for declared WS32 parser/runtime `ishape` lineage
  seam.
- Closed in this package:
  - aligned watershed channel parser strict-domain authority to accept
    naturally eroded class (`ishape=3`),
  - aligned compatibility normalization authority to map legacy out-of-domain
    `ishape>3` values to naturally eroded class (`3`) with explicit warning,
  - added explicit WS10 runtime seed guard for `ws10_channel_{id}_ishape`
    domain `[1,3]` with typed failure on violation,
  - added parser/runtime seam vectors and fixtures that verify naturally
    eroded class projection continuity and out-of-domain rejection behavior,
  - amended contract/index rows for WS32 traceability.
- Remaining blockers:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`

## Ran
- Validation gates recorded in `gate-results.md`.
