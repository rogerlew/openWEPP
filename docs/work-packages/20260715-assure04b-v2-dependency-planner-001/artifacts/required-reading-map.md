# ASSURE-04B Required-Reading Map

Status: complete for implementation intake

Evidence class: Static

## Reading Budget

- local required bytes: 147,212, including this package
- threshold: `OK` (at most 400,000 bytes)
- method: `wc -c` over the 13 Core files; the work-package catalog is read only
  at its relevant active section rather than counted as historical authority
- measured: 2026-07-15

## Core

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Root scope, truthfulness, validation, and safety governance |
| `docs/codex_exec_plans.md` | Living autonomous ExecPlan contract |
| `docs/work-packages/AGENTS.md` | Package gates, review, CRAP, delegation, and closure |
| `docs/work-packages/20260715-assure04b-v2-dependency-planner-001/package.md` | Active scope, states, write set, and acceptance |
| `docs/ROADMAP.md` | Prospective assurance priority |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | 04B outcome and 04C/04D boundaries |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | Accepted manuscript-first/mechanical-tooling rationale |
| `docs/governance/scientific-assurance-v2-architecture.md` | Reader/product/build boundary |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Identities, dependency classes, and plan requirements |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Ownership, change impact, and publication prohibition |
| `docs/standards/scientific-model-evaluation-report.md` | Scientific argument versus machine-plan boundary |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/artifacts/worker-handoff.md` | Exact starting state and ownership split |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/artifacts/source-contract-map.md` | Implemented serialization and identity roots |

## Triggered Conditional

| Path | Trigger |
| --- | --- |
| `crates/AGENTS.md` | Rust source edits |
| `tests/AGENTS.md` | Integration-test edits |
| `docs/standards/AGENTS.md` and `docs/standards/prompt-wording-guidance.md` | Prompt authoring |
| `docs/standards/local-ci-gate-selection.md` | Focused/quick/full gate selection |

All triggered conditional files were read before corresponding actions.

## On-Demand

- `assurance/v2/**`, `crates/openwepp-assurance/**`, and the two assurance
  integration suites for exact implementation behavior.
- ASSURE-04A package/test evidence when a compatibility or admission question
  cannot be resolved from the handoff/source map.
- `docs/work-packages/README.md` relevant active/catalog sections for queue
  edits; historical entries are not implementation authority.

## Change Log

- 2026-07-15, Codex: initialized, measured below the local threshold, and
  completed Core plus triggered Conditional reading before implementation.
