# Constraint-Removal Effect Screen

## Caption

Matched fractions and matched-row residual ranges for BASE, one-indicator removals, combined photoperiod/VPD removal, and the single canonical generalized-default trajectory. Circles are absolute GSI 0.5; squares are retrospective event-year relative midpoints.

## Plain-language takeaway

VPD removal creates all 148 relative-midpoint matches, but falling transitions remain early and rising transitions late. Photoperiod removal creates many absolute matches with smaller yet still directionally structured residuals. The canonical default crosses all four events but makes falling late and rising early. These results show mathematical sensitivity without identifying correct thresholds, forcing, or missing process physics.

## Methods and source bindings

`scenario-event-screen.csv` has 1,488 rows keyed by `(scenario,member_or_default,event_id,operator)`: 296 rows for each 37-member scenario and eight rows for the single default trajectory. Each unconstrained scenario is recomputed from 2022-01-01 with only its named indicator set to one before multiplication and FIFO admission. For each operator, event windows use adjacent date-50 midpoints and select the first same-direction crossing under `lower < crossing <= upper`. Empty `residual_days` means unmatched and contributes to the printed denominator but not the residual range. `decision-screen.csv` has seven rows keyed by `hypothesis` and supplies interpretation only; the plotted counts and ranges come from the event screen.

Exact result bindings:

- `scenario-event-screen.csv`, SHA-256 `51b967bdce7aaa8c72c7a1a73ba3b97b72b691279b5dd6db83e50ff952733051`
- `decision-screen.csv`, SHA-256 `063b9423f51230875d9f8430defa194df4000c45aefb2d26f04a9240659f2c8a`

## Assumptions and evidence ceiling

All relative model levels and constraint-removal scenarios are
`ASSUMED_FOR_EXECUTION`. They diagnose scale and mathematical suppression;
they are not fitted observation operators, calibrated parameters,
physiological bounds, process replacements, or production recommendations.
PhenoCam GCC is not treated as GSI, LAI, biomass, or canopy cover. Order 7
remains held.

## Limitations

Match availability is not timing accuracy. Residual ranges are shown without a pass tolerance. Combined removal can erase absolute crossings by keeping the trajectory above 0.5.

## Accessibility

Absolute and relative operators use separate marker shapes and colors. Counts are printed beside every point; residual ranges include a zero reference gridline.
