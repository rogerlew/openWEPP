---
contract_id: SC-SNOWENERGY-001
title: Snow-Surface Energy and Sub-Canopy Longwave Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + snow-process reviewer
contract_version: 1
producer_scope:
  - Hourly atmospheric longwave evaluated from hourly temperature and daily vapor/cloud state
  - Native-canopy effective cover to diffuse sky-view translation
  - Complementary sky and canopy longwave incident at the snow surface
consumer_scope:
  - Shared snow-surface energy carrier selected by SNOW-SURFACE-EB-03
  - Snow sublimation and melt components
  - Snow-energy diagnostics and assurance outputs
evidence_level: static
last_reviewed: 2026-07-30
supersedes: []
superseded_by: []
---

# SC-SNOWENERGY-001 Snow-Surface Energy and Sub-Canopy Longwave Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define the canonical hourly atmospheric and sub-canopy longwave equations,
using hourly temperature with daily vapor/cloud state, plus operand meanings,
guards, and coupling obligations for openWEPP snow-surface energy
calculations. The contract derives diffuse sky view from existing effective
canopy cover and does not introduce a user-entered sky-view coefficient or a
required remote-sensing input.

This contract is equation and interface authority. Production activation is
held until `SNOW-SURFACE-EB-03` selects one shared snow-surface-temperature and
cold-content provider for the common `B/L/S/LS` surface-energy carrier.

## Scientific Scope

In scope:

- Dilley-O'Brien clear-sky longwave from air temperature and vapor pressure.
- Unsworth-Monteith cloud correction derived from daily clearness index.
- An effective-cover-to-diffuse-sky-view transformation derived from the same
  Beer-law canopy extinction basis used by FSM2.
- Complementary atmospheric and canopy longwave at the snow surface.
- Effective-unity canopy and snow emissivity convention; atmospheric
  emissivity remains the variable Dilley-Unsworth result.
- Outgoing snow longwave and the positive-toward-snow net-longwave sign.
- Typed runtime obligations and deterministic analytical test vectors.

Out of scope:

- Selecting a snow-surface-temperature or cold-content provider.
- Production runtime, selector, input-schema, output-schema, or default changes.
- A prognostic canopy-temperature energy balance.
- Explicit trunks, canopy gaps, terrain horizons, multiple reflections, or
  three-dimensional ray tracing.
- Site fitting, empirical calibration, or a new user radiative coefficient.
- Shortwave, sensible, latent, rain-heat, melt, or refreezing equations except
  where their shared-state boundary must remain explicit.

Validity is limited to an equivalent, horizontally homogeneous one-layer
canopy with the FSM2 random-orientation angular extinction approximation and
an isotropic diffuse sky hemisphere. The native structural-cover floor is
treated as effective vertical optical depth in that equivalent medium. The
contract does not claim direct validity for directional crowns, explicit
gaps/edges/trunks, terrain-obstructed sky, or anisotropic diffuse radiation.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| `REF-SNOWENERGY-FLERCHINGER` | Flerchinger et al. (2009), *Water Resources Research* 45:W03423, doi: `10.1029/2008WR007394`, corrected Table 1, Tables 2 and 9 | Corrected Dilley-O'Brien clear-sky equation, precipitable-water proxy, Unsworth-Monteith cloud correction, daily clearness bounds, and reported uncertainty. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-ESSERY2008` | Essery et al. (2008), *Hydrological Processes* 22:2788-2800, doi: `10.1002/hyp.6930` | Hemispherical view-factor integration and two-component forest longwave exchange. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-FSM2` | Essery et al. (2025), “FSM2.1.1: an efficient model of snow processes and surface energy balance”, *Geoscientific Model Development* 18:3583-3607, doi: `10.5194/gmd-18-3583-2025`, Eq. 13-14 and §2.3 | Beer-law direct and diffuse canopy transmission, factor `1.6`, and complementary longwave exchange with unity emissivity. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-SVS2` | Leonardini et al. (2025), “SVS2-Crocus”, *Geoscientific Model Development* 18:9119-9154, doi: `10.5194/gmd-18-9119-2025`, Eq. 1 and §2.1.2 | Independent support that canopy sky-view factor is an exponential extinction function rather than direct plan-view cover. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-RUTTER2023` | Rutter et al. (2023), *Journal of Geophysical Research: Atmospheres* 128:e2022JD037980, doi: `10.1029/2022JD037980` | Stand-scale complementary sky/canopy formulation, effective-unity canopy behavior, and canopy-temperature approximation evidence and limits. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PLANT` | `SC-PLANT-001#INV-PLANT-034` and native canopy variables | Canonical openWEPP meanings of effective canopy cover, structural cover floor, LAI, and height. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-EB01A` | `docs/work-packages/20260730-snow-surface-eb-01a-longwave-authority-research-001/` | Package evidence that reconciled atmospheric longwave candidates and admitted the FSM2 canopy route. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PHYSICAL` | Stefan-Boltzmann law and bounded-fraction physical invariants | Thermal emission, finite-temperature, and bounded-transmission requirements. | `[INFERENCE][Static]` |

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `T_a` | `K` | Hourly above-canopy air temperature. | hourly climate forcing | atmospheric longwave |
| `e_a` | `kPa` | Daily actual vapor pressure held across the declared hourly evaluations. | daily climate forcing | precipitable-water proxy |
| `w` | `kg m^-2` | Precipitable-water proxy, `4650 e_a / T_a`. | atmospheric longwave | clear-sky equation |
| `L_clear` | `W m^-2` | Clear-sky downward longwave irradiance. | atmospheric longwave | clear-sky emissivity |
| `epsilon_clear` | `dimensionless` | Effective clear-sky emissivity. | atmospheric longwave | cloud mixture |
| `R_s` | `MJ m^-2 d^-1` | Daily incident above-canopy shortwave radiation. | climate forcing | clearness index |
| `R_a` | `MJ m^-2 d^-1` | Daily extraterrestrial radiation on the horizontal surface. | solar geometry | clearness index |
| `k_t` | `dimensionless` | Daily clearness index, `R_s/R_a`. | cloud inference | cloud fraction |
| `c` | `fraction` | Effective daily cloud fraction. | cloud inference | all-sky emissivity |
| `epsilon_all` | `dimensionless` | Effective all-sky emissivity. | atmospheric longwave | atmospheric irradiance |
| `L_atm` | `W m^-2` | Hourly all-sky downward atmospheric longwave above canopy. | atmospheric longwave | sub-canopy mixture |
| `C` | `fraction` | Effective plan-view overhead canopy interception fraction. | plant/canopy state | sky-view translation |
| `P_0` | `fraction` | Vertical canopy gap fraction, `1-C`. | sky-view translation | Beer-law elimination |
| `f_sky` | `fraction` | Hemispherical diffuse sky-view/transmission factor. | sky-view translation | sub-canopy mixture |
| `T_c` | `K` | Effective radiating canopy temperature. | EB-03 provider boundary | canopy emission |
| `L_can` | `W m^-2` | Effective canopy longwave emission, `sigma T_c^4`. | canopy emission | sub-canopy mixture |
| `L_sub` | `W m^-2` | Downward longwave incident at the snow surface. | sub-canopy mixture | snow energy carrier |
| `T_s` | `K` | Snow-surface radiating temperature. | EB-03 provider boundary | snow emission |
| `L_out` | `W m^-2` | Upward longwave emitted by snow. | snow emission | net longwave |
| `L_net` | `W m^-2` | Net longwave, positive toward snow. | longwave balance | snow energy carrier |
| `sigma` | `W m^-2 K^-4` | Stefan-Boltzmann constant. | fixed constant | emission equations |

## Algorithm State Surfaces

### Required inputs

| Surface | Required state |
|---|---|
| Above-canopy meteorology | hourly finite `T_a > 0 K`; daily finite `e_a >= 0 kPa` and `R_s >= 0 MJ m^-2 d^-1` |
| Solar geometry | finite `R_a >= 0 MJ m^-2 d^-1` plus an explicit daylight/polar-night classification |
| Canopy | finite effective daily `C` in `[0, 1)` |
| Thermal provider | finite `T_c > 0 K` and `T_s > 0 K` from the single EB-03-selected provider |

### Required outputs

`w`, `L_clear`, `epsilon_clear`, `k_t`, `c`, `epsilon_all`, `L_atm`,
`P_0`, `f_sky`, `L_can`, `L_sub`, `L_out`, and `L_net`, each with the
units and lineage declared above.

### Mutated state surfaces

The longwave evaluator is pure. It may not mutate canopy, snow mass, snow
temperature, cold content, or forcing state. The shared snow-energy carrier may
consume `L_net` only after EB-03 supplies the coherent thermal state.

## Algorithm Specification

Required evaluation order:

1. Validate cadence, units, finiteness, and physical input domains.
2. Once per daylight day, calculate `k_t` and the bounded cloud fraction `c`;
   otherwise take the explicit polar-night unavailable branch.
3. For each hour, evaluate `w`, `L_clear`, `epsilon_clear`,
   `epsilon_all`, and `L_atm` using hourly `T_a` and held daily `e_a`/`c`;
   enforce the no-clamp derived-emissivity guard.
4. Translate the current effective canopy cover to `P_0` and `f_sky`.
5. Obtain coherent hourly `T_c`, `T_s`, and snow cold content from the
   EB-03-selected provider; stop on unavailable state.
6. Evaluate `L_can`, `L_sub`, `L_out`, and `L_net` in the specified order.
7. Publish the component operands to the shared energy carrier exactly once
   only after all guards and the real-consumer closure gate pass.

### Atmospheric longwave

For each hourly evaluation, use hourly air temperature `T_a` in kelvin and
the daily actual vapor pressure `e_a` in kilopascals:

```text
w = 4650 e_a / T_a
L_clear = 59.38
          + 113.7 (T_a / 273.16)^6
          + 96.96 sqrt(w / 25)
epsilon_clear = L_clear / (sigma T_a^4)
```

The constants and units are inseparable from this equation. A vapor pressure
in pascals or temperature in degrees Celsius is invalid.

Evaluate `w`, `L_clear`, `epsilon_clear`, `epsilon_all`, and `L_atm` at the
hourly `T_a`; do not substitute daily mean temperature into the nonlinear
`T_a^6` and `T_a^4` equations. The daily clearness-derived `c` and daily
`e_a` are held constant across that day's hourly evaluations. A future
subdaily humidity/cloud route requires a contract amendment.

When daylight permits a daily clearness index:

```text
k_t = R_s / R_a
c = clamp((0.80 - k_t) / (0.80 - 0.15), 0, 1)
epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c
L_atm = epsilon_all sigma T_a^4
```

The clamp belongs only to the declared empirical cloud mapping. It must not
repair a non-finite input or an invalid radiation unit.

Flerchinger et al. report approximately `24.5 W m^-2` subdaily RMSD and
`14.9 W m^-2` daily RMSD for the Dilley-Unsworth combination across their
development comparison. These are model-form context, not openWEPP
calibration tolerances or guaranteed errors. A daily clearness operator cannot
recover observed subdaily cloud variation, and solar-index inference is
undefined during polar night.

The reviewed authority does not provide a transferable numeric
temperature/humidity envelope for every openWEPP climate. Therefore each
evaluation must require finite `L_clear`, `epsilon_clear`, `epsilon_all`, and
`L_atm`, with `0 <= epsilon_clear <= 1` and `0 <= epsilon_all <= 1`.
Out-of-range derived emissivity is typed `out-of-authority`; it is not clamped
to one. Passing this physical output guard does not assert site validation.

### Effective canopy cover to diffuse sky view

`C` is the effective overhead interception state already produced by the
canopy model. Identify its complement with vertical Beer-law gap fraction:

```text
P_0 = 1 - C = exp(-k_ext VAI_eff)
```

FSM2's hemispherical diffuse transmission is:

```text
f_sky = exp(-1.6 k_ext VAI_eff)
```

Eliminating the unobserved product `k_ext VAI_eff` gives the canonical
openWEPP translation:

```text
f_sky = P_0^1.6 = (1 - C)^1.6
```

This is a model-state translation, not a fitted coefficient. The exponent is
the FSM2 diffuse-transmission factor. The effective extinction coefficient
cancels algebraically, so openWEPP does not request it from the user.

The translation treats the complete effective cover state—including the
native structural floor—as equivalent vertical optical depth under the same
homogeneous, randomly oriented canopy and isotropic diffuse-sky regime. It
does not assert that structural cover is a measured stem-area index.

`LAI`, native structural cover, and canopy height are not separately added:

- daily `C` already includes the leaf-on/leaf-off canopy trajectory and the
  structural-cover floor;
- structural cover is a fraction, not stem-area index, so adding it to LAI
  would be dimensionally and semantically invalid;
- adding LAI again would double count foliage represented in `C`;
- homogeneous Beer-law gap transmission contains no independent height term.

Height and LAI remain diagnostic provenance for `C` and may support future
adjudication; they do not enter this canonical one-layer translation.

### Complementary sub-canopy exchange

`L_atm` arrives as the already evaluated incident atmospheric flux with
variable effective `epsilon_all`. With effective canopy and snow emissivities
fixed to one:

```text
L_can = sigma T_c^4
L_sub = f_sky L_atm + (1 - f_sky) L_can
L_out = sigma T_s^4
L_net = L_sub - L_out
```

Positive `L_net` supplies energy to snow. Negative `L_net` removes energy.
Sky and canopy weights are complementary and must sum to exactly one within
the numeric tolerance.

The admitted first implementation may use above-canopy `T_a` for `T_c` only
as an explicitly named homogeneous-stand approximation. It must not be
described as a prognostic canopy energy balance. Stable nocturnal inversions,
forest edges, large gaps, and strongly sunlit or intercepted-snow canopies are
known limitations.

## Branch and Guard Table

| Branch/condition | Required behavior | Guard class | Failure class |
|---|---|---|---|
| Any required scalar is non-finite | Reject before arithmetic. | runtime | typed invalid forcing/state |
| `T_a <= 0 K`, `T_c <= 0 K`, or `T_s <= 0 K` | Reject. | runtime | typed invalid temperature |
| `e_a < 0 kPa`, `R_s < 0`, or `R_a < 0` | Reject. | runtime | typed invalid forcing |
| Derived `L_clear`, `epsilon_clear`, `epsilon_all`, or `L_atm` is non-finite, or either emissivity is outside `[0,1]` | Reject without clamping. | runtime | typed out-of-authority atmospheric state |
| Daylight and `R_a > R_a,min` | Calculate `k_t`, clamp only the empirical cloud mapping, and continue. | runtime | none |
| Polar night or `R_a <= R_a,min` | Do not divide and do not infer cloud from `R_s/R_a`; require an independently authoritative cloud state or return a typed unavailable state. | runtime | typed cloud-forcing unavailable |
| `C < 0` or `C >= 1` | Reject; do not silently clamp. | runtime | typed invalid canopy state |
| `C = 0` | Require `f_sky = 1` and `L_sub = L_atm`. | test | blocked promotion on mismatch |
| `C -> 1` within valid domain | Require `f_sky -> 0` and `L_sub -> L_can`. | test | blocked promotion on mismatch |
| EB-03 thermal provider absent | Do not publish `L_out`, `L_net`, or a production snow-energy update. | governance | hard `HOLD` |
| `T_c = T_a` approximation active | Emit/retain explicit approximation identity in configuration or diagnostics. | profile | blocked promotion if unlabeled |
| Canopy is outside equivalent homogeneous/random-orientation/isotropic-diffuse regime | Do not expand the claim; retain a diagnostic/model-limitation classification. | governance | model limitation |

`R_a,min` is a numerically explicit implementation threshold in
`MJ m^-2 d^-1`; EB-03 must bind its value before runtime activation.

## Invariants and Guard Map

### Invariants

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-SNOWENERGY-001` | Temperatures entering fourth-power emission are finite kelvin values greater than zero. | `REF-SNOWENERGY-PHYSICAL` | `[INFERENCE][Static]` | pre-arithmetic temperature guard | typed invalid temperature |
| `INV-SNOWENERGY-002` | Vapor pressure is finite, non-negative, and expressed in `kPa`. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | forcing/unit boundary | typed invalid forcing |
| `INV-SNOWENERGY-003` | `0 <= c <= 1`; clamping occurs only after a valid finite daylight `k_t` exists. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | cloud branch guard | typed invalid or unavailable cloud forcing |
| `INV-SNOWENERGY-004` | `0 <= C < 1`, `P_0=1-C`, and `f_sky=(1-C)^1.6`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PLANT` | `[DIRECT][Static] + [INFERENCE][Static]` | canopy-domain guard and vectors | typed invalid canopy state |
| `INV-SNOWENERGY-005` | `0 < f_sky <= 1` and sky view decreases monotonically with `C` on the valid domain. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | analytical property test | blocked contract/runtime promotion |
| `INV-SNOWENERGY-006` | Sky and canopy weights are complementary: `f_sky+(1-f_sky)=1`. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-RUTTER2023` | `[DIRECT][Static]` | independent reconstruction | blocked contract/runtime promotion |
| `INV-SNOWENERGY-007` | At `C=0`, `L_sub=L_atm`; as `C->1`, `L_sub->sigma T_c^4`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | limiting-case vectors | blocked contract/runtime promotion |
| `INV-SNOWENERGY-008` | `L_net=L_sub-sigma T_s^4`, positive toward snow. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static]` | sign and reconstruction tests | typed energy-evaluation failure |
| `INV-SNOWENERGY-009` | LAI, structural cover, and height are not independently added to effective cover in this translation. | `REF-SNOWENERGY-PLANT`, `REF-SNOWENERGY-FSM2` | `[DIRECT][Static] + [INFERENCE][Static]` | contract/source review | hard `HOLD` on alternate composition |
| `INV-SNOWENERGY-010` | No production snow-energy flux is published before EB-03 supplies coherent `T_c`, `T_s`, and cold-content coupling. | `REF-SNOWENERGY-EB01A` | `[DIRECT][Static]` | runtime-consumer closure gate | hard `HOLD` |
| `INV-SNOWENERGY-011` | Polar-night cloud inference never divides by zero or silently reuses unrelated legacy cloud state. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | daylight/cloud branch guard | typed cloud-forcing unavailable |
| `INV-SNOWENERGY-012` | Package-local analytical code is evidence only and is never imported by production crates. | ADR-0011 and package scope | `[DIRECT][Static]` | source/write-set inventory | blocked package closure |
| `INV-SNOWENERGY-013` | Atmospheric longwave uses hourly `T_a`; daily `e_a` and clearness-derived `c` may be held across the day. Daily-mean-temperature substitution is prohibited. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | cadence/aggregation test | typed cadence mismatch / hard `HOLD` |
| `INV-SNOWENERGY-014` | Derived atmospheric fluxes are finite and both effective emissivities lie in `[0,1]`; out-of-range results fail without clamping. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | derived-domain guard | typed out-of-authority state |

### Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-SNOWENERGY-001` | future atmospheric/exchange evaluator; analytical invalid-temperature cases | runtime | typed invalid temperature | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-002` | future forcing boundary; negative/non-finite vapor tests | runtime | typed invalid forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-003` | future daylight cloud operator; endpoint/clamp cases | runtime | typed invalid/unavailable cloud forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-004` | future canopy translator; valid/invalid cover cases | runtime | typed invalid canopy state | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-005` | monotonic analytical vector/property gate | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-006` | independently fixed complementary-mixture vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-007` | open/near-closed canopy limiting vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-008` | future shared energy carrier and fixed net-flux vectors | runtime | typed evaluation failure | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-009` | contract review and production source inspection | governance | hard `HOLD` | EB-02 `canopy-sky-view-derivation.md` |
| `INV-SNOWENERGY-010` | EB-03 provider decision and real-consumer closure gate | governance | hard `HOLD` | `GAP-SNOWENERGY-001` |
| `INV-SNOWENERGY-011` | explicit polar-night unavailable branch | runtime | typed cloud-forcing unavailable | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-012` | intended/exact write-set reconciliation | governance | blocked package closure | EB-02 `exact-diff-reconciliation.md` |
| `INV-SNOWENERGY-013` | hourly-forcing/provider cadence assertion | profile | typed cadence mismatch / hard `HOLD` | EB-02 `operand-lineage.csv`; EB-03 evidence required |
| `INV-SNOWENERGY-014` | future atmospheric derived-domain guard; out-of-authority vector | runtime | typed out-of-authority state | EB-02 `analytical-test-vectors.csv` |

## Producer and Consumer Obligations

| Obligation ID | Role | Requirement |
|---|---|---|
| `OBL-SNOWENERGY-P-001` | climate producer | Publish hourly `T_a` plus daily `e_a` and `R_s` with declared units, cadence, and finite-domain validation. |
| `OBL-SNOWENERGY-P-002` | solar-geometry producer | Publish `R_a` and explicit daylight/polar-night classification. |
| `OBL-SNOWENERGY-P-003` | canopy producer | Publish one effective daily plan-view canopy cover `C`; preserve its leaf-on/leaf-off and structural-floor semantics. |
| `OBL-SNOWENERGY-P-004` | EB-03 thermal producer | Publish one coherent `T_c`, `T_s`, and snow cold-content state or a typed unavailable result. |
| `OBL-SNOWENERGY-C-001` | longwave evaluator | Apply the equations and guards in the specified order without silent unit conversion or fallback. |
| `OBL-SNOWENERGY-C-002` | shared energy carrier | Consume `L_net` exactly once with the positive-toward-snow convention. |
| `OBL-SNOWENERGY-C-003` | sublimation/melt consumers | Use the same EB-03 snow state as longwave; do not reconstruct an independent surface temperature. |
| `OBL-SNOWENERGY-C-004` | diagnostics | Preserve component operands sufficient to reconstruct `L_atm`, `f_sky`, `L_sub`, `L_out`, and `L_net`. |
| `OBL-SNOWENERGY-C-005` | configuration/reporting | Identify when `T_c=T_a` is used and communicate its approximation limits. |
| `OBL-SNOWENERGY-C-006` | runtime implementation package | Prove the real common `B/L/S/LS` consumer reads this path before claiming activation. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract | Required interpretation |
|---|---|---|---|---|---|
| `T_a` | `air_temperature_k` | hourly climate to longwave | `K` -> `K` | `SC-CLIMATE-001` | Hourly above-canopy air temperature. |
| `e_a` | `vapor_pressure_kpa` | daily climate to hourly longwave | `kPa` -> `kPa` | `SC-CLIMATE-001` | Daily actual vapor pressure held across the day; not pascals or relative humidity. |
| `R_s` | `solar_radiation_mj_m2_day` | climate to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` | Incident daily shortwave. |
| `R_a` | `extraterrestrial_radiation_mj_m2_day` | solar geometry to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` / `SC-SNOWENERGY-001` | Same daily energy units as `R_s`. |
| `C` | `canopy_cover_fraction` | canopy to sky-view translation | `fraction` -> `fraction` | `SC-PLANT-001` | Effective overhead interception; not LAI or sky-view factor. |
| `f_sky` | `subcanopy_sky_view_fraction` | sky view to longwave mixture | `fraction` -> `fraction` | `SC-SNOWENERGY-001` | Derived diffuse transmission `(1-C)^1.6`; never alias directly to `1-C`. |
| `T_c` | `canopy_temperature_k` | thermal provider to canopy emission | `K` -> `K` | `SC-SNOWENERGY-001` | Effective radiating canopy temperature selected by EB-03. |
| `T_s` | `snow_surface_temperature_k` | thermal provider to snow emission | `K` -> `K` | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | EB-03-selected radiating snow-surface temperature. |
| `L_atm` | `atmospheric_longwave_w_m2` | atmosphere to sub-canopy mixture | `W m^-2` -> same | `SC-SNOWENERGY-001` | Hourly all-sky downward longwave above canopy. |
| `L_sub` | `subcanopy_longwave_w_m2` | mixture to snow energy | `W m^-2` -> same | `SC-SNOWENERGY-001` | Downward longwave incident at snow. |
| `L_net` | `net_longwave_w_m2` | longwave to shared energy carrier | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive toward snow. |

## Constants and Parameters

| Name | Value | Units | Status/provenance |
|---|---:|---|---|
| `sigma` | `5.670374419e-8` | `W m^-2 K^-4` | fixed SI Stefan-Boltzmann constant |
| Dilley intercept | `59.38` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature coefficient | `113.7` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature reference | `273.16` | `K` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water coefficient | `96.96` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water reference | `25` | `kg m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| vapor-pressure conversion | `4650` | `K kg m^-2 kPa^-1` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| clear clearness bound | `0.80` | `dimensionless` | EB-01A mapping |
| overcast clearness bound | `0.15` | `dimensionless` | EB-01A mapping |
| cloud mixture weight | `0.84` | `dimensionless` | `REF-SNOWENERGY-FLERCHINGER` |
| diffuse extinction multiplier | `1.6` | `dimensionless` | `REF-SNOWENERGY-FSM2`, Eq. 14 |
| canopy/snow emissivity | `1` | `dimensionless` | effective-unity exchange convention admitted by EB-01A/FSM2; atmospheric emissivity remains variable |

None of these constants is a user calibration coefficient in the admitted
model.

## Unit Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_a` | `K` | EB-02 runtime registry gap; climate owner must bind before implementation | none; kelvin required at boundary | no EB-02 runtime scalar; future typed temperature required | not published by EB-02 |
| `e_a` | `kPa` | EB-02 runtime registry gap | none; climate producer must publish `kPa` | no EB-02 runtime scalar; future typed pressure required | not published by EB-02 |
| `R_s`, `R_a` | `MJ m^-2 d^-1` | EB-02 runtime registry gap | none; operands must share exact units and daily window | no EB-02 runtime scalar; future typed daily radiation required | not published by EB-02 |
| `k_t`, `c` | `dimensionless fraction` | EB-02 runtime registry gap | none | no EB-02 runtime scalar; bounded fraction wrapper required | future diagnostic metadata must say `fraction` |
| `C` | `dimensionless fraction` | existing native canopy state; registry disposition owned by `SC-PLANT-001` | none | existing typed producer semantics; no new exception | existing canopy publication unchanged |
| `P_0`, `f_sky` | `dimensionless fraction` | EB-02 runtime registry gap | none | no EB-02 runtime scalar; bounded fraction wrapper required | future diagnostic metadata must say `fraction` |
| `w` | `kg m^-2` | internal derived operand; no boundary entry in EB-02 | equation-local `4650 e_a/T_a` only | package evidence scalar only; future typed internal value or recorded exception | not published by EB-02 |
| `epsilon_clear`, `epsilon_all` | `dimensionless` | EB-02 runtime registry gap | none | no EB-02 runtime scalar; bounded wrapper required | future diagnostic metadata must say `dimensionless` |
| `T_c`, `T_s` | `K` | EB-03 provider/registry gap | none; kelvin required at boundary | no EB-02 runtime scalar; future typed temperature required | future metadata must say `K` if published |
| `L_clear`, `L_atm`, `L_can`, `L_sub`, `L_out`, `L_net` | `W m^-2` | EB-02 runtime registry gap | none for flux; any energy integration must use an explicit seconds-duration helper | no EB-02 runtime scalar; future typed flux required | future component metadata must say `W m^-2`; not published by EB-02 |

Energy-carrier integration from `W m^-2` to a daily or subdaily energy increment
is owned by the future runtime package and must use an explicit duration in
seconds. This contract does not authorize a hidden factor of `86400`.

## Tolerance and Numeric Notes

- Analytical evidence uses an absolute tolerance of `1e-9` for dimensionless
  identity checks and `1e-6 W m^-2` for independently reconstructed fluxes.
- Runtime tolerances must be justified by the numeric types and integration
  cadence chosen in EB-03; they may not relax physical domains.
- Evaluate fourth powers in finite `f64`; reject non-finite intermediate
  values.
- The canonical canopy producer currently caps effective cover below one.
  The longwave consumer nevertheless must guard `C >= 1` rather than inventing
  its own epsilon clamp.
- The limit `C -> 1` is a scientific test; `C=1` is outside the admitted
  finite inversion domain.

## Calibration and Identifiability

Disposition: `CALIBRATION_NOT_APPLICABLE`.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`NOT_IMPLEMENTED` refers to executable production science: version 1 provides
canonical equation/interface authority and analytical evidence, while runtime
publication remains held. The two `NOT_APPLICABLE` fields reflect that EB-02
defines no empirically estimated parameter surface.

The admitted equations use fixed literature constants and existing forcing or
state variables. EB-02 performs no fitting and introduces no tunable
sky-view, extinction, canopy-temperature, or emissivity coefficient.

| Candidate | Calibration status | Reason |
|---|---|---|
| `1.6` diffuse multiplier | fixed | literature equation constant |
| `0.15`, `0.80`, `0.84` cloud mapping | fixed for canonical route | changing them would require a new authority/calibration package |
| `C` | externally produced state | governed by `SC-PLANT-001`, not fit here |
| `T_c`, `T_s` | provider states | EB-03 selection problem, not EB-02 calibration |
| `R_a,min` | unresolved numeric guard | implementation-threshold decision, not empirical calibration |

Identifiability warning: fitting an extinction coefficient after deriving
`f_sky` from effective cover would confound canopy-state calibration with the
radiative translation and is prohibited in this version.

## Test Vector Obligations

The implementation increment must reproduce package artifact
`analytical-test-vectors.csv` and include at least:

1. open canopy: `C=0`, `f_sky=1`, `L_sub=L_atm`;
2. intermediate cover values independently evaluated as `(1-C)^1.6`;
3. near-closed-canopy limiting behavior;
4. monotonic decrease of `f_sky` with increasing `C`;
5. clear and overcast clearness endpoints and both clamp sides;
6. independent reconstruction of Dilley clear-sky and all-sky fluxes;
7. complementary sky/canopy mixing and net-longwave sign;
8. typed rejection of invalid temperatures, cover, and forcing;
9. typed unavailable polar-night cloud inference;
10. distinct hourly-temperature fluxes under held daily vapor/cloud state and
    an explicit nonzero daily-mean-temperature substitution bias; and
11. runtime hold when the EB-03 thermal provider is absent.

Producer-only analytical vectors cannot close runtime activation. A later
runtime package must prove the real shared snow-energy consumer reads the
contracted operands.

## Binding Exposure Index

No binding addendum, sidecar amendment, companion implementation contract, or
provisional binding residue exists for version 1. The package row below maps
the originating evidence to authority promoted into this canonical core.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SNOWENERGY-EB02-AUTHORITY` | `docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-001, INV-SNOWENERGY-002, INV-SNOWENERGY-003, INV-SNOWENERGY-004, INV-SNOWENERGY-005, INV-SNOWENERGY-006, INV-SNOWENERGY-007, INV-SNOWENERGY-008, INV-SNOWENERGY-009, INV-SNOWENERGY-010, INV-SNOWENERGY-011, INV-SNOWENERGY-012, INV-SNOWENERGY-013, INV-SNOWENERGY-014` | `none` | Package-local source reconciliation and analytical artifacts are evidence; all binding equations, guards, and obligations are in this canonical contract. |

## Gap Register

| Gap ID | Gap | Owner | Required closure | Current disposition |
|---|---|---|---|---|
| `GAP-SNOWENERGY-001` | Shared `T_s` and cold-content provider is not selected. | `SNOW-SURFACE-EB-03` | Contract amendment or implementation record selecting one coherent provider and proving common-consumer use. | `RUNTIME HOLD` |
| `GAP-SNOWENERGY-002` | Canopy temperature uses no prognostic canopy energy balance. | future adjudication after EB-04 | Assess factorial sensitivity and field/literature evidence; retain `T_c=T_a` only within stated homogeneous-stand limits. | accepted approximation |
| `GAP-SNOWENERGY-003` | Polar-night cloud state cannot be inferred from daily clearness index. | EB-03/runtime package | Bind an authoritative alternate cloud input or typed unavailable policy. | `RUNTIME HOLD` for affected days |
| `GAP-SNOWENERGY-004` | `R_a,min` numeric threshold is not bound. | EB-03/runtime package | Choose a unit-explicit threshold and test the daylight transition. | open prerequisite |
| `GAP-SNOWENERGY-005` | Effective-cover translation has not been evaluated against hemispherical photography across heterogeneous stands. | future validation campaign | Compare without making observations a runtime prerequisite. | non-blocking research gap |
| `GAP-SNOWENERGY-006` | The Dilley-Unsworth review does not establish a transferable numeric meteorological input envelope for every openWEPP climate. | EB-03/runtime package and future validation | Enforce the no-clamp derived-emissivity guard; report extrapolation diagnostics and evaluate climate-envelope adequacy. | runtime prerequisite / validation gap |

## Change Log

| Version | Date | Change | Evidence |
|---:|---|---|---|
| 1 | 2026-07-30 | Initial contract: atmospheric longwave, effective-cover-derived diffuse sky view, complementary canopy exchange, runtime hold, and analytical obligations. | `SNOW-SURFACE-EB-01A` and `SNOW-SURFACE-EB-02` static/analytical evidence |
