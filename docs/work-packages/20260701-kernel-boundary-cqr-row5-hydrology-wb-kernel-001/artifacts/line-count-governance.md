# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
```

Result:

```text
   619 crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs
   342 crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs
  1287 crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs
  1800 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs
  1967 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs
  1915 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs
  1228 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
  9158 total
```

## Disposition

Status: `PASS`.

All row #5 touched Rust files remain below the 3000-line governance threshold.
No line-count exception is used for this row.
