# Required Reading Map

Status: complete

Evidence mode: Static.

| Path | Tier | Rationale | Applicability |
| --- | --- | --- | --- |
| `AGENTS.md` | Core | Root repository governance and work-package authorization. | Always. |
| `docs/codex_exec_plans.md` | Core | ExecPlan and openWEPP addendum requirements. | Always. |
| `docs/work-packages/AGENTS.md` | Core | Work-package gates, DC requirements, and review rules. | Always. |
| `docs/work-packages/README.md` | Core | Package catalog and current state. | Always. |
| `docs/work-packages/20260624-snowfreeze-direct-storage-reconciliation-unblock-001/package.md` | Core | Package objective, envelope, and gates. | Always. |
| `docs/defect_closure_execplans.md` | Conditional | Defect-Closure package structure and HOLD legitimacy. | This package closes fail-closed defects. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Kernel guard and physics authority rules. | Runtime storage projection touches kernel-facing direct state. |
| `crates/AGENTS.md` | Conditional | Rust crate authoring and validation gates. | Production Rust edits. |
| `tests/AGENTS.md` | Conditional | Test expectations and contract assertions. | Focused regression tests. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | On-demand | R4B storage projection implementation. | Production edit target. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs` | On-demand | R4B test helper and regression surface. | If tests land in R3C/R4B module. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs` | On-demand | R7G/R7H frost projection tests. | If tests land in frost module. |
| `tools/snowfreeze_observed/observed_harness.py` | On-demand | Site comparison command and metric-bearing report behavior. | Harness validation only. |

Required-reading budget: local files inspected during scaffolding are below the
`OK` threshold for the package-required pre-edit set. On-demand files are loaded
only as needed.
