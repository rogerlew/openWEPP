# Required Reading Map

Status: `complete`

## Authority And Maintenance Responsibility

The orchestrator must keep this living map accurate as scope is discovered.
Newly required readings are added immediately, and conditional or on-demand
items are promoted before they become mandatory.

## Reading Budget

- local_required_bytes_total: `475371`
- threshold_outcome: `WARN`
- measurement_method: `wc -c` over the eight exact Core paths in the kickoff prompt
- measured_at_utc: `2026-08-04T00:04:14Z`

Thresholds are defined by
`docs/standards/kernel-work-package-preparation.md`.

The `WARN` is dominated by `docs/work-packages/README.md`. It remains Core
because the current package catalog and process context are required to avoid
duplicating or contradicting an existing snow package. The total remains below
the `REQUIRES-JUSTIFICATION` threshold.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing |
|---|---|---|---|---|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance | Always | Pre-analysis |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Living ExecPlan obligations | Always | Pre-analysis |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Package lifecycle, delegation, review, and gate rules | Always | Pre-analysis |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Package process and catalog context | Always | Pre-analysis |
| `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md` | Core | Direct validation and evidence lifecycle | Always | Pre-analysis |
| `package.md` | Core | Objective, write set, questions, and claims | Always | Pre-analysis |
| predecessor `package.md` | Core | Frozen experiment and evidence roles | Always | Pre-analysis |
| predecessor `artifacts/scientific-disposition.md` | Core | Quantified symptom and evidence limits | Always | Pre-analysis |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract interpretation rules | Contract reasoning | Before triggered reasoning |
| `crates/AGENTS.md` | Conditional | Production-source interpretation rules | Rust reasoning | Before triggered reasoning |
| `docs/standards/kernel-work-package-preparation.md` | Conditional | Package amendment rules | Scope/gate amendment | Before amendment |
| `SC-SNOWFREEZE-001.md` | On-demand | Snow and freeze authority | Snow mechanism | Phase-local |
| `SC-CLIMATE-001.md` | On-demand | Hourly forcing authority | Forcing mechanism | Phase-local |
| `SC-RUNOFFPART-001.md` | On-demand | Melt consumer and routing authority | Downstream routing | Phase-local |
| `SC-WATBAL-001.md` | On-demand | Water-balance authority | Closure/publication | Phase-local |
| `references/50201000/chap3.pdf` | On-demand | Documented WEPP winter equations | Equation audit | Phase-local |
| pinned and fixed legacy source | On-demand | Provenance and implementation lineage | Legacy comparison | Phase-local |
| mechanism-specific Rust files | On-demand | Current producer/consumer implementation | Assigned investigation | Phase-local |

## Change Log

| UTC | Agent | Change |
|---|---|---|
| 2026-08-04T00:04:14Z | Codex | Initialized map for read-only audit scaffold. |
| 2026-08-04T00:25:00Z | Codex | Promoted and read both conditional AGENTS files before contract and Rust interpretation. |
