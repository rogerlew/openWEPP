# PERFIDX05 — Independent Review (Claude Code)

Verdict: **Correct but performance-NEGATIVE — should NOT land as a completed Stage 5.**
The code is bit-identical (I reproduced it), the failure-path tests are genuine, the
irrigation carve-out held, and Codex correctly *stopped* at the prefix→range trap I
flagged. But it regresses H2637 **−5.3–5.8%** (and OFE2–5 −4.4–8.6%), eroding part of
PERFIDX04's +24%. A perf-optimization package that regresses perf has not met its
objective. **Recommend HOLD.**

Evidence mode: **Static** (full +908/−115 diff + artifacts) + **Ran** (my own OFE2
bit-identity re-run). Anchor/gates/timings otherwise Codex's runs.

## The structural finding (this is the important part)

PERFIDX05 confirms a **ceiling in the current architecture**, and it's worth stating
plainly because it should steer what comes next:

- **PERFIDX04 (reads) won (+24%)** because a read just reads the indexed mirror; the
  logical `BTreeMap` is kept in sync cheaply off the hot read path.
- **PERFIDX05 (writes/guards) loses (−5.7%)** because every writeback / transfer / guard
  must **mutate *both*** the authoritative logical `BTreeMap` **and** the indexed mirror
  to keep them consistent. That **dual-write cost exceeds the id-lookup saving** on the
  write/guard side (Review-B reaches the same conclusion independently).

This dual-write cost is *inherent* to the "logical-authoritative + non-authoritative read
mirror" design that PERFIDX04 established. The only way to remove it is to make the
indexed surface **authoritative** (drop the dual-write) — which is exactly the PERFIDX03
authority flip that regressed for a *different* reason (the full-`BTreeMap` export seam).
So the program is now pinched between two designs, each with its own cost, and **PERFIDX04
appears to have captured most of the win available under the current one.**

## Verified

1. **Bit-identity — I reproduced it.** OFE2 (exercises the transfer mirror-sync path),
   baseline `82c6cac7` (PERFIDX04) vs current `4eebabb5` (PERFIDX05): `H1.hbp`,
   `loss.json`, `wat.parquet`, `plot.parquet` byte-equal; `pass.parquet` expected churn.
   Matches Codex's 7-case anchor. ✓
2. **The prefix→range trap I flagged is the residual blocker — correctly stopped at.**
   `ensure_no_overflow_indexed_symbols_for_decomposition` (`07_decomposition_equations.rs`)
   still uses the logical `strip_prefix` scan because converting it to an id-range needs
   the interloper-proof that the range is exactly the `pl_decomp_slot_*_crop_*_<root>_<NNNN>`
   set. Codex did **not** ship an unsafe conversion — the hard stop worked as intended. ✓
3. **Failure-path tests are genuine** (the Stage-5 load-bearing gate): unknown-symbol
   rejected-before-mutation with logical name; applied-symbol vectors stay logical-sorted
   while application is id-ordered; indexed consumer-boundary reports the same missing
   symbol as the logical path. Missing/non-finite/out-of-range covered via exercised
   existing negative tests. ✓
4. **Irrigation: clean.** Zero `irrig` in the diff; Review-B confirms no irrigation
   surface activated. ✓
5. **Correctness of the by-id apply:** resolves all writeback fields *before* mutation, so
   an unknown symbol fails closed with no partial application. ✓

## Why it regressed (not noise, not just incompleteness)

Consistent negatives across every multi-OFE case (OFE1 neutral) → real. Two compounding
causes: (a) the **dual-write cost** above (structural), and (b) the one scan whose removal
could have paid — the decomposition overflow scan — is **trap-blocked**, so the package
paid the cost without collecting the payoff. Even unblocking (b) would only be worth it if
decomposition-scan removal beats the dual-write it adds, which is unproven.

## Recommendation

**HOLD PERFIDX05 — do not land the regression**, and **pivot to `PERFIDX06` (re-measure)
before investing further.** Rationale:

- The whole program chases ≤10×. We have not measured where PERFIDX04 actually put us.
  If PERFIDX06 shows we're already at/near ≤10×, the entire write/guard-side squeeze is
  moot and we stop. Measure before grinding.
- Landing PERFIDX05 would slow the codebase ~5.7% and contaminate the PERFIDX06 baseline.
  PERFIDX06 should measure the **best** state (PERFIDX04), not a regressed one.
- The PERFARCH01 Stage-5 premise ("write/guard by-id is a win") is **undercut** by this
  evidence. If PERFIDX06 says we still need more, the next lever is a deliberate
  re-design choice (decomposition-scan-with-proof *iff* it beats dual-write, or revisiting
  indexed-authoritative without the export seam) — not "finish Stage 5 as specified."

The PERFIDX05 failure-path tests test the indexed wiring, so they cannot be salvaged
independently of the (regressing) code — this is discard-or-keep-whole, like PERFIDX03.
The durable value is this structural finding, captured here.

## Disposition options (operator's call)

- **(A, recommended)** Discard the PERFIDX05 code (keep this record + artifacts as the
  HOLD record), pivot to PERFIDX06 re-measure.
- **(B)** Keep the code uncommitted/held while PERFIDX06 runs, decide after the verdict.
- **(C)** Continue Stage 5 (PERFIDX05B: prefix-range proof + decomposition-scan migration)
  — higher risk, uncertain it flips net-positive given the dual-write cost.

## Decision taken (2026-06-18, operator: A)

Option A. All PERFIDX05 working-tree code discarded; `crates/` returns to the PERFIDX04
state; this record is docs-only. Next: scaffold **PERFIDX06** (re-measure vs ≤10×) — the
finding above (dual-write ceiling; PERFIDX04 likely captured most of the available win)
is the durable output and feeds the PERFIDX06 disposition.
