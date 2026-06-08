# REFACTOR011 modularization plan report

Static:
- Objective: split `crates/openwepp-sim-contract/src/units.rs` into cohesive unit-registry modules without changing runtime behavior, public surface, or contract semantics.
- Mechanical seam: unit-type metadata + boundary/output catalogs + registry validation remain unchanged semantically but moved to dedicated files.

Ran:
- Decomposed registry logic into `crates/openwepp-sim-contract/src/units_mod/`:
  - `types.rs` (shared enums/structs for boundary and output metadata)
  - `boundary_catalog.rs` (canonical boundary entries + required alias list)
  - `output_catalog.rs` (canonical output entries and publication-only metadata)
  - `registries.rs` (typed registry containers, constructors, validation paths, and lookup helpers)
  - `mod.rs` (module wiring and re-exports)
- Replaced `crates/openwepp-sim-contract/src/units.rs` with a thin façade re-export:
  - `pub use crate::units_mod::*;`
  - `mod units_mod;` added in `crates/openwepp-sim-contract/src/lib.rs`.
- Updated coupling points that intentionally read boundary-registry source text:
  - `tools/release/check_sc_unit_compliance.py`
  - `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
  - `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`
- Verified split preserved all previously declared API names by re-export parity from `units_mod::mod`.
