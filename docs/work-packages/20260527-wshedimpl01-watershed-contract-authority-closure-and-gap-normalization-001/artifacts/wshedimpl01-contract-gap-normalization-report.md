# WSHEDIMPL01 Contract Gap Normalization Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static

### Normalized residual gap mapping

| WSHEDPLAN01 gap lineage | Canonicalized contract rows in WSHEDIMPL01 | Follow-on owner |
|---|---|---|
| `WSHED-GAP-001`, `WSHED-GAP-002`, `WSHED-GAP-003` (WS11 branch/runtime hydrology closure) | `SC-ROUTE-001`: `GAP-ROUTE-008` | `WSHED04`, `WSHED05` |
| `WSHED-GAP-004` (channel sediment migration closure) | `SC-ROUTE-001`: `GAP-ROUTE-009`; `SC-SED-001`: `GAP-SED-006`; `SC-SYSTEM-001`: `GAP-SYSTEM-008` | `WSHED06` |
| `WSHED-GAP-005`, `WSHED-GAP-006` (impoundment continuity + runtime seam closure) | `SC-IMPOUND-001`: `GAP-IMPOUND-005`, `GAP-IMPOUND-006`; `SC-SYSTEM-001`: `GAP-SYSTEM-007` | `WSHED04`, `WSHED07` |
| `WSHED-GAP-007` (watershed parquet publication blocker) | `SC-SYSTEM-001`: `GAP-SYSTEM-006` | `WSHED08` |
| `WSHED-GAP-008` (missing end-to-end fixture/vector closure) | `SC-SYSTEM-001`: `GAP-SYSTEM-005` | `WSHED03`, `WSHED09` |
| `WSHED-GAP-009` (baseline provenance correction) | `SC-ROUTE-001`: added `REF-ROUTE-CHRQIN-WAVE`; channel sediment lineage rows retained on `detach.for` authority | `WSHED03+` evidence consumers |

### Contract authority deltas applied
- `SC-ROUTE-001`:
  - added explicit `chrqin.for` wave-routing lineage anchor.
  - normalized unresolved runtime rows for WS11 branch migration and channel
    sediment migration.
- `SC-IMPOUND-001`:
  - normalized unresolved runtime rows for WS12 RK4/regime-transition closure
    and parser-to-runtime coefficient projection closure.
- `SC-SED-001`:
  - normalized explicit cross-domain watershed sediment closure dependency row
    while retaining hillslope boundary authority closure.
- `SC-SYSTEM-001`:
  - normalized explicit system-blocking rows for end-to-end vector coverage,
    `OWSOUT-E-004`, WS12 seam dependency, and channel-sediment integration.

### Expected next-package validation posture
- `WSHED03` must add failing contract-derived vectors for new unresolved gap
  rows before runtime migration.
- `WSHED04+` runtime seam packages must remove synthetic/manual coefficient
  seeding and satisfy typed guard posture in production paths.
- `WSHED08` must eliminate `OWSOUT-E-004` from valid watershed execution lanes.
- `WSHED09` must run full gate/comparator disposition for closure claims.

## Ran
- `rg -n` and `sed -n` over:
  - `SC-ROUTE-001.md`
  - `SC-IMPOUND-001.md`
  - `SC-SED-001.md`
  - `SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`
  - WSHEDPLAN01 gap/queue artifacts for source-gap lineage mapping.
