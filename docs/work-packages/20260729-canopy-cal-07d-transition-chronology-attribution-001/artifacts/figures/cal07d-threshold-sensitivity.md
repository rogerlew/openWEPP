# Model-Level Threshold Sensitivity

## Caption

Matched member fractions and median matched-row residuals across eleven prospectively frozen retrospective model levels. Each line is one source date-50 event.

## Plain-language takeaway

Lower relative levels produce many crossings, but inferred timing and even match availability change sharply with level. No single relative level resolves both rising and falling chronology across years.

## Methods and source bindings

`model-level-sensitivity.csv` has 1,628 rows keyed by `(member_or_default,event_id,model_level)`: 37 members × four events × eleven levels. For each member/event/level, one threshold is calculated from the complete event-year GSI range and held constant over the adjacent-event window. The first chronological same-direction crossing under `lower < crossing <= upper` is selected. Every fraction denominator retains all 37 rows; an empty `residual_days` is unmatched. Residual medians use matched rows only, and a level with no matches is omitted from the right panel rather than imputed.

Exact result bindings:

- `model-level-sensitivity.csv`, SHA-256 `b9e6d6a2839f9c4cb2d6e9c0726a856c397e1ac505432561e6111aa1906fa4d9`

## Assumptions and evidence ceiling

All relative model levels and constraint-removal scenarios are
`ASSUMED_FOR_EXECUTION`. They diagnose scale and mathematical suppression;
they are not fitted observation operators, calibrated parameters,
physiological bounds, process replacements, or production recommendations.
PhenoCam GCC is not treated as GSI, LAI, biomass, or canopy cover. Order 7
remains held.

## Limitations

Complete-year extrema use retrospective information and cannot serve as a predictive operator. No timing tolerance or preferred level is selected.

## Accessibility

Events use distinct colors and marker shapes. The left panel retains unmatched rows through matched fractions; the right panel omits levels with no matched residual rather than imputing values.
