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
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `none` | `pending` |  | Seeded by SCI-07 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-20` |  | SCI-02 reopened for procedure-delta compliance (guard map + symbol alias map); remains non-promotable while GAP-PLANT-004 and GAP-PLANT-005 are open. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-20` |  | SCI-06 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-RUNOFFPART-002`..`GAP-RUNOFFPART-004` are open. |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `open` | `proposed` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `none` | `pending` |  | Seeded by SCI-05 work-package prep; requires dual-agent review/disposition cycle before promotion. |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-20` |  | SCI-04 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-WATBAL-002` and `GAP-WATBAL-003` are open. |

## Entry Order

Sort rows by `contract_id`.
