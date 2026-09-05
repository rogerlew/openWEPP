# Science Contract Registry

Status: Active
Last updated: 2026-09-04

This is the canonical lifecycle registry for openWEPP science contracts.

## Governance Pointers

Kernel-process contract governance is mandatory and integrated through:

1. `docs/specifications/science-contract-authoring-procedure.md`
2. `docs/specifications/science-contracts/kernel-process-contract-profile.md`
3. `docs/specifications/unit-governance.md`
4. `docs/specifications/correctness-authority-model.md`
5. `docs/specifications/external-authority/README.md`
6. `docs/specifications/external-authority/suite-schema.md`
7. `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
8. `docs/governance/openwepp-release-procedure-draft.md`
9. `.github/workflows/release-gates.yml`

For kernel-affecting changes, missing profile/procedure compliance keeps
disposition in `HOLD`.

ADR0017 registry note: ADR-0017 is active comparator-governance authority: comparator agreement is a flag, not a target; comparator/ledger contracts must fail closed on unit or lineage-stage ambiguity, support `HARNESS-SURFACE-MISMATCH`, and require independent correctness authority before any `OPENWEPP-DEFECTIVE` verdict. Active invariant pointers: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039`, `SC-WATBAL-001#INV-WATBAL-087`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-040`, `SC-WATBAL-001#INV-WATBAL-088`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-041`, `SC-WATBAL-001#INV-WATBAL-089`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-042`, `SC-WATBAL-001#INV-WATBAL-090`, `SC-CLIMATE-001#INV-CLIMATE-015`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-043`, `SC-WATBAL-001#INV-WATBAL-091`, `SC-CLIMATE-001#INV-CLIMATE-016`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-044`, `SC-WATBAL-001#INV-WATBAL-092`, `SC-CLIMATE-001#INV-CLIMATE-017`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-045`, `SC-WATBAL-001#INV-WATBAL-093`, and `SC-CLIMATE-001#INV-CLIMATE-018`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-046`, `SC-WATBAL-001#INV-WATBAL-094`.

Unit governance remains mandatory for science contracts, runtime boundary symbols, conversions, output metadata, and work-package gates; see `docs/specifications/unit-governance.md`.

## Registry Fields

| Field | Required | Description |
|---|---|---|
| `contract_id` | Yes | Stable ID matching `SC-<DOMAIN>-<NNN>`. |
| `title` | Yes | Human-readable contract title. |
| `status` | Yes | Lifecycle status (`open`, `in_review`, `approved`, `retired`, `withdrawn`). |
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
| `SC-BIOGEOCHEM-001` | Vegetation Biogeochemistry Exchange and Receiving-State Contract | `approved` | `active` | openWEPP maintainers + forest biogeochemistry reviewer | `docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md` | `static` | `2026-08-11` |  | Mineral-N arbitration and litter/CWD C/N/dry-material receiving boundary; transformations remain an explicit dependency. |
| `SC-CLIMATE-001` | Climate Forcing Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | `static` | `2026-06-03` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-COUPLEDTIME-001` | Coupled Time Support, Event, and Atomic Chronology Contract | `approved` | `active` | openWEPP maintainers + time/numerics + transaction/restart reviewers | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `static+independent_oracle+contract_vectors` | `2026-09-03` |  | v17 binds a private same-live-revision validated publication-support capability so trusted append performs only exact tail joins; restart, wire, archive, and untrusted reconstruction retain full independent validation. |
| `SC-EVAP-001` | Evapotranspiration Stress Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` | `static` | `2026-08-08` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-GWBASEFLOW-001` | Groundwater Reservoir Baseflow Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md` | `static` | `2026-07-08` |  | M-T2A linear groundwater-reservoir baseflow authority; implementation follows in M-T2B. |
| `SC-HYDRAULICS-001` | Overland Hydraulics Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | `static` | `2026-05-25` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-IMPOUND-001` | Surface Impoundment Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | `static` | `2026-05-28` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-IRRIG-001` | Irrigation Event Coupling Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md` | `static` | `2026-05-23` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-LANDSURFACEENERGY-001` | Land-Surface Energy-Balance Process Contract | `approved` | `active` | openWEPP maintainers + land-surface-energy/hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `static+independent_oracle+contract_vectors` | `2026-09-04` |  | v31 contract-first authority admits same-sweep component-temperature dependency replay under a topology-generic static graph, one shared canonical evaluator implementation and a source-real fallibility/crossability oracle while retaining complete non-component probes and bit-identical dense-solver behavior. |
| `SC-OFEROUTE-001` | Hillslope OFE-by-OFE Overland-Flow Routing Process Contract | `approved` | `active` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | `static` | `2026-09-01` |  | v52 requires accepted Stage 3 publication to traverse the real Lane-D OFE-day route exactly once with typed incomplete/mixed-authority rejection. |
| `SC-OFEROUTE-002` | Hybrid Implicit-Explicit Kinematic-Wave Stepping Contract | `withdrawn` | `deprecated` | openWEPP maintainers + hydrology reviewer | `abandoned/hybrid-implicit-stepping:docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` | `static` | `2026-07-07` |  | Withdrawn by ADR-0037; contract deleted from main with final working state archived on branch `abandoned/hybrid-implicit-stepping` at `b1d5fd4410b700012d857ef4056000163e6aa6a0`. |
| `SC-OUTPUT-WAT5-001` | Five-Minute Hillslope Water Diagnostic Output Contract | `approved` | `active` | openWEPP maintainers + hydrology/output reviewer | `docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md` | `static` | `2026-09-02` |  | v5 projects receipt-complete Stage-3 sources from the sealed SurfaceLiquid ingress ledger while retaining precipitation/non-rain lineage; v4 admits bounded source-supported closing-ledger placement and v3 retains exact typed segment replay. |
| `SC-PERC-001` | Percolation Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PERC-001.md` | `static` | `2026-06-02` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-PLANT-001` | Plant Growth Process Contract | `approved` | `active` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | `static` | `2026-08-08` |  | CP-GSI02 remains active; future vegetation compatibility requires atomic real-consumer cutover. |
| `SC-RESIDUE-001` | Residue Management Process Contract | `approved` | `active` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | `static` | `2026-08-08` |  | Current litter authority plus future exact-once vegetation dry-matter/C/N custody. |
| `SC-ROOTZONEHYDRAULICS-001` | Root-Zone Hydraulic Owner Contract | `approved` | `active` | openWEPP maintainers + soil/plant hydraulics reviewers | `docs/specifications/science-contracts/contracts/SC-ROOTZONEHYDRAULICS-001.md` | `primary-source constitutive authority + contract vectors` | `2026-08-19` |  | Required non-defaulted stratum path and live Brooks--Corey owner implemented in the default-off V10 shadow with terminal PASS. |
| `SC-ROUTE-001` | Watershed Routing and Channel Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | `static` | `2026-07-10` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-RUNOFFPART-001` | Surface Runoff Partition Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | `static` | `2026-09-01` |  | v49 binds accepted hourly Stage 3 excess as the Lane-D source without normalization or duplicate runoff partition. |
| `SC-SED-001` | Hillslope Erosion Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | `static` | `2026-08-09` |  | Revision 63 binds erosion to the source-complete maximum-hour depth-rate peak and the active absolute-seconds rectangular-duration custody check; amendment history lives in the canonical contract. |
| `SC-SNOWENERGY-001` | Snow-Surface Energy and Sub-Canopy Longwave Contract | `approved` | `active` | openWEPP maintainers + snow-process reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `static+independent_oracle+contract_vectors` | `2026-09-04` |  | Contract revision 61 retains the non-versioned ADR-0044 amendments: one native-identity represented-snow map and typed candidate-only unpublished-soil input with one complete final owner/restart promotion, all with no V58. It also binds one typed feed-forward real carrier call per invocation while preserving distinct step-doubling/retry/root/discovery/exact/batch/final paths; no process-solver V61 or physics/tolerance/adaptive/event/wire change. |
| `SC-SNOWFREEFORCING-001` | Snow-Free Half-Hour Forcing Provider Contract | `approved` | `active` | openWEPP maintainers + climate/radiation + vegetation/LSE reviewers | `docs/specifications/science-contracts/contracts/SC-SNOWFREEFORCING-001.md` | `static+independent_oracle` | `2026-08-18` |  | V1 contract-first authority; no production activation or cutover. |
| `SC-SNOWFREEZE-001` | Snow and Freeze Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `static` | `2026-08-27` |  | v140 owner amendment selects an exact 60-second adaptive Stage-3 floor with stable ordinary supports substantially larger, while preserving joint lanes, disappearance, exact-once terminal liquid, solid-precipitation reappearance, conservation/custody/receipt/rollback obligations, and additive operation-boundary restart; prior 600-ms floor evidence is superseded and awaits rerun; v137-v139 remain rejected historical candidates. |
| `SC-SOIL-001` | Soil State and Erodibility Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` | `static` | `2026-05-31` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-SUBHYD-001` | Subsurface Hydrology and Drainage Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | `static` | `2026-06-18` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-SURFACELIQUID-001` | Persistent Snow-Free Surface-Liquid Hydrology Custody Contract | `approved` | `active` | openWEPP maintainers + hydrology/land-surface-energy reviewer | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | `static+contract_vectors` | `2026-09-03` |  | v29 consumes the converged pending map's own surface-ingress/WB14 prefix into final custody once while retaining v28 inactive-prefix authority and forbidding final replay or promotion. |
| `SC-SYSTEM-001` | System Integration Boundary and Watershed Assembly Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | `static` | `2026-06-14` |  | Lifecycle-only row; amendment history lives in the canonical contract. |
| `SC-VEGETATION-001` | Native Vegetation State and Cross-Domain Boundary Contract | `approved` | `active` | openWEPP maintainers + forest ecohydrology/hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `static+independent_oracle+contract_vectors` | `2026-09-02` |  | v31 admits only private immutable revision-bound V10/V11 validation handoffs and retains fresh full validation after mutation and at restart/external/untrusted boundaries; v30 retains V9 generation-host/provider-equivalence, exact-60 support, and terminal-composition authority. |
| `SC-VEGETATIONTRANSACTION-001` | Coupled Vegetation Occupancy Owner-Transaction Contract | `approved` | `active` | openWEPP maintainers + vegetation/hydrology/energy reviewer | `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md` | `static+independent_oracle+contract_vectors` | `2026-09-03` |  | v18 constructs vegetation/material/BGC/joint/complete-owner custody once from the converged pending map's own physical prefix, with exclusive dispositions and no final replay or promotion. |
| `SC-WATBAL-001` | Water Balance Process Contract | `in_review` | `draft` | openWEPP maintainers + hydrology reviewer | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `static` | `2026-09-01` |  | v171 requires accepted Stage 3 hourly runoff depth, Lane-D volume/rate, and daily publication to close from one operand lineage. |

## Entry Order

Sort rows by `contract_id`.
