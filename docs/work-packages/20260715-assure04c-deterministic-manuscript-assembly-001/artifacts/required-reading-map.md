# ASSURE-04C Required-Reading Map

Status: complete for implementation intake

Evidence class: Static

## Reading Budget

- local required bytes: 179,725, including this package and all triggered
  Conditional files
- threshold: `OK` (at most 400,000 bytes)
- method: `wc -c` over Core plus all triggered Conditional authority; the
  work-package catalog and current implementation are read only at relevant
  sections and are On-demand
- measured: 2026-07-15

## Core

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Root scope, truthfulness, validation, and safety governance |
| `docs/codex_exec_plans.md` | Autonomous living-plan contract |
| `docs/work-packages/AGENTS.md` | Package gates, review, CRAP, delegation, and closure |
| `docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/package.md` | Active objective, write set, design boundary, and acceptance |
| `docs/ROADMAP.md` | Prospective priority and zero-public boundary |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | 04C outcome and 04D boundary |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | Accepted manuscript-first/mechanical-tooling decision |
| `docs/governance/scientific-assurance-v2-architecture.md` | Reader path, ownership, and build boundary |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Assembly, identity, result, link, and operation contract |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Staging/public separation and ownership |
| `docs/standards/scientific-model-evaluation-report.md` | Report shape, quantitative, figure, and reproduction standard |
| `docs/standards/usersum-authoring-style-guide.md` | Actual downstream Markdown/link/audience contract |
| `docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/worker-handoff.md` | Exact planner API and protected starting boundary |

## Triggered Conditional

| Path | Trigger |
| --- | --- |
| `crates/AGENTS.md` | Rust source edits |
| `tests/AGENTS.md` | Integration-test edits |
| `docs/standards/AGENTS.md` and `docs/standards/prompt-wording-guidance.md` | Prompt authoring |
| `docs/standards/local-ci-gate-selection.md` | Focused/quick/full gate selection |

All triggered conditional files are read before corresponding actions.

## On-Demand

- `assurance/v2/**`, `crates/openwepp-assurance/**`, and the assurance
  integration suites for exact implementation behavior.
- ASSURE-04A/04B source and planner evidence when compatibility cannot be
  resolved from the handoff.
- `docs/work-packages/README.md` relevant queue/catalog section only.

## Change Log

- 2026-07-15, Codex: initialized below the local threshold and completed Core
  plus triggered Conditional reading before implementation.
