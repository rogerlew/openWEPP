---
contract_id: SC-WATBAL-001
title: Water Balance Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Daily root-zone water balance accounting surfaces
  - Daily evapotranspiration distribution and percolation-routing accounting surfaces
  - Daily coupling surfaces linking climate/infiltration/runoff/snow state into water-balance closure
consumer_scope:
  - Plant growth stress and daily growth-regulation consumers
  - Runoff partition and infiltration antecedent-moisture consumers
  - Subsurface/lateral-flow and drainage consumers using daily loss-accounting surfaces
evidence_level: static
last_reviewed: 2026-05-20
supersedes: []
superseded_by: []
---

# SC-WATBAL-001 Water Balance Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for daily root-zone water-balance closure,
flux/state accounting, and required coupling boundaries for openWEPP hydrology.

## Scientific Scope

In scope:
- Daily water-balance closure at root-zone scale.
- Daily accounting for precipitation/snow contributions, interception, runoff,
  evapotranspiration, percolation, and subsurface/drain losses.
- Root-zone evapotranspiration distribution and percolation-routing invariants
  needed to preserve daily closure semantics.
- Producer/consumer boundary obligations between Chapter-5 water-balance
  surfaces and coupled climate, runoff, subsurface, and plant domains.

Out of scope:
- Kernel implementation details and Rust API layout.
- Event-scale infiltration/runoff partition internals owned by `SC-RUNOFFPART-001`.
- Detailed ET-physics option selection beyond the Chapter-5 closure/accounting
  requirements captured here.
- Watershed/channel routing and impoundment accounting surfaces.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-WATBAL-CH5-BAL | `references/50201000/chap5.pdf` §5.1, Eq. [5.1.1] | Daily continuous water-balance equation and required closure terms (`Θ`, `Θin`, `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd`). | `[DIRECT][Static]` |
| REF-WATBAL-CH5-INTERCEPT | `chap5.pdf` §5.1, Eq. [5.1.2] | Biomass-driven interception term semantics and sign/domain expectations. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-SNOW | `chap5.pdf` §5.1 text (rain/snow partition and melt treatment) | Snow accumulation/melt contribution sign convention in Eq. [5.1.1]. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-ETDIST | `chap5.pdf` §5.3, Eq. [5.3.1]-[5.3.4] | Soil-evaporation depth distribution, root-zone uptake distribution, and water-deficit adjustment semantics. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-PERC | `chap5.pdf` §5.4, Eq. [5.4.1]-[5.4.5] | Percolation eligibility at field capacity, conductivity adjustment, and lower-layer restriction semantics. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-LINK | `chap5.pdf` §5.5, Eq. [5.5.1] | Coupling with infiltration and plant growth (`Ws`) and daily closure linkage expectations. | `[DIRECT][Static]` |
| REF-WATBAL-CH3-COUPLING | `references/50201000/chap3.pdf` §3.1-§3.2 | Snowpack accumulation/melt state that contributes signed `S` terms in Chapter-5 closure. | `[DIRECT][Static]` |
| REF-WATBAL-CH4-COUPLING | `references/50201000/chap4.pdf` §4.2 and §4.5 | Infiltration/runoff outputs feeding daily `Q` and antecedent-moisture coupling with Chapter 5. | `[DIRECT][Static]` |
| REF-WATBAL-CH6-COUPLING | `references/50201000/chap6.pdf` §6.2.1, Eq. [6.2.1]-[6.2.5] | Daily subsurface lateral-flow/drainage terms consistent with Chapter-5 `Qd` accounting. | `[DIRECT][Static]` |
| REF-WATBAL-CH8-COUPLING | `references/50201000/chap8.pdf` §8.2.4 and §8.1 coupling text | Plant-water-stress consumption of `Ws` and required plant-surface inputs to water balance. | `[DIRECT][Static]` |
| REF-WATBAL-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative flux magnitudes and bounded stress factors required for physically valid accounting. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Θ` | `m` | Root-zone soil water content at day end/state point. | water-balance component | infiltration antecedent-state and reporting consumers |
| `Θin` | `m` | Initial root-zone soil water content for the accounting step. | water-balance component input state | daily closure computation |
| `P` | `m` | Daily precipitation contribution. | climate/winter coupling | water-balance closure |
| `I` | `m` | Daily interception by vegetation. | water-balance interception routine | water-balance closure |
| `S` | `m` | Snow-water contribution term (`+` melt, `-` accumulation). | winter hydrology coupling | water-balance closure |
| `Q` | `m` | Daily surface-runoff contribution. | runoff partition coupling | water-balance closure |
| `ET` | `m` | Daily evapotranspiration removal term. | ET distribution routine | water-balance closure |
| `D` | `m` | Daily percolation loss below root zone. | percolation routing routine | water-balance closure |
| `Qd` | `m` | Daily subsurface lateral flow or drain-tile loss term. | subsurface/drainage coupling | water-balance closure |
| `VE` | `kg m^-2` | Above-ground biomass used in interception relation. | plant growth coupling | interception calculation |
| `Es`, `Esb`, `Esp` | `m d^-1` | Actual, bare-soil, and potential soil evaporation rates. | ET distribution routine | ET and layer-water updates |
| `Etp` | `m d^-1` | Potential plant transpiration. | ET partition routine | root uptake distribution and stress factor |
| `UPi`, `Ui` | `m d^-1` | Potential and actual layer-wise plant water use for soil layer `i`. | ET/root-uptake routine | stress factor and layer updates |
| `dx`, `ds` | `m` | Max evaporation-influence depth and effective evaporated depth. | ET distribution routine | layer-water updates |
| `Θr` | `m^3 m^-3` | Residual soil moisture threshold for soil evaporation. | soil-state parameterization | ET reduction logic |
| `Θi` | `m` | Soil water content of layer `i`. | soil-layer state | ET and percolation routing |
| `Θc` | `m^3 m^-3` | Critical soil-water fraction for plant-stress response. | crop parameterization | layer uptake adjustment |
| `FCi` | `m` | Field-capacity water content for layer `i` (per Chapter-5 convention). | soil-layer parameterization | percolation eligibility |
| `ULi` | `m` | Upper-limit water content for layer `i`. | soil-layer parameterization | uptake/percolation limits |
| `pei` | `m d^-1` | Percolation rate through layer `i`. | percolation routine | lower-layer routing and `D` term assembly |
| `ti`, `Δt` | `s` | Travel time through layer `i` and travel interval. | percolation routine | percolation step update |
| `Ksi`, `Ksai`, `Bi` | `m s^-1`, `m s^-1`, `fraction` | Saturated and adjusted hydraulic conductivity with conductivity-shape parameter. | soil/percolation routine | percolation routing |
| `Ws` | `fraction` | Plant growth water-stress factor (`0..1`) from supply/demand ratio. | water-balance/ET coupling | plant growth regulation |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-WATBAL-001 | Daily closure invariant: for each daily step, root-zone accounting must satisfy Eq. [5.1.1] with explicit step residual calculation for `Θ = Θin + (P-I) ± S - Q - ET - D - Qd`; residual beyond tolerance is invalid. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-002 | Flux/storage domain invariant: `Θ`, `Θin`, `P`, `I`, `Q`, `ET`, `D`, and `Qd` are non-negative magnitudes; `S` sign is restricted to Chapter-5 convention (`+` melt, `-` accumulation). | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-003 | Interception invariant: interception is computed from Eq. [5.1.2] with `VE >= 0`, and `0 <= I <= P` for each daily step. | hard-fail | REF-WATBAL-CH5-INTERCEPT, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-004 | Plant-stress coupling invariant: `Ws = (Σ Ui)/Etp` per Eq. [5.5.1], with `0 <= Ws <= 1`, and layer-use adjustment obeys Eq. [5.3.4] (`0 <= Ui <= UPi`); for `Etp = 0` days, the zero-demand branch is explicit (`Σ Ui = 0` and `Ws = 1`). | hard-fail | REF-WATBAL-CH5-ETDIST, REF-WATBAL-CH5-LINK, REF-WATBAL-CH8-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-005 | Soil-evaporation-depth invariant: Eq. [5.3.1]-[5.3.2] constraints hold, including `0 <= ds <= dx` and no extraction below residual-moisture limits. | hard-fail | REF-WATBAL-CH5-ETDIST, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-006 | Percolation eligibility invariant: per-layer percolation obeys Eq. [5.4.1] (`pei = 0` when `Θi <= FCi`; otherwise `pei >= 0`), and adjusted conductivity/percolation semantics follow Eq. [5.4.2]-[5.4.5]. | hard-fail | REF-WATBAL-CH5-PERC | `[DIRECT][Static]` |
| INV-WATBAL-007 | Coupling invariant: daily runoff/infiltration (`Q`, antecedent near-surface moisture), snow contribution (`S`), subsurface/drain loss (`Qd`), and plant forcing inputs (`LAI`, root depth, biomass/residue context for ET partition) must be present with declared units before closure is accepted. | hard-fail | REF-WATBAL-CH3-COUPLING, REF-WATBAL-CH4-COUPLING, REF-WATBAL-CH5-LINK, REF-WATBAL-CH6-COUPLING, REF-WATBAL-CH8-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-008 | Governance invariant: water moving below root zone (`D`) is treated as loss from this component and cannot be silently reintroduced into root-zone closure without explicit cross-contract authority. | governance-fail | REF-WATBAL-CH5-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-WATBAL-001` | runtime | Daily closure assembler for Eq. [5.1.1] | Typed hard error on residual beyond tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-002` | runtime | Daily state/flux domain validator | Typed hard error on negative magnitudes or invalid `S` sign convention | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-003` | runtime | Interception calculation + post-calc bounds check | Typed hard error for invalid `VE` domain or `I` out of `[0,P]` | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-004` | runtime | ET distribution and stress-factor computation path | Typed hard error on invalid `Ws`, `Ui`, or `UPi` domain relation, or undefined denominator handling when `Etp = 0` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-005` | runtime | Soil-evaporation depth updater | Typed hard error on invalid `ds`/`dx` relation or residual-moisture violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-006` | runtime | Percolation routing by layer | Typed hard error on invalid percolation eligibility/conductivity domains | Tier-A gate | `[DIRECT][Static]` |
| `INV-WATBAL-007` | runtime | Component-boundary payload validator | Typed hard error on missing/invalid required coupling surfaces | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-008` | governance | Contract review/disposition/verification and promotion checklist | Promotion `HOLD` until cross-contract authority for reuse of `D` is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-5 WEPP notation and legacy
lineage names by default. Concrete openWEPP boundary/API names are not yet
fixed for this domain, so identity mapping is required until downstream
implementation contracts introduce explicit aliases.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Θ` | `Θ` (identity) | root-zone daily state surface | `m` -> `m` | `[DIRECT][Static]` |
| `Θin` | `Θin` (identity) | root-zone daily initial-state surface | `m` -> `m` | `[DIRECT][Static]` |
| `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd` | identity names | daily closure terms | `m` -> `m` | `[DIRECT][Static]` |
| `VE` | `VE` (identity) | interception input surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Es`, `Esb`, `Esp`, `Etp` | identity names | ET partition surfaces | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `UPi`, `Ui` | identity names | layer-wise uptake surfaces | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `dx`, `ds` | identity names | evaporation-depth surfaces | `m` -> `m` | `[DIRECT][Static]` |
| `Θr`, `Θi`, `FCi`, `ULi` | identity names | layer-state and thresholds | chapter-declared units preserved | `[DIRECT][Static]` |
| `Θc` | `Θc` (identity) | layer critical-water threshold surface | `m^3 m^-3` -> `m^3 m^-3` | `[DIRECT][Static]` |
| `pei`, `ti`, `Δt` | identity names | percolation routing surfaces | `m d^-1` and `s` preserved | `[DIRECT][Static]` |
| `Ksi`, `Ksai`, `Bi` | identity names | conductivity/routing parameter surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Ws` | `Ws` (identity) | plant-stress coupling surface | `fraction` -> `fraction` | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Dry day with depletion | `P = 0` and no snowmelt contribution, while `ET` and/or `D` reduce `Θ`. | Valid no-rain daily balance mode. |
| Snow-accumulation day | `S < 0` (accumulation), with rainfall partition and closure still maintained. | Chapter-5 explicitly allows signed snow-storage contribution. |
| Snowmelt-driven input day | `S > 0` contributes melt water as rainfall-equivalent for runoff/percolation accounting. | Chapter-5 text defines melt treatment in Eq. [5.1.1]. |
| Field-capacity-limited layer | `Θi <= FCi` causing `pei = 0` for the layer. | Explicit Eq. [5.4.1] branch behavior. |
| Water-stress day | `Ws` near `0` with limited `Ui` relative to `Etp`. | Valid stress response passed to plant-growth component. |
| Zero-demand transpiration day | `Etp = 0`, `Σ Ui = 0`, and `Ws = 1` by explicit branch semantics. | Prevents undefined ratio while preserving physically consistent no-demand behavior. |

## Invalid States

- Daily closure residual for Eq. [5.1.1] exceeding declared tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative flux/storage magnitudes for `Θ`, `Θin`, `P`, `I`, `Q`, `ET`, `D`, or `Qd` beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Interception outside `[0,P]` or interception computed from invalid biomass domain (`VE < 0`). `[DIRECT][Static] + [INFERENCE][Static]`
- Invalid layer-use relationship (`Ui > UPi` or negative `Ui`/`UPi`) or stress factor outside `[0,1]`. `[DIRECT][Static] + [INFERENCE][Static]`
- `Etp = 0` day with non-zero `Σ Ui`, or undefined/non-explicit `Ws` branch handling. `[DIRECT][Static] + [INFERENCE][Static]`
- Percolation emitted when `Θi <= FCi` or non-physical conductivity/percolation domain failure in Eq. [5.4.*]. `[DIRECT][Static]`
- Missing required coupling payloads from climate/runoff/subsurface/plant surfaces at daily closure assembly time. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-WATBAL-P-001: Emit daily closure terms (`Θ`, `Θin`, `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd`) with declared units and sign conventions. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-002: Compute and retain daily closure residual for Eq. [5.1.1] and fail explicitly on violation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-003: Enforce all runtime guard checks before publishing downstream daily boundary outputs. `[INFERENCE][Static]`
- OBL-WATBAL-P-004: Propagate invariant violations as typed errors; no silent clamping/defaulting of hydrologic terms. `[INFERENCE][Static]`

## Consumer Obligations

- OBL-WATBAL-C-001: Plant-growth consumers must use `Ws` only within declared domain and reject malformed stress payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-002: Infiltration/runoff consumers must treat `Θ`/near-surface moisture linkage in declared units without hidden reinterpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-003: Subsurface/drainage consumers must preserve `Qd` accounting semantics and avoid untracked reinjection into root-zone closure. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-004: All consumers must fail explicitly on invariant-violating payloads and propagate invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Daily closure and sign domains (`INV-WATBAL-001/002`) | daily balance assembly | Hard error; reject day-state publish; log invariant ID and residual | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Interception and ET/uptake domains (`INV-WATBAL-003/004/005`) | ET/interception update stage | Hard error on invalid domains or non-physical extraction | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Percolation routing (`INV-WATBAL-006`) | layer routing stage | Hard error on eligibility/domain failure; block downstream percolation publish | Tier-A gate | `[DIRECT][Static]` |
| Coupling completeness (`INV-WATBAL-007`) | daily boundary handoff | Hard error on missing malformed field/units mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Governance boundary (`INV-WATBAL-008`) | review/verification/promotion | Governance `HOLD` until cross-contract ownership of `D` reuse is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific tolerances used for comparator interpretation:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-WATBAL-001 | Daily closure residual for Eq. [5.1.1] | `<= 1e-9 m` | Residual is computed each daily step; runtime still hard-fails on material violation. |
| TOL-WATBAL-002 | Non-negative-domain comparator tolerance for state/flux magnitudes | lower bound `>= -1e-12 m` | Comparator-noise allowance only; no silent runtime clamping. |
| TOL-WATBAL-003 | Layer percolation non-negativity comparator tolerance (`pei`) | lower bound `>= -1e-12 m d^-1` | Negative values beyond tolerance are invalid-state failures. |
| TOL-WATBAL-004 | Stress-factor bound tolerance for `Ws` | `abs(bound violation) <= 1e-12` | Domain expectation remains `[0,1]`. |
| TOL-WATBAL-005 | Zero-demand transpiration threshold for denominator guard | `Etp <= 1e-12 m d^-1` treated as zero-demand branch | Runtime still requires explicit `Σ Ui = 0` and `Ws = 1` behavior. |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-WATBAL-001 | Per-invariant comparator vectors for all `INV-WATBAL-*` families are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance gating. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-WATBAL-002 | Downstream companion contracts (`SC-RUNOFFPART-001`, `SC-EVAP-001`, `SC-PERC-001`, `SC-SUBHYD-001`) are not yet fully authored. | Cross-contract ownership boundaries remain provisional. | non-promotable | `[DIRECT][Static]` |
| GAP-WATBAL-003 | Concrete openWEPP API/field aliases for canonical Chapter-5 symbols are not yet fixed. | Alias map remains identity-only and requires amendment when boundary names diverge. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-WATBAL-004 | Chapter-5 validation reports stronger near-surface (`0.05 m`) water-content agreement than full-profile (`0-2 m`) agreement in the cited watershed test. | Deep-profile closure interpretation confidence remains lower than near-surface Tier-A signals. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-04 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-04 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: clarified daily-step closure enforcement, added explicit zero-demand `Ws` branch, expanded alias map coverage, and added Chapter-5 validation caveat gap entry. |
