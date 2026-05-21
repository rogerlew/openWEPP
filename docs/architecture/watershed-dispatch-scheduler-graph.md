# Watershed Dispatch Scheduler Graph

Status: Draft (ARCH06)
Evidence: Static
Ran evidence: none

## Purpose

Define the deterministic watershed dispatch scheduler graph used by the
watershed orchestrator before kernel-boundary integration.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`.

## Ownership Boundary

- Scheduler ownership: `simulation::watershed_orchestrator`
- Upstream precondition authority: `openwepp-topology` validation gate
- Downstream boundary: watershed route/impoundment kernel invocations

This scheduler is orchestration authority for route/impoundment dispatch order.

## Preconditions

Dispatch execution is a hard-gated precondition on topology validation:

- `TopologyValidationReport::is_valid() == true` is required.
- On precondition failure, dispatch does not execute any node and returns typed
  failure status/diagnostic.

No silent fallback is allowed when preconditions fail.

## Dispatch Node Set

Scheduler nodes are the downstream topology elements:

- `channel:<id>`
- `impoundment:<id>`

`hillslope:<id>` contributors are treated as external contributors for a node,
not as scheduler nodes in the watershed dispatch graph.

## Dependency Semantics

For each downstream node:

- Non-zero `channel` contributor references create ordered dependencies on
  `channel:<id>` nodes.
- Non-zero `impoundment` contributor references create ordered dependencies on
  `impoundment:<id>` nodes.
- Non-zero `hillslope` contributors are tracked as contributor metadata but do
  not create scheduler-order dependencies.

## Deterministic Ordering Rule

Scheduling uses deterministic topological dispatch:

1. Build DAG dependencies from non-zero channel/impoundment contributors.
2. Initialize ready set with all zero-indegree dispatch nodes.
3. At each step, select the smallest key from a stable ordered set
   (`kind`, then `id`) and emit one dispatch step.
4. Decrement dependent indegrees and insert newly-ready nodes in the same
   ordered set.

Result: repeated runs over the same validated topology produce the same
dispatch order.

## Failure Classes

Scheduler emits typed failures when ordering cannot proceed:

- `TOPOLOGY_PRECONDITION_FAILED` when validation precondition fails.
- `MISSING_DEPENDENCY` when a dependency edge references a missing dispatch
  node.
- `DEPENDENCY_CYCLE_DETECTED` when topological sort cannot consume all nodes.

All failures map to `SimulationStatus` with:
- phase `watershed_kernel`
- boundary class `TOPOLOGY_INVALID`

## Step-Level Output Surface

Each deterministic dispatch step emits:

- sequence index
- node key (`channel:<id>` or `impoundment:<id>`)
- ordered dependency node list
- ordered hillslope contributor ID list
- typed step status (`SimulationStatus`)

## ARCH06 Test Linkage

Covered by crate-local tests in:
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
