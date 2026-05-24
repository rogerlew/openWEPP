---
contract_id: SC-EVAP-001
title: Evapotranspiration Stress Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 7
producer_scope:
  - Potential and actual evapotranspiration partition surfaces
  - Evaporation/transpiration stress and availability-limited ET surfaces
  - Root-zone ET extraction and atmospheric-demand coupling surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Plant-growth and residue-state consumers influenced by ET stress signals
  - Comparator/replay surfaces using Tier-A daily closure confidence signals
evidence_level: Static
last_reviewed: 2026-05-23
supersedes: []
superseded_by: []
---

# SC-EVAP-001 Evapotranspiration Stress Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for evapotranspiration partition and
stress-domain behavior, including daily potential/actual ET semantics and
cross-domain coupling with water-balance, climate forcing, winter hydrology,
and plant-growth consumers.

## Scientific Scope

In scope:
- Daily potential ET estimation domain and input prerequisites. `[INFERENCE][Static]`
- Partition of potential ET into potential soil evaporation and potential plant
  transpiration. `[DIRECT][Static] + [INFERENCE][Static]`
- Bare-soil evaporation staging, residue attenuation, and root-zone ET
  distribution/stress semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- Plant-growth water-stress coupling boundaries. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Percolation/groundwater routing internals owned by `SC-PERC-001` and
  `SC-SUBHYD-001`. `[INFERENCE][Static]`
- Vegetation growth equations beyond ET stress-boundary obligations owned by
  `SC-PLANT-001`. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-EVAP-CH5-BAL | `references/50201000/chap5.pdf` §5.1 Eq. [5.1.1] | Daily closure includes ET withdrawal term and signed water-balance context. | `[DIRECT][Static]` |
| REF-EVAP-CH5-POT | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.1]-[5.2.7] | Potential ET formulation (Penman/Priestly-Taylor pathways) and climate-input prerequisites. | `[DIRECT][Static]` |
| REF-EVAP-CH5-PART | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.8]-[5.2.9] | Partition of potential ET into potential soil evaporation and potential plant transpiration. | `[DIRECT][Static]` |
| REF-EVAP-CH5-STAGE | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.10]-[5.2.13] | Bare-soil evaporation stage-one/stage-two behavior and residue attenuation relation. | `[DIRECT][Static]` |
| REF-EVAP-CH5-LAI | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.14] | LAI-based adjustment of potential transpiration up to `L <= 3`. | `[DIRECT][Static]` |
| REF-EVAP-CH5-DIST | `references/50201000/chap5.pdf` §5.3 Eq. [5.3.1]-[5.3.4] | Root-zone depth distribution for soil evaporation and layer-wise transpiration extraction under deficit conditions. | `[DIRECT][Static]` |
| REF-EVAP-CH5-LINK | `references/50201000/chap5.pdf` §5.5 Eq. [5.5.1] | Water-stress factor definition `Ws = (Σ Ui)/Etp` and coupling to plant-growth component. | `[DIRECT][Static]` |
| REF-EVAP-CH8-LINK | `references/50201000/chap8.pdf` §8.2.4 Eq. [8.2.14]-[8.2.15] | Plant growth regulation consumes ET-derived water-stress factor. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-CH2-FORCING | `references/50201000/chap2.pdf` §2.1.6-§2.1.8 Eq. [2.1.12]-[2.1.14] | Climate generator provides daily solar radiation, dew point, and wind inputs used by ET potential pathways. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-CH5-SNOW | `references/50201000/chap5.pdf` §5.1 and §5.3 text | Soil evaporation can be satisfied from snowpack first, then soil water. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative rate/depth domains and bounded stress factors. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Eu` | `m d^-1` | Daily potential evapotranspiration after energy-to-depth conversion from Eq. [5.2.1]/[5.2.2] pathway. | ET potential pathway | ET partition pathway |
| `Esp` | `m d^-1` | Potential soil evaporation (Eq. [5.2.8]). | ET partition pathway | soil-evaporation stage logic |
| `Etp` | `m d^-1` | Potential plant transpiration before/after LAI adjustment (Eq. [5.2.9], [5.2.14]). | ET partition pathway | layer uptake and stress-factor pathway |
| `Esu` | `m d^-1` | Stage-one soil-evaporation upper limit (Eq. [5.2.10]). | soil-evaporation stage logic | stage-transition logic |
| `Esb`, `Es` | `m d^-1` | Bare-soil and residue-adjusted actual soil evaporation (Eq. [5.2.12], [5.2.13]). | soil-evaporation stage logic | root-zone ET withdrawal |
| `Tr`, `d2` | `mm d^-0.5`, `d` | Soil transmissivity and stage-two day counter driving Eq. [5.2.11]-[5.2.12]. | soil parameterization + stage state | stage-two evaporation computation |
| `Cr` | `kg ha^-1` | Plant residue mass for evaporation attenuation (Eq. [5.2.13]). | residue/crop state | soil-evaporation attenuation |
| `L` | `m^2 m^-2` | Leaf area index for ET partition and transpiration adjustment. | crop-growth state | ET partition and LAI adjustment |
| `dx`, `ds` | `m` | Maximum and realized soil-evaporation depth (Eq. [5.3.1]-[5.3.2]). | ET root-zone pathway | soil-layer water updates |
| `UPi`, `Ui` | `m d^-1` | Potential and actual layer `i` plant water-use rates (Eq. [5.3.3]-[5.3.4]). | ET root-zone pathway | stress-factor and layer-water updates |
| `Θ`, `Θi`, `Θr`, `Θc`, `ULi` | `m`, `m`, `m^3 m^-3`, `fraction`, `m` | Root-zone/layer water states and thresholds used for ET extraction constraints. | soil/water state pathway | ET distribution logic |
| `Ws` | `fraction` | Plant-growth water-stress factor (`0..1`) from supply-demand ratio (Eq. [5.5.1], Eq. [8.2.15]). | ET coupling pathway | plant-growth regulation |
| `ET` | `m` | Daily cumulative evapotranspiration withdrawal term in water-balance closure Eq. [5.1.1]. | ET integration pathway | daily water-balance closure consumer |
| `RA`, `Tmax`, `Tmin`, `Tdp`, `u_z` | `Ly`, `degC`, `degC`, `degC`, `m s^-1` | Climate forcing surfaces required by potential ET formulations. | climate forcing pathway | ET potential pathway |
| `S` | `m` | Snow-water state that can satisfy evaporation demand before soil-water extraction. | winter hydrology pathway | ET withdrawal precedence logic |

## Algorithm State Surfaces (WB17 ET Production Kernel)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| ET consumer-boundary state family | `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| WB17 ET state inputs | `wb11_soil_water`, `wb11_et_demand`, `lai`, `wb17_residue_interception` |

### Required Outputs

| Surface | Output |
|---|---|
| ET flux outputs | `ET`, `Ws`, `Ep`, `Es`, `Er` |
| ET state updates | `wb11_soil_water` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range ET state domains |

### Mutated State Surfaces

WB17 mutates ET boundary surfaces deterministically:
- potential partition: `Esp = wb11_et_demand * exp(-0.4 * lai)` and
  `Etp = wb11_et_demand - Esp`.
- residue partition: `Er = min(Esp, wb17_residue_interception)` and
  `Es = Esp - Er`.
- soil extraction + plant extraction:
  - `Es_actual = min(Es, wb11_soil_water)`
  - `Ep = min(Etp, wb11_soil_water - Es_actual)`
  - `ET = Er + Es_actual + Ep`
- stress update: `Ws = 1` for `Etp <= 1e-12`, otherwise `Ep / Etp`.
- state update:
  `wb11_soil_water = wb11_soil_water - Es_actual - Ep`.

## Algorithm Specification (WB17 ET Production Execution)

1. Require finite ET inputs (`wb11_soil_water`, `wb11_et_demand`, `lai`,
   `wb17_residue_interception`) and enforce non-negative domains.
2. Compute deterministic potential partition (`Esp`, `Etp`) from Eq. [5.2.8]
   and Eq. [5.2.9] using runtime alias
   (`Eu` -> `wb11_et_demand`, `L` -> `lai`).
3. Compute explicit residue evaporation partition (`Er`) and remaining
   soil-evaporation demand (`Es`) before soil-water extraction.
4. Compute explicit soil evaporation, plant transpiration, total ET, and stress
   ratio (`Ws`) with zero-demand handling (`Etp <= 1e-12` => `Ws = 1`).
5. Reject missing, non-finite, or out-of-range ET inputs/outputs with typed
   hard-fail status; no silent fallback/clamping/defaulting paths are permitted.

## Branch and Guard Table (WB17 ET Kernel)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-EVAP-WB17-EXECUTE` | phase class `hydrology_evapotranspiration` | `wb11_soil_water`, `wb11_et_demand`, `lai`, `wb17_residue_interception` | runtime | deterministic ET partition + writeback execution |
| `BR-EVAP-WB11-MISSING` | required ET symbol absent | ET required symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-001`) |
| `BR-EVAP-WB11-NONFINITE` | ET symbol is NaN/Inf | ET required symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-002`) |
| `BR-EVAP-WB11-DOMAIN` | ET symbol/derived flux outside domain bounds | ET required + emitted symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-003`) |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-EVAP-001 | Potential ET partition invariant: partition equations (Eq. [5.2.8]-[5.2.9]) must be applied so that emitted potential components satisfy `Eu = Esp + Etp` within declared tolerance in ET-depth units. | hard-fail | REF-EVAP-CH5-PART, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-002 | Potential ET input-domain invariant: selected potential ET pathway (Eq. [5.2.1] or [5.2.2]) requires finite climate inputs and explicit pathway declaration; negative or undefined `Eu` is invalid. | hard-fail | REF-EVAP-CH5-POT, REF-EVAP-CH2-FORCING, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-003 | Bare-soil stage invariant: stage-one/stage-two evaporation transitions and transmissivity dependence must follow Eq. [5.2.10]-[5.2.12], including explicit restart to stage one when precipitation satisfies accumulated stage-two evaporation condition. | hard-fail | REF-EVAP-CH5-STAGE | `[DIRECT][Static]` |
| INV-EVAP-004 | Residue attenuation invariant: actual soil evaporation must follow Eq. [5.2.13], remain non-negative, and not exceed bare-soil evaporation for non-negative residue mass. | hard-fail | REF-EVAP-CH5-STAGE, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-005 | Soil-evaporation depth invariant: Eq. [5.3.1]-[5.3.2] constraints hold with `0 <= ds <= dx`; if snowpack water satisfies `Es`, soil-water extraction branch remains zero. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-CH5-SNOW, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-006 | Root-zone uptake distribution invariant: layer-wise potential uptake distribution (Eq. [5.3.3]) must preserve declared root-depth weighting and emit non-negative `UPi` values with finite layer sums. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-007 | Water-deficit adjustment invariant: actual layer uptake branch (Eq. [5.3.4]) must enforce threshold behavior at `Θi` vs `Θc ULi`, with `0 <= Ui <= UPi` and explicit scaling in deficit branch. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-008 | Stress-factor invariant: stress factor `Ws` must follow Eq. [5.5.1]/[8.2.15], remain within `[0,1]`, and be emitted with declared units/semantics for plant-growth consumers. | hard-fail | REF-EVAP-CH5-LINK, REF-EVAP-CH8-LINK, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-009 | Coupling completeness invariant: ET boundary payload must include required surfaces for water-balance closure (`ET` term context) and plant-growth stress coupling (`Ws`, demand/supply terms) with unit-consistent semantics. | hard-fail | REF-EVAP-CH5-BAL, REF-EVAP-CH5-LINK, REF-EVAP-CH8-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-010 | Governance limitation invariant: ET contract interpretation must remain explicit about daily-step process scope and cited method assumptions (modified Ritchie framework and pathway preconditions); missing scope labeling blocks promotion. | governance-fail | REF-EVAP-CH5-POT, REF-EVAP-CH5-STAGE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-011 | WB17 ET execution invariant: ET phase computes deterministic partitioned ET components (`Er`, `Es`, `Ep`) and derived closure outputs (`ET`, `Ws`) from required WB17 ET symbols and updates `wb11_soil_water` without implicit fallback branches. | hard-fail | REF-EVAP-CH5-PART, REF-EVAP-CH5-LINK, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-012 | WB17 ET guard invariant: missing/non-finite/out-of-range WB17 ET domains must surface typed hard failures (`HKERNEL-WB11-ET-E-001..003`) and cannot be silently clamped/defaulted. | hard-fail | REF-EVAP-PHYS-BOUNDS | `[INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-EVAP-001` | runtime | ET partition assembler | Typed hard error on partition residual above tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-002` | runtime | Potential ET pathway selector/validator | Typed hard error on missing forcing inputs, undefined pathway, or negative/non-finite `Eu` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-003` | runtime | Soil-evaporation stage state machine | Typed hard error on invalid stage transition or transmissivity branch misuse | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-EVAP-004` | runtime | Residue attenuation calculator | Typed hard error when `Es` violates Eq. [5.2.13] or non-negative/bounded domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-005` | runtime | Root-zone soil-evaporation depth updater | Typed hard error on invalid `ds`/`dx` bounds or snow-first precedence violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-006` | runtime | Layer-wise potential uptake distributor | Typed hard error on negative/non-finite `UPi` or invalid root-depth weighting output | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-007` | runtime | Layer-wise actual uptake branch evaluator | Typed hard error on threshold-branch mismatch or invalid `Ui` bounds | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-008` | runtime | Stress-factor calculator and boundary validator | Typed hard error on out-of-range/undefined `Ws` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-009` | runtime | Cross-domain ET boundary payload validator | Typed hard error on missing required ET/stress surfaces or units mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-010` | governance | Contract review + promotion checklist | Promotion `HOLD` if method/scope caveats are not explicit in contract/disposition artifacts | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-011` | runtime | WB17 ET production kernel execution path | Typed hard error on non-deterministic/malformed partition/writeback ET outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-012` | runtime | WB17 ET guard table (`HKERNEL-WB11-ET-E-001..003`) | Typed hard error on missing/non-finite/domain-invalid ET inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-5/Chapter-8 WEPP notation.
WB17 implementation now uses explicit runtime aliases for executed ET
equation vectors.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Eu` | `wb11_et_demand` | ET demand surface consumed by WB17 partition runtime | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `L` | `lai` | LAI-driven partition surface | `m^2 m^-2` -> `m^2 m^-2` | `[DIRECT][Static]` |
| `Er` | `wb17_residue_interception` (input) + `Er` (flux output) | residue evaporation partition surface | `m d^-1` -> `m` daily flux output | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Esp`, `Etp`, `Es`, `Ep` | `Esp`, `Etp` (derived runtime), `Es`, `Ep` (flux outputs) | ET partition and component output surfaces | `m d^-1` potential -> `m` daily component flux outputs | `[DIRECT][Static] + [INFERENCE][Static]` |
| `dx`, `ds`, `UPi`, `Ui` | identity names | root-zone distribution and uptake surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Θ`, `Θi`, `Θr`, `Θc`, `ULi` | identity names | soil-water state surfaces used by ET | chapter-declared units preserved | `[DIRECT][Static]` |
| `Ws` | identity name | ET-to-plant stress boundary surface | `fraction` preserved | `[DIRECT][Static]` |
| `ET` | identity name | ET-to-water-balance closure boundary surface | `m` preserved | `[DIRECT][Static]` |
| `RA`, `Tmax`, `Tmin`, `Tdp`, `u_z` | identity names | climate-to-ET forcing surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `S` | identity name | snow-to-ET precedence surface | `m` preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| Zero-canopy partition day | `L = 0` causing `Esp = Eu` and `Etp = 0` under Eq. [5.2.8]-[5.2.9]. | Explicit partition-equation outcome. | `[DIRECT][Static]` |
| High-canopy LAI cap day | `L > 3` where potential transpiration is not additionally adjusted by Eq. [5.2.14]. | Explicit LAI-adjustment branch limit. | `[DIRECT][Static]` |
| Stage-two soil evaporation day | Accumulated evaporation exceeds stage-one limit and stage-two equation governs. | Explicit Eq. [5.2.10]-[5.2.12] stage behavior. | `[DIRECT][Static]` |
| Snow-supplied evaporation day | Snow-water content satisfies daily `Es`, yielding no soil-water extraction for evaporation. | Explicit §5.3 snow-first soil-evaporation ordering. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Layer compensation day | Some layers have deficit-adjusted uptake while others compensate within Eq. [5.3.4] bounds. | Explicit root-compensation statement in §5.3. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Negative or non-finite ET rates/depths (`Eu`, `Esp`, `Etp`, `Esb`, `Es`, `UPi`, `Ui`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Partition mismatch for Eq. [5.2.8]-[5.2.9] beyond declared tolerance (`Eu` not equal to `Esp + Etp`). `[DIRECT][Static] + [INFERENCE][Static]`
- Bare-soil stage output violating declared stage transitions or residue attenuation relation. `[DIRECT][Static] + [INFERENCE][Static]`
- Soil-evaporation depth bounds violated (`ds < 0` or `ds > dx`) or snow-first precedence violated. `[DIRECT][Static] + [INFERENCE][Static]`
- Layer uptake branch mismatch (`Ui > UPi`, negative uptake, or missing Eq. [5.3.4] threshold handling). `[DIRECT][Static] + [INFERENCE][Static]`
- Stress factor outside `[0,1]` or undefined at boundary emission time. `[DIRECT][Static] + [INFERENCE][Static]`
- ET boundary payload missing required surfaces for closure/stress coupling semantics. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-EVAP-P-001: Emit ET partition/stress surfaces (`Eu`, `Esp`, `Etp`, `Es`, `UPi`, `Ui`, `Ws`) with canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-002: Apply explicit Eq. [5.2.*] and Eq. [5.3.*] branch logic for stage transitions, LAI adjustment, and deficit uptake; no implicit fallback branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-003: Enforce invariant failures via typed errors; no silent clamping/defaulting for invalid ET/stress states. `[INFERENCE][Static]`
- OBL-EVAP-P-004: Preserve coupling-ready stress semantics for plant-growth consumers (`Ws` bounded and unit-consistent). `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-EVAP-C-001: Water-balance consumers must ingest ET withdrawal with Eq. [5.1.1] sign/units semantics unchanged. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-002: Plant-growth consumers must reject malformed/out-of-domain stress payloads and preserve Eq. [8.2.15] supply-demand interpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-003: Climate and winter boundary consumers/providers must preserve forcing/snow semantics required for ET branch selection and snow-first evaporation precedence. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-004: All consumers must fail explicitly on invariant-violating payloads and carry invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Potential ET and partition closure (`INV-EVAP-001/002`) | ET potential + partition assembly | Hard error; reject ET publish for the day | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Soil-evaporation stage and attenuation (`INV-EVAP-003/004`) | stage-state and attenuation evaluation | Hard error on branch/order/domain failure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Root-zone extraction and uptake (`INV-EVAP-005/006/007`) | root-zone ET distribution stage | Hard error on bounds or branch mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stress and boundary coupling (`INV-EVAP-008/009`) | ET-to-plant and ET-to-water-balance handoff | Hard error on malformed stress or missing boundary payload | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scope/governance labeling (`INV-EVAP-010`) | review/verification/promotion | Governance `HOLD` until scope/method caveats are explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB17 ET production execution and guards (`INV-EVAP-011/012`) | ET kernel execution and guard validation | Hard error on malformed ET domains or invalid deterministic partition updates | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB11_ET_STATUS_OK` | status message id | `HKERNEL-WB11-ET-OK-001` | Typed nominal status for successful ET phase execution | REF-EVAP-CH5-BAL |
| `WB11_ET_GUARD_MISSING` | status message id | `HKERNEL-WB11-ET-E-001` | Typed missing-input guard code | REF-EVAP-PHYS-BOUNDS |
| `WB11_ET_GUARD_NONFINITE` | status message id | `HKERNEL-WB11-ET-E-002` | Typed non-finite guard code | REF-EVAP-PHYS-BOUNDS |
| `WB11_ET_GUARD_DOMAIN` | status message id | `HKERNEL-WB11-ET-E-003` | Typed domain guard code | REF-EVAP-PHYS-BOUNDS |
| `WB17_PARTITION_EXP_COEFF` | coefficient | `0.4` | WB17 LAI partition coefficient in `Esp = Eu * exp(-0.4 * L)` | REF-EVAP-CH5-PART |
| `WB17_ETP_ZERO_THRESHOLD` | `m d^-1` | `1e-12` | Explicit zero-demand denominator guard for `Ws` | REF-EVAP-CH5-LINK |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). `[DIRECT][Static]` Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-EVAP-001 | Partition closure residual for Eq. [5.2.8]-[5.2.9] | `<= 1e-9 m d^-1` | Residual computed as `Eu - (Esp + Etp)` in ET-depth units. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-EVAP-002 | Non-negative comparator tolerance for ET rates/depths | lower bound `>= -1e-12` in declared ET units | Comparator-noise allowance only; runtime hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-EVAP-003 | Soil-evaporation depth bounds tolerance | `-1e-12 m <= ds <= dx + 1e-12 m` | Preserves explicit `0..dx` semantics with floating-noise allowance. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-EVAP-004 | Stress-factor bounds tolerance | `-1e-12 <= Ws <= 1 + 1e-12` | Runtime still enforces bounded stress semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Test-Vector Obligations

Minimum WB17 ET production-kernel conformance vectors:

1. WB17 nominal partition vector emits deterministic ET components (`Ep`, `Es`,
   `Er`) plus aggregate `ET`/`Ws` and updates `wb11_soil_water` from valid WB17
   inputs.
2. Soil-limited vector preserves residue evaporation (`Er`) while reducing
   transpiration (`Ep`) and `Ws` via explicit supply-demand stress relation.
3. Missing required WB17 ET symbol (`wb17_residue_interception`) hard-fails
   with typed status `HKERNEL-WB11-ET-E-001`.
4. Non-finite/domain-invalid WB17 ET inputs hard-fail with typed status
   family `HKERNEL-WB11-ET-E-002/003` and do not mutate orchestrator
   writeback surfaces.

## WB13 Daily Output Coupling Addendum

### WB13 ET-Coupled Output Symbols

| WB13 column | ET coupling surface | Units |
|---|---|---|
| `Ep` | Plant-transpiration component exported for daily reporting | `mm` |
| `Es` | Soil-evaporation component exported for daily reporting | `mm` |
| `Er` | Residue-evaporation component exported for daily reporting | `mm` |

### WB13 Coupling Requirements

1. WB13 daily rows must include finite ET-component symbols `Ep`, `Es`, and
   `Er` with non-negative daily magnitudes.
2. ET-component omission or malformed ET-component values are invalid WB13
   output states and must hard-fail via WB13 typed guards.
3. ET-component boundaries emitted to WB13 remain unit-consistent with
   Chapter-5 daily water-balance closure expectations.

## WB15 Canopy-Interception Coupling Addendum

### WB15 Coupling Surfaces

| Surface | Symbols |
|---|---|
| Interception closure surface | `I` |
| ET closure surfaces | `ET`, `Ws` |
| Plant-state interception drivers (external producer) | `cancov`, `lai`, `vdmt` |

### WB15 Coupling Requirements

1. Canopy interception remains an explicit Chapter-5 closure term (`I`) and is
   not implicitly folded into `ET` by boundary consumers.
2. ET kernel outputs (`ET`, `Ws`) must remain finite and domain-valid when
   interception coupling is active.
3. Missing/non-finite/out-of-domain interception symbol `I` at storage-closure
   consumers is a typed hard-fail state; no silent ET-side substitution is
   allowed.
4. Plant-runtime canopy interception drivers are owned by plant-domain
   producers; ET consumers must not redefine or clamp producer payloads.

### WB15 Contract-Test Vectors

1. Canopy-coupled storage closure vector verifies `I` is consumed as a distinct
   closure term alongside ET and does not overwrite ET semantics.
2. Missing interception symbol `I` at closure consumer boundaries hard-fails
   with typed missing-input status.
3. Non-finite interception symbol `I` hard-fails with typed non-finite status.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-EVAP-001 | Per-invariant comparator vectors for all ET invariant families are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-EVAP-002 | WB17 now fixes executed runtime aliases for `Eu`, `L`, and residue-partition ET symbols, but cross-domain alias harmonization for full Chapter-5 ET variable family remains incomplete. | Partial alias closure still leaves downstream harmonization risk. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-003 | Companion contracts (`SC-PERC-001`, `SC-SUBHYD-001`, `SC-RESIDUE-001`) are not fully authored, so coupled ownership boundaries remain provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-EVAP-004 | Chapter-5 validation emphasizes total ET and water-balance behavior; component-level partition validation (`Esp` vs `Etp` vs stage transitions) is not fully separated in available cited evidence. | Partition-subcomponent confidence is lower than aggregate ET confidence until dedicated evidence is added. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-005 | Full legacy stage-memory/state transition physics (`s1`, `s2`, `tu`, root-layer extraction surfaces) is not yet projected as first-class WB17 runtime symbols in openWEPP. | Limits strict one-to-one legacy state trajectory comparison despite equation-driven partition replacement of WB11 surrogate behavior. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-07 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-5/8 authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-07 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added missing symbol/alias coverage (`Θc`, `ET`), normalized evidence-mode casing, strengthened snow provenance anchor wording, and evidence-tagged degenerate/tolerance claims. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added explicit ET phase-entry routing authority, unsupported-class typed hard-fail posture, and WB10 ET test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted ET section from routing-only scaffolding to production-kernel authority with deterministic `ET`/`Ws` updates, typed ET guard codes (`HKERNEL-WB11-ET-E-001..003`), and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB13 amendment: added ET component coupling authority for canonical daily output columns (`Ep`, `Es`, `Er`) with explicit WB13 malformed-output hard-fail posture. |
| `2026-05-23` | `6` | `Codex` | WB15 amendment: added explicit canopy-interception coupling requirements so `I` remains a distinct closure term relative to ET outputs (`ET`, `Ws`) under typed consumer guard posture. |
| `2026-05-23` | `7` | `Codex` | WB17 amendment: replaced WB11 ET surrogate algorithm authority with equation-driven WB17 partition semantics (`Esp`, `Etp`, `Er`, `Es`, `Ep`) using explicit runtime alias mapping and WB17 contract-derived vector obligations. |
