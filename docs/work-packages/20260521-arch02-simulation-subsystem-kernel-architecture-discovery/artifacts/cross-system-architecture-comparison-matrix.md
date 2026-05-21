# cross-system architecture comparison matrix

Evidence: Static
Ran evidence: none

| decision axis | wepp-forest | rancor | rust exemplars | openWEPP decision |
| --- | --- | --- | --- | --- |
| state ownership model | Static: [DIRECT] mixed global common-block state + partial typed kernel structs | Static: [DIRECT] singleton + broad mutable model objects | Static: [DIRECT] typed resources/components/struct-based state separation | Static: [INFERENCE] adopt typed per-subsystem ownership; no global mutable cross-subsystem state |
| orchestration loop shape | Static: [DIRECT] monolithic procedural orchestrators (`main`/`contin`/`wshdrv`) | Static: [DIRECT] timer callback loop (`DispatcherTimer`) | Static: [DIRECT] explicit schedule graph or `step`-style pipeline | Static: [INFERENCE] implement explicit Rust orchestrator crates with deterministic phase order |
| kernel interface contract | Static: [DIRECT] newer typed kernel input/flux/status modules | Static: [DIRECT] method/property reflection surface | Static: [DIRECT] typed API surfaces with compile-time checks | Static: [INFERENCE] enforce typed kernel trait contracts + typed status IDs |
| error behavior | Static: [DIRECT] hard-stop on kernel dispatch failure in watershed kernels | Static: [DIRECT] many operations continue with runtime mutation flexibility | Static: [DIRECT] explicit result/status idioms and configurable execution controls | Static: [INFERENCE] preserve fail-stop for contract violations and non-finite/domain faults |
| topology handling | Static: [DIRECT] watershed structure checks in `wshinp` plus stop on invalid graph | Static: [DIRECT] no equivalent environmental topology contract emphasis | Static: [DIRECT] graph/schedule tooling patterns available | Static: [INFERENCE] create explicit topology graph validation phase before simulation |
| sidecar/compatibility boundary | Static: [DIRECT] sidecar files and HBP bridges are orchestration-owned | Static: [DIRECT] API/controller can mutate simulation directly | Static: [DIRECT] boundary-specific crates/features common in Rust ecosystem | Static: [INFERENCE] isolate sidecar/legacy compatibility in adapters, not kernel crates |
| parallelism semantics | Static: [DIRECT] mostly sequential legacy flow; some kernelized dispatch points | Static: [DIRECT] GUI timer loop; not deterministic scientific scheduling | Static: [DIRECT] parallel-by-default requires explicit dependency/conflict controls | Static: [INFERENCE] permit parallelism only for independent partitions with explicit deterministic reductions |
| dimensional safety | Static: [DIRECT] mixed implicit units in legacy fields | Static: [DIRECT] mixed runtime values with no dimensional typing | Static: [DIRECT] zero-cost dimensional typing (`uom`) and compile/runtime matrix dimension control (`nalgebra`) | Static: [INFERENCE] add unit-safe wrappers at kernel boundaries |
| external mutation surface | Static: [DIRECT] no general network mutation interface in core kernels | Static: [DIRECT] HTTP `setproperty` and `executemethod` direct mutation | Static: [DIRECT] idiomatic Rust favors typed command APIs | Static: [INFERENCE] allow only typed command DTOs validated against contracts |

## Synthesis
- Static: [INFERENCE] wepp-forest contributes domain sequencing and kernel migration patterns.
- Static: [INFERENCE] rancor contributes anti-pattern warnings for deterministic scientific kernels (runtime reflection mutability and singleton sprawl).
- Static: [INFERENCE] rust exemplars provide concrete patterns for schedule graphs, typed boundaries, and controlled parallel execution.
