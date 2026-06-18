# PERFIDX06 Kickoff — High-OFE Target Assessment (Stage 6)

Execution mode: package-end-to-end (characterization / assessment — measures and decides,
lands no code).

Autonomy: execute end-to-end (endpoint pin → profiler re-run → wall-clock ladder →
legacy-ratio → bottleneck analysis → disposition) without asking for direction. This
package may legitimately conclude "≤10× is not reachable by more id-table work" — that is a
**successful** outcome, not a failure to push harder.

## The question

The whole perf program exists to answer: **how slow is openWEPP vs legacy now, and is ≤10×
(≤5×) reachable?** Measure the PERFIDX04 endpoint, compute the **actual legacy ratio**, find
the new dominant bottleneck, and dispose honestly.

## Set expectations — do not manufacture a pass

Prior data: PERFHO01 H2637 = **978.55 s** vs legacy **~9–12 s** (~80–110×). PERFIDX04
endpoint ≈ **673 s** → implied **~56–75×** now. Both **read-side** levers are captured;
PERFIDX05 showed the **write/guard** side is net-negative (dual-write ceiling). So this
package is expected to show ≤10× **not** closed — its job is to **quantify** that and decide
whether the gap is reachable incrementally or needs a redesign. A candid negative verdict
with "here's what would actually be required" is the win.

## The one real trap: apples-to-apples ratio

The ratio is the headline, so the legacy baseline must be **like-for-like**. The FARPOINT01
~9–12 s may be from different hardware. **Re-measure legacy on the same machine + H2637
fixture** if the pinned legacy WEPP binary is available; otherwise use FARPOINT01 with an
explicit cross-condition caveat and call the ratio approximate. Same fixture, same machine,
same wall-clock method on both sides — or the number is an artifact, not a result.

## Steps

1. **Pin the endpoint** — record the measured binary SHA; confirm it's the committed
   PERFIDX04 endpoint and `crates/` is clean (post-PERFIDX05-discard). No code change.
2. **Profiler re-run** — PERFHO02 `perf` flow on H2637; report the **new** hot-path split
   (PERFHO01 was runtime-surface map 53% / writeback 20% / hydrology-frost guards 27%;
   PERFIDX04 cut lookup/`format!` — show what dominates now).
3. **Wall-clock ladder** — OFE1-OFE5 + H2637 both `wepp_ui` variants; method + variance.
4. **Legacy ratio** — legacy H2637 (re-measured like-for-like, or FARPOINT01 + caveat) /
   openWEPP H2637. Report ratio + absolute gap.
5. **Bottleneck analysis** — what dominates now; is it reachable by continuing the
   indexed-surface migration, or only by a deeper change (fixed-index state arrays;
   removing the logical `BTreeMap` from the hot path)? Weigh the PERFIDX05 dual-write
   finding and the PERFIDX03 export-seam finding.
6. **Disposition** — ≤10× closed? ≤5× plausible? next concrete lever (continue / redesign /
   stop), with evidence.

## Constraints

- No production / `SC-*` / contract change — assessment only.
- Irrigation stays deferred/inert.
- Truthfulness: timings, profiles, ratio are empirical — label `Ran:`; attribute the
  legacy baseline source; state the like-for-like method or the caveat. The disposition is
  a judgment from evidence, not a target to hit.

## Required reading

- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/package.md`
- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/artifacts/perfho01-verdict.md`
  (978.55 s baseline; legacy ~9–12 s; ~80–110× gap; hot-path split + named hot paths)
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-profiler-evidence.md`
  (profiler flow)
- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/artifacts/{perfidx04-realized-speedup,perfidx04-profiler-evidence}.md`
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/{perfidx05_disposition,review-claude-independent}.md`
  (dual-write ceiling)
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- FARPOINT01 legacy baseline (pinned legacy WEPP binary + H2637 fixture)
- `docs/numerics/README.md`, `AGENTS.md`, `docs/work-packages/AGENTS.md`
