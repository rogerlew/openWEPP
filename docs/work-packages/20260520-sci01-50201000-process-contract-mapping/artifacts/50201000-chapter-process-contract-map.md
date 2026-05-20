# 50201000 Chapter-to-Process Contract Map

Status: draft-complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in SCI-01 mapping artifact (docs-only extraction/mapping)

## Purpose
Translate the WEPP technical-reference chapter corpus (`50201000`) into an
openWEPP contract-authoring spine that is process-based (not routine-name-based)
and aligned to ARCH-01 subsystem boundaries.

## Contract-Domain Naming Spine (Initial)

| Domain | Proposed contract seed | Primary ARCH-01 subsystem alignment | Notes |
|---|---|---|---|
| system integration | `SC-SYSTEM-001` | SS-03, SS-04, SS-08 | Component boundary and pass-through process semantics from model overview domains. |
| climate forcing | `SC-CLIMATE-001` | SS-03 | Weather-generation and forcing disaggregation authority. |
| snow/freeze winter hydrology | `SC-SNOWFREEZE-001` | SS-03, SS-06 | Hourly snowpack, melt, frost, thaw process authority. |
| surface runoff partition | `SC-RUNOFFPART-001` | SS-03, SS-06 | Infiltration/rainfall-excess/depression storage/runoff partition authority. |
| water balance | `SC-WATBAL-001` | SS-03, SS-06 | Daily closure and store/flux accounting authority. |
| evapotranspiration stress | `SC-EVAP-001` | SS-03, SS-06 | ET partition and stress-domain safety authority. |
| percolation | `SC-PERC-001` | SS-03, SS-06 | Percolation and below-root losses authority. |
| subsurface/drainage | `SC-SUBHYD-001` | SS-03, SS-06 | Lateral/subsurface flow and drain behavior authority. |
| soil state and erodibility | `SC-SOIL-001` | SS-02, SS-05, SS-06 | Soil-property state evolution and erodibility parameter authority. |
| plant growth | `SC-PLANT-001` | SS-03, SS-05 | Vegetation growth drivers and hydrology/erosion coupling authority. |
| residue management | `SC-RESIDUE-001` | SS-03, SS-05 | Residue partition/decomposition and management operation authority. |
| overland hydraulics | `SC-HYDRAULICS-001` | SS-03, SS-05 | Friction/shear/hydraulic coefficient process authority for erosion coupling. |
| hillslope erosion | `SC-SED-001` | SS-03, SS-05, SS-06 | Sediment continuity, detachment, transport/deposition authority. |
| irrigation | `SC-IRRIG-001` | SS-03, SS-05 | Sprinkler/furrow event coupling and scheduling authority. |
| watershed routing/channel | `SC-ROUTE-001` | SS-04, SS-07, SS-08 | Channel hydrology/erosion and watershed routing authority. |
| impoundment | `SC-IMPOUND-001` | SS-04, SS-05, SS-08 | Surface impoundment hydraulic/sedimentation authority. |

## Chapter-to-Contract Mapping

| Chapter PDF | Chapter process domain (from chapter heading/intro) | Proposed contract domain(s) | Seed invariant families to author first | Evidence |
|---|---|---|---|---|
| `chap1.pdf` | Model overview and hillslope/channel/impoundment decomposition | `SC-SYSTEM-001` | `INV-SYSTEM-001` explicit producer/consumer boundaries; `INV-SYSTEM-002` required state handoff completeness at component boundaries | `[INFERENCE][Static]` |
| `chap2.pdf` | Weather generator (daily occurrence, storm structure/intensity disaggregation) | `SC-CLIMATE-001` | `INV-CLIMATE-001` precipitation depth non-negative; `INV-CLIMATE-002` wet/dry transition probabilities bounded [0,1]; `INV-CLIMATE-003` disaggregated storm blocks conserve event depth | `[INFERENCE][Static]` |
| `chap3.pdf` | Winter hydrology: snow accumulation/melt + frost/thaw (hourly) | `SC-SNOWFREEZE-001` | `INV-SNOWFREEZE-001` melt cannot exceed available snowpack; `INV-SNOWFREEZE-002` snow water/depth non-negative; `INV-SNOWFREEZE-003` freeze/thaw state transitions are explicit (no silent branch) | `[INFERENCE][Static]` |
| `chap4.pdf` | Hillslope surface hydrology: GAML infiltration, rainfall excess, depression storage, peak runoff | `SC-RUNOFFPART-001` | `INV-RUNOFFPART-001` event water-partition closure holds for the declared accounting boundary (`inputs = outputs + storage_delta +/- residual_tolerance`); exact term table deferred to `SC-RUNOFFPART-001`; `INV-RUNOFFPART-002` runoff begins only after depression-storage satisfaction; `INV-RUNOFFPART-003` OFE aggregation preserves event water volume | `[INFERENCE][Static]` |
| `chap5.pdf` | Daily water balance/percolation/ET root-zone accounting | `SC-WATBAL-001`, `SC-EVAP-001`, `SC-PERC-001` | `INV-WATBAL-001` daily closure residual computed and bounded; `INV-WATBAL-002` storage states non-negative unless explicitly signed; `INV-EVAP-001` ET stress-domain arithmetic excludes invalid denominators; `INV-PERC-001` percolation losses bounded by available water | `[INFERENCE][Static]` |
| `chap6.pdf` | Subsurface hydrology and drainage routines | `SC-SUBHYD-001` | `INV-SUBHYD-001` subsurface/lateral/drain fluxes are non-negative when emitted as losses; `INV-SUBHYD-002` water-table transitions remain within classified domain bounds; `INV-SUBHYD-003` subsurface flow terms are closure-accounted in daily balance | `[INFERENCE][Static]` |
| `chap7.pdf` | Soil component state/erodibility/hydraulic parameters | `SC-SOIL-001` | `INV-SOIL-001` state variables (roughness, ridge height, bulk density, hydraulic conductivity) remain physically bounded; `INV-SOIL-002` erodibility/shear parameters remain in valid domains for downstream erosion math; `INV-SOIL-003` update ordering is explicit across tillage/weather/freeze drivers | `[INFERENCE][Static]` |
| `chap8.pdf` | Plant growth and management impacts on hydrology/erosion | `SC-PLANT-001` | `INV-PLANT-001` biomass/canopy/root state non-negative; `INV-PLANT-002` management operations cannot remove more biomass than available; `INV-PLANT-003` hydrology/erosion coupling surfaces are emitted with required units/state completeness | `[INFERENCE][Static]` |
| `chap9.pdf` | Residue decomposition and management (standing/flat/root/buried) | `SC-RESIDUE-001` | `INV-RESIDUE-001` residue mass partitions conserve total mass across operations; `INV-RESIDUE-002` residue pools non-negative; `INV-RESIDUE-003` decomposition/update sequencing is explicit and deterministic within timestep | `[INFERENCE][Static]` |
| `chap10.pdf` | Overland flow hydraulics, friction partitioning, shear partition implications | `SC-HYDRAULICS-001` | `INV-HYDRAULICS-001` friction coefficients non-negative and finite; `INV-HYDRAULICS-002` equivalent friction term construction is bounded and area-weight consistent; `INV-HYDRAULICS-003` active-soil shear fraction bounded [0,1] when represented fractionally | `[INFERENCE][Static]` |
| `chap11.pdf` | Hillslope erosion continuity, detachment/deposition, transport capacity | `SC-SED-001` | `INV-SED-001` sediment continuity residual computed and bounded; `INV-SED-002` rill/interrill delivery and detachment sign conventions are consistent; `INV-SED-003` transport-capacity-limited regimes are explicitly classified | `[INFERENCE][Static]` |
| `chap12.pdf` | Irrigation systems and rainfall-event coupling/scheduling | `SC-IRRIG-001` | `INV-IRRIG-001` irrigation additions are non-negative and scheduling-explicit; `INV-IRRIG-002` concurrent rainfall+irrigation hydrograph combination conserves added water; `INV-IRRIG-003` OFE targeting obeys scheduling-mode constraints | `[INFERENCE][Static]` |
| `chap13.pdf` | Watershed channel hydrology/erosion with hillslope pass-file coupling | `SC-ROUTE-001`, `SC-SYSTEM-001` | `INV-ROUTE-001` channel/watershed flux closure accounting is explicit; `INV-ROUTE-002` hillslope-to-channel handoff payload is complete/parseable; `INV-ROUTE-003` routing option selection and assumptions are explicit and branch-safe | `[INFERENCE][Static]` |
| `chap14.pdf` | Surface impoundment hydraulics and sedimentation | `SC-IMPOUND-001` | `INV-IMPOUND-001` stage-area/volume relationships monotonic and non-negative; `INV-IMPOUND-002` outflow/sediment outputs bounded by physically available storage/load; `INV-IMPOUND-003` daily update preserves mass accounting across inflow, storage, and outflow | `[INFERENCE][Static]` |

## Legacy Static Lineage Anchors (Secondary Authority)

These anchors are implementation-provenance signals only. Per ADR-0011, they
do not replace top-down contract authority from references/literature.

| Proposed contract domain | Representative legacy source anchors | Evidence |
|---|---|---|
| `SC-SYSTEM-001` | `wshpas.f90`, `wshdrv.f90`, `wshrun.f90`, `hbp_mode2_bridge.f90`, `hbp_legacy_bridge.f90` | `[DIRECT][Ran]` |
| `SC-CLIMATE-001` | `brkpt.for`, `disag.for` | `[DIRECT][Ran]` |
| `SC-SNOWFREEZE-001` | `winter.for`, `snowd.for`, `melt.for`, `frostn.for`, `watbal_hourly.for` | `[DIRECT][Ran]` |
| `SC-RUNOFFPART-001` | `watbal.for`, `watbal_hourly.for` | `[INFERENCE][Ran]` |
| `SC-WATBAL-001` | `watbal.for`, `watbal_hourly.for` | `[DIRECT][Ran]` |
| `SC-EVAP-001` | `evap.for`, `evappm.for`, `watbal.for`, `watbal_hourly.for` | `[DIRECT][Ran]` |
| `SC-PERC-001` | `perc.for`, `watbal.for`, `watbal_hourly.for` | `[DIRECT][Ran]` |
| `SC-SUBHYD-001` | `watbal.for`, `hydchn.for`, `route.for` | `[INFERENCE][Ran]` |
| `SC-SOIL-001` | `soil.for`, `tilage.for` | `[DIRECT][Ran]` |
| `SC-PLANT-001` | `grow.for`, `growop.for`, `range.for` | `[DIRECT][Ran]` |
| `SC-RESIDUE-001` | `decomp.for`, `res_dp.for`, `resup.for` | `[DIRECT][Ran]` |
| `SC-HYDRAULICS-001` | `hydout.for`, `hydchn.for`, `route.for`, `chnrt.for` | `[INFERENCE][Ran]` |
| `SC-SED-001` | `erod.for`, `chnero.for` | `[DIRECT][Ran]` |
| `SC-IRRIG-001` | `irrig.for` | `[DIRECT][Ran]` |
| `SC-ROUTE-001` | `route.for`, `chnrt.for`, `chnpar.for`, `chnvar.for`, `wshinp.for`, `wshout.for`, `wshred.for` | `[DIRECT][Ran]` |
| `SC-IMPOUND-001` | `wshimp.for`, `impday.for`, `impflo.for`, `impint.for`, `impmai.for`, `impmon.for`, `imppro.for`, `impyr.for` | `[DIRECT][Ran]` |

## Coupling and Authoring Order (Top-Down)

Recommended contract authoring order for first implementation slices:

Prerequisites before kernel-facing contract implementation:
- SS-01/SS-02 scaffolding is in place (input contract layer + typed state surface
  layer) per ARCH-01 follow-on sequencing.

1. `SC-CLIMATE-001` + Tier-A subset of `SC-SNOWFREEZE-001` (daily snow storage
   and melt-bound invariants)
2. `SC-RUNOFFPART-001` + `SC-WATBAL-001` + `SC-EVAP-001` + `SC-PERC-001`
3. `SC-SOIL-001` + `SC-PLANT-001` + `SC-RESIDUE-001` (state-driver contracts)
4. `SC-HYDRAULICS-001` + `SC-SED-001` (hillslope erosion physics layer)
5. `SC-IRRIG-001` (event-source extension)
6. `SC-ROUTE-001` + `SC-IMPOUND-001` + `SC-SYSTEM-001` (watershed assembly)

Rationale:
- Steps 1-2 establish Tier-A confidence surfaces (single OFE + daily water
  balance) first, while deferring hourly-heavy snow/freeze internals to
  investigation-first staging.
- Steps 4-6 add higher-complexity coupling where legacy comparator confidence is
  lower and investigation routing is more important.

## Tier-A / Tier-B Acceptance Alignment

- Tier-A emphasis for first executable contract slices:
  - `SC-RUNOFFPART-001`
  - `SC-WATBAL-001`
  - selected invariants from `SC-SNOWFREEZE-001`
- Tier-B-first investigation surfaces:
  - hourly-heavy snow/frost internals beyond daily closure
  - watershed/channel/impoundment complex routing states (`SC-ROUTE-001`,
    `SC-IMPOUND-001`)

This aligns with ADR-0011 comparator policy: legacy deltas are investigation
signals; only high-confidence surfaces are promotion-gating by default.

## Open Gaps

| Gap ID | Statement | Impact | Next action |
|---|---|---|---|
| GAP-SCI01-001 | Invariant families are mapped at domain level but not yet authored into full `SC-*.md` documents with lifecycle metadata and tolerance tables. | Contract IDs cannot yet be used as kernel acceptance gates. | Create `SCI-02` for contract authoring (`SC-CLIMATE-001` .. `SC-IMPOUND-001`) with one domain per phase checkpoint. |
| GAP-SCI01-002 | Chapter-level equation-anchor inventory is not yet captured in SCI-01 artifacts (beyond introductory chapter extraction). | Quantitative contract drafting readiness may be overstated until explicit anchor tables exist. | Add equation-anchor appendix during SCI-02 domain-contract authoring. |
| GAP-SCI01-003 | Legacy static code evidence linkage is domain-level but not yet per-invariant/per-equation. | Provenance traceability is partial for implementation-level acceptance checks. | Add per-invariant linkage annexes when each `SC-*` contract is authored. |
