# SCSTRUCT06 Kickoff — SC-SUBHYD Binding Exposure Index + triage

Scope: local repository contract-structure task; flat-file reads/edits only.
Execution mode: package-end-to-end (autonomous, conservative).

Autonomy: build the SC-SUBHYD Binding Exposure Index, classify every addendum row,
route ambiguity to science review, run the lint, author the handoff — without
asking. Relocate nothing. Ask/HOLD only at a declared boundary.

## What and why

`SC-SUBHYD-001` (88KB / ~22 kt, 31 INV-SUBHYD, 25 addenda) has no Binding Exposure
Index. Add a conservative index + triage so SCSTRUCT07 can adjudicate it — the same
flow proven on WATBAL (02→03) and SC-SYSTEM (04→05). Err conservative: most addenda
only *look* unpromoted; never relocate or gate-to-`none` without a real
`INV-SUBHYD-*` ID. SC-SUBHYD is the WB19 subsurface family — expect a heavier
live-authority cohort downstream than SC-SYSTEM had.

Read `package.md` first. **Non-negotiable: index + triage only; no narrative
relocation, no `INV-*`/`OBL-*` change, no kernel edit.**

## Required reading
- `package.md` (this WP)
- `docs/specifications/science-contract-spec.md` (BEI schema + lint contract)
- the `## Binding Exposure Index` in `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (worked example)
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (target)
- `../20260610-scstruct04-system-binding-exposure-index-mapping-001/package.md` (precedent)

## Tasks
1. Enumerate every SC-SUBHYD addendum section (heading + line range).
2. Add a `## Binding Exposure Index` per `science-contract-spec.md`, one row per
   section: classify `maps-to-existing-INV` (same-section `INV-SUBHYD-*` IDs, gate
   `none`) / `unpromoted-binding` / `undecidable` / `historical-or-superseded`;
   route everything not cleanly `maps-to-existing-INV` to `science-review-follow-on`.
3. Run `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`;
   confirm `PASS-DEFERRED`, 0 malformed rows, 0 gamed gate-flips. Record output.
4. Author `artifacts/subhyd-addendum-classification.md` and
   `artifacts/science-review-followon-queue.md` (the SCSTRUCT07 input).
5. Dual review/disposition/verification.

## Outputs
- SC-SUBHYD `## Binding Exposure Index` (additive only).
- `artifacts/`: classification, science-review queue, lint output, dual
  review/verification, disposition.

## Hard stops
- Cannot reach `PASS-DEFERRED` due to a real structural defect → HOLD with
  command-level evidence.
- Any edit would relocate narrative, change an invariant, or touch kernel code →
  out of scope, stop.
