# PERFIDX06 - High-OFE Target Assessment (Stage 6)

Status: executed 2026-06-18 (Stage 6 — final assessment — of the PERFARCH01 staged
plan; follows PERFIDX04 complete + PERFIDX05 held)

Package type: **Characterization / assessment — no production or contract change.** Like
PERFHO01/PERFHO02: this package measures and decides; it lands no code. Truthfulness:
every timing/ratio/profile is empirical — label `Ran:`; the disposition is an evidence-based
judgment, not a target to satisfy.

## Objective

Answer the question the whole perf program was opened to answer: **how slow is openWEPP vs
legacy now, and is the ≤10× (≤5×) target reachable?** Re-measure the PERFIDX04 endpoint,
compute the **actual legacy ratio**, identify the new dominant bottleneck, and dispose:
is ≤10× closed, is ≤5× plausible, and — given the PERFIDX05 finding — is the remaining gap
reachable by more incremental id-work or does it need a redesign?

## What we already know (set expectations honestly)

This package is **not** expected to show ≤10× closed. The arithmetic from prior packages:

- **PERFHO01 baseline:** H2637 = **978.55 s** (19-OFE, 34-year); legacy FARPOINT01
  substrate **~9–12 s** → the **~80–110×** gap.
- **PERFIDX04 endpoint:** H2637 ≈ **673 s** (the cumulative PERFOPT01 + PERFIDX03B clone +
  PERFIDX04 lookup wins).
- Implied current ratio: **673 / (9–12) ≈ 56–75×.** Real progress (978→673, ~31%), but
  still an order of magnitude above ≤10×.

So the honest framing: both **read-side** levers are captured, and **PERFIDX05 showed the
write/guard side is net-negative under the read-mirror design (dual-write ceiling)**. This
package quantifies where that leaves us and whether incremental id-work can ever reach the
target or a different architecture is required. **Do not manufacture a pass.**

## The measurement must be apples-to-apples (the one real trap)

The ratio is the headline number, so the legacy baseline must be **like-for-like**. The
FARPOINT01 ~9–12 s figure may be from different hardware/conditions; a cross-hardware ratio
is a measurement artifact, not a result. **Re-measure legacy on the same machine + fixture**
as the openWEPP endpoint if the pinned legacy WEPP binary is available (the FARPOINT01 /
`wepp_260606`-class baseline); if it is not, use the FARPOINT01 figure but state the
cross-condition caveat explicitly and treat the ratio as approximate. Same fixture
(H2637), same machine, same wall-clock method for both sides.

## Scope

In scope (measurement + analysis only):

1. **Pin the endpoint** — confirm the measured binary is the committed PERFIDX04 endpoint
   (record its SHA; `crates/` is at the post-PERFIDX05-discard state). No code change.
2. **Profiler re-run** — re-run the PERFHO02 `perf` flow on H2637 at this endpoint; report
   the **new** hot-path split (PERFHO01 was runtime-surface map 53% / writeback 20% /
   hydrology-frost guards 27%; PERFIDX04 cut the lookup/`format!` — show what dominates now).
3. **Wall-clock ladder** — OFE1-OFE5 + H2637 (both `wepp_ui` variants), same method as
   PERFIDX04's realized-speedup table.
4. **Legacy ratio** — legacy H2637 (re-measured like-for-like, or FARPOINT01 with caveat) /
   openWEPP H2637. Report the ratio and the absolute gap.
5. **Disposition** — is ≤10× closed? is ≤5× plausible? what is the next non-storage
   bottleneck, and is it reachable incrementally or only by redesign?

Out of scope:

- No production / `SC-*` / contract change (assessment only).
- No new optimization implementation (that is whatever package the disposition recommends).
- Irrigation remains deferred/inert.

## The disposition must weigh the PERFIDX05 finding

The decision is the deliverable, and it must account for what PERFIDX03–05 established:

- **Read-mirror design:** reads are cheap (PERFIDX04 won), writes/guards are dual-write-bound
  (PERFIDX05 lost). Incremental id-work on the write side is net-negative here.
- **Indexed-authoritative design:** removes dual-write but reintroduces the PERFIDX03 export
  seam unless redesigned.
- The remaining ~56–75× is dominated by the **symbol-keyed runtime-surface architecture**
  itself vs legacy's fixed arrays. The disposition should state plainly whether the target
  is reachable by continuing the indexed-surface migration, or whether it requires a deeper
  change (e.g. fixed-index state arrays, eliminating the logical `BTreeMap` from the hot
  path entirely), and what the realistic ceiling of the current approach is.

A candid "≤10× is not reachable by more id-table work; here is what would be required"
**is a successful outcome** for this package. So is "≤10× is closer than expected; here is
the next concrete lever." The failure mode is a vague or optimistic disposition.

## Acceptance Criteria

- **Endpoint pinned:** measured binary SHA recorded = committed PERFIDX04 endpoint; no
  uncommitted code.
- **Profiler artifacts:** `perf` report at the endpoint with the new hot-path split.
- **Timings:** OFE1-OFE5 + H2637 both variants, method stated, run-to-run variance noted.
- **Legacy ratio:** computed with the baseline source + like-for-like method (or explicit
  caveat) stated.
- **Determinism note:** the endpoint binary is the within-config-deterministic PERFIDX04
  build (no behavior change introduced by this package).
- **Disposition:** ≤10× verdict + ≤5× plausibility + next-bottleneck + reachable-incrementally
  -or-redesign judgment, weighing the PERFIDX05 dual-write finding.
- Markdown lint clean. (No Rust gates — no code change.)

## Deliverables

- `artifacts/perfidx06-endpoint-pin.md` (binary SHA, tree state)
- `artifacts/perfidx06-profiler-evidence.md` (endpoint hot-path split)
- `artifacts/perfidx06-wallclock-ladder.md` (OFE1-5 + H2637)
- `artifacts/perfidx06-legacy-ratio.md` (the ratio + baseline method/caveat)
- `artifacts/perfidx06-bottleneck-analysis.md` (what dominates now; reachable vs redesign)
- `artifacts/perfidx06_disposition.md` (≤10× / ≤5× verdict + recommended next move)

## Dependencies

- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/artifacts/perfho01-verdict.md` (978.55 s baseline; legacy ~9–12 s; ~80–110× gap; hot-path split)
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-profiler-evidence.md` (profiler flow + prior hot paths)
- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/artifacts/{perfidx04-realized-speedup,perfidx04-profiler-evidence}.md` (endpoint timings + method)
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/{perfidx05_disposition,review-claude-independent}.md` (the dual-write ceiling finding)
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- FARPOINT01 legacy baseline (the pinned legacy WEPP binary + H2637 fixture)
- `docs/numerics/README.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the profiler-report analysis (reading
`perf` reports for the hot-path split) is parallelizable. Run all timings locally and record
command evidence.

## Autonomy

Execute end-to-end through endpoint pin, profiler re-run, wall-clock ladder, legacy-ratio
measurement, bottleneck analysis, and disposition. This package may end on a candid "target
not reachable incrementally" verdict — that is a valid, successful closure. Record the
recommended next move (continue / redesign / stop) with its evidence.
