# Hillslope Phase Scheduler Graph

Status: Implemented (ARCH23 export-backed)
Evidence: Static + generated artifacts
Ran evidence: see `docs/work-packages/20260604-arch23-schedule-export-and-introspection-001/artifacts/`

## Purpose

Define how the deterministic hillslope phase scheduler graph is reviewed and
kept congruent with the openWEPP implementation after successful topology
pre-execution validation.

The live schedule graph is implemented in:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`

## Single Source of Truth

The canonical phase set, ranks, consumer-adapter metadata, dependency edges,
and topological order are generated from `HillslopePhaseGraph::canonical()`.
Do not maintain a parallel hand-authored phase list or edge list in this
document.

Generated review artifacts:

- `docs/architecture/generated/hillslope-phase-schedule.json`
- `docs/architecture/generated/hillslope-phase-schedule.mmd`
- `docs/architecture/generated/hillslope-phase-schedule.dot`

Export tooling, the developer-task interface (congruence gate, diff,
validation), and review dispositions are specified in the canonical subsystem
spec:
[schedule-export-and-introspection.md](../specifications/subsystems/schedule-export/schedule-export-and-introspection.md).

The JSON artifact is the text-diffable source for node rank, consumer adapter,
edge, and `topological_order` review. The Mermaid artifact is the GitHub-rendered
diagram form. The DOT artifact supports standalone graph tooling.

## Regeneration and Drift Gate

Regenerate artifacts locally with:

```bash
cargo run --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml \
  --bin openwepp_hillslope_schedule_export \
  -- generate --output-dir docs/architecture/generated
```

Check committed artifacts for congruence with code with:

```bash
bash tools/release/check_hillslope_schedule_export.sh
```

The gate regenerates into a temporary directory and compares the generated
Mermaid, JSON, and DOT outputs against committed files. It fails on drift and
does not write repository files.

## Ownership Boundary

- Scheduler ownership: `openwepp-hillslope-orchestrator`
- Upstream dependency: validated topology gate output from `openwepp-topology`
- Downstream boundary: hillslope kernel phase execution surfaces

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
- Scheduler invariant failures, including dependency closure or status-phase
  mismatch, are surfaced as typed failure statuses.

## Outcome Classes

The scheduler report classifies completion as one of:

- `Completed`
- `TopologyPreconditionFailed`
- `PhaseFailure`
- `SchedulerInvariantFailure`

This is orchestration-level control-flow metadata and does not replace per-phase
status detail.

## Follow-On Scope

Watershed dispatch scheduler export is a declared follow-on. It should use the
same generated-artifact and congruence-gate shape, but it must account for
topology-derived watershed nodes and watershed-specific diagnostics.
