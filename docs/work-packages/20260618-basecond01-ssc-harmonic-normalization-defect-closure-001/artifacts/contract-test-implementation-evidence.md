# Contract-Test Implementation Evidence

Evidence class: Static + Ran

Status: complete.

Updated test:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs`
- Test:
  `runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols`

Non-aliased fixture:

- `tests/fixtures/infile/soil/valid_9002.sol`
- Source layer 1: `100 mm`, `ksat = 15.0 mm/h`, anisotropy `1.20`.
- Source layer 2: next `100 mm` of normalized layer, `ksat = 8.0 mm/h`,
  anisotropy `1.10`.

Expected top-interval surfaces:

- vertical `ssc_0001`: `15.0 mm/h` by the baseline top source-layer `ksat`
  rule;
- hourly `wb19_lateral_ssh_0001`:
  `(100*15*1.20 + 100*15*1.10) / 200 = 17.25 mm/h`;
- assertion: `ssc_0001 != wb19_lateral_ssh_0001`.

Expected below-top split-layer surfaces in the synthetic H2637-shaped test:

- source overlap: `160 mm @ 330.2755 mm/h` plus
  `40 mm @ 33.0275 mm/h`;
- source anisotropy ratios: `1.25` for the high-`ksat` layer and `0.65`
  for the low-`ksat` layer;
- vertical `ssc_0003`:
  `200 / (160/330.2755 + 40/33.0275) = 117.955408163210 mm/h`;
- hourly `wb19_lateral_ssh_0003`:
  `(160*330.2755*1.25 + 40*33.0275*0.65) / 200 =
  334.569075 mm/h`;
- assertion: `ssc_0003 != wb19_lateral_ssh_0003`.
- homogeneous-layer assertions:
  - `ssc_0002 = 330.2755 mm/h`;
  - `wb19_lateral_ssh_0002 = 330.2755*1.25 mm/h`;
  - `ssc_0004 = 33.0275 mm/h`;
  - `wb19_lateral_ssh_0004 = 33.0275*0.65 mm/h`.

Guard coverage:

- missing `ksat`: `HS-RUNTIME-E-033`;
- non-finite `ksat`: `HS-RUNTIME-E-034`;
- non-positive `ksat`: `HS-RUNTIME-E-035`.

Ran before production edit:

```text
cargo test -p openwepp-hillslope-orchestrator \
  runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols \
  -- --nocapture
```

Result before production edit:

- Failed as expected.
- Failure occurred at the exact-check assertion because production still
  emitted arithmetic vertical `ssc`.

Ran after production edit:

```text
cargo test -p openwepp-hillslope-orchestrator \
  runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols \
  -- --nocapture
```

Result after production edit:

- 1 passed, 0 failed.

Additional focused gate after review-driven anisotropy and guard coverage:

```text
cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture
```

Result:

- 79 passed, 0 failed.
