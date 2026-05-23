---
contract_id: SC-HYDRAULICS-001
title: Overland Hydraulics Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 4
producer_scope:
  - Overland-flow friction-factor and rill-geometry state surfaces
  - Shear-partition semantics coupling hydraulics to hillslope erosion
  - Boundary payloads needed by runoff partition and sediment-continuity consumers
consumer_scope:
  - Runoff-partition and hillslope-routing consumers requiring friction and width semantics
  - Soil and management consumers that provide roughness/cover/canopy controls
  - Hillslope erosion consumers requiring soil-active shear and transport-capacity inputs
evidence_level: Static
last_reviewed: 2026-05-23
supersedes: []
superseded_by: []
---

# SC-HYDRAULICS-001 Overland Hydraulics Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for overland-flow hydraulics in openWEPP,
including friction-factor construction for cropland and rangeland,
rill/interrill partition semantics, rill-width estimation, and shear-partition
coupling required by hillslope erosion calculations.

## Scientific Scope

In scope:
- Darcy-Weisbach friction-factor semantics and equivalent friction aggregation
  for rill/interrill areas. `[DIRECT][Static]`
- Cropland and rangeland roughness-coefficient equations for soil, residue,
  canopy, rock, and cryptogam cover effects. `[DIRECT][Static]`
- Temporal-variation obligations for roughness drivers (tillage, rainfall,
  decomposition, canopy variation) at the contract boundary. `[DIRECT][Static] + [INFERENCE][Static]`
- Rill-density and rill-width relation used by downstream shear and erosion
  routines. `[DIRECT][Static]`
- Soil-active shear partition semantics linking hydraulics to erosion equations.
  `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Sediment-detachment and transport-capacity solution internals owned by
  `SC-SED-001`. `[INFERENCE][Static]`
- Runoff-generation/infiltration internals owned by `SC-RUNOFFPART-001`. `[INFERENCE][Static]`
- Channel/watershed routing mechanics owned by `SC-ROUTE-001`. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-HYD-CH10-FOUND | `references/50201000/chap10.pdf` §10.1 Eq. [10.1.1]-[10.1.2] | Darcy-Weisbach foundation and equivalent cropland friction aggregation. | `[DIRECT][Static]` |
| REF-HYD-CH10-CROP-RILL | `references/50201000/chap10.pdf` §10.2 Eq. [10.2.1]-[10.2.4] | Cropland rill friction composition and canopy-residue terms. | `[DIRECT][Static]` |
| REF-HYD-CH10-CROP-INT | `references/50201000/chap10.pdf` §10.3 Eq. [10.3.1]-[10.3.5] | Cropland interrill friction composition and roughness/residue relations. | `[DIRECT][Static]` |
| REF-HYD-CH10-RANGE-RILL | `references/50201000/chap10.pdf` §10.4 Eq. [10.4.1]-[10.4.7] | Rangeland rill friction relations and lower-bound rule (`frr >= frs`). | `[DIRECT][Static]` |
| REF-HYD-CH10-RANGE-INT | `references/50201000/chap10.pdf` §10.5 Eq. [10.5.1]-[10.5.6] | Rangeland interrill friction relations and lower-bound rule (`fir >= fbi`). | `[DIRECT][Static]` |
| REF-HYD-CH10-TEMP | `references/50201000/chap10.pdf` §10.6 | Temporal variation drivers for hydraulics coefficients. | `[DIRECT][Static]` |
| REF-HYD-CH10-WIDTH | `references/50201000/chap10.pdf` §10.7 Eq. [10.7.1] | Rill-width relation from rill discharge. | `[DIRECT][Static]` |
| REF-HYD-CH11-SHEAR | `references/50201000/chap11.pdf` §11.2.3 Eq. [11.2.7] | Soil-active shear partition uses `fs/ft` ratio with rill hydraulics terms. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-HYD-CH11-TC | `references/50201000/chap11.pdf` §11.2.4 Eq. [11.2.8] | Transport-capacity dependence on hydraulic shear (`τf`). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-HYD-CH7-ROUGH | `references/50201000/chap7.pdf` §7.2, §7.5 | Soil roughness state supplied by soil/tillage pathways affects hydraulics coefficients. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-HYD-CH4-RUNOFF | `references/50201000/chap4.pdf` §4.4 + `references/50201000/chap10.pdf` §10.7 | Rainfall excess and rill density/discharge context supplies `Qe` used for rill width and shear. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-HYD-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative fractions/rates, finite coefficients, bounded shear-partition ratios. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `f` | `dimensionless` | Darcy-Weisbach friction coefficient (Eq. [10.1.1]). | hydraulics core | coefficient family constructors |
| `fe`, `fr`, `fi` | `dimensionless` | Equivalent cropland friction and rill/interrill component totals (Eq. [10.1.2], [10.2.1], [10.3.1]). | hydraulics core | runoff routing + erosion shear coupling |
| `frr`, `fir` | `dimensionless` | Equivalent rangeland rill/interrill friction totals (Eq. [10.4.1], [10.5.1]). | hydraulics core | runoff routing + erosion shear coupling |
| `fsr`, `fcr`, `flive` | `dimensionless` | Cropland rill soil/residue/live-plant friction components (Eq. [10.2.2]-[10.2.4]). | hydraulics core + canopy/residue inputs | cropland rill friction aggregator |
| `fsi`, `fci`, `fbi`, `fo` | `dimensionless` | Cropland interrill roughness/cover/bare-soil/form coefficients (Eq. [10.3.2]-[10.3.5]). | hydraulics core + soil roughness inputs | cropland interrill friction aggregator |
| `frs`, `fro`, `fcs`, `frkr`, `fltr`, `fpbr` | `dimensionless` | Rangeland rill soil/random/cover and subcomponent coefficients (Eq. [10.4.1]-[10.4.7]). | hydraulics core + rangeland cover inputs | rangeland rill friction aggregator |
| `fri`, `frki`, `flti`, `fpbi` | `dimensionless` | Rangeland interrill cover-total and subcomponent coefficients (Eq. [10.5.3]-[10.5.6]). | hydraulics core + rangeland cover inputs | rangeland interrill friction aggregator |
| `Ar` | `fraction` | Fraction of area in rills for equivalent-cropland weighting. | topography/management surfaces | Eq. [10.1.2] aggregator |
| `rc`, `ic`, `rrock`, `irock` | `fraction` | Residue/rock cover fractions for rill/interrill relations. | residue/rock cover surfaces | friction component equations |
| `Bar`, `Bai`, `Cc`, `Cccr`, `Ccci` | `fraction` | Basal/canopy/cryptogam cover fractions on rangelands. | vegetation/cover surfaces | rangeland friction equations |
| `canhgt`, `hmax` | `m`, `m` | Actual and maximum canopy height for live-plant friction scaling. | plant state surfaces | Eq. [10.2.4] |
| `ro`, `ri`, `rr` | `m`, `fraction`, `m` | Initial/ratio/current random roughness controls for cropland and rangeland relations. | soil state surfaces | Eq. [10.3.2]-[10.3.3], [10.4.3] |
| `Qe`, `w` | `m^3 s^-1`, `m` | Rill discharge and resulting rill width relation (Eq. [10.7.1]). | runoff partition + hydraulics width routine | shear and erosion coupling |
| `R`, `S`, `V` | `m`, `fraction`, `m s^-1` | Hydraulic radius, slope, and flow velocity in Darcy-Weisbach relation. | hydraulics geometry state | friction calculation + shear coupling |
| `fs`, `ft`, `τf`, `τfe` | `dimensionless`, `dimensionless`, `Pa`, `Pa` | Soil/total friction factors and hydraulic shear terms used in erosion coupling Eq. [11.2.7]-[11.2.8]. | hydraulics + erosion-coupling boundary | detachment and transport-capacity consumers |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-HYDRAULICS-001 | Friction-domain invariant: all emitted friction coefficients (`f`, `fe`, `fr`, `fi`, `frr`, `fir` and component terms) must be finite and non-negative. | hard-fail | REF-HYD-CH10-FOUND, REF-HYD-CH10-CROP-RILL, REF-HYD-CH10-CROP-INT, REF-HYD-CH10-RANGE-RILL, REF-HYD-CH10-RANGE-INT, REF-HYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-002 | Equivalent-cropland weighting invariant: Eq. [10.1.2] must be respected so `fe` is computed as area-weighted combination of `fr` and `fi` using `Ar`, with `0 <= Ar <= 1`. | hard-fail | REF-HYD-CH10-FOUND, REF-HYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-003 | Cropland-rill composition invariant: Eq. [10.2.1]-[10.2.4] branch semantics must be applied so `fr = fsr + fcr + flive` with explicit residue and canopy terms. | hard-fail | REF-HYD-CH10-CROP-RILL | `[DIRECT][Static]` |
| INV-HYDRAULICS-004 | Cropland-interrill composition invariant: Eq. [10.3.1]-[10.3.5] branch semantics must be applied so `fi = fsi + fci + fbi + flive`, with explicit roughness-ratio inputs and nonmoveable-residue fractions. | hard-fail | REF-HYD-CH10-CROP-INT | `[DIRECT][Static]` |
| INV-HYDRAULICS-005 | Rangeland-rill floor invariant: Eq. [10.4.1]-[10.4.7] must be applied with explicit lower bound `frr >= frs`; if computed `frr < frs`, corrected `frr` equals `frs`. | hard-fail | REF-HYD-CH10-RANGE-RILL | `[DIRECT][Static]` |
| INV-HYDRAULICS-006 | Rangeland-interrill floor invariant: Eq. [10.5.1]-[10.5.6] must be applied with explicit lower bound `fir >= fbi`; if computed `fir < fbi`, corrected `fir` equals `fbi`. | hard-fail | REF-HYD-CH10-RANGE-INT | `[DIRECT][Static]` |
| INV-HYDRAULICS-007 | Fraction-domain invariant: cover and area fractions (`Ar`, `rc`, `ic`, `rrock`, `irock`, `Bar`, `Bai`, `Cc`, `Cccr`, `Ccci`) must remain in `[0,1]` within tolerance; out-of-range values are invalid. | hard-fail | REF-HYD-CH10-CROP-RILL, REF-HYD-CH10-CROP-INT, REF-HYD-CH10-RANGE-RILL, REF-HYD-CH10-RANGE-INT, REF-HYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-008 | Temporal-driver invariant: hydraulics updates must consume declared time-varying roughness/cover/canopy drivers from §10.6 without silent freezing of coefficients across events. | hard-fail | REF-HYD-CH10-TEMP, REF-HYD-CH7-ROUGH | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-009 | Rill-width invariant: when Eq. [10.7.1] is used, `Qe` must be non-negative and `w` must be finite and non-negative, with explicit rill-density assumption handling. | hard-fail | REF-HYD-CH10-WIDTH, REF-HYD-CH4-RUNOFF, REF-HYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-010 | Shear-partition invariant: soil-active shear coupling must preserve Eq. [11.2.7] semantics with explicit `fs/ft` ratio, requiring finite `ft > 0`, `fs >= 0`, and bounded partition ratio in `[0,1]` for physically valid shear partition. | hard-fail | REF-HYD-CH11-SHEAR, REF-HYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-011 | Erosion-coupling completeness invariant: hydraulics boundary payload to erosion must include required friction/shear/width semantics (`fr`, `fi`/`fe`, `w`, `τfe` or equivalent declared shear surfaces) with unit-consistent sign conventions. | hard-fail | REF-HYD-CH11-SHEAR, REF-HYD-CH11-TC, REF-HYD-CH10-WIDTH | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-HYDRAULICS-012 | Governance limitation invariant: contract interpretation must explicitly carry Chapter-10 limitations (no litter transport prediction, no debris-dam dynamics, constant gravel/cobble cover assumption, no erosion-pavement formation) and block promotion when omitted. | governance-fail | REF-HYD-CH10-RANGE-RILL, REF-HYD-CH10-RANGE-INT, REF-HYD-CH10-TEMP | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-HYDRAULICS-001` | runtime | Friction-domain validator | Typed hard error on negative/non-finite friction factors | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-002` | runtime | Equivalent friction assembler | Typed hard error on weighting residual or invalid `Ar` domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-003` | runtime | Cropland-rill friction calculator | Typed hard error on Eq. [10.2.*] branch mismatch | Tier-A gate | `[DIRECT][Static]` |
| `INV-HYDRAULICS-004` | runtime | Cropland-interrill friction calculator | Typed hard error on Eq. [10.3.*] branch mismatch | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-HYDRAULICS-005` | runtime | Rangeland-rill floor-rule validator | Typed hard error when `frr` floor correction semantics are violated | Tier-A gate | `[DIRECT][Static]` |
| `INV-HYDRAULICS-006` | runtime | Rangeland-interrill floor-rule validator | Typed hard error when `fir` floor correction semantics are violated | Tier-A gate | `[DIRECT][Static]` |
| `INV-HYDRAULICS-007` | runtime | Fraction-domain validator | Typed hard error on out-of-domain fraction inputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-008` | runtime | Temporal driver ingestion validator | Typed hard error when required roughness/cover drivers are missing or silently ignored | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-009` | runtime | Rill width calculator/validator | Typed hard error on invalid `Qe` domain or non-finite/negative `w` | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-010` | runtime | Shear-partition boundary validator | Typed hard error on invalid `fs/ft` partition domains or undefined shear terms | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-011` | runtime | Hydraulics-to-erosion payload validator | Typed hard error on missing required friction/shear/width payload fields | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-HYDRAULICS-012` | governance | Review/disposition/promotion checklist | Promotion `HOLD` when Chapter-10 limitation labels are missing | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-10 and Chapter-11 WEPP
notation. Concrete openWEPP runtime-field names are not fixed yet, so identity
aliases are required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `f`, `fe`, `fr`, `fi`, `frr`, `fir` | identity names | primary friction-factor surfaces | dimensionless preserved | `[DIRECT][Static]` |
| `fsr`, `fcr`, `flive`, `fsi`, `fci`, `fbi`, `fo` | identity names | cropland component-friction surfaces | dimensionless preserved | `[DIRECT][Static]` |
| `frs`, `fro`, `fcs`, `frkr`, `fltr`, `fpbr`, `fri`, `frki`, `flti`, `fpbi` | identity names | rangeland component-friction surfaces | dimensionless preserved | `[DIRECT][Static]` |
| `Ar`, `rc`, `ic`, `rrock`, `irock`, `Bar`, `Bai`, `Cc`, `Cccr`, `Ccci` | identity names | cover/area-fraction boundary surfaces | fraction semantics preserved | `[DIRECT][Static]` |
| `canhgt`, `hmax`, `ro`, `ri`, `rr` | identity names | canopy/roughness driver surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qe`, `w` | identity names | rill discharge/width surfaces | `m^3 s^-1`, `m` preserved | `[DIRECT][Static]` |
| `R`, `S`, `V` | identity names | hydraulic geometry/velocity surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `fs`, `ft`, `τf`, `τfe` | identity names | shear-partition and erosion-coupling surfaces | `dimensionless`, `Pa` preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-cover cropland rill | `rc = 0` so Eq. [10.2.3] yields `fcr = 0`, and rill friction is governed by `fsr + flive`. | Explicit functional behavior of Eq. [10.2.3] and Eq. [10.2.1]. | `[DIRECT][Static]` |
| No-cover cropland interrill | `ic = 0` so Eq. [10.3.4] yields `fci = 0`; interrill friction remains `fsi + fbi + flive`. | Explicit functional behavior of Eq. [10.3.4] and Eq. [10.3.1]. | `[DIRECT][Static]` |
| Floor-rule activated rangeland rill | Raw Eq. [10.4.1] result below `frs`; model sets `frr = frs`. | Explicit rule statement in §10.4. | `[DIRECT][Static]` |
| Floor-rule activated rangeland interrill | Raw Eq. [10.5.1] result below `fbi`; model sets `fir = fbi`. | Explicit rule statement in §10.5. | `[DIRECT][Static]` |
| Zero-rill-discharge width limit | `Qe = 0` with Eq. [10.7.1] giving `w = 0`; no negative width is emitted. | Power-law form in Eq. [10.7.1] at zero discharge. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Negative or non-finite friction coefficients on any exported hydraulics surface. `[DIRECT][Static] + [INFERENCE][Static]`
- `Ar`, cover fractions, or canopy/cryptogam fractions outside `[0,1]` beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing floor-rule enforcement in §10.4 (`frr >= frs`) or §10.5 (`fir >= fbi`). `[DIRECT][Static]`
- Negative `Qe` or non-finite rill width `w` in Eq. [10.7.1] workflows. `[DIRECT][Static] + [INFERENCE][Static]`
- Invalid shear partition with `ft <= 0`, `fs < 0`, or undefined `fs/ft` ratio used for soil-active shear coupling. `[DIRECT][Static] + [INFERENCE][Static]`
- Hydraulics-to-erosion payload missing required friction/shear/width semantics. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-HYD-P-001: Emit friction, roughness-driver, and rill-width surfaces using canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-HYD-P-002: Apply Eq. [10.1.*] through Eq. [10.7.1] branch logic explicitly; no silent fallback or coefficient freezing. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-HYD-P-003: Enforce invariant violations via typed errors; no silent clamping/defaulting for materially invalid hydraulic states. `[INFERENCE][Static]`
- OBL-HYD-P-004: Publish shear-partition-ready payloads needed by erosion consumers (`fs`, `ft`, `τf`/`τfe`, `w`) with explicit units/sign semantics. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-HYD-C-001: Runoff-routing consumers must preserve friction and width semantics without unit conversion drift. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-HYD-C-002: Erosion consumers must consume soil-active shear partition semantics consistent with Eq. [11.2.7]-[11.2.8]. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-HYD-C-003: Soil/residue/plant providers must publish bounded roughness and cover fractions required by Chapter-10 relations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-HYD-C-004: All consumers must fail explicitly on invariant-violating hydraulics payloads and retain invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Friction-domain and area-weighted aggregation (`INV-HYDRAULICS-001/002`) | friction assembly stage | Hard error on invalid domains or aggregation mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Cropland/rangeland composition and floor rules (`INV-HYDRAULICS-003`..`006`) | component-friction calculators | Hard error on branch mismatch or floor-rule violation | Tier-A gate | `[DIRECT][Static]` |
| Fraction domains and temporal drivers (`INV-HYDRAULICS-007/008`) | boundary ingestion and update stage | Hard error on invalid/missing roughness-cover inputs | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rill width and shear partition coupling (`INV-HYDRAULICS-009/010`) | width + erosion-coupling stage | Hard error on nonphysical width/shear partition states | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Erosion payload completeness (`INV-HYDRAULICS-011`) | hydraulics-to-erosion handoff | Hard error on missing required payload fields | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Chapter-10 limitation labeling (`INV-HYDRAULICS-012`) | review/verification/promotion | Governance `HOLD` until limitations are explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). `[DIRECT][Static]` Contract-level tolerance declarations:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-HYD-001 | Non-negative friction tolerance for coefficient surfaces | lower bound `>= -1e-12` | Comparator-noise allowance only; runtime hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-HYD-002 | Fraction-domain tolerance for area/cover fractions | `-1e-12 <= value <= 1 + 1e-12` | Preserves bounded fraction semantics under floating-noise. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-HYD-003 | Equivalent-cropland aggregation residual tolerance for Eq. [10.1.2] | `<= 1e-12` absolute residual | Residual evaluated as `fe - (fr*Ar + fi*(1-Ar))`. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-HYD-004 | Rill-width non-negative tolerance for Eq. [10.7.1] | lower bound `>= -1e-12 m` | Negative widths beyond tolerance are invariant failures. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-HYD-005 | Shear-partition ratio tolerance for `fs/ft` | `-1e-12 <= fs/ft <= 1 + 1e-12` | Runtime rejects materially out-of-domain partition ratios. | `[DIRECT][Static] + [INFERENCE][Static]` |

## WB16 Peak-Flow Coupling Readiness Addendum

### WB16 Required Incoming Hydrology Surfaces

| Surface | Symbols |
|---|---|
| Peak-runoff coupling inputs | `peakro`, `watdur`, `Q` |
| WB16 diagnostics metadata | `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

### WB16 Coupling Rules

1. Hydraulics coupling acceptance for WB16 requires finite, non-negative
   `peakro` and `watdur` with explicit continuity `watdur = Q/peakro`
   (within declared numeric tolerance).
2. WB16 branch metadata is required for traceability and comparator
   diagnostics; missing branch metadata is a typed boundary failure.
3. Hydraulics consumers must not reconstruct peak-runoff by fallback formulas
   when WB16 peak surfaces are present; WB16 outputs are authoritative.
4. Missing/non-finite/out-of-domain WB16 peak surfaces are hard-fail states.

### WB16 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB16-PEAK-E-001` |
| Non-finite required symbol | `HKERNEL-WB16-PEAK-E-002` |
| Domain/closure violation | `HKERNEL-WB16-PEAK-E-003` |

## WS10 Routing/Impoundment Consumer Coupling Addendum

### WS10 Consumer Coupling Rules

1. Watershed routing/impoundment consumers must treat WB16-derived peak-flow
   payloads (`peakro`, `watdur`) as authoritative inputs for downstream WS10
   assembly and must not silently reconstruct substitute peaks when payloads are
   present.
2. WS10 consumers must preserve branch provenance emitted by WS10 channel and
   impoundment kernels (`ws10_channel_*`, `ws10_impoundment_*`) as typed
   boundary payloads for downstream continuity diagnostics.
3. Missing/non-finite/out-of-domain coupling symbols at WS10 consumers are
   typed hard failures with explicit WS10 guard families:
   - `WKERNEL-WS10-CHANNEL-E-001..003`
   - `WKERNEL-WS10-IMPOUNDMENT-E-001..003`
4. Consumer-side fallback defaulting/clamping that masks invalid WS10 coupling
   payloads is prohibited.

### WS10 Contract-Derived Coupling Vectors

Minimum WS10 coupling vectors:
1. Nominal WS10 channel/impoundment payload consumption preserves finite
   non-negative routed peak/discharge terms.
2. Missing required WS10 coupling payload fails with corresponding `-E-001`
   guard family code.
3. Non-finite WS10 coupling payload fails with corresponding `-E-002` code.
4. Domain/dependency WS10 coupling violation fails with corresponding `-E-003`
   code.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-HYD-001 | Per-invariant comparator vectors for all hydraulics families are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-HYD-002 | Concrete openWEPP runtime-field aliases for hydraulics/shear payloads are not yet fixed. | Alias map remains identity-only pending boundary finalization. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-HYD-003 | Coupled sediment contract `SC-SED-001` remains incomplete, so hydrology-to-erosion ownership boundaries are still provisional. | Promotion-readiness depends on companion contract completion/consistency. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-HYD-004 | Chapter-10 assumptions explicitly omit litter transport/debris dams and erosion pavement dynamics; explicit limitation labeling is present but implementation-level policy checks are not yet automated. | Governance enforcement is documented but not yet tooling-backed. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-12 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-10/11 authority anchors, invariants, guard map, alias map, obligations, boundary dispositions, tolerances, and gap register for SCI-12 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence-mode token casing, standardized Chapter-10 source-path anchors, added missing `τfe` alias coverage, and evidence-tagged degenerate/tolerance claims. |
| `2026-05-23` | `3` | `Codex` | WB16 amendment: added peak-flow coupling readiness authority requiring `peakro`/`watdur` boundary acceptance with WB16 diagnostic metadata and typed guard posture. |
| `2026-05-23` | `4` | `Codex` | WS10 amendment: added routing/impoundment consumer coupling authority for production WS10 payload families, including typed WS10 guard family requirements and WS10 coupling test-vector obligations. |
