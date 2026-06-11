# SCSTRUCT04 — SC-SYSTEM Binding Exposure Index + Triage

Status: executed-deferred-science-review-follow-on
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Routes-to: defect `SCSTRUCT04-SYSTEM-BEI-SCIENCE-REVIEW` (adjudication follow-on SCSTRUCT05)
Execution mode: package-end-to-end (autonomous triage; conservative)

## Objective

Apply the proven SCSTRUCT framework to the second-largest science contract,
`SC-SYSTEM-001` (116KB / 1003 lines / ~29 kt; 29 `INV-SYSTEM-*` invariants; no
Binding Exposure Index yet). Add a **conservative Binding Exposure Index** over
every addendum section and triage each row, **without dropping, weakening, or
silently adding any binding obligation**. This is the SCSTRUCT02-equivalent for
SC-SYSTEM: it builds the index and routes unresolved rows to science review; it
does **not** relocate narrative (that is SCSTRUCT05).

Success: a Binding Exposure Index conforming to `science-contract-spec.md`, the
binding-exposure lint reporting `PASS-DEFERRED` (rows routed, none gamed), and a
classification artifact handing the adjudication queue to SCSTRUCT05.

## Background

The framework (procedure split + `science-contract-spec.md` +
`science-contract-provenance-spec.md` + `tools/check_sc_binding_exposure.py` with
the `PASS` / `PASS-DEFERRED` / `--strict` verdicts) was authored in SCSTRUCT01 and
exercised end-to-end on WATBAL (SCSTRUCT02 triage → SCSTRUCT03 six-batch
adjudication). The WATBAL lesson: most addenda are **already promoted** to
invariants and only look unpromoted because the section body lacks the `INV-*`
string — so the mechanical pass must err conservative (route to review, never
relocate) and the semantic mapping happens in the adjudication package.

`SC-SYSTEM-001` (System Integration Boundary + Watershed Assembly) addenda are
largely WSHEDIMPL channel/sediment/routing integration migrations (each closed a
`GAP-SYSTEM-*`/`GAP-ROUTE-*`), plus HPARITY02 profile-capacity publication-lineage
governance (`INV-SYSTEM-027`) and HPHYS0202–0209 FC/WP publication governance.

## Per-row triage rule (conservative; the core mechanic)

For each addendum section, add a Binding Exposure Index row per
`science-contract-spec.md` and classify:

1. **maps-to-existing-INV** — the section body carries an explicit
   `INV-SYSTEM-*`/`OBL-SYSTEM-*` reference. Record the IDs; gate `none`. (Mapping
   *precision* is confirmed in SCSTRUCT05 before any relocation.)
2. **unpromoted-binding** — binding/obligation language but no same-section
   `INV-SYSTEM-*`. Route `science-review-follow-on` (retain in core).
3. **undecidable** — broad invariant-token scrape or no clear status. Route
   `science-review-follow-on`.
4. **historical-or-superseded** — explicitly marked historical/superseded with an
   ID mapping. Record, but do **not** relocate here.

Do not relocate any narrative. Do not gate a row to `none` without a real
`INV-SYSTEM-*`/`OBL-SYSTEM-*` ID (the lint's no-gaming rule).

## Authority Envelope

### In-scope
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` — **add the
  Binding Exposure Index section only**.
- `tools/check_sc_binding_exposure.py` — run as the gate (no code change).

### Allowed edit classes
- Add the `## Binding Exposure Index` section + rows.
- Author the classification artifact + the science-review handoff queue.

### Protected boundaries (do not cross)
- **No narrative relocation, no `INV-*`/`OBL-*` row added/removed/weakened, no
  kernel/runtime edit.** This package is index + triage only.
- **No comparator re-tiering**; relocate nothing.

## Acceptance criteria
1. Binding Exposure Index present over every addendum section, schema-conformant.
2. `python3 tools/check_sc_binding_exposure.py SC-SYSTEM-001.md` reports
   `PASS-DEFERRED` (or `PASS` if every row maps cleanly); **0 malformed rows, 0
   gamed gate-flips** (no `none`/`none` resolved rows).
3. `git diff` on SC-SYSTEM-001 shows **only** the added index section — no removed
   section, no invariant-table change.
4. Classification + science-review handoff queue authored; SCSTRUCT05 scaffolded
   target named.
5. Dual review + disposition + dual verification; no undispositioned finding.

## Legitimate HOLD conditions
- A row's binding status is genuinely undecidable → route
  `science-review-follow-on` (that is the expected outcome, not a HOLD).
- The lint cannot reach `PASS-DEFERRED` because of a real structural defect → HOLD
  with command-level evidence.

## Dependencies
- SCSTRUCT01 framework docs (`science-contract-{authoring-procedure,spec,provenance-spec}.md`).
- SCSTRUCT02/03 (the proven WATBAL precedent + the adjudication template).
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/codex_exec_plans.md`

## Autonomy
Execute end-to-end and conservatively. Route ambiguity to science review rather
than guessing; relocate nothing. Hand the adjudication queue to SCSTRUCT05.
