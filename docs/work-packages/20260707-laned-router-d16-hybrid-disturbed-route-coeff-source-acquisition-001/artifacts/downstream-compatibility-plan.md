# Downstream Compatibility Plan

Status: EXECUTED. Evidence mode: Static.

Compatibility decisions:

- The extended lookup schema change is additive. No existing column is renamed
  or removed.
- New value columns:
  `route_skin_friction_coefficient_ko`, `route_form_drag_coefficient`,
  `route_roughness_element_height_m`, `route_roughness_concentration`,
  `route_vegetation_drag_coefficient`.
- New provenance columns:
  `route_coeff_source_ref`, `route_coeff_authority_class`,
  `route_coeff_confidence`, `route_coeff_notes`.
- Static `extended_land_soil_lookup.csv` is populated for all active rows.
  Run-scoped extended lookup regeneration enriches every generated row through
  `enrich_route_coefficient_row(...)`.
- Existing base lookup copies remain valid. The new route columns are only
  guaranteed after extended lookup generation/migration.
- Legacy WEPP output is unchanged by default. Native output requires explicit
  use of `Management.as_openwepp_native_cropland(...)` or
  `Disturbed.build_openwepp_native_management(...)`.
- Native smoke artifact written in this package:
  `artifacts/generated-native-smoke/p1.man`.
- openWEPP fixture written for downstream parse/projection proof:
  `tests/fixtures/disturbed_native_route_coefficients/p1.man`.
- Missing or invalid route coefficients fail closed through validation; no
  fallback row, H2637 recipe, or legacy-field bridge is used.
- Rollback is additive: remove the route coefficient module, CSV columns,
  extended lookup enrichment call, and native output helper. Legacy output does
  not need migration because it remains opt-in isolated.
