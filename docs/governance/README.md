# Governance

This directory holds governance artifacts that are normative for maintainers
but are not model-physics contracts.

## Contents

- `repository-transition-plan-2026-05-11.md` — planned repository transition
  sequence; documentation-only in this change, no execution.
- `openwepp-release-procedure-draft.md` — canonical draft release runbook for
  candidate assembly, release linting, and evidence expectations.
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
