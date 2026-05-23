# Science Contract Registry

Status: Active
Last updated: 2026-05-23

This is the canonical lifecycle registry for openWEPP science contracts.

## Governance Pointers

Kernel-process contract governance is mandatory and integrated through:

1. `docs/specifications/science-contract-authoring-procedure.md`
2. `docs/specifications/science-contracts/kernel-process-contract-profile.md`

For kernel-affecting changes, missing profile/procedure compliance keeps
disposition in `HOLD`.

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
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `static` | `2026-05-23` |  | WB13 amendment added ET component output coupling authority (`Ep`, `Es`, `Er`) for canonical daily water-balance output rows on top of WB11 production guards; remains non-promotable while `GAP-EVAP-002` and `GAP-EVAP-003` are open. |
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-20` |  | SCI-12 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-HYD-003` is open. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `Static` | `2026-05-20` |  | SCI-16 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-IMPOUND-001`..`GAP-IMPOUND-003` are open (`GAP-IMPOUND-004` is promotable-with-risk). |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `static` | `2026-05-20` |  | SCI-14 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-IRRIG-002` and `GAP-IRRIG-003` are open. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-05-23` |  | WB13 amendment added percolation/profile output coupling authority (`Dp`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`) on top of WB11 production guard posture; remains non-promotable while `GAP-PERC-002` and `GAP-PERC-003` are open. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-23` |  | INT10 amendment added explicit coupled lane-ordering invariant/guard authority for `decomp -> growth -> watbal` with typed failure posture for missing/non-finite ordering symbols; promotion remains blocked by cross-contract gap `GAP-PLANT-004`. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-05-23` |  | INT10 amendment extended PL12/PL13 transition authority with coupled replay lane-order closure (`INV-RESIDUE-016`) and ordering/state-transfer replay obligations; remains non-promotable while `GAP-RESIDUE-002` and `GAP-RESIDUE-003` are open. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-05-20` |  | SCI-15 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-ROUTE-002`, `GAP-ROUTE-003`, and `GAP-ROUTE-005` are open. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-23` |  | WB13 amendment added canonical daily output coupling authority for runoff/runon symbols (`Q`, `QOFE`, `UpStrmQ`, `RM`, `P`) on top of WB12 reconciliation posture; remains non-promotable while `GAP-RUNOFFPART-002`..`GAP-RUNOFFPART-004` are open. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `static` | `2026-05-20` |  | SCI-13 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-SED-002` and `GAP-SED-003` are open (`GAP-SED-001` and `GAP-SED-004` are promotable-with-risk). |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-05-20` |  | SCI-05 cycle-1 complete with dual review/disposition/verification; promotion remains `HOLD` while non-promotable gaps (`GAP-SNOWFREEZE-002/003/004`) remain open. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-20` |  | SCI-10 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-SOIL-002` and `GAP-SOIL-003` are open (`GAP-SOIL-001` and `GAP-SOIL-004` are promotable-with-risk). |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `static` | `2026-05-23` |  | WB13 amendment added canonical daily output coupling authority for subsurface/drainage symbols (`latqcc`, `Tile`, `SubRIn`) with deterministic `Qd` relation posture while retaining WB11/WB12 production authority; remains non-promotable while `GAP-SUBHYD-002` and `GAP-SUBHYD-003` are open (`GAP-SUBHYD-001` and `GAP-SUBHYD-004` are promotable-with-risk). |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-05-23` |  | PL14 amendment added strict replay artifact/provenance completeness authority (`INV-SYSTEM-012`) with explicit `abs_tol=0`/`rel_tol=0` Tier-A closeout lane posture and no-fallback replay guard semantics. |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-23` |  | PL14 amendment added WB13 replay-candidate emission invariant/guard authority (`INV-WATBAL-012`) requiring canonical 25-column schema/order and explicit failure for missing replay artifacts. |

## Entry Order

Sort rows by `contract_id`.
