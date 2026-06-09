# Required Reading Map

Status: complete

| Path | Tier | Rationale | Applicability trigger | Read status |
|---|---|---|---|---|
| `package.md` | Core | Package authority | Always | read |
| `AGENTS.md` | Core | Repository governance | Always | read |
| `docs/work-packages/AGENTS.md` | Core | Work-package execution rules | Always | read |
| `docs/work-packages/README.md` | Core | Package process/catalog | Always | read |
| `tools/owcmp/specification.md` | Core | OWCMP behavior contract | Always | read |
| `tools/legacy_comparison_suite/README.md` | Conditional | Current PL14S tool contract | Before porting behavior | read |
| `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py` | Conditional | Current semantic comparator behavior | Before implementing `wat semantic` | read |
| `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py` | Conditional | Current suite runner behavior | Before implementing `pl14s run` | read |
| `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs` | Conditional | Active contract test expectations | Before adding tests | read |
| `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | Conditional | PL14S replay invariants | Before validating policy metadata | read for PL14S replay invariants |
| `docs/specifications/subsystems/observability/legacy-observe-migration.md` | On-demand | Observe-sidecar boundary | Only if observe questions arise | read for deferred boundary |
| `docs/codex_exec_plans.md` | On-demand | ExecPlan reference | If package execution plan needs expansion | read for package/review governance; no active ExecPlan expansion needed |

## Notes

- OWCMP01 is tooling-local; no production kernel/science code was edited.
- `legacy-observe-migration.md` was read only to verify that
  `owcmp observe normalize` must remain deferred and fail closed in this
  package.
