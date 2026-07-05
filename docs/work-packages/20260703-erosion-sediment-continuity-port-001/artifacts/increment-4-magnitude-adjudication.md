# Increment 4 (ROADMAP §E.5) — Erosion Magnitude Adjudication

Author: Claude Code, 2026-07-05. Evidence: **Ran** (openWEPP executions on
the full-length source inputs + legacy output comparison) + **Static**
(operand attribution). Branch: `erosion-e5-magnitude-adjudication`.
Posture: ADR-0017 — legacy is an investigation flag, never an acceptance
oracle; verdicts are attributions, not tuning targets.

## 1. Method

Two instruments, like-for-like cut-points (per the E.1 record correction:
per-width DELIVERY `Sed.Del kg/m` vs `(tdet − tdep)/fwidth`, never the
field-width-scaled totals; water cut FIRST):

- **p61** (single-OFE disturbed forest, `assisted-weakness`, 11-year
  climate): openWEPP run on the FULL source inputs vs `H61.ebe.dat`
  per-event rows. Ran clean.
- **p102** (2-OFE disturbed forest, `insensible-aliquot`, 50-year
  climate): openWEPP (E.4 binary) vs `H102.ebe.dat`/`H102.loss.dat`
  aggregates. Outlet-row caveat: openWEPP pass rows are outlet-scoped, so
  the chain-export series is not directly published per day; the
  comparison uses outlet detachment/deposition sums and the legacy annual
  means — adequate for order-of-magnitude attribution, labeled as such.

## 2. Results

### Water cut (FIRST)
- **p102, 50 years: openWEPP runoff volume ≈ 8,520 mm vs legacy event-sum
  8,439 mm — within ~1%.** At this site the water system is NOT the
  magnitude driver. (A materially positive datapoint alongside the
  H2637/FARPOINT01 lateral-magnitude contract gap — different site,
  different regime, near-parity.)
- p61 dominant event: openWEPP 9.1 mm vs legacy 12.5 mm (0.73×) — same
  order; cannot explain an over-delivery.

### Delivery
- **p61 dominant event (yr 4, 8/22): openWEPP exported 25.1 kg/m vs
  legacy `Sed.Del` 4.2 kg/m — ~6× OVER with LESS water.** Legacy is
  detachment-limited on this event (`Av-det 0.03 kg/m² × 120.5 m ≈ 3.6 ≈
  Sed.Del`); openWEPP's average detachment is ~0.21 kg/m² — the gap is
  the DETACHMENT RATE, not transport/deposition.
- p102, 50 years: openWEPP outlet-lane detachment ≈ 84 kg/m/yr vs legacy
  whole-hillslope ≈ 19.4 kg/m/yr (`H102.loss.dat` avg annual) — the same
  ~4–6× over-detachment class.
- p61 small events: legacy routes a 3.9 mm event (`Sed.Del 0.1`);
  openWEPP's counterpart (4.4 mm) publishes zero sediment — small-event
  gate divergence (secondary; bounded UNDER-routing).

### Attribution (ephemeral operand probe — REMOVED after evidence capture)
An env-gated stderr probe (`OPENWEPP_DEBUG_EROSION_OPERANDS`) was added
to the quantum assembly for this adjudication and removed in the same
branch after capturing the evidence below (Codex round-1: ad-hoc stderr
in the kernel hot path is not the governed trace style; the follow-on
WP re-instruments through the established trace-path mechanism when it
re-runs the instruments).
On every routed p61 event day: **`rilcov = 0.000`, `inrcov = 0.000`** —
zero ground cover reaching the erosion assembly on a forest hillslope
(canopy is correct at 0.75). The erosion daily covers consume ONLY the
mass-derived residue-partition cover (`erosion.rs`: both cover fields ←
`residue_partition.cover_fraction`). The p61 IC scenario (`Tah_2307`,
"no growth, no decomp") DECLARES `inrcov = 0.85`, `rilcov = 0.85` with
near-zero seeded residue mass (`sumrtm/sumsrm 0.1/0.1`); legacy holds
the declared covers (its no-decomp scenario preserves them), openWEPP's
mass-derived cover is ~0, so detachment runs bare-soil. The declared-
cover authority ALREADY reaches two other consumers — the WB16 friction
path (`direct_production_typed_wb16_ealpha`) and the frost residue-depth
seeding (`legacy_initial_residue_depth_m` back-derives mass from the
declared covers) — but NOT the erosion cover operands. Bare-soil vs
0.85-covered spans roughly the observed 4–6× detachment factor class.

## 3. Verdicts

| # | Finding | Verdict | Disposition |
|---|---|---|---|
| 1 | ~6× over-delivery at matched cut-points (p61); the same ~4–6× class at outlet-aggregate scope (p102, order-of-magnitude corroboration) | **CONFIRMED DEFECT — erosion ground-cover pathway** (input authority: the management IC's declared `inrcov`/`rilcov` never reach the erosion cover operands; mass-derived cover is ~0 on forest no-decomp ICs) | Follow-on defect-closure item (below); NOT fixed in E.5 — the correct fix requires resolving legacy's daily cover-update semantics (`cvrcal.for`: recomputed-from-pools vs carried-state) from source, which crosses into the SC-RESIDUE-001 system |
| 2 | Small-event routing divergence (legacy routes 3.9 mm; openWEPP gates) | Secondary divergence — bounded UNDER-routing on trace events | Recorded; adjudicate the day-gate threshold against `contin.for` in the same follow-on |
| 3 | Water magnitude at p102 | **NEAR-PARITY (~1% over 50 years)** — water is not the erosion-magnitude driver at these sites | On record as a positive water datapoint (does not generalize to the H2637 lateral-magnitude gap — different regime) |
| 4 | Absolute erosion magnitude overall | **OPEN-BUT-ATTRIBUTED**: cover-dominated, not water-dominated, not structural (the E.1–E.4 closures all hold; the solve chain is conservation-exact) | `GAP-SED-009` (SC-SED-001 rev 48): magnitude validation blocked behind the cover-pathway fix, then re-judged |

## 4. Follow-on (the cover-pathway defect-closure item)

Scope for the next WP (defect-closure shape — diagnose AND correct):
1. Resolve from source how legacy maintains `inrcov`/`rilcov` daily
   (`cvrcal.for` / `resup.for`): recomputed from pools each day, or
   carried state adjusted by decomposition/tillage events? This decides
   whether the fix is (a) seed the residue pools consistently with the
   declared covers (the frost path's back-derivation, applied to the
   POOLS the partition covers derive from), or (b) carry declared-cover
   state with dynamics on top.
2. Wire the chosen authority into the erosion daily covers; the frost
   arc's `legacy_initial_residue_depth_m` inverse is the existing
   in-repo precedent for (a).
3. Re-run this adjudication's instruments; expected: p61 dominant event
   into the legacy order (the water cut bounds the residual at ~0.6–1×);
   re-judge `GAP-SED-009`.
4. Same WP adjudicates the small-event gate against `contin.for`.

## 5. What E.5 explicitly did NOT do

No parameter tuning toward legacy numbers (ADR-0017); no cover fix
without the `cvrcal` semantics resolved (no provisional math); no
per-event p102 chain-export series (needs either per-day chain sums on
an interchange surface or replay instrumentation — noted as a tooling
gap, not blocking the attribution).

## 6. Codex review round 1 — response record (2026-07-05)

Three findings, all CONFIRMED and fixed:
1. **Medium — p102 overclaimed as matched cut-point evidence:** the
   artifact's own §1 caveat was correct but SC-SED-001/ROADMAP flattened
   it. `GAP-SED-009` (rev 49) and the ROADMAP now distinguish p61
   (matched per-event, per-width delivery) from p102 (outlet-aggregate
   corroboration, order-of-magnitude only); verdict row 1 reworded.
2. **Medium — the operand probe in the production hot path:** removed
   after evidence capture (recorded above); the follow-on WP
   re-instruments via the governed trace style.
3. **Low — nonstandard evidence label:** normalized to
   `[DIRECT][Static] + [Ran]`.
