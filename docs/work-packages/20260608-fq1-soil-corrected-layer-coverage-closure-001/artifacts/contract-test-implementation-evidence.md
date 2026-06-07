# Contract-Test Implementation Evidence

Evidence mode: `Static:` and `Ran:`.

Static: Added three unit regressions in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`:

- `fq1_maps_valid_parser_tail_below_legacy_normalized_cap_from_deepest_corrected_layer`
  covers a p1-shaped parser profile with depths
  `200,280,580,800,1100,2000 mm`.
- `fq1_runnable_control_mapping_within_normalized_depth_is_unchanged` covers a
  p8-shaped control profile with depths `200,430,990,1600 mm`.
- `fq1_mapping_still_fails_closed_for_nonmonotone_parser_layer` verifies invalid
  parser layer ordering still returns `CorrectedLayerMappingIncomplete`.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator fq1_ --lib
result: 3 passed; 0 failed
```

Red evidence: the pre-fix package symptom ledger and the old fixed-interval code
show the p1-shaped test would expose the same `1100..2000 mm, covered 700 mm`
coverage gap without the deepest-interval extension.
