# Disturbed Schema Design

Status: EXECUTED. Evidence mode: Static.

Final additive route-coefficient columns:

- `route_skin_friction_coefficient_ko`
- `route_form_drag_coefficient`
- `route_roughness_element_height_m`
- `route_roughness_concentration`
- `route_vegetation_drag_coefficient`
- `route_coeff_source_ref`
- `route_coeff_authority_class`
- `route_coeff_confidence`
- `route_coeff_notes`

The extended table is the source surface. A separate `ow-1-land_soil_lookup.csv`
was not introduced because the existing extended table already owns the merged
Disturbed class, texture, soil, PMET, initial-condition, and plant field shape.

Implementation:

- `wepppy/nodb/mods/disturbed/route_coefficients.py` owns the coefficient
  matrix, provenance defaults, enrichment, and validation.
- `build_extended_land_soil_lookup()` applies `enrich_route_coefficient_row(...)`
  before writing every run-scoped extended row.
- Static `extended_land_soil_lookup.csv` carries the same columns for operator
  visibility and regression tests.
- `managements.py` owns native `ow-lanuse-1` parsing/writing. Disturbed does not
  hand-edit management text.
