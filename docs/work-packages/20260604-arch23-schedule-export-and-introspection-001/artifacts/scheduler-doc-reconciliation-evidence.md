# Scheduler Doc Reconciliation Evidence

Status: complete
Evidence mode: Static + Ran

## Target Docs

- `docs/architecture/hillslope-phase-scheduler-graph.md`
- `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`

## Evidence

Static: stale hand-authored 9-phase lists and predecessor edge lists were removed from both target docs.

Static: both target docs now identify generated artifacts under `docs/architecture/generated/` as the authority for phase set, rank, consumer adapter, edges, and topological order.

Static: precondition gate placement, execution halt semantics, outcome classes, status rules, message IDs, and no-fallback policy remain documented as prose.

Static: `docs/architecture/schedule-export-and-introspection.md` status was updated to `Implemented by ARCH23 on 2026-06-04` and lists implementation/gate paths.

Ran: `bash tools/release/check_hillslope_schedule_export.sh` passed after doc reconciliation and artifact generation.

## Drift Disposition

- Accepted: ARCH05 doc/contract phase-list drift was real and fixed by generated artifact authority.
- Rejected: none.
- Deferred: watershed schedule export.
- Follow-up: optional subsystem-spec promotion if maintainers want a stable spec home separate from architecture docs.
