---
contract_id: SC-SED-001
title: Hillslope Erosion Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 8
producer_scope:
  - Hillslope sediment continuity, detachment/deposition, and transport-capacity surfaces
  - Event erosion boundary payloads consumed by routing/channel domains
  - Sediment size-class and enrichment surfaces at OFE and hillslope exits
consumer_scope:
  - Watershed/channel routing consumers requiring hillslope erosion payload semantics
  - Comparator and replay consumers using erosion closure and sign-consistency surfaces
  - Adjacent soil/runoff/hydraulics domains providing required coupling inputs
evidence_level: Static
last_reviewed: 2026-05-25
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
| REF-SED-LEGACY-PARAM | `/workdir/wepp-forest_260430_baseline/src/param.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy normalized-parameter authority (`eata`, `tauc`, `theta`, `phi`) used for Wave-1 runtime parameter derivation provenance. | `[DIRECT][Static]` |
| REF-SED-LEGACY-EROD | `/workdir/wepp-forest_260430_baseline/src/erod.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy detachment-capacity and branch-condition authority used for Wave-1 detachment/deposition runtime branch ordering. | `[DIRECT][Static]` |
| REF-SED-LEGACY-RUNGE | `/workdir/wepp-forest_260430_baseline/src/runge.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy continuity evolution form (`dG/dx` update term as `dcap*((tcap-load)/tcap) + theta`) used for Wave-1 branch/continuity guard alignment. | `[DIRECT][Static]` |
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
| `sed_det_total`, `sed_dep_total` | `kg` | Hillslope event detachment/deposition totals exported in watershed pass-file semantics. | hillslope erosion aggregator | channel/watershed routing consumers |
| `sed_conc_i`, `sed_frac_i` | `kg m^-3`, `fraction` | Particle-class concentration and class fraction at hillslope/OFE exits. | size-class routing/enrichment pathway | channel/watershed routing consumers |
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
| INV-SED-010 | Coupling payload invariant: hillslope erosion event outputs (`sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i`) are emitted with units/sign conventions required by watershed/channel consumers. | hard-fail | REF-SED-CH13-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `sed_det_total`, `sed_dep_total` | identity names | hillslope sediment totals to routing pass-file boundary | `kg` preserved | `[DIRECT][Static]` |
| `sed_conc_i`, `sed_frac_i` | identity names | sediment class concentration/fraction boundary surfaces | `kg m^-3` and fraction semantics preserved | `[DIRECT][Static]` |
| `ER` | identity name | enrichment-ratio boundary surface | fraction semantics preserved | `[DIRECT][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionFluxSymbol::Wb12RunoffQ`; `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur,Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB12/WB16 kernels | `SC-SED-001` (`INV-SED-004`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-002` | `fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-HYDRAULICS-001` | `SC-SED-001` (`INV-SED-005`, `INV-SED-006`, `INV-SED-007`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-003` | `sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-SED-001` | `SC-ROUTE-001` (`INV-ROUTE-011`) | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hydrology forcing intake (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` (`INV-RUNOFFPART-009`, `INV-RUNOFFPART-011`) + `SC-WATBAL-001` (`INV-WATBAL-007`, `INV-WATBAL-016`) | `SC-SED-001` (`INV-SED-004`) | Canonical producer/consumer guard ownership is explicit for required Wave-0 forcing symbols. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydraulics shear/friction intake (`fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe`) | `SC-HYDRAULICS-001` (`INV-HYDRAULICS-009`..`011`) | `SC-SED-001` (`INV-SED-005`..`007`) | Guard ownership and failure posture are explicit with no remaining Wave-0 ownership ambiguity. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment payload export to routing (`sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i`) | `SC-SED-001` (`INV-SED-010`) | `SC-ROUTE-001` (`INV-ROUTE-011`) | Cross-domain payload validation ownership is explicit for downstream routing intake. | `[DIRECT][Static] + [INFERENCE][Static]` |

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
- Missing or unit-inconsistent hillslope-to-routing sediment payload fields (`sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i`). `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SED-P-001: Publish hillslope erosion continuity and branch surfaces using canonical Chapter-11 symbol semantics and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-002: Enforce explicit detachment/deposition branch predicates and threshold behavior before emitting event outputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-003: Propagate invariant violations as typed errors; do not silently clamp or default materially invalid erosion states. `[INFERENCE][Static]`
- OBL-SED-P-004: Emit routing-boundary sediment payload completeness (`sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i`) with unit/sign integrity. `[DIRECT][Static] + [INFERENCE][Static]`

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

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SED-001 | Per-invariant comparator vectors for sediment-branch transitions and class-wise enrichment closures remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where those vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-SED-002 | Wave-0 erosion-lane alias-ownership ambiguity for required cross-contract boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-003 | EROD12 ratifies cross-domain ownership/guard closure for required erosion-lane companion boundaries (`SC-HYDRAULICS-001`, `SC-ROUTE-001`) using canonical `SC-*` addenda and row-scoped guard ownership mapping. | Required Wave-0 cross-domain ownership semantics are canonicalized; erosion production implementation remains separately gated by `EROD13+` and non-Wave-0 companion gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-004 | Chapter-11 enrichment caveats for mixed-soil, multi-OFE composition effects remain and are explicitly retained as a documented limitation with governance risk acceptance. | Mixed-soil enrichment interpretation may still require manual investigation; this is accepted as an explicit model-governance caveat. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |

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
