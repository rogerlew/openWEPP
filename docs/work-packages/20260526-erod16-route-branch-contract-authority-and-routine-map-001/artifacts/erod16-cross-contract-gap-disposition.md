# EROD16 Cross-Contract Gap Disposition

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- EROD16 is a contract-authority package and does not implement runtime
  migration.
- Disposition focus is contract ownership clarity and queue handoff readiness.

## Disposition Table
| item | prior state | EROD16 action | post state |
|---|---|---|---|
| Hillslope `route.for` branch authority location | implicit / fragmented | Canonicalized in `SC-SED-001` addendum with explicit invariants and routine map | closed (authority documented) |
| Watershed vs hillslope routing ownership boundary | partially ambiguous | `SC-ROUTE-001` addendum explicitly partitions WS10 vs `CONTIN -> ROUTE` ownership | closed |
| `rtpart.for` routing provenance ambiguity | incorrect coupling in audit lineage | Corrected with explicit baseline anchor and scope exclusion in both contracts | closed |
| Runtime migration of route branch family | not implemented | Deferred to EROD19 per queue; explicit non-promotable row recorded in `GAP-SED-005` | open (expected) |

## EROD17 Handoff Prerequisites
1. Contract authority for branch-map and provenance correction is complete.
2. Contract-derived test vectors for route branch families can be authored
   against canonical text without ownership ambiguity.
3. Pre-implementation contract gate evidence can target known gaps
   (`GAP-SED-005`) without redefining authority.

## Ran
- `rg -n "GAP-SED-005|EROD16" docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `rg -n "GAP-ROUTE-007|EROD16" docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
