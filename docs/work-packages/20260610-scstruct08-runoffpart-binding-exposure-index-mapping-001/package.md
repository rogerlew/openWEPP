# SCSTRUCT08 — SC-RUNOFFPART Binding Exposure Index + Triage

Status: queued
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Routes-to: defect `SCSTRUCT08-RUNOFFPART-BEI-SCIENCE-REVIEW` (adjudication follow-on SCSTRUCT09)
Execution mode: package-end-to-end (autonomous triage; conservative)

## Objective

Apply the proven SCSTRUCT framework to `SC-RUNOFFPART-001` (Surface Runoff
Partition; 93KB / 910 lines / ~23 kt; 27 `INV-RUNOFFPART-*` invariants; no Binding
Exposure Index yet). Add a **conservative Binding Exposure Index** over every
addendum section and triage each row, **without dropping, weakening, or silently
adding any binding obligation**. SCSTRUCT02/04/06-equivalent: build the index,
route unresolved rows to science review; do **not** relocate narrative (that is
SCSTRUCT09).

Success: a Binding Exposure Index conforming to `science-contract-spec.md`, the
lint reporting `PASS-DEFERRED` (rows routed, none gamed), and a classification
artifact handing the adjudication queue to SCSTRUCT09.

## Background

Framework authored in SCSTRUCT01, exercised on WATBAL (02→03), SC-SYSTEM (04→05),
and SC-SUBHYD (06→07). Recurring lesson: most addenda are **already promoted** and
only look unpromoted because the section body lacks the `INV-*` string — triage
errs conservative (route to review, never relocate); semantic mapping is SCSTRUCT09.

**SC-RUNOFFPART is mixed-character** — between SC-SYSTEM (clean relocations) and
SC-SUBHYD (all live map-in-core). Its addenda span:
- the **ADR-0017-retired snow/`RM` comparator arc** (HPHYS0296–0298) — likely
  `historical`/relocatable (the WATBAL snow-arc pattern);
- WB12/WB14 runoff-carryover + spring snowmelt runoff/infiltration partition
  (HPHYS0240–0293, SNOWSCI) — live runoff-partition lineage, map-in-core/historical;
- WB16 `ealpha` producer migration (HILLSTAB06–08), SIMIMPL36, and
  `GAP-RUNOFFPART-001..005` — live authority.
So expect SCSTRUCT09 to yield **some** token reduction (the retired snow-arc rows)
plus map-in-core for the live runoff/WB16 authority — between SC-SYSTEM and
SC-SUBHYD.

## Per-row triage rule (conservative; the core mechanic)

For each addendum section, add a Binding Exposure Index row per
`science-contract-spec.md` and classify:

1. **maps-to-existing-INV** — section body carries an explicit
   `INV-RUNOFFPART-*`/`OBL-RUNOFFPART-*` reference. Record IDs; gate `none`.
2. **unpromoted-binding** — binding/obligation language but no same-section
   `INV-RUNOFFPART-*`. Route `science-review-follow-on` (retain in core).
3. **undecidable** — broad invariant-token scrape or no clear status. Route
   `science-review-follow-on`.
4. **historical-or-superseded** — explicitly marked historical/superseded with an
   ID mapping. Record, but do **not** relocate here.

Do not relocate any narrative. Do not gate a row to `none` without a real
`INV-RUNOFFPART-*`/`OBL-RUNOFFPART-*` ID (the lint's no-gaming rule).

## Authority Envelope

### In-scope
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` — **add
  the Binding Exposure Index section only**.
- `tools/check_sc_binding_exposure.py` — run as the gate (no code change).

### Allowed edit classes
- Add the `## Binding Exposure Index` section + rows.
- Author the classification artifact + the science-review handoff queue.

### Protected boundaries (do not cross)
- **No narrative relocation, no `INV-*`/`OBL-*` row added/removed/weakened, no
  kernel/runtime edit.** Index + triage only.
- **No comparator re-tiering**; relocate nothing. ADR-0017 governs the snow/`RM`
  rows — classify, do not re-tier.

## Acceptance criteria
1. Binding Exposure Index present over every addendum section, schema-conformant.
2. `python3 tools/check_sc_binding_exposure.py SC-RUNOFFPART-001.md` reports
   `PASS-DEFERRED` (or `PASS`); **0 malformed rows, 0 gamed gate-flips**.
3. `git diff` on SC-RUNOFFPART-001 shows **only** the added index section — no
   removed section, no invariant-table change.
4. Classification + science-review handoff queue authored; SCSTRUCT09 target named.
5. Dual review + disposition + dual verification; no undispositioned finding.

## Legitimate HOLD conditions
- A row's binding status is genuinely undecidable → route
  `science-review-follow-on` (the expected outcome, not a HOLD).
- The lint cannot reach `PASS-DEFERRED` because of a real structural defect → HOLD
  with command-level evidence.

## Dependencies
- SCSTRUCT01 framework docs (`science-contract-{authoring-procedure,spec,provenance-spec}.md`).
- SCSTRUCT02/03, 04/05, 06/07 precedents.
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
  (for the snow/`RM` arc rows).
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/codex_exec_plans.md`

## Autonomy
Execute end-to-end and conservatively. Route ambiguity to science review rather
than guessing; relocate nothing. Hand the adjudication queue to SCSTRUCT09.
Subagent requirement: none (docs-only triage; no heavy runs).
