# Required Reading Map

The executing agent owns this living map. A stale map blocks disposition.

## Reading Budget

- local_required_bytes_total: 89,108 bytes including this package/map
- threshold_outcome: `OK`
- measurement_method: `wc -c` over initial mandatory local files
- measured_at_utc: 2026-07-13

Recompute at intake because living-document edits change the total. `OK` is at
most 400,000 bytes.

## Map

| Path | Tier | Why | Trigger/timing |
| --- | --- | --- | --- |
| `/home/workdir/openWEPP/AGENTS.md` | Core | root governance | always/pre-edit |
| `docs/codex_exec_plans.md` | Core | ExecPlan contract | always/pre-edit |
| `docs/work-packages/AGENTS.md` | Core | package gates | always/pre-edit |
| `package.md` | Core | campaign authority | always/pre-edit |
| this map | Core | reading control | always/pre-edit |
| `docs/standards/local-ci-gate-selection.md` | Core | gate/timing rules | intake |
| CQR final assessment | Core | authorization/baseline | intake |
| `tools/release/README.md` | Core | release lanes | intake |
| ADR-0017 | Core | comparator posture | intake |
| `crates/AGENTS.md` | Conditional | crate rules | before crate edits |
| `tests/AGENTS.md` | Conditional | test rules | before test edits |
| `docs/defect_closure_execplans.md` | Conditional | defect transition | reproduced defect |
| science-contract governance and kernel preparation | Conditional | authority rules | kernel/contract analysis |
| exact mechanism contract | On-demand | correctness authority | active failing lane |
| scenario package/fixture provenance | On-demand | expected behavior | active lane |
| pinned baseline source | On-demand | lineage analysis | comparator flag |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| 2026-07-13 | Codex | Initialized for integrated validation. |
| 2026-07-13 | Codex | Recomputed 89,108-byte core budget and completed intake readings. |
