# Comparator Seam Proof

Status: complete.

Evidence mode: Static + Ran.

Static:

- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/03_frost_comparator_seam.rs`
  is the named seam for remaining `DirectFrostRunoffSurface::from_surface_maps`
  construction.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  calls `direct_publication_frost_comparator_surface_from_seed_surface` and
  `direct_production_frost_comparator_surface_template` instead of constructing
  `DirectFrostRunoffSurface` directly.

Ran:

- `rg -n "DirectFrostRunoffSurface::from_surface_maps|lane\.frost_runtime_carry" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs || true`
  returned no matches.
- `rg -n "DirectFrostRunoffSurface::from_surface_maps" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers`
  returned only `03_frost_comparator_seam.rs`.
- `cargo test -p openwepp-runner r7g_direct_production_reads_winter_column_frost_and_isolates_comparator_seam -- --nocapture`
  passed.

Follow-up blocker:

- The production path still uses the comparator seam to produce
  `DirectFrostLiquidPartition`. Typed frost solver extraction is required
  before `DirectFrostRunoffSurface` bridge deletion, output parity closure,
  default activation, or R7G closure can be claimed.
