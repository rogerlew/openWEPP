---
contract_id: SC-SNOWFREEFORCING-001
title: Snow-Free Half-Hour Forcing Provider Contract
status: approved
maturity: active
owner: openWEPP maintainers + climate/radiation + vegetation/LSE reviewers
contract_version: 1
producer_scope:
  - Daily climate and breakpoint storm projection
  - SIMIMPL28 hourly meteorological parent reconstruction
  - Snow-free 1800-second atmospheric forcing receipts
consumer_scope:
  - Default-off V10 vegetation and snow-free LSE-V2 real-consumer shadow
evidence_level: static+independent_oracle
last_reviewed: 2026-08-18
supersedes: []
superseded_by: []
---

# SC-SNOWFREEFORCING-001 Snow-Free Half-Hour Forcing Provider Contract

Status: `approved`

Maturity: `active`

Evidence mode: `Static + independent contract vectors`

## Purpose

Define `OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1`, the exact default-off
repository operator that composes existing daily climate, breakpoint storms,
SIMIMPL28 hourly mechanics, static run configuration, and separately owned live
state into 48 consecutive 1,800-second receipts for the V10 vegetation and
snow-free land-surface-energy shadow.

## Scientific scope

In scope: horizontal parent-hour radiation, conservative half-hour refinement,
four-component shortwave partition, atmospheric-only downward longwave,
station-elevation pressure, dew-point humidity, Harder--Pomeroy hydrometeor
temperature, explicit CO2/reference-height/GSI/WB14 providers, receipt identity,
and typed unsupported domains.

Out of scope: sub-hour atmospheric variability, observed-pressure switching,
per-OFE atmospheric averaging, live soil/canopy-state interpolation, canopy or
terminal snow, soil transformations, activation, production output, calibration,
empirical validation, and transferability.

## Authority anchors

| ID | Source | Exact use | Evidence |
|---|---|---|---|
| REF-SFF-WEISS-NORMAN | Weiss and Norman (1985), *Agricultural and Forest Meteorology* 34:205--213, doi `10.1016/0168-1923(85)90020-6`, pp. 207--210, lawful author repository copy SHA-256 `76b1283039a8f383b68b80aa185834555ef6098ba254e2c3ec94c9b541e2e00d` | Equations 1--12 and stated ratio caps for direct/diffuse visible and NIR partition on a horizontal plane. | `[DIRECT][Static]` |
| REF-SFF-DILLEY | Dilley and O'Brien (1998), *QJRMS* 124:1391--1401, doi `10.1002/qj.49712454903` | Clear-sky downward longwave from screen temperature and precipitable-water proxy. | `[DIRECT][Static]` |
| REF-SFF-UNSWORTH | Unsworth and Monteith (1975), *QJRMS* 101:13--24, doi `10.1002/qj.49710142703` | All-sky emissivity `epsilon(c)=(1-0.84c)epsilon(0)+0.84c`. | `[DIRECT][Static]` |
| REF-SFF-FAO56 | FAO Irrigation and Drainage Paper 56, Chapter 3, Eq. 7 and Eq. 14 | Static station-elevation pressure and actual vapor pressure at dew point. | `[DIRECT][Static]` |
| REF-SFF-CLM | CTSM/CLM5 technical description, atmospheric humidity boundary | Exact specific-humidity/vapor-pressure relationship and its algebraic inverse. | `[DIRECT][Static]` |
| REF-SFF-SIMIMPL28 | `SC-CLIMATE-001#INV-CLIMATE-013`, pinned baseline `sunmap.for`, `radcur.for`, `hr_tmp.for`, and `runtime_inputs/06_simimpl28_hourly_forcing.rs` intake SHA-256 `011f3195149c133a021821637e1cf2daf3bcd1df1f2e630daa76bd72fd3e068b` | Daily-to-hourly temperature, horizontal extraterrestrial hour weighting, effective cloud fraction, and precipitation phase parent mechanics. | `[DIRECT][Static]` |
| REF-SFF-SNOWENERGY | `SC-SNOWENERGY-001` Dilley--Unsworth branch | Existing equation/constants parity; atmospheric component only. | `[DIRECT][Static]` |
| REF-SFF-HARDER | `SC-SNOWFREEZE-001` Harder--Pomeroy authority and `openwepp_meteorology::psychrometrics` | Liquid-water saturation `0.611 exp(17.3T/(237.3+T))`, normalized RH only for phase, hydrometeor temperature. | `[DIRECT][Static]` |
| REF-SFF-PHYSICAL | Conservation, half-open support, finite-domain, and owner-separation invariants | Exact support, mass/energy closure, nonnegative components, no owner fabrication. | `[INFERENCE][Static]` |

## Variables and units

| Symbol | Units | Meaning | Boundary alias |
|---|---|---|---|
| `h` | parent-hour index | `0..23` | `parent_hour_index` |
| `j` | child-interval index | `0..47` | `interval_index` |
| `Delta t` | `s` | child support duration, exactly `1800` | `interval_s` |
| `R_g,h` | `W m^-2` | parent-hour global horizontal shortwave mean flux | `global_horizontal_shortwave_w_m2` |
| `E_g,h` | `MJ m^-2 h^-1` | parent-hour integrated global horizontal shortwave | `horizontal_radiation_mj_m2` |
| `mu_h` | fraction | parent-hour mean horizontal solar-zenith cosine from the exact `radcur` hour-angle integral | `solar_zenith_cosine` |
| `P` | `kPa` | climate-station elevation pressure | `pressure_kpa` |
| `R_DV,R_dV` | `W m^-2` | potential direct/diffuse visible radiation | internal |
| `R_DN,R_dN` | `W m^-2` | potential direct/diffuse NIR radiation | internal |
| `S_DV,S_dV,S_DN,S_dN` | `W m^-2` | actual four-component shortwave | receipt fields |
| `T_a,T_d` | `degC` | hourly air temperature and daily dew point | forcing fields |
| `e_a` | `kPa` | actual vapor pressure | `actual_vapor_pressure_kpa` |
| `q` | `kg kg^-1` | specific humidity | `specific_humidity_kg_kg` |
| `VPD` | `kPa` | unclipped vapor-pressure deficit | `vpd_kpa` |
| `c` | fraction | SIMIMPL28 effective daily cloud fraction | `cloud_fraction` |
| `L_down` | `W m^-2` | top-boundary atmospheric downward longwave | `downward_longwave_w_m2` |
| `m_j` | `kg m^-2` | liquid precipitation mass on child support | parcel mass |

## Algorithm state surfaces

Required immutable inputs are one validated repository climate day, climate
metadata latitude/elevation, exact breakpoint supports when present, the
SIMIMPL28 hourly parent result, explicit CO2, digest-bound reference height,
daily admitted GSI, typed per-OFE WB14 configuration, run/day/OFE/tile identities,
and provider definition identity. Output is one immutable 48-receipt day object
with consecutive half-open supports and a canonical digest. The meteorological
operator mutates no production or scientific owner state. Soil water/hydraulics,
soil thermal state, surface/litter liquid, V10 canopy/C/N state, BGC inventory,
and accepted runon remain separately owned and are re-read after each accepted
half-hour by the consumer.

## Algorithm specification

### 1. Parent-hour construction

Run the common SIMIMPL28 hourly kernel in complete-row mode without changing the
production winter trigger. `T_a,h`, effective `c`, phase/RH/hydrometeor facts,
and hour weights are the authoritative parents. Construct horizontal energy as

```text
E_g,h = radmj * radcur_h / rpoth
```

for the ordinary branch, with the existing near-isothermal `radmj/24` branch.
This is the horizontal `radly` lineage. Slope-adjusted `estrad` and internal
`sb`/`sd` are not the exported global/direct/diffuse forcing. The horizontal
provider always uses the `radcur/rpoth` distribution, including near-isothermal
days. The legacy `radmj/24` exception remains unchanged only on the existing
winter `hr_tmp` output; it does not define the new horizontal forcing. Thus
near-isothermal temperature is held at the daily midpoint while nighttime
horizontal shortwave remains exact zero.

For the same hour-angle bounds `omega_1,omega_2` and solar-time correction used
by `radcur`, define the whole-parent-hour mean horizontal cosine:

```text
mu_h = max(0,
  [cos(phi)cos(delta)(sin(omega_2)-sin(omega_1))
   +(omega_2-omega_1)sin(phi)sin(delta)]
  /(omega_2-omega_1))
```

Equivalently, `mu_h` is the `radcur` horizontal energy divided by the same
hour's extraterrestrial normal-beam energy. This exact integral-average
composition is selected because the source quantity is hourly integrated
radiation; midpoint sampling is not used. `mu_h` is digest-bound.

### 2. Child supports and zero-order hold

For every `h`:

```text
j0=2h; j1=2h+1
support(j)=[1800j,1800(j+1))
E_g,j0=E_g,j1=E_g,h/2
R_g,j0=R_g,j1=E_g,h*1e6/3600
```

All parent-hour intensive/mean atmospheric fields use zero-order hold in both
children. This asserts no sub-hour meteorological variability.

### 3. Weiss--Norman partition

Provider branch identity: `WEISS_NORMAN_1985`.

At exact `R_g=0`, return four exact positive zeros without evaluating air mass.
At `R_g>0`, require finite `P>0` and finite `mu>0`, then:

```text
m = 1/mu
p = P/101.325
R_DV = 600 exp(-0.185 p m) mu
R_dV = 0.4 (600 - R_DV/mu) mu
w = 1320 * 10^(-1.1950 + 0.4459 log10(m) - 0.0345 log10(m)^2)
R_DN = (720 exp(-0.06 p m) - w) mu
R_dN = 0.6 (720 - R_DN/mu - w) mu
R_V = R_DV + R_dV
R_N = R_DN + R_dN
ratio = R_g/(R_V+R_N)
S_V = R_g R_V/(R_V+R_N)
S_N = R_g R_N/(R_V+R_N)
r_V = min(ratio,0.9)
r_N = min(ratio,0.88)
f_V = max(0,(R_DV/R_V) [1 - ((0.9-r_V)/0.7)^(2/3)])
f_N = max(0,(R_DN/R_N) [1 - ((0.88-r_N)/0.68)^(2/3)])
S_DV=f_V S_V; S_dV=(1-f_V)S_V
S_DN=f_N S_N; S_dN=(1-f_N)S_N
```

The two upper ratio caps and the paper's explicit rule that direct fractions
are never permitted negative define these branches; the `max(0,...)` operation
is constitutive, not post-hoc component clipping. All outputs must be
finite/nonnegative and reconstruct `R_g` under the admitted representation
tolerance.

### 4. Atmospheric longwave

With `T_K=T_a+273.15`, `e_a` in kPa, and retained `c`:

```text
w_lw = 4650 e_a/T_K
L_clear=59.38+113.7(T_K/273.16)^6+96.96 sqrt(w_lw/25)
epsilon_clear=L_clear/(sigma T_K^4)
epsilon_all=(1-0.84c)epsilon_clear+0.84c
L_down=epsilon_all sigma T_K^4
```

This is atmospheric-only. It has no canopy blending and is evaluated day or
night with the daily effective cloud assumption.

### 5. Pressure and humidity

```text
P=101.3*((293-0.0065 z)/293)^5.26
e_a=0.611 exp(17.3 T_d/(237.3+T_d))
q=0.622 e_a/(P-0.378 e_a)
VPD=0.611 exp(17.3 T_a/(237.3+T_a))-e_a
```

Use climate-station elevation only. Do not clip VPD. Nonpositive VPD is a typed
unsupported vegetation domain while `q` remains physical for condensation.
Only the precipitation-phase branch may use `min(1,e_a/e_sat(T_a))`.

### 6. Precipitation and enthalpy

For exact breakpoints, retain `stmstr_h` from the shared breakpoint record.
Convert every event-relative support to absolute day-clock seconds with
`absolute_s=3600*stmstr_h+relative_s`, then integrate piecewise-constant
intensity over each half-open child overlap. Support beyond `86400` is not
dropped or wrapped into the current day: it becomes a provider-cursor carry
with support translated by `-86400` and the original parcel/source identity for
the next day. Otherwise split each admitted parent-hour mass equally.
Daily and parent-hour mass close exactly under the admitted representation
tolerance; daily mass is never uniformly spread over 48 intervals. Liquid
parcel enthalpy is

```text
H = m * 4218 * (T_liquid_K - 273.15)
```

and runon retains accepted upstream parcel identity, support, temperature, and
enthalpy rather than being meteorologically regenerated. Snow or mixed-phase
mass is not relabeled liquid: the snow-free provider returns typed unsupported
when the selected parent phase has positive snow mass. Each liquid parcel uses
the selected parent Harder--Pomeroy hydrometeor temperature; no independent
parcel temperature may be supplied.

### 7. Other fields and receipt identity

CO2 is explicit with no hidden default. Reference height is configuration-owned.
GSI is the accepted daily result of the existing stateful GSI owner, computed
exactly once with its configuration/date/state receipt and held over all 48
intervals; this provider cannot advance or recompute GSI. WB14 is static typed
per-OFE configuration. Receipt identity binds provider/model/source
climate/run/day/OFE/tile/support and every physical operand. Mixed versions,
missing/duplicate supports, or digest mismatches fail closed.

`provider_definition_sha256` is the SHA-256 of the immutable canonical
`receipt-provider-definition.json` semantic descriptor. The higher-level model
definition binds that descriptor plus this contract, schema, calculator, and
vectors. This two-level identity is intentional and noncircular: receipts bind
the provider semantics directly rather than attempting to hash a model file
that itself binds those receipts.

## Branch and guard table

| Trigger | Branch | Typed posture |
|---|---|---|
| `R_g == 0` | four exact zeros | valid degenerate |
| `R_g > 0 && (mu<=0 or P<=0)` | no optical evaluation | unsupported domain |
| nonfinite/negative component or failed closure | reject | invariant error |
| invalid elevation/base/pressure or `P<=e_a` | reject | atmospheric-domain error |
| `VPD<=0` | construct the physical atmospheric/LSE receipt, but reject the coupled interval before any owner advances | unsupported vegetation domain; atomic no-op |
| zero/nonpositive wind | no flooring | unsupported aerodynamic domain |
| exact breakpoints | overlap integration | valid |
| no breakpoint support | parent-hour equal split | valid |
| positive snow or mixed-phase snow mass | do not relabel as liquid | unsupported snow-free domain |
| heterogeneous OFE global atmospheric operands | no average/first selection | unsupported multi-OFE domain |
| missing explicit CO2/GSI/reference/WB14 | no default | missing-provider error |

## Invariants and guard map

| ID | Statement | Authority | Guard | Failure posture |
|---|---|---|---|---|
| INV-SFF-001 | Exactly 48 ordered nonoverlapping 1,800-second half-open supports cover `[0,86400)`. | REF-SFF-PHYSICAL | constructor/test | typed identity error |
| INV-SFF-002 | Each child holds its parent intensive fields and receives half parent energy. | REF-SFF-SIMIMPL28, REF-SFF-PHYSICAL | projection/closure | typed closure error |
| INV-SFF-003 | Horizontal `radly` lineage is used; slope-adjusted `estrad` and `sb/sd` are excluded. | REF-SFF-SIMIMPL28 | source/projection guards | typed lineage error |
| INV-SFF-004 | Weiss--Norman components are finite/nonnegative and reconstruct global horizontal shortwave. | REF-SFF-WEISS-NORMAN | runtime/vector closure | typed radiation error |
| INV-SFF-005 | Atmospheric longwave contains no canopy blend and uses retained effective cloud fraction day/night. | REF-SFF-DILLEY, REF-SFF-UNSWORTH | parity/source guard | typed atmospheric error |
| INV-SFF-006 | Pressure is station-elevation FAO-56 pressure; provider substitution is forbidden. | REF-SFF-FAO56 | configuration identity | typed pressure error |
| INV-SFF-007 | `q` is exact from dew-point vapor pressure and VPD is unclipped. | REF-SFF-CLM, REF-SFF-FAO56, REF-SFF-HARDER | runtime/vector guard | typed humidity/unsupported error |
| INV-SFF-008 | Exact breakpoint overlap or parent-hour fallback preserves hourly/daily precipitation mass. | REF-SFF-SIMIMPL28, REF-SFF-PHYSICAL | independent mass closure | typed precipitation error |
| INV-SFF-009 | Live scientific owner state is not serialized or interpolated into meteorological forcing. | REF-SFF-PHYSICAL | DTO/source exclusion | typed owner error |
| INV-SFF-010 | Receipt digest changes for every identity or physical operand and mixed provider versions reject. | REF-SFF-PHYSICAL | canonical digest poisons | typed identity error |
| INV-SFF-011 | Global atmospheric forcing heterogeneity is rejected, never averaged. | REF-SFF-PHYSICAL | multi-OFE preflight | typed unsupported error |
| INV-SFF-012 | Provider remains explicit/default-off and cannot mutate production state or output. | campaign boundary | source exclusion and consumer tests | package HOLD |

## Producer and consumer obligations

The producer must use actual repository climate/runtime inputs, including
breakpoint `stmstr_h`, issue a complete
digest-bound day receipt, and expose no public completed-array injection route
to the closure-eligible consumer. The consumer must validate identity and
support before physics, re-read live owner state after every accepted child,
retain signed condensation/runon custody, and commit only isolated shadow state.
A nonpositive-VPD coupled interval is fail-atomic: neither LSE nor vegetation
advances, even though the independently valid atmospheric receipt preserves the
specific humidity needed for an LSE-only future consumer.

## Symbol alias map

| Canonical | Boundary/API | Scope | Units check | Owner |
|---|---|---|---|---|
| `radly` | daily climate `rad` | parser/runtime | Ly/day conversion helper | SC-CLIMATE-001 |
| `E_g,h` | `horizontal_radiation_mj_m2` | hourly parent | MJ m^-2 h^-1 | this contract |
| `S_DV,S_dV` | LSE-V2 `direct_vis_w_m2`/`diffuse_vis_w_m2` and V10 `direct_par_w_m2`/`diffuse_par_w_m2` | receipt | same Weiss--Norman 400--700 nm energy flux bytes; no photon conversion | this contract |
| `S_DN,S_dN` | LSE-V2/V10 direct/diffuse NIR fields | receipt | W m^-2 | this contract |
| `L_down` | LSE atmospheric downward longwave | receipt | W m^-2 | this contract |
| `e_a` | actual vapor pressure | meteorology/LSE | kPa/Pa named conversion | this contract |
| `q` | specific humidity | LSE forcing | kg kg^-1 | this contract |

## Constants and parameters

All Weiss--Norman constants (`600`, `720`, `0.185`, `0.06`, `0.4`, `0.6`,
`1320`, `-1.1950`, `0.4459`, `-0.0345`, `0.9`, `0.7`, `0.88`, `0.68`,
`101.325`) are fixed authority constants, not calibration parameters. Dilley--
Unsworth constants (`4650`, `59.38`, `113.7`, `273.16`, `96.96`, `25`, `0.84`)
are fixed by the selected branch. FAO pressure constants and Harder--Pomeroy
saturation constants are likewise fixed. CO2, GSI, station elevation,
reference height, WB14 parameters, and source climate are external inputs.

## Unit governance

Named conversions are required for Langleys/day to MJ m^-2 day^-1, MJ energy
to W m^-2 over 3,600 s, kPa to Pa, precipitation depth to kg m^-2, Celsius to
Kelvin, and liquid enthalpy. Raw dimensional literals must be equation-local and
bound to the constants table. No public output metadata is introduced.

## Tolerance and numeric notes

Support indices and identities are exact. Canonical JSON/digests are byte exact.
Mass/energy and four-component reconstruction use the package-selected
representation tolerance `1e-12 * max(1, abs(reference))` only for accumulated
binary64 arithmetic; no input or component is clipped to obtain closure. The
two Weiss--Norman ratio caps are constitutive branches, not tolerances.

## Calibration and identifiability posture

`CALIBRATION_NOT_APPLICABLE`: this package transcribes selected authority and
composes existing providers. It introduces no fitted openWEPP parameter.

- `science_implementation_status = NOT_IMPLEMENTED` until the successor package
- `calibration_evidence_status = NOT_APPLICABLE`
- `identifiability_status = NOT_APPLICABLE`

Empirical accuracy and transferability are not claimed; Weiss--Norman remains
an explicit forcing uncertainty/claim ceiling.

## Test-vector obligations

The authority fixture must cover dry clear/cloudy, night, dawn/dusk, zero
radiation, high/low pressure, humid positive VPD, dew point equal/above air,
zero wind, breakpoint crossing a half-hour and midnight, fallback split,
Harder--Pomeroy temperature/enthalpy, shortwave/hourly/daily closure, 48-support
continuity, missing/duplicate/mixed-version poisons, and heterogeneous multi-OFE
rejection. Expected values come from an independent non-Rust calculator.

## Binding Exposure Index

| Residue | Canonical binding | Status |
|---|---|---|
| package authority selections | INV-SFF-001..012 | active |
| prior Child-4 broad provider HOLD | prospective dependency disposition | historical; not deleted |

## Gap register

| Gap | Disposition | Promotion impact |
|---|---|---|
| Native half-hour meteorology absent | zero-order parent-hour refinement is the selected V1 authority | none |
| Weiss--Norman empirical transferability | retained uncertainty; no calibration/validation claim | none for implementation, blocks empirical claims |
| Heterogeneous OFE atmosphere | typed unsupported until a later schema/provider version | domain limitation |
| Canopy/terminal snow and soil transformations | explicitly out of scope | none |

## Change log

- 2026-08-18: Version 1 drafted contract-first to lift the bounded Child-4
  forcing HOLD without activation or new constitutive vegetation/LSE physics.
