# MOFEFID-B01 — Stakeholder-Brief Adjudication-First Audit

Status: **EXECUTED — REVIEW-READY** (2026-07-01). Eleven dual verdicts issued; no openWEPP production defect found; two contract-decision follow-ups spawned (B07→MOFEFID-B02 QOFE; B10→MOFEFID-B03 SC-SNOWFREEZE-015 reconciliation). See `artifacts/verdict-table.md`.
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane B.
Owner: Claude Code (operator-directed role break). Reviewer at close: Codex.

## Objective

Audit openWEPP against the eleven defect classes reported by the
wepp-forest water-balance program
(`/workdir/wepp-forest/docs/20260504-stakeholder-watbalance.md`), under the
campaign §4 **adjudication-first protocol**. Operator posture is binding:
**the brief is a flag list, not an authority** — its conclusions are not
fully trusted; every claim earns its grade independently.

## Protocol (per class — campaign §4, binding)

1. Restate the brief's claim; separate the *problem observation* from the
   *repair conclusion* (distinct claims, distinct evidence).
2. Grade authority: `conservation-forced` / `source-intent` (only after
   re-reading the cited legacy source ourselves) / `convention` /
   `unverified`.
3. Derive the openWEPP-native correct behavior from our own authority chain
   (`SC-*`, conservation identities, ADR-0024 source intent, ADR-0019
   consumer closure semantics). "wepp-forest concluded X" is never a
   justification.
4. Audit openWEPP against the adjudicated ground truth (**Ran** where a
   test/run decides, **Static** otherwise).
5. Dual verdict: brief-claim disposition (`upheld` / `partially-upheld` /
   `convention-not-defect` / `unsubstantiated`) × openWEPP disposition
   (`correct-by-construction` / `defect` / `not-applicable` /
   `contract-decision`).

## Class roster

B1 hourly q-cap bottom-OFE bypass · B2 snowmelt closure-basis double-count ·
B3 interception-storage export · B4 zero-input flux invariant ·
B5 rain-routing conflation (rain-on-snow) · B6 clamp+preserve interaction ·
B7 `QOFE` denominator convention (**known contract-decision-shaped**; Ran
evidence at `01_publication.rs:370-376`) · B8 R01 cascade-tail rain-event
counting (repair failed its own cohort gate) · B9 dry-day per-OFE residual
(open in legacy) · B10 winter mixed-melt day-end aggregation math ·
B11 surface-pulse diagnostic over-reach on `latqcc`-only days.

Pre-registered skepticism hooks (campaign §4) apply to B2/B6/B7/B8/B10.

## Acceptance gates

1. Eleven-row verdict table in `artifacts/verdict-table.md`, each row:
   authority grade, dual verdict, evidence class, file:line evidence.
2. Per-class working notes in `artifacts/class-notes.md` including the
   independent source reads for any `source-intent` grade.
3. Comparator-hygiene entries recorded for classes where legacy production
   carries the defect (B8/B9 expected).
4. Contract-decision rows produce named follow-on packages (B7 → `MOFEFID-B02`
   QOFE ecosystem-contract adjudication expected); no fix is implemented
   inside this package.
5. No production source modified by this package.
6. Codex review + disposition artifacts at close.
