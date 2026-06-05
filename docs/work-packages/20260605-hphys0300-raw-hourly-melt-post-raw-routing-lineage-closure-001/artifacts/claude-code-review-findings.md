# Claude Code Review Findings — HPHYS0299 + HPHYS0300 (combined)

Reviewer: Claude Code (independent review).
Scope: HPHYS0299 (hourly snow partition unit/provenance correction) and
HPHYS0300 (raw hourly melt / post-raw routing lineage). Also records a revision
to the prior HPHYS0298 review.
Verdict: **0299 APPROVE (a consequential correction that supersedes 0298's
headline). 0300 APPROVE as a principled, evidence-gated diagnostic — with an
infinite-regress flag.**
Evidence mode: static (dispositions, ledgers, contracts) + ran (arithmetic,
review-status, metric cross-check).

## Revision to the HPHYS0298 review (important)

HPHYS0298's celebrated root cause — "openWEPP produces ~10% of baseline hourly
snowfall (`hrsnow` 68.569 vs 6.857 mm), all 9 windows `hourly-forcing`" — was a
**depth-vs-water-equivalent unit mismatch in the diagnostic comparison harness**,
not a physics defect. Baseline `hrsnow` is snowfall *depth* (`stmtim.for`);
HPHYS0298 compared it against openWEPP `snow_hourly_snowfall_water_equiv_sum_m`.
`68.569 / 6.857 = 9.9999` — exactly the snow-density ratio (ρ≈100 kg/m^3): the
two surfaces describe the *same* snowfall in different units.

HPHYS0299 corrected the comparison to depth-vs-depth
(`snow_hourly_snowfall_depth_sum_m`) and re-verdicted: **8 of 9 windows
reclassify from `hourly-forcing` to `raw-hourly-melt`**; only H39 first-2013
remains a real depth-vs-depth `hourly-forcing` producer defect.

My prior HPHYS0298 `claude-code-review-findings.md` took the 68.569/6.857 delta
at face value as a definitive ~10x physics defect and rated 0298 the strongest
package of the arc. That over-read is corrected here: 0298's *machinery* (paired
instrumentation, observe-identity) was sound, but its *comparison surfaces* were
unit-mismatched, and neither 0298's dual review nor mine caught it. A clean ~10x
ratio in a snow-depth/water-equivalent context should have triggered a
unit-mismatch hypothesis. Note the irony: this depth-vs-water-equiv bug lived in
the diagnostic comparator, which the 0272-0280 unit-governance arc never covered.

## HPHYS0299 — APPROVE (correction)

- Caught and fixed the comparison-surface unit mismatch; re-ran the paired ledger
  with observe-identity re-verified bit-identical.
- Honest re-verdict: `raw-hourly-melt=7`, `negative-melt-correction=1`,
  `hourly-forcing=1`; `OPENWEPP-DEFECTIVE=9`.
- No production change; zero metric movement (expected — diagnostic correction).
- Dual review was back ON (off for 0292-0296) and is what is catching these.

## HPHYS0300 — APPROVE (principled diagnostic) with flag

Good discipline:
- Refused to fix `raw-hourly-melt` on `aggregate-only` evidence; holds pending
  paired melt-*term* instrumentation (`amelt`/`bmelt`/`cmelt`/`dmelt`) per new
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-031` / `SC-WATBAL-001#INV-WATBAL-075`.
- Did NOT falsely accept H7 first-2013 as a baseline-negative-melt-bug window
  (`baseline_negative_raw_melt_sum = 0.0` -> can't attribute to the bug -> hold).
  The A-F acceptance discipline is holding: no false acceptance, H39 kept
  separate.
- No production change.

### CLAUDE-0300-001 [MEDIUM] — Infinite-regress instrumentation risk
This is the 8th consecutive zero-movement package (0293-0300), 0298's
"definitive" finding was a harness artifact, and 0300's recommendation is yet
another instrumentation level before any fix. `INV-SNOWFREEZE-031`
("paired melt-term/state evidence required before correction") is sound rigor but
can become a perpetual gate. There is no committed criterion for when evidence is
sufficient to commit a fix.

### CLAUDE-0300-002 [recommendation] — Forcing function: fix the window already root-caused
H39 first-2013 is a clean depth-vs-depth `hourly-forcing` defect *as of 0299* — it
does not need the raw-melt-term work. The next package should:
1. Fix H39 `hourly-forcing` now (an earned, real fix that breaks the
   zero-movement streak), and
2. Instrument the paired melt terms for the 7 `raw-hourly-melt` windows AND
   commit that fix if the term-level divergence is unambiguous — not a ninth
   pure-diagnostic.
Split actionable-now from needs-more-evidence rather than holding all nine behind
the slowest window.

## Positives
- Dual review is back on (0298/0299/0300) and demonstrably effective.
- Acceptance discipline intact: no correlational acceptance, no false
  negative-melt-bug attribution, no downstream compensation.
- The system self-corrected the 0298 unit artifact within one package.

## Bottom line
0299 is the machinery self-correcting an over-read (mine included) of a unit
artifact; 0300 is disciplined and evidence-gated. But eight packages of
deepening diagnosis with a harness bug en route is enough — the next package
needs a forcing function: fix the H39 window already root-caused, instrument the
melt terms for the seven, and commit a fix. Watch for the next package being
pure-diagnostic again, or `INV-SNOWFREEZE-031` being used to defer the fix
indefinitely.
