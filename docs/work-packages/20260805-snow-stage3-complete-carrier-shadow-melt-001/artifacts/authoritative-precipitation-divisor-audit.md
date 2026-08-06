# Authoritative Precipitation Divisor Audit

Evidence class: Static + Ran, 2026-08-06.

The closeout review confirmed that the shadow's original precipitation bug was
not duplicated on the authoritative snow-mass path.

- Authoritative hourly forcing validates
  `active_precipitation_m = rain_m + 0.1 * snowfall_m`.
- The authoritative path iterates the 24 hourly totals once and consumes their
  rain and snowfall amounts. It does not divide snow mass by a stability
  substep duration.
- The complete-carrier shadow alone converts each hourly amount to a flux held
  across stability substeps. Its `/3600 s` divisor is correct.
- Closeout review found that the shadow initially omitted the `0.1` geometric
  snowfall-to-SWE conversion, making snowfall advected heat tenfold. Commit
  `2d035638` corrected that defect and added a direct regression.

Ran: `cargo nextest run --test snow_surface_eb03_runtime` passed `21/21` after
the correction. A future evaluation package should add cadence-invariance
vectors across the normal, medium, and small shadow substep schedules.
