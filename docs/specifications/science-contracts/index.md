# Science Contract Registry

Status: Active
Last updated: 2026-05-24

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
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-24` |  | WS11 amendment added channel-routing consumer-coupling authority requiring explicit `ipeak` branch semantics (Rational/CREAMS/KW/MC) and deauthorizing gain-factor surrogate substitution while preserving `WKERNEL-WS10-CHANNEL-E-001..003`; WS12 impoundment consumer-coupling authority, ARCH22, and EROD11/EROD12 closure posture remain active. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `static` | `2026-05-24` |  | WS12 amendment replaced WS10 headroom-retention surrogate parity authority with legacy-equivalent continuity + stage-discharge impoundment authority (with baseline provenance anchors in `imphnw/impflo/impmai/wshiqi/wshimp`) while preserving `WKERNEL-WS10-IMPOUNDMENT-E-001..003`; remains non-promotable while `GAP-IMPOUND-001`..`GAP-IMPOUND-003` are open (`GAP-IMPOUND-004` is promotable-with-risk). |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `static` | `2026-05-23` |  | IRRIG10 amendment added concrete runtime alias mappings plus deterministic fixed-date/depletion schedule-source precedence and coupled `Irr` runoff/storage closure authority; remains non-promotable while furrow-runtime and downstream boundary gaps (`GAP-IRRIG-002`, `GAP-IRRIG-003`) remain open. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-05-23` |  | WB18 amendment replaced WB11 scalar surrogate authority with per-layer percolation runtime authority (`wb18_perc_theta/fc/ul/ssc/pei_####`) and conductivity-domain routing semantics; WB13 profile-output coupling authority remains active. Contract remains non-promotable while `GAP-PERC-003` is open. |
| `SC-PLANT-001` | Plant Growth Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered hydrology/plant coupling interfaces (`HillslopeProduction*Symbol`) while preserving WB15 missing/non-finite/domain failure posture; promotion remains blocked by cross-contract gap `GAP-PLANT-004`. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-05-23` |  | ARCH22 amendment added typed production-surface authority for covered residue-coupled interfaces (`HillslopeProduction*Symbol`) while preserving PL17/INT10 typed failure posture; remains non-promotable while `GAP-RESIDUE-002` and `GAP-RESIDUE-003` are open. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-05-24` |  | WS11 amendment replaced WS10 gain-factor surrogate routing authority with legacy-equivalent `ipeak`-selected branch authority (Rational/CREAMS/KW/MC) anchored to pinned baseline provenance (`wshcqi/wshdrv/wshpek/wshchr`) while preserving `WKERNEL-WS10-CHANNEL-E-001..003`; ARCH22 and EROD11/EROD12 closure posture remain active. Contract remains non-promotable while applicability guard row `GAP-ROUTE-005` is open (WS11 lineage-note row `GAP-ROUTE-006` is promotable-with-risk). |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-05-23` |  | WB20 forward-solver lane authority remains active; EROD11/EROD12 closure amendments dispositioned Wave-0 alias/cross-domain ownership rows (`GAP-RUNOFFPART-002`, `GAP-RUNOFFPART-004`) to `closed` and canonicalized erosion-lane guard ownership for required runoff boundaries. Contract remains non-promotable while Hortonian-scope governance row `GAP-RUNOFFPART-003` is open. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `static` | `2026-05-23` |  | WB16 hydrologic peak/duration intake authority remains active; EROD11/EROD12 closure amendments dispositioned `GAP-SED-001`..`GAP-SED-004` to `closed` and canonicalized cross-domain ownership/guard semantics for required hydraulics/routing erosion boundaries. |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-05-23` |  | CLIM05/CLIM06 amendments added parsed snow and frost runtime coupling authority (`snow.options.*`, `frost.options.*`), explicit signed `S` and frozen-soil infiltration-capacity coupling (`frost.runtime_infcap_frz`), and active-coupling guard posture; promotion remains `HOLD` while non-promotable gaps (`GAP-SNOWFREEZE-002/003/004`) remain open. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-23` |  | CLIM06 amendment added frost-state conductivity coupling authority from parsed frost controls with bounded `Dfrost`/`Dthaw`/`Nft`/`Ws_frz`/`InfCap_frz` surfaces and active-coupling typed hard-fail posture; remains non-promotable while `GAP-SOIL-002` and `GAP-SOIL-003` are open (`GAP-SOIL-001` and `GAP-SOIL-004` are promotable-with-risk). |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `static` | `2026-05-23` |  | WB19 amendment replaced WB11 lateral/drain surrogate authority with layer-aware Eq. [6.2.4]/[6.2.10]-[6.2.11] production-kernel authority using WB18/WB19 symbol aliases and legacy-ID typed guard continuity; WB13 daily output coupling authority remains active. Contract remains non-promotable while `GAP-SUBHYD-002` and `GAP-SUBHYD-003` are open (`GAP-SUBHYD-001` and `GAP-SUBHYD-004` are promotable-with-risk). |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-05-24` |  | SIMIMPL03 amendment added production runner-execution ownership, mode-propagation manifest closure, simulation-owned replay-surface provenance, and selective consolidated-intake governance authority (`INV-SYSTEM-018..021`) while preserving PL14S semantic evidence, WS11/WS12 routing posture, CLI02 replay-surface requirements, WB20 lane governance, ARCH22 typed-surface authority, and EROD12 closure context. |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-05-24` |  | SIMIMPL03 amendment added production execution ownership, effective-mode lane closure, simulation-owned WB13 provenance, and consolidated-intake guardrail authority (`INV-WATBAL-018..021`) with typed SIMPIPE/SIMMODE/SIMOUT/SIMCONS guards; PL14S semantic diagnostics, CLI02 interchange surfaces, and canonical 25-column WB13 schema posture remain active with companion-gap context (`GAP-WATBAL-002`). |

## Entry Order

Sort rows by `contract_id`.
