# Contract Test Implementation Evidence

Status: completed
Evidence mode: ran

Static: HPHYS0278 contract tests were added to
`tests/integration/sim_contract_boundary_unit_registry.rs`.

Ran:

- Pre-implementation red gate:
  `cargo test --test sim_contract_boundary_unit_registry hphys0278_output_unit_registry_covers_output_schema_unit_metadata -- --nocapture`
  failed before production edits because `OutputUnitRegistry`,
  `watershed_interchange_schemas`, and fallible `hillslope_wat_schema` did not
  exist.
- Final:
  `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`
  passed, 13 tests.

Contract checks added:

- Output schema `units` metadata must resolve through `OutputUnitRegistry`.
- Output schema unit labels must match registry labels.
- Boundary-backed output entries reject stale boundary-unit mismatches.
- Publication-only output entries reject missing rationale.
- Dynamic row-level `value` columns require `unit_source = "units"` metadata,
  a sibling `units` column, and output-registry coverage.
- Publication-only output entries reject missing contract/invariant authority.
