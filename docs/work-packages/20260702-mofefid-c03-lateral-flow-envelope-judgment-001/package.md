# MOFEFID-C03 — Lateral-Flow Envelope Judgment Run

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane C
(stage C3). Owner: Claude Code. Read-only analysis (no production edits).

## Objective

Issue the formal verdict for H2637's DC01-corrected forest lateral-flow
magnitude against the ratified `SC-SUBHYD-001#INV-SUBHYD-033` four-tier
envelope: run the per-storm event-tier decomposition C01 deferred (ENV-T,
ENV-E), re-confirm the annual tiers (ENV-Y, ENV-ET), and re-verify the
load-bearing anchor numbers first-hand.

## Verdict: NOT-CONTRADICTED on all four tiers (3 PASS, 1 PASS-with-note)

| Tier | Envelope | H2637 post-DC01 (Ran) | Verdict |
|---|---|---|---|
| ENV-Y annual yield | [0.55, 0.72] | **0.673** | **PASS** |
| ENV-ET annual ET | [500, 1000] mm/yr | **863** | **PASS** |
| ENV-E event ratio | [0.25, 0.80], ascending | **0.46** (large, wet-antecedent); Spearman vs antecedent **+0.65**, vs size **+0.29** | **PASS** (band + shape) |
| ENV-T commencement | detectable in 15–40 mm | shape present (median-zero small storms; ascending); threshold value **~10–20 mm** | **PASS-with-note** (wet-end) |

Per `INV-SUBHYD-033` these are *not-contradicted*, not *validated*: H2637's
corrected magnitude is consistent with observed steep-wet-forest behavior.
**This resolves the FARPOINT01 magnitude question (open since 2026-06-18):
the DC01-corrected 47% runoff / 67% combined yield is what field-observed
wet forest hillslopes do — judged against data, not legacy.**

## Load-bearing methodological finding (contract-refining)

The event tiers **cannot be judged on total (runvol + latqcc) daily export**:
baseflow contaminates it — small storms (<15 mm) show combined-export ratios
of 500–600% (sustained lateral drainage + snowmelt on low-rain days) and the
ratio *anti-correlates* with storm size (Spearman −0.51), the opposite of
the observed law. This is a false-negative trap. The observed ENV-T/ENV-E
laws were defined on **quickflow** (Hewlett–Hibbert separated) or trench
stormflow — the event-responsive component. Applying a Hewlett–Hibbert-style
baseflow separation flips the result to coherent (positive size/antecedent
correlation, in-band ratio). `INV-SUBHYD-033` is amended (rev 14) to require
quickflow separation for the event tiers, aligning the contract with its own
cited authority.

## ENV-T wet-end note

H2637's commencement threshold (~10–20 mm, from the size-bin median-ratio
transition) sits at/just below the envelope's [15, 40] mm band. This is
**consistent with the wetness–threshold inverse relation** across the
anchors: Panola 1,240 mm → 55 mm; WS10 2,220 mm → 30 mm; Maimai 2,450 mm →
~23 mm; H2637 at 2,825 mm extrapolates below Maimai. The band was derived
from drier anchors; H2637 is wetter than all of them, so a lower threshold
is expected physics, not a defect. Recommended follow-up: re-examine the
ENV-T lower edge for the very-wet (>2,600 mm) sub-class. The exact value is
also blurred by snowmelt (15.6% of outlet days carry snowpack), which
injects export on small-rain days; a precise threshold needs melt-aware
event delineation.

## Evidence

- `artifacts/verdict.md` — full result, size-bin table, sensitivity.
- Anchor re-verification (Ran, first-hand PDF reads this package): Panola
  55 mm (Tromp-van Meerveld 2006 §4.3 ¶20, p.6); WS10 30 mm + mean Q_F/P
  0.22/0.31 (McGuire & McDonnell 2010 Table 1 + p.5). Annual anchors
  triangulated ×4 in C01.
- Recession-window sensitivity (3/5/7 days): verdict stable.

## Follow-ups

- ENV-T very-wet sub-class lower-edge refinement (backlog).
- Melt-aware event delineation for precise threshold value (optional).
