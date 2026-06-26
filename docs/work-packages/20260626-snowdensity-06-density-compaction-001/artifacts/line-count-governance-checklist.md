# Line-Count Governance Checklist

Evidence mode: Ran.

Command:

```text
wc -l crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs tools/snowfreeze_observed/physics_bulk_adjudication.py tests/integration/snowdensity06_density_compaction.rs docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
```

Results:

- `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`: 1124 lines.
- `tools/snowfreeze_observed/physics_bulk_adjudication.py`: 388 lines.
- `tests/integration/snowdensity06_density_compaction.rs`: 117 lines.
- `SC-SNOWFREEZE-001.md`: 1726 lines.

No Rust file is at or above the 2000-line warning threshold.
