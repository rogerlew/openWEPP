---
contract_id: SC-SOIL-001
title: Soil State and Erodibility Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 11
producer_scope:
  - Soil-state evolution surfaces (roughness, ridge state, bulk density, porosity)
  - Infiltration-facing conductivity parameter surfaces (effective and saturated conductivity)
  - Erodibility and critical-shear parameter surfaces consumed by hillslope erosion routines
consumer_scope:
  - Runoff/infiltration consumers requiring valid roughness and conductivity semantics
  - Percolation and water-balance consumers requiring conductivity and storage-domain consistency
  - Erosion/hydraulics consumers requiring valid interrill/rill erodibility and shear-threshold surfaces
evidence_level: Static
last_reviewed: 2026-05-30
supersedes: []
superseded_by: []
---

# SC-SOIL-001 Soil State and Erodibility Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for soil-state evolution and soil erodibility
semantics in openWEPP, including hydrology-facing conductivity behavior and
erosion-facing detachment-threshold behavior.

## Scientific Scope

In scope:
- Soil random roughness and ridge-height evolution after tillage and rainfall. `[DIRECT][Static]`
- Bulk-density, porosity, and conductivity update semantics that drive hydrology/percolation pathways. `[DIRECT][Static] + [INFERENCE][Static]`
- Interrill/rill erodibility and critical shear parameter semantics and temporal adjustments. `[DIRECT][Static]`
- Coupled boundary obligations for runoff partition, percolation, snow/freeze, and hillslope erosion consumers. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Channel/watershed sediment-routing mechanics owned by channel/watershed contracts. `[INFERENCE][Static]`
- Plant-growth and residue-decomposition internals except for soil-parameter coupling inputs they provide. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SOIL-CH7-INTRO | `references/50201000/chap7.pdf` §7.2.1-§7.2.2 | Declares the core soil-state and erodibility variables and their role in infiltration, runoff, and detachment. | `[DIRECT][Static]` |
| REF-SOIL-CH7-RR | `references/50201000/chap7.pdf` §7.5 Eq. [7.5.1]-[7.5.5] | Random roughness initialization/decay and roughness-driven critical-shear adjustment. | `[DIRECT][Static]` |
| REF-SOIL-CH7-RH | `references/50201000/chap7.pdf` §7.6 Eq. [7.6.1] | Ridge-height decay and explicit ridge-floor behavior for defined ridge-furrow systems. | `[DIRECT][Static]` |
| REF-SOIL-CH7-BD | `references/50201000/chap7.pdf` §7.7 Eq. [7.7.1]-[7.7.14] | Bulk-density updates from tillage, rainfall consolidation, and weathering consolidation. | `[DIRECT][Static]` |
| REF-SOIL-CH7-POR | `references/50201000/chap7.pdf` §7.8 Eq. [7.8.1]-[7.8.5] | Total/effective porosity and coarse-fragment/entrapped-air adjustments used in soil-water storage properties. | `[DIRECT][Static]` |
| REF-SOIL-CH7-KE | `references/50201000/chap7.pdf` §7.9 Eq. [7.9.1]-[7.9.22] | Effective/saturated conductivity semantics, temporal adjustment branches, and frozen-soil conductivity adjustment. | `[DIRECT][Static]` |
| REF-SOIL-CH7-KI | `references/50201000/chap7.pdf` §7.10 Eq. [7.10.1]-[7.10.15], Table 7.10.1, Table 7.10.4 | Baseline and adjusted interrill erodibility semantics and suggested domain limits for cropland/rangeland. | `[DIRECT][Static]` |
| REF-SOIL-CH7-KRTAU | `references/50201000/chap7.pdf` §7.11 Eq. [7.11.1]-[7.11.18], Table 7.10.1, Table 7.10.4 | Baseline and adjusted rill erodibility/critical shear semantics and suggested domain limits. | `[DIRECT][Static]` |
| REF-SOIL-CH4-DEPSTOR | `references/50201000/chap4.pdf` §4.3 Eq. [4.3.3]-[4.3.4] | Runoff partition consumes soil random roughness (`rr`) through maximum depression storage relation. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SOIL-CH5-PERC | `references/50201000/chap5.pdf` §5.4 Eq. [5.4.3]-[5.4.4] | Percolation uses saturated/adjusted conductivity (`Ksi`, `Ksai`) semantics tied to Chapter-7 soil parameterization. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SOIL-LEGACY-WB11 | `/workdir/wepp-forest_260430_baseline/src/watbal.for:960-967` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline layer-storage to aggregate-water lineage authority (`st(i)`/`soilw(i)`/`watcon`) consumed by ET/soil-water closure surfaces. | `[DIRECT][Static]` |
| REF-SOIL-CH11-ERODE | `references/50201000/chap11.pdf` §11.2 Eq. [11.2.3], §11.3 Eq. [11.3.7]-[11.3.10] | Hillslope erosion equations consume adjusted interrill/rill erodibility and critical shear parameters. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SOIL-CH7-FT | `references/50201000/chap7.pdf` §7.10.2.6, §7.11.2.4, §7.11.3.3 + `references/50201000/chap3.pdf` intro context | Freeze-thaw state and cycles influence soil erodibility and conductivity adjustments. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SOIL-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative depths/rates, bounded porosity fractions, and finite parameter domains. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `RRi`, `RRt`, `RRo`, `Tds`, `Rc`, `Cbr` | `m`, `m`, `m`, `fraction`, `m`, `fraction` | Random-roughness initialization/decay state and tillage/rainfall modifiers from Eq. [7.5.1]-[7.5.3]. | soil state updater | runoff partition + shear adjustment pathways |
| `RHt`, `RHo`, `RINT` | `m`, `m`, `m` | Ridge-height state, post-tillage ridge height, and ridge spacing from §7.6 and Table 7.5.1. | soil state updater | runoff partition microrelief semantics |
| `ρt`, `ρc`, `Δρrf`, `Δρwt`, `daycnt` | `kg m^-3`, `kg m^-3`, `kg m^-3`, `kg m^-3`, `d` | Bulk-density state and consolidation increments from Eq. [7.7.1]-[7.7.14]. | soil state updater | infiltration/percolation parameter surfaces |
| `φt`, `Fa`, `Fcf`, `φe` | `fraction`, `fraction`, `fraction`, `fraction` | Total porosity and adjusted effective porosity from Eq. [7.8.1]-[7.8.5]. | soil state updater | soil-water storage parameterization |
| `Kb`, `Kec`, `Ke` | `mm h^-1` | Baseline, constant-input, and event-effective conductivity semantics from §7.9 branch modes. | soil conductivity pathway | runoff infiltration pathway |
| `Ksi`, `Ksai`, `Bi` | `m s^-1`, `m s^-1`, `fraction` | Saturated and adjusted layer conductivity surfaces used by percolation Eq. [5.4.3]-[5.4.4]. | soil/percolation coupling pathway | percolation routing |
| `FSa`, `Fθ`, `θf`, `θfc`, `Kfrozen` | `fraction`, `fraction`, `m^3 m^-3`, `m^3 m^-3`, `mm h^-1` | Frozen-soil conductivity adjustment and resulting frozen conductivity from Eq. [7.9.20]-[7.9.22]. | soil-winter coupling pathway | infiltration/percolation frozen-layer branch |
| `Kib`, `Kiadj` | `kg s m^-4`, `kg s m^-4` | Baseline and adjusted interrill erodibility from Eq. [7.10.1]-[7.10.15]. | soil erodibility pathway | hillslope erosion interrill delivery |
| `Krb`, `Kradj` | `s m^-1`, `s m^-1` | Baseline and adjusted rill erodibility from Eq. [7.11.1]-[7.11.11]. | soil erodibility pathway | hillslope erosion rill detachment |
| `τcb`, `τcadj` | `Pa`, `Pa` | Baseline and adjusted critical hydraulic shear from Eq. [7.11.2], [7.11.4], [7.11.12]-[7.11.16]. | soil erodibility pathway | hillslope erosion detachment threshold |
| `Ψsurf`, `cycles` | `kPa`, `count` | Surface-soil matric potential and freeze-thaw cycle count controlling freeze-thaw adjustment factors. | snow/freeze + soil coupling pathway | erodibility and conductivity adjustments |
| `rr`, `So`, `Sd` | `m`, `m m^-1`, `m` | Roughness/slope/depression-storage relation consumed by runoff partition Eq. [4.3.4]. | soil + topography boundary | runoff partition depression-storage branch |
| `Kiadj`, `Kradj`, `τcadj`, `Di`, `Df` | mixed | Soil erodibility outputs entering erosion continuity and detachment equations (Eq. [11.2.1]-[11.2.3], [11.3.10]). | soil boundary publisher | erosion continuity routines |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SOIL-001 | Random-roughness update invariant: post-tillage and post-rainfall roughness updates must follow Eq. [7.5.1]-[7.5.3], with explicit branch inputs (`Tds`, `Rc`, `Cbr`) and non-negative emitted roughness states. | hard-fail | REF-SOIL-CH7-RR, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-002 | Ridge-state invariant: ridge-height updates follow Eq. [7.6.1], and when a ridge-furrow system is classified by §7.6 criteria (`RH after tillage >= 0.1 m` and `0.6 <= RINT <= 1.4 m`), emitted `RHt` cannot decay below `0.1 m`. | hard-fail | REF-SOIL-CH7-RH, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-003 | Bulk-density sequencing invariant: tillage, rainfall-consolidation, and weathering-consolidation pathways must follow Eq. [7.7.1]-[7.7.14] with explicit ordering and finite density states; negative or non-finite densities are invalid. | hard-fail | REF-SOIL-CH7-BD, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-004 | Porosity-domain invariant: porosity relations from Eq. [7.8.1]-[7.8.5] must preserve bounded fraction domains (`0 <= φt, Fa, Fcf, φe <= 1` within tolerance) and explicit dependence on bulk-density/coarse-fragment terms. | hard-fail | REF-SOIL-CH7-POR, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-005 | Conductivity-mode invariant: conductivity mode selection (`Kb`-adjusted vs constant `Kec`) must be explicit and consistent with the declared mode flag semantics in §7.9; mixed/implicit mode use is invalid. | hard-fail | REF-SOIL-CH7-KE | `[DIRECT][Static]` |
| INV-SOIL-006 | Frozen-soil conductivity invariant: frozen-layer conductivity updates follow Eq. [7.9.20]-[7.9.22], including `FSa` cap rule when `Fθ >= 100`; frozen conductivity cannot exceed unfrozen conductivity for the same layer state. | hard-fail | REF-SOIL-CH7-KE, REF-SOIL-CH7-FT, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-007 | Interrill-erodibility invariant: interrill baseline/adjusted erodibility relations (Eq. [7.10.1]-[7.10.15]) must be applied with explicit adjustment-factor composition and bounded, non-negative outputs within declared cropland/rangeland limit policy. | hard-fail | REF-SOIL-CH7-KI, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-008 | Rill-erodibility/shear invariant: rill erodibility and critical shear updates (Eq. [7.11.1]-[7.11.18]) must preserve valid threshold semantics (`τcadj > 0`) and explicit factorized update branches; undefined or negative threshold outputs are invalid. | hard-fail | REF-SOIL-CH7-KRTAU, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-009 | Runoff-coupling invariant: roughness and conductivity boundary payloads consumed by runoff partition (`rr`, `Ke`, and derived `Sd` pathway) must remain unit-consistent with Chapter-4 equations and branch semantics. | hard-fail | REF-SOIL-CH4-DEPSTOR, REF-SOIL-CH7-RR, REF-SOIL-CH7-KE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-010 | Erosion-coupling invariant: soil erodibility/shear boundary payloads (`Kiadj`, `Kradj`, `τcadj`) must be emitted with units/sign conventions compatible with Chapter-11 continuity/detachment equations. | hard-fail | REF-SOIL-CH11-ERODE, REF-SOIL-CH7-KI, REF-SOIL-CH7-KRTAU | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-011 | Update-order invariant: daily soil-state updates must retain explicit ordering across disturbance, consolidation/weathering, and freeze-thaw adjustments; no silent reordering is permitted. | hard-fail | REF-SOIL-CH7-INTRO, REF-SOIL-CH7-BD, REF-SOIL-CH7-KI, REF-SOIL-CH7-KRTAU | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-012 | Governance-range invariant: when empirical equations are used outside cited calibration ranges or suggested limits, outputs are non-promotable unless explicitly labeled and dispositioned with risk rationale. | governance-fail | REF-SOIL-CH7-KI, REF-SOIL-CH7-KRTAU, REF-SOIL-CH7-KE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SOIL-013 | SIMIMPL21 soil-water alias-lineage invariant: ET/soil-water closure surfaces must preserve deterministic alias continuity from layer storage (`st(i)` / `Θi`) to aggregate publication lineage (`watcon`, `Total-Soil`, `SoilWaterTotal`) without projection-side surrogate reconstruction. | hard-fail | REF-SOIL-LEGACY-WB11, REF-SOIL-CH5-PERC, REF-SOIL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SOIL-001` | runtime | Roughness state updater | Typed hard error on invalid roughness branch/domain state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-002` | runtime | Ridge-state updater | Typed hard error when ridge-floor rule or decay branch is violated | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-003` | runtime | Bulk-density update sequencer | Typed hard error on non-finite/negative density or invalid update ordering | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-004` | runtime | Porosity calculator | Typed hard error on out-of-domain porosity fractions or malformed adjustment terms | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-005` | runtime | Conductivity mode selector/validator | Typed hard error on inconsistent `Kb`/`Kec` mode semantics | Tier-A gate | `[DIRECT][Static]` |
| `INV-SOIL-006` | runtime | Frozen-soil conductivity branch | Typed hard error on invalid freeze-adjustment domain or amplified frozen conductivity | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-007` | runtime | Interrill erodibility updater | Typed hard error on invalid adjustment-factor composition or out-of-domain erodibility | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-008` | runtime | Rill erodibility/critical shear updater | Typed hard error on invalid threshold/erodibility state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-009` | runtime | Soil-to-runoff boundary payload validator | Typed hard error on units/sign/missing-field mismatch for runoff consumers | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-010` | runtime | Soil-to-erosion boundary payload validator | Typed hard error on malformed erodibility/shear payload semantics | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-011` | runtime | Cross-driver soil update-order validator | Typed hard error on silent branch reordering across daily update drivers | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-012` | governance | Review/disposition/promotion checklist | Promotion `HOLD` when range-exceedance labels/rationale are missing | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SOIL-013` | runtime + governance | Soil layer-to-aggregate alias-lineage validator for ET/soil-water publication surfaces | Typed hard error / explicit `HOLD` when layer storage aliases and aggregate lineage cannot be traced from runtime-owned state | SIMIMPL soil-water lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract use Chapter-7 and Chapter-11 WEPP notation.
Concrete openWEPP runtime-field names are not fixed yet, so identity aliases are
required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `RRi`, `RRt`, `RRo`, `Tds`, `Rc`, `Cbr` | identity names | roughness-update state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `RHt`, `RHo`, `RINT` | identity names | ridge-state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `ρt`, `ρc`, `Δρrf`, `Δρwt`, `daycnt` | identity names | bulk-density state and consolidation surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `φt`, `Fa`, `Fcf`, `φe` | identity names | porosity and storage-parameter surfaces | fraction semantics preserved | `[DIRECT][Static]` |
| `Kb`, `Kec`, `Ke` | identity names | effective-conductivity mode surfaces | `mm h^-1` preserved | `[DIRECT][Static]` |
| `Ksi`, `Ksai`, `Bi` | identity names | per-layer conductivity surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `st(i)` / `Θi` | `wb18_perc_theta_####` | ET/soil-water layer-storage coupling surface | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `watcon` lineage | `wb11_soil_water` -> `Total-Soil` / `SoilWaterTotal` | aggregate soil-water publication lineage surface | runtime `m` -> publication `mm` with declared conversion | `[DIRECT][Static] + [INFERENCE][Static]` |
| `FSa`, `Fθ`, `θf`, `θfc`, `Kfrozen` | identity names | frozen-soil conductivity adjustment surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Kib`, `Kiadj`, `Krb`, `Kradj` | identity names | erodibility state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `τcb`, `τcadj` | identity names | critical-shear threshold baseline/adjusted surfaces | `Pa` preserved | `[DIRECT][Static]` |
| `Ψsurf`, `cycles` | identity names | freeze-thaw control surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `rr`, `So`, `Sd` | identity names | runoff-coupling roughness/depression-storage surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Di`, `Df` | identity names | erosion continuity coupling surfaces | chapter-declared units preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-disturbance day | `Tds = 0` and roughness/bulk-density updates proceed through non-tillage branches only. | Disturbance term explicitly scales tillage effects in Eq. [7.5.1] and Eq. [7.7.1]. | `[DIRECT][Static]` |
| Fully satisfied freeze-cycle cap state | `cycles > 10` and freeze-thaw cycle factor uses capped constant value (`acyc = 1.31`). | Explicit cycle-cap behavior in Eq. [7.10.12] text. | `[DIRECT][Static]` |
| Frozen-layer partial-thickness state | Only a fraction of layer thickness is frozen, and weighted frozen/unfrozen conductivity is used. | Explicit weighted-thickness conductivity rule in §7.9.7. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Zero-rill-detachment threshold crossing | `τf <= τc`, giving zero rill detachment despite nonzero runoff. | Explicit threshold behavior in Eq. [11.2.3] narrative. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Low-event depression retention state | Event rainfall excess does not exceed `Sd`, yielding no routed runoff while storage infiltrates. | Explicit depression-storage condition in Eq. [4.3.3] and related text. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Negative roughness/ridge-height or non-finite roughness/bulk-density states emitted by soil updates. `[DIRECT][Static] + [INFERENCE][Static]`
- Out-of-domain porosity fractions (`φt`, `Fa`, `Fcf`, `φe`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Mixed or contradictory conductivity mode semantics (`Kb`-adjusted and constant `Kec` behavior simultaneously active). `[DIRECT][Static] + [INFERENCE][Static]`
- Frozen conductivity branch that increases conductivity above unfrozen value for the same layer state. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative or non-finite adjusted erodibility/shear outputs used in erosion coupling (`Kiadj`, `Kradj`, `τcadj`). `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required soil boundary fields for runoff or erosion consumers. `[DIRECT][Static] + [INFERENCE][Static]`
- Silent reordering of disturbance/consolidation/freeze-thaw update paths. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SOIL-P-001: Emit soil-state and erodibility surfaces using canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-P-002: Apply Eq. [7.5.*], [7.6.*], [7.7.*], [7.9.*], [7.10.*], and [7.11.*] branch logic explicitly; no implicit fallbacks. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-P-003: Propagate invariant failures as typed errors; do not silently clamp materially invalid states. `[INFERENCE][Static]`
- OBL-SOIL-P-004: Publish coupling-ready payloads for runoff/percolation/erosion domains with explicit unit/sign semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-P-005: Preserve ET/soil-water alias-lineage surfaces (`st(i)`/`Θi` to `watcon` to WB13 aggregates) so downstream publication closure checks remain layer-authoritative. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SOIL-C-001: Runoff-partition consumers must treat roughness/conductivity surfaces consistently with Chapter-4 depression-storage and infiltration equations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-C-002: Percolation consumers must enforce Chapter-5 conductivity semantics (`Ksi`, `Ksai`) and reject malformed soil-conductivity states. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-C-003: Erosion consumers must reject invalid erodibility/shear payloads and preserve Chapter-11 sign/unit conventions. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-C-004: Snow/freeze and management consumers must preserve freeze-thaw and disturbance signals needed by soil adjustment branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SOIL-C-005: Hydrology/publication consumers must reject aggregate soil-water surfaces that are not traceable to runtime layer-storage lineage. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Roughness/ridge update semantics (`INV-SOIL-001/002`) | soil microrelief update stage | Hard error on branch/order/domain failure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Bulk-density/porosity semantics (`INV-SOIL-003/004`) | soil physical-state update stage | Hard error on invalid density/porosity states | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Conductivity and frozen-layer semantics (`INV-SOIL-005/006`) | soil conductivity pathway | Hard error on mode/domain/branch violations | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Erodibility/shear semantics (`INV-SOIL-007/008`) | erodibility updater stage | Hard error on invalid erodibility/shear outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Cross-domain payload completeness (`INV-SOIL-009/010/011`) | soil boundary publish stage | Hard error on malformed or incomplete coupling payloads | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Empirical-range governance labeling (`INV-SOIL-012`) | review/verification/promotion | Governance `HOLD` until range-exceedance handling is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL21 soil-water alias-lineage closure (`INV-SOIL-013`) | ET/soil-water boundary publication stage | Hard error / `HOLD` when `st(i)`/`Θi` to aggregate publication lineage is incomplete or synthetic | SIMIMPL soil-water lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). `[DIRECT][Static]`

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-SOIL-001 | Roughness/ridge non-negativity comparator tolerance | lower bound `>= -1e-12 m` | Runtime hard-fails on material negatives; tolerance only for floating-noise interpretation. | `[INFERENCE][Static]` |
| TOL-SOIL-002 | Porosity bounds tolerance (`φt`, `Fa`, `Fcf`, `φe`) | `-1e-12 <= value <= 1 + 1e-12` | Maintains bounded fraction semantics while allowing tiny floating deviations. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SOIL-003 | Conductivity non-negative tolerance (`Ke`, `Ksi`, `Ksai`, `Kfrozen`) | lower bound `>= -1e-12` in declared units | Negative values beyond tolerance are invariant failures. | `[INFERENCE][Static]` |
| TOL-SOIL-004 | Critical shear positivity tolerance | `τcadj >= 1e-12 Pa` | Prevents undefined detachment-threshold branch behavior near zero. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SOIL-005 | Suggested limit enforcement tolerance for cropland/rangeland `Ki`, `Kr`, `τc` | `<= 1e-9` relative tolerance at policy boundaries | Used for governance/range-label checks, not for replacing model equations. | `[DIRECT][Static] + [INFERENCE][Static]` |

## CLIM06 Frost-State Conductivity Coupling Addendum

### CLIM06 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.ksnowf`, `frost.options.kresf`, `frost.options.ksoilf`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Soil conductivity surfaces | `Ke`, `Ksi`, `Ksai` |
| Frozen-state outputs | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |

### CLIM06 Deterministic Coupling Rules

1. Parsed frost controls are immutable runtime inputs; soil/runtime kernels must
   not rewrite `frost.options.*` values in place.
2. Active CLIM06 frozen-soil coupling (`frost_file_present = 1` and
   `wintRed = 1`) constrains effective infiltration conductivity by bounded
   frozen-state surfaces.
3. CLIM06 frozen-state outputs are bounded and non-negative:
   - `0 <= Dfrost <= 0.20 m`
   - `0 <= Dthaw <= 0.20 m`
   - `Nft >= 0`
   - `Ws_frz >= 0`
   - `0 <= InfCap_frz <= Ke`
4. Missing/non-finite/out-of-domain active-coupling frost symbols are typed
   hard-fail states; no silent defaults/clamping are allowed at this boundary.

### CLIM06 Contract-Test Vectors

1. Active CLIM06 vector publishes bounded frozen-state outputs and
   non-amplifying `InfCap_frz` coupling.
2. Missing active-coupling frost symbol hard-fails with typed missing-input
   posture.
3. Non-finite active-coupling frost symbol hard-fails with typed non-finite
   posture.
4. Out-of-domain active-coupling frost symbol/state hard-fails with typed
   domain posture.

## SIMIMPL21 WB11 Soil-Water Alias-Lineage Addendum

1. Soil-layer storage alias continuity for ET/soil-water closure is explicit:
   canonical `st(i)`/`Θi` layer states map to WB18 runtime layer-storage
   surfaces (`wb18_perc_theta_####`) consumed by ET extraction and aggregate
   lineage checks.
2. Aggregate soil-water publication lineage is explicit and layered:
   `st(i)` -> `soilw(i)` -> `watcon` -> WB13 aggregate fields
   (`Total-Soil`, `SoilWaterTotal`).
3. Contract-derived tests in SIMIMPL22 must fail closed when aggregate
   publications cannot be traced to runtime-owned layer storage surfaces.
4. This addendum closes contract-authority ambiguity for ET/soil-water alias
   lineage without asserting production implementation completion.

## HPARITY02 WB13 Profile-Capacity Seed Projection Addendum

1. Soil runtime adapters may publish WB13 profile-capacity seed symbols:
   `wb13_profile_depth_mm`, `wb13_profile_porosity_cap_mm`,
   `wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`.
2. These symbols must be derived from baseline-authoritative preprocessing
   lineage (input-layer depth normalization plus `scon.for` porosity/theta
   correction families), not synthesized publication-time placeholder formulas.
3. For `solwpv >= 7778` soils with measured field-capacity/wilting fields,
   authoritative WB13 profile-capacity seeds consume measured
   `fc`/`wp` lineage (`thetf2`/`thetd2`) with baseline correction/guard
   posture.
4. Missing/non-finite/domain-invalid seed derivations remain fail-closed and do
   not authorize silent substitution from unrelated publication surfaces.
5. HPHYS0202 narrows publication authority so WB13 profile-storage publication
   is runtime-owned and must not be synthesized from placeholder formulas.
6. HPHYS0205 requires authoritative runtime `thetfc_####`/`thetdr_####`
   symbols to carry the same baseline-corrected lineage family used by
   profile-capacity correction (`scon`-equivalent moisture corrections), not
   raw parser theta values when corrected lineage is available.
7. HPHYS0206 requires those authoritative `thetfc_####`/`thetdr_####` symbols
   to be projected from the same baseline-normalized corrected-layer set used
   by profile-capacity lineage (`wb13_profile_depth_mm`,
   `wb13_profile_porosity_cap_mm`) before OFE layer publication mapping.
8. HPHYS0206 prohibits raw-theta fallback for authoritative FC/WP layer
   publication when normalized corrected-lineage projection is required; missing
   normalized lineage or mapping closure is a typed fail-closed runtime
   boundary condition.
9. HPHYS0207 closes depth-authority mismatch by making
   `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` runtime-owned,
   baseline-corrected, normalized-profile storage authorities aligned with
   `wb13_profile_depth_mm`/`wb13_profile_porosity_cap_mm` for WB13 publication.
10. HPHYS0207 requires explicit normalized-tail handling policy: FC/WP
   publication authority is normalized-profile storage projection, so residual
   normalized depth beyond OFE layer publication depth must be consumed into
   `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` (no silent tail
   truncation and no parser-theta fallback authority repair).

## HPHYS0203 Soil-Water Robustness Validation Addendum

1. Contract-derived robustness vectors for soil-water publication lineage must
   include deterministic checks for:
   - `Total-Soil` publication from runtime-owned `wb11_soil_water` lineage,
   - aggregate closure `SoilWaterTotal = Total-Soil + frozwt`,
   - non-negative and finite publication-domain behavior for
     `Total-Soil`/`SoilWaterTotal`.
2. Robustness vectors must include deterministic perturbation checks that
   preserve aggregate closure and ordering continuity when bounded
   storage-domain inputs are perturbed.
3. Missing/non-finite/domain-invalid storage lineage symbols remain typed
   fail-closed boundary states and are not eligible for projection-side
   surrogate reconstruction.

## HPHYS0208 WB11 Seed Threshold-Lineage Projection Addendum

1. Soil runtime projection must publish canonical WB11 seed threshold-lineage
   symbols for each emitted layer:
   - `por_####` from normalized corrected-layer porosity authority,
   - `cpm_####` from normalized corrected-layer coarse-fragment correction
     authority,
   - `thetfc_####`, `thetdr_####`, and `dg_####` from the same
     normalized corrected-layer mapping authority,
   - profile saturation control `sat`.
2. `por_####`/`cpm_####` authority must share deterministic normalized-layer
   overlap mapping with `thetfc_####`/`thetdr_####`; mixed raw/corrected source
   publication is invalid.
3. Missing/non-finite/domain-invalid threshold-lineage projection symbols are
   fail-closed runtime-boundary states and must not be repaired by surrogate
   FC/WP seed reconstruction.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SOIL-001 | Per-invariant comparator vectors for soil-state transitions and erodibility adjustment factors are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SOIL-002 | SIMIMPL21 closes ET/soil-water alias-lineage authority, but full soil-domain runtime alias finalization (including non-hydrology consumers) remains incomplete. | ET/soil-water contract authority is explicit; broader soil-boundary alias harmonization remains open. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SOIL-003 | Companion contracts for residue-management and hillslope sediment-domain internals are incomplete, so some coupled obligations remain provisional. | Full cross-domain closure semantics remain partially provisional. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SOIL-004 | Chapter-7 empirical equations include documented calibration ranges and suggested limits; enforcement strategy for out-of-range operational runs is not yet wired to comparator policy artifacts. | Promotion-risk labeling is available, but implementation-level policy automation is incomplete. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-10 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-7 soil authority anchors, coupling invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-10 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence-mode token casing, strengthened freeze-thaw anchor specificity/path consistency, added `τcadj` alias coverage, and evidence-tagged all degenerate-state claims. |
| `2026-05-23` | `3` | `Codex` | CLIM06 amendment: added frozen-soil conductivity coupling authority from parsed frost controls, bounded `Dfrost/Dthaw/Nft/Ws_frz/InfCap_frz` runtime-state requirements, and typed active-coupling guard posture. |
| `2026-05-25` | `4` | `Codex` | SIMIMPL21 amendment: added WB11 ET/soil-water alias-lineage authority (`INV-SOIL-013`) with explicit layer-storage to aggregate publication mapping and downstream SIMIMPL22 gating obligations. |
| `2026-05-29` | `5` | `Codex` | HPARITY02 amendment: added WB13 profile-capacity seed-projection authority (`wb13_profile_*_mm`) anchored to baseline preprocessing/correction lineage and fail-closed derivation posture. |
| `2026-05-29` | `6` | `Codex` | HPHYS0202 amendment: clarified WB13 publication authority split where FC/WP adapter seeds remain diagnostic carry surfaces and canonical `ProfileFCStore`/`ProfileWPStore` publication must use layer-authoritative runtime aggregation. |
| `2026-05-29` | `7` | `Codex` | HPHYS0205 amendment: bound authoritative runtime `thetfc_####`/`thetdr_####` symbols to baseline-corrected moisture lineage (no raw-theta authority when corrected lineage is available). |
| `2026-05-30` | `8` | `Codex` | HPHYS0206 amendment: required authoritative FC/WP layer symbols to originate from baseline-normalized corrected-layer lineage with deterministic publication mapping and typed fail-closed posture when normalized correction lineage is unavailable. |
| `2026-05-30` | `9` | `Codex` | HPHYS0207 amendment: ratified WB13 FC/WP depth-authority alignment to normalized-profile runtime storage symbols (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`) and added explicit normalized-tail consumption policy authority. |
| `2026-05-30` | `10` | `Codex` | HPHYS0203 amendment: added soil-water robustness validation obligations for `Total-Soil`/`SoilWaterTotal` lineage, deterministic closure-preserving perturbation vectors, and explicit fail-closed non-finite/domain posture requirements. |
| `2026-05-30` | `11` | `Codex` | HPHYS0208 amendment: required soil runtime projection to publish coupled WB11 threshold-lineage symbols (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`) from shared normalized corrected-layer authority with explicit fail-closed posture. |
