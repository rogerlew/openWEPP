# Hillslope Phase Scheduler Contract

Status: Draft (ARCH05)
Evidence: Static
Ran evidence: none

## Purpose

Specify the typed contract for deterministic hillslope phase scheduling,
precondition enforcement, and failure classification.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`.

## Input Contract

Input:
- `TopologyValidationReport` from `openwepp-topology`
  - `status: SimulationStatus`
  - `violations: Vec<ClosureViolation>`

Executor callback:
- deterministic phase runner `FnMut(HillslopePhase) -> SimulationStatus`

## Output Contract

Output:
- `HillslopeSchedulerReport`
  - `outcome_class: SchedulerOutcomeClass`
  - `topology_precondition_status: SimulationStatus`
  - `scheduler_status: SimulationStatus`
  - `ordered_phases: Vec<HillslopePhase>`
  - `outcomes: Vec<HillslopePhaseOutcome>`
  - `precondition_violations: Vec<ClosureViolation>`
  - `halted_phase: Option<HillslopePhase>`

## Deterministic Phase Set

`HillslopePhase` members:
- `normalization`
- `storage_bounds`
- `evapotranspiration`
- `percolation_deep_seepage`
- `lateral_transfer`
- `drainage`
- `runoff_reconciliation`
- `storage_reconciliation`
- `closure_diagnostics`

Execution order is deterministic and graph-derived.

## Preconditions

Scheduler may execute hillslope phases only when:
1. topology status classification is not `Failure`, and
2. topology violation set is empty.

On precondition failure:
- `outcome_class = TopologyPreconditionFailed`
- no hillslope phases execute
- typed status is surfaced with topology-invalid semantics

## Status Rules

Per-phase status requirements:
- status is typed `SimulationStatus`
- status phase must be `hillslope_kernel`

On status-phase mismatch:
- scheduler halts
- `outcome_class = SchedulerInvariantFailure`
- scheduler emits typed failure with `boundary_class=MODE_MISMATCH`

On first phase failure classification:
- scheduler halts fail-fast
- `outcome_class = PhaseFailure`
- scheduler status is the failing phase status

On full completion with no failures:
- `outcome_class = Completed`
- scheduler emits nominal or advisory aggregate status

## Message ID Surface

Scheduler-level message IDs:
- `HSCHED-OK-001`: nominal scheduler completion
- `HSCHED-W-ADVISORY`: advisory aggregate completion
- `HSCHED-E-TOPOLOGY-PRECONDITION`: topology precondition violations present
- `HSCHED-E-GRAPH-CYCLE`: non-resolvable scheduler graph order
- `HSCHED-E-DEPENDENCY-CLOSURE`: unsatisfied dependency at execution point
- `HSCHED-E-PHASE-STATUS-PHASE`: phase status lifecycle mismatch

Phase nominal message IDs:
- `HSCHED-PHASE-OK-001` through `HSCHED-PHASE-OK-009`
  (mapped 1:1 to canonical phase order)

## No-Fallback Policy

The scheduler contract forbids silent continuation when required preconditions,
phase status semantics, or dependency closure are violated.
