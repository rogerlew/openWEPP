---
contract_id: SC-SNOWENERGY-001
title: Snow-Surface Energy and Sub-Canopy Longwave Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + snow-process reviewer
contract_version: 4
producer_scope:
  - Hourly atmospheric longwave evaluated from hourly temperature and daily vapor/cloud state
  - Native-canopy effective cover to diffuse sky-view translation
  - Complementary sky and canopy longwave incident at the snow surface
consumer_scope:
  - Shared Stage 3 snow-surface energy carrier
  - Snow sublimation and melt components
  - Snow-energy diagnostics and assurance outputs
evidence_level: static
last_reviewed: 2026-07-31
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

Version 3 replaced the failed version-2 snowfall-event top-layer provider with
the Marks/SNOBAL active thermal control volume. The active volume spans the
upper `min(z_s, 0.25 m)` of snow except for the version-4 terminal lower-volume
collapse, crosses depositional-layer boundaries
conservatively, and exchanges conductive heat with the remaining lower pack
inside each stability substep. Marks/SNOBAL mass-dependent timestep
subdivision is required. The production default remains unchanged. EB-03A
real-consumer evidence passes, but EB-04 remains blocked until EB-03A's
declared full-workspace quick and ADR-0043 Critical full profiles can pass.

Version 4 defines the terminal resolved thermal-layer domain. The exact
libsnobal `1 kg m^-2` threshold suspends Stage 3 exchange only when total pack
ice mass is at or below the boundary. In a resolved pack, a lower thermal
volume strictly below the same threshold collapses into a one-volume thermal
solve; equality remains a two-volume solve. CoE remains authoritative for snow
existence and mass, so persistent layer state is retained rather than
converted to water.

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

- Default activation, user-facing selectors, or public output-schema changes.
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
| `REF-SNOWENERGY-MARKS1999` | Marks et al. (1999), *Hydrological Processes* 13:1935-1959, doi: `10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C` | Two-layer SNOBAL energy balance, active-layer thermal state, conductive exchange, and progressively smaller shallow-layer timesteps. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-LIBSNOBAL` | CC0 libsnobal at `/home/workdir/pysnobal`, commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `_calc_layers.c`, `_adj_layers.c`, `_e_bal.c`, `g_snow.c`, `_divide_tstep.c`, `_below_thold.c`, and `snobal.h` | Equation-reference implementation for `z_s_0`, `G_0`, harmonic two-layer transfer, the `60/10/1 kg m^-2` mass-dependent `60/15/1 minute` timestep hierarchy, exact total-`<=`/lower-`<` terminal-layer ordering, and residual-snow phase disposition. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-LUTE2022` | Lute et al. (2022), *Geoscientific Model Development* 15:5045-5071, doi: `10.5194/gmd-15-5045-2022`, section 2.2.7 | Independent documentation that Marks et al. address shallow-snow energy instability with progressively smaller timesteps. SnowClim's alternative temperature replacement and fitted cold-content tax are not admitted. | `[DIRECT][Static]` |
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
| `z_0` | `m` | Active thermal-layer depth, normally `min(z_s, 0.25 m)`; whole-pack depth when `INV-SNOWENERGY-026` collapses a lower volume with `0 < m_l < 1 kg m^-2`. | Stage 3 thermal partition | shared energy carrier |
| `m_0` | `kg m^-2` | Snow-ice mass contained in `z_0`. | Stage 3 thermal partition | active-layer heat capacity |
| `T_0` | `K` | Heat-capacity-weighted active-layer temperature. | active-layer cold content | radiation and turbulent exchange |
| `T_l` | `K` | Heat-capacity-weighted lower-pack temperature when `z_s > z_0`. | lower-pack cold content | interface conduction |
| `G_0` | `W m^-2` | Conductive exchange, positive from the lower pack into the active layer. | coupled thermal provider | active/lower energy balances |
| `p_a` | `Pa` | Atmospheric pressure derived from run elevation. | climate metadata projection | SNOBAL effective snow conductivity |
| `k_d`, `k_eff` | `W m^-1 K^-1` | Dry Yen snow conductivity and Anderson pore-vapor-enhanced effective conductivity. | density, temperature, pressure | active/lower series resistance and `G_0` |
| `Q_cc` | `J m^-2` | Positive active-layer cold-content deficit relative to `0 degC` ice. | Stage 3 thermal partition | shared energy carrier |
| `m_v` | `kg m^-2` | Signed hourly vapor mass exchange; deposition positive, sublimation negative. | shared turbulent exchange | snow mass and latent-energy ledgers |
| `Q_E` | `J m^-2` | Hour-integrated applied surface energy, positive toward snow. | shared energy carrier | cold-content update |
| `m_res` | `kg m^-2` | Total ice-mass boundary above which the Marks/SNOBAL Stage 3 thermal layer is resolved. | fixed libsnobal threshold | Stage 3 domain branch |
| `t_unres` | `s` | Duration for which CoE snow exists below the resolved Stage 3 thermal domain. | Stage 3 domain branch | diagnostics and runtime evidence |
| `m_l` | `kg m^-2` | Ice mass in the selected lower thermal volume. | Stage 3 thermal partition | one/two-volume branch |
| `t_collapse` | `s` | Duration for which a resolved pack uses one thermal volume because `0 < m_l < m_res`. | Stage 3 layer branch | diagnostics and runtime evidence |
| `sigma` | `W m^-2 K^-4` | Stefan-Boltzmann constant. | fixed constant | emission equations |

## Algorithm State Surfaces

### Required inputs

| Surface | Required state |
|---|---|
| Above-canopy meteorology | hourly finite `T_a > 0 K`; daily finite `e_a >= 0 kPa` and `R_s >= 0 MJ m^-2 d^-1` |
| Solar geometry | finite `R_a >= 0 MJ m^-2 d^-1` plus an explicit daylight/polar-night classification |
| Canopy | finite effective daily `C` in `[0, 1)` |
| Thermal provider | supported internal `layered_thermal_liquid_v1`; finite active-layer `T_0 > 0 K`, non-negative finite active/lower cold content, conservative depositional-to-thermal partition, and `T_c=T_a` with the named approximation identity |

### Required outputs

`w`, `L_clear`, `epsilon_clear`, `k_t`, `c`, `epsilon_all`, `L_atm`,
`P_0`, `f_sky`, `L_can`, `L_sub`, `L_out`, and `L_net`, each with the
units and lineage declared above.

### Mutated state surfaces

The longwave evaluator is pure. It may not mutate canopy, snow mass, snow
temperature, cold content, or forcing state. The shared Stage 3 energy carrier
consumes `L_net` exactly once and is solely responsible for mutating cold
content. Optional sublimation uses the same pre-exchange `T_s`; signed vapor
exchange and latent heat are two views of one transfer.

## Algorithm Specification

Required evaluation order:

1. Validate cadence, units, finiteness, and physical input domains.
2. Once per daylight day, calculate `k_t` and the bounded cloud fraction `c`;
   otherwise take the explicit polar-night unavailable branch.
3. For each hour, evaluate `w`, `L_clear`, `epsilon_clear`,
   `epsilon_all`, and `L_atm` using hourly `T_a` and held daily `e_a`/`c`;
   enforce the no-clamp derived-emissivity guard.
4. Translate the current effective canopy cover to `P_0` and `f_sky`.
5. Partition the current snow column at
   `z_0=min(total_snow_depth, 0.25 m)` except for the `INV-SNOWENERGY-026`
   strict lower-volume collapse, integrating mass, heat capacity, cold
   content, and thermal resistance across depositional-layer boundaries.
   Obtain `T_s=T_0` from the active control volume and set `T_c=T_a` under the
   named homogeneous-stand approximation; stop on missing or invalid state.
6. Evaluate `L_can`, `L_sub`, `L_out`, and `L_net` in the specified order.
7. If sublimation is enabled, evaluate one signed vapor exchange at the same
   `T_s` and derive latent heat from that exact exchange.
8. Select the `60`, `15`, or `1 minute` stability substep from the
   Marks/SNOBAL active/lower mass thresholds and reevaluate `T_0`, vapor
   exchange, radiation, and `G_0` at every substep.
9. Sum shortwave, optional `L_net`, optional latent heat, and `G_0` in the
   active-layer balance; apply `-G_0` to the lower balance. Apply the bounded
   energy to cold content exactly once without converting positive excess to
   energy-balance melt.
10. Remove sublimated ice from the active surface downward, retaining it only
    as vapor export, and publish reconstruction operands after all guards pass.

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

When daylight permits a daily clearness index and
`R_a > R_a,min = 1e-9 MJ m^-2 d^-1`:

```text
k_t = R_s / R_a
c = clamp((0.80 - k_t) / (0.80 - 0.15), 0, 1)
epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c
L_atm = epsilon_all sigma T_a^4
```

`R_a,min` is a numeric divide/branch threshold, not a user coefficient or
empirical calibration parameter. The clamp belongs only to the declared
empirical cloud mapping. It must not repair a non-finite input or an invalid
radiation unit.

When `R_a <= R_a,min`, the clearness route is unavailable. Version 2 has no
independently authoritative polar-night cloud producer, so an enabled
longwave cell returns typed `CloudForcingUnavailable`. It must not reuse the
legacy SIMIMPL28 cloud fraction or a prior daylight value. Disabled longwave
cells do not require this operand.

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

### Shared Stage 3 active-layer thermal and sublimation composition

The sole version-3 thermal provider is the Marks/SNOBAL active control volume
constructed from the layers carried by
`snow_stage3_liquid_routing_model=layered_thermal_liquid_v1`:

```text
z_0 = min(sum_i(z_i), 0.25 m)
m_0 = integral over [0, z_0] of layer mass
Q_cc,0 = integral over [0, z_0] of layer cold content
T_s = T_0 = 273.15 K - Q_cc,0 / (m_0 c_i)
T_c = T_a
```

The normal `0.25 m` maximum is fixed Marks/SNOBAL structural authority, not a
user coefficient. The sole exception is the exact version-4 lower-volume
collapse, which makes the complete resolved pack active for that substep.
Depositional boundaries do not define the radiating/turbulent
heat capacity. A depositional layer intersected by `z_0` is partitioned
conservatively; mass, depth, liquid, refrozen mass, and cold content must close
before flux evaluation. All material within the active thermal volume shares
the resulting `T_0` after projection. When the pack is no deeper than
`0.25 m`, the complete pack is the active control volume.

When a lower volume exists, derive its aggregate temperature and effective
conductivity from its mass, cold content, and series thermal resistance.
Thermal partitions persist across substeps; a boundary-intersected
depositional layer must not be recombined in a way that erases the current
active/lower temperature gradient.

Use the exact libsnobal `KTS` plus `efcon` conductivity formulation, not the
Sturm density-only frost-insulation relation:

```text
p_a = 101300 Pa * (1 - 0.0065 z_elev / 293 m)^5.26
rho_r = rho_s / (1000 kg m^-3)
k_d = 4.186798188 * 0.0077 * rho_r^2
D_e = 0.65 * (101324.6 Pa / p_a) * (T_s / 273.16 K)^14 * 1e-4 m^2 s^-1
w_s = (18.0153 / 28.9644) * e_si(T_s) / (p_a - e_si(T_s))
k_eff = k_d + L_s(T_s) D_e w_s
```

Here `e_si` is the admitted SNOBAL ice-saturation relation already used by
the vapor provider. Elevation is existing climate metadata; `p_a` is derived
internally and is not a user coefficient. Each volume's series resistance
uses its current shared temperature. The SNOBAL harmonic interface exchange
is:

```text
G_0 = 2 k_0 k_l (T_l - T_0) / (k_l z_0 + k_0 z_l)
```

where positive `G_0` supplies heat to the active layer. The same transfer is
`-G_0` in the lower-layer balance; it cancels exactly in the whole-pack
ledger. A transfer that would warm the receiving control volume above
`0 degC` is bounded by available cold content because this candidate preserves
the CoE melt boundary. The rejected excess is reported, not converted to melt
and not debited from the donor.

Use the minimum active/lower control-volume mass to select the stability
timestep:

```text
m_min >= 60 kg m^-2  -> 60 minute substeps
10 <= m_min < 60     -> 15 minute substeps
m_min < 10           -> 1 minute substeps
```

If no lower volume exists, `m_min=m_0`. The `1 kg m^-2` small-timestep
threshold is also the exact libsnobal terminal resolved-pack boundary. Before
partitioning or constructing `T_0`, conductivity, or the next carrier,
calculate total represented ice mass `m_s`. If
`m_s <= m_res = 1 kg m^-2`, the Stage 3 thermal and exchange domain is
unresolved for the remainder of the hour:

```text
Q_shortwave = Q_longwave = Q_latent = 0
m_v = m_sub = 0
G_0 = Q_E = 0
t_unres += remaining hour duration
```

When `m_s > m_res`, select the normal active/lower partition. If a lower volume
exists and `0 < m_l < m_res`, do not evaluate that sub-resolution volume.
Collapse the thermal partition to one whole-pack active volume for the current
substep and continue the ordinary surface-energy solve. Existing conservative
projection may coalesce thermally identical fragments, but total mass, liquid,
refrozen mass, and cold content must remain closed:

```text
active thermal volume = complete represented pack
lower thermal volume = none
t_collapse += substep duration
```

The lower-volume comparison is strict: `m_l = 1 kg m^-2` remains a resolved
two-volume solve. This reproduces libsnobal `_calc_layers.c` ordering and
branch sides; it is distinct from the `m_s <= 1 kg m^-2` no-layer branch.

This translation adopts libsnobal's resolved-layer boundary but not its
residual-snow-to-water conversion. CoE remains authoritative for snow
existence, melt, and liquid routing. Persistent layer mass, liquid, refrozen
mass, and cold content therefore remain unchanged by the suspended Stage 3
exchange. Existing conservative projection may change them only when an
authoritative CoE mass or phase update occurs. If later snowfall or another
authoritative mass update makes `m_s > m_res`, normal Stage 3 partitioning and
substeps resume from the retained projected state. The unresolved branch must not create a
temperature, evaluate ice saturation or conductivity, route unresolved mass as
melt, reset cold content, or apply one final energy/vapor debit.

All four `B/L/S/LS` cells use this provider and the same CoE melt, density,
phase, liquid-routing, forcing, and albedo selections. Longwave and
sublimation are separate default-off selectors. Enabling either without the
Stage 3 provider is a typed missing-provider error. Enabling the new
sublimation selector together with legacy
`coe_open_sublimation_stage_a_v1` or
`coe_open_sublimation_stage_b_v1` is a typed incompatible-selection error,
preventing double mass loss.

For the retained Marks/SNOBAL-lineage neutral exchange, calculate one
loss-positive substep sublimation amount `m_sub >= 0` from the same `T_s` used
by longwave, then define:

```text
m_v = -m_sub
q_v = m_v / delta_t
Q_latent = q_v L_s(T_s)
```

where `delta_t` is the selected substep duration, `q_v` is in
`kg m^-2 s^-1`, and `L_s(T_s)` is the temperature-appropriate latent heat of
sublimation in `J kg^-1`. Thus
`Q_latent <= 0` during sublimation. The implementation must derive both mass
and energy from `q_v`; it may not independently recompute either view.

Each substep carrier is:

```text
Q_surface = Q_shortwave
            + I_L L_net
            + I_S Q_latent
Q_E,0,potential = (Q_surface + G_0) delta_t
Q_E,l,potential = -G_0 delta_t
```

where `I_L` and `I_S` are the independent longwave and sublimation selector
indicators. Stage 3 may apply only the portion that changes cold content under
the existing no-energy-balance-melt boundary; unused positive potential is
reported and is not converted to melt. Sublimated ice is removed from the
active surface downward after the coupled update. Cold content associated
with that removed ice is a separately reported energy export. Active and
lower ledgers retain `G_0` with opposite signs; it cancels in whole-pack
closure:

```text
Q_surface,applied + Q_refreeze + Q_cc,export
    = Q_cc,before - Q_cc,after
```

and post-CoE vapor mass closure is:

```text
M_ice,before - M_ice,after = M_sublimation
```

Sublimation must not enter routed melt, retained liquid, released liquid, or
refreeze operands.

## Branch and Guard Table

| Branch/condition | Required behavior | Guard class | Failure class |
|---|---|---|---|
| Any required scalar is non-finite | Reject before arithmetic. | runtime | typed invalid forcing/state |
| `T_a <= 0 K`, `T_c <= 0 K`, or `T_s <= 0 K` | Reject. | runtime | typed invalid temperature |
| `e_a < 0 kPa`, `R_s < 0`, or `R_a < 0` | Reject. | runtime | typed invalid forcing |
| Derived `L_clear`, `epsilon_clear`, `epsilon_all`, or `L_atm` is non-finite, or either emissivity is outside `[0,1]` | Reject without clamping. | runtime | typed out-of-authority atmospheric state |
| Daylight and `R_a > R_a,min` | Calculate `k_t`, clamp only the empirical cloud mapping, and continue. | runtime | none |
| Polar night or `R_a <= 1e-9 MJ m^-2 d^-1` with longwave enabled | Do not divide, reuse SIMIMPL28 cloud fraction, or carry a prior value; return typed unavailable state. | runtime | typed cloud-forcing unavailable |
| `C < 0` or `C >= 1` | Reject; do not silently clamp. | runtime | typed invalid canopy state |
| `C = 0` | Require `f_sky = 1` and `L_sub = L_atm`. | test | blocked promotion on mismatch |
| `C -> 1` within valid domain | Require `f_sky -> 0` and `L_sub -> L_can`. | test | blocked promotion on mismatch |
| Stage 3 thermal provider absent while longwave or sublimation is enabled | Do not publish or mutate candidate energy/mass state. | runtime | typed missing thermal provider |
| New sublimation selector plus legacy Stage A/B melt variant | Reject before hourly processing. | runtime | typed incompatible selector |
| Sublimation demand exceeds available ice | Bound the transfer to available ice and derive latent energy from the bounded transfer. | runtime | none; bounded physical availability |
| Total snow depth is `<= 0.25 m` | Use the complete pack as the active thermal control volume. | runtime | none |
| Total represented ice mass is `<= 1 kg m^-2` | Before thermal partition, preserve persistent CoE/layer state and suspend all Stage 3 thermal, radiation, conduction, and vapor exchange without constructing temperature or conductivity. | runtime/model domain | explicit unresolved-duration and total-mass diagnostics; no typed thermal failure |
| Total mass is `> 1 kg m^-2` and `0 < m_l < 1 kg m^-2` | Conservatively project to one whole-pack thermal volume and continue normal exchange; thermally identical fragments may coalesce only with closed aggregate state. | runtime/model domain | explicit collapse-duration and lower-mass diagnostics |
| Lower thermal mass is exactly `1 kg m^-2` | Retain the resolved active/lower two-volume solve. | runtime/model domain | no collapse diagnostic; ordinary coupled-state guards |
| A depositional layer crosses `z_0` | Partition/project conservatively; reject nonclosing state. | runtime | typed thermal-partition closure failure |
| Active/lower mass selects a smaller timestep | Execute every required substep and reevaluate the coupled state; do not retain an hourly energy debit. | runtime | typed cadence/closure failure |
| Coupled update would require `T <= 0 K` | Reject; no clamp, temperature replacement, or cold-content tax is allowed. | runtime | typed invalid thermal state / blocked campaign |
| `T_c = T_a` approximation active | Emit/retain explicit approximation identity in configuration or diagnostics. | profile | blocked promotion if unlabeled |
| Canopy is outside equivalent homogeneous/random-orientation/isotropic-diffuse regime | Do not expand the claim; retain a diagnostic/model-limitation classification. | governance | model limitation |

`R_a,min` is the numerically explicit `1e-9 MJ m^-2 d^-1`
divide/branch threshold.

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
| `INV-SNOWENERGY-015` | All `B/L/S/LS` cells use the same Stage 3 top-layer `T_s`/cold-content provider; `T_c=T_a` is explicitly identified. | `REF-SNOWENERGY-EB01A`, `REF-SNOWENERGY-RUTTER2023`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085` | `[DIRECT][Static] + [INFERENCE][Static]` | selector/provider guard and real-consumer test | typed missing provider / blocked campaign |
| `INV-SNOWENERGY-016` | Longwave and sublimation are orthogonal default-off selectors; neither changes the CoE melt-model selector. | snow-surface EB roadmap | `[DIRECT][Static]` | selector matrix test | blocked EB-04 admission |
| `INV-SNOWENERGY-017` | Signed vapor mass and latent heat are derived from one bounded exchange at the shared `T_s`; sublimation is negative latent energy and cannot be debited twice. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent latent/mass reconstruction | typed closure failure |
| `INV-SNOWENERGY-018` | Sublimation reduces ice storage only and never aliases routed melt, retained/released liquid, or refreeze. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-073`, `#INV-SNOWFREEZE-076` | `[DIRECT][Static]` | independent mass closure and alias-separation test | typed closure failure |
| `INV-SNOWENERGY-019` | Cold-content change closes from applied surface energy, interlayer conduction, refreeze energy, and exported cold content on the declared control volume. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-080`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent energy reconstruction | typed closure failure |
| `INV-SNOWENERGY-020` | The radiating/turbulent control volume is normally the upper `min(z_s,0.25 m)` of snow and is independent of snowfall-event boundaries; `INV-SNOWENERGY-026` exclusively authorizes whole-pack depth for the strict sub-resolution-lower-volume collapse. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | active-layer partition and anti-alias test | typed partition failure |
| `INV-SNOWENERGY-021` | Active/lower mass, depth, cold content, and thermal resistance reconstruct the persistent column exactly before and after projection. | physical conservation, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static] + [INFERENCE][Static]` | independent partition reconstruction | typed closure failure |
| `INV-SNOWENERGY-022` | `G_0` is positive into the active layer, appears as `-G_0` in the lower balance, and cancels from the whole-pack ledger. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | sign, limiting, and reconstruction tests | typed closure failure |
| `INV-SNOWENERGY-023` | Mass-dependent `60/15/1 minute` substeps are selected from the `60/10/1 kg m^-2` Marks/SNOBAL thresholds; substep fluxes are reevaluated from current state. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-LUTE2022` | `[DIRECT][Static]` | cadence and thin-pack tests | typed cadence failure / blocked campaign |
| `INV-SNOWENERGY-024` | No active/lower update may use an absolute-zero clamp, air-temperature replacement, fitted cold-content tax, or user limiter. | `REF-SNOWENERGY-PHYSICAL`, EB-03A authority envelope | `[INFERENCE][Static]` | source scan and physical-domain tests | hard `HOLD` |
| `INV-SNOWENERGY-025` | Active and lower partitions retain distinct shared temperatures across substeps, and `G_0` uses libsnobal `KTS+efcon` effective conductivity with elevation-derived pressure; the Sturm frost-insulation relation is not an admissible substitute. | `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-ANDERSON1976` | `[DIRECT][Static]` | unequal-temperature persistence and conductivity vectors | typed conductivity/projection failure |
| `INV-SNOWENERGY-026` | At total represented ice mass `m_s <= 1 kg m^-2`, the Stage 3 thermal/exchange domain is unresolved before partition: CoE and persistent-layer mass, liquid, refrozen mass, cold content, and topology are preserved; no temperature, conductivity, surface energy, conduction, vapor exchange, sublimation, or melt alias is produced. For `m_s > 1`, `0 < m_l < 1 kg m^-2` collapses to one whole-pack thermal volume and continues exchange, while `m_l = 1` remains two-volume. | `REF-SNOWENERGY-LIBSNOBAL`, CoE ownership, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | exact total-mass threshold sides, strict lower-layer collapse/equality, state-preservation, resume, and real-consumer trace tests | typed closure failure above boundary / blocked campaign on alias or mutation |

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
| `INV-SNOWENERGY-015` | Stage 3 selector/provider validation and hourly diagnostics | runtime/profile | typed missing provider or blocked campaign | EB-03 runtime tests |
| `INV-SNOWENERGY-016` | direct-production selector matrix | runtime/test | typed selector failure or blocked EB-04 | EB-03 consumer-path evidence |
| `INV-SNOWENERGY-017` | shared vapor/latent operands and reconstruction residual | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-018` | snow-layer/aggregate mutation and liquid-alias separation | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-019` | Stage 3 cold-content closure ledger | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-020` | active-volume constructor and surface-provider diagnostics | runtime/test | typed partition failure | EB-03A evidence |
| `INV-SNOWENERGY-021` | independent persistent/thermal partition reconstruction | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-022` | coupled active/lower energy update | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-023` | mass-selected substep scheduler | runtime/test | typed cadence failure | EB-03A real-consumer evidence |
| `INV-SNOWENERGY-024` | production-source scan and invalid-state vectors | governance/test | hard `HOLD` | EB-03A review and verification |
| `INV-SNOWENERGY-025` | SNOBAL effective-conductivity primitive and persistent unequal-temperature runtime vector | runtime/test | typed conductivity/projection failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-026` | pre-temperature total-ice-mass branch plus unresolved-domain trace diagnostics | runtime/test/profile | preserve state and emit zero exchange below boundary; typed guards remain above boundary | EB-04C contract, replay, and conservation evidence |

## Producer and Consumer Obligations

| Obligation ID | Role | Requirement |
|---|---|---|
| `OBL-SNOWENERGY-P-001` | climate producer | Publish hourly `T_a` plus daily `e_a` and `R_s` with declared units, cadence, and finite-domain validation. |
| `OBL-SNOWENERGY-P-002` | solar-geometry producer | Publish `R_a` and explicit daylight/polar-night classification. |
| `OBL-SNOWENERGY-P-003` | canopy producer | Publish one effective daily plan-view canopy cover `C`; preserve its leaf-on/leaf-off and structural-floor semantics. |
| `OBL-SNOWENERGY-P-004` | Stage 3 thermal producer | Above `m_res`, publish active-layer `T_s`, mass, depth, cold content, lower state when present, and explicitly identified `T_c=T_a`, or a typed unavailable result. At or below `m_res`, publish unresolved duration/mass without fabricating thermal state. |
| `OBL-SNOWENERGY-P-005` | sublimation exchange | Publish one bounded signed vapor mass exchange and derive its latent heat using the same `T_s`. |
| `OBL-SNOWENERGY-C-001` | longwave evaluator | Apply the equations and guards in the specified order without silent unit conversion or fallback. |
| `OBL-SNOWENERGY-C-002` | shared energy carrier | Consume `L_net` exactly once with the positive-toward-snow convention. |
| `OBL-SNOWENERGY-C-003` | sublimation/melt consumers | Use the same EB-03 snow state as longwave; do not reconstruct an independent surface temperature. |
| `OBL-SNOWENERGY-C-004` | diagnostics | Preserve component operands sufficient to reconstruct `L_atm`, `f_sky`, `L_sub`, `L_out`, and `L_net`. |
| `OBL-SNOWENERGY-C-005` | configuration/reporting | Identify when `T_c=T_a` is used and communicate its approximation limits. |
| `OBL-SNOWENERGY-C-006` | runtime implementation package | Prove the real common `B/L/S/LS` Stage 3 consumer reads this path before claiming activation. |
| `OBL-SNOWENERGY-C-007` | snow state | Remove sublimated mass from layer and aggregate ice state only, publish cold-content export, and reject double-selector composition. |
| `OBL-SNOWENERGY-C-008` | thermal partition | Reconstruct the Marks/SNOBAL active volume independently of depositional boundaries and conserve projection operands. |
| `OBL-SNOWENERGY-C-009` | coupled solver | Apply equal-and-opposite `G_0` within each selected stability substep and close active/lower and whole-pack energy. |
| `OBL-SNOWENERGY-C-010` | stability scheduler | Select `60/15/1 minute` substeps from the fixed mass thresholds and reevaluate fluxes from current substep state. |
| `OBL-SNOWENERGY-C-011` | conductive provider | Publish current active/lower state, applied `G_0`, cadence, and separate active/lower/cancellation residuals from the production solve. |

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
| `T_s`, `T_0` | `snow_surface_temperature_k` | active thermal provider to snow emission | `K` -> `K` | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Heat-capacity-weighted Marks/SNOBAL active-layer temperature. |
| `G_0` | `snow_active_lower_conduction_w_m2` | lower pack to active thermal volume | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive into active; equal negative lower operand. |
| `L_atm` | `atmospheric_longwave_w_m2` | atmosphere to sub-canopy mixture | `W m^-2` -> same | `SC-SNOWENERGY-001` | Hourly all-sky downward longwave above canopy. |
| `L_sub` | `subcanopy_longwave_w_m2` | mixture to snow energy | `W m^-2` -> same | `SC-SNOWENERGY-001` | Downward longwave incident at snow. |
| `L_net` | `net_longwave_w_m2` | longwave to shared energy carrier | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive toward snow. |
| `Q_cc` | `cold_content_j_m2` | Stage 3 layer to shared energy carrier | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive energy deficit relative to `0 degC` ice. |
| `m_v` | `vapor_mass_exchange_kg_m2` | turbulent exchange to snow state | `kg m^-2` -> same | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Signed exchange; sublimation negative. |
| `Q_E` | `applied_surface_energy_j_m2` | shared carrier to cold content | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive toward snow. |

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
| `R_a,min` | `1e-9` | `MJ m^-2 d^-1` | EB-03 numeric divide/branch threshold; not a fit parameter |
| hourly duration | `3600` | `s` | typed hourly forcing cadence and named time conversion |
| normal maximum active-layer depth | `0.25` | `m` | fixed Marks/SNOBAL `max_z_s_0`; exceeded only by the exact `INV-SNOWENERGY-026` lower-volume collapse; not a user coefficient |
| libsnobal sea-level pressure | `101324.6` | `Pa` | fixed `SEA_LEVEL` constant used by `efcon` |
| libsnobal dry-snow conductivity factor | `4.186798188 * 0.0077` | `W m^-1 K^-1` | exact `CAL_TO_J(0.0077)` factor in `KTS`; density enters as `(rho/1000)^2` |
| normal mass threshold | `60` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| medium mass threshold | `10` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| minimum resolved thermal mass, `m_res` | `1` | `kg m^-2` | exact libsnobal threshold: total mass `<=` suspends; lower-volume mass `<` collapses to one volume; lower-volume equality remains two-volume |
| medium duration | `900` | `s` | fixed `15 minute` Marks/SNOBAL level |
| small duration | `60` | `s` | fixed `1 minute` Marks/SNOBAL level |

None of these constants is a user calibration coefficient in the admitted
model.

## Unit Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_a` | `K` | typed `TemperatureCelsius` boundary with named kelvin conversion | temperature helper | none | internal diagnostic only |
| `e_a` | `kPa` | typed non-negative vapor-pressure wrapper | daily dewpoint-to-vapor helper | none | internal diagnostic only |
| `R_s`, `R_a` | `MJ m^-2 d^-1` | typed non-negative daily-radiation wrapper | named hourly-sum and solar producer | none | internal diagnostic only |
| `k_t`, `c` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `C` | `dimensionless fraction` | existing native canopy state; registry disposition owned by `SC-PLANT-001` | none | existing typed producer semantics; no new exception | existing canopy publication unchanged |
| `P_0`, `f_sky` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `w` | `kg m^-2` | internal derived operand; no boundary entry in EB-02 | equation-local `4650 e_a/T_a` only | package evidence scalar only; future typed internal value or recorded exception | not published by EB-02 |
| `epsilon_clear`, `epsilon_all` | `dimensionless` | typed bounded fraction | none | none | internal diagnostic only |
| `T_c`, `T_s` | `K` | typed `TemperatureCelsius` provider | named kelvin conversion | none | internal diagnostic only |
| `L_clear`, `L_atm`, `L_can`, `L_sub`, `L_out`, `L_net` | `W m^-2` | typed energy/radiative flux wrappers | named hourly integration helper | none | internal diagnostic only |
| `Q_cc`, `Q_E` | `J m^-2` | Stage 3 scalar with contract-bound guard | no conversion | retained scalar exception: internal area-normalized energy ledger | internal diagnostic only |
| `m_v` | `kg m^-2` | Stage 3 scalar with contract-bound guard | named mass-flux hourly integration | retained scalar exception: internal area-normalized mass ledger | internal diagnostic only |
| `m_res` | `kg m^-2` | fixed internal model-domain constant | named SWE-to-mass conversion using `rho_w` | no user boundary or scalar exception | contract and environment-gated trace metadata |
| `t_unres` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `m_l` | `kg m^-2` | Stage 3 lower-volume diagnostic scalar | named SWE-to-mass conversion using `rho_w` | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `t_collapse` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `p_a` | `Pa` | typed positive pressure wrapper | named elevation-to-pressure projection | none | environment-gated research trace |
| `k_d`, `k_eff` | `W m^-1 K^-1` | typed positive thermal-conductivity wrapper | exact named `KTS+efcon` helper | none | environment-gated research trace operands |

Energy-carrier integration from `W m^-2` uses the explicit typed hourly
duration of `3600 s`; a hidden daily factor of `86400` is prohibited.

## Tolerance and Numeric Notes

- Analytical evidence uses an absolute tolerance of `1e-9` for dimensionless
  identity checks and `1e-6 W m^-2` for independently reconstructed fluxes.
- Runtime mass closure uses `1e-9 m` water equivalent and energy closure uses
  `1e-6 J m^-2`, matching the existing Stage 3 ledgers. These tolerances do not
  relax physical domains.
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
science_implementation_status = IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`IMPLEMENTED` applies to the canonical longwave equations, their default-off
diagnostic/reproduction seam, and the version-3 active-layer coupled provider.
EB-03A production, analytical reconstruction, and real B/L/S/LS consumer gates
pass. The two `NOT_APPLICABLE` fields reflect that this contract defines no
empirically estimated parameter surface.

The admitted equations use fixed literature constants and existing forcing or
state variables. EB-03 performs no fitting and introduces no tunable
sky-view, extinction, canopy-temperature, or emissivity coefficient.

| Candidate | Calibration status | Reason |
|---|---|---|
| `1.6` diffuse multiplier | fixed | literature equation constant |
| `0.15`, `0.80`, `0.84` cloud mapping | fixed for canonical route | changing them would require a new authority/calibration package |
| `C` | externally produced state | governed by `SC-PLANT-001`, not fit here |
| `T_c`, `T_s` | provider states | Stage 3 / air-temperature approximation, not fitted |
| `R_a,min` | fixed numeric guard | divide/branch threshold, not empirical calibration |

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
11. typed failure when the Stage 3 thermal provider is absent;
12. all four orthogonal selector cells with identical non-candidate settings;
13. independent latent/mass equivalence and wrong-sign rejection;
14. sublimation mass closure with explicit non-aliasing to liquid/melt; and
15. cold-content energy closure including exported cold content;
16. active depth and mass reconstructed across at least three depositional
    layers with a boundary-crossing split;
17. a shallow pack whose complete mass, rather than a thin snowfall-event
    layer, supplies the active heat capacity;
18. `G_0` sign, equal-and-opposite active/lower ledgers, isothermal zero-flux,
    and harmonic-conductivity reconstruction;
19. exact `60/15/1 minute` transitions around `60/10/1 kg m^-2`; and
20. a thin-pack chronology proving substep reevaluation and rejecting hourly
    debit, absolute-zero clamp, air-temperature replacement, and cold-content
    tax alternatives; and
21. unequal depositional temperatures proving one shared active `T_0`, a
    distinct persistent lower temperature, nonzero correctly signed `G_0`,
    exact active/lower cancellation, and exact libsnobal `KTS+efcon`
    conductivity rather than the Sturm frost relation; and
22. exact total `m_s < 1`, `m_s = 1`, and `m_s > 1 kg m^-2` vectors proving
    pre-partition zero exchange and unchanged persistent state below/at the
    boundary; plus lower-volume `m_l < 1` collapse, `m_l = 1` two-volume
    equality, normal evaluation/resume, explicit runner diagnostics, and
    rejection of forced melt, deletion, temperature clamp, epsilon vapor
    pressure, and one-more-flux aliases.

Producer-only analytical vectors cannot close runtime activation. EB-03 must
prove the real shared Stage 3 snow-energy consumer reads the contracted
operands.

## Binding Exposure Index

No binding addendum, sidecar amendment, companion implementation contract, or
provisional binding residue exists for version 2. The package rows below map
the originating evidence to authority promoted into this canonical core.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SNOWENERGY-EB02-AUTHORITY` | `docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-001, INV-SNOWENERGY-002, INV-SNOWENERGY-003, INV-SNOWENERGY-004, INV-SNOWENERGY-005, INV-SNOWENERGY-006, INV-SNOWENERGY-007, INV-SNOWENERGY-008, INV-SNOWENERGY-009, INV-SNOWENERGY-010, INV-SNOWENERGY-011, INV-SNOWENERGY-012, INV-SNOWENERGY-013, INV-SNOWENERGY-014` | `none` | Package-local source reconciliation and analytical artifacts are evidence; all binding equations, guards, and obligations are in this canonical contract. |
| `SNOWENERGY-EB03-COMPOSITION` | `docs/work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-015, INV-SNOWENERGY-016, INV-SNOWENERGY-017, INV-SNOWENERGY-018, INV-SNOWENERGY-019` | `none` | Package evidence binds the Stage 3 provider, orthogonal selectors, and mass/energy composition implemented by version 2. |
| `SNOWENERGY-EB03A-COUPLING` | `docs/work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-020, INV-SNOWENERGY-021, INV-SNOWENERGY-022, INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-025` | `none` | Package evidence must implement and verify the version-3 active thermal control volume and coupled substep solver. |
| `SNOWENERGY-EB04C-THERMAL-DOMAIN` | `docs/work-packages/20260731-snow-surface-eb-04c-thin-pack-thermal-domain-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-026` | `dual review and verification required` | Package evidence must implement and verify the exact minimum-resolved-mass branch without importing libsnobal's phase conversion or weakening typed guards. |

## Gap Register

| Gap ID | Gap | Owner | Required closure | Current disposition |
|---|---|---|---|---|
| `GAP-SNOWENERGY-001` | Shared `T_s` and cold-content provider selection. | `SNOW-SURFACE-EB-03` | Stage 3 top-layer provider plus common-consumer proof. | candidate selected; common-consumer proof failed / runtime `HOLD` |
| `GAP-SNOWENERGY-002` | Canopy temperature uses no prognostic canopy energy balance. | future adjudication after EB-04 | Assess factorial sensitivity and field/literature evidence; retain `T_c=T_a` only within stated homogeneous-stand limits. | accepted approximation |
| `GAP-SNOWENERGY-003` | Polar-night cloud state cannot be inferred from daily clearness index. | future authoritative cloud producer | Version 2 returns typed unavailable when longwave is enabled and `R_a <= R_a,min`; a future producer requires contract amendment. | bounded runtime limitation |
| `GAP-SNOWENERGY-004` | `R_a,min` numeric threshold. | `SNOW-SURFACE-EB-03` | Unit-explicit `1e-9 MJ m^-2 d^-1` and transition tests. | resolved in version 2 |
| `GAP-SNOWENERGY-005` | Effective-cover translation has not been evaluated against hemispherical photography across heterogeneous stands. | future validation campaign | Compare without making observations a runtime prerequisite. | non-blocking research gap |
| `GAP-SNOWENERGY-006` | The Dilley-Unsworth review does not establish a transferable numeric meteorological input envelope for every openWEPP climate. | future validation | Enforce the no-clamp derived-emissivity guard; report extrapolation diagnostics and evaluate climate-envelope adequacy. | implemented guard / validation gap |
| `GAP-SNOWENERGY-007` | The Stage 3 cold-content carrier used a snowfall-event top layer instead of the Marks/SNOBAL active thermal control volume and applied hourly surface energy outside a mass-dependent coupled active/lower substep. | `SNOW-SURFACE-EB-03A` | Implement and independently test version-3 active-volume construction, `G_0`, conservative projection, and stability substeps without a clamp, fitted limiter, new user coefficient, or changed frozen controls. | resolved in version 3; real B/L/S/LS and rollback cells pass |
| `GAP-SNOWENERGY-008` | Stage 3 continued thermal/exchange evaluation below libsnobal's minimum resolved layer mass, producing 17 impossible temperatures and five valid-Kelvin vapor-pressure underflows. | `SNOW-SURFACE-EB-04C` | Apply the exact fixed resolved-layer boundary before temperature/conductivity evaluation while preserving CoE mass and persistent cold content; prove all 22 captured thermal failures pass their original boundary. | resolved in version 4; 22/22 captured failures pass their formerly rejected processing day with zero forbidden thermal errors |

## Change Log

| Version | Date | Change | Evidence |
|---:|---|---|---|
| 4 | 2026-07-31 | Defined the exact libsnobal `1 kg m^-2` branches. Total mass `<=1` suspends before partition while CoE retains snow state; in a resolved pack, lower mass `<1` collapses to one thermal volume and continues, while lower equality remains two-volume. Both branches publish explicit diagnostics. | `SNOW-SURFACE-EB-04C` authority reconciliation and required runtime replay |
| 3 | 2026-07-30 | Replaced the failed snowfall-event top-layer provider with the Marks/SNOBAL upper-`0.25 m` active thermal control volume, harmonic active/lower `G_0`, conservative depositional-layer projection, and mass-dependent `60/15/1 minute` substeps. The amendment retains CoE snow existence/melt authority and prohibits shallow-pack temperature replacement, cold-content tax, fitted limiter, or new user coefficient. | `SNOW-SURFACE-EB-03A` contract-first authority trace |
| 2 | 2026-07-30 | Selected the Stage 3 top-layer thermal provider; bound `T_c=T_a`, polar-night typed unavailability, `R_a,min`, orthogonal default-off selectors, exact-one vapor/latent composition, snow-state mutation, and mass/energy closure obligations. Real S/LS execution then retained the seam as diagnostic/reproduction-only and opened `GAP-SNOWENERGY-007` because the common provider reaches `0 K` with material SWE remaining. | `SNOW-SURFACE-EB-03` contract-first implementation and terminal consumer evidence |
| 1 | 2026-07-30 | Initial contract: atmospheric longwave, effective-cover-derived diffuse sky view, complementary canopy exchange, runtime hold, and analytical obligations. | `SNOW-SURFACE-EB-01A` and `SNOW-SURFACE-EB-02` static/analytical evidence |
