# CLI04 Crate Rename And Shared Boundary Plan

Status: completed (Phase A)
Evidence mode: Static

## Static

### Decision
- Adopt `crates/openwepp-output/` as the shared output boundary target for
  hillslope + watershed output families.
- Treat existing `crates/openwepp-hillslope-output/` as CLI03 predecessor
  path valid only during explicit transition phases.

### Why This Decision
- CLI04 scope includes both hillslope and watershed output-family ownership;
  crate naming and module ownership must match that cross-surface authority.
- WAT parity work requires a single typed boundary for output contracts,
  schema metadata validation, and checksum/manifest helper behavior.

### Transition Plan (Phase-Scoped)
1. Phase A (this artifact): ratify contract/spec authority and shared-boundary
   target naming.
2. Phase B: add contract-derived tests that enforce:
   - WAT field metadata parity (`units`, `description`),
   - WAT dataset metadata parity
     (`dataset_version`, `dataset_version_major`,
     `dataset_version_minor`, `schema_version`),
   - dependency posture checks rejecting new `arrow2` adoption.
3. Phase C: implement production migration:
   - create/rename crate path to `crates/openwepp-output/`,
   - rewire workspace and runner dependencies,
   - port/implement WAT writer on required `arrow-rs` stack
     (`parquet`, `arrow-array`, `arrow-schema`),
   - keep typed guard behavior fail-closed (no silent defaults/fallback files).
4. Phase D/E: execute verification gates and disposition evidence updates.

### Planned Write Surfaces For Rename Implementation (Phase C)
- `Cargo.toml` (workspace membership and crate dependency rewiring)
- `crates/openwepp-hillslope-output/**` and/or successor `crates/openwepp-output/**`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/lib.rs`
- affected integration tests under `tests/integration/**`

### Guarded Completion Criteria
- Shared-boundary crate target is fully wired and referenced by runner/output
  code paths.
- WAT parity tests pass against emitted parquet files.
- `arrow-rs` dependency posture is satisfied and new `arrow2` adoption is absent.
- Required gates (`cargo fmt --check`, clippy, tests, deny) recorded in package
  verification artifacts before disposition.

## Ran
- Not run (Phase A planning artifact only).
