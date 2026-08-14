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

Line-count evidence: `solver.rs` 2,783; `transaction.rs` 1,544; `physics.rs`
665; `closure.rs` 484; orchestrator shadow module 475; integration test 364.
Every production source file remains below the 3,000-line package threshold.
