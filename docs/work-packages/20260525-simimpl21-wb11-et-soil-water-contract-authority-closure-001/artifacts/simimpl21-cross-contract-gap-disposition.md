# SIMIMPL21 Cross-Contract Gap Disposition

Status: complete-with-hold
Evidence mode: static+ran
Date: 2026-05-25
Decision: HOLD

## Static
| Contract gap | Post-SIMIMPL21 posture | Disposition rationale |
|---|---|---|
| `SC-EVAP-001 GAP-EVAP-005` | `non-promotable` | Stage-memory and uptake-lineage authority is now explicit, but runtime migration and contract-derived tests remain queued (`SIMIMPL22/SIMIMPL23`). |
| `SC-WATBAL-001 GAP-WATBAL-002` | `non-promotable` | Companion contracts are explicit; full WB11 ET/soil-water runtime promotability remains pending follow-on execution. |
| `SC-PLANT-001 GAP-PLANT-004` | `non-promotable` | Stress/uptake lineage authority is explicit, but coupled runtime closure for WB11 lineage obligations is not yet implemented/tested. |
| `SC-SOIL-001 GAP-SOIL-002` | `promotable-with-risk` | WB11 ET/soil-water alias-lineage authority is closed for scoped surfaces; broader soil alias harmonization remains open. |
| `SC-SYSTEM-001 GAP-SYSTEM-002` | `non-promotable` | WB13 ET/soil-water alias obligations are now explicit, but broader system-boundary alias finalization is incomplete. |

- SIMIMPL21 closes contract-authority scope only; downstream test and runtime
  implementation packages remain mandatory before hold-lift.

## Ran
- `rg -n "GAP-EVAP-005|INV-EVAP-013" docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `rg -n "GAP-WATBAL-002|INV-WATBAL-028|INV-WATBAL-029" docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `rg -n "GAP-PLANT-004|INV-PLANT-023" docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `rg -n "GAP-SOIL-002|INV-SOIL-013" docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `rg -n "GAP-SYSTEM-002|INV-SYSTEM-027" docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
