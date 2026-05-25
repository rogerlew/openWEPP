---
contract_id: SC-CLIMATE-001
title: Climate Forcing Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 10
producer_scope:
  - Weather-generator forcing surfaces (daily precipitation occurrence/amount)
  - Storm disaggregation forcing surfaces (duration, intensity distribution)
  - Climate boundary inputs used by winter, runoff partition, ET, and irrigation coupling
consumer_scope:
  - Winter hydrology forcing consumers
  - Runoff partition and infiltration forcing consumers
  - Water-balance and irrigation event-coupling consumers
evidence_level: static
last_reviewed: 2026-05-25
supersedes: []
superseded_by: []
---

# SC-CLIMATE-001 Climate Forcing Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for climate/weather forcing generation and
forcing payload boundaries consumed by downstream openWEPP hydrology and
erosion domains.

## Scientific Scope

In scope:
- Daily precipitation occurrence and wet-day precipitation amount generation.
- Storm descriptor and storm-disaggregation semantics used to build breakpoint
  rainfall intensity patterns.
- Climate forcing payload completeness for snow/freeze, runoff partition,
  water-balance/ET, and irrigation coupling.

Out of scope:
- Kernel implementation details and data structure layout.
- Non-climate domains except required forcing boundary definitions.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-CLIMATE-CH2-OCC | `references/50201000/chap2.pdf` §2.1.1, Eq. [2.1.1]-[2.1.4] | Two-state wet/dry Markov occurrence semantics and probability complement rules. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-AMT | `chap2.pdf` §2.1.2, Eq. [2.1.5] | Wet-day precipitation amount generation semantics. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-STORM | `chap2.pdf` §2.1.3-§2.1.4, Eq. [2.1.6]-[2.1.9] | Storm duration, peak intensity, and time-to-peak distribution semantics. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-SOLAR | `chap2.pdf` §2.1.6, Eq. [2.1.12]-[2.1.13] | Solar-radiation generation and required min/max bounds. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-BRKPT | `chap2.pdf` §2.2, Eq. [2.2.1]-[2.2.2], Table 2.2.1 convention text | Breakpoint rainfall format and start/end intensity conventions. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-DISAG | `chap2.pdf` §2.2.1-§2.2.2, Eq. [2.2.3]-[2.2.12] | Normalized disaggregation and restoration of dimensional storm quantities. | `[DIRECT][Static]` |
| REF-CLIMATE-CH2-LIMIT | `chap2.pdf` §2.1.4 and §2.2.3 notes | Model-limit statements (tentative equations, multi-storm/day future work). | `[DIRECT][Static]` |
| REF-CLIMATE-CH3-COUPLING | `references/50201000/chap3.pdf` §3.1-§3.2 | Winter routine consumes daily climate forcing and derives hourly precipitation/temperature/radiation timing. | `[DIRECT][Static]` |
| REF-CLIMATE-CH4-COUPLING | `references/50201000/chap4.pdf` §4.2 and §4.4 validation note | Runoff partition/infiltration consumes breakpoint rainfall intensities. | `[DIRECT][Static]` |
| REF-CLIMATE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1-§5.2 | Water-balance/ET consumes generated precipitation/temperature/radiation/wind/dew-point forcing. | `[DIRECT][Static]` |
| REF-CLIMATE-CH12-COUPLING | `references/50201000/chap12.pdf` §12.2.1 | Irrigation concurrent-event logic combines irrigation with Chapter-2 rainfall disaggregation. | `[DIRECT][Static]` |
| REF-CLIMATE-PHYS-BOUNDS | Physical/common-sense invariant class | Probability bounds and non-negative rainfall/intensity domains are required for physical validity. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `α` (`alpha`) | `fraction` | Conditional probability of wet day following dry day (`P(W|D)`). | climate generator | occurrence transition logic |
| `β` (`beta`) | `fraction` | Conditional probability of dry day following wet day (`P(D|W)`). | climate generator | occurrence transition logic |
| `P(W\\|D)` | `fraction` | Wet-day transition probability given prior dry day. | occurrence transition logic | wet/dry day sampler |
| `P(D\\|D)` | `fraction` | Dry-day transition probability given prior dry day. | occurrence transition logic | wet/dry day sampler |
| `P(D\\|W)` | `fraction` | Dry-day transition probability given prior wet day. | occurrence transition logic | wet/dry day sampler |
| `P(W\\|W)` | `fraction` | Wet-day transition probability given prior wet day. | occurrence transition logic | wet/dry day sampler |
| `P` | `mm` | Storm precipitation amount. | climate generator | disaggregation and downstream hydrology |
| `X` | `mm` | Generated wet-day precipitation amount raw variate. | climate generator | daily forcing payload |
| `D` | `h` | Storm duration. | climate generator | disaggregation and downstream hydrology |
| `rp` | `mm h^-1` | Peak storm intensity. | climate generator | disaggregation and downstream hydrology |
| `Dp` | `h` | Time from storm start to peak intensity. | climate generator | time-to-peak distribution logic |
| `De` | `h` | Effective precipitation duration (inter-storm zero-rain periods removed). | climate generator | time-to-peak distribution logic |
| `Ak` | `fraction` | Accumulated class frequency for time-to-peak distribution classes. | climate generator | time-to-peak sampler |
| `Nk` | `count` | Number of storms in class interval `k`. | climate generator | accumulated frequency construction |
| `N` | `count` | Total number of storms in station record used for accumulation. | climate generator | accumulated frequency normalization |
| `tp` | `fraction` | Normalized time to peak (`Dp/D`). | climate generator | disaggregation function |
| `ip` | `fraction` | Ratio of peak to average rainfall intensity (`rp/ib`). | climate generator | disaggregation function |
| `RA` | `Ly` | Generated daily solar radiation. | climate generator | ET and snow/freeze forcing |
| `RAmax` | `Ly` | Maximum possible solar radiation for day-of-year/station. | climate generator | radiation bound logic |
| `Tmax` | `degC` | Generated daily maximum air temperature. | climate generator | snow/rain partition and ET forcing |
| `Tmin` | `degC` | Generated daily minimum air temperature. | climate generator | snow/rain partition and ET forcing |
| `Tdp` | `degC` | Generated daily dew-point temperature. | climate generator | ET and energy-balance forcing |
| `W` | `direction-class` | Generated wind direction sampled from accumulated distribution. | climate generator | snow/freeze and ET forcing |
| `u_w` | `m s^-1` | Generated daily wind speed conditioned on direction class. | climate generator | snow/freeze and ET forcing |
| `T_i` | `min` (dimensional) or `fraction` (normalized) | Breakpoint cumulative storm time sequence. | disaggregation routine | runoff/infiltration consumers |
| `I_i` | `mm h^-1` (dimensional) or `fraction` (normalized) | Breakpoint average intensity sequence. | disaggregation routine | runoff/infiltration consumers |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-CLIMATE-001 | Occurrence-probability boundedness and complement closure: `0 <= alpha <= 1`, `0 <= beta <= 1`, `P(W|D)=alpha`, `P(D|D)=1-alpha`, `P(D|W)=beta`, and `P(W|W)=1-beta`. | hard-fail | REF-CLIMATE-CH2-OCC, REF-CLIMATE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-002 | Wet-day precipitation amount domain: generated wet-day precipitation amount is non-negative, and precipitation phase-partition preconditions use daily temperature conditions exactly as described for snow/rain path selection. | hard-fail | REF-CLIMATE-CH2-AMT, REF-CLIMATE-CH3-COUPLING, REF-CLIMATE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-003 | Storm descriptor domain constraints: for generated events with `P > 0`, enforce `D > 0`, `rp >= 0`, and normalized time-to-peak `tp` in `[0,1]`; accumulated class frequency used for time-to-peak sampling is monotone nondecreasing and bounded in `[0,1]`. | hard-fail | REF-CLIMATE-CH2-STORM, REF-CLIMATE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-004 | Breakpoint rainfall convention (storm-event scoped): for generated events where `P > 0`, intensity sequence begins with a non-zero event intensity at time `0` and ends with `0` intensity at storm end; cumulative time and cumulative depth sequences are nondecreasing. | hard-fail | REF-CLIMATE-CH2-BRKPT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-005 | Disaggregation closure: normalized pattern has duration `1.0` and unit area; dimensional restoration preserves input storm depth `P` and duration `D` (subject to discretization tolerance). | hard-fail | REF-CLIMATE-CH2-DISAG | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-006 | Radiation bound invariant: generated `RA` satisfies `0.05 * RAmax <= RA <= RAmax`. | hard-fail | REF-CLIMATE-CH2-SOLAR | `[DIRECT][Static]` |
| INV-CLIMATE-007 | Coupling payload completeness: forcing payloads needed by Chapter-3 hourly winter processes, Chapter-4 runoff partition, Chapter-5 water-balance/ET, and Chapter-12 irrigation concurrent-event logic must be emitted in required units and sequence completeness. | hard-fail | REF-CLIMATE-CH3-COUPLING, REF-CLIMATE-CH4-COUPLING, REF-CLIMATE-CH5-COUPLING, REF-CLIMATE-CH12-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-CLIMATE-008 | Explicit model-limit governance invariant: depth-duration-frequency sensitivity limitations, tentative duration/peak equations, and unresolved multi-storm-per-day behavior must remain explicit and cannot be silently treated as closed science. | governance-fail | REF-CLIMATE-CH2-LIMIT | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-CLIMATE-001` | runtime | Occurrence transition assembly | Typed hard error on out-of-range or broken complement rule | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-002` | runtime | Wet-day amount generation and phase-precondition checks | Typed hard error on negative amount or invalid phase precondition | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-003` | runtime | Storm descriptor/time-to-peak pipeline | Typed hard error on invalid duration/intensity/domain | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-004` | runtime | Breakpoint array builder/validator | Typed hard error on broken breakpoint convention or non-monotone arrays | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-005` | runtime | Disaggregation normalization + restore step | Typed hard error on failed closure beyond tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-006` | runtime | Daily radiation bound check | Typed hard error on out-of-bound radiation value | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-CLIMATE-007` | runtime | Forcing boundary payload validator | Typed hard error on missing/invalid forcing field | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-CLIMATE-008` | governance | Review/disposition/verification + promotion checklist | Promotion `HOLD` until limitation scope/risk is explicitly dispositioned | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-2 WEPP notation and legacy
WEPP lineage names by default. For this revision, no divergent openWEPP
boundary/API field names are declared for these forcing surfaces; boundary names
are constrained to canonical symbols until downstream implementation contracts
introduce explicit aliases.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `α` (`alpha`) | `α`/`alpha` (identity) | daily occurrence transition surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `β` (`beta`) | `β`/`beta` (identity) | daily occurrence transition surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `P(W\\|D)`, `P(D\\|D)`, `P(D\\|W)`, `P(W\\|W)` | identity expressions | conditional transition payload semantics | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `P` | `P` (identity) | storm descriptor payload | `mm` -> `mm` | `[DIRECT][Static]` |
| `X` | `X` (identity) | wet-day precipitation raw variate surface | `mm` -> `mm` | `[DIRECT][Static]` |
| `D` | `D` (identity) | storm descriptor payload | `h` -> `h` | `[DIRECT][Static]` |
| `rp` | `rp` (identity) | storm descriptor payload | `mm h^-1` -> `mm h^-1` | `[DIRECT][Static]` |
| `Dp`, `De` | `Dp`, `De` (identity) | time-to-peak descriptor payload | `h` -> `h` | `[DIRECT][Static]` |
| `Ak`, `Nk`, `N` | `Ak`, `Nk`, `N` (identity) | accumulated class-frequency surfaces | `fraction/count` -> `fraction/count` | `[DIRECT][Static]` |
| `tp`/`ip` | `tp`/`ip` (identity) | normalized storm-shape payload | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `T_i` | `T_i` (identity) | breakpoint time sequence | `min` or normalized fraction by mode | `[DIRECT][Static]` |
| `I_i` | `I_i` (identity) | breakpoint intensity sequence | `mm h^-1` or normalized fraction by mode | `[DIRECT][Static]` |
| `RA`, `RAmax` | `RA`, `RAmax` (identity) | daily radiation forcing payload | `Ly` -> `Ly` | `[DIRECT][Static]` |
| `Tmax`, `Tmin`, `Tdp` | `Tmax`, `Tmin`, `Tdp` (identity) | daily thermodynamic forcing payload | `degC` -> `degC` | `[DIRECT][Static]` |
| `W`, `u_w` | `W`, `u_w` (identity) | daily wind forcing payload | direction class + speed unchanged | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Dry day | Generated day classified dry, no wet-day precipitation amount emitted (`P = 0` daily event equivalent). | Valid weather-sequence outcome under Markov occurrence. |
| Snow-only day | Precipitation present with `Tmax < 0 degC` and routed to snow pathway. | Explicitly described in climate/winter coupling rules. |
| Boundary low radiation day | `RA` at lower clamp `0.05 * RAmax`. | Explicit lower bound in Chapter 2 radiation generation. |
| No rainfall/irrigation concurrency day | Irrigation concurrent-event merge not invoked because no rainfall event exists. | Normal operating mode in Chapter 12 logic. |

## Invalid States

- Any occurrence probability parameter outside `[0,1]`. `[DIRECT][Static] + [INFERENCE][Static]`
- Broken complement closure for wet/dry transition probabilities. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative wet-day precipitation amount, negative storm duration, or negative peak intensity. `[DIRECT][Static] + [INFERENCE][Static]`
- Breakpoint sequences with decreasing cumulative time/depth or missing required start/end intensity conventions. `[DIRECT][Static] + [INFERENCE][Static]`
- Disaggregation restoration that materially fails to preserve input `P` and `D`. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required forcing payload fields for downstream chapter consumers. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-CLIMATE-P-001: Emit climate forcing surfaces with canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-CLIMATE-P-002: Enforce all `INV-CLIMATE-*` runtime guards before boundary publish. `[INFERENCE][Static]`
- OBL-CLIMATE-P-003: Surface typed errors for invalid probability/storm/disaggregation domains; no silent defaulting. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-CLIMATE-P-004: Keep model-limit assumptions explicit in contract and disposition artifacts. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-CLIMATE-C-001: Winter, runoff, ET, and irrigation consumers must reject malformed forcing payloads explicitly. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-CLIMATE-C-002: Consumers must preserve forcing units and sequence semantics (daily vs breakpoint/hours) without silent reinterpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-CLIMATE-C-003: Consumers must propagate invariant-violation context to orchestrator-level typed errors. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Occurrence and storm domains (`INV-CLIMATE-001/002/003`) | climate generation stage before payload emission | Hard error, payload rejected, invariant ID logged | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Breakpoint and disaggregation closure (`INV-CLIMATE-004/005`) | breakpoint/disaggregation output validation | Hard error on convention/closure failure beyond tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Radiation domain (`INV-CLIMATE-006`) | radiation generation step | Hard error on out-of-range radiation | Tier-A/B gate | `[DIRECT][Static]` |
| Coupling completeness (`INV-CLIMATE-007`) | forcing boundary handoff | Hard error on missing/invalid field or sequence | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Model-limit governance (`INV-CLIMATE-008`) | review/verification/promotion check | Governance `HOLD` until explicit limitation disposition | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific interpretation tolerances:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-CLIMATE-001 | Probability-domain tolerance for `alpha`, `beta`, and derived complements | `abs(bound violation) <= 1e-12` for comparator interpretation only | Runtime still fails explicitly when materially out-of-domain. |
| TOL-CLIMATE-002 | Storm-depth closure residual after disaggregation restore | `<= 1e-10 mm` | Applies to restore step from normalized to dimensional sequence. |
| TOL-CLIMATE-003 | Storm-duration closure residual after disaggregation restore | `<= 1e-10 h` | Applies to breakpoint sequence end-time consistency. |
| TOL-CLIMATE-004 | Monotonicity tolerance for cumulative breakpoint time/depth sequences | nondecreasing within `1e-12` numeric noise | Negative steps beyond tolerance are invalid-state failures. |

## WB14 Hyetograph-Coupling Addendum

### WB14 Runoff-Coupling Forcing Requirements

For event days consumed by runoff reconciliation:

1. The runtime climate payload must expose point-count metadata (`ninten` for
   no-breakpoint forcing, `nbrkpt` for breakpoint forcing) and the matching
   `timem_####` / `intsty_####` series.
2. `timem_####` values are elapsed-seconds offsets from event start and must be
   strictly increasing for indexed points.
3. `intsty_####` values are interval rainfall intensities (`m s^-1`), finite,
   and non-negative; terminal zero-intensity endpoint semantics are retained.
4. Forcing surfaces must remain compatible with Chapter-4 Green-Ampt lineage
   infiltration consumers without implicit unit conversion.

### WB14 Typed-Guard Alignment

Malformed hyetograph payloads at runoff-consumer boundary (missing symbols,
non-finite values, non-monotone times, or negative intensities) are invalid
states and must hard-fail through typed consumer guard posture (`HKERNEL-WB14-RUNOFF-E-00x`).

### WB14 Contract-Test Vectors

1. Valid climate forcing projects deterministic hyetograph symbol families into
   runtime surfaces for runoff consumers.
2. Missing required hyetograph symbol family fails at runoff consumer boundary
   with typed missing-input posture.
3. Non-monotone breakpoint/disaggregation time sequence or negative intensity
   fails with typed domain posture and no fallback re-assembly.

## CLIM05 Snow-Control Runtime Coupling Addendum

### CLIM05 Required Snow-Coupling Surfaces

| Surface | Symbols |
|---|---|
| Parsed snow-control payload | `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` |
| Climate partition drivers | `Tmax`, `Tmin`, hyetograph rainfall (`timem_####`, `intsty_####`) |
| Snow-coupled runtime state/output | `snow.runtime_swe`, `S` |

### CLIM05 Deterministic Coupling Requirements

1. When snow coupling is active (`snow.options.snow_file_present` projected at
   runtime seam), hydrology consumers must use `rst` as the rain/snow
   partition threshold for daily forcing handoff.
2. Snow accumulation/melt coupling must publish signed daily snow-water term
   `S` where `S = melt - accumulation` and retain non-negative runtime snow
   storage state (`snow.runtime_swe`).
3. Snow-coupled liquid input to runoff/water-balance closure must be explicit;
   no silent reinterpretation of malformed/missing snow controls is allowed.

### CLIM05 Typed-Guard Alignment

When CLIM05 snow coupling is active, missing/non-finite/out-of-domain
`snow.options.*` symbols or snow-state closure failures are invalid boundary
states and must hard-fail with typed hydrology guard posture.

### CLIM05 Contract-Test Vectors

1. Active snow coupling emits deterministic `S` and runtime `snow.runtime_swe`
   with valid domain closure.
2. Missing required active-coupling snow control symbol hard-fails with typed
   missing-input posture.
3. Non-finite/out-of-domain snow-control or snow-state values hard-fail with
   typed non-finite/domain posture and no fallback defaulting.

## CLIM06 Frost-Control Runtime Coupling Addendum

### CLIM06 Required Frost-Coupling Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost-control payload | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.ksnowf`, `frost.options.kresf`, `frost.options.ksoilf`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Climate freeze/thaw drivers | `Tmax`, `Tmin`, hyetograph rainfall (`timem_####`, `intsty_####`) |
| Frost-coupled runtime state/output | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |

### CLIM06 Deterministic Coupling Requirements

1. Frost coupling is active only when runtime seam projects
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. Climate-driven freeze/thaw branch selection for CLIM06 coupling is explicit
   (`Tmin <= 0 degC` freeze-active branch, `Tmin > 0 degC` thaw/inactive
   branch); no silent branch fallback is allowed.
3. Active CLIM06 coupling must publish bounded frozen-soil state surfaces
   (`Dfrost`, `Dthaw`, `Nft`, `Ws_frz`) and frozen infiltration-capacity
   surface (`InfCap_frz`) used by runoff/infiltration consumers.
4. Missing/non-finite/out-of-domain active-coupling frost controls or derived
   frost runtime surfaces are invalid boundary states and must hard-fail with
   typed hydrology guard posture.

### CLIM06 Contract-Test Vectors

1. Active frost coupling vector emits deterministic frozen-soil state surfaces
   and reduced infiltration-capacity coupling with valid runoff closure.
2. Missing required active-coupling frost symbol hard-fails with typed
   missing-input posture.
3. Non-finite active-coupling frost symbol hard-fails with typed non-finite
   posture.
4. Out-of-domain active-coupling frost symbol/state hard-fails with typed
   domain posture and no fallback defaulting.

## IRRIG10 Irrigation Scheduler Climate-Key Addendum

### IRRIG10 Required Climate-Key Surfaces

| Surface | Symbols |
|---|---|
| Daily schedule keys | `day`, `year` |
| Event forcing series | `timem_####`, `intsty_####`, `ninten`/`nbrkpt` |
| Irrigation-coupled runoff closure key | `wb12_rainfall_input` |

### IRRIG10 Deterministic Climate Coupling Requirements

1. Climate runtime seam must publish finite `day` and `year` keys for each
   simulated day consumed by irrigation fixed-date scheduling, with
   deterministic day-sequence progression across the full available climate
   run span.
2. Hyetograph forcing-series symbols remain authoritative forcing inputs under
   irrigation coupling; irrigation additions are coupled downstream without
   mutating climate-source parser records.
3. When irrigation coupling is active, rainfall-input closure remains explicit:
   `wb12_rainfall_input = wb14_hyetograph_rainfall + irrigation.runtime_depth_m`.
4. Missing/non-finite schedule keys or malformed forcing-series symbols are
   invalid downstream irrigation-coupled boundary states and must hard-fail via
   typed hydrology guard posture.

### IRRIG10 Contract-Test Vectors

1. Fixed-date irrigation vector consumes climate `day`/`year` and activates
   scheduled irrigation deterministically.
2. Depletion scheduling vector consumes climate day key with deterministic
   event-period gating.
3. Missing/non-finite climate schedule key symbols hard-fail with typed missing
   or non-finite guard posture at irrigation-coupled runoff phase.

## CLIM07 Comparator and Seam-Closure Addendum

### CLIM07 Required Comparator Vector Families

| Vector family | Mode | Primary invariants exercised |
|---|---|---|
| Continuous-daily runtime forcing vector | `ibrkpt=0` | `INV-CLIMATE-003`, `INV-CLIMATE-005`, `INV-CLIMATE-007` |
| Breakpoint runtime forcing vector | `ibrkpt=1` | `INV-CLIMATE-004`, `INV-CLIMATE-005`, `INV-CLIMATE-007` |
| Parser-to-kernel seam projection vector | hillslope + watershed assignment surfaces | `INV-CLIMATE-007` |
| Comparator confidence-tier routing vector | single-OFE daily vs hourly/watershed classes | governance guidance tied to ADR-0011 tier semantics |

### CLIM07 Deterministic Comparator Requirements

1. Continuous-daily vector checks must assert deterministic projection of
   required climate forcing symbols (`datver`, `iclig`, `itemp`, `ibrkpt`,
   `prcp`, `stmdur`, `timep`, `ip`, `ninten`, `timem_####`, `intsty_####`)
   at parser-to-kernel seams.
2. Breakpoint vector checks must assert deterministic projection of breakpoint
   forcing symbols (`stmstr`, `nbrkpt`, elapsed `timem_####`, interval
   `intsty_####`, `mxint`) and verify non-monotone/negative-event guards stay
   typed hard-fail.
3. Watershed assignment projection must preserve hillslope-scoped climate
   symbol namespaces (`hs{id}_*`) without silent omission/rewrite.
4. Comparator confidence-tier routing evidence must explicitly demonstrate:
   - single-OFE daily surfaces map to higher-confidence routing metadata,
   - hourly and watershed surfaces map to investigation-tier metadata,
   - missing required metadata remains typed hard-fail.

### CLIM07 Contract-Test Vectors

1. Continuous-daily comparator vector emits deterministic non-breakpoint
   forcing projections and disaggregation-derived series symbols with stable
   closure semantics.
2. Breakpoint comparator vector emits deterministic breakpoint timing/intensity
   projections (including elapsed-time conversion and peak-intensity closure)
   from curated breakpoint fixtures.
3. Parser-to-kernel seam vectors confirm both hillslope and watershed climate
   runtime surfaces publish required symbol families for accepted policy
   branches.
4. Confidence-tier reporting vectors verify deterministic routing metadata and
   typed missing-metadata failure behavior.

## CLIM08 Governance Closeout Addendum

### CLIM08 HOLD-Closure Mapping

1. Parser/runtime seam ownership closure is ratified through implemented
   CLIM02 seam authority and executable seam-guard behavior, with canonical
   parser/runtime boundary disposition tracked in
   `SC-INFILE-CLIMATE-001` (`CLI-GAP-002` now `RESOLVED-IN-OPENWEPP`).
2. Climate seam integration-test closure is ratified through CLIM07 executed
   parser-to-kernel seam checks for both hillslope and watershed assignment
   surfaces.

### CLIM08 Promotability Impact

1. CLIM08 closes seam-specific governance HOLD objectives from the CLIM queue.
2. CLIM08 does not retire non-seam climate limitations tracked in
   `GAP-CLIMATE-003`..`GAP-CLIMATE-005`; those rows continue to control overall
   promotability posture for this contract.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-CLIMATE-001 | CLIM07 curated comparator vectors now cover accepted continuous-daily and breakpoint runtime forcing seams plus confidence-tier routing metadata. Remaining fully per-invariant expansion across all climate branches is tracked as follow-on comparator depth, not a CLIM07 blocker. | Immediate CLIM07 comparator/seam closure is evidence-backed; deeper branch-family expansion remains future scope. | resolved-in-openWEPP | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-CLIMATE-002 | Chapter-2 notes that storm-duration and peak-intensity equations are tentative and may change as additional data are analyzed. This is treated as a legacy caveat (not a standalone promotion blocker) until superseding authority or targeted validation evidence is added. | Parameter/regional uncertainty remains and may alter forcing tails. Retirement criterion: add superseding authority citation or targeted validation evidence package and update this row. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-CLIMATE-003 | Multi-storm-per-day disaggregation behavior is identified as future work in Chapter 2 and is not fully modeled in current assumptions. | Can under-represent observed sub-daily event structure for some regions/events. | non-promotable | `[DIRECT][Static]` |
| GAP-CLIMATE-004 | Downstream forcing-consumer contracts (`SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-IRRIG-001`) are not yet fully authored. | Cross-contract forcing closure is provisional. | non-promotable | `[DIRECT][Static]` |
| GAP-CLIMATE-005 | Concrete openWEPP API field aliases for canonical climate symbols are not yet fixed by implementation contracts. | Alias map remains identity-only and must be revised when boundary names diverge. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-03 package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with climate invariants, guard map, symbol alias map, and dual-review workflow readiness for SCI-03. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: canonical symbol continuity fixes, added missing `Ak`/`Nk`/`N` and alias coverage, and scoped breakpoint invariant to storm events. |
| `2026-05-20` | `3` | `Codex` | Reclassified `GAP-CLIMATE-002` from non-promotable to promotable-with-risk, with explicit retirement criterion for future authority/validation updates. |
| `2026-05-23` | `4` | `Codex` | WB14 amendment: added explicit runoff hyetograph-coupling forcing requirements, typed-guard alignment for malformed subdaily series, and WB14 vector obligations. |
| `2026-05-23` | `5` | `Codex` | CLIM05 amendment: added parsed snow-control runtime coupling requirements, signed `S` term publication constraints, and typed active-coupling guard posture for snow boundary failures. |
| `2026-05-23` | `6` | `Codex` | CLIM06 amendment: added parsed frost-control runtime coupling requirements, explicit climate freeze/thaw branch-selection authority, and typed active-coupling guard posture for frozen-soil boundary failures. |
| `2026-05-23` | `7` | `Codex` | IRRIG10 amendment: added climate schedule-key authority (`day`, `year`) and downstream irrigation-coupled forcing closure requirements for fixed-date/depletion runtime scheduling. |
| `2026-05-23` | `8` | `Codex` | CLIM07 amendment: added explicit comparator/seam vector obligations for continuous-daily and breakpoint climate modes, parser-to-kernel namespace projection checks, and confidence-tier routing evidence requirements; reclassified `GAP-CLIMATE-001` as resolved in openWEPP scope. |
| `2026-05-23` | `9` | `Codex` | CLIM08 governance closeout: added seam HOLD-closure mapping to CLIM02/CLIM07 evidence and clarified that seam closeout does not retire non-seam promotability gaps (`GAP-CLIMATE-003`..`GAP-CLIMATE-005`). |
| `2026-05-25` | `10` | `Codex` | SIMIMPL14 amendment: clarified schedule-key continuity authority so climate day/year keys progress deterministically across full continuous-run spans used by replay publication and irrigation-coupled forcing consumers. |
