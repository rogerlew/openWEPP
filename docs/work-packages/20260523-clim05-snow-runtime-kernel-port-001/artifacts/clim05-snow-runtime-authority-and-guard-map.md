# CLIM05 Snow Runtime Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical Authority Map

| Authority surface | CLIM05 requirement | Implementation anchor |
|---|---|---|
| `SC-CLIMATE-001` (`contract_version: 5`) | Active snow-control coupling consumes parsed `snow.options.*` controls and publishes signed `S` plus non-negative `snow.runtime_swe`. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`resolve_active_snow_coupling`, `compute_active_snow_coupling`) |
| `SC-SNOWFREEZE-001` (`contract_version: 3`, `INV-SNOWFREEZE-010`) | Enforce finite/domain-valid controls (`newsnw > 0`, `ssd > 0`, `newsnw <= ssd`), publish `S = melt - accumulation`, maintain non-negative SWE. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (lines ~1986-2115), `runtime_inputs.rs` (lines ~2095-2141) |
| `SC-RUNOFFPART-001` (`contract_version: 6`) | Runoff reconciliation uses snow-coupled liquid input `wb14_liquid_input = wb14_hyetograph_rainfall + S` under typed hard-fail guards. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (lines ~2869-2873) |
| `SC-WATBAL-001` (`contract_version: 10`, `INV-WATBAL-013`) | Storage reconciliation includes signed `S`: `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd`. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (lines ~2971-3001) |

## Runtime Surface Projection Map

| Symbol | Source | Runtime projection / mutation |
|---|---|---|
| `snow.options.rst` | parsed snow sidecar | seeded by `seed_hillslope_runtime_surface_from_snow` |
| `snow.options.newsnw` | parsed snow sidecar | seeded by `seed_hillslope_runtime_surface_from_snow` |
| `snow.options.ssd` | parsed snow sidecar | seeded by `seed_hillslope_runtime_surface_from_snow` |
| `snow.options.snow_file_present` | parsed snow sidecar presence | projected as binary activation flag (`1.0`/`0.0`) |
| `snow.runtime_swe` | runtime state surface | initialized at seam to `0.0`, updated during active coupling |
| `S` | hydrology snow-coupling output | emitted as flux writeback each runoff reconciliation (`active` and `inactive` branches) |

## Guard and Error Mapping

| Failure family | Code family | Boundary class | Implementation source |
|---|---|---|---|
| Missing required active-coupling symbol | `HKERNEL-WB14-RUNOFF-E-001` | `MissingRequiredInput` | `Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol` |
| Non-finite active-coupling symbol | `HKERNEL-WB14-RUNOFF-E-002` | `NonFinite` | `Wb11HydrologyKernelGuardError::NonFiniteStateSymbol` |
| Domain/closure violation in active coupling | `HKERNEL-WB14-RUNOFF-E-003` | `DomainViolation` | `Wb11HydrologyKernelGuardError::StateSymbolOutOfRange` |
| Runtime seam non-finite snow control | `HS-RUNTIME-E-052` | typed runtime-input error | `HillslopeRuntimeInputError::NonFiniteSnowControl` |
| Runtime seam domain-invalid snow control | `HS-RUNTIME-E-053` | typed runtime-input error | `HillslopeRuntimeInputError::SnowControlOutOfDomain` |

## Coupling Branch Posture

| Branch | Condition | Outcome |
|---|---|---|
| Inactive coupling | `snow.options.snow_file_present` absent or `0` | snow term fixed to `S=0`, runoff uses hyetograph rainfall only, no SWE mutation |
| Active coupling | `snow.options.snow_file_present = 1` | enforce typed guards, compute `accumulation`, `melt`, `S = melt - accumulation`, update SWE, propagate coupled liquid input |

## Contract/Profile Alignment Notes

- No silent defaults/clamping are applied for active-coupling missing/non-finite/out-of-domain snow controls.
- Guard-family semantics remain consistent with WB14 typed hydrology failure taxonomy.
