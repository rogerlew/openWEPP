---
contract_id: SC-PLANT-001
title: Plant Growth Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Plant state evolution for cropland and rangeland growth submodels
  - Plant to water-balance coupling surfaces (LAI, root depth, plant biomass/residue descriptors)
  - Plant to erosion and residue-component coupling surfaces
consumer_scope:
  - Water balance and evapotranspiration surfaces consuming plant state descriptors
  - Erosion surfaces consuming canopy and cover descriptors
  - Residue decomposition and management surfaces consuming plant-to-residue transfers
evidence_level: static
last_reviewed: 2026-05-20
supersedes: []
superseded_by: []
---

# SC-PLANT-001 Plant Growth Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for plant growth behavior and plant-driven
coupling surfaces used by openWEPP hydrology, erosion, and residue domains.

## Scientific Scope

In scope:
- Cropland and rangeland plant-state evolution used by WEPP plant growth
  component semantics.
- Plant biomass, canopy, root, and yield state/flux invariants.
- Required producer/consumer boundaries between plant growth and Chapter 5
  (water balance), Chapter 9 (residue), and Chapter 11 (erosion) domains.

Out of scope:
- Kernel implementation details and data-structure layout.
- Nutrient, pest, and aeration stress process modeling not implemented by WEPP.
- Non-plant domains except boundary definitions required for coupling safety.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-PLANT-CH8-INTRO | `references/50201000/chap8.pdf` §8.1 | Declares plant outputs and cross-domain coupling to Chapters 5, 9, 11. | `[DIRECT][Static]` |
| REF-PLANT-CH8-PHENO | `chap8.pdf` §8.2, Eq. [8.2.1]-[8.2.2] | Cropland heat-unit and maturity-index semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-GROWTH | `chap8.pdf` §8.2.1, Eq. [8.2.3]-[8.2.5] | Potential biomass and daily biomass accumulation semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-STRESS | `chap8.pdf` §8.2.4, Eq. [8.2.14]-[8.2.16] | Water/temperature stress boundedness and growth regulation. | `[DIRECT][Static]` |
| REF-PLANT-CH8-SENESCENCE | `chap8.pdf` §8.2.3, Eq. [8.2.9]-[8.2.13] | Canopy decline and live-biomass to flat-residue transfer semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-ROOT | `chap8.pdf` §8.2.7, Eq. [8.2.20]-[8.2.25] | Root biomass partitioning and root-depth upper bounds. | `[DIRECT][Static]` |
| REF-PLANT-CH8-MGMT | `chap8.pdf` §8.3-§8.5 | Management conversion/removal constraints (harvest, grazing, dormancy, burning). | `[DIRECT][Static]` |
| REF-PLANT-CH8-RANGE | `chap8.pdf` §8.4-§8.5 | Rangeland growth-curve (`gi`) and dormancy/stress transfer semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH5-COUPLING | `references/50201000/chap5.pdf` §5.5 | ET/water-balance receives daily LAI, root depth, biomass, residue cover; returns plant water-stress factor. | `[DIRECT][Static]` |
| REF-PLANT-CH9-COUPLING | `references/50201000/chap9.pdf` §9.2, §9.4 | Residue domain consumes standing/flat/root biomass transfers and management outcomes. | `[DIRECT][Static]` |
| REF-PLANT-CH11-COUPLING | `references/50201000/chap11.pdf` §11.6 | Erosion adjustments depend on canopy/surface cover and residue surfaces from plant/residue routines. | `[DIRECT][Static]` |
| REF-PLANT-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative mass/depth and bounded fractions are required for physically valid state. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `HU` | `degC day` | Daily heat-unit increment. | plant growth | phenology logic |
| `HUI` | `fraction` | Heat-unit index (`0` at planting, `1` at physiological maturity). | plant growth | growth/senescence/yield logic |
| `Bm` | `kg m^-2` | Live above-ground biomass. | plant growth | yield, senescence, management, ET coupling |
| `Brt` | `kg m^-2` | Total live root biomass. | plant growth | root partitioning, coupling checks |
| `Rd` | `m` | Root depth. | plant growth | ET root-zone distribution (Chapter 5) |
| `LAI` | `m^2 m^-2` | Leaf area index. | plant growth | ET and interception components |
| `Cc` | `fraction` | Canopy cover (`0..1`). | plant growth | erosion and interception coupling |
| `Hc` | `m` | Canopy height. | plant growth | erosion/interception coupling |
| `YLD` | `kg m^-2` | Economic yield. | plant growth | output/reporting, management evaluation |
| `Mf` | `kg m^-2` | Flat residue mass. | plant growth / residue mgmt | residue decomposition and erosion cover |
| `Ms` | `kg m^-2` | Standing residue/standing dead mass. | plant growth / residue mgmt | residue decomposition and cover |
| `WS` | `fraction` | Water-stress factor (`0..1`). | ET/water-balance coupling | plant growth regulation |
| `TS` | `fraction` | Temperature-stress factor (`0..1`). | plant growth | plant growth regulation |
| `REG` | `fraction` | Growth regulation factor `min(WS, TS)`. | plant growth | daily biomass update |
| `EP` / `Etp` | `m d^-1` | Potential plant transpiration demand surfaces. | ET component | plant stress/uptake logic |
| `u_l` | `mm` | Layer water use for plant uptake by soil layer `l`. | ET component | water-stress computation |
| `DeltaBp`, `DeltaBi` | `kg m^-2 d^-1` | Potential and stress-adjusted daily biomass increment. | plant growth | daily biomass update invariants |
| `Rdx` | `m` | Crop/community maximum root depth parameter. | plant parameterization | root-depth envelope invariant |
| `CRITVM` | `kg m^-2` | Critical lower biomass floor under heavy grazing (where defined). | plant parameterization | grazing management invariant |
| `gi` | `fraction` | Rangeland growth-curve increment (`0..1` progression). | rangeland growth submodel | rangeland growth/dormancy gating |
| `RGCMIN` | `fraction` | Minimum live-biomass growth-curve floor for evergreen behavior. | rangeland growth submodel | allowed evergreen degenerate behavior |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-PLANT-001 | State-domain boundedness: `Bm >= 0`, `Brt >= 0`, `Mf >= 0`, `Ms >= 0`, `Rd >= 0`, `Hc >= 0`, `LAI >= 0`, `YLD >= 0`, and `0 <= Cc <= 1`. | hard-fail | REF-PLANT-CH8-INTRO, REF-PLANT-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-002 | Cropland phenology bounds and gating: for cropland submodel paths only, `0 <= HUI <= 1`; annual growth does not proceed at/under base temperature and stops when maturity (`HUI = 1`) is reached. | hard-fail | REF-PLANT-CH8-PHENO, REF-PLANT-CH8-MGMT (model summary) | `[DIRECT][Static]` |
| INV-PLANT-003 | Stress boundedness: `0 <= WS <= 1`, `0 <= TS <= 1`, and `REG = min(WS, TS)` with `0 <= REG <= 1`; adjusted biomass update must use `DeltaBi = DeltaBp * REG`. | hard-fail | REF-PLANT-CH8-STRESS | `[DIRECT][Static]` |
| INV-PLANT-004 | Senescence transfer closure: daily reduction in live above-ground biomass attributable to senescence is added to flat residue mass in the same step (signed conservation for the transfer pair). | hard-fail | REF-PLANT-CH8-SENESCENCE, REF-PLANT-CH9-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-005 | Root-depth envelope: computed root depth cannot exceed crop maximum root depth (`Rdx`) or configured maximum soil depth; root-mass partitioning follows declared depth-zone logic. | hard-fail | REF-PLANT-CH8-ROOT | `[DIRECT][Static]` |
| INV-PLANT-006 | Management-removal bound: harvest, grazing, herbicide, and burn operations may convert/remove biomass only from available pools; no operation may produce negative residual pool mass; grazing floor (`CRITVM`) is respected where defined. | hard-fail | REF-PLANT-CH8-MGMT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-007 | Coupling payload completeness: plant component emits required state surfaces to water-balance/ET (`LAI`, `Rd`, biomass/residue descriptors, stress linkage), erosion (canopy cover/height and cover context), and residue components (senescence/management transfers). | hard-fail | REF-PLANT-CH8-INTRO, REF-PLANT-CH5-COUPLING, REF-PLANT-CH9-COUPLING, REF-PLANT-CH11-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-008 | Rangeland stress-transfer caps: drought-stress-driven daily conversion from standing live to standing dead is bounded by chapter-defined daily limits (3% for old standing live; 5% for old standing dead transfer/depletion constraint). | hard-fail | REF-PLANT-CH8-RANGE | `[DIRECT][Static]` |
| INV-PLANT-009 | Explicit model-limit invariant: nutrient/pest/aeration stress is not natively simulated by plant routines; any such effects must be represented through explicit parameterization/inputs, not hidden default factors. | governance-fail | REF-PLANT-CH8-MGMT (yield-adjustment and model-summary limitations) | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-010 | Rangeland growth-curve bounds and gating: for rangeland submodel paths, `0 <= gi <= 1`; growth period initiation occurs when `gi > 0.001`, and growth for the period stops once `gi` reaches `1.0`. | hard-fail | REF-PLANT-CH8-RANGE | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Bare/fallow surface | `Bm = 0`, `Brt = 0`, `Cc = 0`, `Hc = 0`, `LAI = 0` | No active crop is a valid simulation state. |
| Dormant perennial | Above-ground live state near zero while root state remains positive | Chapter 8 dormancy transitions permit this behavior. |
| Senescence completion | Live biomass reduced while flat/standing residue pools increase | Expected transfer from plant to residue domain. |
| Full water stress day | `WS = 0` and `REG = 0` | Growth can halt under severe water stress without violating physics. |
| Evergreen floor behavior | `gi` lower-bounded by `RGCMIN` for evergreen communities | Chapter 8 rangeland formulation permits non-zero baseline live biomass. |

## Invalid States

- Any negative biomass or residue mass (`Bm`, `Brt`, `Mf`, `Ms`) beyond numeric tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- `Cc`, `WS`, `TS`, `REG`, `HUI`, or `gi` outside `[0,1]` on applicable submodel paths. `[DIRECT][Static] + [INFERENCE][Static]`
- `Rd` greater than `min(Rdx, configured max soil depth)`. `[DIRECT][Static]`
- Management conversion/removal larger than available biomass pool. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required plant coupling outputs at daily boundary handoff. `[DIRECT][Static]`
- Hidden nutrient/pest/aeration stress multiplier not declared in inputs/contract. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-PLANT-P-001: Emit daily plant state surfaces (`Bm`, `Brt`, `Rd`, `LAI`, `Cc`, `Hc`, residue-transfer quantities) with declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-002: Enforce all `INV-PLANT-*` bounds before publishing boundary payloads. `[INFERENCE][Static]`
- OBL-PLANT-P-003: Apply management events as explicit state transitions with non-negative residual pools. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-004: Surface typed boundary errors when invalid plant state or missing coupling payload occurs. `[INFERENCE][Static]`

## Consumer Obligations

- OBL-PLANT-C-001: Water-balance/ET consumer must treat plant-provided units exactly as declared and must return stress-linked surfaces consistently. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-002: Erosion consumer must not assume canopy/cover inputs outside declared domains and must fail explicitly on invalid payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-003: Residue consumer must preserve mass-accounting semantics for plant-to-residue transfers. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-004: All consumers must propagate invariant-violation context without silent clamping/defaulting. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| State-domain bounds (`INV-PLANT-001/002/003/005/010`) | plant daily update before publish | Hard error; boundary payload rejected; violation logged with invariant ID | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Transfer closure (`INV-PLANT-004`) | senescence/management transfer step | Hard error if closure residual exceeds tolerance; require fix before promotion | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Management removal bounds (`INV-PLANT-006/008`) | management event application | Hard error and event rejection on impossible removal/conversion | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling completeness (`INV-PLANT-007`) | plant->consumer handoff | Hard error on missing/invalid field; no fallback payload synthesis | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Model-limit governance (`INV-PLANT-009`) | review/verification and runtime config audit | Governance failure; requires explicit contract amendment before promotion | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific tolerances used for comparator interpretation:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-PLANT-001 | Senescence transfer closure residual: `abs((Bm(i-1)-Bm(i)) - (Mf(i)-Mf(i-1)))` | `<= 1e-10 kg m^-2` | Applies only to the Eq. [8.2.13] transfer pair. |
| TOL-PLANT-002 | Fraction-domain tolerance for `{Cc, WS, TS, REG, HUI, gi}` | `abs(bound violation) <= 1e-12` allowed for comparator noise only | Runtime must not silently clamp; typed error if materially out-of-domain. |
| TOL-PLANT-003 | Non-negative-domain tolerance for biomass/depth states | lower bound `>= -1e-12` for comparator interpretation | Runtime violation remains explicit when negative beyond tolerance. |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-PLANT-001 | Per-equation comparator vectors for `INV-PLANT-*` are not yet curated in this package. | Limits immediate regression-gate automation for each invariant family. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PLANT-002 | Nutrient/pest/aeration coupling is outside current WEPP plant routines and remains parameterization-only. | Reduces causal fidelity for yield stress attribution without external calibration workflow. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PLANT-003 | Legacy routine provenance is mapped at domain level (`grow.for`, `growop.for`, `range.for`) but not yet per-invariant line anchor. | Traceability for implementation-level acceptance is partial. | promotable-with-risk | `[INFERENCE][Static]` |
| GAP-PLANT-004 | Boundary contract IDs for plant consumers (`SC-WATBAL-001`, `SC-RESIDUE-001`, `SC-SED-001`) are not yet fully authored. | Cross-contract closure is provisional until downstream contracts reach draft status. | non-promotable | `[DIRECT][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-02 package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with invariant set, boundary obligations, and citation anchors per SCI-02 kickoff prompt. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: scoped cropland/rangeland invariants, added missing symbols/anchors, added claim-level evidence tags, and labeled gap promotability. |
