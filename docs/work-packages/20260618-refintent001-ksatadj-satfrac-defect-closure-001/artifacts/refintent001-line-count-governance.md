# REFINTENT001 Line-Count Governance

Evidence class: Ran

Final touched-file counts:

| File | Lines | Status |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs` | 677 | below WARN band |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 167 | below WARN band |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | 2164 | inherited WARN band, below hard threshold |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 2453 | inherited WARN band, below hard threshold |
| `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs` | 1253 | below WARN band |

Command:

```bash
wc -l \
  crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs \
  crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs
```

Disposition: no touched file is over the 3000-line required-refactor threshold.
The two WARN-band files were already large support surfaces; this package added
only scoped source-intent access and hot-root registration.
