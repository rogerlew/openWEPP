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
| `SC-CLIMATE-001` | Climate Forcing Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | `static` | `2026-05-23` |  | CLIM05/CLIM06 amendments added parsed snow/frost runtime coupling requirements (`snow.options.*`, `frost.options.*`), IRRIG10 amendment added climate schedule-key authority (`day`, `year`) for fixed-date/depletion runtime irrigation coupling, CLIM07 added continuous-daily/breakpoint comparator-seam vector obligations plus confidence-tier routing evidence, and CLIM08 ratified seam-HOLD closure mapping (including parser/runtime boundary closure via `SC-INFILE-CLIMATE-001` `CLI-GAP-002`); overall contract remains non-promotable while `GAP-CLIMATE-003`..`GAP-CLIMATE-005` are open (`GAP-CLIMATE-002` is promotable-with-risk). |
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `static` | `2026-05-23` |  | WB17 amendment replaced WB11 ET surrogate authority with equation-driven ET partition semantics (`Esp`, `Etp`, `Er`, `Es`, `Ep`, `ET`, `Ws`) and explicit runtime alias mapping (`Eu -> wb11_et_demand`, `L -> lai`, `Er -> wb17_residue_interception`); remains non-promotable while companion-contract and full stage-memory closure gaps remain open. |
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered hydraulics-coupled interfaces (`HillslopeProduction*Symbol`, `WatershedProduction*Symbol`) while preserving WB14/WB16/WS10 guard families; EROD11 ratified Wave-0 alias ownership and downgraded `GAP-HYD-002` to promotable-with-risk. Contract remains non-promotable while `GAP-HYD-003` is open. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered WS10 impoundment interfaces (`WatershedProduction*Symbol` + node-scoped builders) while preserving `WKERNEL-WS10-IMPOUNDMENT-E-001..003`; remains non-promotable while `GAP-IMPOUND-001`..`GAP-IMPOUND-003` are open (`GAP-IMPOUND-004` is promotable-with-risk). |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `static` | `2026-05-23` |  | IRRIG10 amendment added concrete runtime alias mappings plus deterministic fixed-date/depletion schedule-source precedence and coupled `Irr` runoff/storage closure authority; remains non-promotable while furrow-runtime and downstream boundary gaps (`GAP-IRRIG-002`, `GAP-IRRIG-003`) remain open. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-05-23` |  | WB18 amendment replaced WB11 scalar surrogate authority with per-layer percolation runtime authority (`wb18_perc_theta/fc/ul/ssc/pei_####`) and conductivity-domain routing semantics; WB13 profile-output coupling authority remains active. Contract remains non-promotable while `GAP-PERC-003` is open. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered hydrology/plant coupling interfaces (`HillslopeProduction*Symbol`) while preserving WB15 missing/non-finite/domain failure posture; promotion remains blocked by cross-contract gap `GAP-PLANT-004`. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered residue-coupled interfaces (`HillslopeProduction*Symbol`) while preserving PL17/INT10 typed failure posture; remains non-promotable while `GAP-RESIDUE-002` and `GAP-RESIDUE-003` are open. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered WS10 routing interfaces (`WatershedProduction*Symbol` + node/hillslope builders) while preserving `WKERNEL-WS10-CHANNEL-E-001..003`; EROD11 ratified Wave-0 alias ownership and downgraded `GAP-ROUTE-002` to promotable-with-risk. Contract remains non-promotable while `GAP-ROUTE-003` and `GAP-ROUTE-005` are open. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered runoff-partition interfaces (`HillslopeProduction*Symbol`) while preserving WB14/WB15/WB16 guard families; EROD11 ratified Wave-0 alias ownership and downgraded `GAP-RUNOFFPART-002` to promotable-with-risk. Contract remains non-promotable while `GAP-RUNOFFPART-003` and `GAP-RUNOFFPART-004` are open. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `static` | `2026-05-23` |  | WB16 amendment added hydrologic peak/duration intake authority (`peakro`, `watdur`) with continuity/traceability and typed-guard requirements for erosion coupling readiness; EROD11 ratified Wave-0 alias ownership and downgraded `GAP-SED-002` to promotable-with-risk. Contract remains non-promotable while `GAP-SED-003` is open (`GAP-SED-001`, `GAP-SED-002`, and `GAP-SED-004` are promotable-with-risk). |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-05-23` |  | CLIM05/CLIM06 amendments added parsed snow and frost runtime coupling authority (`snow.options.*`, `frost.options.*`), explicit signed `S` and frozen-soil infiltration-capacity coupling (`frost.runtime_infcap_frz`), and active-coupling guard posture; promotion remains `HOLD` while non-promotable gaps (`GAP-SNOWFREEZE-002/003/004`) remain open. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-23` |  | CLIM06 amendment added frost-state conductivity coupling authority from parsed frost controls with bounded `Dfrost`/`Dthaw`/`Nft`/`Ws_frz`/`InfCap_frz` surfaces and active-coupling typed hard-fail posture; remains non-promotable while `GAP-SOIL-002` and `GAP-SOIL-003` are open (`GAP-SOIL-001` and `GAP-SOIL-004` are promotable-with-risk). |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `static` | `2026-05-23` |  | WB13 amendment added canonical daily output coupling authority for subsurface/drainage symbols (`latqcc`, `Tile`, `SubRIn`) with deterministic `Qd` relation posture while retaining WB11/WB12 production authority; remains non-promotable while `GAP-SUBHYD-002` and `GAP-SUBHYD-003` are open (`GAP-SUBHYD-001` and `GAP-SUBHYD-004` are promotable-with-risk). |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered hillslope/watershed integration interfaces (`HillslopeProduction*Symbol`, `WatershedProduction*Symbol`) while preserving deterministic publication and existing failure-class/message continuity; PL14R amendment added rerun reproducibility authority (`INV-SYSTEM-014`); PL15R amendment added recloseout supersession governance authority requiring active blocker classification from latest schema-aligned strict replay evidence (`INV-SYSTEM-015`). |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-23` |  | WB18 amendment updated hydrology authority to WB17 ET + WB18 per-layer percolation + WB11 lateral/drain execution, including explicit per-layer percolation symbol aliases and guard posture continuity; EROD11 Wave-0 runoff/peak ownership and PL14R/PL15R replay governance requirements remain active. |

## Entry Order

Sort rows by `contract_id`.
