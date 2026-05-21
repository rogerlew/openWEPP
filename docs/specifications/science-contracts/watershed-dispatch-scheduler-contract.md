# Watershed Dispatch Scheduler Contract

Status: Draft (ARCH06)
Evidence: Static
Ran evidence: none

## Purpose

Specify the deterministic watershed dispatch scheduler contract and typed
status/diagnostic surfaces used by `openwepp-watershed-orchestrator`.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`.

## Contract Interface

Inputs:
- `TopologyGraph`
- `TopologyValidationReport`

Outputs:
- `WatershedDispatchReport`
  - `precondition_status: SimulationStatus`
  - `dispatch_status: SimulationStatus`
  - `steps: Vec<DispatchStep>`
  - `diagnostics: Vec<DispatchDiagnostic>`

## Status Phase and Classification Policy

Precondition status:
- inherited from ARCH04 topology validation gate (`pre_execution_validation`)

Dispatch status:
- phase: `watershed_kernel`
- nominal:
  - `message_id = WATERSHED-DISPATCH-OK-001`
  - `boundary_class = OK`
- failure:
  - `boundary_class = TOPOLOGY_INVALID`
  - typed failure message id depends on failure class

No silent fallback is permitted for precondition, dependency, or cycle failures.

## Deterministic Ordering Contract

Dispatch order is deterministic and explicit:

1. Scheduler node set is all channel and impoundment nodes.
2. Dependency edges are non-zero channel/impoundment contributors.
3. Hillslope contributors are tracked metadata, not ordering edges.
4. Topological execution uses stable ordered selection by node key
   (`kind`, `id`) at every ready-set extraction.

## Failure Class Map

| failure class | diagnostic code | dispatch status message id | boundary class |
| --- | --- | --- | --- |
| topology precondition failed | `TOPOLOGY_PRECONDITION_FAILED` | `WATERSHED-DISPATCH-E-PRECONDITION-TOPOLOGY` | `TOPOLOGY_INVALID` |
| dependency references missing node | `MISSING_DEPENDENCY` | `WATERSHED-DISPATCH-E-MISSING-DEPENDENCY` | `TOPOLOGY_INVALID` |
| unresolved cycle in dependency graph | `DEPENDENCY_CYCLE_DETECTED` | `WATERSHED-DISPATCH-E-CYCLE-DETECTED` | `TOPOLOGY_INVALID` |

## Invariants

- `INV-WDS-001`: If topology precondition fails, scheduler emits zero dispatch
  steps.
- `INV-WDS-002`: For valid topology and acyclic dependencies, every scheduler
  node appears exactly once in dispatch steps.
- `INV-WDS-003`: For each step, all dependency nodes must have strictly smaller
  sequence indices.
- `INV-WDS-004`: Re-running scheduler on identical topology yields identical
  dispatch node order.

## Integration with ARCH03 and ARCH04

- ARCH03: Uses `SimulationStatus` taxonomy for all scheduler outcomes.
- ARCH04: Treats `validate_pre_execution_topology` output as hard precondition
  authority.

## ARCH06 Test Linkage

Crate-local tests cover:
- deterministic dependency order (`INV-WDS-002`, `INV-WDS-003`, `INV-WDS-004`)
- precondition hard-gate behavior (`INV-WDS-001`)
- cycle and missing-dependency typed failure classification
