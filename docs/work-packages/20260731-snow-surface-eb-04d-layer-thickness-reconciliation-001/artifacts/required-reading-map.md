# Required Reading Map

Evidence class: `Static`

Required pre-read total: `1158378 bytes`; disposition:
`REQUIRES-JUSTIFICATION` because the triggered kernel, defect, and unit-authority
set exceeds the `800000` escalation boundary.

| Tier | Path | Rationale / trigger |
| --- | --- | --- |
| Core | `AGENTS.md` | Repository invariants. |
| Core | `docs/codex_exec_plans.md` | Living ExecPlan requirements. |
| Core | `docs/work-packages/AGENTS.md` | Package preparation and closure governance. |
| Core | `docs/work-packages/README.md` | Catalog and campaign context. |
| Core | `package.md` | Authorized correction envelope and acceptance gates. |
| Conditional, triggered | `docs/defect_closure_execplans.md` | Defect-closure execution. |
| Conditional, triggered | `docs/specifications/science-contracts/AGENTS.md` | Canonical science-authority edit. |
| Conditional, triggered | `docs/specifications/science-contract-authoring-procedure.md` | Contract-first amendment procedure. |
| Conditional, triggered | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Kernel contract profile. |
| Conditional, triggered | `docs/specifications/science-contracts/index.md` | Contract registry lifecycle update. |
| Conditional, triggered | `docs/specifications/unit-governance.md` | Named conversion, registry, and scalar-exception requirements. |
| Conditional, triggered | `docs/standards/AGENTS.md` | Standards-local instructions. |
| Conditional, triggered | `docs/standards/kernel-work-package-preparation.md` | Kernel preparation and reading-budget requirements. |
| Conditional, triggered | `docs/standards/testing-and-gate-strategy.md` | Critical terminal validation selection. |
| Conditional, triggered | `docs/standards/local-ci-gate-selection.md` | Direct quick/frost/full command selection. |
| Conditional, triggered | `docs/standards/prompt-wording-guidance.md` | Authorized review/verification wording. |
| Conditional, triggered | `crates/AGENTS.md` | Production Rust write-set rules. |
| Conditional, triggered | `tests/AGENTS.md` | Integration-test write-set rules. |
| On-demand, triggered | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | Governing process authority and `INV-SNOWENERGY-027`. |
| On-demand, triggered | EB-04B `package.md`, `coupled-dynamics-analysis.json`, and `geometry-failure-summary.csv` | Mechanism diagnosis and exact failing states. |
| On-demand, triggered | EB-04A `artifacts/diagnostic-replay.json` | Frozen fixture, selector, failure-day, and typed-snapshot authority. |
| On-demand, triggered | EB-04C `package.md` | Protected thermal-domain correction and regression boundary. |
| On-demand, triggered | `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | Lifecycle predicate, density handoff, and aggregate guard. |
| On-demand, triggered | `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs` | Typed mismatch replay. |
| On-demand, triggered | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | Real density/Stage 3 consumers and partial sublimation. |
| On-demand, triggered | `crates/openwepp-unit-boundary/src/lib.rs` | Named SWE-to-area-mass conversion. |
| On-demand, triggered | `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs` | Executable unit registry and scalar exceptions. |
| On-demand, triggered | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | Opt-in full-layer trace serialization. |
| On-demand, triggered | `tests/integration/snow_surface_eb03_contract.rs` | Contract binding coverage. |
| On-demand, triggered | `tests/integration/snow_surface_eb03_runtime.rs` | Real-consumer exact-side and state-preservation coverage. |

Applicable instruction chains were resolved with `tools/agents/find-agents`
before edits. The large catalog and JSON evidence are required by the package
playbook and exact-state reconstruction. The three heavy production files are
necessary because the same lifecycle decision occurs in density initialization,
Stage 3 partition/sublimation, and the real CLI trace consumer; reading only a
local excerpt would miss the duplicated-deletion defect found in review. The
EB-04A/EB-04B JSON evidence is necessary to bind both exact captured states.
This is the narrowest authority set that supports a defensible kernel closure.
