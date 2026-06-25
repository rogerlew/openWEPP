# Line-Count Governance

Static:

```text
 1075 crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs
  221 tools/snowfreeze_observed/physics_bulk_snotel_profile.py
  321 tools/snowfreeze_observed/physics_bulk_adjudication.py
   90 tests/integration/snowdensity03_physics_bulk_offline_contract.rs
  110 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
   22 crates/openwepp-runner/src/hillslope/mod.rs
   45 crates/openwepp-runner/src/lib.rs
  105 docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/package.md
 1989 total
```

Assessment:

- No `.rs` file reaches the `2000` line warning threshold.
- The largest file remains the offline snowbench candidate module, below the
  warning threshold.
- No refactor is required for SNOWDENSITY-04 closure.
