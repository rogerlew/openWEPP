# Calibration Forcing Authority Resolution

Status: `FROZEN PROSPECTIVE AMENDMENT / NOT EXECUTED`

Evidence class: `Static: result-blind resolution before population execution`

## Resolution

For CAL-04B's Hubbard calibration only, this prospective amendment supersedes
the older CAL-04 binding of one protected composite `p10` climate lane and one
modeled date `m_y` broadcast across all Hubbard plots. CAL-04A subsequently
admitted a checksum-bound nine-plot Daymet forcing population and explicitly
classified the protected fixture as comparison-only. CAL-04B therefore uses a
plot-specific modeled date `m_(plot,y)` and never uses `p10.cli` as calibration
forcing.

The protected Hubbard member retains its separate role in the native
production-consumer proof. This amendment does not change Harvard's sealed
composite-member holdout semantics.

## Frozen Plot-Year State and Crossing Rule

For each candidate and each of the nine canonical Hubbard plots:

1. Every year from 1989 through 2024 is a separate native `GsiState` cold
   start.
2. The state admits exactly the real CAL-04A Daymet rows yday 1 through 180 in
   order, using `tmin_c`, `derived_vpd_pa`, and the source-EML
   `latitude_deg`. No synthetic prefill, date, or cross-year/plot state carry
   is permitted.
3. Days 1 through 59 are warm-up and cannot be selected as a modeled spring
   date.
4. The first eligible crossing is the first pair ending on yday 60 through
   180 satisfying `previous < 0.5 && current >= 0.5`. The yday 59-to-60 pair
   is eligible. A crossing completed before yday 60 is ignored; if no eligible
   crossing follows, the required plot-year crossing is missing.

This boundary is chosen before any candidate result inspection. It matches
CAL-04A's admitted yday 60–180 spring forcing-support population, gives every
eligible date at least 59 real warm-up samples, and avoids inventing Daymet's
omitted December 31 in leap years. All admitted Hubbard intervals end by yday
167.

## Preserved Objective

Every admitted observation joins exactly once by
`(candidate_id, plot_id, year, record_id)` and uses its plot-specific
`m_(plot,y)`. Closed-interval distance, the within-year mean of squared
distances over all admitted records across plots/species, equal-year root mean
square, the `+infinity` missing-crossing rule, and the complete
`minimum + 1.0 day` accepted ensemble are unchanged. No plot, species, record,
or year receives a fitted weight.

A missing required `(candidate_id, plot_id, year)` crossing retains all
affected records and makes that candidate objective `+infinity`.

## Required Proof

The producer trace and identity must bind the nine plot lanes, source-EML
latitudes, annual cold starts, warm-up/eligibility windows, complete
candidate/plot/year/day cardinality, inputs, source, binary, command, and
typed failures. Both independent reconstructors must rebuild plot-keyed
crossings and exactly match the complete crossing, observation, annual,
objective, failure, and accepted-membership ledgers. Tests must reject lane
broadcast, swap, duplicate, missing, and state-carry behavior.

