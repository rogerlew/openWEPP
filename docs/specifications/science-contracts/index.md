# Science Contract Registry

Status: Active
Last updated: 2026-05-20

This is the canonical lifecycle registry for openWEPP science contracts.

## Registry Fields

| Field | Required | Description |
|---|---|---|
| `contract_id` | Yes | Stable ID matching `SC-<DOMAIN>-<NNN>`. |
| `title` | Yes | Human-readable contract title. |
| `status` | Yes | Lifecycle status (`open`, `in_review`, `approved`, `retired`). |
| `maturity` | Yes | `proposed`, `draft`, `active`, or `deprecated`. |
| `owner` | Yes | Named maintainer or review group. |
| `path` | Yes | Relative path to canonical contract file. |
| `evidence_level` | Yes | Highest evidence level currently supporting the contract. |
| `last_reviewed` | Yes | UTC date or `pending`. |
| `replacement` | No | Replacement contract ID when deprecated. |
| `notes` | No | Short scope/lifecycle note. |

## Current Registry

| contract_id | title | status | maturity | owner | path | evidence_level | last_reviewed | replacement | notes |
|---|---|---|---|---|---|---|---|---|---|
| `SC-CLIMATE-001` | Climate Forcing Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | `static` | `2026-05-20` |  | SCI-03 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-CLIMATE-003`..`GAP-CLIMATE-005` are open (`GAP-CLIMATE-002` is promotable-with-risk). |
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `static` | `2026-05-20` |  | SCI-07 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-EVAP-002` and `GAP-EVAP-003` are open. |
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-20` |  | SCI-12 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-HYD-003` is open. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `none` | `pending` |  | Seeded by SCI-16 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `none` | `pending` |  | Seeded by SCI-14 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-05-20` |  | SCI-08 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-PERC-002` and `GAP-PERC-003` are open. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-20` |  | SCI-02 reopened for procedure-delta compliance (guard map + symbol alias map); remains non-promotable while GAP-PLANT-004 and GAP-PLANT-005 are open. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-05-20` |  | SCI-11 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-RESIDUE-002` and `GAP-RESIDUE-003` are open. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-05-20` |  | SCI-15 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-ROUTE-002`, `GAP-ROUTE-003`, and `GAP-ROUTE-005` are open. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-20` |  | SCI-06 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-RUNOFFPART-002`..`GAP-RUNOFFPART-004` are open. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `none` | `pending` |  | Seeded by SCI-13 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-05-20` |  | SCI-05 cycle-1 complete with dual review/disposition/verification; promotion remains `HOLD` while non-promotable gaps (`GAP-SNOWFREEZE-002/003/004`) remain open. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-20` |  | SCI-10 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-SOIL-002` and `GAP-SOIL-003` are open (`GAP-SOIL-001` and `GAP-SOIL-004` are promotable-with-risk). |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `none` | `pending` |  | Seeded by SCI-09 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-05-20` |  | SCI-17 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-SYSTEM-001` and `GAP-SYSTEM-002` are open (`GAP-SYSTEM-003` and `GAP-SYSTEM-004` are promotable-with-risk). |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-20` |  | SCI-04 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-WATBAL-002` and `GAP-WATBAL-003` are open. |

## Entry Order

Sort rows by `contract_id`.
