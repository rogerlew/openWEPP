# Required Reading Map

Status: complete

Evidence mode: Static

## Core

| Path | Rationale |
| --- | --- |
| `AGENTS.md` | Repository invariants and kernel authority. |
| `docs/codex_exec_plans.md` | Living ExecPlan requirements. |
| `docs/work-packages/AGENTS.md` | Package, DC, review, verification, and gate governance. |
| `docs/work-packages/README.md` | Queue/catalog context. |
| `package.md` | Self-contained 21K authority envelope and phase plan. |

## Conditional — Triggered

| Path | Trigger |
| --- | --- |
| `docs/defect_closure_execplans.md` | Confirmed defect closure. |
| `docs/specifications/science-contract-authoring-procedure.md` | Canonical contract amendment. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Production kernel operand change. |
| `docs/specifications/science-contracts/index.md` | Contract lifecycle context. |
| `docs/standards/testing-and-gate-strategy.md` | Critical and fixture-impacting validation. |
| `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` | Pinned baseline provenance. |
| `crates/AGENTS.md` | Rust source/test rules. |
| `tests/AGENTS.md`; `tests/fixtures/AGENTS.md` | Derived external-authority fixture custody. |

## On-Demand

| Path | Trigger / use |
| --- | --- |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | Wet-compaction and mass-transition authority. |
| `references/copyrighted/noaa_6392_DS1.md` | Anderson compaction/melt-metamorphism source. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | Wet-compaction implementation. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/{infiltration_reconciliation,runoff_reconciliation,snow_mass_transition}.rs` | Generated melt/rain, routed handoff, and exact ledger lineage. |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_{melt,density}.rs` | Offline mirror equivalence. |
| `/workdir/wepp-forest_260430_baseline/src/{winter,snowd,melt}.for` | CoE melt/rain provenance and negative evidence for later SNOBAL compaction. |
| `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/artifacts/{integrated-audit,authority-equation-map,mechanism-matrix,disposition}.md` | Confirmed duplicate and predecessor claim boundary. |
| `docs/work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001/artifacts/{post-closure-target-feasibility-review,post-closure-target-feasibility-disposition}.md` | Defect-first roadmap trigger. |

## Budget

Local required pre-read byte count: `548607`.

Disposition: `WARN`, accepted. The required work-package catalog dominates the
pre-read budget; the contract, source, baseline, and predecessor corpus were
read on demand before their respective authority, implementation, and review
decisions.
