# Owned File Manifest

The retained checkpoint owns:

- workspace registration in `Cargo.toml` and `Cargo.lock`;
- `crates/openwepp-land-surface-energy/**`;
- the default-off orchestrator dependency, export and
  `land_surface_energy_shadow/**`;
- narrowly scoped read-only accessors in
  `vegetation_real_hydrology_shadow.rs`;
- `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` and
  its root test registration; and
- this Child-3 package lifecycle/evidence tree.

No runner, production selector, default, publication or production scheduler
call site is in the diff.

Resumed ownership additionally includes the typed root-owner addition in
`transaction.rs`, `land_surface_energy_shadow/covered_forest.rs`, the shadow
module exports and `covered_forest_tests.rs`.

Current line-count evidence: `solver.rs` 2,802; `transaction.rs` 1,674;
`physics.rs` 665; `closure.rs` 484; orchestrator shadow `mod.rs` 2,943;
`covered_forest.rs` 158; integration root 2,757; covered tests 677. The two
2,000+ production/test files remain WARN with active decomposition through
submodules. Every file remains below the 3,000-line closure threshold.
