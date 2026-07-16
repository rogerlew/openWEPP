# ASSURE-06 Required Reading Map

## Authority And Maintenance Responsibility

This is a living control artifact. Scope changes must update paths, tiers,
triggers, byte budget, and verification time before affected edits.

## Reading Budget

- local_required_bytes_total: 141405
- threshold_outcome: OK
- measurement_method: `wc -c` over every Core path below
- measured_at_utc: 2026-07-16T21:23:48Z

Thresholds are defined in
`docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger | Read timing |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | Core | Root governance | Always | Pre-edit |
| `docs/codex_exec_plans.md` | Core | ExecPlan contract | Always | Pre-edit |
| `docs/work-packages/AGENTS.md` | Core | Package, review, and evidence gates | Always | Pre-edit |
| `docs/ROADMAP.md` | Core | User-directed queue authority | Always | Pre-edit |
| `docs/work-packages/20260716-assure06-snow-frost-flagship-synthesis-001/package.md` | Core | Scope, phases, write set, and acceptance | Always | Pre-edit |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | Core | Accepted communication boundary | Always | Pre-edit |
| `docs/standards/scientific-model-evaluation-report.md` | Core | Manuscript and publication standard | Always | Pre-edit |
| `docs/governance/scientific-assurance-v2-architecture.md` | Core | Reader, ownership, and record separation | Always | Pre-edit |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Core | Lifecycle and human authority | Always | Pre-edit |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Core | Identity, dependency, build, and review locks | Always | Pre-edit |
| `assurance/v2/README.md` | Core | Implemented V2 mechanics and boundaries | Always | Pre-edit |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Science-contract reading rules | Snow/frost synthesis | Before scientific drafting |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | On-demand | Snow/frost formulation and evidence authority | Claim authoring | Phase 1-3 |
| `usersum/snow-frost-modeling-and-validation.md` | On-demand | Model rationale and future public cross-reference | Manuscript | Phase 1-4 |
| `tests/fixtures/snotel_observed/**` | On-demand | Admitted SNOTEL observations and provenance | Snowpack methods | Phase 1-3 |
| `tests/fixtures/precip_phase_observed/**` | On-demand | Admitted Jennings precipitation-phase data | Phase methods | Phase 1-3 |
| `tests/fixtures/snowfreeze_observed/**` | On-demand | Admitted frost and soil-temperature observations | Frozen-soil methods | Phase 1-3 |
| Named package evidence in `artifacts/evidence-inventory.md` | On-demand | Retained quantitative operands and limitations | Relevant claim | Phase 1-3 |
| `docs/standards/prompt-wording-guidance.md` | Conditional | Delegation and prompt requirements | Prompt authoring | Scaffold |
| `tests/AGENTS.md` | Conditional | Test rules | Test edit after amendment | Before edit |
| `crates/AGENTS.md` | Conditional | Rust rules | Rust edit after amendment | Before edit |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| 2026-07-16 | Codex | Initialized the map for the user-authorized ASSURE-06 package. |
| 2026-07-16 | Codex | Measured 139578 Core bytes (`OK`) and confirmed the root-only instruction chain for the declared write set. |
| 2026-07-16 | Codex | Re-measured 141405 Core bytes (`OK`) after the test-contract amendment and V2 README update. |
