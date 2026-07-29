# Event-By-Member Crossing Map

## Caption

Residual days for every frozen member and four internally bracketed Beza events. Symbols distinguish the absolute GSI 0.5 comparison from source-level-aligned retrospective model thresholds; colored crosses in the right column are unmatched rows.

## Plain-language takeaway

Changing the comparison scale recovers many crossings, especially at model levels 0.10 and 0.25, but rising 0.50 transitions remain unmatched and many recovered dates are still substantially displaced.

## Methods and source bindings

`absolute-reproduction.csv` has 148 rows keyed by `(member_or_default,event_id)`; `source-level-audit.csv` has 444 rows keyed by `(member_or_default,event_id,source_level)`. The figure plots all 592 rows for 37 members and four eligible events. Event windows use adjacent source date-50 midpoints; the first chronological same-direction crossing inside `lower < crossing <= upper` is selected. Residual is modeled fractional ordinal minus the source date at the same normalized level. An empty `residual_days` field is rendered in the explicit unmatched column rather than assigned a numeric value.

Exact result bindings:

- `absolute-reproduction.csv`, SHA-256 `fbe8d294808b7093990f3ad052cef533825ee68813788f8230fcb08f98dce9b4`
- `source-level-audit.csv`, SHA-256 `c4f685141f7ca3939f0316aae9da803e71b54b742bfaa40b458990173d599c7f`

## Assumptions and evidence ceiling

All relative model levels and constraint-removal scenarios are
`ASSUMED_FOR_EXECUTION`. They diagnose scale and mathematical suppression;
they are not fitted observation operators, calibrated parameters,
physiological bounds, process replacements, or production recommendations.
PhenoCam GCC is not treated as GSI, LAI, biomass, or canopy cover. Order 7
remains held.

## Limitations

The source level/model level analogy is retrospective and does not establish that the two quantities measure the same biological state. POWER is gridded forcing and source transitions are provisional.

## Accessibility

Operator levels use distinct marker shapes as well as colors. Every member occupies the same vertical order; unmatched results have an explicit separate column.
