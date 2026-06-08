# REFACTOR011 refactor011 public api surface parity report

Static:
- No new public API was introduced for contract/runtime consumers.
- No production API contracts were intentionally changed.
- Public exports are preserved by explicit crate-level re-export from the `units_mod` module.

Ran:
- Preserved export set in `openwepp-sim-contract`:
  - `canonical_boundary_unit_entries`
  - `hphys0274_required_boundary_aliases`
  - `canonical_output_unit_entries`
  - `BoundaryUnitRegistry`, `BoundaryUnitRegistryError`
  - `OutputUnitRegistry`, `OutputUnitRegistryError`
  - `validate_output_schema_unit`
  - `BoundaryUnitEntry`, `DimensionClass`, `DomainClass`, `OutputUnitAuthority`, `OutputUnitEntry`, `TypedBoundaryRequirement`
- `crates/openwepp-sim-contract/src/units.rs` now forwards all previously public exports via `pub use crate::units_mod::*`.
- `crates/openwepp-sim-contract/src/units_mod/mod.rs` contains explicit re-export list for all boundary/output/registry symbols.
- No intentional symbol renames, removals, or `pub` visibility reductions were introduced.
