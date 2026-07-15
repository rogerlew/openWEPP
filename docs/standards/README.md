# Coding, Package, and Prompt Standards

Normative coding, package, prompt, and quality standards for openWEPP
implementation work.

| Standard | Scope | Status |
|---|---|---|
| [rust-scientific-coding-standard.md](rust-scientific-coding-standard.md) | Rust code structure, comments, naming compatibility with legacy WEPP symbols, QA gates | Active |
| [kernel-work-package-preparation.md](kernel-work-package-preparation.md) | Kernel/science work-package authoring, contract-first sequencing, conservation/output acceptance gates | Active |
| [module-test-enhancement-authoring-guide.md](module-test-enhancement-authoring-guide.md) | Module test-enhancement work-package authoring; coverage closure threshold, obligation-to-test binding, eligible-surface exclusions | Active |
| [prompt-wording-guidance.md](prompt-wording-guidance.md) | Kernel/science kickoff prompt wording, required-reading tiers, subagent and acceptance-gate wording | Active |
| [mechanical-refactor-authoring-guide.md](mechanical-refactor-authoring-guide.md) | Mechanical refactor package authoring, tool usage, split patterns, compile/test validation flow | Active |
| [code-quality-refactor-authoring-guide.md](code-quality-refactor-authoring-guide.md) | Metric-driven behavior-preserving refactors (CRAP/complexity decomposition, dead code, duplication, lint debt); numeric-equivalence guard; lint ratchet | Active |
| [local-ci-gate-selection.md](local-ci-gate-selection.md) | Local agent/maintainer gate tiering, timing diagnostics, and empirical nextest scheduling changes | Active |
| [usersum-authoring-style-guide.md](usersum-authoring-style-guide.md) | End-user documentation under `usersum/`: document shapes, audience/register, narrative structure, style rules, claims/evidence register, vendoring mechanics, pre-landing checklist | Active |
| [scientific-model-evaluation-report.md](scientific-model-evaluation-report.md) | Manuscript-first public model-evaluation reports: scientific question, methods, quantitative results, discussion, limitations, reproduction, and review | Active v2 |
| [scientific-assurance-dossier.md](scientific-assurance-dossier.md) | Retired v1 dossier notice and stable pointer to the active v2 report standard | Retired |

The v2 report standard is governed by the
[architecture](../governance/scientific-assurance-v2-architecture.md),
[lifecycle contract](../governance/scientific-assurance-dossier-lifecycle.md),
and [source/build contract](../governance/scientific-assurance-v2-source-build-contract.md).
ADR-0038 acceptance, v2 activation, and final v1 retirement occurred as one
atomic transition when the user explicitly instructed ASSURE-03 execution.

## Change control

- Standards in this directory are normative unless superseded by an accepted ADR.
- When a standard changes, update related templates/checklists in the same change.
