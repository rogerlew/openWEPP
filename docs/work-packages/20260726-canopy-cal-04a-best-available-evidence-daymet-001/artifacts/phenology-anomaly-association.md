# Phenology and Fixed-Window Forcing Anomalies

Evidence class: `Ran deterministic descriptive association`

To avoid defining meteorological predictors relative to the observed event
date, this analysis uses the fixed Daymet window yday 60–120. One descriptive
P3 interval midpoint is averaged per plot-year only for association
diagnostics; it is not treated as the calibration observation or exact event
date. Plot means are removed, leaving within-plot interannual anomalies
(`n=313` plot-years).

| Association | Pearson correlation |
| --- | ---: |
| P3 timing anomaly / mean Tmin anomaly | -0.4315 |
| P3 timing anomaly / mean VPD anomaly | -0.3304 |
| mean Tmin anomaly / mean VPD anomaly | 0.5493 |

Warmer and higher-VPD early springs are associated with earlier P3 brackets
(negative timing correlations). In a standardized two-predictor descriptive
regression, coefficients are -0.3581 for Tmin and -0.1337 for
VPD. The weaker conditional VPD coefficient and predictor correlation show
that the observations contain temperature leverage but limited independent VPD
leverage.

This is not a fitted GSI model, causal attribution, or threshold estimate.
Photoperiod is omitted from the fixed-window anomaly regression because it is
deterministic by latitude/calendar and has negligible interannual variation at
a fixed plot. Threshold calibration must still profile the complete GSI vector
and retain equifinality.
