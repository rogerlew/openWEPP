# Implementation Evidence

Status: `implemented`

Evidence class: `Static + Ran`

`snow_density_layer_has_resolved_mass` calls the named
`snow_water_equivalent_meters_to_area_mass_kg_m2` helper before comparing with
the existing
`SNOW_DENSITY_ZERO_MASS_KG_M2 = 1e-9 kg m^-2` boundary. Both multilayer density
initialization, typed mismatch replay, both downstream Stage 3 retention sites,
partial-sublimation layer removal, and Stage 3 target trimming consume this one
predicate. Partial trims preserve proportional liquid, refrozen mass, and cold
content while retaining density, temperature, and settle identity. The typed
layer SWE/thickness seams and scalar exceptions are executable registry rows.

The opt-in direct-production snow trace serializes complete before/after layer
vectors so replay evidence can reconstruct conservation independently.

The independent `SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M = 1e-9 m` residual
checks are unchanged. No tolerance, error type, selector, coefficient, or
EB-04C thermal branch changed. Review-driven write-set expansion was recorded
in `package.md`; the final downstream predicate implementation was reapplied
after that amendment, and trace/unit edits followed it.
