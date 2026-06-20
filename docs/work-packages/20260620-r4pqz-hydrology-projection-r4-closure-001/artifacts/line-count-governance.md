# Line-Count Governance

Status: passed with WARN disposition.

Static: R4P/Q/Z will add `direct_runtime/projection.rs` and
`tests/tests_mod/direct_runtime_r4pqz.rs` rather than expanding the existing
WARN-band aggregate test module.

Baseline:

```text
1849 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1636 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
1095 crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs
2003 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 447 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4n.rs
1260 docs/work-packages/README.md
 352 docs/ROADMAP.md
9582 total
```

Final:

```text
1870 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
 803 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
1636 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
1095 crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs
 520 crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs
2013 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 403 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs
 658 crates/openwepp-runner/src/hillslope/03_tests.rs
9938 total
```

Disposition:

- Production direct-runtime files remain below the 2000-line WARN band.
- `direct_runtime/projection.rs` keeps the R4P/Q/Z implementation isolated at
  520 lines.
- `tests/tests_mod/direct_runtime.rs` is in the WARN band at 2013 lines but
  below the 3000-line blocker. This package touched it only to add aggregate
  R4P/Q/Z counter constants and source-scan coverage. New projection-specific
  fixture coverage lives in `direct_runtime_r4pqz.rs` to avoid growing the
  aggregate file further.
- No touched file is at or above the 3000-line blocker threshold.
