# Required Reading Map

Status: complete for Phase-1 HOLD execution

## Reading Budget

- local_required_bytes_total: 533249
- threshold_outcome: WARN
- measurement_method: `wc -c` over Core and triggered Conditional paths
- measured_at_utc: 2026-08-06T00:29:03Z

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
| `docs/specifications/correctness-authority-model.md` | Conditional | HOLD legitimacy and A0 admission routes | Verifier finding | Closeout |
| `docs/decisions/0024-reference-implementation-intent-authority.md` | Conditional | Source-intent admission test | Verifier finding | Closeout |
| `docs/decisions/0028-observed-data-admission-authority.md` | Conditional | Observed-data admission test | Verifier finding | Closeout |
| `SC-SNOWENERGY-001` | On-demand | Snow energy/melt authority | Snow phase work | Phase-local |
| `SC-SNOWFREEZE-001` | On-demand | Snow/frost/liquid authority | Liquid/frost handoff | Phase-local |
| `SC-WATBAL-001` | On-demand | Soil-water storage authority | Soil recipient | Phase-local |
| `SC-SOIL-001` | On-demand | Soil-state ownership | Receiving regime | Phase-local |
| `SC-RUNOFFPART-001` | On-demand | Infiltration-first partition | Liquid handoff | Phase-local |
| `SC-EVAP-001` | On-demand | Cover/water-limited evaporation | Receiving regime | Phase-local |
| named libsnobal sources | On-demand | Threshold precedent | Terminal solve | Phase-local |
| pinned WEPP `hr_tmp`/`tmpadj`/frost sources | On-demand | Surface/thermal provenance | Receiving regime | Phase-local |
| pinned WEPP water/runoff/evaporation sources | On-demand | Liquid/soil recipient provenance | Receiving regime | Phase-local |
| predecessor package and seasonal evaluation | On-demand | Current shadow limitations and terminal evidence | Phase-1 audit | Phase-local |
| prior frost Qwet/wet-heat disposition | On-demand | Determine whether a live receiving heat term exists | Phase-1 audit | Phase-local |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| 2026-08-05 | Codex | Initialized scaffold reading map. |
| 2026-08-05 | Codex | Added coupled receiving-surface authority after independent review. |
| 2026-08-06 | Codex | Remeasured at execution and completed the Phase-1 authority/source audit. |
| 2026-08-06 | Codex | Added and remeasured the authority-admission routes required by terminal verification. |
