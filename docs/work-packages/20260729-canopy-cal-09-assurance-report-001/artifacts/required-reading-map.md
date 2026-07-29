# Required Reading Map

Status: `scaffolded`

## Core

| Path | Bytes | Purpose |
| --- | ---: | --- |
| `AGENTS.md` | 11,927 | Repository governance and science authority |
| `docs/work-packages/AGENTS.md` | 26,013 | Package execution and closure |
| `docs/codex_exec_plans.md` | 20,921 | Living ExecPlan contract |
| `docs/standards/testing-and-gate-strategy.md` | 22,200 | Direct validation selection |
| `docs/standards/scientific-model-evaluation-report.md` | 13,067 | Manuscript structure and claim rules |
| `docs/governance/scientific-assurance-v2-architecture.md` | 9,562 | Report/supplement/bundle separation |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | 14,154 | Ownership, review, and publication |
| `docs/governance/scientific-assurance-v2-source-build-contract.md` | 12,520 | Identities, research objects, and builds |
| `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md` | 4,607 | Accepted communication direction |
| `docs/planning/canopy-phenology-assurance-roadmap.md` | 52,263 | Campaign questions and retained outcomes |
| `usersum/openwepp-canopy-phenology.md` | 25,563 | Durable model narrative and coefficient guide |
| `assurance/v2/README.md` | 13,346 | Implemented V2 workflow |
| `assurance/v2/schemas/report.schema.json` | 22,146 | Active descriptor contract |
| **Total** | **248,289** | **`OK` (`<=400000` bytes)** |

The package itself is also mandatory and is excluded from the fixed external
Core byte total because it remains a living plan.

## Conditional

- `docs/specifications/science-contracts/AGENTS.md` and cited `SC-*` contracts
  before binding formulation or consumer claims.
- V2 amendment, builder, schema, and integration-test sources only if execution
  prospectively amends the write set to those surfaces.
- Release and export contracts only if authenticated approval selects release
  transfer.

## On Demand

Read only the claim-bearing ledgers, strict results, procedures, figures,
sidecars, review records, and primary references needed for the active
manuscript section from CANOPY-PHENOLOGY-01 through CAL-07F.

Before execution edits, rerun:

```console
tools/agents/find-agents --for \
  docs/work-packages/20260729-canopy-cal-09-assurance-report-001 \
  assurance/v2/reports/native-forest-canopy-phenology-evaluation \
  assurance/v2/catalog.yaml assurance/v2/README.md
```

Record any changed instruction chain or byte total before proceeding.
