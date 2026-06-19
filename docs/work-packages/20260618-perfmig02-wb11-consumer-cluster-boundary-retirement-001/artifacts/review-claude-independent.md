# PERFMIG02 — Independent Review (Claude Code)

Verdict: **REDIRECT confirmed — honest, well-evidenced, and decisive.** This is the gate working as
designed: the hardened "measured endpoint improvement" criterion caught a non-converting tactic before it
accreted across the codebase. Two consecutive net-negative rungs falsify the **widen-first** strategy (not
the floor — PERFARCH03's floor stands). Pivot to the deep cut.

Evidence mode: **Static** (read disposition + reader-map + endpoint-timing + the production diff before it
was reverted) + **Ran** (`cargo check -p openwepp-kernel-contract` post-revert: clean).

## Why the REDIRECT is correct

- **The endpoint failed the gate, measured twice.** 672.14 s and 675.00 s vs PERFMIG01 669.97 s
  (+0.32% to +0.75%), 73.46× → 73.70–74.01×. Both final-code runs negative. ✓
- **The attribution subgate failed for a *structural* reason, not a tuning miss.** Retiring six symbols'
  materialization cost **more** (`apply` 105.46 µs vs materialize-all 104.75 µs) because the fail-closed
  stale-logical removal outweighs six avoided inserts. This is the **PERFIDX05 dual-write ceiling in a new
  guise**: maintaining two representations consistently during a partial migration costs more than it
  saves. ✓
- **The reader-map exposes the wall.** Of the 543+8 symbols, only **six** were internal-only and safely
  retireable; the rest are consumed by publication/reporting/diagnostic boundaries that still require
  logical materialization. So the widen-first path can only ever pick off a handful per rung while paying
  full dual-representation bookkeeping — it **cannot** reach the bulk incrementally. ✓
- **Identity preserved** (HBP/WAT byte-identical, PASS Arrow-equal) — the experiment was *correct*, it just
  didn't *convert*. A clean falsification, not a bug. ✓
- **Honest disclosure noted:** a pre-clippy intermediate measured 656.09 s but was superseded; Codex
  correctly dispositioned on the final gate-clean binary (672–675 s), not the faster non-compliant
  intermediate. That is the right call — you cannot bank a win from a binary that fails gates. (The 656 s
  hint is worth remembering as weak evidence that dense-first reads *can* help once the bookkeeping is
  gone — which is exactly what the deep cut removes.)

## The strategic lesson — why incrementalism is dominated, and what wins

Two rungs now establish the shape of the problem with arithmetic that should drive the pivot:

- **Writeback-only / partial-retirement is net-negative by construction.** Each rung adds (or keeps)
  dense↔logical bookkeeping (materialize, stale-removal) for a *small* slice of the work, capturing far too
  little internal-compute win to beat the ~108 µs boundary it carries.
- **Only the deep cut clears the bar.** PERFARCH03's numbers: a *fully* array-native branch (dense
  read+compute+write) is 0.96 µs vs the 140.83 µs logical kernel — a ~139 µs/branch internal-compute win.
  A complete phase migrated whole carries one ~108 µs edge boundary, so **139 − 108 ≈ +31 µs/branch net
  positive** even at one phase, *and* it eliminates the per-symbol/per-phase bookkeeping that sank
  PERFMIG01/02. Captured-win must exceed boundary-cost; only migrating a **complete unit** does that.
- **ADR-0023 is not refuted — its *incremental application* is.** Dense-array authority is still the
  destination; the lesson is migrate by **complete unit** (phase, or contiguous chain), not symbol-by-symbol.

## Disposition

REDIRECT. The production code was reverted per operator direction (it regressed +0.5% and implemented the
abandoned tactic); `main` stays at the clean PERFMIG01 baseline; the artifacts are the durable output. The
next package is **PERFDEEP01** — productionize the PERFARCH03 shape: one **complete** unit (the warm-rain
runoff phase, fully array-native read+compute+write), logical materialized **once** at the unit edge(s),
**measured** for a net endpoint win. If a single phase's two edge boundaries (~216 µs) exceed its internal
win (~139 µs), extend to a **contiguous fully-array-native chain** so internal boundaries vanish and only
the chain's outer edges materialize. The gate remains a measured endpoint improvement — but this time the
arithmetic predicts a win, because the internal-compute lever is finally pulled.
