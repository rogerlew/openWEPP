# Contract Amendment

Evidence class: `Static`

`SC-PLANT-001` revision 21 adds:

- `REF-PLANT-JOLLY-GSI` and `REF-PLANT-FAO56-DAYLIGHT`;
- `INV-PLANT-028` through `INV-PLANT-032` for indicator equations, exact
  trailing state, signed-latitude photoperiod, typed failures, scope, and
  year-aware chronological admission;
- guard-map and canonical-to-Rust alias-map rows for the complete GSI surface;
- the `CP-GSI01 Generalized Foliar-Phenology Addendum`, including units,
  defaults, algorithm, vectors, and the integration hold; and
- `GAP-PLANT-009`, which prevents a process-kernel pass from being represented
  as canopy/biomass/litter or downstream integration.

The amendment preceded the Rust implementation. Review disposition then made
the cold-start policy explicitly inferential and added restart/calendar
authority without changing the contract lifecycle.
