# Complete Carrier Shadow Implementation

Ran: 2026-08-05 focused implementation increment.

The opt-in `complete_carrier_shadow` path now computes, without mutating the
authoritative compatibility result:

- net shortwave and mandatory net longwave;
- Monin-Obukhov sensible heat;
- Monin-Obukhov latent heat and its signed vapor-mass companion;
- rain/snow precipitation-advected heat; and
- active/lower conduction added to the shadow complete-energy ledger.

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
