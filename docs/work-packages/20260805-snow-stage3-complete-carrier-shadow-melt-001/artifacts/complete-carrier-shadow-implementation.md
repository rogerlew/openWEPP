# Complete Carrier Shadow Implementation

Ran: 2026-08-05 focused implementation increment.

The opt-in `complete_carrier_shadow` path now computes, without mutating the
authoritative compatibility result:

- net shortwave and mandatory net longwave;
- Monin-Obukhov sensible heat;
- Monin-Obukhov latent heat and its signed vapor-mass companion;
- rain/snow precipitation-advected heat; and
- active/lower conduction added to the shadow complete-energy ledger.

Hourly precipitation totals are converted to a rate with the fixed
`3600 s` hourly boundary and held across stability substeps. This prevents
rain/snow mass and precipitation-advected heat from being integrated once per
substep instead of exactly once per hour.

Closeout review found and corrected two shadow-only arithmetic defects on
2026-08-06. Geometric snowfall is now converted to SWE with the authoritative
`0.1` factor before forming mass flux, and fusion uses contract-bound
`333,600 J kg^-1`. A focused snow-only advected-heat regression guards both the
SWE conversion and once-per-hour cadence. The corrected Snowbird result is
recorded separately.

The shadow uses the typed version-8 CLIGEN geometry. It fails closed when net
longwave is absent, geometry is invalid, precipitation lacks hydrometeor
temperature, or a typed meteorology primitive rejects its domain. The
production runner retains `complete_carrier_shadow=false`; CoE ownership and
all state/output behavior therefore remain unchanged in this increment.

Ran:

- `cargo nextest run --test snow_surface_eb03_runtime` — PASS, 19/19.
- warnings-denied Clippy for `openwepp-hillslope-orchestrator` and
  `openwepp-runner`, all targets — PASS.

The non-mutating sequential cold-content/melt shadow, retained-window
execution, and atomic CoE retirement gates remain open. Complete-carrier
diagnostics alone do not authorize cutover.
