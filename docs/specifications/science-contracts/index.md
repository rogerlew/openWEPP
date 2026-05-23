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
| `SC-CLIMATE-001` | Climate Forcing Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | `static` | `2026-05-23` |  | CLIM05/CLIM06 amendments added parsed snow/frost runtime coupling requirements (`snow.options.*`, `frost.options.*`), and IRRIG10 amendment added climate schedule-key authority (`day`, `year`) for fixed-date/depletion runtime irrigation coupling; remains non-promotable while `GAP-CLIMATE-003`..`GAP-CLIMATE-005` are open (`GAP-CLIMATE-002` is promotable-with-risk). |
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `static` | `2026-05-23` |  | WB15 amendment added explicit canopy-interception closure coupling so `I` remains distinct from ET outputs (`ET`, `Ws`) while preserving WB11 production guard posture; remains non-promotable while `GAP-EVAP-002` and `GAP-EVAP-003` are open. |
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-20` |  | SCI-12 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-HYD-003` is open. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `Static` | `2026-05-20` |  | SCI-16 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-IMPOUND-001`..`GAP-IMPOUND-003` are open (`GAP-IMPOUND-004` is promotable-with-risk). |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `static` | `2026-05-23` |  | IRRIG10 amendment added concrete runtime alias mappings plus deterministic fixed-date/depletion schedule-source precedence and coupled `Irr` runoff/storage closure authority; remains non-promotable while furrow-runtime and downstream boundary gaps (`GAP-IRRIG-002`, `GAP-IRRIG-003`) remain open. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-05-23` |  | WB13 amendment added percolation/profile output coupling authority (`Dp`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`) on top of WB11 production guard posture; remains non-promotable while `GAP-PERC-002` and `GAP-PERC-003` are open. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-23` |  | WB15 amendment added plant-to-hydrology interception producer authority for `cancov`, `lai`, and `vdmt` payloads (typed missing/non-finite/domain failure posture), on top of PL17 decomposition-kinetics projection authority; promotion remains blocked by cross-contract gap `GAP-PLANT-004`. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-05-23` |  | PL17 amendment added decomposition payload equation-update authority for tracked seed pools (`sumrtm_seed`, `sumsrm_seed`), including event-transfer obligations and required-symbol typed hard-fail guard posture, while retaining INT10 coupled lane-order closure; remains non-promotable while `GAP-RESIDUE-002` and `GAP-RESIDUE-003` are open. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-05-20` |  | SCI-15 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-ROUTE-002`, `GAP-ROUTE-003`, and `GAP-ROUTE-005` are open. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-23` |  | WB15 amendment added canopy interception runtime coupling; IRRIG10 amendment added explicit irrigation schedule-source forcing term (`irrigation.runtime_depth_m`/`Irr`) in runoff closure while retaining WB14 typed runoff guard family; remains non-promotable while `GAP-RUNOFFPART-002`..`GAP-RUNOFFPART-004` are open. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `static` | `2026-05-20` |  | SCI-13 cycle-1 dual review/disposition/verification complete; remains non-promotable while `GAP-SED-002` and `GAP-SED-003` are open (`GAP-SED-001` and `GAP-SED-004` are promotable-with-risk). |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-05-23` |  | CLIM05/CLIM06 amendments added parsed snow and frost runtime coupling authority (`snow.options.*`, `frost.options.*`), explicit signed `S` and frozen-soil infiltration-capacity coupling (`frost.runtime_infcap_frz`), and active-coupling guard posture; promotion remains `HOLD` while non-promotable gaps (`GAP-SNOWFREEZE-002/003/004`) remain open. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-23` |  | CLIM06 amendment added frost-state conductivity coupling authority from parsed frost controls with bounded `Dfrost`/`Dthaw`/`Nft`/`Ws_frz`/`InfCap_frz` surfaces and active-coupling typed hard-fail posture; remains non-promotable while `GAP-SOIL-002` and `GAP-SOIL-003` are open (`GAP-SOIL-001` and `GAP-SOIL-004` are promotable-with-risk). |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `static` | `2026-05-23` |  | WB13 amendment added canonical daily output coupling authority for subsurface/drainage symbols (`latqcc`, `Tile`, `SubRIn`) with deterministic `Qd` relation posture while retaining WB11/WB12 production authority; remains non-promotable while `GAP-SUBHYD-002` and `GAP-SUBHYD-003` are open (`GAP-SUBHYD-001` and `GAP-SUBHYD-004` are promotable-with-risk). |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-05-23` |  | PL15 amendment added residual Tier-A closeout governance authority (`INV-SYSTEM-013`) requiring explicit risk-acceptance references for unresolved blockers and prohibiting silent down-classification / implicit risk acceptance (after PL14 `INV-SYSTEM-012` replay staging guards). |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-23` |  | WB15 amendment added canopy interception closure term authority (`I`); IRRIG10 amendment added explicit irrigation storage-coupling term (`Irr`) in WB12 storage reconciliation with typed runoff/storage guard posture. |

## Entry Order

Sort rows by `contract_id`.
