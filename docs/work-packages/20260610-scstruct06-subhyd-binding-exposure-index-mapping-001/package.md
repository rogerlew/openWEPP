# SCSTRUCT06 — SC-SUBHYD Binding Exposure Index + Triage

Status: queued
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Routes-to: defect `SCSTRUCT06-SUBHYD-BEI-SCIENCE-REVIEW` (adjudication follow-on SCSTRUCT07)
Execution mode: package-end-to-end (autonomous triage; conservative)

## Objective

Apply the proven SCSTRUCT framework to `SC-SUBHYD-001` (Subsurface Hydrology and
Drainage; 88KB / 836 lines / ~22 kt; 31 `INV-SUBHYD-*` invariants; 25 addendum
sections; no Binding Exposure Index yet). Add a **conservative Binding Exposure
Index** over every addendum section and triage each row, **without dropping,
weakening, or silently adding any binding obligation**. SCSTRUCT02/04-equivalent:
build the index, route unresolved rows to science review; do **not** relocate
narrative (that is SCSTRUCT07).

Success: a Binding Exposure Index conforming to `science-contract-spec.md`, the
lint reporting `PASS-DEFERRED` (rows routed, none gamed), and a classification
artifact handing the adjudication queue to SCSTRUCT07.

## Background

Framework authored in SCSTRUCT01, exercised on WATBAL (SCSTRUCT02→03) and SC-SYSTEM
(SCSTRUCT04→05). The recurring lesson: most addenda are **already promoted** and
only look unpromoted because the section body lacks the `INV-*` string — so triage
errs conservative (route to review, never relocate); semantic mapping happens in
the adjudication package.

**SC-SUBHYD differs in character from SC-SYSTEM.** SC-SYSTEM was mostly WSHEDIMPL
GAP-closures that mapped/relocated cleanly. SC-SUBHYD is the **WB19 subsurface
lateral/drainage/water-yield family** (HPHYS0218–0267) plus `GAP-SUBHYD-001..004`,
with several Level-4 constitutive suite linkages (HPHYS0224–0227). Expect a
heavier **live-authority** cohort in SCSTRUCT07 — more map-in-core / promote and
fewer historical relocations, so a smaller token yield. That is the correct
outcome, not a shortfall: the obligations are load-bearing.

## Per-row triage rule (conservative; the core mechanic)

For each addendum section, add a Binding Exposure Index row per
`science-contract-spec.md` and classify:

1. **maps-to-existing-INV** — section body carries an explicit
   `INV-SUBHYD-*`/`OBL-SUBHYD-*` reference. Record the IDs; gate `none`. (Mapping
   *precision* is confirmed in SCSTRUCT07 before any relocation.)
2. **unpromoted-binding** — binding/obligation language but no same-section
   `INV-SUBHYD-*`. Route `science-review-follow-on` (retain in core).
3. **undecidable** — broad invariant-token scrape or no clear status. Route
   `science-review-follow-on`.
4. **historical-or-superseded** — explicitly marked historical/superseded with an
   ID mapping. Record, but do **not** relocate here.

Do not relocate any narrative. Do not gate a row to `none` without a real
`INV-SUBHYD-*`/`OBL-SUBHYD-*` ID (the lint's no-gaming rule).

## Authority Envelope

### In-scope
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` — **add the
  Binding Exposure Index section only**.
- `tools/check_sc_binding_exposure.py` — run as the gate (no code change).

### Allowed edit classes
- Add the `## Binding Exposure Index` section + rows.
- Author the classification artifact + the science-review handoff queue.

### Protected boundaries (do not cross)
- **No narrative relocation, no `INV-*`/`OBL-*` row added/removed/weakened, no
  kernel/runtime edit.** Index + triage only.
- **No comparator re-tiering**; relocate nothing.

## Acceptance criteria
1. Binding Exposure Index present over every addendum section, schema-conformant.
2. `python3 tools/check_sc_binding_exposure.py SC-SUBHYD-001.md` reports
   `PASS-DEFERRED` (or `PASS`); **0 malformed rows, 0 gamed gate-flips**.
3. `git diff` on SC-SUBHYD-001 shows **only** the added index section — no removed
   section, no invariant-table change.
4. Classification + science-review handoff queue authored; SCSTRUCT07 target named.
5. Dual review + disposition + dual verification; no undispositioned finding.

## Legitimate HOLD conditions
- A row's binding status is genuinely undecidable → route
  `science-review-follow-on` (the expected outcome, not a HOLD).
- The lint cannot reach `PASS-DEFERRED` because of a real structural defect → HOLD
  with command-level evidence.

## Dependencies
- SCSTRUCT01 framework docs (`science-contract-{authoring-procedure,spec,provenance-spec}.md`).
- SCSTRUCT02/03 (WATBAL) and SCSTRUCT04/05 (SC-SYSTEM) precedents.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/codex_exec_plans.md`

## Autonomy
Execute end-to-end and conservatively. Route ambiguity to science review rather
than guessing; relocate nothing. Hand the adjudication queue to SCSTRUCT07.
