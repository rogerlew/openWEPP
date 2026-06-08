# REFACTOR011 refactor011 line count governance checklist

Static:
- Baseline check was required because the source file exceeded 3000 lines before decomposition.

Ran:
- Pre-refactor baseline: `crates/openwepp-sim-contract/src/units.rs` (historical) ~3914 lines (single mixed-concern file).
- Post-refactor counts:
  - `crates/openwepp-sim-contract/src/units.rs`: 3 lines
  - `crates/openwepp-sim-contract/src/units_mod/types.rs`: 238 lines
  - `crates/openwepp-sim-contract/src/units_mod/registries.rs`: 954 lines
  - `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`: 1409 lines
  - `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`: 1320 lines
  - `crates/openwepp-sim-contract/src/units_mod/mod.rs`: 15 lines
- Files >=2000 lines after split:
  - none
- Files >=3000 lines after split:
  - none
- Decomposition rationale: monolithic unit-registry file was split by responsibility to reduce governance risk and review complexity.
- Exception owner/sunset: not applicable.
