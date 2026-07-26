# CAL-04 Partition and Objective

Status: `EXECUTED / REVIEW FINDINGS CORRECTED`

CAL-04 calibration uses only Hubbard Brook EDI `knb-lter-hbr.51.16`.
For each site, species, and year, spring P3 brackets half leaf expansion.
Only sugar maple, yellow birch, and American beech are admitted. Fall P1 is
retained in the original source but is not scoreable because its combined
no-green/half-fallen definition has no complete native color observable.

The independent holdout is Harvard Forest HF003 v37 leaf fall for those same
three species. Raw observations bracket 50% leaf fall. Budbreak, leaves at 75%
size, and leaf color remain retained in the original source but are not
scoreable because the current native output has no direct equivalent. All 1992
fall values are excluded because the EML says no fall campaign occurred that
year, despite sparse values in the raw object.

The frozen model-to-observation operator reads `/gsi/gsi21` from the
`openwepp-canopy-research-daily-v1` trace. Current ratified CP2 makes
deciduous foliar biomass directly proportional to GSI21, so the modeled
transition is the first day with a previous/current 0.5 crossing: upward for
Hubbard P3 and downward for Harvard 50% leaf fall. Equality belongs to the
current day; no subdaily interpolation is performed. A missing crossing is a
failed member, not an omitted record. Any candidate with a missing required
crossing is invalid with objective `+infinity`; its failed-record and
failed-year counts remain retained.

The Hubbard model member is the `hubbardbrook_deciduous_nh` fixture and native
runfile `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.native.run.toml`
(SHA-256 `7938bd6fa16614230bf38b02d746ba3d8d2af9ad185bc71f78c659d49c4e1498`).
The Harvard holdout member is `harvard_deciduous_ma` and
`tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.native.run.toml`
(SHA-256 `1ddc0833a914f04153bb149b1c7535de2f7588fc53cdd66e695fa0fa60a98997`).
Each run must emit a fresh trace with site IDs `hubbard_brook` or `harvard`
and arm ID `deciduous`. The fixture produces one composite deciduous crossing
per calendar year, not species/tree/subsite members. That annual crossing is
compared to every eligible observation in that site's year to characterize
the observed stand distribution. No observation is represented as a separate
model run. Hubbard 2025 is excluded because the protected fixture ends on
2024-12-31.

For record `i`, signed interval distance `d_i` is zero inside the closed
observed interval and distance in days to the nearest bound otherwise.
Within each year, compute the mean of `d_i^2` across all eligible observations;
calibration minimizes the square root of the unweighted mean of those annual
means. Thus each year, rather than each tree/site count, has equal weight.
Report species RMSE, observation-level and year-level median absolute distance,
interval coverage fraction, and failed counts. No site, species, tree, or year
receives a fitted weight. Harvard uses the identical equal-year statistics
only after the accepted parameter range and all choices are frozen; it cannot
select parameters, bounds, weights, tolerances, stopping rules, or the
operator.

`cal04-timing-windows.csv` contains 932 Hubbard calibration intervals and 319
Harvard holdout intervals. It spans 1989-2024 and 1991-2023, respectively,
with 1992 absent from the fall holdout. Observation cadence supplies interval
censoring rather than a measurement standard error. No Gaussian error or
interpolated observation date is invented. Qualitative CAL-03 records remain
screens.
