# Comparator Parity Proof

Status: COMPLETE.

Static:

- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`
  now contains focused adapter parity fixtures for the typed active-frost entry.
- `r7g_typed_active_no_freeze_partition_matches_surface_adapter` compares
  `Wb11HydrologyKernel::compute_direct_frost_liquid_partition_from_typed`
  against `DirectFrostRunoffSurface::compute_frost_liquid_partition` on an
  active no-freeze / no coarse projection fixture.
- `r7g_typed_inactive_frost_partition_matches_surface_adapter_without_material`
  compares the typed entry against the adapter when `wintRed` is disabled and
  no material frost state exists.
- The remaining surface construction is isolated in
  `direct_publication_frost_comparator_surface_from_seed_surface`; production
  no longer has `direct_production_frost_comparator_surface_template`.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7g_typed_active_no_freeze_partition_matches_surface_adapter -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator r7g_typed_inactive_frost_partition_matches_surface_adapter_without_material -- --nocapture`
  passed.
- `cargo test --workspace` passed on the final tree.
