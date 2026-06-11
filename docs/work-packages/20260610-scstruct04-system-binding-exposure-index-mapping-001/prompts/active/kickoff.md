# SCSTRUCT04 Kickoff — SC-SYSTEM Binding Exposure Index + triage

Scope: local repository contract-structure task; flat-file reads/edits only.
Execution mode: package-end-to-end (autonomous, conservative).

Autonomy: build the SC-SYSTEM Binding Exposure Index, classify every addendum row,
route ambiguity to science review, run the lint, and author the handoff — without
asking for direction. Relocate nothing. Ask/HOLD only at a declared boundary.

## What and why

`SC-SYSTEM-001` is the second-largest contract (116KB / ~29 kt) and has no Binding
Exposure Index. Add a conservative index + triage so SCSTRUCT05 can adjudicate it,
exactly as SCSTRUCT02 did for WATBAL before SCSTRUCT03. The WATBAL lesson: err
conservative — most addenda are already-promoted and only *look* unpromoted; never
relocate or gate-to-`none` without a real `INV-SYSTEM-*` ID.

Read `package.md` first — it is the authority for the triage rule and protected
boundaries. **Non-negotiable: index + triage only; no narrative relocation, no
`INV-*`/`OBL-*` change, no kernel edit.**

## Required reading
- `package.md` (this WP)
- `docs/specifications/science-contract-spec.md` (Binding Exposure Index schema + lint contract)
- the `## Binding Exposure Index` in
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (worked example)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (target)
- `../20260608-scstruct02-watbal-binding-exposure-index-mapping-001/package.md` (precedent)

## Tasks
1. Enumerate every SC-SYSTEM addendum section (heading + line range).
2. Add a `## Binding Exposure Index` per `science-contract-spec.md`, one row per
   section: classify `maps-to-existing-INV` (with same-section `INV-SYSTEM-*` IDs,
   gate `none`) / `unpromoted-binding` / `undecidable` / `historical-or-superseded`;
   route everything not cleanly `maps-to-existing-INV` to `science-review-follow-on`.
3. Run `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`;
   confirm `PASS-DEFERRED`, 0 malformed rows, 0 gamed gate-flips. Record the output.
4. Author `artifacts/system-addendum-classification.md` and
   `artifacts/science-review-followon-queue.md` (the SCSTRUCT05 input).
5. Dual review/disposition/verification.

## Outputs
- SC-SYSTEM `## Binding Exposure Index` (additive only).
- `artifacts/`: classification, science-review queue, lint output, dual
  review/verification, disposition.

## Hard stops
- Cannot reach `PASS-DEFERRED` due to a real structural defect → HOLD with
  command-level evidence.
- Any edit would relocate narrative, change an invariant, or touch kernel code →
  out of scope, stop.
