# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-9 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row9-full-after.json`.

Extraction:

```text
jq -r '[.entries[] | select(.file | contains("/crates/openwepp-hillslope-orchestrator/src/hydrology/")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row9-full-after.json
```

Result:

- Row #5 duplicated report rows above CRAP 30: `22`.
- Row #5 unique production offenders above CRAP 30: `11`.

Offenders:

| Location | Function | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs:103` | `Wb11HydrologyKernelGuardError::code` | 24.0 | 0.00 | 600.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs:521` | `Wb11HydrologyKernel::compute_active_snow_coupling_from_typed` | 119.0 | 77.98 | 270.19 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:144` | `r7g_json_string` | 9.0 | 0.00 | 90.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs:187` | `Wb11HydrologyKernelGuardError::phase_display_parts` | 8.0 | 0.00 | 72.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs:333` | `HydrologyGuardErrorDisplayParts::fmt_with_code` | 7.0 | 0.00 | 56.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs:1058` | `Wb11HydrologyKernel::snow_density_guard_error` | 7.0 | 0.00 | 56.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs:313` | `Wb11HydrologyKernel::compute_simimpl29_melt_hour` | 25.0 | 68.68 | 44.20 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs:121` | `SnowAlbedoError::fmt` | 6.0 | 0.00 | 42.00 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs:1500` | `Wb11HydrologyKernel::legacy_tmpadj_surface_temperature_from_typed` | 36.0 | 94.90 | 36.17 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:869` | `Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed` | 7.0 | 18.92 | 33.12 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs:826` | `apply_multilayer_boundary_mass` | 12.0 | 49.23 | 30.84 |

Disposition: baseline recorded.
