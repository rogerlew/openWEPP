# Phase 1 Framework Operator Sign-Off Gate

Evidence mode: Static
Status: accepted

## Completed Phase 1 outputs

- Slimmed `docs/specifications/science-contract-authoring-procedure.md` in place. The path is unchanged.
- Added `docs/specifications/science-contract-spec.md` for canonical artifact schema and Binding Exposure Index requirements.
- Added `docs/specifications/science-contract-provenance-spec.md` for sidecar format, lifecycle, retention, and non-binding history rules.
- Recorded Phase 0 inventory and SC-WATBAL addendum classification in `artifacts/phase0-watbal-addendum-classification.md`.
- Dispositioned Phase 1 review findings in `artifacts/disposition_phase1_review.md`; Phase 0 is now explicitly a mechanical first-cut inventory and Phase 2 relocation requires semantic re-adjudication.

## Normative authority change

This gate changes the science-contract authoring authority model by splitting workflow, artifact schema, and provenance-sidecar lifecycle into separate normative documents. Phase 2 must not begin until the operator accepts this framework split.

## Protected-boundary check

- No kernel/runtime code edited.
- No `SC-WATBAL-001` binding rows edited.
- No `INV-*` or `OBL-*` IDs removed, weakened, or added.
- No procedure-doc path rename.
- No comparator re-tiering; ADR-0017 governance was carried forward as workflow text.

## Required operator decision

Operator acceptance required before Phase 2 reference consolidation.

Decision options:

- `accepted`: proceed to Phase 2.
- `revise`: amend framework docs before Phase 2.
- `hold`: stop package at framework gate and record follow-on.

## Operator decision (recorded)

- Decision: `accepted` (proceed to Phase 2).
- Operator: Roger Lew.
- Date: 2026-06-08.
- Basis: operator instructed Codex to proceed to Phase 2; this constitutes Phase 1
  acceptance of the amended framework (procedure split + `science-contract-spec.md`
  + `science-contract-provenance-spec.md` + binding-exposure lint), with Phase 1
  review findings F1–F5 accepted and folded per
  `artifacts/disposition_phase1_review.md`. Recorded here retroactively for the
  audit trail; the acceptance preceded Phase 2 execution.
- Scope note: acceptance ratifies the framework. The amended Phase 0 inventory is
  a mechanical first-cut; Phase 2 semantic re-adjudication of `unpromoted-binding`
  / `undecidable` / historical rows remains required before any narrative is
  relocated (it was — Phase 2 failed closed at HOLD without relocating).
