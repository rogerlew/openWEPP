# CQR16 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in sim-contract
unit registry code. No public API change is authorized.

Static: preserved public types and functions include:

- `OutputUnitRegistryError`
- `BoundaryUnitRegistryError`
- `validate_output_schema_unit`
- `BoundaryUnitRegistry`
- `BoundaryUnitRegistry::new`
- `BoundaryUnitRegistry::canonical_registry`
- `BoundaryUnitRegistry::entries`
- `BoundaryUnitRegistry::entry_for_canonical`
- `BoundaryUnitRegistry::entry_for_boundary_alias`
- `BoundaryUnitRegistry::require_boundary_aliases`
- `OutputUnitRegistry`
- `OutputUnitRegistry::new`
- `OutputUnitRegistry::canonical_registry`
- `OutputUnitRegistry::entries`
- `OutputUnitRegistry::entry_for_output_column`

Static: added production items are private file-local formatter helpers:

- `format_boundary_required_field_error`
- `format_boundary_alias_conflict_error`
- `format_boundary_unit_shape_error`
- `format_boundary_lookup_error`

Static: no `Cargo.toml`, module visibility, enum variant, registry row,
schema, CLI, parser, symbol, alias, or unit-surface change was made.
