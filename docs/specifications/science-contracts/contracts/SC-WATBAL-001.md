---
contract_id: SC-WATBAL-001
title: Water Balance Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 14
producer_scope:
  - Daily root-zone water balance accounting surfaces
  - Daily evapotranspiration distribution and percolation-routing accounting surfaces
  - Daily coupling surfaces linking climate/infiltration/runoff/snow state into water-balance closure
consumer_scope:
  - Plant growth stress and daily growth-regulation consumers
  - Runoff partition and infiltration antecedent-moisture consumers
  - Subsurface/lateral-flow and drainage consumers using daily loss-accounting surfaces
evidence_level: static
last_reviewed: 2026-05-23
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

## Algorithm State Surfaces (WB11 Hydrology Production Kernels)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| Coupled PL ordering preconditions | `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth` (validated at growth dispatch before hydrology lane entry) |
| Runoff reconciliation state family | `nslpts`, `slplen`, `avgslp`, `xinput_0001`, `slpinp_0001`, `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| Storage reconciliation state family | `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| WB11 ET/perc/lateral/drain state inputs | `wb11_soil_water`, `wb11_et_demand`, `wb11_field_capacity`, `wb11_perc_fraction`, `wb11_drainable_storage`, `wb11_lateral_fraction`, `wb11_drainage_fraction`, `wb11_drainage_coefficient` |

### Required Outputs

| Surface | Output |
|---|---|
| WB11 ET/perc flux outputs | `ET`, `Ws`, `D`, `Pe` |
| WB11 lateral/drainage outputs | `q`, `Qdd`, `Qd` |
| WB11 state updates | `wb11_soil_water`, `wb11_drainable_storage` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range WB11 hydrology domains |

### Mutated State Surfaces

WB11 mutates water-balance hydrology surfaces deterministically through
phase-specific ET/percolation/lateral/drainage kernels while preserving
orchestrator-owned writeback commit authority.

## Algorithm Specification (WB11 Scheduler Hydrology Production Execution)

1. Map canonical scheduler phase to typed hillslope kernel phase class.
2. Execute WB11 ET/percolation/lateral/drainage phase kernels with deterministic
   state/flux updates and typed invariant guards.
3. Enforce finite and domain bounds for all required WB11 inputs and emitted
   outputs prior to writeback acceptance.
4. Preserve explicit routing hard-fail posture for unsupported/mismatched
   hydrology phase-class combinations.
5. Apply only accepted writeback payloads via orchestrator-owned state/flux
   maps; reject malformed payloads with typed status signals.

## Branch and Guard Table (WB11 Hydrology Kernel Set)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-WATBAL-WB11-ET` | phase class `hydrology_evapotranspiration` | `wb11_soil_water`, `wb11_et_demand` | runtime | deterministic ET/writeback execution with typed guards (`HKERNEL-WB11-ET-E-001..003`) |
| `BR-WATBAL-WB11-PERC` | phase class `hydrology_percolation_deep_seepage` | `wb11_soil_water`, `wb11_field_capacity`, `wb11_perc_fraction` | runtime | deterministic percolation/writeback execution with typed guards (`HKERNEL-WB11-PERC-E-001..003`) |
| `BR-WATBAL-WB11-LAT` | phase class `hydrology_lateral_transfer` | `wb11_drainable_storage`, `wb11_lateral_fraction`, `Pe` | runtime | deterministic lateral/writeback execution with typed guards (`HKERNEL-WB11-LAT-E-001..003`) |
| `BR-WATBAL-WB11-DRAIN` | phase class `hydrology_drainage` | `wb11_drainable_storage`, `wb11_drainage_fraction`, `wb11_drainage_coefficient`, `q` | runtime | deterministic drainage/writeback execution with typed guards (`HKERNEL-WB11-DRAIN-E-001..003`) |
| `BR-WATBAL-WB11-UNSUPPORTED` | unsupported hydrology phase-class state | scheduler phase + phase class metadata | runtime | typed hard-fail (`HS-HYDRO-E-001`) and scheduler halt |

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
| INV-WATBAL-009 | WB11 production execution invariant: ET/percolation/lateral/drainage kernels must emit deterministic state/flux updates (`ET`, `Ws`, `D`, `Pe`, `q`, `Qdd`, `Qd`) and update owned WB11 state surfaces. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH6-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-010 | WB11 guard + routing invariant: unsupported hydrology phase classes and missing/non-finite/out-of-range WB11 domains must surface typed hard failures (`HS-HYDRO-E-001`, `HKERNEL-WB11-*-E-*`) without silent reassignment/clamping/defaulting. | hard-fail | REF-WATBAL-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-WATBAL-011 | INT10 coupled lane-entry invariant: watbal/hydrology phases execute only after successful plant-lane decomposition/growth transition completion with valid ordering preconditions (`pl_order_growth_after_decomp = 1`, `pl_order_watbal_after_growth = 1`); ordering-symbol violations must hard-fail before watbal-lane completion. | hard-fail | REF-WATBAL-CH5-LINK, REF-WATBAL-CH8-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-012 | PL14 replay-candidate emission invariant: WB13 candidate rows staged for strict Tier-A replay must preserve canonical 25-column schema and deterministic `(Y, J, OFE)` ordering; missing required symbols/artifacts or schema/arity violations must hard-fail replay staging without truncation, padding, or legacy-surface substitution. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-013 | CLIM05 snow-coupled closure invariant: when active snow coupling publishes signed `S`, WB12 storage reconciliation must use `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd` and hard-fail on missing/non-finite/domain-invalid `S`. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-CH3-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `INV-WATBAL-009` | runtime | WB11 ET/perc/lateral/drain production kernel execution paths | Typed hard error on non-deterministic/malformed WB11 hydrology writeback outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-010` | runtime | WB11 routing + guard tables | Typed hard error on unsupported phase classes or WB11 domain-invalid inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |
| `INV-WATBAL-011` | runtime | Scheduler phase closure and coupled lane-entry guard between growth dispatch and hydrology execution | Typed hard error on ordering-precondition violation and halt before watbal completion | Tier-A gate for INT10 coupled replay | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-012` | runtime | WB13 replay-candidate staging gate before strict comparator execution | Typed hard error on missing/invalid WB13 replay rows or missing replay artifacts; no schema rewrite/fallback padding | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-013` | runtime | WB12 storage reconciliation with active CLIM05 snow-coupled `S` term | Typed hard error on missing/non-finite/domain-invalid `S` or violated CLIM05 storage closure equation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| WB11 hydrology production execution + guards (`INV-WATBAL-009/010`) | ET/perc/lateral/drain kernel execution and routing/guard validation | Hard error on malformed WB11 domains or unsupported hydrology phase classes | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB13 replay-candidate schema/order and artifact completeness (`INV-WATBAL-012`) | WB13 output staging and replay boundary | Hard error when strict replay staging sees missing required WB13 symbols/artifacts or schema/ordering violations | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| CLIM05 snow-coupled WB12 storage closure (`INV-WATBAL-013`) | WB12 storage reconciliation stage | Hard error on missing/non-finite/domain-invalid signed `S` term or CLIM05 storage equation violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB11_HYDRO_UNSUPPORTED_ROUTING_CODE` | status message id | `HS-HYDRO-E-001` | Typed failure code for unsupported hydrology phase-class routing states | REF-WATBAL-PHYS-BOUNDS |
| `WB11_ET_GUARD_CODES` | status message id range | `HKERNEL-WB11-ET-E-001..003` | Typed ET guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB11_PERC_GUARD_CODES` | status message id range | `HKERNEL-WB11-PERC-E-001..003` | Typed percolation guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB11_LATERAL_GUARD_CODES` | status message id range | `HKERNEL-WB11-LAT-E-001..003` | Typed lateral guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB11_DRAINAGE_GUARD_CODES` | status message id range | `HKERNEL-WB11-DRAIN-E-001..003` | Typed drainage guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_STATUS_OK` | status message id | `HKERNEL-WB13-HWAT-OK-001` | Typed nominal status for WB13 daily output-row emission success | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_MISSING` | status message id | `HKERNEL-WB13-HWAT-E-001` | Typed missing-required-symbol guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_NONFINITE` | status message id | `HKERNEL-WB13-HWAT-E-002` | Typed non-finite-value guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_DOMAIN` | status message id | `HKERNEL-WB13-HWAT-E-003` | Typed domain/order/schema guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |

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

## Test-Vector Obligations

Minimum WB11 hydrology production-kernel conformance vectors:

1. ET/percolation/lateral/drain phases emit deterministic WB11 outputs and
   mutate only declared WB11 state surfaces via writeback.
2. Non-finite and domain-invalid WB11 hydrology inputs hard-fail with typed
   WB11 guard codes and halt at the affected phase.
3. Unsupported hydrology phase-class combinations hard-fail with typed routing
   status (`HS-HYDRO-E-001`) and no fallback/default class rewrite.
4. INT10 coupled replay vectors:
   - canonical replay demonstrates watbal lane execution after successful plant
     transition phases in scheduler order;
   - hydrology phases observe state written by prior decomposition/growth
     transitions (state-transfer continuity);
   - missing or non-finite coupled ordering symbols hard-fail before watbal
     lane completion.

## WB12 Reconciliation Authority Addendum

### WB12 Required Surfaces

| Surface | Symbols |
|---|---|
| Runoff reconciliation inputs | `wb12_rainfall_input`, `wb12_runon_input`, `wb12_infiltration`, `wb12_depression_storage_delta`, `wb12_runoff_observed`, `wb12_runoff_closure_tolerance` |
| Storage reconciliation inputs | `wb12_storage_initial`, `wb12_storage_observed`, `wb12_storage_closure_tolerance`, `wb12_precip_input`, `S`, `Q`, `ET`, `D`, `Qd` |
| Runoff reconciliation outputs | `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |
| Storage reconciliation outputs | `wb12_storage_closure_delta`, `wb12_storage_reconciled` |

### WB12 Deterministic Reconciliation Rules

1. Runoff reconciliation emits:
   - `Q = wb12_rainfall_input + wb12_runon_input - wb12_infiltration - wb12_depression_storage_delta`
   - `wb12_runoff_closure_delta = Q - wb12_runoff_observed`
2. Storage reconciliation emits:
   - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd`
   - `wb12_storage_closure_delta = wb12_storage_reconciled - wb12_storage_observed`
3. Absolute closure deltas above declared per-phase tolerances are invalid closure states.
4. Missing/non-finite/out-of-range inputs and invalid closure states hard-fail with typed status and do not apply writeback.

### WB12 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB12-RUNOFF-E-001` | `HKERNEL-WB12-RUNOFF-E-002` | `HKERNEL-WB12-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### WB12 Contract-Test Vectors

1. Valid WB12 runoff/storage inputs produce deterministic reconciliation outputs and state updates.
2. Non-finite WB12 runoff/state input hard-fails at the corresponding reconciliation phase with typed non-finite guard code.
3. Closure-delta overflow beyond tolerance hard-fails with typed domain/closure guard code and no writeback mutation.

## CLIM05 Snow-Coupled Reconciliation Addendum

### CLIM05 Required Coupling Surface

| Surface | Symbols |
|---|---|
| Signed snow-water coupling term | `S` (daily `+` melt, `-` accumulation) |

### CLIM05 Deterministic Coupling Rule

1. Runoff reconciliation consumes snow-coupled liquid input through signed `S`
   authority and emits `S` to downstream storage reconciliation.
2. WB12 storage reconciliation must include `S` exactly once in the storage
   closure equation (additive signed term).
3. Missing/non-finite/domain-invalid `S` is a hard-fail storage state.

### CLIM05 Contract-Test Vectors

1. Active snow coupling nominal vector changes both runoff closure (`Q`) and
   WB12 storage closure through signed `S`.
2. Missing `S` at storage reconciliation hard-fails with typed missing-input
   posture.
3. Non-finite or domain-invalid `S` hard-fails with typed non-finite/domain
   posture and no writeback mutation.

## CLIM06 Frozen-Soil Infiltration Coupling Addendum

### CLIM06 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Frozen-state runtime outputs | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |
| Runoff/storage reconciliation symbols | `wb12_infiltration`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled`, `wb12_storage_reconciled` |

### CLIM06 Deterministic Coupling Rules

1. Active CLIM06 coupling is explicit when
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. WB14 runoff reconciliation must consume `frost.runtime_infcap_frz` as the
   frozen-soil effective infiltration-capacity term when active CLIM06 coupling
   is enabled.
3. CLIM06 frozen-state domains are bounded and non-negative:
   - `0 <= frost.runtime_dfrost <= 0.20`
   - `0 <= frost.runtime_dthaw <= 0.20`
   - `frost.runtime_nft >= 0`
   - `frost.runtime_ws_frz >= 0`
   - `0 <= frost.runtime_infcap_frz <= ssc`
4. Missing/non-finite/out-of-domain active-coupling frost symbols are
   hard-fail states in WB14 reconciliation; no fallback/default branch is
   allowed.

### CLIM06 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |

### CLIM06 Contract-Test Vectors

1. Active CLIM06 vector reduces infiltration-capacity and updates frozen-state
   outputs while preserving typed runoff/storage reconciliation closure.
2. Missing required active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain active-coupling frost symbol/state hard-fails with
   `HKERNEL-WB14-RUNOFF-E-003`.

## WB14 Infiltration and Hyetograph Coupling Addendum

### WB14 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Runoff reconciliation forcing | `ninten` or `nbrkpt`; `timem_####`; `intsty_####`; `ssc`; `dg`; `thetdr`; `thetfc` |
| Runoff reconciliation state inputs | `wb12_rainfall_input`, `wb12_runon_input`, `wb12_depression_storage_delta`, `wb12_runoff_observed`, `wb12_runoff_closure_tolerance` |
| Runoff reconciliation outputs | `wb12_infiltration`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### WB14 Deterministic Coupling Rules

1. Runoff reconciliation computes infiltration from subdaily hyetograph forcing
   within the runoff kernel branch; externally seeded `wb12_infiltration` is no
   longer a required input for acceptance paths.
2. Reconciliation uses computed infiltration and hyetograph rainfall depth in:
   - `Q = wb14_hyetograph_rainfall + wb12_runon_input - wb12_infiltration - wb12_depression_storage_delta`
3. `wb12_rainfall_input` remains a required closure-consistency surface and must
   match hyetograph-integrated rainfall depth within
   `wb12_runoff_closure_tolerance`.
4. Reconciliation and downstream storage closure (`wb12_storage_reconciled`)
   remain deterministic and typed-fail on missing/non-finite/domain-invalid
   inputs.

### WB14 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |

### WB14 Contract-Test Vectors

1. Valid hyetograph + soil infiltration forcing emits deterministic
   `wb12_infiltration`, `Q`, and WB12 runoff closure diagnostics.
2. Missing required WB14 forcing symbol hard-fails at runoff reconciliation
   with `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite WB14 forcing/reconciliation symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Non-monotone hyetograph time, negative intensity, rainfall mismatch, or
   runoff closure overflow hard-fail with `HKERNEL-WB14-RUNOFF-E-003`.

## WB15 Canopy Interception Coupling Addendum

### WB15 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Plant runtime interception inputs | `cancov`, `lai`, `vdmt` |
| Runoff/infiltration forcing inputs | `timem_####`, `intsty_####`, `wb12_rainfall_input` |
| Interception + closure outputs | `I`, `wb12_infiltration`, `Q`, `wb12_storage_reconciled` |

### WB15 Deterministic Coupling Rules

1. Canopy interception is computed before runoff/infiltration reconciliation
   and before daily storage closure acceptance.
2. Biomass context for interception uses live above-ground biomass proxy:
   - `VE = vdmt * 10000` (`kg ha^-1`)
3. Canopy-interception potential follows Chapter-5 Eq. [5.1.2] lineage:
   - `Ipot = cancov * ((0.000627 * VE - 3.73349e-8 * VE^2) / 1000)` (`m`)
4. Runtime interception is bounded by available hyetograph rainfall:
   - `I = min(Ipot, wb14_hyetograph_rainfall)` when `lai > 0` and `cancov > 0`
   - `I = 0` when `lai <= 0` or `cancov <= 0`
5. Domain requirements are hard-fail:
   - `0 <= cancov <= 0.999`
   - `lai >= 0`
   - `0 <= vdmt <= 0.8` (`kg m^-2`) so `0 <= VE <= 8000` (`kg ha^-1`)
6. Runoff/infiltration reconciliation consumes interception explicitly:
   - `wb14_hyetograph_liquid_after_interception = wb14_hyetograph_rainfall - I`
   - `Q = wb14_hyetograph_liquid_after_interception + S + wb12_runon_input - wb12_infiltration - wb12_depression_storage_delta`
7. Daily storage closure consumes interception as an explicit Chapter-5 term:
   - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - I - Q - ET - D - Qd`
8. Missing/non-finite/out-of-domain canopy interception symbols are hard-fail
   invalid states. No fallback/default/clamp behavior is allowed.

### WB15 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### WB15 Contract-Test Vectors

1. Valid `cancov`/`lai`/`vdmt` inputs emit finite `I` and deterministic
   coupled `wb12_infiltration`, `Q`, and `wb12_storage_reconciled`.
2. Missing canopy interception input symbol (`cancov`, `lai`, or `vdmt`)
   hard-fails with `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite canopy interception input hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain canopy interception input (`cancov`, `lai`, `vdmt`) or
   coupled closure overflow hard-fails with `HKERNEL-WB14-RUNOFF-E-003` or
   `HKERNEL-WB12-STORAGE-E-003` at the affected phase.

## IRRIG10 Irrigation Storage-Coupling Addendum

### IRRIG10 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Runtime irrigation schedule traces | `irrigation.runtime_schedule_source`, `irrigation.runtime_depth_m`, `Irr` |
| Runoff reconciliation coupled outputs | `Q`, `wb12_runoff_reconciled`, `wb12_runoff_closure_delta` |
| Storage reconciliation outputs | `wb12_storage_reconciled`, `wb12_storage_closure_delta` |

### IRRIG10 Deterministic Storage Rules

1. Storage reconciliation consumes daily irrigation depth as explicit input:
   `Irr = irrigation.runtime_depth_m`.
2. Storage closure equation under irrigation is explicit:
   `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S + Irr - I - Q - ET - D - Qd`.
3. `Irr` must be finite and non-negative whenever published.
4. Missing/non-finite/out-of-domain irrigation storage symbols are hard-fail
   invalid states; no fallback/default branch is allowed.

### IRRIG10 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### IRRIG10 Contract-Test Vectors

1. Active fixed-date sprinkler event emits positive `Irr` and deterministic
   irrigation-coupled `wb12_storage_reconciled`.
2. Active depletion sprinkler event emits positive `Irr` and deterministic
   irrigation-coupled `wb12_storage_reconciled`.
3. Missing irrigation scheduling/storage symbols hard-fail with typed missing
   guard posture.
4. Non-finite/out-of-domain irrigation scheduling/storage symbols hard-fail
   with typed non-finite/domain guard posture.

## WB16 Peak Runoff Closure-Diagnostics Addendum

### WB16 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Closure-diagnostics required inputs | `Q`, `timem_####`, `intsty_####`, `timep`, `efflen`, `ealpha`, `m`, `I`, `irrigation.runtime_rate_m_per_s` |
| Closure-diagnostics peak outputs | `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

### WB16 Deterministic Peak-Flow Rules

1. WB16 executes at closure diagnostics and consumes reconciled runoff depth
   `Q` from WB14 plus coupled runtime forcing metadata from the accepted event.
2. Event duration for WB16 is derived from hyetograph elapsed time:
   - `effdrr = timem_last - timem_first` (`s`)
3. Mean runoff rate and runoff-maximum ratio terms are:
   - `vave = Q / effdrr`
   - `remax = max(intsty_####) + irrigation.runtime_rate_m_per_s`
   - `vstar = vave / remax`
4. Kinematic-wave time ratio and branch selector terms follow Chapter-4
   lineage (`appmth.for`):
   - `te = (efflen / (ealpha * vave^(m-1)))^(1/m)`
   - `tstar = te / effdrr`
   - `tc = timep`
5. Peak-runoff nondimensional ratio `qpstar` is branch-authoritative:
   - partial-equilibrium branch (`tstar >= 1`): `qpstar = 1 / tstar^m`
   - quasi-equilibrium branch A (`tc < tstar < 1`): `qpstar = 1 / tstar`
   - quasi-equilibrium branch B (`0 < tstar <= tc`):
     `qpstar = 1/vstar - 0.6 * ((1 - vstar) / vstar) * tstar`
6. Peak runoff and duration outputs are:
   - `peakro_raw = vave * qpstar`
   - `peakro = max(peakro_raw, 3.63e-8)` (legacy minimum-flow floor from
     `conrun.for`)
   - `watdur = Q / peakro`
7. Duration cap rule is explicit:
   - if `watdur > 86400`, set `watdur = 86400`.
8. WB16 domain posture is hard-fail for missing/non-finite/out-of-domain
   symbols and non-physical intermediates (`effdrr <= 0`, `vave <= 0`,
   `remax <= 0`, `vstar <= 0`, `vstar > 1`, `m <= 1`, `ealpha <= 0`,
   `efflen <= 0`, `timep` outside `[0,1]`, or non-finite `peakro`/`watdur`).
   No fallback/default branch is allowed.

### WB16 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Closure diagnostics | `HKERNEL-WB16-PEAK-E-001` | `HKERNEL-WB16-PEAK-E-002` | `HKERNEL-WB16-PEAK-E-003` |

### WB16 Contract-Test Vectors

1. Nominal WB16 vector emits finite `peakro` and `watdur` with continuity
   `watdur = Q/peakro` and one authoritative method branch id.
2. Branch-selector vectors independently trigger:
   - `tstar >= 1`,
   - `tc < tstar < 1`,
   - `0 < tstar <= tc`.
3. Missing required WB16 symbol hard-fails in closure diagnostics with
   `HKERNEL-WB16-PEAK-E-001`.
4. Non-finite WB16 required symbol hard-fails with `HKERNEL-WB16-PEAK-E-002`.
5. Domain-invalid WB16 symbol/intermediate hard-fails with
   `HKERNEL-WB16-PEAK-E-003`.

## WB13 Daily Output-Surface Authority Addendum

### WB13 Canonical Daily Output Schema (`H5.wat.dat` Equivalent)

WB13 daily output rows are authoritative at exactly 25 numeric columns in this
canonical order:

1. `OFE`
2. `J`
3. `Y`
4. `P`
5. `RM`
6. `Q`
7. `Ep`
8. `Es`
9. `Er`
10. `Dp`
11. `UpStrmQ`
12. `SubRIn`
13. `latqcc`
14. `Total-Soil`
15. `frozwt`
16. `Snow-Water`
17. `QOFE`
18. `Tile`
19. `Irr`
20. `Area`
21. `SoilWaterTotal`
22. `ProfileDepth`
23. `ProfilePorosityCap`
24. `ProfileFCStore`
25. `ProfileWPStore`

### WB13 Deterministic Row Keys and Ordering

1. WB13 row keys are authoritative as `(Y, J, OFE)` and must be strictly
   monotonic non-decreasing in emitted order.
2. Duplicate row keys within one emitted daily surface are invalid.
3. Output rows must remain deterministic under identical accepted upstream
   daily kernel surfaces.

### WB13 Output-Surface Invariants

1. `QOFE = Q` for single-OFE daily Tier-A rows.
2. `SoilWaterTotal = Total-Soil + frozwt` within `1e-6 mm`.
3. `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.
4. Required depth-like and storage-like columns in this WB13 surface are
   non-negative.
5. Missing required symbols, non-finite values, and schema/order violations
   are hard-fail invalid states.

### WB13 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB13-HWAT-E-001` |
| Non-finite required symbol | `HKERNEL-WB13-HWAT-E-002` |
| Domain/order/schema violation | `HKERNEL-WB13-HWAT-E-003` |

### WB13 Contract-Test Vectors

1. Nominal WB13 vector emits deterministic 25-column rows in canonical schema
   order and deterministic `(Y, J, OFE)` ordering.
2. Missing required WB13 symbol (for example `ProfileDepth`) hard-fails with
   `HKERNEL-WB13-HWAT-E-001`.
3. Non-finite WB13 symbol hard-fails with `HKERNEL-WB13-HWAT-E-002`.
4. Domain/order/schema violations hard-fail with `HKERNEL-WB13-HWAT-E-003`
   and do not emit malformed rows.
5. PL14 strict replay staging rejects missing WB13 replay artifacts and does
   not synthesize fallback rows/files to satisfy comparator include surfaces.

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
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added scheduler hydrology phase-entry routing authority for runoff/storage classes with explicit unsupported-class hard-fail posture and WB10 test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted hydrology section from routing-only scaffolding to ET/percolation/lateral/drain production-kernel authority with deterministic WB11 updates, typed WB11 guard codes, and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB12 amendment: added runoff/storage reconciliation kernel authority with deterministic closure diagnostics, typed WB12 guard codes, and WB12 contract-derived vectors. |
| `2026-05-23` | `6` | `Codex` | WB13 amendment: added canonical daily water-balance output surface authority (`H5.wat.dat` equivalent) with fixed 25-column schema, deterministic row-order key rules, typed WB13 output guards, and WB13 contract-derived vectors. |
| `2026-05-23` | `7` | `Codex` | INT10 amendment: added coupled watbal lane-entry invariant (`INV-WATBAL-011`), scheduler ordering-precondition guard authority, and INT10 coupled replay test-vector obligations for ordering/state-transfer validation. |
| `2026-05-23` | `8` | `Codex` | PL14 amendment: added replay-candidate emission invariant (`INV-WATBAL-012`) with strict WB13 schema/order + artifact completeness guard authority for Tier-A closeout staging. |
| `2026-05-23` | `9` | `Codex` | WB14 amendment: added computed infiltration + hyetograph coupling authority for runoff reconciliation, replacing required externally seeded infiltration input in acceptance paths and adding typed WB14 runoff guards. |
| `2026-05-23` | `10` | `Codex` | CLIM05 amendment: added signed `S` snow-coupling authority for WB12 storage reconciliation, including required storage-surface inputs, deterministic storage equation update, and typed active-coupling guard vectors. |
| `2026-05-23` | `11` | `Codex` | CLIM06 amendment: added frozen-soil infiltration-capacity coupling authority (`frost.runtime_infcap_frz`) for WB14 runoff reconciliation, bounded frozen-state surface requirements, and typed active-coupling guard vectors. |
| `2026-05-23` | `12` | `Codex` | WB15 amendment: added canopy interception coupling authority from plant runtime surfaces (`cancov`, `lai`, `vdmt`) with Eq. [5.1.2] lineage, explicit runoff/storage closure integration of `I`, and typed hard-fail guard posture for missing/non-finite/domain-invalid canopy symbols. |
| `2026-05-23` | `13` | `Codex` | IRRIG10 amendment: added explicit irrigation depth coupling (`Irr`) into WB12 storage reconciliation equation and typed guard/test-vector obligations for irrigation-triggered runoff/storage closure. |
| `2026-05-23` | `14` | `Codex` | WB16 amendment: added closure-diagnostics peak-runoff authority (`peakro`, `watdur`) with three method branches (`tstar`/`tc`), explicit minimum-flow and max-duration rules, and typed WB16 guard/test-vector posture. |
