# Architecture Spec Disposition

Status: complete
Evidence mode: Static + Ran

## Source Spec

- `docs/architecture/schedule-export-and-introspection.md`

## Accepted Requirements

Static: accepted and implemented the required Mermaid and JSON export formats from `HillslopePhaseGraph::canonical()`.

Static: implemented recommended DOT output because it was low-risk and deterministic.

Static: implemented developer-task support for doc congruence, validation diagnostics, topological-order listing, schedule diff, and phase metadata via rank plus consumer adapter.

Static: reconciled stale `docs/architecture/hillslope-phase-scheduler-graph.md` and `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md` to generated artifact authority.

Ran: generator command produced `docs/architecture/generated/hillslope-phase-schedule.{json,mmd,dot}`.

Ran: `bash tools/release/check_hillslope_schedule_export.sh` passed on congruent artifacts and failed on an intentionally drifted JSON artifact before restoration.

## Rejected Requirements

None.

## Deferred Requirements

Static: watershed dispatch scheduler export remains a declared follow-on because ARCH23 scoped hillslope export first.

## Follow-Up Requirements

Static: promote the stable interface spec into a subsystem specification only if maintainers want a separate spec home after implementation.
