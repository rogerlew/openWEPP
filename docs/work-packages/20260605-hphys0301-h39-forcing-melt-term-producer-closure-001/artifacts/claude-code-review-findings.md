# Claude Code Review Findings — HPHYS0301

Reviewer: Claude Code (independent review).
Verdict: **APPROVE.** 0301 correctly refused to fix H39 because the "clean H39
forcing defect" was another comparison-surface artifact in the diagnostic
harness, not an openWEPP physics defect. This refutes — and withdraws — the
CLAUDE-0300-002 "fix H39 now" recommendation.
Evidence mode: static (disposition, ledger) + ran (metric cross-check, corrected
H39 deltas).

## What 0301 found

H39 first-2013 — reclassified by 0299 as "the one real depth-vs-depth
hourly-forcing defect," and which CLAUDE-0300-002 and the 0300 disposition called
"cleanly root-caused, fix now" — is a **comparison-surface mismatch**:

- HPHYS0300 H39 compared baseline residual rain-on-snow against openWEPP raw
  rain: `baseline_minus_open_raw_rain_mm = -16.476986` (looked like a forcing
  defect).
- Correct comparison (baseline residual rain-on-snow vs openWEPP released +
  post-winter rain): `baseline_minus_open_released_plus_post_rain_mm =
  -0.237193` — essentially closed.

There was no H39 forcing defect to fix. 0301 reclassified
`corrected-depth-hourly-forcing-hold` -> `h39-rain-release-lineage-reclassified-hold`
and authorized no production edit. No production change; metrics unchanged from
0298-0300 (correct, because nothing was broken at H39).

## Withdrawal of CLAUDE-0300-002

My "fix H39 now" recommendation is **withdrawn**. Acting on it would have patched
production to chase a -0.24 mm artifact — a spurious fix. I propagated 0299's
"H39 is the one real forcing defect" without checking whether it, too, was a
surface mismatch. This is the second time in three packages I over-trusted a
harness-derived root cause (first 0298 `hrsnow` depth-vs-water-equiv; now H39
raw-rain-vs-residual-rain). The bounded evidence-gate criterion from the 0300
review is what correctly prevented the spurious H39 fix.

## The reframed concern — the harness, not the fix cadence

The candidate "defects" keep evaporating under correct comparison:

| Package | Apparent defect | Cause | Corrected delta |
|---|---|---|---|
| 0298 | `hrsnow` 68.6 vs 6.9 mm | depth vs water-equiv (10x density ratio) | ~closes |
| 0301 | H39 rain -16.5 mm | raw rain vs released+post rain | -0.24 mm (~closes) |

Two-for-two, the "clean root causes" were **comparison-surface mismatches in the
diagnostic comparator, not openWEPP physics defects.** Implications:

1. The nine zero-movement packages (0293-0301) partly reflect that **there is
   less real openWEPP defect than the comparator suggests** — some residual is
   harness measurement error, not model error. Zero movement is partly *correct*:
   you don't move metrics by fixing things that aren't broken.
2. The next "melt-term producer defect" (`amelt`/`bmelt`/`cmelt`/`dmelt`) is at
   high prior risk of being a third surface artifact if attributed from aggregate
   deltas.

## CLAUDE-0301-001 [HIGH, recommendation] — Audit comparator paired surfaces before any further producer-defect conclusion

Before attributing any remaining residual to an openWEPP producer defect, prove
for each residual symbol (`RM`, `Snow-Water`, and the melt terms) that the
baseline and openWEPP cut-points are **the same physical quantity in the same
units** (depth vs depth, water-equiv vs water-equiv, raw vs raw, released vs
released). Make this a gate, not an afterthought — two artifacts have already
slipped through. Recompute the post-surface-correction residual: dominant
`Total-Soil` (56 mm) and `Snow-Water` (2.9 mm) may shrink materially once
mismatches are removed.

## CLAUDE-0301-002 [POSITIVE] — Discipline working correctly

No production change, no compensation, honest reclassification, dual review run.
The refusal to fix without valid comparison authority is exactly what caught both
the H39 artifact and my endorsement of it. `INV-SNOWFREEZE-032` /
`INV-WATBAL-076` added.

## Bottom line

0301 is good work that caught a real diagnostic error (including the one I
endorsed). Updated read of the arc: **openWEPP's snow/RM physics may be closer to
correct than the suite shows, and the comparison harness is itself a significant
source of apparent residual.** Next package should be a comparator-surface audit,
not another producer-defect hunt, and certainly not a production patch against a
delta not yet proven to be like-for-like.
