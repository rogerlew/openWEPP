# CAL-07E Literature Synthesis

Evidence class: `Static + externally retrieved primary literature`

## Answer in brief

The literature supports water availability as an important contributor to
Bezà Mahafaly canopy phenology, but it does not support a single rainfall,
VPD, photoperiod, or temperature trigger. The direct-site record shows strong
seasonal association with rainfall and later leaf retention in gallery forest
than in xerophytic forest. It does not
measure soil water, groundwater access, plant water status, or VPD well enough
to select a production equation.

The observation side also needs reconciliation before more calibration work.
CAL-07 used PhenoCam `gcc_mean` transition dates, while CAL-07D attached
`smooth_gcc_90` as ancillary daily context. The products agree closely for
rising transitions but differ by 12 to 43 days for four of six falling
threshold-year combinations. That inconsistency does not erase the broad
seasonal contradiction, but it prevents treating CAL-07D's falling-date
residuals as product-invariant.

## Direct-site evidence

Rasamimanana, Ratsirarson, and Richard (2012) provide the strongest admitted
authority. They observed 307 trees from 26 species twice monthly in gallery
and xerophytic forest, alongside local daily rainfall and minimum/maximum
temperature. Leaf abundance was strongly associated with monthly rainfall
(`rs = 0.84` gallery; `rs = 0.94` xerophytic; `n = 12`; reported `p < 0.05`).
The authors propose that plants may advance defoliation in dry years and delay
it in wet years. The paper does not report a quantified interannual
defoliation-timing test, so CAL-07E retains that proposed response as
`UNRESOLVED`, not as a direct observation.

The two forest types did not share one seasonal envelope. Leafing began around
1 November in gallery forest and 6 November in xerophytic forest, but the
reported mean end of leafing was around 3 July in gallery forest and 19 May in
xerophytic forest. This supports an ecologically meaningful habitat contrast.
The proposed influence of Sakamena groundwater on gallery forest is plausible
site interpretation, not a measured groundwater mechanism.

Temperature was correlated with phenology, but its seasonal covariance with
rainfall and the study design do not isolate an independent temperature
control. VPD was not evaluated as an independent variable. CAL-07D's VPD
sensitivity therefore remains a model attribution, not a field-confirmed VPD
mechanism.

O'Mara and Hickey (2014) corroborate the existence and scale of direct
phenology monitoring at Bezà—402 woody individuals across 22 transects during
a below-typical-rainfall interval—but their feeding-ecology design does not
isolate a canopy trigger.

## Regional and analogue evidence

At Tsimanampetsotsa in southwestern Madagascar, Ratovonamana et al. (2011)
found leaflessness negatively associated with contemporary rainfall across
three vegetation types (`rs = -0.71` to `-0.76`). Day length covaried with
other seasonal variables, and reported leaf-fall correlations with day length
were not significant. Species differed: some patterns were compatible with
shorter days or temperature, while many tracked ongoing water stress. This
supports multiple cues and species/habitat stratification, not one transferable
threshold.

Chapotin, Razanameharizaka, and Holbrook (2006) supply a useful physiological
separation in western-Madagascar baobabs. Stored stem water supported leaf
flush before the rains, while stomata remained largely closed until rainfall
increased soil water. Leaf appearance and physiologically active canopy can
therefore have distinct water controls. This is species-specific regional
evidence and cannot be promoted into a general Bezà forest rule.

Studies outside Madagascar show why photoperiod, soil water, and hydraulic
traits remain credible hypotheses. Rivera et al. (2002) documented
increasing-day-length-associated pre-rain flushing in many tropical dry-forest
trees, but also described climatic and species limits. Méndez-Alonzo et al.
(2013) associated leaf habits with soil water availability, topography, and
xylem traits in Mexico. These are mechanism analogues only.

## What a PhenoCam transition means

Richardson et al. (2018) define PhenoCam transition dates from spline-fitted
seasonal GCC trajectories. Dates 10, 25, and 50 mark relative fractions of the
fitted seasonal GCC amplitude. They are not 10%, 25%, or 50% leaf cover, LAI,
biomass, canopy water use, or GSI. Standard transition records include
uncertainty bounds and multiple GCC products.

Young et al. (2025) distinguish the simplified transition record from the
standard record. The simplified record is derived from `GCC_mean` and omits
uncertainty and other fit context carried by the standard product. Network
release consistency is high overall, but variable and arid systems are among
the cases where automated alignment can be harder.

Method comparisons further constrain interpretation. Donnelly et al. (2022)
found that GCC threshold dates can disagree materially with in-situ
phenophases and that the species represented in a camera ROI matters. Keenan
et al. (2014) showed that canopy GCC and structural or physiological measures
are nonlinear, including bright early-leaf greenness. Published offsets from
those temperate studies are warnings, not Bezà correction factors.

## CAL-07/CAL-07D product audit

The retained CAL-07 simplified transition file exactly identifies
`gcc_mean` transitions. CAL-07D's daily observation-support artifact uses
`smooth_gcc_90`. Comparing both products in the provisional standard Data
Record 5 processed on 26 July 2026 shows:

- rising nominal dates differ by zero or one day in 2024 and 2025;
- 2024 falling T50 differs by 12 days;
- 2025 falling T10, T25, and T50 differ by 29, 43, and 21 days; and
- uncertainty intervals do not make the products interchangeable.

The product mismatch is most consequential for falling chronology. A bounded
follow-up should rerun the observation comparison separately for
`gcc_mean` and `gcc_90`, carry standard-product confidence intervals, inspect
the ROI and daily curves, and freeze the chosen observation operator before
any calibration. It must not choose the product that happens to fit the model
best.

All exact site dates and confidence intervals in this package are provisional
as processed on 26 July and retrieved on 29 July 2026. CAL-07E retains the
eight source rows and source/archive checksums needed to reproduce this
snapshot. CAL-07F must freeze its input by checksum and must not assume a later
provider archive is byte-identical.

## Implications for the four solution routes

### Observation semantics

`ADVANCE TO METHOD AUDIT`, not correction. This is the strongest immediate
route. Direct product evidence identifies an operator inconsistency and
provides uncertainty metadata. Field phenology corroboration remains needed
to decide what the camera transition represents biologically.

### Forcing bias

`HOLD / ACQUIRE`. Direct-site literature establishes rainfall relevance but
does not validate NASA POWER precipitation, temperature, humidity, or VPD for
2024–2025. Quality-controlled on-site weather aligned with camera and field
phenology is still required.

### Parameter or ecotype transfer

`HOLD / STRATIFY HYPOTHESES`. Gallery-versus-xerophytic differences and
regional species heterogeneity make a single generalized threshold doubtful.
No admitted source supplies a defensible Bezà GSI threshold or independently
reserved calibration/validation lane.

### Missing process

`HOLD / MEASURE`. Rainfall, rooting-zone or groundwater access, stored plant
water, and photoperiod are scientifically credible contributors. Their
relative roles are unresolved at the ROI. No production water-cue equation is
authorized.

## Scientific stop rule

The review stopped after:

1. one full-text direct-site phenology study and one direct-site context study;
2. two full-text Madagascar regional mechanism studies;
3. two full-text tropical dry-forest analogues;
4. four full-text PhenoCam method or validation studies;
5. the retained provisional standard-record site transition product; and
6. targeted searches for the cataloged 2014 thesis and post-2011 site records.

Additional generic dry-forest papers would add mechanisms but not change the
authority ceiling. Progress now depends on direct-site acquisition and a
product-consistent observation-method audit.

## Boundaries

This review does not authorize production code, contract amendments, forcing
substitution, parameter selection, or a new process formulation. It supports
a method-audit follow-up and precise requests for direct-site records.
