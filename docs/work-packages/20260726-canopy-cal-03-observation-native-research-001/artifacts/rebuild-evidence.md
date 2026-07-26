# Deterministic Rebuild Evidence

Evidence class: `Ran`

The Hubbard Brook deciduous native fixture was copied to a fresh temporary
root and executed through `openwepp-cli-hill` with the CAL-03 trace selector.
The real direct-production path emitted 16,437 daily rows spanning
1980-01-01 through 2024-12-31.

`tools/canopy_phenology/cal03_research.py` validated:

- finite/bounded GSI values and chronological unique keys;
- native producer-to-growth, ET, canopy, and litter consumer identities;
- daily foliar stock-flow closure;
- daily aggregate residue closure;
- exact current/previous/old shadow-cohort reconciliation; and
- 45 calendar-year annual summaries.

Hashes are recorded in `run-manifest.json`. Rebuilding from the same trace
produced annual SHA-256
`e21a917c8e0c3c7dc37512a8c997ea0be1b55f8b0e5b12a5f8c5c2f199cd90b3`
and cohort SHA-256
`f34c3daba5910efd614b5ca38740b76e86f44c4e72ea5c9c1e66f90a185bdacd`.

The retained uncalibrated native seed has `surface_decay_factor = 1` for this
lane, so annual litter accumulates from 0.2 to 11.5917149255 kg/m2 by 2024.
No ten-year practical-equilibrium window passes. The CAL-02 years 91–100 rule
is explicitly `NOT_EVALUABLE_PERIOD_LT_100` because the protected fixture
period is 45 years; the period was not extended to manufacture that result.

This is pre-calibration characterization. The accumulating stock is not a
field verdict and was not corrected by changing litter or decay.

The required comparator executor then ran the complete nine-lane matrix from
fresh copies with release executable SHA-256
`4a73dc634112f1bad4499dcbac9a718b46309156dfc415a2f1b118c19e3fe2fc`.
All nine CLIs passed. All seven forest lanes emitted and validated 16,437 daily
rows plus 45 annual/cohort rows. The Marcell and Harvard open controls passed
without a canopy trace, correctly classified `NOT_APPLICABLE` because their
source management remains native cropland. Exact per-lane identities and
diagnostics are retained in `matrix-results.csv`.

Only the evergreen Marcell conifer seed meets the ten-year numerical
equilibrium screen (first passing window ends in 1989) because its categorical
seed has no seasonal litter transfer. All deciduous and mixed seeds accumulate
residue and report `NOT_FOUND`. This ordering is characterization of the
uncalibrated seed operands, not calibration success.
