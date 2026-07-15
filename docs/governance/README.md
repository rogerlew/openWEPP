# Governance

This directory holds governance artifacts that are normative for maintainers
but are not model-physics contracts.

## Contents

- `repository-transition-plan-2026-05-11.md` — planned repository transition
  sequence; documentation-only in this change, no execution.
- `openwepp-release-procedure-draft.md` — draft release runbook with separate
  validation and explicit release-assembly routes plus zero-report assurance
  preflight.
- `openwepp-verification-validation-strategy.md` — active scientist-facing V&V
  philosophy and adoption strategy for hard software verification, nonterminal
  empirical corroboration, decision-owner application fitness, and manuscript-
  first scientific model-evaluation reports.
- `scientific-assurance-v2-architecture.md` — public report, technical
  supplement, public research objects, internal machine bundle, model
  narrative, and application-assessment boundaries.
- `scientific-assurance-v2-source-build-contract.md` — canonical source,
  stable identity, dependency, deterministic build, agent-assistance, review-
  lock, and snapshot contract.
- `scientific-assurance-dossier-lifecycle.md` — v2 report ownership,
  staging-only draft/review lifecycle, approval, publication, supersession, and
  release-transfer contract; legacy filename retained as a stable link.
- `legacy-source-attribution-and-contributors-policy.md` — required attribution
  and contributor-governance metadata for Rust files that port legacy WEPP
  source units.
- `reference-vendoring-policy.md` — reference-corpus storage and rights policy
  (`vendorable/` tracked, restricted files in gitignored local cache).
- `unsafe-and-interop-restrictions-policy.md` — hard restrictions for `unsafe`
  Rust and foreign-language interoperability boundaries.

## Rules

- Governance plans in this directory are decision records and execution
  checklists; they do not imply that corresponding git/GitHub actions were run.
- Any document with external repo operations must include explicit status
  (`planned`, `in_progress`, `completed`) and an execution note.
