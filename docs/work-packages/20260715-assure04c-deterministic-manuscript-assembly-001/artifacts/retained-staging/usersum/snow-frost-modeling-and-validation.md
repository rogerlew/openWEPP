# Snow and Frost in openWEPP

*Version 0.5 — 2026-07-14*

*Audience: hydrologists and scientific reviewers evaluating openWEPP's snowpack
and soil-frost behavior, and how it differs from legacy WEPP.*

openWEPP's winter physics were not ported line-for-line from legacy WEPP. Each
process was re-derived from its published description, implemented against a
written specification with explicit tolerances, and evaluated against field
observations — snow pillows, frost tubes, and soil-temperature records — with
legacy WEPP scored on the same rubric as a diagnostic reference rather than
matched as a target. This document describes the resulting model: what was
kept from the WEPP winter-hydrology lineage, what was replaced, what was
tested and deliberately not adopted, and what the observations can and cannot
support. Several sections report modernizations that were built, evaluated,
and rejected; these are outcomes of the method, and knowing what failed is as
useful to a reviewer as knowing what shipped.

This narrative owns the scientific rationale: why the model has its current
form, how the processes interact, and which evaluations shaped the production
defaults. The retained five-climate SNOTEL campaign, observation fixtures,
machine-readable comparisons, activation traces, and conservation checks remain
in the repository. A future manuscript-first report will synthesize that work
with full methods, quantitative tables and figures, uncertainty, limitations,
and independent review. This narrative is the durable model explanation, not a
substitute for that bounded evaluation report.

Two constraints frame everything below. Mass and energy conservation is
enforced unconditionally — composite snow-state closure residuals run at
roughly 1 × 10⁻¹⁶ m, and no fit to observations is accepted at conservation's
expense. And because the model is driven by reconstructed rather than measured
forcing, the evaluation must first decide which disagreements with
observations are even attributable to the model. That problem comes first.

---

## 1. The evaluation problem: what the forcing can support

Hillslope climate in a typical openWEPP application is assembled from DAYMET
daily precipitation and temperature, spatialized with PRISM normals, with
sub-daily storm structure generated stochastically by CLIGEN. Three
uncertainties in that chain are irreducible at the point of a snow gauge:

- solid-precipitation amount, where gauge undercatch is large and
  wind-dependent;
- magnitude at the hillslope, since a grid-cell value lapsed to a point is not
  co-located with the snow pillow it is compared against;
- sub-daily intensity, which is a stochastic realization rather than a
  measured time series.

A modeled peak SWE that differs from a snow-pillow record by tens of percent
is therefore consistent with input uncertainty and says little about the snow
model. Holding the model to that number would reward tuning against noise.

The evaluation consequently splits every scored quantity into two classes.
*Forcing-limited* quantities — absolute SWE and snow-depth magnitude — are
reported but never carry a standalone pass/fail verdict. *Forcing-robust*
quantities survive the input uncertainty and are held to tolerances: event and
seasonal timing, snow density and the shape of the densification trajectory,
the depth–SWE relationship, the ordering of behavior across climate regimes,
bias-sign consistency, and conservation. This distinction is the backbone of
the rubric in Section 4, and it recurs in the interpretation guidance of
Section 5. This classification says which discrepancies are more attributable
to the modeled process; it does not itself establish that timing or density is
accurate. Transfer to an application still requires the user to compare the
study domain and forcing with the watershed and decision at hand.

---

## 2. Snow

### 2.1 Melt

openWEPP retains the surface energy-balance melt of the WEPP lineage — the
USACE formulation documented in WEPP Chapter 3, "Winter Hydrology" (Savabi et
al., 1995) — rather than a degree-day index. Hourly melt accumulates the heat
available from its physical sources; in documentation form,

```
M_hourly = 0.0254 · (Q_sw − Q_lw + Q_turb + Q_rain)     [m h⁻¹]
Q_sw     = 0.0607 · R_s · (1 − cancov)
Q_turb   = 0.0188 · U · (1 − 0.8·cancov) · (…)
```

with `R_s` incoming shortwave, `cancov` fractional canopy cover attenuating
radiation and wind, `U` wind speed, `Q_lw` the longwave/temperature term, and
`Q_rain` the advected heat of rain on snow. The constants above are the
documented form; the production code carries the same physics with its own
internally consistent coefficients and sign conventions.

One modernization of this term was built and rejected. Gridded daily shortwave
radiation (unavailable to the original 1976 formulation) paired with an
evolving snow-albedo state — a Brock et al. (2000) parameterization bounded to
a physical range — was implemented and evaluated across a canopy-cover
gradient. It scored neutral to worse than the fixed formulation, worst under
deciduous canopy, and was not adopted. The production default remains the
energy-balance melt without an evolving-albedo shortwave term.

### 2.2 Liquid-water retention

Legacy WEPP releases meltwater by a density-threshold rule. openWEPP replaces
this with a physical liquid-water-holding capacity: the pack retains liquid up
to a fraction of its pore space before draining, following Anderson (1976),
the SNOW-17 holding-capacity term, and the SNOBAL maximum-liquid formulation
(Marks et al., 1999). Pore fraction is `1 − ρ_snow/ρ_ice` with
`ρ_ice = 917 kg m⁻³`, and the maximum retained-liquid volume fraction is 0.01.
On the coupled water-balance comparison surface — the paired daily checks
where snow is the controlling input — adopting this scheme alone cut failing
checks from 1147 to 761; the full set of adopted snow changes brings the count
to 498 (Section 4.3).

### 2.3 Densification

Fresh snow deposits at a temperature-dependent new-snow density (base
75 kg m⁻³, range 75–250 kg m⁻³, threshold temperature −15 °C) and densifies
through the three standard mechanisms of snowpack metamorphism (Anderson,
1976; Marks et al., 1999): destructive (equi-temperature) metamorphism of new
snow, overburden compaction, and melt–refreeze compaction. The operative
density ceiling is 522 kg m⁻³; a 550 kg m⁻³ ceiling from the SNOBAL
formulation exists in the parameter set but is evaluated only as a projection
and is not active.

Densification is the one winter process where openWEPP and the pinned legacy
build were found to behave similarly in the retained five-site diagnostic.
The retained site fixtures and comparison artifacts preserve the evidence
behind that finding for the future manuscript synthesis.

A snow-climate-class density model after Sturm et al. (1995, 2010) — the
six-class classifier with class-specific density parameters for the five
measured classes (alpine, maritime, prairie, tundra, taiga; the ephemeral
class has no published parameter set) — is implemented but not a default, for
two reasons. The available source does not expose the numeric wind,
precipitation, and temperature thresholds needed to assign a class from
forcing alone, and the present five-site network falls entirely within the
high-density alpine/maritime/prairie cluster, where the classes do not
separate and the model cannot be discriminated from the default. It remains
available capability.

### 2.4 Precipitation phase

In place of a fixed rain/snow air-temperature threshold, openWEPP partitions
hourly precipitation phase with a clean-room implementation of the Harder and
Pomeroy (2013) psychrometric method, solving for the falling hydrometeor's
ice-bulb temperature so that humidity influences phase near 0 °C, where a dry
atmosphere favors snow at air temperatures a fixed threshold would call rain.
The implementation was checked against the Jennings et al. (2018) Northern
Hemisphere rain–snow phase dataset and is the default phase model. The retained
five-site diagnostic favored it under a retrospective rubric. That is useful
evidence for the bounded mechanism-selection decision; because model variants
and thresholds were examined retrospectively, it is not presented as an
independent held-out estimate of predictive accuracy.

### 2.5 Multilayer snowpack

A staged multilayer implementation established a negative result worth
recording: bulk-average density and bulk thermal insulation are insensitive
to vertical layering, and a layered profile did not outperform the
single-layer model on snow density or on the downstream frost response. The
one multilayer product with no bulk equivalent — a per-layer meltwater
temperature — ships as an optional capability that leaves snow behavior
unchanged when enabled and is off by default. Its intended use is seeding
future winter stream-temperature work.

### 2.6 Differences from legacy snow

Legacy WEPP uses the empirical snow-settlement relation of WEPP Chapter 3,
which carries a documented, unresolved discrepancy between the published
settling equation and the code; its snow-redistribution (drifting) equations
are documented but inactive in the production lineage. openWEPP reproduces
similar as-built densification behavior while replacing the meltwater-release
rule and the phase threshold with the evaluated formulations above.

---

## 3. Soil frost

### 3.1 From a freeze-index proxy to a heat-flow model

openWEPP's original frost depth was a placeholder: frost depth set to
`0.20 m × clamp(−T̄_air / 6 °C, 0, 1)`, hard-capped at 0.20 m, and ratcheting
— it could deepen but never retreat. Against the frost-observation network it
correlated with measured frost depth at 0.13 and overpredicted frozen
duration by +258 days.

It was replaced by a heat-flow model: a layered freeze/thaw front scheme with
an explicit lower thermal boundary and a surface temperature derived from a
surface energy balance. The replacement raised frost-depth correlation to
0.76 and cut the frozen-duration bias to +61 days (Section 4.3), while
conserving energy to machine precision. This is the central frost result; the
remaining residuals, and where they were traced, are in Section 5.

### 3.2 The freeze/thaw front scheme

The soil column is a stack of thin layers with explicit freezing and thawing
fronts. Each step, a selector chooses among downward freezing, surface
freezing within a partially thawed column, top-down surface thaw, and bottom
thaw driven by heat from below. Two depths are tracked, and they are not the
same quantity: frost depth (`frdp`) is the bottom extent of the frozen zone,
while thaw-cap depth (`thdp`) is the depth of a thawed surface cap overlying
still-frozen soil. A mid-winter warm spell can deepen the thaw cap while the
bottom extent holds — a buried frozen layer thawing from both ends. The
distinction matters when comparing modeled and observed frost timing, since a
frost tube and the model can disagree about "thawed" while agreeing about the
frozen mass between the fronts.

### 3.3 Surface insulation

Frost penetration is governed by the summed thermal resistance (depth over
conductivity) of the layers above the mineral soil: snow, using the Sturm et
al. (1997) density–conductivity relation; surface residue and litter; and
tilled or untilled frozen soil. Deeper snow or thicker litter delays frost
onset and slows thaw — which is also why snow-magnitude error propagates into
frost error, a coupling that shapes the residual attribution in Section 5.

### 3.4 Dynamic forest-litter cover

openWEPP has always simulated a dynamic surface-residue mass, but the residue
*depth* entering the frost resistance was originally a static
initial-condition value for every land use — insulation never tracked the
mass it supposedly measured. Residue depth is now derived from the simulated
residue mass through a published mass-to-depth conversion, and a seasonal
forest-litter input was added: autumn leaf fall into the surface-residue
pool, decaying at a forest-litter turnover rate of 0.5 yr⁻¹. Deciduous sites
thereby gain the seasonal litter-insulation cycle they previously lacked. One
limitation stands: the autumn litter-fall window is currently tied to the
management file's fall date; re-anchoring it to a photoperiod or frost cue is
a documented follow-on.

### 3.5 Differences from legacy frost

Two legacy behaviors matter when interpreting any frost comparison. First,
legacy WEPP disables frost on non-agricultural land: a land-use switch
defaults to off for forest, grass, and shrub cover, so only cropland engages
the frost routine at all. openWEPP simulates frost on non-agricultural soils,
and the frost-observation test cases enable it explicitly. Second, legacy
WEPP's water-migration ("frost heave") heat term is dead code — the
Clausius–Clapeyron migration-heat block is gated by a parameter set to zero
in the production lineage, so the documented physics is never executed.
openWEPP does not currently implement a migration-heat term either; adding
one with proper literature authority is a deferred item (Section 5).

---

## 4. Evaluation

### 4.1 Observation networks

Snow was evaluated at five snow-pillow (SNOTEL) sites reporting paired SWE and
physical snow depth (most also soil temperature), chosen to span maritime to
continental snow climates: Paradise, WA (Cascades, maritime); Snowbird, UT
(Wasatch, intermountain); Central Sierra Snow Lab, CA (Sierra Nevada,
maritime); Mica Creek, ID (Northern Rockies); and Niwot, CO (Front Range,
continental). The retained manifest covers 70,999 daily station rows and
13,590 paired SWE-depth rows from records beginning between water years 2002
and 2006 and extending through 2024. The fixtures use DAYMET, GRIDMET, CLIGEN,
or PRISM forcing according to site; that mixed forcing is part of both the
study's breadth and its attribution limit.

Frost is evaluated at five sites using three measurement types. Frost tubes —
liquid-filled tubes read directly for the frozen segment, serving as the
depth-magnitude reference — at Sleepers River South Field (cropland) and W9
Hardwood (forest) in Vermont, and at GGD498 Morris, MN (grass). A
soil-temperature 0 °C isotherm at the SCAN Mandan, ND site provides a timing
reference and an upper bound on depth. Reynolds Creek, ID (shrub) contributes
a modeled soil-temperature record.

### 4.2 Evaluation design

The retained snow study used a multi-site matrix rather than one aggregate
score. It separated forcing-robust signatures from forcing-limited absolute
magnitude, compared historical variants on the same cells, treated legacy
WEPP and PySnobal as diagnostic flags rather than targets, and required phase
partition conservation for activation.

Those choices were retrospective. They were useful for selecting among model
variants, but their thresholds, partitions, and incomplete portable provenance
cannot be recast as a prospectively specified held-out accuracy experiment.
The normalized observations, site manifest, comparison procedure, machine-
readable diagnostic, and activation procedure remain content-identified in the
repository for reconstruction and future independent review.

### 4.3 Results

For snow, the retained comparison favored the current Harder-Pomeroy phase
treatment under the study's retrospective ordinal rubric. The activation trace
then verified that all 159,986 selected-model rows used the intended default;
an explicit legacy selector remained available in the same number of rollback
trace rows. Across 53,711 precipitation rows, rain plus snow reconstructed
active precipitation with a maximum absolute residual of
5.55 × 10⁻¹⁷ m. Together these results support the bounded default-selection,
implementation, and conservation conclusions.

They do not by themselves estimate predictive accuracy for arbitrary sites.
The observation processing has normalized-file identities, but some original
acquisition paths are not yet portable; forcing uncertainty was not propagated
into claim-level intervals; model variants and thresholds were examined
retrospectively; and the study has not received external hydrologist review.
The future manuscript will present the quantitative site results and these
limitations together rather than reducing them to an aggregate grade.

Frost, against the observation network:

| Measure | Freeze-index proxy | Heat-flow model |
|---|---|---|
| Frost-depth correlation | 0.13 | 0.76 |
| Frozen-duration bias | +258 days | +61 days |
| Depth representation | hard-capped at 0.20 m | physically bounded |

The heat-flow model is a step change over the proxy it replaced. The
remaining +61-day duration bias and the residual depth misses are attributed
in Section 5 — notably, not to the frost solver.

---

## 5. Interpreting Winter Output And Evidence Limits

For a user reading openWEPP winter output, the practical guidance follows
directly from the forcing analysis of Section 1 and the results above.

**Timing, density, and trajectory shape are more attributable than absolute
magnitude, but they are not application guarantees.** Melt-out dates,
accumulation onset, densification trajectories, and depth–SWE slopes are
forcing-robust quantities in the retained method. Peak SWE or depth at a
specific point may differ substantially because the forcing cannot localize
magnitude to a point. A user must not translate that relative attribution
advantage into an unnamed site's accuracy claim.

**Snow-affected downstream outputs changed when the new defaults were
adopted.** Runoff, erosion, and watershed outputs can differ because the snow
state and timing supplied to downstream processes changed. The retained
activation evidence establishes phase-partition conservation, not improved
downstream accuracy. Absolute values are not directly comparable across the
default change.

**The frost residuals are attributed but remain incomplete evidence.** The residual
frost misses were traced, and the frost solver was not found to be in error.
The dominant drivers were input parameterization — the static residue depth
of Section 3.4, since corrected — and forcing-limited snow magnitude, where
over-deep modeled snow over-insulates the soil at some sites (the coupling of
Section 3.3). What remains open is small and named: two snow-free sites
flagged for a future wet/advective-thaw investigation (related to the
migration-heat gap of Section 3.5), and a rare stalled-thaw edge case in the
front scheme affecting roughly 2.6 % of thaw days.

**Implemented is not the same as active.** The Sturm climate-class density
model, the evolving-albedo melt term, and the multilayer pack are implemented
and were evaluated; none is a default, because the evidence did not support
adoption. The per-layer meltwater temperature is available as an opt-in and
does not alter snow behavior.

The snow method was retrospective, and external hydrologist review remains
outstanding. Compare its five-site, mixed-forcing evidence envelope with the
climate, forcing, canopy, topography, scale, quantity, accuracy need, and
consequence of error for a named use. The application decision belongs to the
responsible user or institution.

The posture throughout is the one stated at the top: hold the model to the
signatures the observations can actually resolve, report the rest, and name
what remains open — rather than tuning to magnitudes the forcing cannot
support.

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

## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 0.1 | 2026-06-29 | Initial document, authored at the close of the frost validation arc. |
| 0.2 | 2026-07-01 | Rewritten as a scientific narrative: framing problem first, results consolidated, internal vocabulary translated. Same claims, numbers, and references. |
| 0.3 | 2026-07-09 | Adopted the version header and revision log convention; no content changes. |
| 0.4 | 2026-07-14 | Reframed as the model-rationale narrative, moved the five-site method and evidence characterization to linked assurance pages, and removed unqualified validation and duplicated snow scores. |
| 0.5 | 2026-07-14 | Retired the failed v1 status-first pages, restored the retained SNOTEL design and positive selector/conservation findings to ordinary scientific prose, and preserved limitations without an aggregate grade. |
