# ASSURE-03 Required-Reading Map

Status: complete

Evidence class: Static

## Core — 88,903 Bytes (`OK`)

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository invariants and routing |
| `docs/codex_exec_plans.md` | Living ExecPlan and closure requirements |
| `docs/work-packages/AGENTS.md` | Package, delegation, gate, review, and CRAP rules |
| `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/package.md` | Active execution authority |
| `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/worker-handoff.md` | Acceptance meaning and first action |
| `docs/planning/scientific-assurance-v2-migration-plan.md` | Exact ASSURE-03 scope and gates |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | Sequence and ASSURE-04 boundary |

The total was recalculated from the seven listed files after final package and
roadmap closure edits.

## Conditional

| Trigger | Path |
| --- | --- |
| Rust compiler edits | `crates/AGENTS.md` |
| Integration-test edits | `tests/AGENTS.md` |
| Standards and prompt edits | `docs/standards/AGENTS.md`, `docs/standards/prompt-wording-guidance.md` |

All three triggers apply and the files were read before their respective edits.

## On-Demand

- ADR-0038 and the v2 architecture, lifecycle, source/build contract, and report
  standard for acceptance-state edits.
- `assurance/**`, `crates/openwepp-assurance/**`, the assurance integration test,
  `usersum/**`, release tooling, and workflow for the surfaces actually changed.
- Historical v1 work-package evidence only for search classification and exact
  recovery; the 295-kilobyte work-package catalog is not a mandatory bulk
  pre-read.
