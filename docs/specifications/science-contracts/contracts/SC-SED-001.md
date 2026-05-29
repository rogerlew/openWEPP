---
contract_id: SC-SED-001
title: Hillslope Erosion Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 38
producer_scope:
  - Hillslope sediment continuity, detachment/deposition, and transport-capacity surfaces
  - Event erosion boundary payloads consumed by routing/channel domains
  - Sediment size-class and enrichment surfaces at OFE and hillslope exits
consumer_scope:
  - Watershed/channel routing consumers requiring hillslope erosion payload semantics
  - Comparator and replay consumers using erosion closure and sign-consistency surfaces
  - Adjacent soil/runoff/hydraulics domains providing required coupling inputs
evidence_level: Static
last_reviewed: 2026-05-28
supersedes: []
superseded_by: []
---

# SC-SED-001 Hillslope Erosion Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for hillslope sediment continuity,
rill/interrill detachment-deposition behavior, transport-capacity constraints,
and hillslope erosion boundary payloads consumed by downstream routing domains.

## Scientific Scope

In scope:
- Hillslope sediment continuity and sign conventions for `Di`, `Df`, and `G`. `[DIRECT][Static]`
- Rill detachment and deposition branch semantics governed by hydraulic shear and transport capacity. `[DIRECT][Static]`
- Normalized erosion-equation parameter semantics used for profile/OFE routing. `[DIRECT][Static] + [INFERENCE][Static]`
- Sediment enrichment and particle-size-fraction update constraints at deposition transitions and hillslope/OFE exits. `[DIRECT][Static] + [INFERENCE][Static]`
- Required coupling surfaces from hydrology/hydraulics/soil domains into hillslope erosion and from hillslope erosion to channel routing domains. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Channel detachment/deposition and watershed outlet sediment routing internals owned by `SC-ROUTE-001`. `[DIRECT][Static] + [INFERENCE][Static]`
- Parameter-calibration campaigns and empirical re-fitting of Chapter-11 coefficients. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SED-CH11-INTRO | `references/50201000/chap11.pdf` §11.1 | Declares hillslope erosion governing-equation scope (sediment continuity, detachment, deposition, shear, transport capacity). | `[DIRECT][Static]` |
| REF-SED-CH11-CONT | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.1] | Sediment continuity equation and sign conventions for `Di` and `Df`. | `[DIRECT][Static]` |
| REF-SED-CH11-DET | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.2]-[11.2.3] | Rill detachment-capacity branch, threshold behavior (`τf` vs `τc`), and dependence on `Kr`. | `[DIRECT][Static]` |
| REF-SED-CH11-DEP | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.4] | Deposition branch when `G > Tc`, including `β`, `Vf`, and `q` scaling. | `[DIRECT][Static]` |
| REF-SED-CH11-HYDRO | `references/50201000/chap11.pdf` §11.2.2 Eq. [11.2.5]-[11.2.6] | Required hydrologic input variables (`Pr`, `tr`, `Ie`, `te`) and event-to-steady transposition definitions. | `[DIRECT][Static]` |
| REF-SED-CH11-SHEAR | `references/50201000/chap11.pdf` §11.2.3 Eq. [11.2.7] | Shear-stress formulation and active-soil shear partition (`fs/ft`). | `[DIRECT][Static]` |
| REF-SED-CH11-TC | `references/50201000/chap11.pdf` §11.2.4 Eq. [11.2.8]-[11.2.9] | Transport-capacity power-law relation and sandy-soil adjustment floor (`tcadjf >= 0.30`). | `[DIRECT][Static]` |
| REF-SED-CH11-NORM | `references/50201000/chap11.pdf` §11.3 Eq. [11.3.7]-[11.3.15] | Normalized detachment/deposition parameters (`η`, `τcn`, `θ`, `φ`) and solution/output conversion constraints. | `[DIRECT][Static]` |
| REF-SED-CH11-DOWNVAR | `references/50201000/chap11.pdf` §11.4 Eq. [11.4.1]-[11.4.6] | Downslope-variability/runon normalization semantics for OFE-strip routing. | `[DIRECT][Static]` |
| REF-SED-CH11-ENRICH | `references/50201000/chap11.pdf` §11.5 Eq. [11.5.1]-[11.5.6] | Size-class mass conservation and enrichment-ratio semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH10-HYDRAULICS | `references/50201000/chap10.pdf` §10.1-§10.2, Eq. [10.1.2], [10.2.1] | Hydraulic roughness/shear partition context for erosion-consumed shear surfaces (`fs`, `ft`). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH7-EROD | `references/50201000/chap7.pdf` §7.10-§7.11, Eq. [7.10.1]-[7.10.15], [7.11.1]-[7.11.18] | Adjusted interrill/rill erodibility and critical-shear parameters (`Kiadj`, `Kradj`, `τcadj`) consumed by Chapter-11 normalizations. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH4-RUNOFF | `references/50201000/chap4.pdf` §4.4.2-§4.4.4, Eq. [4.4.17]-[4.4.30] | Hydrology component authority for peak-runoff and effective-duration surfaces used by erosion equations. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH13-COUPLING | `references/50201000/chap13.pdf` §13.1 pass-file list | Downstream coupling semantics for hillslope erosion outputs (detachment, deposition, class concentrations, class fractions). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-HBP-FORMAT | `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`EVENT Payload`) | Canonical binary pass serialization field names and units for routing-boundary sediment payloads (`total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3[npart]`, `particle_diameter_m[npart]`, `particle_flow_fraction[npart]`). | `[DIRECT][Static]` |
| REF-SED-HBP-READER | `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Read Contract`, `Required Invariants`) | Watershed reader fail-closed semantics for missing/invalid hillslope payload fields and no-text-fallback posture. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-LEGACY-PARAM | `/workdir/wepp-forest_260430_baseline/src/param.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy normalized-parameter authority (`eata`, `tauc`, `theta`, `phi`) used for Wave-1 runtime parameter derivation provenance. | `[DIRECT][Static]` |
| REF-SED-LEGACY-EROD | `/workdir/wepp-forest_260430_baseline/src/erod.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy detachment-capacity and branch-condition authority used for Wave-1 detachment/deposition runtime branch ordering. | `[DIRECT][Static]` |
| REF-SED-LEGACY-RUNGE | `/workdir/wepp-forest_260430_baseline/src/runge.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy continuity evolution form (`dG/dx` update term as `dcap*((tcap-load)/tcap) + theta`) used for Wave-1 branch/continuity guard alignment. | `[DIRECT][Static]` |
| REF-SED-LEGACY-CONTIN-ROUTE | `/workdir/wepp-forest_260430_baseline/src/contin.for` + `/workdir/wepp-forest_260430_baseline/src/route.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy call-chain authority for hillslope sediment routing (`call route` from CONTIN) and per-segment upper-boundary detach/deposit routing control flow. | `[DIRECT][Static]` |
| REF-SED-LEGACY-XCRIT | `/workdir/wepp-forest_260430_baseline/src/xcrit.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy `mshear` case classification authority (`1..5`) used by hillslope segment routing branch dispatch. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPC | `/workdir/wepp-forest_260430_baseline/src/depc.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy deposition-equation partial-solution authority used at route segment upper boundaries and post-detachment deposition follow-up. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPEND | `/workdir/wepp-forest_260430_baseline/src/depend.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy authority for solving where deposition ends inside a segment (`xdend`) under increasing/decreasing flow cases. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPOS | `/workdir/wepp-forest_260430_baseline/src/depos.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy segment deposition profile update authority (`detach`, `tc`, `load`) in route deposition branches. | `[DIRECT][Static]` |
| REF-SED-LEGACY-ENRICH | `/workdir/wepp-forest_260430_baseline/src/enrich.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy particle-class enrichment authority for deposition transitions and OFE-end finalization (`iendfg` terminal call). | `[DIRECT][Static]` |
| REF-SED-LEGACY-RTPART | `/workdir/wepp-forest_260430_baseline/src/rtpart.for` + `/workdir/wepp-forest_260430_baseline/src/grow.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Provenance correction anchor: `rtpart.for` is plant root-mass partitioning (growth domain) and is not an erosion-routing companion routine. | `[DIRECT][Static]` |
| REF-SED-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative magnitudes for rates/loads where signed behavior is not explicitly defined; finite denominators and bounded fractions. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `x` | `m` | Downslope distance coordinate in sediment continuity equation. | hillslope geometry surface | erosion solver core |
| `G` | `kg s^-1 m^-1` | Sediment load per unit rill width. | erosion continuity solver | hillslope/channel sediment handoff |
| `Di` | `kg s^-1 m^-2` | Interrill sediment delivery to rills (non-negative). | interrill delivery pathway | sediment continuity solver |
| `Df` | `kg s^-1 m^-2` | Net rill erosion rate (`>0` detachment, `<0` deposition). | rill erosion pathway | sediment continuity solver |
| `Dc` | `kg s^-1 m^-2` | Rill detachment capacity term in Eq. [11.2.2]. | rill detachment branch | sediment continuity solver |
| `Tc` | `kg s^-1 m^-1` | Sediment transport capacity per unit rill width. | transport-capacity pathway | detachment/deposition branch selector |
| `Kr`, `τc` | `s m^-1`, `Pa` | Rill erodibility and critical-shear threshold in Eq. [11.2.3]. | soil/erodibility coupling pathway | rill detachment capacity computation |
| `τf` | `Pa` | Hydraulic shear stress acting on soil particles in rills. | hydraulics coupling pathway | branch threshold and transport-capacity computation |
| `β`, `Vf`, `q` | `fraction`, `m s^-1`, `m^2 s^-1` | Deposition branch parameters in Eq. [11.2.4]. | rainfall-type + hydraulics + sediment pathway | deposition rate computation |
| `Pr`, `tr`, `Vt` | `m s^-1`, `s`, `m` | Peak runoff, effective runoff duration, and event runoff depth (`tr = Vt/Pr`). | hydrology/runoff partition coupling | steady-state hydrologic input set |
| `Ie`, `te` | `m s^-1`, `s` | Effective rainfall intensity and effective duration for interrill delivery. | rainfall-disaggregation/hydrology coupling | interrill delivery computation |
| `Kiadj`, `Kradj`, `τcadj` | `kg s m^-4`, `s m^-1`, `Pa` | Adjusted soil erodibility/shear parameters from Chapter-7 consumed by Chapter-11 normalizations. | soil contract boundary payload | erosion normalization parameters |
| `σir`, `SDRRR`, `Fnozzle`, `Rs`, `w` | `m s^-1`, `fraction`, `fraction`, `m`, `m` | Interrill delivery and geometric/nozzle factors in Eq. [11.3.10]. | interrill/hydraulics/irrigation coupling | interrill parameter `θ` computation |
| `η`, `τcn`, `θ`, `φ` | `fraction`, `fraction`, `fraction`, `fraction` | Normalized erosion parameters controlling detachment/deposition equation forms. | normalization pathway | normalized erosion ODE solver |
| `total_detachment_kg`, `total_deposition_kg` | `kg` | Hillslope event detachment/deposition totals exported in watershed pass-file semantics. | hillslope erosion aggregator | channel/watershed routing consumers |
| `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i` | `kg m^-3`, `m`, `fraction` | Particle-class concentration, representative class diameter, and class fraction at hillslope/OFE exits. | size-class routing/enrichment pathway | channel/watershed routing consumers |
| `particle_class_count` | `count` | Particle-class cardinality for serialized routing-boundary payload arrays. | hillslope erosion aggregator | watershed routing payload validator |
| `ER` | `fraction` | Specific-surface-area enrichment ratio (`SSAsed/SSAsoil`). | enrichment pathway | sediment-quality interpretation consumers |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SED-001 | Sediment continuity invariant: `dG/dx = Df + Di` is enforced with declared units/sign semantics, including `Di >= 0` and signed `Df` branch interpretation. | hard-fail | REF-SED-CH11-CONT, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-002 | Rill-detachment branch invariant: when `τf > τc` and `G < Tc`, detachment follows Eq. [11.2.2]-[11.2.3] with explicit threshold handling (`Df = 0` when `τf <= τc`). | hard-fail | REF-SED-CH11-DET | `[DIRECT][Static]` |
| INV-SED-003 | Deposition branch invariant: when `G > Tc`, deposition follows Eq. [11.2.4], preserving denominator domain (`q > 0`) and signed deposition behavior (`Df < 0`). | hard-fail | REF-SED-CH11-DEP, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-004 | Hydrologic-input invariant: peak runoff, effective runoff duration, and effective rainfall intensity surfaces are present and consistent with Eq. [11.2.5]-[11.2.6] transposition semantics. | hard-fail | REF-SED-CH11-HYDRO, REF-SED-CH4-RUNOFF | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-005 | Shear-partition invariant: soil-active shear partition ratio (`fs/ft`) is explicit and bounded, and `τf` surfaces used by erosion routines are finite and non-negative. | hard-fail | REF-SED-CH11-SHEAR, REF-SED-CH10-HYDRAULICS, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-006 | Transport-capacity invariant: transport capacity follows Eq. [11.2.8], and sandy adjustment in Eq. [11.2.9] preserves `tcadjf >= 0.30` when applicable. | hard-fail | REF-SED-CH11-TC | `[DIRECT][Static]` |
| INV-SED-007 | Normalization invariant: normalized parameters (`η`, `τcn`, `θ`, `φ`) are derived by Eq. [11.3.7]-[11.3.11] with finite denominators and consistent adjusted soil parameters from Chapter 7. | hard-fail | REF-SED-CH11-NORM, REF-SED-CH7-EROD, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-008 | Downslope-variability invariant: OFE-strip routing with runon uses Eq. [11.4.1]-[11.4.6] branch semantics and preserves explicit sediment/water boundary conditions between strips. | hard-fail | REF-SED-CH11-DOWNVAR | `[DIRECT][Static]` |
| INV-SED-009 | Enrichment mass-conservation invariant: class-wise outgoing sediment from deposition transitions cannot exceed incoming-plus-local-contribution mass after correction steps. | hard-fail | REF-SED-CH11-ENRICH, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-010 | Coupling payload invariant: hillslope erosion event outputs (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) are emitted with units/sign conventions required by watershed/channel consumers. | hard-fail | REF-SED-CH13-COUPLING, REF-SED-HBP-FORMAT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-011 | Governance-scope invariant: Chapter-11 simplifications (steady-state transposition, fitted transport-capacity adjustments, enrichment procedure caveats) must remain explicit; unlabeled scope over-claims block promotion. | governance-fail | REF-SED-CH11-INTRO, REF-SED-CH11-TC, REF-SED-CH11-ENRICH | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SED-001` | runtime | Sediment continuity assembler | Typed hard error on unit/sign/closure inconsistency | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-002` | runtime | Rill detachment branch evaluator | Typed hard error on threshold/branch violation | Tier-A gate | `[DIRECT][Static]` |
| `INV-SED-003` | runtime | Deposition branch evaluator | Typed hard error on invalid `q` domain or sign inversion | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-004` | runtime | Hydrologic input validator | Typed hard error on missing/inconsistent (`Pr`, `tr`, `Ie`, `te`) payload surfaces | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-005` | runtime | Shear partition/domain validator | Typed hard error on invalid partition ratio or non-finite `τf` surfaces | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-006` | runtime | Transport-capacity calculator | Typed hard error on invalid transport-capacity domain or sandy-adjustment floor violation | Tier-A gate | `[DIRECT][Static]` |
| `INV-SED-007` | runtime | Normalization parameter calculator | Typed hard error on invalid normalized-parameter derivation or denominator domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-008` | runtime | OFE-strip routing branch logic | Typed hard error on runon/branch-condition mismatch | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-SED-009` | runtime | Particle-class enrichment updater | Typed hard error on class-wise mass-conservation breach | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-010` | runtime | Hillslope-to-routing boundary payload validator | Typed hard error on missing/malformed sediment payload fields | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-011` | governance | Review/disposition/verification checklist | Promotion `HOLD` on unlabeled scope-limit or over-claim conditions | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-11 WEPP notation. EROD11
ratifies Wave-0 erosion-lane boundary alias ownership for the required
cross-contract coupling surfaces while preserving canonical identity aliases
for not-yet-implemented erosion internals.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `G`, `Di`, `Df`, `Dc`, `Tc` | identity names | continuity and branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Kr`, `τc`, `τf`, `β`, `Vf`, `q` | identity names | detachment/deposition threshold and rate surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Pr`, `tr`, `Vt`, `Ie`, `te` | identity names | hydrologic erosion-input surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Q` (WB12 runoff coupling) | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q` | runoff-depth coupling surface consumed by erosion forcing | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `peakro`, `watdur` | `HillslopeProductionStateSymbol::Wb16Peakro -> peakro`; `HillslopeProductionStateSymbol::Wb16Watdur -> watdur` | peak-runoff and runoff-duration forcing surfaces for erosion branches | `m^3 s^-1`, `s` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionStateSymbol::{Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | WB16 branch-traceability surfaces required by erosion observability guards | branch metadata + scalar continuity diagnostics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Kiadj`, `Kradj`, `τcadj` | identity names | soil-to-erosion coupling surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `σir`, `SDRRR`, `Fnozzle`, `Rs`, `w` | identity names | interrill and geometry adjustment surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `η`, `τcn`, `θ`, `φ` | identity names | normalized erosion parameter surfaces | dimensionless semantics preserved | `[DIRECT][Static]` |
| `total_detachment_kg`, `total_deposition_kg` | identity names | hillslope sediment totals to routing pass-file boundary | `kg` preserved | `[DIRECT][Static]` |
| `particle_class_count` | identity name | particle-class cardinality for serialized boundary vectors | count semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `sediment_concentration_kg_m3_i` | `sediment_concentration_kg_m3_{class:04}` | sediment class concentration boundary surfaces for pass serialization and routing intake | `kg m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `particle_diameter_m_i` | `particle_diameter_m_{class:04}` | sediment class particle-diameter boundary surfaces for pass serialization and routing intake | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `particle_flow_fraction_i` | `particle_flow_fraction_{class:04}` | sediment class flow-fraction boundary surfaces for pass serialization and routing intake | fraction semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ER` | identity name | enrichment-ratio boundary surface | fraction semantics preserved | `[DIRECT][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionFluxSymbol::Wb12RunoffQ`; `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur,Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB12/WB16 kernels | `SC-SED-001` (`INV-SED-004`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-002` | `fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-HYDRAULICS-001` | `SC-SED-001` (`INV-SED-005`, `INV-SED-006`, `INV-SED-007`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-003` | `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i` | `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_{class:04}`, `particle_diameter_m_{class:04}`, `particle_flow_fraction_{class:04}` (hillslope export), plus contributor-prefixed routing aliases `hs{ID}_*` | `SC-SED-001` | `SC-ROUTE-001` (`INV-ROUTE-011`) | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hydrology forcing intake (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` (`INV-RUNOFFPART-009`, `INV-RUNOFFPART-011`) + `SC-WATBAL-001` (`INV-WATBAL-007`, `INV-WATBAL-016`) | `SC-SED-001` (`INV-SED-004`) | Canonical producer/consumer guard ownership is explicit for required Wave-0 forcing symbols. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydraulics shear/friction intake (`fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe`) | `SC-HYDRAULICS-001` (`INV-HYDRAULICS-009`..`011`) | `SC-SED-001` (`INV-SED-005`..`007`) | Guard ownership and failure posture are explicit with no remaining Wave-0 ownership ambiguity. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment payload export to routing (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) | `SC-SED-001` (`INV-SED-010`) | `SC-ROUTE-001` (`INV-ROUTE-011`) | Cross-domain payload validation ownership is explicit for downstream routing intake. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| Shear-below-threshold state | `τf <= τc` with `Df = 0` and no rill detachment contribution. | Explicit threshold behavior in Eq. [11.2.3] narrative. | `[DIRECT][Static]` |
| Transport-at-capacity state | `G = Tc` giving zero net rill source/sink contribution from Eq. [11.2.2]/[11.2.4] branch terms. | Continuity-consistent branch boundary between detachment and deposition regimes. | `[DIRECT][Static] + [INFERENCE][Static]` |
| No-runoff erosion-inactive event | `Pr = 0` and/or `tr = 0` resulting in no active rill transport update for the event. | Hydrologic forcing can yield erosion-inactive events while preserving daily model continuity. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rainfall-type switch in deposition coefficient | `β = 0.5` for raindrop-impact rill flow and `β = 1.0` for snowmelt/furrow-irrigation cases. | Explicit Chapter-11 deposition coefficient mode rule. | `[DIRECT][Static]` |
| Zero-sprinkler-impact day | `Fnozzle = 1.0` on non-sprinkler rainfall days by default. | Explicit default behavior for nozzle-impact factor in §11.3.3 discussion. | `[DIRECT][Static]` |

## Invalid States

- Missing required hydrologic erosion-driver surfaces (`Pr`, `tr`, `Ie`, `te`) when erosion computations are invoked. `[DIRECT][Static] + [INFERENCE][Static]`
- Non-finite or negative transport/shear/flow-domain terms where not physically permitted (`Tc`, `q`, `τf`, `Vf`, `Kr`). `[DIRECT][Static] + [INFERENCE][Static]`
- Rill-detachment branch executed when threshold conditions are not met (`τf <= τc`) without explicit zero-detachment handling. `[DIRECT][Static] + [INFERENCE][Static]`
- Deposition branch executed with zero/negative `q` denominator semantics in Eq. [11.2.4]. `[DIRECT][Static] + [INFERENCE][Static]`
- Particle-class enrichment update that violates class-wise mass conservation across deposition transition updates. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing or unit-inconsistent hillslope-to-routing sediment payload fields (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`). `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SED-P-001: Publish hillslope erosion continuity and branch surfaces using canonical Chapter-11 symbol semantics and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-002: Enforce explicit detachment/deposition branch predicates and threshold behavior before emitting event outputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-003: Propagate invariant violations as typed errors; do not silently clamp or default materially invalid erosion states. `[INFERENCE][Static]`
- OBL-SED-P-004: Emit routing-boundary sediment payload completeness (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) with unit/sign integrity. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SED-C-001: Hydraulics and runoff consumers supplying erosion drivers must preserve unit/sign semantics for shear/runoff/intensity fields consumed by Chapter-11 equations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-C-002: Routing/channel consumers must reject malformed hillslope sediment payloads and propagate typed failures with invariant context. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-C-003: Soil/management consumers must preserve adjusted erodibility/critical-shear payload semantics (`Kiadj`, `Kradj`, `τcadj`) consumed by erosion normalization. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Continuity and branch-threshold semantics (`INV-SED-001/002/003`) | erosion branch assembly | Hard error on branch/sign/closure failure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydrologic/shear/transport input semantics (`INV-SED-004/005/006`) | pre-solve boundary validation | Hard error on malformed domains or missing coupling inputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Normalization + downslope routing semantics (`INV-SED-007/008`) | normalized solver + OFE strip transitions | Hard error on invalid normalized states; investigation routing for complex OFE strip mismatches | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Enrichment and routing payload semantics (`INV-SED-009/010`) | class-fraction updater + export boundary | Hard error on class-mass inconsistency or payload incompleteness | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scope and authority governance (`INV-SED-011`) | review/verification/promotion | Governance `HOLD` until limitations and caveats are explicitly carried forward | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity).

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-SED-001 | Continuity residual tolerance for `INV-SED-001` event accounting surfaces | `<= 1e-9` in `kg s^-1 m^-1`-equivalent residual space | Residual reporting is mandatory; tolerance is for comparator interpretation, not silent runtime clamping. | `[INFERENCE][Static]` |
| TOL-SED-002 | Non-negative-domain tolerance for transport/load magnitudes (`G`, `Tc`, `Di`) | lower bound `>= -1e-12` in declared units | Material negatives beyond tolerance are hard-fail invariant violations. | `[INFERENCE][Static]` |
| TOL-SED-003 | Shear-threshold comparator tolerance for `τf` vs `τc` branch boundary | `abs(τf - τc) <= 1e-9 Pa` treated as threshold boundary condition | Prevents numeric jitter from toggling branch semantics near threshold. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SED-004 | Deposition denominator floor tolerance for `q` | `q >= 1e-12 m^2 s^-1` for deposition-branch evaluation | Values below floor are invalid for Eq. [11.2.4] denominator semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SED-005 | Class-fraction closure tolerance in enrichment updates | `abs(sum(sed_frac_i) - 1.0) <= 1e-9` | Ensures exported sediment class fractions remain normalized. | `[DIRECT][Static] + [INFERENCE][Static]` |

## WB16 Hydrologic Peak/Duration Intake Addendum

### WB16 Required Hydrology Inputs

| Surface | Symbols |
|---|---|
| Peak-flow forcing inputs | `peakro`, `watdur`, `Q` |
| WB16 diagnostics payload | `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

### WB16 Coupling Rules

1. Sediment-kernel readiness requires finite/non-negative `peakro` and
   `watdur` for erosion forcing derived from Chapter-11 hydrologic inputs.
2. WB16 continuity constraint remains explicit:
   - `watdur = Q/peakro` (within tolerance).
3. Missing or malformed WB16 trace payload symbols are typed boundary failures
   for erosion observability/replay diagnostics.
4. Sediment consumers must not synthesize fallback peak-flow inputs when WB16
   peak/duration symbols are absent or invalid.

### WB16 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB16-PEAK-E-001` |
| Non-finite required symbol | `HKERNEL-WB16-PEAK-E-002` |
| Domain/closure violation | `HKERNEL-WB16-PEAK-E-003` |

## EROD13 Wave-1 Core Runtime Addendum

### Runtime Integration Point

1. Wave-1 core erosion runtime executes in the hillslope
   `closure_diagnostics` scheduler phase after WB16 peak-runoff state writeback
   completes. The dispatched kernel phase class remains
   `hydrology_peak_runoff`; EROD13 authority is enforced by explicit guard-code
   family and symbol set below.
2. Runtime activation is explicit:
   - `erod13_core_enabled = 1` enables Wave-1 erosion execution.
   - `erod13_core_enabled = 0` disables the erosion path for fixtures that do
     not yet seed Wave-1 sediment inputs.
3. When enabled, missing/non-finite/domain-invalid erosion inputs are typed
   hard failures; no fallback synthesis, silent defaults, or silent clamping
   is permitted. `[DIRECT][Static] + [INFERENCE][Static]`

### Wave-1 Runtime Symbols

| Surface family | Symbols | Runtime role |
|---|---|---|
| Activation + hydrology forcing | `erod13_core_enabled`, `Q`, `peakro`, `watdur`, `Ie`, `te` | Activation gate and Chapter-11 hydrologic forcing semantics (`INV-SED-004`). |
| Hydraulics/shear forcing | `fs`, `ft`, `taufe`, `q` | Shear-partition and deposition-denominator semantics (`INV-SED-003`, `INV-SED-005`). |
| Sediment branch forcing | `G`, `Di`, `beta`, `vf`, `dGdx` | Continuity + branch-state forcing for detachment/deposition evaluation (`INV-SED-001`..`003`). |
| Normalization forcing | `cntlen`, `kr`, `kradjf`, `tcadjf`, `shrsol`, `tcend`, `shcrit`, `detinr`, `effdrr`, `effdrn`, `veleff`, `pkro` | Legacy-authority normalized parameter derivation (`INV-SED-007`). |
| Transport-capacity forcing | `erod13_tc_k`, `erod13_tc_m` | Eq. [11.2.8]-style power-law transport-capacity coefficients (`INV-SED-006`). |
| Core outputs | `Dc`, `Tc`, `Df`, `eta`, `taucn`, `theta`, `phi` | Wave-1 core runtime outputs required for invariant checks and downstream Wave-2 entry. |

### Wave-1 Algorithm Specification

1. Validate activation + required forcing symbols:
   - when `erod13_core_enabled = 1`, all required symbols in the table above
     are mandatory and finite.
2. Validate hydrology-forcing continuity (`INV-SED-004`):
   - `Q > 0`, `peakro > 0`, `watdur > 0`, `Ie >= 0`, `te > 0`,
   - `abs(watdur - (Q/peakro)) <= TOL-SED-001`.
3. Compute shear partition (`INV-SED-005`):
   - require `ft > 0`, `0 <= fs <= ft`,
   - compute `tau_f = taufe * (fs/ft)`.
4. Compute normalized parameters (`INV-SED-007`) using pinned legacy `param.for`
   lineage:
   - `eta = cntlen * kr * kradjf * shrsol / tcend`,
   - `taucn = tcadjf * shcrit / shrsol`,
   - `theta = (cntlen * detinr / tcend) * (effdrr/effdrn)`,
   - `phi = beta * veleff / pkro`.
5. Compute transport capacity (`INV-SED-006`):
   - require `tcadjf >= 0.30`,
   - `Tc = tcadjf * erod13_tc_k * tau_f^(erod13_tc_m)`.
6. Compute detachment/deposition branch (`INV-SED-002`, `INV-SED-003`):
   - detachment branch (`tau_f > taucn` and `G < Tc`):
     `Dc = eta * (tau_f - taucn)`,
     `Df = Dc * ((Tc - G)/Tc)`.
   - deposition branch (`G > Tc`):
     require `q > 0`,
     `Df = -(beta * vf / q) * (G - Tc)`.
   - threshold/equilibrium branch:
     `Df = 0`.
7. Enforce continuity (`INV-SED-001`):
   - `abs(dGdx - (Df + Di)) <= TOL-SED-001`.

### EROD13 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol (Wave-1 enabled path) | `HKERNEL-EROD13-CORE-E-001` |
| Non-finite required symbol | `HKERNEL-EROD13-CORE-E-002` |
| Domain/closure violation | `HKERNEL-EROD13-CORE-E-003` |

### Wave-1 Contract-Derived Test Vectors

Minimum vectors required by EROD13 contract-derived tests:

1. Nominal detachment vector (`tau_f > taucn`, `G < Tc`) emits finite
   non-negative `Dc`, positive `Df`, and continuity residual within tolerance.
2. Threshold vector (`tau_f <= taucn`) emits `Df = 0`.
3. Deposition vector (`G > Tc`) emits negative `Df` with valid `q > 0`.
4. Domain guard vector: `tcadjf < 0.30` fails with
   `HKERNEL-EROD13-CORE-E-003`.
5. Missing-symbol vector fails with `HKERNEL-EROD13-CORE-E-001`.
6. Non-finite-symbol vector fails with `HKERNEL-EROD13-CORE-E-002`.
7. Continuity residual violation (`dGdx != Df + Di`) fails with
   `HKERNEL-EROD13-CORE-E-003`.

## EROD14 Wave-2 Multi-OFE and Enrichment Runtime Addendum

### Runtime Integration Point

1. Wave-2 executes in the hillslope `closure_diagnostics` scheduler phase
   after WB16 peak/runoff closure and after EROD13 Wave-1 core calculations
   when both lanes are enabled.
2. Runtime activation is explicit:
   - `erod14_wave2_enabled = 1` enables Wave-2 multi-OFE/enrichment logic.
   - `erod14_wave2_enabled = 0` disables the Wave-2 lane.
3. Production hillslope runner activation policy is explicit:
   - if `erod14_wave2_enabled` is explicitly supplied on the runtime surface,
     it must be finite and binary (`0|1`);
   - if it is absent, runner-owned intake projection must set it from aligned
     OFE topology authority: `1` when validated `nelem > 1`, else `0`.
4. Runner-owned Wave-2 ingress seeding policy is explicit for enabled paths:
   - required symbols must be present before `closure_diagnostics` execution,
   - symbol synthesis must be deterministic from parsed/runtime surfaces plus
     canonical seed families,
   - missing/non-finite/domain-invalid derivation inputs are typed hard-fail
     states (no silent fallback).
5. When enabled, missing/non-finite/domain-invalid Wave-2 symbols are typed
   hard failures; fallback synthesis, silent defaults, and silent domain
   masking are prohibited.

### Wave-2 Runtime Symbols

| Surface family | Symbols | Runtime role |
|---|---|---|
| Activation and case semantics | `erod14_wave2_enabled`, `erod14_case`, `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`, `erod14_Fp` | Multi-OFE branch-classification and runon/runoff condition guards (`INV-SED-008`). |
| Geometry and load-transition surfaces | `erod14_xtop`, `erod14_xbot`, `erod14_xdetst`, `erod14_ldtop`, `erod14_ldbot`, `erod14_lddend`, `erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen` | Deposition-strip transition boundaries and normalized load scaling (`INV-SED-008`, `INV-SED-009`). |
| Coefficients and class families | `erod14_class_count`, `erod14_ktrato`, `erod14_ainftc`, `erod14_binftc`, `erod14_cinftc`, `erod14_beta`, `theta`, `erod14_fall_*`, `erod14_frcflw_*`, `erod14_frac_*`, `erod14_fidel_*`, `erod14_tcf1_*` | Class-wise enrichment/deposition update surfaces with pinned legacy-form branch semantics (`INV-SED-009`). |
| Class outputs and enrichment exports | `erod14_gend_*`, `erod14_sedmax_*`, `sed_frac_*`, `erod14_sumg`, `ER`, `erod14_ssa_soil`, `erod14_ssa_class_*` | Class-mass closure and enrichment ratio export surfaces. |

### Wave-2 Algorithm Specification

1. Validate multi-OFE case classification (`erod14_case`) using explicit
   case semantics:
   - Case 1: `Qj-1 = 0`, `Vj = 0`, `Qj = 0`
   - Case 2: `Qj-1 > 0`, `Vj > 0`, `Qj > 0`
   - Case 3: `Qj-1 > 0`, `Vj = 0`, `Fh - Fp > 0`, `Qj > 0`
   - Case 4: `Qj-1 > 0`, `Vj = 0`, `Fh - Fp <= 0`, `Qj = 0`
2. Compute class-wise deposition-transition predictions (`gend_i`) from
   legacy `enrich.for` lineage using `pkro`, per-class `phi`, and
   (`xtop`,`xbot`,`qostar`) ratio terms.
3. Scale class-wise loads to `ldbot`, enforce non-negative floors, and enforce
   class-wise maximum mass constraints:
   - `sedmax_i = gu_i + ftheta_i*(xbot-xtop)`,
   - iterative reproportioning for classes below `sedmax_i` until closure.
   - baseline `enrich.for` semantics are authoritative for the
     reproportion loop: when at least one class is clipped to `sedmax_i` and no
     class remains below `sedmax_i` (`ratbot = 0`), do not fail solely on
     `ratbot = 0`; re-enter the clipping pass and accept all-class `sedmax_i`
     saturation once no further clipping is required.
4. Enforce `INV-SED-009` class-mass conservation:
   - `gend_i <= sedmax_i` for all classes at convergence,
   - `sum(gend_i)` must remain finite and non-negative,
   - emitted `sed_frac_i` values are normalized from final class loads.
5. Compute enrichment ratio export:
   - `ER = (sum_i(sed_frac_i * ssa_class_i) / ssa_soil) + 0.005`.

### EROD14 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol (Wave-2 enabled path) | `HKERNEL-EROD14-WAVE2-E-001` |
| Non-finite required symbol | `HKERNEL-EROD14-WAVE2-E-002` |
| Domain/closure violation | `HKERNEL-EROD14-WAVE2-E-003` |

### Wave-2 Contract-Derived Test Vectors

Minimum vectors required by EROD14 contract-derived tests:

1. Nominal multi-OFE vector emits finite `erod14_gend_*`, normalized
   `sed_frac_*`, and finite `ER`.
2. Case-four branch vector (`Fh - Fp <= 0`) retains explicit case-four closure
   with zero downstream runoff indicator (`Qj = 0`).
3. Missing-symbol vector fails with `HKERNEL-EROD14-WAVE2-E-001`.
4. Non-finite-symbol vector fails with `HKERNEL-EROD14-WAVE2-E-002`.
5. Case-classification mismatch vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.
6. Class-conservation violation vector (no feasible reproportion closure)
   re-enters clipping when `ratbot = 0` under all-class `sedmax_i` saturation
   and completes without typed domain failure.
7. Class-fraction normalization violation vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.

## EROD15 Wave-3 HBP Routing-Boundary Export Addendum

### Wave-3 Export Symbols and Alias Family

Wave-3 routing-boundary payload export is anchored to pass-serialization
authority in `REF-SED-HBP-FORMAT` and carries the following symbol family:

- hillslope export symbols:
  - `total_detachment_kg`
  - `total_deposition_kg`
  - `particle_class_count`
  - `sediment_concentration_kg_m3_{class:04}`
  - `particle_flow_fraction_{class:04}`
- watershed contributor aliases (routing intake family):
  - `hs{ID}_total_detachment_kg`
  - `hs{ID}_total_deposition_kg`
  - `hs{ID}_particle_class_count`
  - `hs{ID}_sediment_concentration_kg_m3_{class:04}`
  - `hs{ID}_particle_flow_fraction_{class:04}`

### Wave-3 Export Projection Rules

When `erod14_wave2_enabled = 1`, Wave-3 export projection is mandatory and
uses the following runtime mapping:

1. `total_detachment_kg = max(erod14_sumg, 0)`.
2. `total_deposition_kg = max(erod14_lddend, 0)`.
3. `particle_flow_fraction_{class:04} = sed_frac_{class:04}`.
4. `particle_class_count = erod14_class_count`.
5. `sediment_concentration_kg_m3_{class:04}` must be finite and
   non-negative:
   - when `erod14_qout > 0`, compute as `erod14_gend_{class:04} / erod14_qout`;
   - when `erod14_qout <= 0`, emit `0`.

No fallback synthesis from non-Wave-2 symbols is allowed.

### Wave-3 Guard Continuity

Wave-3 payload export preserves existing EROD14 guard-family continuity:

- missing required symbol: `HKERNEL-EROD14-WAVE2-E-001`
- non-finite required symbol: `HKERNEL-EROD14-WAVE2-E-002`
- domain/closure violation: `HKERNEL-EROD14-WAVE2-E-003`

Missing/non-finite/domain-invalid payload surfaces are hard-fail states under
`INV-SED-010`; silent defaults, silent clamping, and consumer-side repair are
prohibited.

### Wave-3 Contract-Derived Test Vectors

Minimum vectors required by EROD15 contract-derived tests:

1. Nominal Wave-3 vector emits finite/non-negative
   `total_detachment_kg`/`total_deposition_kg`, finite
   `sediment_concentration_kg_m3_{class:04}`, and normalized
   `particle_flow_fraction_{class:04}`.
2. Zero-outflow vector (`erod14_qout <= 0`) emits zero concentration and zero
   particle-flow-fraction payloads for all classes.
3. Missing payload symbol vector fails with `HKERNEL-EROD14-WAVE2-E-001`.
4. Non-finite payload symbol vector fails with `HKERNEL-EROD14-WAVE2-E-002`.
5. Domain-invalid payload symbol vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.

## EROD16 Hillslope ROUTE Branch Authority Addendum

### Authoritative Routine Chain and Ownership

1. Baseline hillslope sediment routing authority is the `CONTIN -> ROUTE`
   call chain (`REF-SED-LEGACY-CONTIN-ROUTE`), not watershed WS10 channel
   routing branches.
2. `route.for` branch companions are:
   - `xcrit.for` (`mshear` classification),
   - `depc.for` and `depend.for` (deposition start/end solution terms),
   - `depos.for` (segment deposition profile updates),
   - `erod.for` (detachment-branch integration),
   - `enrich.for` (class-fraction updates and OFE-end finalization).
3. `rtpart.for` is explicitly out of routing scope (`REF-SED-LEGACY-RTPART`);
   it belongs to plant-growth/root partitioning lineage and must not be cited
   as a sediment-routing companion.

### ROUTE Branch Invariants (Canonical)

1. Segment-loop invariant: route branch execution iterates segmentwise over
   `k = 2..nslpts(iplane)` with explicit case-4 flow-end guards before
   branch dispatch.
2. Upper-boundary deposition invariant: when upper-segment `du < 0`,
   routing must execute `depc -> depend -> depos` semantics before any
   detachment-after-deposition branch calls.
3. `mshear` dispatch invariant: route branch selection is explicit for cases
   `1..5` from `xcrit`, with no branch collapsing or silent fallback between
   case families.
4. Post-detachment deposition invariant: when `ndep != 0`, deposition follow-up
   from `xdbeg` to segment end must execute before advancing to the next
   segment.
5. OFE-end enrichment invariant: terminal `enrich(..., iendfg=1)` semantics are
   required at OFE end even when no additional deposition region is opened in
   the final segment.

### Alias Continuity Requirements

Canonical symbol continuity for route migration in this contract must preserve:

- segment geometry/state: `xu`, `xl`, `nslpts`, `xdbeg`, `xdend`, `xdetst`,
  `ldlast`, `lddend`
- branch controls: `du`, `dl`, `mshear`, `ndep`, `xc1`, `xc2`
- deposition/transport controls: `ktrato`, `qostar`, `ainftc`, `binftc`,
  `cinftc`, `phi`, `theta`

If runtime symbol names differ, explicit alias mappings are required in
canonical `SC-*` contracts before production migration packages.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SED-001 | Per-invariant comparator vectors for sediment-branch transitions and class-wise enrichment closures remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where those vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-SED-002 | Wave-0 erosion-lane alias-ownership ambiguity for required cross-contract boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-003 | EROD12 ratifies cross-domain ownership/guard closure for required erosion-lane companion boundaries (`SC-HYDRAULICS-001`, `SC-ROUTE-001`) using canonical `SC-*` addenda and row-scoped guard ownership mapping. | Required Wave-0 cross-domain ownership semantics are canonicalized; erosion production implementation remains separately gated by `EROD13+` and non-Wave-0 companion gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-004 | Chapter-11 enrichment caveats for mixed-soil, multi-OFE composition effects remain and are explicitly retained as a documented limitation with governance risk acceptance. | Mixed-soil enrichment interpretation may still require manual investigation; this is accepted as an explicit model-governance caveat. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SED-005 | Baseline `route.for` segment-level branch family (`mshear 1..5`, upper-end deposition/detachment trees, post-detachment deposition closure) was migrated into openWEPP runtime kernels by EROD19 and revalidated by EROD21 parity rerun/hold-lift disposition. | Hillslope sediment-routing process parity closure is now recorded; ongoing comparator monitoring continues under EROD21 evidence artifacts. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-006 | WSHEDIMPL38 closed the residual companion watershed channel sediment seam for `chnero/chnrt/detach` integration by retiring unresolved-detachment diagnostics publication (`ws20_detachment_unmigrated_segment_count`, `ws21_detach_unmigrated_segment_count`) and replacing residual invalid-segment fallback continuation in WS20/WS21 process lanes with typed fail-closed guard behavior (`ws20_case12_next_flux_{class:04}`, `ws21_case3_next_flux_{class:04}`, `ws21_case4_next_flux_{class:04}`). | Companion watershed sediment integration now relies on explicit typed guard failure for domain violations and no longer carries unresolved-detachment surrogate counters in production publication surfaces. | closed | `[DIRECT][Static] + [Ran]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-13 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-11 authority anchors, erosion invariants, guard map, symbol alias map, obligations, tolerances, and gap register for SCI-13 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence-mode casing, corrected `Di` non-negative continuity language, added `ER` alias coverage, added per-row evidence tags in degenerate states, and narrowed companion-gap wording. |
| `2026-05-23` | `3` | `Codex` | WB16 amendment: added hydrologic WB16 peak/duration intake authority (`peakro`, `watdur`) with continuity, traceability, and typed guard requirements for erosion coupling readiness. |
| `2026-05-23` | `4` | `Codex` | EROD11 amendment: ratified Wave-0 alias ownership across runoff/peak-duration and sediment handoff boundaries, added explicit cross-contract boundary ownership register, and downgraded `GAP-SED-002` from non-promotable to promotable-with-risk pending `EROD13+` internal alias expansion. |
| `2026-05-23` | `5` | `Codex` | EROD11 closure amendment: dispositioned alias-ownership ambiguity row `GAP-SED-002` to `closed` for required boundary symbols and made explicit that erosion-physics implementation remains separately governed by non-promotable holds. |
| `2026-05-23` | `6` | `Codex` | EROD11 risk-acceptance amendment: dispositioned `GAP-SED-001` and `GAP-SED-004` from promotable-with-risk to `closed` via explicit governance risk acceptance while preserving non-promotable erosion-physics HOLD posture. |
| `2026-05-23` | `7` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum for required erosion-lane boundaries and dispositioned `GAP-SED-003` to `closed` while preserving non-Wave-0 implementation holds. |
| `2026-05-25` | `8` | `Codex` | EROD13 Wave-1 amendment: added pinned-baseline legacy authority anchors (`param.for`, `erod.for`, `runge.for`), runtime integration semantics, algorithm/guard specification, and contract-derived vector obligations for `INV-SED-001`..`007` core execution. |
| `2026-05-25` | `9` | `Codex` | EROD14 Wave-2 amendment: added multi-OFE case-classification/runtime authority and class-wise enrichment mass-conservation closure semantics (`INV-SED-008..009`) with typed guard-family continuity (`HKERNEL-EROD14-WAVE2-E-001..003`). |
| `2026-05-25` | `10` | `Codex` | EROD15 Wave-3 amendment: replaced generic sediment handoff naming with HBP pass-serialization field authority, added contributor-prefixed routing alias family, and added Wave-3 export mapping/guard continuity requirements for `INV-SED-010`. |
| `2026-05-25` | `11` | `Codex` | MOFE03 amendment: added production runner activation/seeding authority for `erod14_wave2_enabled` and enabled-path deterministic Wave-2 ingress synthesis from aligned topology/runtime surfaces with typed hard-fail derivation posture. |
| `2026-05-26` | `12` | `Codex` | EROD16 amendment: added canonical hillslope `CONTIN -> ROUTE` branch-authority mapping (`mshear 1..5`, `depc/depend/depos/erod/enrich` routine chain), codified route-branch invariants and alias continuity requirements, corrected `rtpart.for` provenance classification, and opened `GAP-SED-005` until runtime migration closure. |
| `2026-05-26` | `13` | `Codex` | EROD21 closure amendment: dispositioned `GAP-SED-005` to `closed` after EROD19 runtime migration + EROD21 parity rerun/hold-lift evidence, and updated impact language to reflect landed process parity. |
| `2026-05-27` | `14` | `Codex` | WSHEDIMPL01 amendment: normalized unresolved cross-domain watershed sediment closure dependency (`GAP-SED-006`) so `SC-SED-001` boundary payload authority remains explicit while companion channel sediment migration (`chnero/chnrt/detach`) remains non-promotable until WSHED06 closure evidence lands. |
| `2026-05-27` | `15` | `Codex` | WSHEDIMPL06 amendment: ratified WS11 channel sediment publication-family closure (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) while preserving non-promotable `GAP-SED-006` posture for unresolved full `chnero/chnrt/detach` process-parity migration and validation closure. |
| `2026-05-27` | `16` | `Codex` | WSHEDIMPL15 amendment: ratified WS15 channel-sediment control projection and baseline conversion scaffold publication (`ws10_channel_{id}_{crsh,depmid,depsid}` with fail-closed guards) while preserving non-promotable `GAP-SED-006` posture pending full companion `chnero/chnrt/detach` process migration closure. |
| `2026-05-27` | `17` | `Codex` | WSHEDIMPL16 amendment: ratified contributor `particle_diameter_m` payload ingress projection (`hs{ID}_particle_diameter_m_{class:04}`) with fail-closed WS10 guard continuity, and narrowed `GAP-SED-006` to remaining full companion `chnero/chnrt/detach` process migration closure scope. |
| `2026-05-27` | `18` | `Codex` | WSHEDIMPL17 amendment: ratified WS17 segment/hydraulic scaffold projection/guard closure (`ws10_channel_{id}_nslpts` + segment `x/slope/depa/depb/wida/widb` families) and narrowed `GAP-SED-006` to remaining full companion `chnero/chnrt/detach` process-family migration scope. |
| `2026-05-27` | `19` | `Codex` | WSHEDIMPL18 amendment: migrated baseline `shield`/`trncap` transport-capacity authority into WS10 channel sediment publication (`tc`) using class-aware contributor payload aggregation and removed surrogate `tc=qsed` identity coupling, while preserving non-promotable `GAP-SED-006` posture for unresolved channel detachment/deposition segment-loop families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `20` | `Codex` | WSHEDIMPL19 amendment: ratified fail-closed WS10 channel sediment branch payload export (`particle_class_count`, `particle_flow_fraction_{class:04}`, `particle_diameter_m_{class:04}`) and upstream channel-dependency payload ingress continuity for class-aware aggregation, while preserving non-promotable `GAP-SED-006` posture for unresolved channel detachment/deposition segment-loop families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` inflow-partition parity closure. |
| `2026-05-27` | `21` | `Codex` | WSHEDIMPL20 amendment: added opt-in WS20 segment-loop `case12` routing scaffolding with unresolved-detachment diagnostics publication and retained non-promotable `GAP-SED-006` posture for remaining baseline-authoritative channel detachment/deposition families (`case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `22` | `Codex` | WSHEDIMPL21 amendment: added WS10 opt-in WS21 case34/enddet diagnostics scaffolding (`ws21_case3_segment_count`, `ws21_case4_segment_count`, `ws21_enddet_segment_count`, `ws21_detach_unmigrated_segment_count`) with explicit unresolved detach-capacity diagnostics publication, while retaining non-promotable `GAP-SED-006` posture for remaining baseline-authoritative `detach/dcap` migration and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `23` | `Codex` | WSHEDIMPL22 amendment: replaced WS21 opt-in unresolved fallback with baseline-lineage `dcap` + `case34/enddet` execution and required fail-closed `crfrac` projection gating (`ws10_channel_{id}_crfrac_{class:04}`), while retaining non-promotable `GAP-SED-006` posture for residual WS21 `case4 -> detach` iterative closure (`nt < cnpart`) and remaining full `chnero/chnrt` parity closure. |
| `2026-05-27` | `24` | `Codex` | WSHEDIMPL23 amendment: migrated baseline-authoritative `detach.for` iterative closure behavior for WS21 `case4` rows (`nt < cnpart`) and removed residual WS21 unresolved-detachment fallback requirement for that branch, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `25` | `Codex` | WSHEDIMPL24 amendment: migrated baseline-authoritative `case12.for` deposition-to-detachment transition continuation (`xdemax < x(i)` into `detach.for`) and added explicit transition diagnostics publication (`ws24_case2_detach_segment_count`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `26` | `Codex` | WSHEDIMPL25 amendment: closed residual WS20 opt-in unresolved-detachment fallback behavior by auto-activating WS21 migration lanes under WS20 opt-in and enforcing fail-closed `crfrac` requirements for WS20-only opt-in lanes, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `27` | `Codex` | WSHEDIMPL26 amendment: migrated baseline-authoritative `dcap(flagm=2)` max-detachment limiter semantics for WS23 iterative detach closure lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `28` | `Codex` | WSHEDIMPL27 amendment: migrated baseline-authoritative `enddet.for` bracket progression semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `29` | `Codex` | WSHEDIMPL28 amendment: migrated baseline-authoritative `chnrt.for` segment boundary-width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in WS20 segment-loop routing lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `30` | `Codex` | WSHEDIMPL29 amendment: migrated rectangular-channel width-mutation semantics by projecting `dcap` eroded-width outcomes (`werb`) into WS20 `widb(i-1)` updates and state-symbol writeback (`ws10_channel_{id}_widb_{point:04}`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `31` | `Codex` | WSHEDIMPL30 amendment: migrated erodible-lane shape-transition continuity by activating `ishape=3` routing pathways and applying `depa/depb`-driven rectangular fallback mapping for WS20/WS21 hydraulic and detach-capacity calls, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `32` | `Codex` | WSHEDIMPL31 amendment: migrated baseline-authoritative lower-boundary width-mutation continuity (`flagc=2`, `wera>wfl`) by projecting detach eroded-width outcomes (`wera`) into WS20 rectangular-lane `wida(i)` updates and state-symbol writeback (`ws10_channel_{id}_wida_{point:04}`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `33` | `Codex` | WSHEDIMPL32 amendment: reconciled parser/runtime naturally eroded shape-class lineage by aligning watershed channel parser projection and WS10 runtime consumption on explicit `ishape=3` mapping semantics (strict domain `1..=3`, compatibility `ishape>3 -> 3`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `34` | `Codex` | WSHEDIMPL33 amendment: reconciled parser/runtime channel `ienslp` lineage by aligning watershed channel parser projection and WS10 runtime seed validation on explicit `ienslp` domain semantics (`1..=2`, fail-closed out-of-domain), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `35` | `Codex` | WSHEDIMPL34 amendment: reconciled parser/runtime watershed-channel Manning relation lineage by aligning parser projection authority and WS10 runtime seed validation on explicit `chnn >= chnnbr` fail-closed semantics, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `36` | `Codex` | WSHEDIMPL35 amendment: reconciled parser/runtime channel control lineage by projecting `icntrl`/`flgout` into WS10 runtime seed surfaces with explicit fail-closed domain semantics (`icntrl in [0,4]`, `flgout in [0,1]`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `37` | `Codex` | WSHEDIMPL36 amendment: reconciled parser/runtime rating-curve control lineage by projecting `ws10_channel_{id}_{rccoef,rcexp,rcoset}` for `icntrl==4` lanes into WS10 runtime seed surfaces with explicit fail-closed payload-shape/domain semantics (`rccoef>0`, `rcexp>0`, `rcoset>=0`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `38` | `Codex` | WSHEDIMPL37 amendment: added trace linkage for companion WS11 hydrology route-chain parity closure (`wshcqi/wshirs/wshrun`) and `GAP-ROUTE-008` disposition while preserving non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `39` | `Codex` | WSHEDIMPL38 amendment: closed `GAP-SED-006` by retiring unresolved-detachment diagnostics symbols and replacing residual WS20/WS21 invalid-segment fallback continuation with typed fail-closed domain guards in companion channel sediment process lanes. |
| `2026-05-28` | `40` | `Codex` | HILLSTAB04 amendment: aligned EROD14 Wave-2 reproportion closure semantics to baseline `enrich.for` for all-class `sedmax` saturation (`ratbot=0` clipping pass re-entry) and updated contract-derived vector obligations to prohibit non-authoritative hard-fail behavior on that branch. |
