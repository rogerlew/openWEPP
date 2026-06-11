# SCSTRUCT07 SUBHYD Binding Crosswalk

Evidence: Static
Date: 2026-06-11

## Conservation

- Binding IDs removed: none.
- Binding IDs added: none.
- Binding IDs weakened: none.
- Kernel/runtime files edited: none.
- Comparator re-tiering: none.
- Narrative relocated: none.
- Promoted new `INV-*` / `OBL-*`: none.

## Mapping Crosswalk

| Entry cohort | Conserved binding IDs |
|---|---|
| WB12/WB13 coupling and anti-shadow rows | `INV-SUBHYD-009`, `INV-SUBHYD-021`, `INV-SUBHYD-022` |
| WB13 robustness / coupled residual / threshold rows | `INV-SUBHYD-009`, `INV-SUBHYD-012`, `INV-SUBHYD-014`, `INV-SUBHYD-019`, `INV-SUBHYD-021`, `INV-SUBHYD-025`, `INV-SUBHYD-026` |
| WB19 water-yield / branch / Level-4 constitutive rows | `INV-SUBHYD-015`, `INV-SUBHYD-016`, `INV-SUBHYD-017`, `INV-SUBHYD-018`, `INV-SUBHYD-019`, `INV-SUBHYD-024` |
| WB19 hourly lane / handoff / tail / trace rows | `INV-SUBHYD-020`, `INV-SUBHYD-021`, `INV-SUBHYD-022`, `INV-SUBHYD-023`, `INV-SUBHYD-024`, `INV-SUBHYD-025`, `INV-SUBHYD-026`, `INV-SUBHYD-027`, `INV-SUBHYD-028`, `INV-SUBHYD-029`, `INV-SUBHYD-030`, `INV-SUBHYD-031` |

## Level-4 Suite Linkage

No new Level-4 suite linkage was promoted by SCSTRUCT07. Existing suite-linked
rows remain mapped to their existing invariants:

| Entry | Suite linkage retained through |
|---|---|
| `HPHYS0224-WB19-REALIZED-WITHDRAWAL-SOIL-WATER-CAP-ADDENDUM` | `INV-SUBHYD-016` |
| `HPHYS0225-WB19-LAYER-POOL-AVAILABLE-CAP-AUTHORITY-ADDENDUM` | `INV-SUBHYD-017` |
| `HPHYS0226-WB19-LATERAL-SATURATED-THICKNESS-RESPONSE-ADDENDUM` | `INV-SUBHYD-018` |
| `HPHYS0227-WB19-FCWP-COCA-WATER-YIELD-COUPLING-ADDENDUM` | `INV-SUBHYD-019` |

## Retained Core Authority

Every row resolved as map-in-core remains in `SC-SUBHYD-001` because the row
carries active constitutive, guard, vector, or cross-domain context. No narrative
was sidecar-eligible in this pass.
