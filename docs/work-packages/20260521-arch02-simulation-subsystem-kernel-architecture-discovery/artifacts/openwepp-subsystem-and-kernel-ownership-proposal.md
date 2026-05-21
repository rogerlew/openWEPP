# openWEPP subsystem and kernel ownership proposal

Evidence: Static
Ran evidence: none

## Proposed subsystem boundaries

| subsystem | ownership scope | owns mutable state | depends on | must not depend on |
| --- | --- | --- | --- | --- |
| `simulation::contracts` | canonical state and kernel contract types | no runtime mutation; type/schema authority | science contracts, ADRs | adapters, I/O |
| `simulation::inputs` | parse `.run` + core inputs + sidecar descriptors | parsed immutable scenario model | file parsers, contract schemas | kernel internals |
| `simulation::topology` | hillslope/watershed graph assembly + validation | topology graph and validation reports | parsed scenario model | kernel math details |
| `simulation::orchestrator::hillslope` | daily/hourly phase scheduling and writeback control | hillslope timestep working state | contracts, kernels, topology, inputs | sidecar file I/O primitives |
| `simulation::orchestrator::watershed` | channel/impoundment dispatch and accumulation scheduling | watershed timestep working state | contracts, routing kernels, topology | parser details |
| `simulation::kernels::hillslope` | pure process kernels (water/plant/soil phases) | none outside function-local state | contracts | file I/O, command/API layers |
| `simulation::kernels::watershed` | pure routing/impoundment kernels | none outside function-local state | contracts | parser and adapter layers |
| `simulation::adapters::legacy_bridge` | HBP + compatibility boundary | bridge buffers, compatibility status | contracts, topology, inputs | core kernel implementation details |
| `simulation::reporting` | summaries, traces, comparator payload shaping | reporting accumulators | contracts, orchestrators | parser internals |

## Proposed dependency direction
- Static: [INFERENCE] `contracts` is the root authority.
- Static: [INFERENCE] `inputs` and `topology` feed orchestrators.
- Static: [INFERENCE] orchestrators call kernels and own writeback policy.
- Static: [INFERENCE] adapters are edge-only and never imported by kernels.
- Static: [INFERENCE] reporting consumes typed outputs and status streams.

## Kernel contract shape
- Static: [INFERENCE] Each kernel signature should be `fn execute(inputs, state) -> (flux, status, next_state)` with status containing `ok`, `finite_ok`, `domain_ok`, `message_id`, and optional closure diagnostics.
- Static: [INFERENCE] Orchestrators own fallback policy; kernels never read sidecars or global state.

## Canonical symbol continuity map

| canonical symbol | proposed Rust field alias |
| --- | --- |
| `runoff` | `runoff_depth_m` |
| `runvol` | `runoff_volume_m3` |
| `sbrunf` | `subsurface_runoff_depth_m` |
| `drainq` | `tile_drain_flow_m` |
| `sep` | `deep_seepage_depth_m` |
| `st` | `layer_storage_m` |
| `frzw` | `layer_frozen_water_m` |
| `frozen` | `layer_frozen_fraction` |
| `thetdr` | `layer_theta_residual` |
| `thetfc` | `layer_theta_field_capacity` |
| `dg` | `layer_thickness_m` |
| `solthk` | `soil_profile_depth_m` |
| `peakro` | `peak_runoff_rate_m3s` |
| `watdur` | `runoff_duration_s` |

## Orchestration boundaries
- Static: [INFERENCE] `inputs` and `topology` complete before any kernel execution.
- Static: [INFERENCE] hillslope orchestrator phase order is fixed and contract-validated.
- Static: [INFERENCE] watershed orchestrator receives only typed hillslope outputs (HBP/typed shard equivalents), not parser internals.
- Static: [INFERENCE] summary accumulation is a distinct phase, not embedded in route kernels.

## Non-transferable patterns explicitly rejected
- Static: [DIRECT] Reflection-based runtime mutation endpoints in rancor (`/workdir/rancor/Rancor/WebAPI/Controllers.cs:52`, `:67`).
- Static: [DIRECT] Singleton mutable simulation state as primary authority (`/workdir/rancor/Rancor/Rancor/RancorSim.cs:100`).
- Static: [INFERENCE] These patterns are rejected for openWEPP deterministic science execution.
