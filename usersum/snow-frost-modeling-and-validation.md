# Snow and Frost in openWEPP — Modeling and Validation

*Audience: hydrologists and scientific reviewers evaluating openWEPP's snowpack
and soil-frost behavior, and how it differs from legacy WEPP.*

This document describes (1) how openWEPP simulates the winter processes —
snowpack accumulation, density, melt, precipitation phase, and seasonally frozen
soil — and how each differs from legacy WEPP; and (2) the methodology by which
those processes were implemented and evaluated against field observations,
including the rubric used to score legacy WEPP and openWEPP side by side and its
results. It states both what is validated and what remains bounded uncertainty,
because for a scientific user the second is as important as the first.

A note on terminology: specialized terms are defined at first use. *Snow water
equivalent (SWE)* is the depth of liquid water that would result from completely
melting the snowpack. *Meteorological forcing* (or simply *forcing*) is the
sequence of weather inputs — precipitation, temperature, radiation, wind,
humidity — that drives the model. *Ablation* is the loss of snowpack mass by melt
and sublimation.

---

## 1. Modeling philosophy (read this first)

openWEPP's winter physics were not ported line-for-line from legacy WEPP. They
were re-derived from published process descriptions and **evaluated against
observed data**, with legacy WEPP used only as a diagnostic reference. Five
principles govern everything that follows.

1. **Process descriptions are the authority.** Each process is governed by a
   written specification with explicit physical relationships and acceptance
   tolerances; that specification — not a reference executable — defines correct
   behavior.
2. **Observed data is the evaluation authority.** Snowpack and frost behavior are
   scored against field measurements (snow-pillow and frost-tube networks, and
   soil-temperature records), not against legacy model output.
3. **Legacy WEPP is a diagnostic reference, not a benchmark to reproduce.** Where
   openWEPP and legacy disagree, that disagreement is an investigation trigger,
   not a verdict. Legacy WEPP contains documented errors and inactive routines
   (Sections 3.7 and 4.5), so agreement with legacy is never the acceptance
   criterion. Disagreements are adjudicated symmetrically — a divergence may
   indicate an openWEPP error, a legacy error, a measurement/comparison mismatch,
   or remain unresolved.
4. **Conservation is a hard constraint.** Mass and energy must balance to machine
   precision regardless of any fit to observations. Composite snow-state closure
   residuals run at roughly 1 × 10⁻¹⁶ m.
5. **Distinguish forcing-limited from forcing-robust quantities.** Some quantities
   cannot be validated precisely because the *input forcing itself* is uncertain —
   for example, gauge undercatch of snowfall, the lapse (extrapolation) of gridded
   precipitation to the hillslope, and stochastically generated sub-daily storm
   intensity. Absolute SWE and snow-depth **magnitude** are forcing-limited, so a
   modeled–observed magnitude difference can reflect input uncertainty rather than
   model error. By contrast, **event timing, snow density, the shape of the
   seasonal trajectory, the ordering of climate regimes, and bias-sign
   consistency** are forcing-robust — they survive that input uncertainty and so
   carry pass/fail verdicts. This distinction is the backbone of the rubric in
   Section 6.

---

## 2. The forcing-uncertainty budget

Before the process descriptions, it is worth stating *why* absolute magnitude is
not treated as the acceptance metric. openWEPP hillslope climate is assembled from
DAYMET daily precipitation and temperature, spatialized with PRISM normals, with
sub-daily storm structure assigned stochastically by the CLIGEN weather generator.
This introduces irreducible uncertainty in:

- **snowfall amount** — gauge undercatch of solid precipitation is large and
  wind-dependent;
- **magnitude at the hillslope** — gridded products represent a grid cell that is
  lapsed to a point, which is not co-located with the snow-pillow gauge;
- **sub-daily intensity** — storm duration and intensity are a stochastic
  realization from CLIGEN, not a measured time series.

Consequently, a modeled peak SWE or peak snow depth that differs from a snow-pillow
record by tens of percent is **consistent with input uncertainty, not necessarily
a model error**. The rubric therefore treats absolute-magnitude cells as
report-only. What it does hold the model to are quantities that survive forcing
uncertainty: the **shape** of the densification trajectory, the **timing** of
accumulation and melt, the **slope** of the depth–SWE relationship, the
**ordering** of climate regimes, and **mass and energy conservation**.

---

## 3. Snow — how openWEPP models it

### 3.1 Snowmelt: an energy-balance approach

Legacy production WEPP and openWEPP both compute snowmelt from a surface energy
balance — the U.S. Army Corps of Engineers (USACE) formulation documented in WEPP
Chapter 3, "Winter Hydrology" (Savabi et al., 1995) — rather than from an empirical
temperature-index (degree-day) factor. An energy-balance scheme accumulates the
heat available for melt from its physical sources. In documentation form the hourly
melt is

```
hourly melt = 0.0254 · (Q_sw − Q_lw + Q_turb + Q_rain)        [m of melt per hour]
Q_sw   = 0.0607 · R_s · (1 − cancov)                          net shortwave radiation
Q_turb = 0.0188 · U  · (1 − 0.8·cancov) · (…)                 turbulent (sensible + latent)
```

where `R_s` is incoming shortwave radiation, `cancov` is the fractional canopy
cover that attenuates radiation and wind, `U` is wind speed, `Q_lw` is a
longwave/temperature term, `Q_turb` is the turbulent sensible- and latent-heat
exchange, and `Q_rain` is the advected heat of rain falling on snow. (The constants
above are the documented form; the production code carries the same physics with
its own internally consistent coefficients and sign conventions.)

**What openWEPP modernizes — and what it deliberately did *not* adopt.** The
designed modernization is to drive the shortwave term with **gridded daily
shortwave radiation** (unavailable to the original 1976 formulation) together with
a snow-**albedo** state that evolves with temperature and snow age (*albedo* is the
fraction of incoming shortwave reflected by the surface; fresh snow is high, aging
snow lower). A candidate doing exactly this — a Brock et al. (2000) albedo bounded
to a physical range — was implemented and tested across a canopy-cover gradient,
and was **not adopted as the default**: it scored neutral-to-worse, worst under
deciduous canopy. It is reported here because an honest account of modernization
includes the options that were tested and rejected; the production default uses the
energy-balance melt without an evolving-albedo shortwave term.

### 3.2 Liquid-water retention (adopted)

openWEPP replaced legacy WEPP's density-threshold rule for releasing meltwater with
a **physical liquid-water-holding capacity**: the snowpack retains liquid water up
to a fraction of its pore space before draining, following Anderson (1976), the
SNOW-17 plausible-liquid-water-holding-capacity term, and the SNOBAL maximum-liquid
formulation (Marks et al., 1999). Pore fraction is `1 − ρ_snow/ρ_ice`
(`ρ_ice = 917 kg m⁻³`), and the maximum retained-liquid volume fraction is 0.01.
Adopting this scheme reduced paired snow-control failures from **1147 to 761** on
the coupled water-balance evaluation surface.

### 3.3 Snowpack densification (adopted)

Fresh snow is deposited at a temperature-dependent new-snow density (base
75 kg m⁻³; range 75–250 kg m⁻³; temperature threshold −15 °C) and then densifies
through the three standard mechanisms of snowpack metamorphism (Anderson, 1976;
Marks et al., 1999):

- **destructive (equi-temperature) metamorphism** — rapid early settling and grain
  rounding of new snow;
- **overburden compaction** — slow densification under the weight of overlying
  snow;
- **melt–refreeze (wet) compaction** — densification driven by liquid water in the
  pack.

The **operative density ceiling is 522 kg m⁻³**. (A higher 550 kg m⁻³ ceiling from
the SNOBAL formulation exists in the parameter set but is evaluated only as a
projection and is **not** active.) openWEPP and the pinned legacy build are
effectively the same as-built density model: mean signed density residuals differ
by only **0.4–4.4 kg m⁻³** across the five snow-pillow sites.

### 3.4 Precipitation phase: a psychrometric partition (default)

Instead of a fixed rain/snow air-temperature threshold, openWEPP determines hourly
precipitation phase with a clean-room implementation of the Harder and Pomeroy
(2013) **psychrometric energy-balance** method, which solves for the falling
hydrometeor's *ice-bulb temperature* (the equilibrium temperature of a precipitation
particle accounting for both air temperature and humidity). This better resolves
phase near 0 °C, where humidity strongly affects whether precipitation falls as rain
or snow. It is validated against the Jennings et al. (2018) observed rain–snow phase
dataset and is the **default phase model**. On the five-site forcing-robust rubric
(Section 6) it is the strongest single improvement: **15 failing cells of 179**,
compared with 17 of 172 for the previous configuration and 16 of 176 for legacy
WEPP.

### 3.5 Climate-class density (implemented; not a default)

A snow-climate-class density model after Sturm et al. (1995, 2010) is implemented —
the six-class classifier of Sturm et al. (1995) and the class-specific density
parameters of Sturm et al. (2010) for five measured classes (alpine, maritime,
prairie, tundra, taiga; the ephemeral class has no measured parameter set). It is
**implemented but not adopted as a default**, for two reasons: the available
parameter source did not expose the numeric wind, precipitation, and temperature
thresholds needed to assign a class from forcing alone, and the present five-site
network maps onto the indistinct high-density {alpine/maritime/prairie} cluster
where the classes do not separate. It is documented as available capability, not
current default behavior.

### 3.6 Multilayer snowpack and meltwater temperature (optional)

A staged multilayer effort established that **bulk-average density and bulk thermal
insulation are insensitive to vertical layering** — a layered profile did not
outperform the bulk single-layer model on either snow density or the downstream
frost response. The one multilayer product with no bulk-model equivalent is a
**per-layer meltwater temperature**, shipped as an **optional, snow-neutral**
capability (the simulated water trajectory can carry a meltwater temperature in °C;
enabling it does not change snow behavior, and it is off by default). It is intended
to seed future winter stream-temperature work.

### 3.7 How legacy snow differs

Legacy WEPP uses the empirical snow-settlement relation of WEPP Chapter 3 (with a
documented, unresolved discrepancy between the published settling equation and the
code), and its snow-**redistribution** (drifting) equations are documented but
**inactive** in the production lineage. openWEPP reproduces the same as-built
density behavior (Section 3.3) while replacing the meltwater-release rule
(Section 3.2) and the phase threshold (Section 3.4) with physically based,
observation-validated formulations.

---

## 4. Soil frost — how openWEPP models it

### 4.1 A heat-flow frost-depth model (replacing a freeze-index proxy)

openWEPP's earlier frost depth was a **freeze-index proxy** — frost depth set to
`0.20 m × clamp(−mean air temperature ÷ 6 °C, 0, 1)`, hard-capped at 0.20 m and
ratcheting (it could deepen but not retreat). It was replaced with a **physically
based heat-flow model**: a layered freeze/thaw front scheme with an explicit lower
thermal boundary and a surface temperature derived from a surface energy balance.
The improvement against the frost-observation network is large and is the central
frost result:

| Metric | Freeze-index proxy | Heat-flow model |
|---|---|---|
| Frost-depth correlation with observations | 0.13 | **0.76** |
| Frozen-duration bias | +258 days | **+61 days** |
| Depth representation | hard-capped at 0.20 m | physically bounded |

Energy conservation holds to machine precision, and the model is certified at the
conservation/activation specification boundary.

### 4.2 The layered freeze/thaw front scheme

The soil column is represented as a stack of thin layers with explicit freezing and
thawing fronts. At each step a selector chooses among downward freezing, surface
freezing within a partially thawed column, **top-down (surface) thawing**, and
**bottom thawing** driven by heat from below. Two depths are tracked, and they are
not the same quantity:

- **frost depth (`frdp`)** — the **bottom extent** of the frozen zone;
- **thaw-cap depth (`thdp`)** — the depth of a **thawed surface cap** overlying a
  still-frozen layer.

A mid-winter warm spell can thaw the soil from the top (increasing the thaw cap)
while the bottom extent of the frozen zone stays fixed — that is, a buried frozen
layer thawing from both ends. This distinction matters when interpreting frost
timing (Section 8).

### 4.3 Surface insulation: snow *and* surface residue

Frost penetration is governed by the thermal resistance of the insulating layers
above the soil, summed as depth divided by thermal conductivity for each layer:

- **snow** (using the Sturm et al. (1997) snow density–conductivity relation);
- **surface residue / litter**;
- **tilled** and **untilled** frozen soil.

Deeper snow or a thicker litter layer increases the surface thermal resistance and
insulates the soil, which delays frost onset and slows thaw.

### 4.4 Dynamic forest-litter cover

openWEPP already simulated a dynamic surface-residue **mass**, but the residue
*depth* that enters the frost surface resistance was originally a **static
initial-condition value** for every land use — it never tracked the changing mass.
That gap was closed: residue depth is now derived from the simulated surface-residue
mass through a published mass-to-depth conversion, and a **seasonal forest-litter
input** was added (autumn leaf fall into the surface-residue pool, decaying at a
forest-litter turnover rate of 0.5 yr⁻¹). Deciduous sites thus gain a seasonal
litter-insulation cycle they previously lacked. *(Current limitation: the autumn
litter-fall window is tied to the management "fall date"; relating it instead to a
physical photoperiod/frost cue is a documented future improvement.)*

### 4.5 How legacy frost differs — and its errors

Two legacy behaviors matter for interpreting frost results:

- **Frost is disabled on non-agricultural land in legacy WEPP.** A land-use switch
  defaults to "off" (no frost) for forest, grass, and shrub cover; only cropland
  engages the frost routine. openWEPP can simulate frost on non-agricultural soils
  (the frost-observation test cases enable it explicitly).
- **Legacy WEPP's water-migration ("frost-heave") heat term is inactive code.** The
  Clausius–Clapeyron migration-heat block is gated by a parameter set to zero in the
  production lineage, so that documented physics is never reached. openWEPP does not
  inherit a working migration-heat term either; adding one is a deferred item
  (Section 8).

---

## 5. The observation networks

Evaluation is against field observations processed through a comparison workflow.

**Snow — five snow-pillow (SNOTEL) sites**, each reporting paired SWE and physical
snow depth (most also soil temperature), spanning maritime to continental snow
climates: Paradise, WA (Cascades, maritime); Snowbird, UT (Wasatch,
intermountain); Central Sierra Snow Lab, CA (Sierra Nevada, maritime); Mica Creek,
ID (Northern Rockies); and Niwot, CO (Colorado Front Range, continental).

**Frost — five frost-depth sites**, using three measurement types: **frost tubes**
(which mark the frozen-water boundary and serve as the depth-magnitude reference) at
Sleepers River South Field (cropland) and W9 Hardwood (forest), VT, and at GGD498
Morris, MN (grass); a **soil-temperature 0 °C isotherm** (a timing reference, and an
upper bound on frost depth) at the SCAN Mandan, ND site; and a **modeled
soil-temperature** site at Reynolds Creek, ID (shrub). A frost tube is a
liquid-filled tube installed in the soil in which the frozen segment is read
directly; an isotherm site infers the frozen boundary from a soil-temperature
profile.

---

## 6. The evaluation rubric — and how legacy is scored against it

### 6.1 The forcing-robust rubric

Snow and frost fidelity are scored on a **multi-site rubric matrix** rather than a
single aggregate number. Each cell is tagged **R** (forcing-robust — carries a
pass/fail verdict) or **L** (forcing-limited — reported but never a standalone
failure). An abbreviated view:

| Time scale | Signature | Tier |
|---|---|---|
| Long-term | peak SWE / peak depth bias | **L** |
| Long-term | cold-season bulk-density bias | **R** |
| Long-term | snow-cover duration; interannual variability | **R** |
| Seasonal | accumulation onset date; build-up rate | **R** |
| Seasonal | peak magnitude / date of peak | **L (magnitude) / R (date)** |
| Seasonal | densification trajectory ρ(t) | **R** |
| Seasonal | depth–SWE seasonal slope | **R** |
| Seasonal | ablation: melt-out date; ablation rate | **R** |
| Event | new-snow density (per storm) | **R** |
| Event | rain-on-snow response | **R** |
| Cross-cutting | regime ordering across the five climates | **R** |
| Cross-cutting | bias-sign consistency | **R** |
| Cross-cutting | **mass/energy conservation** | **R (hard)** |

Time-series cells are scored with the **Kling–Gupta efficiency (KGE)** (Gupta et
al., 2009) — a standard goodness-of-fit measure that combines correlation, bias,
and variability into a single score (1.0 is perfect; higher is better). Magnitude
cells are scored by median signed bias and interquartile range; timing cells by
date offset. **Legacy WEPP (and the SNOBAL reference model) are scored on the exact
same rubric** — as diagnostic profiles, never as targets. The result is a
per-model, per-site, per-cell profile (a heatmap), so a model is judged on *where*
it is robustly right or wrong, not on a single number.

### 6.2 Acceptance tolerances

- Frost-depth magnitude (frost tubes): within the greater of 0.10 m or 25 % of the
  observed seasonal-maximum depth.
- Frost onset/thaw timing and frozen duration: within ±14 days.
- Snow-insulation control (a prerequisite for attributing a frost residual to the
  frost model rather than to snow): paired modeled–observed snow depth within the
  greater of 0.10 m or 30 %.
- Snow-pillow SWE within the greater of 0.05 m water-equivalent or 25 %; density
  within the greater of 60 kg m⁻³ or 25 %.
- Rubric pass levels: forcing-robust time-series cells pass at KGE ≥ 0.6 (marginal
  0.3–0.6); timing within ±14 days; forcing-limited magnitude cells are reported
  only.

*(These tolerances are provisional, pending review by an external hydrologist.)*

### 6.3 Default-activation criterion

A new process formulation becomes the **default** only if it (1) is strictly better
than the current default on the eligible observed surfaces, (2) shows no regression
across the full model surface, and (3) conserves mass and energy. It is **not**
required to drive paired failures to zero — requiring zero would over-fit to a
forcing-limited target. *(For snow, the activation evidence rests on improved snow
inputs, conservation, and reversibility at the full test-suite level; as a result,
snow-affected runoff, erosion, and watershed outputs differ from the previous
default while total water remains conserved.)*

---

## 7. Rubric results: legacy WEPP vs openWEPP

### Snow

| Measure | Legacy WEPP | openWEPP (default) |
|---|---|---|
| Forcing-robust failing cells (5 sites) | 16 / 176 | **15 / 179** (psychrometric phase) |
| Cold-season density bias (median) | **−55.6 kg m⁻³** | **≈ 0** |
| As-built density agreement (mean signed-residual spread) | — | within **0.4–4.4 kg m⁻³** of legacy |
| Coupled water-balance snow-control failures | 1147 (previous default) | **498** with the adopted improvements |

openWEPP removes the legacy cold-season density bias (−55.6 → ≈ 0 kg m⁻³) and edges
legacy WEPP on the forcing-robust cross-climate rubric, with the psychrometric phase
partition as the decisive contributor.

### Frost

| Measure | Freeze-index proxy | openWEPP heat-flow model |
|---|---|---|
| Frost-depth correlation with observations | 0.13 | **0.76** |
| Frozen-duration bias | +258 days | **+61 days** |
| Depth representation | capped at 0.20 m | physically bounded |

The heat-flow model is a step change in frost-depth fidelity over the proxy it
replaced.

---

## 8. Known limits and bounded residuals (honest scope)

openWEPP's winter physics are validated where the forcing supports it and
**bounded** elsewhere. A user interpreting outputs should keep the following in
mind.

- **Absolute snow magnitude is forcing-limited.** Peak SWE and peak depth may differ
  from a point gauge by tens of percent and still be consistent with input
  uncertainty. Treat absolute SWE and depth as approximate; rely on the **timing,
  density, and trajectory shape**, which are forcing-robust.
- **Snow-affected downstream outputs changed when the new snow defaults were
  adopted.** Runoff, erosion, and watershed outputs differ from the previous default
  because the snow inputs improved. Total water is conserved, but absolute values are
  not directly comparable across the change.
- **Frost residuals are attributed and bounded, not driven to zero.** The frost
  evaluation is **open but attributed and bounded**. The frost model was **not found
  to be in error**; the dominant residual drivers were traced to (a) input
  parameterization (static versus seasonal surface litter, now addressed) and
  (b) forcing-limited snow magnitude (over-deep modeled snow at some sites
  over-insulating the soil), rather than to the frost solver itself. A small set of
  cases remains: two snow-free sites flagged for a future wet/advective-thaw
  investigation, and a roughly 2.6 % rare "stalled-thaw" edge case in the
  freeze/thaw scheme.
- **Implemented-but-not-default capabilities.** The Sturm snow-climate-class density
  model and the evolving-albedo shortwave melt term are implemented but are not
  active defaults, because the evidence did not support adopting them. They are
  capability, not current behavior.
- **Tolerances are provisional**, pending external hydrology review.

The guiding posture throughout: report what the observations can support, hold the
model to the signatures that survive forcing uncertainty, and name the remainder as
bounded — rather than tuning to a magnitude the input forcing cannot resolve.

---

## References

Anderson, E. A. (1976). *A point energy and mass balance model of a snow cover*
(NOAA Technical Report NWS 19). National Oceanic and Atmospheric Administration,
U.S. Department of Commerce.

Brock, B. W., Willis, I. C., & Sharp, M. J. (2000). Measurement and parameterization
of albedo variations at Haut Glacier d'Arolla, Switzerland. *Journal of Glaciology,
46*(155), 675–688.

Gupta, H. V., Kling, H., Yilmaz, K. K., & Martinez, G. F. (2009). Decomposition of
the mean squared error and NSE performance criteria: Implications for improving
hydrological modelling. *Journal of Hydrology, 377*(1–2), 80–91.

Harder, P., & Pomeroy, J. (2013). Estimating precipitation phase using a
psychrometric energy balance method. *Hydrological Processes, 27*(13), 1901–1914.

Jennings, K. S., Winchell, T. S., Livneh, B., & Molotch, N. P. (2018). Spatial
variation of the rain–snow temperature threshold across the Northern Hemisphere.
*Nature Communications, 9*, 1148.

Marks, D., Domingo, J., Susong, D., Link, T., & Garen, D. (1999). A spatially
distributed energy balance snowmelt model for application in mountain basins.
*Hydrological Processes, 13*(12–13), 1935–1959.

Savabi, M. R., Young, R. A., Benoit, G. R., Witte, J. M., & Flanagan, D. C. (1995).
Winter hydrology. In D. C. Flanagan & M. A. Nearing (Eds.), *USDA–Water Erosion
Prediction Project: Hillslope profile and watershed model documentation* (NSERL
Report No. 10, Chapter 3). USDA–ARS National Soil Erosion Research Laboratory.

Sturm, M., Holmgren, J., & Liston, G. E. (1995). A seasonal snow cover
classification system for local to global applications. *Journal of Climate, 8*(5),
1261–1283.

Sturm, M., Holmgren, J., König, M., & Morris, K. (1997). The thermal conductivity of
seasonal snow. *Journal of Glaciology, 43*(143), 26–41.

Sturm, M., Taras, B., Liston, G. E., Derksen, C., Jonas, T., & Lea, J. (2010).
Estimating snow water equivalent using snow depth data and climate classes. *Journal
of Hydrometeorology, 11*(6), 1380–1394.

U.S. Department of Agriculture, Natural Resources Conservation Service. (n.d.).
*Snow Telemetry (SNOTEL) and snow course data and products* [Data set]. National
Water and Climate Center.
