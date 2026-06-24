# Deletion Proof

Status: COMPLETE.

Static:
- Deleted file:
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/03_frost_comparator_seam.rs`
- Removed/de-exported production symbols:
  - `DirectFrostRunoffSurface`
  - `DirectFrostLiquidPartition`
  - `compute_direct_frost_liquid_partition`
  - `compute_frost_liquid_partition`
- Production source scan:

```sh
rg -n "DirectFrostRunoffSurface|DirectFrostLiquidPartition|frost_runoff_surface|frost_liquid_partition|compute_direct_frost_liquid_partition|compute_frost_liquid_partition" \
  crates/openwepp-hillslope-orchestrator/src crates/openwepp-runner/src \
  -g '*.rs' -g '!03_tests.rs' -g '!tests/**' -g '!tests_mod/**'
```

Result: no matches; `rg` exited `1` because the deleted bridge symbols are
absent from production source.

Remaining references are intentional test-only source-scan strings in
`crates/openwepp-runner/src/hillslope/03_tests.rs`.
