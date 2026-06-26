# Line Count Governance Checklist

Evidence class: Static.

Command:

```sh
wc -l docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  docs/work-packages/20260626-snowdensity-05d-opt-in-coe-melt-implementation-001/package.md \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs \
  crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs \
  tests/integration/snowdensity05d_opt_in_coe_melt.rs
```

Observed counts:

- `SC-SNOWFREEZE-001.md`: 1564 lines.
- 05D `package.md`: 117 lines.
- `infiltration_reconciliation.rs`: 1513 lines.
- `direct_runtime/storage.rs`: 1566 lines.
- `direct_runtime/00_core_frames.rs`: 2379 lines.
- runner direct publication builder: 3015 lines.
- `snowdensity05d_opt_in_coe_melt.rs`: 204 lines.

## Assessment

The oversized Rust files were already large shared surfaces. 05D made narrow
contracted wiring edits at existing snow melt, direct runtime, and publication
seams. A mechanical split was not performed because it would be a separate
refactor package with different risk and review surface.
