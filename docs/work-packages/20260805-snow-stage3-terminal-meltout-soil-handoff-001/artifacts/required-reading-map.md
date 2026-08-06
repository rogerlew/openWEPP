# Required Reading Map

Status: reviewed; execution agent must remeasure before edits.

## Reading Budget

- local_required_bytes_total: 508833
- threshold_outcome: WARN
- measurement_method: `wc -c` over Core and triggered Conditional paths
- measured_at_utc: 2026-08-06T00:00:15Z

## Map

| Path | Tier | Why required | Trigger | Timing |
| --- | --- | --- | --- | --- |
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance | Always | Pre-edit |
| `docs/codex_exec_plans.md` | Core | ExecPlan requirements | Always | Pre-edit |
| `docs/work-packages/AGENTS.md` | Core | Package gates/reviews | Always | Pre-edit |
| `docs/work-packages/README.md` | Core | Catalog/process context | Always | Pre-edit |
| package-local `package.md` | Core | Scope and authority | Always | Pre-edit |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract workflow | Contract/kernel edit | Pre-edit |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract authoring | Contract edit | Pre-edit |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel profile | Kernel authority edit | Pre-edit |
| `docs/specifications/science-contracts/index.md` | Conditional | Contract registry | Contract edit | Pre-edit |
| `docs/standards/testing-and-gate-strategy.md` | Conditional | Gate lifecycle | Execution | Pre-edit |
| `SC-SNOWENERGY-001` | On-demand | Snow energy/melt authority | Snow phase work | Phase-local |
| `SC-SNOWFREEZE-001` | On-demand | Snow/frost/liquid authority | Liquid/frost handoff | Phase-local |
| `SC-WATBAL-001` | On-demand | Soil-water storage authority | Soil recipient | Phase-local |
| `SC-SOIL-001` | On-demand | Soil-state ownership | Receiving regime | Phase-local |
| `SC-RUNOFFPART-001` | On-demand | Infiltration-first partition | Liquid handoff | Phase-local |
| `SC-EVAP-001` | On-demand | Cover/water-limited evaporation | Receiving regime | Phase-local |
| named libsnobal sources | On-demand | Threshold precedent | Terminal solve | Phase-local |
| pinned WEPP `hr_tmp`/`tmpadj`/frost sources | On-demand | Surface/thermal provenance | Receiving regime | Phase-local |
| pinned WEPP water/runoff/evaporation sources | On-demand | Liquid/soil recipient provenance | Receiving regime | Phase-local |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| 2026-08-05 | Codex | Initialized scaffold reading map. |
| 2026-08-05 | Codex | Added coupled receiving-surface authority after independent review. |
