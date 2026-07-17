# Required Reading Map

## Core

- `AGENTS.md`: repository invariants and package authorization.
- `docs/work-packages/AGENTS.md`: package evidence, review, CRAP, delegation,
  and closure requirements.
- `crates/AGENTS.md`: Rust implementation and line-count rules.
- `tests/AGENTS.md`: integration-test contracts.
- `docs/standards/AGENTS.md`: reusable documentation procedure routing.
- `docs/work-packages/20260716-assure-maint01-mechanical-amendment-workflow-001/package.md`:
  complete autonomous execution contract.
- `docs/specifications/assurance-amendment-and-identity-workflow.md`: canonical
  target behavior.
- `docs/governance/scientific-assurance-v2-source-build-contract.md`: current
  source, build, identity, and review-lock authority.
- `docs/governance/scientific-assurance-dossier-lifecycle.md`: review,
  approval, versioning, withdrawal, and release authority.
- `docs/standards/scientific-model-evaluation-report.md`: attribution,
  communication, and reviewer-binding requirements.
- `docs/standards/local-ci-gate-selection.md`: proportional and full gate
  authority.
- `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`:
  manuscript-first and human-judgment boundary.
- `assurance/v2/README.md`: current CLI and lifecycle behavior.

## Conditional

- `docs/codex_exec_plans.md`: required when updating package scope or milestones.

## On Demand

- `docs/work-packages/20260716-assurance-editorial-fast-path-001/`: existing
  transaction, timing, gate, review, and CRAP evidence.
- ASSURE-04B, ASSURE-04C, and ASSURE-04D packages: planner, staging, review-root,
  publication, and snapshot implementation provenance.
- Current report descriptors and schemas: load only the files touched by the
  active milestone.

Initial core reading total: 156,239 bytes (`OK`, at or below the canonical
400,000-byte threshold). Recalculate it at execution kickoff if core files have
changed. Keep implementation details and historical artifacts conditional or
on demand so the amendment intended to reduce context does not begin by loading
the complete assurance history.
