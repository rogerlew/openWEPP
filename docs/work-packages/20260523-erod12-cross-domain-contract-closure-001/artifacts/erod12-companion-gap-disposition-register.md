# EROD12 Companion Gap Disposition Register

Status: `completed`
Evidence mode: `Static + Ran`

## Wave-0 Target Rows (EROD10-AH-002)

| gap_id | contract | pre-EROD12 | post-EROD12 | disposition basis |
|---|---|---|---|---|
| `GAP-SED-003` | `SC-SED-001` | `non-promotable` | `closed` | Canonical cross-domain ownership/guard closure addendum ratifies required hydraulics/routing boundary guard ownership. |
| `GAP-HYD-003` | `SC-HYDRAULICS-001` | `non-promotable` | `closed` | Canonical hydrology-to-erosion ownership and guard semantics are explicit across hydraulics/sediment boundary surfaces. |
| `GAP-ROUTE-003` | `SC-ROUTE-001` | `non-promotable` | `closed` | Canonical routing cross-domain ownership/guard semantics are explicit for required erosion-lane boundaries. |
| `GAP-RUNOFFPART-004` | `SC-RUNOFFPART-001` | `non-promotable` | `closed` | Canonical runoff companion ownership/guard semantics are explicit for required erosion-lane runoff boundaries. |

## Non-Wave-0 Holds Explicitly Retained

| gap_id | contract | status | retention rationale |
|---|---|---|---|
| `GAP-ROUTE-005` | `SC-ROUTE-001` | `non-promotable` | Chapter-13 applicability runtime guard binding remains a separate non-Wave-0 governance hold. |
| `GAP-RUNOFFPART-003` | `SC-RUNOFFPART-001` | `non-promotable` | Hortonian-only scope limitation remains explicit and promotion-blocking outside declared scope. |
| `GAP-WATBAL-002` | `SC-WATBAL-001` | `non-promotable` | Broader downstream companion completion posture remains outside EROD12 Wave-0 ownership closure scope. |
| `GAP-SYSTEM-001` | `SC-SYSTEM-001` | `non-promotable` | Broader watershed-release promotability remains provisional despite explicit Wave-0 erosion-boundary closure. |

Static:
- Row disposition outcomes are canonicalized in `SC-*` gap registers.

Ran:
- Gap-row statuses were verified by row-scoped inspection and by executing the
  EROD12 integration contract test.
