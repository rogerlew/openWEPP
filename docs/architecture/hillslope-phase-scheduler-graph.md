# Hillslope Phase Scheduler Graph

Status: Draft (ARCH05)
Evidence: Static
Ran evidence: none

## Purpose

Define the deterministic hillslope phase scheduler graph used by openWEPP after
successful topology pre-execution validation.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`.

## Ownership Boundary

- Scheduler ownership: `simulation::hillslope_orchestrator`
- Upstream dependency: validated topology gate output from
  `openwepp-topology`
- Downstream boundary: hillslope kernel phase execution surfaces

## Canonical Deterministic Order

The scheduler phase order is explicit and fixed:
1. `normalization`
2. `storage_bounds`
3. `evapotranspiration`
4. `percolation_deep_seepage`
5. `lateral_transfer`
6. `drainage`
7. `runoff_reconciliation`
8. `storage_reconciliation`
9. `closure_diagnostics`

No implicit ordering is allowed.

## Dependency Graph

Each phase depends on its predecessor in the canonical chain:
- `storage_bounds` <- `normalization`
- `evapotranspiration` <- `storage_bounds`
- `percolation_deep_seepage` <- `evapotranspiration`
- `lateral_transfer` <- `percolation_deep_seepage`
- `drainage` <- `lateral_transfer`
- `runoff_reconciliation` <- `drainage`
- `storage_reconciliation` <- `runoff_reconciliation`
- `closure_diagnostics` <- `storage_reconciliation`

The scheduler resolves execution by topological order with deterministic tie
breaking on canonical phase rank.

## Precondition Gate Placement

Topology validation is a hard precondition:
- If topology status classification is `Failure`, hillslope phase execution is
  blocked.
- If topology violations are non-empty, hillslope phase execution is blocked.
- No silent fallback/default execution path is permitted.

## Execution and Halt Semantics

- Phase statuses must be typed `SimulationStatus` records.
- Phase statuses must declare `phase=hillslope_kernel`.
- Execution is fail-fast on the first failure classification.
- Scheduler invariant failures (dependency closure or status-phase mismatch)
  are surfaced as typed failure statuses.

## Outcome Classes

The scheduler report classifies completion as one of:
- `Completed`
- `TopologyPreconditionFailed`
- `PhaseFailure`
- `SchedulerInvariantFailure`

This is orchestration-level control-flow metadata and does not replace per-phase
status detail.
