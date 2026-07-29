# Owned-file Manifest

Status: `complete for external-boundary implementation`

Evidence class: `Static`

Owned implementation changes are confined to:

- this package, the work-package catalog, and canopy roadmap;
- `SC-PLANT-001`, `SC-RESIDUE-001`, and the science-contract index;
- root Cargo test registration/lock;
- management schema and input-contract forest data;
- management-to-runtime forcing projection;
- runner daily source projection, trace publication, and focused guards/tests;
- `tests/integration/canopy_litter_external_boundary_contract.rs`.

The terminal review must ignore unrelated pre-existing CAL-04B/CAL-05 and
gate-planner worktree changes. No production fixture is persisted; the runtime
test creates authenticated sidecars in an isolated temporary run directory.
