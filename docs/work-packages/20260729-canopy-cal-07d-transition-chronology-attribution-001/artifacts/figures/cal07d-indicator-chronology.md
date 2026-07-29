# GSI Indicator Chronology

## Caption

Daily BASE-ensemble median constraint indicators, instantaneous product, and 21-day GSI during 2024 and 2025. Vertical dashed markers are source date-50 transitions; the lower categorical strip shows the daily modal smallest-indicator tie set.

## Plain-language takeaway

Photoperiod is the modal limiting indicator through much of austral winter, while VPD becomes strongly suppressive near both observed green-up periods. At the date-50 rises, the median instantaneous product is near zero and the trailing GSI remains below 0.03.

## Methods and source bindings

`daily-scenario-ensemble.csv` is keyed by `(scenario,date)` and has 9,996 rows: 1,666 dates for each of five 37-member scenarios plus one single-trajectory default. This figure uses the 731 BASE rows in 2024-2025; each line value is a 37-member daily median. `event-indicator-attribution.csv` has 12 rows keyed by `(event_id,source_level)`; the four source-level 0.50 rows provide the event markers. Event-window selection and unmatched crossing encoding do not apply to these daily chronology lines; the markers are retained source dates, not modeled matches.

Exact result bindings:

- `daily-scenario-ensemble.csv`, SHA-256 `1444597741751e14cd3580817d91546ca018531534e7192c87ad66f099213594`
- `event-indicator-attribution.csv`, SHA-256 `aae4967eb43201285588d88e26cf4a21ebcedb0e95a73be87eb7efa389f77d98`

## Assumptions and evidence ceiling

All relative model levels and constraint-removal scenarios are
`ASSUMED_FOR_EXECUTION`. They diagnose scale and mathematical suppression;
they are not fitted observation operators, calibrated parameters,
physiological bounds, process replacements, or production recommendations.
PhenoCam GCC is not treated as GSI, LAI, biomass, or canopy cover. Order 7
remains held.

## Limitations

Smallest-indicator rank is mathematical, not causal. Ties are retained as categories. On-site meteorology, rainfall, soil moisture, and physiological observations are unavailable.

## Accessibility

Each quantity has a distinct line color and dash pattern. Events are labeled in text, axes are shared, and the limiting-category strip is supplemental to the numeric sidecar binding.
