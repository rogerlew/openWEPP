# openWEPP Documentation Index

| Doc | Audience | Purpose |
|---|---|---|
| [ROADMAP.md](ROADMAP.md) | All | **Canonical** engine roadmap: forward-only planning queue (next + deferred); completed work lives in the work-packages execution log |
| [dev-guide/README.md](dev-guide/README.md) | New developers | Human onboarding guide: architecture narrative, codeflows, principles, concepts/glossary, contribution workflow |
| [architecture/README.md](architecture/README.md) | All | Runtime topology, process model, data flow |
| [specifications/README.md](specifications/README.md) | All | Science-contract authority model and source hierarchy |
| [specifications/subsystems/README.md](specifications/subsystems/README.md) | Maintainers | Canonical subsystem specification authority and promotion model |
| [specifications/subsystems/observability/README.md](specifications/subsystems/observability/README.md) | Maintainers | Observability subsystem contracts for intent, trace events, replay windows, and legacy migration |
| [specifications/wepp-input-files/README.md](specifications/wepp-input-files/README.md) | Maintainers | Canonical openWEPP-owned WEPP input-file specifications used for parser/contract work |
| [specifications/wepp-input-files/parser-contract-requirements.md](specifications/wepp-input-files/parser-contract-requirements.md) | Maintainers | Normative requirements for parser-contract data models and parse-to-simulation propagation maps |
| [specifications/wepp-input-files/input-surface-registry.md](specifications/wepp-input-files/input-surface-registry.md) | Maintainers | Canonical registry of hillslope, watershed, and sidecar parser input surfaces and dispositions |
| [specifications/wepp-input-file-parser-contract-authoring-procedure.md](specifications/wepp-input-file-parser-contract-authoring-procedure.md) | Maintainers | Required parser-contract authoring workflow (dual-agent review, disposition, verification) |
| [specifications/science-contracts/README.md](specifications/science-contracts/README.md) | Maintainers | Canonical location and registry for `SC-*` science contracts |
| [contracts/README.md](contracts/README.md) | All | Interface contracts: `.run`, HBP, parquet schemas, CLI ABI |
| [decisions/README.md](decisions/README.md) | All | Architecture decision records (ADRs) |
| [governance/README.md](governance/README.md) | Maintainers | Governance policies, transition plans, lifecycle controls |
| [governance/openwepp-verification-validation-strategy.md](governance/openwepp-verification-validation-strategy.md) | Scientific users and maintainers | Active V&V philosophy: hard software verification, nonterminal empirical corroboration, decision-owner application fitness, and public evidence dossiers |
| [governance/scientific-assurance-dossier-lifecycle.md](governance/scientific-assurance-dossier-lifecycle.md) | Maintainers and scientific reviewers | Dossier ownership, lifecycle, deterministic build, review-lock, release-snapshot, and wepppy handoff contract |
| [governance/openwepp-release-procedure-draft.md](governance/openwepp-release-procedure-draft.md) | Maintainers | Draft end-to-end release runbook for candidate assembly, gate execution, sidecar validation, and stability evidence |
| [numerics/README.md](numerics/README.md) | All | Floating-point, RNG, summation policy |
| [backlog/README.md](backlog/README.md) | Maintainers | Concept-stage ideas and promotion criteria before work-package activation |
| [../references/README.md](../references/README.md) | Maintainers | Reference corpus policy, bibliography, and vendoring layout |
| [standards/README.md](standards/README.md) | Maintainers | Rust coding, comments, and QA standards |
| [standards/scientific-assurance-dossier.md](standards/scientific-assurance-dossier.md) | Scientific authors and reviewers | Human-first evidence dossiers with separate verification and corroboration statuses, an application-context worksheet, reproducibility, and review |
| [work-packages/README.md](work-packages/README.md) | All | Dated initiative tracking convention |
| [codex_exec_plans.md](codex_exec_plans.md) | All contributors | ExecPlan authoring requirements (self-contained, living, milestone-driven autonomous specs) |
| [defect_closure_execplans.md](defect_closure_execplans.md) | All contributors | Authoring Defect-Closure ExecPlans: the diagnose-and-correct package shape and the rationale for why it replaces diagnostic-only relay packages |
| [../usersum/documentation-agent.md](../usersum/documentation-agent.md) | End users | End-user documentation agent entrypoint for CLI documentation discoverability |
| [planning/wepp-input-file-parser-survey.md](planning/wepp-input-file-parser-survey.md) | Maintainers | Survey of parser coverage in `wepppy`/`wepppyo3`/`wepp-forest` and roadmap for hillslope, watershed, and sidecar input contracts |
| [planning/openwepp-observability-subsystem-assessment.md](planning/openwepp-observability-subsystem-assessment.md) | Maintainers | Decision to replace ad-hoc `wepp_observe*` sidecar flags with a first-class observability subsystem |
| [work-packages/20260520-obs01-observability-subsystem-foundation/package.md](work-packages/20260520-obs01-observability-subsystem-foundation/package.md) | Maintainers | OBS01 scope, deliverables, and promotion targets for observability subsystem foundation |
| [work-packages/20260520-obs01-observability-subsystem-foundation/artifacts/obs01_disposition.md](work-packages/20260520-obs01-observability-subsystem-foundation/artifacts/obs01_disposition.md) | Maintainers | OBS01 artifact-to-canonical mapping and hold register |

End-user documentation lives in [/usersum](../usersum/), following the wepppy `usersum` convention so it can be vendored into wepppy's in-app documentation engine.

Root-level agent guides:
- [../README.md](../README.md) — broad audience identity and scope
- [../AGENTS.md](../AGENTS.md) — Codex coding playbook
- [../CLAUDE.md](../CLAUDE.md) — Claude Code review / debug playbook

Provenance policy:
- [decisions/0011-architecture-first-top-down-science-contracts.md](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first strategy, contract authority model, comparator tier policy.
