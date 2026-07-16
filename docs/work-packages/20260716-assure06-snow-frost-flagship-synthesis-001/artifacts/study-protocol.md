# ASSURE-06 Study Protocol

Evidence class: Static

## Scientific Question

What does the admitted observational and production evidence show about the
ability of openWEPP's current snow and frozen-soil process chain to partition
precipitation, represent seasonal snow accumulation and ablation, conserve snow
and frozen-soil water, and reproduce observed snowpack and frozen-soil response
across the tested domains?

This is a synthesis question, not a universal model grade. Four substudies have
different referents and therefore retain separate methods and conclusions:

1. hourly rain/snow classification against the Jennings et al. station corpus;
2. daily SWE, physical snow depth, bulk density, and seasonal signatures at five
   SNOTEL and five canopy-site surfaces;
3. frost-tube depth and soil-temperature zero-isotherm behavior at five
   non-SNOTEL sites; and
4. conservation and production-path verification from generated outputs.

## Integrated-Versus-Split Decision

ASSURE-06 will author one integrated synthesis. The causal chain is the reason:
precipitation phase controls snow input; accumulation, ablation, and density
control snow cover and insulation; snow insulation controls how frost-depth and
soil-temperature comparisons may be interpreted. The retained frozen-soil
evidence explicitly demonstrates that frost residuals cannot be attributed to
frost physics while snow-depth control fails or is unavailable. Separating the
frozen-soil result from the snow evidence would make the conclusion easier to
misread.

Integration is allowed only with hard internal boundaries. Each substudy has a
separate method ID, result section, table, figure, uncertainty statement, and
claim. No pooled accuracy, pass rate, or aggregate validation label may cross
the substudy boundary. A later study seeking an independently corroborated
predictive claim for precipitation phase, snowpack state, or frost depth should
be published as its own report.

## Study Design And Prior Use Of Evidence

This is a retrospective synthesis of retained evidence. The observations were
not an untouched held-out validation set:

- Jennings observations were used to adjudicate and later activate the
  Harder-Pomeroy phase treatment. The report may describe agreement and the
  activation basis, but not independent post-selection validation.
- SNOTEL and canopy-site comparisons informed successive mechanism diagnoses,
  candidate rejection, and default selection. Their retained current-default
  profile is model-development and comparative evaluation evidence.
- Frost and soil-temperature observations were used to classify residuals and
  constrain which mechanism families could be changed. They do not provide an
  independent final frost-accuracy test.
- Conservation and consumer evidence verifies implementation properties; it is
  not empirical accuracy evidence.

The manuscript must state this retrospective role in the abstract, methods,
discussion, and conclusion. It may conclude that the evidence is substantial,
quantitative, and scientifically informative without relabeling it as an
independent validation campaign.

## Claim Envelopes

### SF-P01 — Precipitation phase

- Quantity: binary rain/snow occurrence classification and station 50% phase
  temperature.
- Scale/domain: hourly station observations in the admitted Jennings corpus.
- Referent: observed phase labels and station threshold summaries.
- Result: confusion-matrix counts, accuracy, threshold error, and humidity
  contrast.
- Boundary: retrospective selection evidence; it does not evaluate precipitation
  amount, mixed-phase fraction accuracy, or snowpack response by itself.

### SF-P02 — Seasonal snowpack

- Quantity: daily SWE, physical snow depth, derived bulk density, seasonal
  timing, and depth-SWE response.
- Scale/domain: five SNOTEL and five canopy-site point/hillslope comparison
  surfaces under their retained forcing configurations.
- Referent: admitted daily observations and a prespecified forcing-robust versus
  forcing-limited profile.
- Result: site sample counts, ordinal profile cells, signed residual direction,
  and residual families.
- Boundary: observations influenced model development; absolute SWE/depth are
  forcing-limited; ordinal scores are rubric summaries, not portable accuracy
  grades.

### SF-P03 — Frozen-soil response

- Quantity: frost-tube depth residual and soil-temperature zero-isotherm upper
  bound, with paired snow-depth control.
- Scale/domain: three frost-tube and two soil-temperature sites in the admitted
  five-site corpus.
- Referent: measured frost depth or measured temperature profiles.
- Result: matched counts, maximum residuals or exceedances, and snow-control
  counts.
- Boundary: the retained realization had failed or unavailable snow control at
  all five sites, so the evidence characterizes response but does not isolate
  frost-process error or support a transferable frost-depth accuracy claim.

### SF-P04 — Conservation and production behavior

- Quantity: hourly precipitation partition, daily snow water storage, and
  combined liquid/frozen soil water.
- Scale/domain: named production traces and focused generated-output fixtures.
- Referent: conservation identities reconstructed from produced outputs.
- Result: two-sided residuals and real WAT consumer evidence.
- Boundary: verifies the named calculations and consumers; it does not establish
  agreement with environmental observations.

## Metrics And Interpretation

- Phase evaluation reports confusion-matrix operands and accuracy for both
  formulations, plus threshold bias/MAE and low-versus-high-humidity contrast.
- Snowpack evaluation reports the full forcing-robust label distribution and
  site sample counts. It explains the 15 residual cells by process signature;
  it does not headline the ordinal score.
- Frozen-soil evaluation reports site-specific matched counts and residual
  extrema. It does not pool frost-tube and temperature-isotherm methods.
- Conservation reports absolute residuals beside tolerances or relevant storage
  magnitude. Exact self-consistency is not empirical validation.

## Uncertainty And Alternative Explanations

The report will address observation quality, spatial-support mismatch,
station-derived snow depth, density error propagation, forcing-product
differences, vegetation representation, wind redistribution, subcanopy energy,
parameter identifiability, retrospective selection, temporal alignment,
numerical tolerance, and software-realization currency. It will distinguish
mechanism evidence from forcing effects and say when alternatives cannot be
separated by the retained design.

## Stop Rules

Hold rather than publish if any material value cannot be reconstructed from an
identified source, if the report collapses the four claims into one grade, if a
development dataset is described as untouched validation, if frozen-soil
residuals are attributed through failed snow control, or if human authority is
absent. Builder success cannot override these rules.
