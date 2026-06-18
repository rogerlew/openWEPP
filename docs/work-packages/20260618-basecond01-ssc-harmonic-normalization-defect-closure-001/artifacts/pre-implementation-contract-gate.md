# Pre-Implementation Contract Gate

Evidence class: Static + Ran

Status: complete.

Gate result: PASS.

Checks:

- Contract authority existed before production edit:
  `SC-INFILE-SOIL-001` v0.1.11 added `D-SOL-006`, `C-SOL-006`, and
  `G-SOL-015`.
- Contract-derived test existed before production edit and failed against the
  old arithmetic vertical `ssc` implementation.
- The failure was non-aliased: expected vertical `ssc_0001` differed from
  expected hourly `wb19_lateral_ssh_0001`.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator \
  runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols \
  -- --nocapture
```

Observed pre-fix result:

- Test failed at
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs:94`.
- 0 passed, 1 failed.
