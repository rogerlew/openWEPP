# SCSTRUCT08 Kickoff — SC-RUNOFFPART Binding Exposure Index + triage

Scope: local repository contract-structure task; flat-file reads/edits only.
Execution mode: package-end-to-end (autonomous, conservative).
Subagent authorization: none (docs-only triage; no heavy runs).

Autonomy: build the SC-RUNOFFPART Binding Exposure Index, classify every addendum
row, route ambiguity to science review, run the lint, author the handoff — without
asking. Relocate nothing. Ask/HOLD only at a declared boundary.

## What and why

`SC-RUNOFFPART-001` (93KB / ~23 kt, 27 INV-RUNOFFPART) has no Binding Exposure
Index. Add a conservative index + triage so SCSTRUCT09 can adjudicate — the flow
proven on WATBAL (02→03), SC-SYSTEM (04→05), SC-SUBHYD (06→07). Err conservative:
most addenda only *look* unpromoted; never relocate or gate-to-`none` without a
real `INV-RUNOFFPART-*` ID. SC-RUNOFFPART is mixed-character: the ADR-0017-retired
snow/`RM` arc rows (HPHYS0296–0298) are likely historical, the WB12/14/16 runoff
authority is live.

Read `package.md` first. **Non-negotiable: index + triage only; no narrative
relocation, no `INV-*`/`OBL-*` change, no kernel edit; no comparator re-tiering.**

## Required reading
- `package.md` (this WP)
- `docs/specifications/science-contract-spec.md` (BEI schema + lint contract)
- the `## Binding Exposure Index` in `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (worked example)
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` (target)
- `../20260610-scstruct06-subhyd-binding-exposure-index-mapping-001/package.md` (precedent)

## Tasks
1. Enumerate every SC-RUNOFFPART addendum section (heading + line range).
2. Add a `## Binding Exposure Index` per `science-contract-spec.md`, one row per
   section: classify `maps-to-existing-INV` (same-section `INV-RUNOFFPART-*` IDs,
   gate `none`) / `unpromoted-binding` / `undecidable` / `historical-or-superseded`;
   route everything not cleanly `maps-to-existing-INV` to `science-review-follow-on`.
3. Run `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`;
   confirm `PASS-DEFERRED`, 0 malformed rows, 0 gamed gate-flips. Run `--strict`
   as a not-consolidated signal; strict exits nonzero while deferred rows remain.
   Record both outputs.
4. Author `artifacts/runoffpart-addendum-classification.md` and
   `artifacts/science-review-followon-queue.md` (the SCSTRUCT09 input).
5. Dual review/disposition/verification.

## Outputs
- SC-RUNOFFPART `## Binding Exposure Index` (additive only).
- `artifacts/`: classification, science-review queue, lint output, dual
  review/verification, disposition.

## Hard stops
- Cannot reach `PASS-DEFERRED` due to a real structural defect → HOLD with
  command-level evidence.
- Any edit would relocate narrative, change an invariant, or touch kernel code →
  out of scope, stop.
