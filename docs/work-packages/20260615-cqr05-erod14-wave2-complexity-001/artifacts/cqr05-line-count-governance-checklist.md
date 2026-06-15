# CQR05 Line Count Governance Checklist

Evidence: Static.

Touched Rust files:

| File | Before | After | Status |
| --- | ---: | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs` | 648 | 1001 | pass |

Checklist:

- No touched `.rs` file is at or above 2000 lines.
- No new module split was performed because the package explicitly scoped the
  work to intra-module decomposition.
- The line-count increase is accepted for this CQR package because explicit
  private stage names replace one 643-line function and reduce maximum CRAP
  from `587.5911363349628` to `23.0`.
