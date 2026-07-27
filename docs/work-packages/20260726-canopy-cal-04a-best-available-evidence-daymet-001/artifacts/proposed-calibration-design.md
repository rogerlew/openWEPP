# Proposed Calibration Design

Evidence class: `Prospective design authority from best-available evidence`

## Executable finite design

This is a coarse sensitivity ensemble, not an exhaustive set of candidates or
evidence-bounded physiological parameter domain. The forcing population is all
nine Hubbard plots for 1989–2024 on Daymet
yday 60–180. For Tmin, derived VPD, and native photoperiod, use the seven
empirical support levels q00, q05, q25, q50, q75, q95, and q100. Construct
every strictly ordered lower/upper pair within each family. This yields 21
pairs per family and `21 × 21 × 21 = 9,261` complete six-threshold vectors.
Enumerate lexicographically by temperature pair ID, VPD pair ID, then
photoperiod pair ID. There is no random seed.

The precise values and field orientation are in `proposed-domain-grid.csv`.
They are forcing-support points, not probability priors and not inferred
threshold estimates. The broad q00/q100 profiles intentionally expose
saturation and boundary behavior rather than pretending those extremes are
likely parameter values.

## Objective and retention

The later calibration package must retain the CAL-04/05 interval-censored,
equal-year timing operator without substituting midpoint dates. Evaluate full
vectors; never optimize one family while freezing another to current native
values. The accepted best-available ensemble is every finite profile whose
equal-year interval RMSE is no more than `minimum finite RMSE + 1.0 day`.
The one-day tolerance is an explicit execution assumption, chosen before
scores and narrower than the weekly observation cadence; it is not a
confidence interval. Publish profile-wise predictions, failures, saturation
class, and boundary flags.

Classes are:

- `INTERIOR`: neither threshold uses q00 or q100;
- `LOWER_SUPPORT_BOUNDARY`: at least one lower threshold uses q00;
- `UPPER_SUPPORT_BOUNDARY`: at least one upper threshold uses q100;
- `DOUBLE_BOUNDARY`: both conditions;
- `SATURATED_ON_OBSERVED_FORCING`: the modeled scalar family factor has
  `max(factor) - min(factor) <= 1e-12` over every Hubbard plot-day from yday
  60 through each interval's upper bound.

## Refinement and stopping

No refinement is authorized. Stop after the complete 9,261-vector coarse grid
and retain the accepted ensemble defined above. A boundary hit is reported; it
does not authorize extrapolation or widening. Any refinement or wider domain
requires a new, prospectively reviewed authority package.

## Interpretation

Temperature shows the strongest available independent timing leverage. VPD
remains a required ensemble axis because its conditional leverage is weaker
and correlated with temperature. Photoperiod supplies seasonal structure but
negligible within-plot interannual leverage. The intended product is therefore
a bounded equifinal ensemble, not a uniquely identified parameter vector.
