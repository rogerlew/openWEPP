# Hillslope Phase Scheduler Contract

Status: Implemented (ARCH23 export-backed)
Evidence: Static + generated artifacts
Ran evidence: see `docs/work-packages/20260604-arch23-schedule-export-and-introspection-001/artifacts/`

## Purpose

Specify the typed contract for deterministic hillslope phase scheduling,
precondition enforcement, failure classification, and schedule-documentation
congruence.

Implementation paths:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs`

Generated schedule artifacts:

- `docs/architecture/generated/hillslope-phase-schedule.json`
- `docs/architecture/generated/hillslope-phase-schedule.mmd`
- `docs/architecture/generated/hillslope-phase-schedule.dot`

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

The authoritative phase set, phase ranks, consumer-adapter metadata,
dependency edges, and topological order are code-derived. They must be reviewed
through the generated JSON artifact:

- `docs/architecture/generated/hillslope-phase-schedule.json`

This contract intentionally does not duplicate the phase list or edge list.
Duplicating those lists here creates a second source of truth and is forbidden.

Required invariants:

- `HillslopePhaseGraph::canonical()` is the graph source of truth.
- Exported `topological_order` must match
  `HillslopePhaseGraph::canonical_order()`.
- Every exported dependency edge must render in execution/data-flow direction:
  `depends_on -> phase`.
- The release congruence gate must fail when committed generated artifacts drift
  from code.

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

Phase nominal message IDs are emitted by `HillslopePhase::ok_message_id()`.
They must not be inferred from a contiguous ordinal range.

## Export and Introspection Contract

The exporter must:

- consume `HillslopePhaseGraph::canonical()` for canonical artifacts;
- emit deterministic Mermaid, JSON, and DOT outputs;
- include rank and consumer-adapter metadata in node output;
- use `HillslopePhaseGraph::topological_order()` for cycle detection;
- report graph cycles, phases unreachable from the canonical root, and
  topological-order drift as typed exporter diagnostics;
- support JSON-to-JSON schedule diffs for added/removed nodes and edges;
- avoid writing repository files from tests by default.

The release gate is:

- `tools/release/check_hillslope_schedule_export.sh`

## No-Fallback Policy

The scheduler contract forbids silent continuation when required preconditions,
phase status semantics, dependency closure, or export congruence checks are
violated.
