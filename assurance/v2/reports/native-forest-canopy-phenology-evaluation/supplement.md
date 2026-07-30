# Technical Supplement: Native-Forest Canopy-Phenology Evaluation

{{assurance:attribution}}

{{assurance:lifecycle}}

## S1. Study Decomposition And Claim Envelopes

The report combines one process chain because the same daily phenology state
feeds foliage, litter, residue, snow, frost, hydrology, and erosion consumers.
It does not pool evidence roles. The separate envelopes are:

1. mathematical, implementation, mass, state, and consumer verification;
2. Hubbard Brook empirical calibration;
3. Harvard independent timing evaluation without refit;
4. litter-source/decomposition calibration readiness;
5. canopy-gradient model-response characterization and bounded observation
   comparisons;
6. Southern Hemisphere phase verification and independent observational
   evaluation; and
7. Elliot legacy comparison.

The verdict vocabulary is `SUPPORTED`, `BOUNDED`, `CONTRADICTED`, and
`NOT_EVALUATED`. These terms describe only the named process, quantity,
domain, realization, referent, and method.

## S2. Science And Consumer Lineage

The authoritative process contracts are `SC-PLANT-001`,
`SC-RESIDUE-001`, and `SC-INFILE-MANAGEMENT-YAML-001`. The implementation
sequence is:

```text
daily weather + signed latitude
  -> temperature, VPD, and photoperiod indicators
  -> instantaneous GSI
  -> available-day GSI21 mean
  -> evergreen + deciduous foliar fraction
  -> foliar biomass, LAI, cover, and height
  -> leaf allocation or litter
  -> surface residue addition and decomposition
  -> residue cover and frost-facing depth
  -> snow, frost, ET, interception, runoff, and erosion consumers
```

The first day initializes foliage without allocation or litter. Later daily
closure is `previous foliage + allocation - litter - current foliage = 0`.
Structural biomass is not a leaf-transfer operand. Predictive needle and
fine-woody deposition are absent; authenticated observed daily forcing is a
separate boundary.

## S3. Realization And Input Identities

The assessed source realization is commit
`c42dde3136bbbf2b9c8a62ffe96ca6d28d77e615`. The report retrospectively
synthesizes executed evidence from the completed canopy implementation and
CAL-01 through CAL-07F. Fresh CAL-09 reconstruction reads seven retained CSV
objects:

| Role | Retained source |
| --- | --- |
| Accepted timing ensemble | `accepted-calibration-ensemble.csv` from CAL-04B |
| Complete timing search | `candidate-configurations.csv` from CAL-04B |
| Harvard timing holdout | `harvard-holdout-results.csv` from CAL-04B |
| Source-decay ridge | `terminal-stock-equifinality.csv` from CAL-05 |
| Canopy-gradient ensemble | `ensemble-summary.csv` from CAL-06 |
| Bezà product/member scores | `member-summary.csv` from CAL-07F |
| Elliot target comparison | `report-comparison.csv` from CAL-02 |

The report descriptor binds these paths as local content. The CAL-09
reproduction record retains SHA-256 identities and verifies that fresh output
matches the strict result byte for byte.

## S4. Evidence Roles And Observation Operators

Hubbard observations were assigned to `CALIBRATION`. The complete candidate
search was frozen before Harvard was opened. Harvard is
`INDEPENDENT_VALIDATION` with respect to member selection, although the small
campaign and shared model-development context still limit generalization.

CAL-05 synthetic daily stocks and ridge values are `SOFTWARE_VERIFICATION` and
`DIAGNOSTIC_ONLY`; they demonstrate identifiability structure, not empirical
forest decomposition. CAL-06 canopy time series are `MODEL_OUTPUT`.
Exact-date Harvard and Marcell snow observations are independent comparison
data, but no complete uncertainty model or acceptance tolerance was available.

Alerce camera and forcing work is diagnostic after hourly VPD reconstruction.
Bezà `gcc_mean` and `gcc_90` are separate `INDEPENDENT_VALIDATION` observation
products. Relative camera greenness thresholds are compared with same-direction
GSI21 crossings within seasonal windows. Missing crossings receive the frozen
penalty; wrong-season recovery crossings are excluded from the principal
operator and retained in sensitivity analysis.

Elliot output is `LEGACY_COMPARISON`. Legacy agreement is not correctness
authority.

## S5. Deterministic Reconstruction

Run from the repository root:

```text
.venv/bin/python \
  assurance/v2/reports/native-forest-canopy-phenology-evaluation/procedures/reproduce_canopy_synthesis.py \
  --accepted-ensemble docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/accepted-calibration-ensemble.csv \
  --candidate-configurations docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/candidate-configurations.csv \
  --harvard-holdout docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/harvard-holdout-results.csv \
  --litter-ridge docs/work-packages/20260728-canopy-cal-05-litter-source-decomposition-readiness-001/artifacts/terminal-stock-equifinality.csv \
  --gradient-summary docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/artifacts/ensemble-summary.csv \
  --beza-members docs/work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/artifacts/member-summary.csv \
  --elliot-comparison docs/work-packages/20260726-canopy-cal-02-elliot-reproduction-001/artifacts/report-comparison.csv \
  --output /tmp/canopy-synthesis.json
cmp /tmp/canopy-synthesis.json \
  assurance/v2/reports/native-forest-canopy-phenology-evaluation/results/canopy-synthesis.json
```

The procedure asserts the accepted/holdout count, exact CAL-06 site/stratum
inventory, {{quantity:CANOPY-V-ACCEPTED-MEMBERS}} in every forest
gradient, and presence of the best Bezà member under both products. It
calculates minima, maxima, counts, percentages, and ridge differences from
retained rows.

## S6. Timing Search And Identifiability

CAL-04B evaluated {{quantity:CANOPY-V-SEARCHED-CONFIGURATIONS}}. The
minimum-plus-one-day rule retained {{quantity:CANOPY-V-ACCEPTED-MEMBERS}}. All
accepted members lie on the admitted support boundary:
{{quantity:CANOPY-V-DOUBLE-BOUNDARY}} are double-boundary cases and
{{quantity:CANOPY-V-UPPER-BOUNDARY}} are upper-support-boundary cases. Timing
is therefore `PARTIALLY_IDENTIFIABLE`.

The admitted Hubbard-only marginal ranges are:

| Coefficient | Retained accepted range | Units |
| --- | ---: | --- |
| Minimum-temperature inactive threshold | {{quantity:CANOPY-V-TMIN-INACTIVE-MIN}} to {{quantity:CANOPY-V-TMIN-INACTIVE-MAX}} | °C |
| Minimum-temperature unconstrained threshold | {{quantity:CANOPY-V-TMIN-UNCONSTRAINED-MIN}} to {{quantity:CANOPY-V-TMIN-UNCONSTRAINED-MAX}} | °C |
| VPD unconstrained threshold | {{quantity:CANOPY-V-VPD-UNCONSTRAINED-MIN}} to {{quantity:CANOPY-V-VPD-UNCONSTRAINED-MAX}} | Pa |
| VPD inactive threshold | {{quantity:CANOPY-V-VPD-INACTIVE-MIN}} to {{quantity:CANOPY-V-VPD-INACTIVE-MAX}} | Pa |
| Photoperiod inactive threshold | {{quantity:CANOPY-V-PHOTOPERIOD-INACTIVE-MIN}} to {{quantity:CANOPY-V-PHOTOPERIOD-INACTIVE-MAX}} | h |
| Photoperiod unconstrained threshold | {{quantity:CANOPY-V-PHOTOPERIOD-UNCONSTRAINED-MIN}} to {{quantity:CANOPY-V-PHOTOPERIOD-UNCONSTRAINED-MAX}} | h |

These ranges are an accepted correlated ensemble, not independent priors,
physiological bounds, or universal calibration guidance.

Harvard scores were finite for {{quantity:CANOPY-V-ACCEPTED-MEMBERS}} out of
{{quantity:CANOPY-V-ACCEPTED-MEMBERS}} and ranged from
{{quantity:CANOPY-V-HARVARD-MIN}} to
{{quantity:CANOPY-V-HARVARD-MAX}}. Interval coverage was zero for
{{quantity:CANOPY-V-HARVARD-ZERO-COVERAGE}} and reached at most
{{quantity:CANOPY-V-HARVARD-MAX-COVERAGE}}.

## S7. Litter And Decomposition

The complete synthetic daily trace uniquely recovered frozen truth `S020-K050`
with zero daily-stock sum of squared errors. All other frozen grid members had
positive error. That demonstrates information flow through the source-rate
operator.

Terminal stock alone is nonidentifying. The five `RIDGE-K000` through
`RIDGE-K200` pairs converge on
{{quantity:CANOPY-V-RIDGE-STOCK}} at year 20; the largest absolute difference
is {{quantity:CANOPY-V-RIDGE-MAX-DIFF}}. These are assumed-for-execution
synthetic pairs, not fitted forest values.

Harvard observations include a pooled nonfoliar contribution but cannot
partition needle and fine-woody sources or convert organic-horizon carbon into
modeled dry residue mass. No missing source is set to zero and no decay
constant is fitted.

## S8. Canopy Gradient And Downstream Cells

The prespecified matrix comprised seven forest lanes crossed with
{{quantity:CANOPY-V-ACCEPTED-MEMBERS}} plus two open controls:
{{quantity:CANOPY-V-GRADIENT-RUNS}}. Winter cover ordering held for every
member:

- Marcell: deciduous < mixed < conifer;
- Harvard: deciduous < mixed; and
- Hubbard Brook: deciduous < mixed.

This is within-model congruence. Seed foliar mass, evergreen fraction, LAI,
structural cover, and closure remain data-limited.

Harvard and Marcell snow depth and density were compared on exact dates. The
reported bias, MAE, and RMSE are bounded because no source uncertainty model or
acceptance threshold was authorized. Harvard SWE is excluded: metadata says
centimeters, while raw values conflict by about tenfold with the same-row
depth-density identity. Marcell SWE remains usable within its own stated
limits.

Predictive needle and fine-woody sources were null in every forest/member run.
Consequently, total residue adequacy and frost consequences do not advance.
ET, interception, and runoff are modeled responses. Erosion-facing inputs were
present, but no erosion consequence output was emitted.

## S9. Southern Hemisphere Diagnostics

CAL-07B attributed Alerce's three negative daily-summary VPD values to the
temperature-extrema summary operator. CAL-07C retained the paired hourly
products, admitted 1,666 finite nonnegative daily signed-mean VPD values, and
proved zero executor residual without clipping.

CAL-07F retained all 24 Bezà product/year/direction/threshold transition
records. Member ranks were identical for `gcc_mean` and `gcc_90`, and their
top-quartile sets overlapped completely. {{quantity:CANOPY-V-BEZA-COMPLETE}}
completed all 12 crossings under both products. Best member `GSI-4831`
intersected one of 12 `gcc_mean` confidence intervals and none of the 12
`gcc_90` intervals:

| Observation product | Interval-hit total |
| --- | ---: |
| `gcc_mean` | {{quantity:CANOPY-V-BEZA-MEAN-HITS}} |
| `gcc_90` | {{quantity:CANOPY-V-BEZA-GCC90-HITS}} |

Penalized
errors were {{quantity:CANOPY-V-BEZA-MEAN-ERROR}} and
{{quantity:CANOPY-V-BEZA-GCC90-ERROR}}.

Only operator independence and mechanical year-role separation passed the six
prospective calibration criteria. Crossing sufficiency, uncertainty fit,
direction coherence, and parameter plausibility failed. Removing the seasonal
window admitted wrong-season recovery crossings but did not produce a joint
uncertainty or direction-coherence passer.

## S10. Elliot Scorecard

The retained comparison contains {{quantity:CANOPY-V-ELLIOT-TARGETS}}, of
which {{quantity:CANOPY-V-ELLIOT-CONTRADICTED}} are classified
`CONTRADICTED`. Hubbard `dropfc=0.95` reproduced live biomass within the frozen
tolerance, but old and total residue were high. Hubbard `dropfc=0.92` produced
substantially lower live biomass. Santee live biomass, residue, hydrology, and
sediment targets mostly disagreed.

The exact historical Windows project and manually transcribed soil remain
unavailable. The `NOT_REPRODUCIBLE` verdict is therefore bounded to the
authorized reconstructions and does not establish a defect in openWEPP.

## S11. Figure And Table Inventory

{{figure:CANOPY-FIGURE-F2}}

{{figure:CANOPY-FIGURE-F7}}

Main-report time-series research objects are F1, F3, F4, F5, F6, and F8.
F2 and F7 are supplement objects because their broader context is useful but
duplicates the main causal spine or precedes the more diagnostic product-
consistent figure. Each SVG has a Markdown sidecar with caption, quantities,
units, source identity, ancillary interpretation, and limitations.

The V2 assembler's current result-figure vocabulary is limited to linear
magnitude bars. `CANOPY-FIGURE-TRANSFER` therefore gives a compact,
schema-native visualization of Harvard error-range endpoints. It does not
replace the reviewed time-series research objects.

## S12. Adverse, Excluded, And Not-Evaluated Evidence

- Harvard timing transfer is contradicted.
- Bezà tropical dry-forest chronology is contradicted.
- Predictive needle and fine-woody source laws lack authority.
- Harvard SWE is excluded for a unit/identity contradiction.
- Camera GCC is not treated as direct LAI or dry foliar mass.
- Canopy-amplitude accuracy lacks independent tolerances.
- Total litter/residue and frost adequacy do not advance where sources are
  missing.
- ET and runoff consequences are not calibration evidence.
- Erosion consequence output is unavailable.
- Elliot numerical targets are not reproducible and are not correctness
  authority.

## S13. Public Research-Object Inventory

The complete public-safe object set is linked here so a reader can inspect the
strict result, its inputs, the figure sources, and every caption:

- {{link:usersum:openwepp-canopy-phenology.md|model narrative}}
- {{link:research-object:CANOPY-OBJECT-RESULT|strict synthesis result}}
- {{link:research-object:CANOPY-OBJECT-REPRODUCER|synthesis reproducer}}
- {{link:research-object:CANOPY-OBJECT-ACCEPTED|accepted calibration ensemble}}
- {{link:research-object:CANOPY-OBJECT-HARVARD|Harvard holdout scores}}
- {{link:research-object:CANOPY-OBJECT-RIDGE|terminal-stock ridge}}
- {{link:research-object:CANOPY-OBJECT-GRADIENT|gradient ensemble summary}}
- {{link:research-object:CANOPY-OBJECT-BEZA|Bezà member summary}}
- {{link:research-object:CANOPY-OBJECT-ELLIOT|Elliot comparison}}
- {{link:research-object:CANOPY-OBJECT-FIGURE-BUILDER|figure builder}}
- {{link:research-object:CANOPY-OBJECT-FIGURE-MANIFEST|figure source manifest}}
- {{link:research-object:CANOPY-OBJECT-DAILY-CLIMATOLOGY|daily canopy climatology}}
- {{link:research-object:CANOPY-OBJECT-CONFIGURATIONS|complete timing configurations}}
- {{link:research-object:CANOPY-OBJECT-RIDGE-DESIGN|source-decay ridge design}}
- {{link:research-object:CANOPY-OBJECT-SOUTHERN-DAILY|Southern daily ensemble}}
- {{link:research-object:CANOPY-OBJECT-BEZA-CURVES|Bezà daily product curves}}
- {{link:research-object:CANOPY-OBJECT-HARVARD-SNOW|Harvard snow observations}}
- {{link:research-object:CANOPY-OBJECT-MARCELL-SNOW|Marcell snow observations}}
- {{link:research-object:CANOPY-OBJECT-F1-DATA|F1 coefficient data}}
- {{link:research-object:CANOPY-OBJECT-F4-DATA|F4 timing data}}
- {{link:research-object:CANOPY-OBJECT-F6-DATA|F6 snow data}}
- {{link:research-object:CANOPY-OBJECT-F1|F1 coefficient-response figure}}
- {{link:research-object:CANOPY-OBJECT-F2|F2 forest-class figure}}
- {{link:research-object:CANOPY-OBJECT-F3|F3 residue/frost figure}}
- {{link:research-object:CANOPY-OBJECT-F4|F4 temperate timing figure}}
- {{link:research-object:CANOPY-OBJECT-F5|F5 source-decay figure}}
- {{link:research-object:CANOPY-OBJECT-F6|F6 snow-response figure}}
- {{link:research-object:CANOPY-OBJECT-F7|F7 hemisphere figure}}
- {{link:research-object:CANOPY-OBJECT-F8|F8 Bezà figure}}
- {{link:research-object:CANOPY-OBJECT-F1-SIDECAR|F1 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F2-SIDECAR|F2 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F3-SIDECAR|F3 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F4-SIDECAR|F4 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F5-SIDECAR|F5 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F6-SIDECAR|F6 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F7-SIDECAR|F7 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-F7-DATA|F7 relative-seasonality data}}
- {{link:research-object:CANOPY-OBJECT-F8-SIDECAR|F8 caption and limitations}}
- {{link:research-object:CANOPY-OBJECT-AGENT-PACKET|agent-assistance disclosure}}

## S14. References

{{reference:CANOPY-REF-JOLLY}}

{{reference:CANOPY-REF-ALLEN}}

{{reference:CANOPY-REF-KEENAN}}

{{reference:CANOPY-REF-LIM}}

{{reference:CANOPY-REF-RIVERA}}

{{reference:CANOPY-REF-CHAPOTIN}}

{{reference:CANOPY-REF-DONNELLY}}

{{reference:CANOPY-REF-FLANAGAN}}

{{reference:CANOPY-REF-KEANE}}

{{reference:CANOPY-REF-MENDEZ}}

{{reference:CANOPY-REF-OLSON}}

{{reference:CANOPY-REF-QUALLS}}

## S15. Review And Publication Boundary

The source remains `DRAFT`. Internal agent review, deterministic build, and
reconstruction cannot supply accountable human scientific approval,
reproduction/publication approval, assurance-steward authorization, or release
transfer. No draft output may enter tracked `usersum`, export, snapshot, or
vendor surfaces.
