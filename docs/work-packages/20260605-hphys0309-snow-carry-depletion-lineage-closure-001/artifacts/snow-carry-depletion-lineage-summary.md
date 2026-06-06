# Snow Carry/Depletion Lineage Summary

Status: complete

Evidence mode: ran

## Counts

- HPHYS0308 snow-state carry/depletion rows: `58`
- Production edit authorized rows: `0`

## Route Counts

- `pre-day-carry-deficit-hold`: `45`
- `prior-day-openwepp-meltout-hold`: `13`

## Depletion Lead Evidence State

- `computed`: `56`
- `not-computable-baseline-no-same-day-zero`: `2`

## OpenWEPP Depletion Lead Hours

- `1`: `1`
- `7`: `7`
- `8`: `8`
- `9`: `27`
- `12`: `13`

## Window Route Counts

| Hillslope | Window | Route | Count |
|---|---|---|---|
| H1 | first-abs-storage-ge-10mm | pre-day-carry-deficit-hold | 1 |
| H1 | spring-2014 | pre-day-carry-deficit-hold | 8 |
| H1 | spring-2016 | pre-day-carry-deficit-hold | 2 |
| H1 | spring-2016 | prior-day-openwepp-meltout-hold | 13 |
| H7 | spring-2014 | pre-day-carry-deficit-hold | 7 |
| H7 | spring-2016 | pre-day-carry-deficit-hold | 9 |
| H39 | spring-2014 | pre-day-carry-deficit-hold | 9 |
| H39 | spring-2016 | pre-day-carry-deficit-hold | 9 |

## Interpretation

The HPHYS0308 baseline-extra melt-call keys are immediate carry-state
deficits, not branch-predicate edit authority. Most rows start the key
day with materially less openWEPP snow depth than the fixed comparator;
the remaining rows start the key day snow-free in openWEPP while the
fixed comparator still carries snow from the prior day. The package
therefore keeps production edits in `HOLD` and routes continuation to
the prior-day/day-start snowpack carry-state lineage.
