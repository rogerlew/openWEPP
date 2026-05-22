# Kernel Trait Boundary and Writeback Ownership

Status: Draft (ARCH07)
Evidence: Static
Ran evidence: none

## Purpose

Define the explicit trait boundary between orchestrators and kernels, and freeze
writeback ownership semantics so kernel code remains a pure transform surface.

Implementation paths:
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

## Ownership Boundary

- Kernel ownership:
  - evaluate process math from typed request surface
  - return typed `KernelRunResponse` with status + writeback proposal
  - no direct mutation of orchestrator-owned runtime state
- Orchestrator ownership:
  - execute scheduler/dispatch order
  - evaluate writeback closure/domain validity
  - decide `accept`/`reject`
  - apply accepted updates to mutable state/flux maps

No silent fallback is permitted for rejected or non-finite writeback payloads.

## Trait Surfaces

Shared contract crate `openwepp-kernel-contract` defines:

- `HillslopeKernel::run_hillslope_phase(&HillslopeKernelRequest) -> KernelRunResponse`
- `WatershedKernel::run_watershed_node(&WatershedKernelRequest) -> KernelRunResponse`
- `KernelWritebackPayload` with typed state and flux update records
- writeback protocol helpers:
  - `evaluate_kernel_writeback(...) -> KernelWritebackDecision`
  - `apply_kernel_writeback(...) -> Result<KernelWritebackApplyResult, WritebackError>`

## Writeback Decision Protocol

Deterministic outcomes:
- `Accept`: proposal passed finite + domain checks
- `Reject`: proposal failed invariant/domain checks
- `Apply`: orchestrator committed accepted updates

Status/message-id surfaces:
- accept: `KWRITEBACK-ACCEPT-001`
- apply: `KWRITEBACK-APPLY-001`
- reject non-finite: `KWRITEBACK-E-NON-FINITE`
- reject domain violation: `KWRITEBACK-E-DOMAIN-VIOLATION`

## Orchestrator Integration

Hillslope orchestrator:
- `HillslopePhaseScheduler::execute_with_kernel(...)`
- schedules canonical ARCH05 phase order
- calls kernel per phase
- applies accepted writeback to `HillslopeWritebackSurface`
- fails fast on writeback reject or status-phase mismatch

Watershed orchestrator:
- `execute_watershed_dispatch_with_kernel(...)`
- consumes ARCH06 dispatch order
- calls kernel per dispatch step
- applies accepted writeback to `WatershedWritebackSurface`
- halts on reject or status-phase mismatch

## ARCH03/ARCH04/ARCH05/ARCH06 Linkage

- ARCH03: all decision surfaces remain typed `SimulationStatus` records.
- ARCH04: topology validation remains hard precondition authority.
- ARCH05: hillslope phase scheduler remains deterministic and orchestrator-owned.
- ARCH06: watershed dispatch scheduler remains deterministic and orchestrator-owned.

