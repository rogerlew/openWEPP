# Claude Code Review — FQ3 ET / Runoff Zero-Term Characterization

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: mixed. **Static:** read the summary, ledger, defect handoff,
disposition. **Ran:** independently parsed the legacy `wepp_260606` WAT
(`/tmp/fq3_exec/legacy/outputs/`) for a Corn (p8) and a Tah (p1) hillslope and
compared to the openWEPP post-FQ1 sums (`duckdb`), to confirm the comparator flag
is real.

Verdict: **Approve.** Well-grounded characterization; the comparator flag is
verified real (not a tool artifact); two openWEPP defects are confirmed and
correctly routed; interception is handled honestly (legacy-unavailable, not
falsely labeled). The analysis-tooling fixes (wepp_id remap, merge column
handling) addressed exactly the tool-bug class that broke the FROSTVAL01 ledger.

---

## F1 — The defect classification is sound; I verified the flag

| | legacy Ep | openWEPP Ep | legacy Q | openWEPP Q |
|---|---:|---:|---:|---:|
| p8 (Corn) | 1831 | 0 | 760 | 0 |
| p1 (Tah_4899) | 5824 | 5511 | 278 | 0 |

- **Corn `Ep`=0 is a real defect** (legacy corn transpires ~1831 mm; openWEPP
  gives 0) and is correctly *not* flagged for Tah (legacy 5824 ≈ openWEPP 5511).
  36/36 Corn prefixes. Confirmed.
- **`Q`=0 is a real, ~universal defect** (legacy runs off on both Corn *and* Tah;
  openWEPP gives 0 everywhere). 35/42. Confirmed.
- **Interception held honestly:** legacy WAT exposes no interception flux term, so
  FQ-3 correctly classified it `legacy-unavailable` rather than labeling a defect
  without a flag — exactly the discipline the package required.
- **Good ADR-0018 grouping:** Corn ET+canopy bundled into one follow-on; `Q`
  separated — correct, because `Q`=0 is universal (it hits Tah too, which
  transpires fine), so runoff is a distinct root cause from the Corn-canopy issue.

## F2 — Strategic: these are partition/magnitude defects, not conservation (the real story)

The water still conserves — rung-1 closure holds — but it is **mis-partitioned**:
on Corn the transpiration that should occur (legacy Ep≈1831) is instead dumped to
soil evaporation (openWEPP Es≈4886 vs legacy 2764), and the runoff that should
occur (legacy Q≈760) infiltrates/stores instead (openWEPP Q=0). So rung-1's
conservation closure was **necessary but not sufficient**: the model conserves
water while getting the ET partition and runoff generation physically wrong.

The rung-2 substrate (Corn cropland + gridmet) exposed two fundamental defects
conservation could never catch:

1. **Annual-crop (Corn) ET engagement is missing** — openWEPP drives perennial /
   continuous-canopy growth (Tah works) but not the annual-crop planting → growth
   → canopy → transpiration cycle. 36 hillslopes.
2. **Runoff generation is absent/under-produced** — `Q`=0 nearly everywhere
   (and where nonzero, materially below legacy). Universal.

These are arguably **higher-priority than the frost gate (FQ-4)** and likely
long-standing — `Q`=0 may have been present on the rung-1 substrate too, simply
never flagged because conservation closed regardless. This is the conservation-vs-
magnitude split the roadmap anticipated, now made concrete: rung-1 closed the
books; rung-2 shows the books are partitioned wrong.

## F3 — Interception is likely a symptom of the Corn-ET root cause

Corn `Interception`=0 (while Tah p1 = 643) is comparator-unavailable via legacy
WAT, but it is almost certainly the **same root cause** as Corn `Ep`=0: no
annual-crop canopy growth → no canopy → no interception, no transpiration, no
residue evaporation (`Er`=0). The handoff correctly folds canopy/interception into
`FQ3-DC-ET-CORN-ENGAGEMENT-001`. Recommendation: that DC should treat
`Ep`+`Interception`+`Er` as one canopy-engagement defect and define interception
acceptance **contract-first** (SC-EVAP/canopy authority), since legacy WAT cannot
flag it — don't let the comparator-availability gap stall the canopy fix.

## F4 — Runoff is under-produced, not merely zero

The handoff notes 7 Corn cases are nonzero but materially below legacy. So
`FQ3-DC-RUNOFFPART-QQOFE-001` should target the runoff-generation *magnitude*
(SC-RUNOFFPART-001), not just "make Q nonzero." Acceptance is contract-first, with
`wepp_260606` as a flag (ADR-0017), not a match target.

Gate note: the doc-lint wrapper panicked on cross-root paths (recorded in
`gate-results.md`) — a tooling issue, not a characterization defect; worth a small
tooling fix so future cross-root packages lint.

---

## Recommendation

Approve. The two routed DC-ExecPlans are the substantive rung-2 work and should
sequence **before FQ-4 (frost)** — a non-engaging crop-ET path and an absent runoff
path would confound any frost-gate assessment. Suggested order: `FQ3-DC-ET-CORN-
ENGAGEMENT-001` and `FQ3-DC-RUNOFFPART-QQOFE-001` (independent envelopes, can
parallelize), with the FQ-2 ledger fix and the p11 percolation follow-on alongside;
FQ-4 frost last, on the repaired substrate. The headline for the roadmap: rung-2
turned up two fundamental partition/magnitude defects (crop ET, runoff) that
conservation closure did not catch — that is the real finding here, not frost.
