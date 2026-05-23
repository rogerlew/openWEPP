---
contract_id: SC-IMPOUND-001
title: Surface Impoundment Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 4
producer_scope:
  - Daily hydraulic routing state/flux surfaces for surface impoundments
  - Stage-discharge, stage-area, and evaporation/infiltration update surfaces
  - Sedimentation/deposition state and effluent concentration surfaces by particle size class
consumer_scope:
  - Watershed/channel routing consumers receiving impoundment outflow hydrograph terms
  - Sediment-routing consumers receiving effluent concentration and sediment-mass terms
  - Comparator/replay consumers using contract confidence signals for watershed investigations
evidence_level: Static
last_reviewed: 2026-05-20
supersedes: []
superseded_by: []
---

# SC-IMPOUND-001 Surface Impoundment Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for process-based surface impoundment
hydraulic routing, stage-storage geometry, evaporation/infiltration stage updates,
and sediment trapping/effluent behavior for openWEPP watershed simulations.

## Scientific Scope

In scope:
- Daily hydraulic routing from inflow hydrograph terms through outlet-structure
  stage-discharge relations.
- Stage-area/stage-length functional geometry and adaptive-time-step routing
  behavior used in continuity integration.
- Daily evaporation/infiltration stage adjustments at impoundment scale.
- Sediment mass continuity, deposition, quiescent settling, and effluent
  concentration behavior by sediment class/subclass.
- Producer/consumer boundary obligations linking hillslope/channel inputs to
  impoundment outputs used by downstream routing/sediment accounting.

Out of scope:
- Kernel implementation details and Rust API layout.
- Design optimization or structural safety certification for spillways, culverts,
  or embankments.
- Reservoir operations beyond the Chapter-14 process scope (e.g., managed gate
  operations or optimization).
- Non-impoundment channel erosion physics owned by `SC-ROUTE-001` and
  `SC-SED-001`.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-IMPOUND-CH14-HYDRO | `references/50201000/chap14.pdf` §14.2, Eq. [14.2.1]-[14.2.6] | Continuity expression, stage-form routing equation, and Runge-Kutta stage update semantics. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-ADAPT | `references/50201000/chap14.pdf` §14.2.3, Eq. [14.2.7]-[14.2.9] | Adaptive time-step behavior, error-triggered step changes, and regime-transition retry semantics. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-STAGEDISCH | `references/50201000/chap14.pdf` §14.3, Eq. [14.3.1]-[14.3.18], Table 14.3.1 | Outlet-structure flow regimes, controlling-flow rules, and overall outflow summation. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-STAGEAREA | `references/50201000/chap14.pdf` §14.4, Eq. [14.4.1] | Stage-area power-function definition used by continuity integration. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-EI | `references/50201000/chap14.pdf` §14.5, Eq. [14.5.1]-[14.5.3] | Daily evaporation/infiltration losses and post-day stage adjustment. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-SEDMASS | `references/50201000/chap14.pdf` §14.6.1, Eq. [14.6.1]-[14.6.2] | Sediment mass continuity and effluent concentration update per time step. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-DEP | `references/50201000/chap14.pdf` §14.6.2, Eq. [14.6.3]-[14.6.11], Tables 14.6.1-14.6.3 | Overflow-rate deposition, detention-time relations, dead-storage logic, and calibration-coefficient usage. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-QUIESCENT | `references/50201000/chap14.pdf` §14.6.3, Eq. [14.6.12]-[14.6.15] | No-flow quiescent settling and concentration update semantics. | `[DIRECT][Static]` |
| REF-IMPOUND-CH14-UNITS | `references/50201000/chap14.pdf` §14.1 text (unit-conversion note) | Chapter-14 internal equations use mixed English/SI units and require explicit boundary conversion semantics. | `[DIRECT][Static]` |
| REF-IMPOUND-CH13-COUPLING | `references/50201000/chap13.pdf` §13.1-§13.2 watershed pass-file and inflow definitions | Required incoming runoff/sediment payloads and watershed routing coupling surfaces. | `[DIRECT][Static]` |
| REF-IMPOUND-CH1-SYSTEM | `references/50201000/chap1.pdf` model overview text (watershed + impoundment integration) | System-level role of impoundment deposition/outflow within watershed simulations. | `[DIRECT][Static]` |
| REF-IMPOUND-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative area/volume/mass expectations and no-creation accounting constraints. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `H`, `Hnew` | `ft` | Water-surface stage before/after routing/time-step or daily E/I update. | impoundment hydraulic routine | outlet-flow and sediment routines |
| `Aimp` | `ft^2` | Impoundment area from stage-area function. | stage-area routine | continuity routing and overflow-rate terms |
| `V`, `Vn`, `Vol` | `ft^3` | Impoundment volume at start/end/average over step. | hydraulic routing routine | sediment continuity and detention-time routines |
| `Qi`, `Qo`, `Qtotal` | `ft^3 s^-1` | Inflow, total outflow, and routing discharge surface terms. | watershed coupling + outlet routing | downstream channel/routing consumers |
| `Qdrop spillway`, `Qperforated riser`, `Qculvert`, `Qopen channel`, `Qrock fill`, `Qfilter fence`, `Qstraw bale` | `ft^3 s^-1` or `m^3 s^-1` (rock-fill branch) | Structure-specific outflow contributions before total summation. | outlet-structure routines | total outflow assembler |
| `Δt`, `Δtnext`, `Emax`, `Emin` | `s`, `s`, `ft`, `ft` | Adaptive time-step and numerical-error control variables. | adaptive integrator | hydraulic routing control flow |
| `a`, `b`, `c` | `varies` | Stage-area power-function coefficients in Eq. [14.4.1]. | front-end regression routine | continuity routing |
| `evap`, `infil`, `PET`, `Ksat` | `mm d^-1`, `mm d^-1`, `mm d^-1`, `mm h^-1` | Daily evaporation/infiltration inputs and losses for stage update. | climate/soil coupling + impound routine | daily stage update |
| `Tday` | `h` | Daily duration used in infiltration-loss relation (`Tday = 24 h`). | daily stage-loss routine | infiltration update branch |
| `Ci`, `Co`, `Con`, `Cavg`, `Cset` | `lbs ft^-3` | Inflow, start-step, end-step, averaged, and quiescent-layer sediment concentrations. | watershed sediment input + sediment routine | downstream sediment consumers |
| `Dep`, `dDep/dt`, `M`, `dM/dt` | `lbs`, `lbs s^-1`, `lbs`, `lbs s^-1` | Deposited sediment and sediment-mass continuity terms. | sediment routine | sediment budget and diagnostics |
| `Vs`, `Vc` | `ft s^-1` | Settling velocity and overflow-rate velocity. | sediment/geometry routines | deposition relations |
| `tD`, `tD100`, `Ct`, `cd`, `DS` | `s`, `s`, `dimensionless`, `dimensionless`, `fraction` | Detention-time and calibration/dead-storage terms used in deposition formulations. | sediment routine | deposition and concentration update |
| `Vset` | `ft^3` | Sediment-laden volume used during quiescent-settling updates. | no-flow settling routine | concentration/deposition cap update |
| `L`, `aL`, `bL`, `cL` | `ft`, `varies` | Stage-length relation terms used for dead-storage classification (`L/W`). | geometry routine | sediment routine |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-IMPOUND-001 | Hydraulic continuity invariant: stage evolution must satisfy Eq. [14.2.5] derived from Eq. [14.2.1]-[14.2.4], with explicit residual tracking over each routing step. | hard-fail | REF-IMPOUND-CH14-HYDRO, REF-IMPOUND-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IMPOUND-002 | Geometry-domain invariant: stage-area relationship (`Aimp = a + bH^c`) must produce finite, non-negative area values over modeled stages, and area/stage ordering must not imply non-physical negative storage increments. | hard-fail | REF-IMPOUND-CH14-STAGEAREA, REF-IMPOUND-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IMPOUND-003 | Outflow-regime invariant: for every outlet structure, no-flow/flow/overtop branch transitions must follow Section 14.3 regime definitions, and structure outflow contributions must obey controlling-flow relations (minimum controls where specified) before total summation via Eq. [14.3.18]. | hard-fail | REF-IMPOUND-CH14-STAGEDISCH | `[DIRECT][Static]` |
| INV-IMPOUND-004 | Adaptive-integration invariant: Runge-Kutta stage update Eq. [14.2.6] and adaptive time-step rules Eq. [14.2.7]-[14.2.9] must be applied with explicit retry when regime transitions are crossed within a step. | hard-fail | REF-IMPOUND-CH14-HYDRO, REF-IMPOUND-CH14-ADAPT | `[DIRECT][Static]` |
| INV-IMPOUND-005 | Daily loss-update invariant: daily stage adjustment must apply Eq. [14.5.1]-[14.5.3] consistently (`evap = 0.7 PET`, `infil = Ksat Tday`) and preserve signed stage-delta semantics from Eq. [14.5.3] (`ΔH = -(evap - infil)/304.8`; stage falls when `evap > infil`, rises when `infil > evap`), with explicit handling that prevents non-physical stage underflow. | hard-fail | REF-IMPOUND-CH14-EI, REF-IMPOUND-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IMPOUND-006 | Sediment continuity invariant: sediment concentration update at each time step must satisfy Eq. [14.6.1]-[14.6.2] and cannot emit negative concentrations or deposition exceeding available suspended mass for each subclass. | hard-fail | REF-IMPOUND-CH14-SEDMASS, REF-IMPOUND-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IMPOUND-007 | Deposition-regime invariant: inflow-period deposition uses Eq. [14.6.10] and post-inflow deposition uses Eq. [14.6.11], with `tD/tD100` semantics consistent with Eq. [14.6.6]-[14.6.8] and dead-storage treatment from Section 14.6.2. | hard-fail | REF-IMPOUND-CH14-DEP | `[DIRECT][Static]` |
| INV-IMPOUND-008 | Quiescent-settling invariant: no-flow periods must use Eq. [14.6.12]-[14.6.15], including the deposition cap `DEP <= Cset Vset` via Eq. [14.6.14]. | hard-fail | REF-IMPOUND-CH14-QUIESCENT | `[DIRECT][Static]` |
| INV-IMPOUND-009 | Coupling-payload invariant: impoundment inputs and outputs must carry required runoff/sediment payloads (hydrograph terms plus sediment-class concentrations/masses) with declared units before routing handoff is considered valid. | hard-fail | REF-IMPOUND-CH13-COUPLING, REF-IMPOUND-CH1-SYSTEM | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IMPOUND-010 | Unit-governance invariant: Chapter-14 mixed-unit equations (English + selected metric branches) require explicit boundary conversion declarations; claims of contract closure without conversion mapping are non-promotable. | governance-fail | REF-IMPOUND-CH14-UNITS | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-IMPOUND-001` | runtime | continuity residual calculator in hydraulic step loop | Typed hard error on residual/domain failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IMPOUND-002` | runtime | stage-area function validator | Typed hard error on non-finite/non-physical area outputs | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IMPOUND-003` | runtime | outlet-structure branch evaluator + total-outflow assembler | Typed hard error on invalid regime transition or controlling-flow violation | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-IMPOUND-004` | runtime | Runge-Kutta/adaptive-step controller | Typed hard error on invalid step adaptation or missed regime-transition retry | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-IMPOUND-005` | runtime | daily stage-loss updater | Typed hard error on invalid daily loss update or stage underflow | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IMPOUND-006` | runtime | sediment mass-balance/concentration update routine | Typed hard error on negative concentration or over-deposition | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IMPOUND-007` | runtime | deposition branch selector and detention-time calculator | Typed hard error on wrong branch application or invalid detention-time domain | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-IMPOUND-008` | runtime | no-flow quiescent-settling routine | Typed hard error on quiescent-settling cap/domain violations | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-IMPOUND-009` | runtime | impoundment input/output payload validator | Typed hard error on missing or malformed coupling payloads | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IMPOUND-010` | governance | review/disposition/verification promotion checklist | Promotion `HOLD` until unit-conversion alias mapping is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols use Chapter-14 notation. Boundary/API symbol names are
provisional and remain identity-mapped until openWEPP runtime surfaces finalize.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `H`, `Hnew`, `Aimp`, `V`, `Vn`, `Vol` | identity names | hydraulic state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qi`, `Qo`, `Qtotal` | identity names | hydrograph inflow/outflow surfaces | `ft^3 s^-1` preserved | `[DIRECT][Static]` |
| `Qdrop spillway`, `Qperforated riser`, `Qculvert`, `Qopen channel`, `Qrock fill`, `Qfilter fence`, `Qstraw bale` | identity names | structure-flow contribution surfaces | structure-specific units preserved per Chapter 14 | `[DIRECT][Static]` |
| `Δt`, `Δtnext`, `Emax`, `Emin` | identity names | adaptive-step control surfaces | `s` and `ft` preserved | `[DIRECT][Static]` |
| `a`, `b`, `c`, `aL`, `bL`, `cL` | identity names | stage-area and stage-length coefficient surfaces | coefficient-unit semantics preserved | `[DIRECT][Static]` |
| `evap`, `infil`, `PET`, `Ksat` | identity names | daily loss-update surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Tday` | identity name | daily infiltration-duration control surface | `h` preserved | `[DIRECT][Static]` |
| `Ci`, `Co`, `Con`, `Cavg`, `Cset`, `Dep`, `M` | identity names | sediment continuity and deposition surfaces | sediment units preserved | `[DIRECT][Static]` |
| `dDep/dt`, `dM/dt` | identity names | sediment-rate continuity surfaces | `lbs s^-1` preserved | `[DIRECT][Static]` |
| `Vs`, `Vc`, `tD`, `tD100`, `Ct`, `cd`, `DS` | identity names | detention/deposition control surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Vset` | identity name | no-flow sediment-laden volume surface | `ft^3` preserved | `[DIRECT][Static]` |
| `L` | identity name | stage-length/dead-storage classification surface | `ft` preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-flow period with storage | `Qi = 0`, all outlet flows at no-flow regime, stage still positive. | Explicit Section 14.3 no-flow branches and Section 14.6.3 quiescent settling behavior. | `[DIRECT][Static]` |
| Partial structure activation | Some outlet structures contribute flow while others remain below inlet stage. | Eq. [14.3.18] sums active structure contributions only. | `[DIRECT][Static]` |
| Full trapping for a subclass | `Vs >= Vc` leading to near-100% trapping for that subclass during a step. | Overflow-rate concept in Eq. [14.6.3]-[14.6.6]. | `[DIRECT][Static]` |
| Inflow without overtopping | Inflow routed entirely through active low-stage structures with no overtopping branches triggered. | Valid multi-regime behavior in Section 14.3. | `[DIRECT][Static]` |
| Sediment-free inflow day | Hydraulic inflow with `Ci = 0` for one or more classes/subclasses. | Mass continuity remains valid with zero sediment-source term. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Non-finite or negative `Aimp`, `Vol`, or sediment mass terms that violate declared domains. `[DIRECT][Static] + [INFERENCE][Static]`
- Outflow contribution from a structure when `H` is below that structure's no-flow threshold. `[DIRECT][Static]`
- Skipping adaptive-step retry when a step crosses an outflow-regime transition. `[DIRECT][Static]`
- Negative effluent concentration (`Con < 0`) or deposition exceeding available suspended mass for a subclass. `[DIRECT][Static] + [INFERENCE][Static]`
- Daily stage update that applies Eq. [14.5.*] branches inconsistently or produces non-physical stage underflow without explicit failure. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required hydrograph/sediment payload terms at impoundment input/output boundaries. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-IMPOUND-P-001: Emit all hydraulic routing terms (`H`, `Aimp`, `Qi`, `Qo`, `Qtotal`) and structure-flow contributions with declared units and branch provenance. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IMPOUND-P-002: Enforce continuity, regime-transition, and adaptive-step invariants before publishing outflow terms. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IMPOUND-P-003: Emit sediment subclass continuity surfaces (`Ci`, `Co`, `Con`, `Dep`, `Vs`, `Vc`, `tD`, `tD100`) with explicit no-flow/quiescent branch handling. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IMPOUND-P-004: Propagate invariant failures as typed errors; no silent clamping/defaulting of stage, outflow, or sediment concentrations. `[INFERENCE][Static]`

## Consumer Obligations

- OBL-IMPOUND-C-001: Routing consumers must preserve impoundment outflow terms and declared units without implicit reinterpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IMPOUND-C-002: Sediment consumers must treat subclass/class concentration outputs as mass-conserving boundaries and reject malformed payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IMPOUND-C-003: Watershed-closure consumers must include impoundment storage/outflow effects explicitly in daily/event accounting and report closure residuals when available. `[INFERENCE][Static]`
- OBL-IMPOUND-C-004: Consumers must fail explicitly on invariant-tagged payload violations and retain invariant IDs in diagnostics. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Hydraulic continuity and geometry (`INV-IMPOUND-001/002`) | hydraulic step loop | Hard error; stop impoundment-step publish | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Outflow-regime and adaptive-step behavior (`INV-IMPOUND-003/004`) | outlet-branch evaluation + integrator control | Hard error on branch/transition violation | Tier-B investigation gate | `[DIRECT][Static]` |
| Daily stage-loss update (`INV-IMPOUND-005`) | daily evaporation/infiltration adjustment | Hard error on domain/branch failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment continuity/deposition/no-flow settling (`INV-IMPOUND-006/007/008`) | sediment update loop per subclass | Hard error on mass/branch/domain violation | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling payload completeness (`INV-IMPOUND-009`) | impoundment input/output boundary checks | Hard error on missing/malformed payload | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Unit-governance mapping (`INV-IMPOUND-010`) | review/disposition/promotion | Governance `HOLD` until conversion/alias evidence is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` semantic-parity policy.
Contract-level tolerances for comparator interpretation:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-IMPOUND-001 | Hydraulic continuity residual tolerance for Eq. [14.2.5] step updates | `<= 1e-6 ft` | Comparator/noise allowance only; runtime still hard-fails on material violations. | `[INFERENCE][Static]` |
| TOL-IMPOUND-002 | Stage/area/volume non-negative-domain comparator bound | lower bound `>= -1e-9` in native units | No silent runtime clamping permitted. | `[INFERENCE][Static]` |
| TOL-IMPOUND-003 | Sediment concentration non-negative comparator bound (`Ci`, `Co`, `Con`) | lower bound `>= -1e-12 lbs ft^-3` | Negative values beyond tolerance are invalid-state failures. | `[INFERENCE][Static]` |
| TOL-IMPOUND-004 | No-flow discharge threshold for regime validation | `Qo <= 1e-9 ft^3 s^-1` treated as numerical zero | Applies only at declared no-flow branches. | `[INFERENCE][Static]` |
| TOL-IMPOUND-005 | Regime-transition stage comparison tolerance | `abs(H - Htransition) <= 1e-6 ft` | Prevents false branch flips from roundoff near transition stages. | `[INFERENCE][Static]` |

## WS10 Watershed Production-Kernel Addendum

### WS10 Runtime Boundary Symbols

| Surface | Symbols |
|---|---|
| Impoundment per-node controls | `ws10_impoundment_{id}_h`, `ws10_impoundment_{id}_hfull`, `ws10_impoundment_{id}_deltat`, `ws10_impoundment_{id}_qinf` |
| Upstream dependency payloads | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout` |
| Contributor peak payloads | `hs{ID}_peakro`, `hs{ID}_watdur` |
| Impoundment published outputs | `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`, `ws10_impoundment_{id}_outflow_volume` |

### WS10 Coupling Rules

1. WS10 impoundment production execution consumes parser-projected impoundment
   controls plus routed upstream payloads from dependency nodes.
2. Impoundment execution must hard-fail on missing/non-finite required symbols
   and on invalid storage/headroom domains (including `h > hfull` and
   non-positive `deltat`).
3. Overflow/retention branch behavior must be explicit and deterministic;
   silent default replacement and silent domain clamping are prohibited.
4. Published impoundment outputs must be finite and non-negative for discharge
   and routed outflow volume.

### WS10 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `WKERNEL-WS10-IMPOUNDMENT-E-001` |
| Non-finite symbol | `WKERNEL-WS10-IMPOUNDMENT-E-002` |
| Domain/dependency violation | `WKERNEL-WS10-IMPOUNDMENT-E-003` |

### WS10 Contract-Derived Test Vectors

Minimum WS10 impoundment conformance vectors:
1. Nominal impoundment execution with finite parser-projected controls and
   routed upstream payloads emits finite non-negative
   `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`,
   `ws10_impoundment_{id}_outflow_volume`.
2. Missing required impoundment control symbol fails with
   `WKERNEL-WS10-IMPOUNDMENT-E-001`.
3. Non-finite required symbol fails with
   `WKERNEL-WS10-IMPOUNDMENT-E-002`.
4. Domain/dependency violation (e.g., `h > hfull`, invalid `deltat`, or missing
   upstream dependency payload) fails with
   `WKERNEL-WS10-IMPOUNDMENT-E-003`.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production impoundment interfaces must use typed ARCH22 symbol
   surfaces (`WatershedProductionStateSymbol`,
   `WatershedProductionFluxSymbol`) for boundary-state/flux resolution.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Node-scoped impoundment symbol families (`h`, `hfull`, `deltat`, `qinf`,
   `qo`, `durout`, `hnext`, `outflow_volume`) must be resolved through typed
   node/field builders.
4. Typed migration must preserve WS10 impoundment guard-family continuity
   (`WKERNEL-WS10-IMPOUNDMENT-E-001..003`) and hard-fail behavior.

### Contract-Derived Migration Vectors

1. Static migration proof: covered impoundment production accessors use typed
   symbol families, not stringly `&str` parameters.
2. Nominal migration vector: impoundment execution preserves deterministic
   output/state publication under typed symbol resolution.
3. Failure migration vectors: missing/non-finite/domain/dependency violations
   preserve existing typed boundary classes and WS10 guard IDs.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-IMPOUND-001 | Chapter-14 calibration models for `Ct`/`cd` are legacy-derived from CSTRS-generated datasets and pilot-scale studies; openWEPP has not yet completed dedicated revalidation for its target scenarios. | Sediment-trapping confidence is provisional for full production claims. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-IMPOUND-002 | Concrete openWEPP boundary/API names and conversion carriers for full mixed-unit Chapter-14 symbol families are not yet fixed. | WS10 production path pins initial runtime aliases (`ws10_impoundment_*`) but complete mixed-unit alias/conversion closure remains incomplete. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-IMPOUND-003 | Coupled canonical contracts (`SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`) are not yet all at draft `in_review` maturity. | Cross-contract closure of routing/sediment ownership boundaries remains provisional. | non-promotable | `[DIRECT][Static]` |
| GAP-IMPOUND-004 | Filter-fence/straw-bale outflow behavior depends on slurry/clogging assumptions that Chapter 14 flags as user-sensitive and not fully captured by current coefficients. | High-flow performance interpretation for those structure types retains elevated uncertainty. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-16 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-14/13/1 authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: completed symbol-alias continuity coverage, added evidence-tag columns for degenerate/tolerance sections, normalized evidence mode to `Static`, clarified Eq. [14.5.3] signed stage-delta semantics, and unified authority-path style. |
| `2026-05-23` | `3` | `Codex` | WS10 amendment: added watershed production-kernel impoundment runtime alias surfaces (`ws10_impoundment_*` + dependency payloads), typed WS10 impoundment guard family (`WKERNEL-WS10-IMPOUNDMENT-E-001..003`), and contract-derived WS10 impoundment test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered WS10 impoundment interfaces to consume boundary symbols via ARCH22 typed symbol families and node-scoped builders while preserving WS10 guard-family continuity. |
