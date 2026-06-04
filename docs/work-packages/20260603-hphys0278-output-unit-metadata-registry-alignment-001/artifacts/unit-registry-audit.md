# Unit Registry Audit

Status: completed
Evidence mode: static

Static: `canonical_output_unit_entries()` covers every current hillslope and
watershed output schema field with a `units` metadata key.

Coverage classes:

- Boundary-backed: hillslope WAT water-balance columns and compatible watershed
  WAT-style depth/area publication columns.
- Publication-only: watershed volumes, routing diagnostics, soil diagnostics,
  sediment/pollutant/ash summaries, and loss/class summary columns.
- Dynamic row-level: `watershed_loss_all_years_out.value` and
  `watershed_loss_average_out.value`, with units stored in the sibling `units`
  column.

Ran:

- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`:
  pass, including schema coverage, dynamic unit-source coverage, and
  deliberate mismatch rejection.
- `tools/release/check_unit_registry.sh`: pass.
