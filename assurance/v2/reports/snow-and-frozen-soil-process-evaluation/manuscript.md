# Observational Evaluation of openWEPP Snow and Frozen-Soil Processes

*Version 1.0 — 2026-08-05*

Prepared with disclosed Codex assistance for openWEPP scientific-assurance
maintainers. The findings below are a bounded synthesis of identified evidence.

{{assurance:attribution}}

{{assurance:lifecycle}}

## Key Findings

- Across {{quantity:SF-V-PHASE-ROWS}} of observed hourly precipitation-phase
  records from {{quantity:SF-V-PHASE-STATIONS}}, the humidity-aware
  Harder-Pomeroy formulation classified {{quantity:SF-V-HP-ACCURACY}} correctly,
  compared with {{quantity:SF-V-LEGACY-ACCURACY}} for the fixed 0 degrees
  Celsius threshold: a difference of {{quantity:SF-V-ACCURACY-DIFFERENCE}}.
  The observations were used in model selection, so this is substantial
  retrospective corroboration rather than untouched post-selection validation.
- Daily snow comparisons at five mountain SNOTEL and five forest-canopy
  surfaces reproduced many timing and depth-SWE response signatures while
  exposing coherent residuals: densification trajectory at
  {{quantity:SF-V-SNOW-DENSITY-FAILS}}, mountain timing or persistence at
  {{quantity:SF-V-SNOW-TIMING-FAILS}}, and depth-SWE geometry at
  {{quantity:SF-V-SNOW-GEOMETRY-FAILS}}. The site-resolved results, not a pooled
  rubric percentage, are the primary interpretation.
- Frozen-soil evidence is heterogeneous and adverse results are retained.
  Frost-tube residuals reached {{quantity:SF-V-FROST-MAX-RESIDUAL}}. The
  isotherm-bound exceedance rate was {{quantity:SF-V-FROST-MANDAN-EXCEEDANCE-PERCENT}}
  at Mandan and {{quantity:SF-V-FROST-REYNOLDS-EXCEEDANCE-PERCENT}} at Reynolds
  Creek. Failed or unavailable paired snow-depth control prevents attribution
  of these coupled residuals solely to frozen-soil physics.
- Production precipitation partition closed across
  {{quantity:SF-V-PARTITION-PRECIP-ROWS}} with active precipitation. Separately,
  four selected production WAT rows—two snow and two frozen-soil rows—were
  reconstructed from their storage and transfer operands and closed at
  floating-point residuals. These are named spot verifications, not a general
  predictive or all-row conservation claim.
- The 2026-08-05 authority refresh separates current implementation from
  target authority: CoE remains the byte-identical compatibility melt runtime,
  while Stage 3 is admitted only as the future sole melt owner under an
  implementation hold. This authority decision is not empirical efficacy,
  noninferiority, runtime cutover, or a change to the retained observational
  results.

## Plain-Language Summary

Snow affects runoff, erosion, soil freezing, and the timing of water delivery.
openWEPP therefore has to answer several linked questions correctly: whether
precipitation falls as rain or snow, how the snowpack stores and releases water,
how depth and density change, and how the insulating pack alters soil freezing.
We assembled the project's observational and production evidence for that
complete chain instead of reporting a single validation grade.

The precipitation-phase result is substantial. The scored hourly observations
show that a physical, humidity-aware formulation classifies
rain and snow more accurately than a fixed freezing-point threshold and
reproduces the observed direction of the humidity effect on the rain-snow
transition. Production traces also show exact selection of that formulation and
near-machine-precision rain-plus-snow closure.

The seasonal snowpack evidence is more informative than a single score.
Site-resolved comparisons show that remaining discrepancies cluster in snow
densification,
early peak or meltout timing at mountain sites, and depth relative to SWE at two
humid New England surfaces. Absolute SWE and depth often inherit large errors
from precipitation and temperature forcing, so those magnitudes cannot be
assigned solely to snow physics.

Later campaign evidence corrected a duplicate wet-compaction input and changed
density and depth without changing generated melt or upstream snow mass. Its
annual-first site medians localized nearly all pre-peak loss to warm or mixed
days, but the
temperature, moisture, radiation, density, and pack-depth signals occur
together and do not identify one cause. The current CoE equations reproduce the
pinned post-2007 baseline, yet the material post-handbook changes lack an
independently validated production envelope. Consequently, CoE remains the
current compatibility runtime while Stage 3 is only the admitted future owner;
the required atomic implementation and cutover have not occurred.

The frozen-soil observations show why the chain must be evaluated together.
Where frost-tube depth was available, the simulated frost response could be
compared directly, but modeled snow depth frequently failed its paired control.
At two soil-temperature sites, observed snow depth was unavailable. Snow is the
dominant surface insulation boundary, so these comparisons cannot tell us how
much residual error belongs to frozen-soil physics alone. They remain valuable
coupled-system evidence and identify exactly what a future independent campaign
must control.

## Abstract

Snow accumulation and soil freezing couple atmospheric phase, snow mass and
energy storage, surface insulation, and subsurface heat and water transfer. We
synthesized retained observational and production evidence for openWEPP's
precipitation-phase, seasonal snowpack, and frozen-soil process chain. The
retrospective design comprised: {{quantity:SF-V-PHASE-ROWS}} of hourly
precipitation-phase observations across {{quantity:SF-V-PHASE-STATIONS}};
{{quantity:SF-V-SNOTEL-PAIRS}} of paired daily SWE-depth observations at five
SNOTEL sites and {{quantity:SF-V-CANOPY-PAIRS}} of paired observations at five
canopy surfaces; frost-tube observations at
{{quantity:SF-V-FROST-SITES}}, with {{quantity:SF-V-FROST-MATCHES}} of matched
depth comparisons; soil-temperature profiles from
{{quantity:SF-V-ISOTHERM-SITES}}, with {{quantity:SF-V-ISOTHERM-ROWS}} of
evaluated zero-degree-isotherm bounds; and
independent reconstruction of production conservation ledgers. The
Harder-Pomeroy hourly phase formulation achieved
{{quantity:SF-V-HP-ACCURACY}} classification accuracy versus
{{quantity:SF-V-LEGACY-ACCURACY}} for the fixed 0 degrees Celsius threshold and
captured the observed direction of humidity dependence. Rubric-designated
forcing-robust seasonal-snow diagnostics identified failures concentrated in
densification, under-persistent mountain timing, and depth-SWE geometry; these
correlated site-by-signature cells are not independent trials or a portable
success rate. Frost-tube residuals
reached {{quantity:SF-V-FROST-MAX-RESIDUAL}}, but paired snow-depth control
failed on {{quantity:SF-V-FROST-SNOW-FAIL-PERCENT}} of evaluated dates; the
soil-temperature sites lacked paired snow-depth observations. Production
partition and four selected-row water-storage spot verifications closed within
floating-point residuals.
The evidence supports the physical and numerical credibility of the coupled
snow/frost implementation and identifies specific residual mechanisms. Because
the observational corpora influenced model development and frozen-soil
comparisons remain snow-confounded, the synthesis does not constitute an
independent predictive validation or authorize fitness for an untested site.
The 2026-08-05 authority refresh does not change those retained empirical
results: it records that current CoE melt remains a compatibility
implementation and that Stage 3 future ownership is not yet implemented or
empirically validated.

## 1. Introduction

In erosion and watershed simulation, snow is not merely delayed rainfall. The
phase of precipitation determines whether water is immediately available for
infiltration and runoff or enters seasonal storage. Snow water equivalent (SWE)
controls stored mass; physical depth and density influence surface heat
exchange; liquid retention and ablation determine release timing; and snow
insulation modifies soil freezing and thawing. Errors propagate downstream to
runoff timing, soil-water availability, and erodibility.

Traditional WEPP snow calculations combined a temperature threshold with a
bulk accumulation and melt treatment. openWEPP retains the established process
lineage but has added a humidity-aware hydrometeor-temperature phase calculation,
explicit liquid holding and release, physical bulk-density evolution, typed
snow-state carry, and a stateful frozen-soil column. The development program
used external observations, conservation constraints, literature, negative
mechanism tests, and production-path checks to decide which changes warranted
activation.

The resulting evidence is unusually broad but was created across many bounded
engineering studies. This synthesis asks what that evidence establishes when
read as a scientific whole. It deliberately keeps four questions separate:
phase classification, seasonal snowpack response, frozen-soil response, and
software/conservation verification. The aim is not to declare the subsystem
“validated,” but to show a domain reader the strongest evidence, the residual
structure, the experimental dependencies, and the limits on inference.

## 2. Model Formulation and Conceptual Basis

### 2.1 Precipitation phase

The active hourly phase method follows Harder and Pomeroy's hydrometeor-
temperature approach. Air temperature, humidity, pressure-dependent vapor
properties, and heat/mass transfer determine the equilibrium temperature of a
falling hydrometeor. A logistic relation then maps hydrometeor temperature to
rain and snow fractions. This formulation represents the physical observation
that humidity changes the air temperature at which solid precipitation melts;
it replaces a universal 0 degrees Celsius threshold as the direct-production
default while retaining that threshold as an explicit rollback and diagnostic
comparison.

### 2.2 Snow accumulation, liquid release, and density

Snowfall adds SWE to a bulk pack. Melt and rain can be retained only to a
bounded liquid-water holding capacity; excess liquid is routed out of the pack.
Physical depth is derived from SWE and evolving bulk density, with fresh-snow,
dry-compaction, and wet-compaction behavior drawn from the Anderson/SNOBAL
lineage. SWE remains the mass authority. Density and depth may change without
inventing or removing water, and the direct runtime keeps the snowpack boundary
used by melt distinct from diagnostic or alternative-model state.

The active wet-compaction driver is now explicitly positive hourly generated
melt plus interval-start snow-contact rain, counted once before runoff. The
retired driver double counted bounded pack loss through an adjacent routed-
liquid alias. This correction materially changes density and depth trajectories
but not generated melt, upstream mass, phase, forcing, canopy, or the active
default selectors.

The current melt owner remains the post-2007 coefficient-of-efficiency (CoE)
generator for compatibility. Canonical
{{link:research-object:SF-OBJECT-ENERGY-CONTRACT|snow-energy v7}} and
snow/frost v126 authority admit the resolved
Stage 3 surface-energy and phase-change system as the future sole melt owner,
but only after one atomic implementation closes complete sensible and
precipitation-advection heat, thin-pack residual-snow phase disposition, a
canonical physical recipient and next-state disposition for terminal remaining
energy without proxy transfer, same-substep liquid handling, linked
ledgers, selectors, rollback, and the real downstream consumer. Until that
implementation passes, Stage 3 cannot generate production melt and CoE cannot
be partially retired.

The bulk representation is intentionally simpler than a multilayer energy-
balance snow model. That simplicity is useful for hillslope-scale erosion
simulation, but it limits representation of vertical temperature gradients,
wind redistribution, canopy interception and unloading, and layer-specific
metamorphism.

### 2.3 Frozen soil

The frozen-soil calculation represents vertical heat flow, latent heat,
freezing and thawing fronts, liquid and frozen water, and the insulating effects
of residue and snow. Physical snow depth is the relevant insulation boundary;
SWE cannot substitute for it. The model publishes frost depth and soil-water
state, but the observational referent matters: frost tubes approximate a
frozen-water boundary, whereas the 0 degrees Celsius soil-temperature isotherm
is an upper-bound/timing referent and need not coincide with the ice front. Dun
et al. (2010) provide the primary published WEPP formulation and evaluation
lineage used here; the frost-tube data publications are separate observational
authorities.

## 3. Data and Methods

### 3.1 Study design and evidence role

This is a retrospective synthesis, not a new held-out experiment. The Jennings
phase observations informed phase-method selection. SNOTEL and canopy-site
profiles informed mechanism development, rejection, and activation. The
frozen-soil observations were used to classify residuals and govern allowable
physics work. Results therefore describe the breadth and consistency of the
evidence supporting the current formulation, while selection bias is carried as
a central limitation.

Each retained source was bound by SHA-256. A standard-library reconstruction
procedure recomputed phase accuracy from integer confusion matrices, summed
site and rubric counts, checked residual-family closure, aggregated method-
specific frozen-soil counts, authenticated the production conservation source,
and emitted the strict result used by every table, figure, and quantitative
statement in this report.

### 3.2 Precipitation-phase observations

The Jennings et al. Northern Hemisphere corpus contains hourly rain/snow phase
labels with colocated air temperature, dew point, relative humidity, and
pressure. Of {{quantity:SF-V-PHASE-READ}} read,
{{quantity:SF-V-PHASE-ROWS}} met the retained scoring requirements across
{{quantity:SF-V-PHASE-STATIONS}}. We evaluated the active
Harder-Pomeroy hourly method and the legacy fixed 0 degrees Celsius threshold
against identical rows.

The scorer excluded {{quantity:SF-V-PHASE-ROWS-SKIPPED}}
({{quantity:SF-V-PHASE-ROWS-SKIPPED-PERCENT}}) before comparison. Eligible rows
had all required numeric fields, a station in the supplied threshold table, a
finite valid temperature and humidity, a successful Harder-Pomeroy evaluation,
and an observed label that was exclusively rain or exclusively snow. Mixed-
phase and neither-phase labels were not scored. The retained aggregate does not
separate the skipped rows by exclusion reason, so no reason-specific exclusion
rate is inferred.

Primary operands were the four confusion-matrix cells. Accuracy was
reconstructed as correctly classified rain plus correctly classified snow
divided by all scored rows. Harder-Pomeroy classified an event as rain when its
predicted rain fraction was at least one-half; the fixed baseline classified air
temperatures above freezing as rain. For each station, the modeled transition
temperature was the scored event whose predicted rain fraction was nearest
one-half; the observed transition came from the Jennings station-threshold file.
Mean station humidity defined the lowest and highest deciles, each containing
{{quantity:SF-V-HUMIDITY-GROUP-STATION-COUNT}}. Bias, mean absolute error, and
the high-minus-low humidity contrast test physical pattern and magnitude
separately from event classification. The exact scoring implementation is
included as the
{{link:research-object:SF-OBJECT-JENNINGS-HARNESS|phase-scoring research object}}.

### 3.3 Seasonal snow observations

Five NRCS SNOTEL stations span northern Rockies, Cascades maritime, Sierra
maritime, Wasatch intermountain, and Front Range continental snow climates. The
paired sonic-depth era supplied {{quantity:SF-V-SNOTEL-PAIRS}} of daily paired
SWE-depth observations. Five additional open, deciduous, and coniferous
surfaces at Harvard and Marcell supplied {{quantity:SF-V-CANOPY-PAIRS}} of
paired observations and extended the vegetation/climate range.

The comparison used a signature-based, multi-timescale profile rather than one
goodness-of-fit score. Each cell was designated forcing-robust or forcing-
limited before interpretation. Forcing-robust cells included seasonal timing,
densification, depth-SWE geometry, and other response signatures. Absolute SWE
and depth magnitudes were reported but did not carry mechanism verdicts because
the fixtures mix DAYMET, GRIDMET, CLIGEN, and PRISM inputs and point-station
support with modeled hillslope support.

The ordinal bands—fail, marginal, pass, and strong—encode prespecified
signature-specific noise floors. They are useful for locating residuals across
regimes; they are not probabilities, regulatory grades, or comparable to an
NSE value. Cells from the same site and signature family are correlated and are
not independent observations. We therefore lead with the site-resolved profile,
paired counts, physical diagnostics, and fail-cell families; the pooled ordinal
distribution is a secondary development diagnostic.

### 3.4 Frozen-soil observations

The frozen-soil corpus contains three frost-tube sites at Sleepers River,
Vermont, and Morris, Minnesota, plus soil-temperature profiles at Mandan, North
Dakota, and Reynolds Creek, Idaho. Frost-tube comparisons used magnitude
residuals at matched dates. Temperature-profile comparisons tested whether the
simulated frost front remained bounded by the observed 0 degrees Celsius
isotherm; they were not treated as equivalent frost-depth measurements.
Sleepers River measurements are identified by the USGS data release; Morris
measurements are identified by the NSIDC GGD498 release. Full persistent
identifiers are given in the references.

At the frost-tube sites, paired observed snow depth supplied an explicit snow-
control test. At the two temperature sites, paired observed snow depth was not
available. This distinction was fixed before frost residual interpretation.

### 3.5 Production verification and conservation

Production traces checked that all {{quantity:SF-V-PARTITION-TRACE-ROWS}}
selected the declared phase, melt, and density models and that rain plus snow
reconstructed active precipitation on
{{quantity:SF-V-PARTITION-PRECIP-ROWS}}. Separately, four selected rows from
independently read production WAT outputs were retained for spot verification:
snow accumulation and release rows, and a dry freeze-growth and material-thaw
row. Their precipitation, routed melt, liquid water, frozen water, external
sink, and prior/current storage operands were retained, content identified, and
reconstructed mechanically. Physical snow depth, frost depth, and diagnostic
state were explicitly rejected as water-storage operands. These rows test exact
identities at named consumers; they do not establish all-row or all-realization
closure.

## 4. Results

### 4.1 Precipitation phase

{{table:SF-TABLE-PHASE}}

{{figure:SF-FIGURE-PHASE}}

The Harder-Pomeroy method correctly classified
{{quantity:SF-V-HP-ACCURACY}} of scored observations, compared with
{{quantity:SF-V-LEGACY-ACCURACY}} for the fixed threshold. Most of the gain
came from reducing observed-snow events classified as rain: the count fell from
{{quantity:SF-V-LEGACY-SNOW-AS-RAIN}} under the fixed threshold to
{{quantity:SF-V-HP-SNOW-AS-RAIN}} under Harder-Pomeroy, while
snow was assigned to {{quantity:SF-V-HP-RAIN-AS-SNOW}} of observed-rain events.
The fixed-threshold confusion operands were
{{quantity:SF-V-LEGACY-RAIN-AS-RAIN}} rain-as-rain,
{{quantity:SF-V-LEGACY-RAIN-AS-SNOW}} rain-as-snow,
{{quantity:SF-V-LEGACY-SNOW-AS-RAIN}} snow-as-rain, and
{{quantity:SF-V-LEGACY-SNOW-AS-SNOW}} snow-as-snow, retaining the full numerator
and denominator behind its reported accuracy.

The modeled station threshold had a mean bias of
{{quantity:SF-V-PHASE-THRESHOLD-BIAS}} and mean absolute error of
{{quantity:SF-V-PHASE-THRESHOLD-MAE}}. More importantly for mechanism
evaluation, the observed high-minus-low-humidity threshold contrast was
{{quantity:SF-V-OBS-HUMIDITY-CONTRAST}}; the model predicted a contrast of
{{quantity:SF-V-PRED-HUMIDITY-CONTRAST}} in the same direction. The
formulation therefore captured the principal humidity dependence, although its
mean station threshold remained warm-biased.

### 4.2 Seasonal snowpack response

{{table:SF-TABLE-SNOW-SITES}}

{{table:SF-TABLE-SNOW}}

The site-resolved table is the primary result. Negative timing offsets identify
early modeled dates: median meltout was
{{quantity:SF-V-SNOW-MICA-MELTOUT-OFFSET-DAYS}} at Mica Creek and
{{quantity:SF-V-SNOW-PARADISE-MELTOUT-OFFSET-DAYS}} at Paradise, while median
peak SWE was {{quantity:SF-V-SNOW-SNOWBIRD-PEAK-SWE-OFFSET-DAYS}} at Snowbird
and {{quantity:SF-V-SNOW-NIWOT-PEAK-SWE-OFFSET-DAYS}} at Niwot. Harvard open
and hardwood depth-SWE slope ratios reached
{{quantity:SF-V-SNOW-HARVARD-OPEN-GEOMETRY-RATIO}} and
{{quantity:SF-V-SNOW-HARVARD-HARDWOOD-GEOMETRY-RATIO}}, respectively. Seasonal
densification KGE varied by site and was negative at most surfaces. These are
physical and timing diagnostics, not interchangeable replicates.

Secondarily, across the {{quantity:SF-V-SNOW-AVAILABLE}} correlated
site-by-signature cells, {{quantity:SF-V-SNOW-STRONG}} were strong,
{{quantity:SF-V-SNOW-PASS}} passed, {{quantity:SF-V-SNOW-MARGINAL}} were
marginal, and {{quantity:SF-V-SNOW-FAIL}} failed. The corresponding
{{quantity:SF-V-SNOW-PASS-STRONG-PERCENT}} is a rubric summary used during
development, not an empirical success probability or portable performance
estimate.

The fail cells were structured rather than random.
{{quantity:SF-V-SNOW-DENSITY-FAILS}} described seasonal densification
trajectory and were distributed across SNOTEL and canopy sites.
{{quantity:SF-V-SNOW-TIMING-FAILS}} described under-persistence: early peak SWE
or early meltout at mountain stations, with substantially early retained
median dates. {{quantity:SF-V-SNOW-GEOMETRY-FAILS}} described excessive depth
relative to SWE at the Harvard open and hardwood surfaces. Absolute paired
values also showed modeled SWE below observations at
all ten surfaces, but that direction cannot be assigned solely to snow physics
because forcing and spatial support affect the magnitude directly.

The retained model-development sequence tested and rejected several plausible
alternatives when they worsened the cross-site profile, including some
sublimation, shallow-pack, spring-densification, and climate-class-density
variants. These negative results increase confidence that the activated process
combination was not selected from one favorable site, while the reuse of the
same observation network still prevents an independent validation claim.

The later
{{link:research-object:SF-OBJECT-21K|21K wet-compaction correction}},
{{link:research-object:SF-OBJECT-21L|21L warm/mixed attribution}},
{{link:research-object:SF-OBJECT-21M|21M CoE authority audit}}, and
{{link:research-object:SF-OBJECT-21N|21N ownership reconciliation}} change how
these residuals are interpreted. The wet-compaction correction removed a
density/geometry confounder, so pre-21K
density, depth, and loss baselines cannot carry causal attribution forward.
Annual-first site medians place almost all pre-peak loss on warm or mixed days
and identify CoE's empirical `cmelt` term as the largest annual-first positive
term at all four canonical mountain sites. These observations are
chronology-confounded: warm,
moist, more radiative, denser, and shallower states co-occur. They do not prove
that `cmelt`, radiation, forcing, density, or another individual process caused
the loss, and they supply no correction or calibration authority.

The CoE implementation audit found exact post-2007 baseline fidelity rather
than a Rust transcription defect. Because the material 2007/2008 changes lack
cited independent validation or bounded transferability authority, v7/v126
admit Stage 3 as the future sole melt owner. This is a target-authority decision,
not evidence that Stage 3 currently controls SWE or runoff. CoE remains the
unchanged default compatibility owner on `IMPLEMENTATION_HOLD`; Stage 3
cutover, efficacy, noninferiority, and warm-maritime conifer transfer remain
unproven.

### 4.3 Frozen-soil response

{{table:SF-TABLE-FROST}}

{{table:SF-TABLE-FROST-TUBE-SITES}}

{{table:SF-TABLE-ISOTHERM-SITES}}

Frost tubes were evaluated at {{quantity:SF-V-FROST-SITES}}. The matched
dataset contained {{quantity:SF-V-FROST-MATCHES}}; the largest site-level
absolute residual was
{{quantity:SF-V-FROST-MAX-RESIDUAL}}. At those same sites, snow-depth control
failed in {{quantity:SF-V-FROST-SNOW-FAILURES}} among
{{quantity:SF-V-FROST-SNOW-ROWS}} of paired dates
({{quantity:SF-V-FROST-SNOW-FAIL-PERCENT}}). Because snow depth controls
thermal resistance, those frost residuals cannot be uniquely attributed to
soil heat-flow or freeze/thaw physics.

At Mandan, {{quantity:SF-V-FROST-MANDAN-EXCEEDANCES}} of
{{quantity:SF-V-FROST-MANDAN-BOUNDS}} isotherm bounds were exceeded
({{quantity:SF-V-FROST-MANDAN-EXCEEDANCE-PERCENT}}), with a maximum margin of
{{quantity:SF-V-FROST-MANDAN-MAX-MARGIN}}. Reynolds Creek had
{{quantity:SF-V-FROST-REYNOLDS-EXCEEDANCES}} exceedances among
{{quantity:SF-V-FROST-REYNOLDS-BOUNDS}}
({{quantity:SF-V-FROST-REYNOLDS-EXCEEDANCE-PERCENT}}), with a maximum margin of
{{quantity:SF-V-FROST-REYNOLDS-MAX-MARGIN}}. Both sites lacked paired observed
snow depth, and the zero-degree isotherm is not the same physical boundary as a
frost-tube ice front. The heterogeneous adverse evidence is retained, but it is
not a transferable frost-depth error distribution. Their pooled exceedance
rate, {{quantity:SF-V-ISOTHERM-EXCEED-PERCENT}}, is retained only as arithmetic
context and must not replace the site-specific rates.

### 4.4 Conservation and real production consumers

{{figure:SF-FIGURE-PARTITION}}

{{table:SF-TABLE-CONSERVATION-ROWS}}

The maximum active-row rain-plus-snow partition residual was
{{quantity:SF-V-PARTITION-RESIDUAL}}, compared with a declared allowance of
{{quantity:SF-V-PARTITION-TOLERANCE}}. For the four selected-row WAT spot
verifications, independent reconstruction gave a snow accumulation-day storage residual of
{{quantity:SF-V-SNOW-ACCUM-RESIDUAL}}, a release-day residual of
{{quantity:SF-V-SNOW-RELEASE-RESIDUAL}}, and a maximum combined frozen-plus-
liquid soil-water residual of
{{quantity:SF-V-FROST-STORAGE-RESIDUAL}}. The table exposes storage and transfer
magnitudes beside each residual. These are selected-row numerical closure
results, not all-row evidence or estimates of measurement or model-form error.

## 5. Discussion

### 5.1 What the evidence supports

The precipitation-phase evidence supports the physical choice to represent
humidity rather than use one universal air-temperature threshold. It is both
large in sample size and mechanistically coherent: classification improves, the
humidity contrast has the observed sign, and production partition mass closes.
The remaining warm station-threshold bias is visible and should be carried into
snowfall interpretation rather than erased by the classification aggregate.

The seasonal snow evidence supports a model that reproduces many robust aspects
of accumulation, density, timing, and depth-SWE response across disparate
climates and vegetation surfaces. The residuals identify three research needs:
vertical or structural density evolution, mountain snow persistence and
representativeness, and canopy/subcanopy control of depth relative to SWE. That
is a more useful conclusion than either “validated” or “insufficient.”

The campaign's authority result sharpens, but does not strengthen, that
empirical conclusion. Corrected wet-compaction lineage is now authoritative;
warm/mixed loss evidence remains multifactor and observational; CoE is
baseline-faithful but lacks an adequate independent production envelope; and
Stage 3 is a future implementation target, not an evaluated production
replacement. No noninferiority or default-change claim follows from authority
admission alone.

The frozen-soil evidence supports implementation credibility and shows that the
model responds across long observational records. It does not yet isolate
frozen-soil predictive skill. This is not because the frost model lacks
evidence; it is because the dominant boundary condition—snow depth—was wrong or
unobserved in the retained comparisons. A scientifically defensible next
campaign must first obtain acceptable paired snow state or propagate snow-
boundary uncertainty into frost conclusions.

### 5.2 Relation to prior knowledge

Harder-Pomeroy phase physics and the Jennings station analysis both imply that
humidity shifts the rain-snow transition, consistent with the present
directional result. Anderson/SNOBAL and Sturm research establish that snow
density evolves with metamorphism, overburden, liquid water, temperature, and
snow climate; the diffuse densification residual is therefore plausible for a
bulk model and motivates, but does not by itself validate, multilayer work.
Published frost literature likewise treats snow thermal resistance as a
first-order control, supporting the decision not to tune frozen-soil physics
against snow-confounded residuals.

### 5.3 Application interpretation

The tested domains include thousands of phase stations, five mountain SNOTEL
climates, five northeastern canopy surfaces, and five frozen-soil sites. That
breadth makes the evidence relevant to scientific scrutiny, but it does not
remove site dependence. A practitioner must compare their forcing source,
elevation, snow climate, canopy, wind exposure, soil, management, quantity of
interest, and consequence of error with these domains. In particular, accurate
phase classification does not guarantee accurate SWE magnitude, and
conservation does not guarantee correct runoff timing.

## 6. Limitations

- All empirical analyses were retrospective and influenced model development,
  mechanism rejection, or activation; no untouched held-out site set was
  retained for this synthesis.
- The full Jennings hourly observation file is large and locally retained from
  the CC0 Dryad archive rather than committed to Git. Reproduction requires
  reacquisition by its DOI and verification against the admitted procedure.
- SNOTEL is point support in open clearings. The modeled surfaces are hillslope
  representations driven by mixed DAYMET, GRIDMET, CLIGEN, and PRISM products;
  absolute precipitation and temperature uncertainty was not propagated.
- SNOTEL snow depth is station-derived, begins later than SWE at every site, and
  can contain sensor artifacts. Bulk density compounds SWE and depth error.
- The ten-surface snow rubric is a development diagnostic. Its ordinal bands
  are signature-specific and cannot be interpreted as a probability of
  correctness or a transferable accuracy grade.
- The later Snowbird precipitation-normalized lane is development-only input-
  sensitivity evidence. It is not precipitation truth, calibration,
  independent validation, or authority to transfer a multiplier to production.
- The 21L warm/mixed contrasts are chronology-confounded and cannot identify a
  unique forcing, radiation, density, canopy, or melt-process owner.
- Stage 3 future melt ownership is canonical target authority, not implemented
  physics, empirical efficacy, noninferiority, a default change, or a runtime
  cutover. CoE remains the compatibility owner until all atomic cutover gates
  pass.
- The evidence does not support transfer to warm-maritime conifer conditions;
  that claim remains explicitly withheld.
- Wind redistribution, canopy interception/unloading, subcanopy longwave
  radiation, and vertical snow layering are incomplete or simplified relative
  to advanced snow models.
- Frost-tube and soil-temperature isotherm observations are not interchangeable.
  Failed or missing snow control prevents clean attribution of frozen-soil
  residuals.
- Evidence spans named historical software realizations. Static source currency
  does not replace a fresh release reproduction, and this report has no release
  transfer.
- The study evaluates process behavior and software realization, not runoff,
  erosion, operational forecasting, design, regulation, or site-specific
  fitness.

## 7. Conclusions

openWEPP's snow and frozen-soil process chain is supported by substantial and
quantitative evidence. The humidity-aware precipitation-phase formulation is
substantially corroborated across the scored retrospective station corpus and
behaves correctly in the production consumer. Seasonal snow comparisons locate
persistent limitations in
densification, mountain persistence, and depth-SWE geometry. Conservation and
consumer checks verify precipitation partition across the retained production
trace and water-storage arithmetic at four selected WAT rows.

The post-synthesis campaign corrected a duplicate wet-compaction operand, and
its annual-first site medians showed that corrected pre-peak loss is
concentrated on warm or mixed days, but it did not isolate one causal
mechanism. The current CoE melt implementation
matches the pinned post-2007 baseline while lacking a sufficient independent
validation envelope for its material changes. Stage 3 is therefore admitted as
the future sole melt owner under v7/v126, while CoE remains the byte-identical
current compatibility runtime on implementation hold. This authority decision
does not establish Stage 3 efficacy, noninferiority, transferability, or
production readiness.

The frozen-soil evidence is valuable but not independently attributable because
snow-depth control frequently failed or was unavailable. The appropriate
scientific conclusion is therefore claim-specific: phase and snow-process
credibility are well supported within the tested retrospective domains;
seasonal residual mechanisms are known and nontrivial; and transferable
frost-depth accuracy remains unresolved pending a design that controls snow
insulation and forcing uncertainty.

These findings equip, but do not replace, an application decision by a
hydrologist, soil scientist, practitioner, or responsible institution.

## 8. Open Research and Reproduction

The report binds the exact 21K-21N terminal dispositions as inputs. Its
version-bound research-object surface exposes their public-safe authority-
impact extracts alongside the compact strict result, reconstruction procedure,
conservation operands, four primary machine-readable evidence sources,
production conservation record, snow/frost and snow-energy science contracts,
exact phase-scoring implementation, selected-row operand log, the
{{link:research-object:SF-OBJECT-REFRESH-PROMPT|archived authority-refresh prompt}},
and {{link:research-object:SF-OBJECT-DATASET-PROVENANCE|dataset provenance}}.
The technical supplement maps each
claim to those objects and gives the exact offline command.

Priority independent work is to preregister and run held-out site evaluation;
propagate forcing and snow-boundary uncertainty; evaluate snow density and
layering without site calibration; add paired snow observations to frozen-soil
sites; implement and close the complete Stage 3 sole-owner cutover without a
dual-owner interval; evaluate warm-maritime conifer transfer independently;
and reproduce the selected evidence against an exact release candidate.

## References

- Anderson, E. A. 1976. *A Point Energy and Mass Balance Model of a Snow Cover*.
  NOAA Technical Report NWS 19.
{{reference:SF-REF-HARDER}}

{{reference:SF-REF-JENNINGS-PAPER}}

{{reference:SF-REF-JENNINGS-DATA}}

- Sturm, M., Holmgren, J., König, M., and Morris, K. 1997. The thermal
  conductivity of seasonal snow. *Journal of Glaciology* 43:26–41. DOI
  `10.3189/S0022143000002781`.
{{reference:SF-REF-STURM}}

{{reference:SF-REF-DUN-2010}}

{{reference:SF-REF-SLEEPERS}}

{{reference:SF-REF-GGD498}}

{{reference:SF-REF-ENERGY-CONTRACT}}

NRCS SNOTEL and SCAN station records and USDA-ARS Reynolds Creek soil-
temperature records are identified in the dataset-provenance research object.
{{reference:SF-REF-CONTRACT}}

## About This Report

This report is production-domain V2 source version 1.0.0. It synthesizes named
historical evidence at the source identities listed in its supplement, was
assembled at openWEPP Git `47c2cf9eae6eef95f0f670d157d2d31df4cbf9cc`, and
was refreshed on 2026-08-05 against campaign increments 21K-21N and canonical
v7/v126 authority.
Codex drafted the report and deterministic reconstruction procedure. The
current attribution and governance status below are generated from the
principal registry, report descriptor, and review lock.

{{assurance:attribution}}

{{assurance:lifecycle}}
