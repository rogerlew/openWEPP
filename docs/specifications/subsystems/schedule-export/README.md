# Schedule Export and Introspection Subsystem (`SCHED-EXPORT`)

Canonical specification home for openWEPP hillslope phase schedule export and
introspection tooling.

## Scope

This subsystem defines:
- Deterministic export of the hillslope phase schedule DAG to Mermaid, JSON, and
  DOT, generated from `HillslopePhaseGraph::canonical()` (single source of
  truth).
- The developer-task interface: doc-congruence CI gate, validation diagnostics
  (cycles, unreachable phases), topological-order listing, schedule diff, and
  per-phase metadata.
- The generated review artifacts and the congruence gate that keep the scheduler
  documents from drifting from the engine.

## Canonical Files

- [schedule-export-and-introspection.md](schedule-export-and-introspection.md)

## Implementation (ARCH23)

- `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs`
- `crates/openwepp-hillslope-orchestrator/src/bin/openwepp_hillslope_schedule_export.rs`
- `docs/architecture/generated/hillslope-phase-schedule.{json,mmd,dot}`
- `tools/release/check_hillslope_schedule_export.sh`

## Provenance

- Pre-promotion location: `docs/architecture/schedule-export-and-introspection.md`.
- Authored by Claude Code; reviewed by Codex (2026-06-04).
- Implemented and promoted by work package
  [20260604-arch23-schedule-export-and-introspection-001](../../../work-packages/20260604-arch23-schedule-export-and-introspection-001/).
