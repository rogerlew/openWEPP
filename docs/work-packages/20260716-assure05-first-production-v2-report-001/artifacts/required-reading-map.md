# ASSURE-05 Required Reading Map

## Authority And Maintenance Responsibility

This is a living control artifact. Scope changes must update paths, tiers,
triggers, budget, and last-verified time before affected edits.

## Reading Budget

- local_required_bytes_total: 453497
- threshold_outcome: WARN
- measurement_method: `wc -c` over every Core path listed below
- measured_at_utc: 2026-07-16T10:25:51Z

Thresholds are defined in
`docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger | Read timing |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | Core | Root governance | Always | Pre-edit |
| `docs/codex_exec_plans.md` | Core | ExecPlan contract | Always | Pre-edit |
| `docs/work-packages/AGENTS.md` | Core | Package, review, conservation, and CRAP gates | Always | Pre-edit |
| `docs/work-packages/README.md` | Core | Package process and active queue context | Always | Pre-edit |
| `docs/work-packages/20260716-assure05-first-production-v2-report-001/package.md` | Core | Scope, phases, write set, and acceptance | Always | Pre-edit |
| `docs/ROADMAP.md` | Core | Authorized ASSURE-05 outcome | Always | Pre-edit |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | Core | Detailed prospective ASSURE-05 contract | Always | Pre-edit |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | Core | Accepted communication and approval boundary | Always | Pre-edit |
| `docs/standards/scientific-model-evaluation-report.md` | Core | Manuscript and publication standard | Always | Pre-edit |
| `docs/governance/scientific-assurance-v2-architecture.md` | Core | Reader, ownership, and record separation | Always | Pre-edit |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Core | Lifecycle and human approval authority | Always | Pre-edit |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Core | Identity, dependency, build, and review locks | Always | Pre-edit |
| `assurance/v2/README.md` | Core | Implemented v2 mechanics and boundaries | Always | Pre-edit |
| `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md` | On-demand | Groundwater formulation authority | Scientific protocol/manuscript | Phase 1-3 |
| `usersum/hillslope-hydrology-and-sediment-physics.md` | On-demand | Model rationale and public cross-reference | Manuscript and conditional publication | Phase 3/6 |
| `assurance/v2/reports/linear-groundwater-reservoir-recurrence/**` | On-demand | Existing architecture fixture to replace | Evidence/source revision | Phase 1-3 |
| `docs/work-packages/20260713-integrated-validation-campaign-001/**` | On-demand | Prior H2637 provenance and method | H2637 protocol only | Phase 1-2 |
| `crates/AGENTS.md` | Conditional | Rust rules | Rust edit triggered by amended package | Before edit |
| `tests/AGENTS.md` | Core | Test rules for the authorized reproduction/consumer amendment | ASSURE-05 test surface | Before edit |
| `docs/standards/prompt-wording-guidance.md` | Conditional | Delegation/prompt requirements | Prompt authoring | Pre-scaffold |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| 2026-07-16 | Codex | Initialized map for the user-authorized ASSURE-05 package. |
| 2026-07-16 | Codex | Measured 446928 Core bytes (`WARN`) and confirmed the root plus work-package instruction chain for every declared path. |
| 2026-07-16 | Codex | Promoted `tests/AGENTS.md` to Core after the preregistered test-surface amendment; refreshed budget to 453497 bytes (`WARN`). |
