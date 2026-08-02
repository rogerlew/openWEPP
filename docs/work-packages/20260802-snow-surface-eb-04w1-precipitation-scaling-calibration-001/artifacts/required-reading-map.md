# Required Reading Map

Status: `WARN / progressive disclosure`

Evidence mode: **Static**.

| Path/group | Tier | Bytes | Rationale |
|---|---|---:|---|
| root `AGENTS.md` | Core | 11,927 | repository invariants |
| `docs/codex_exec_plans.md` | Core | 20,921 | living ExecPlan contract |
| `docs/work-packages/AGENTS.md` | Core | 26,013 | package and calibration governance |
| `docs/work-packages/README.md` | Core | 362,769 | catalog and campaign context |
| package-local `package.md` | Core | 11,604 | autonomous execution authority |
| ADR-0042 and testing strategy | Conditional | 27,090 | empirical calibration roles and validation |
| EB-04W package/freeze/result/tool | On demand | 388,947 | predecessor population, operators, and diagnostics |
| EB-04R execution tool | On demand | 54,216 | real runner/environment harness |
| `SC-SNOWFREEZE-001` | On demand | 498,769 | unchanged diagnostic semantics only |

Core total is `433,234 bytes`, which is `WARN` because it exceeds `400,000`
but remains below `800,000`. The large catalog is required by package
governance; mechanism-heavy predecessor and contract files remain on demand.
