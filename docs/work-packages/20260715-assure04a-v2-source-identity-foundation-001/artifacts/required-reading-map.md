# ASSURE-04A Required-Reading Map

Status: complete for implementation intake

Evidence class: Static

## Reading Budget

- local required bytes: 152,057
- threshold: `OK` (at most 400,000 bytes)
- method: `wc -c` over the Core paths below
- measured: 2026-07-15

## Core

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Root governance and validation posture |
| `docs/codex_exec_plans.md` | Living autonomous ExecPlan requirements |
| `docs/work-packages/AGENTS.md` | Package, gate, review, CRAP, and delegation rules |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/package.md` | Active scope, write set, and gates |
| `docs/ROADMAP.md` | Prospective queue and A–D boundary |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | ASSURE-04A outcome and exclusions |
| `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/artifacts/worker-handoff.md` | Zero-report starting state and authorization boundary |
| `docs/governance/scientific-assurance-v2-architecture.md` | Reader/product/source boundary |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | Source identities and build boundary |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Ownership, states, and publication prohibitions |
| `docs/standards/scientific-model-evaluation-report.md` | Manuscript and claim-envelope requirements |
| `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/prototype-linear-groundwater-reservoir-evaluation.md` | Accepted positive fixture design basis |

## Conditional

| Trigger | Path |
| --- | --- |
| Rust source edits | `crates/AGENTS.md` |
| Integration-test edits | `tests/AGENTS.md` |
| Prompt authoring | `docs/standards/AGENTS.md`, `docs/standards/prompt-wording-guidance.md` |
| Local CI tier selection | `docs/standards/local-ci-gate-selection.md` |

All four listed conditions apply and are read before the corresponding work.

## On-Demand

- ADR-0038 for accepted manuscript-first rationale.
- ASSURE-02 claim matrix, current-tree confirmation, pilot decision, and review
  records for fixture fields and claim limits.
- Current `assurance/**`, `crates/openwepp-assurance/**`, assurance integration
  tests, release guards, and public zero-report surfaces for touched consumers.
- `SC-GWBASEFLOW-001` only for checking stable authority identity; this package
  does not edit or reinterpret the science contract.

## Change Log

- 2026-07-15, Codex: initialized from the canonical required-reading template.
- 2026-07-15, Codex: measured the exact Core set at 152,057 bytes and
  completed all Core and triggered Conditional reading before implementation.
