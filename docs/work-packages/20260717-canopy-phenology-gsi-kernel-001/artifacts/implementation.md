# Implementation Evidence

Evidence class: `Static` and `Ran`

The new `openwepp-plant-phenology` crate implements only the CP-GSI01 process
surface:

- generalized typed parameters and daily forcing;
- FAO-56 photoperiod from signed latitude and year-aware ordinal day;
- published temperature, VPD, and photoperiod indicators;
- instantaneous product and an exact 21-sample FIFO arithmetic mean;
- typed parameter, forcing, geometry, history, and chronology errors; and
- deterministic state restoration (FIFO plus newest date) and replay.

Thirteen crate tests cover the published breakpoints/interiors; a three-constraint
product; independently reconstructed 20-, 21-, and FIFO-eviction means; an
ordinary-latitude FAO-56 anchor; opposite hemisphere phase; polar day/night;
typed failures and every error-format branch; calendar continuity and year
rollover; explicit first admission; and a public-API anchored restart that
continues bit-identically to uninterrupted state.

The crate has no dependency on the hillslope orchestrator and no production
consumer. No management, canopy, biomass, litter, snow, ET, erosion, output, or
assurance source changed. This is intentional enforcement of `GAP-PLANT-009`,
not an integration claim.
