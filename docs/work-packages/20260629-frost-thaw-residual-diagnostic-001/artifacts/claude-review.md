# Claude Code Review — Frost Thaw-Residual Diagnostic

**Reviewer:** Claude Code · **Date:** 2026-06-29
**Evidence mode:** Static + Ran (I read the diagnostic + analyzer, and ran probes over
`thaw_residual_buckets.json` daily rows; the harness run is Codex's).

## Verdict: H2 rejection ACCEPTED; the H1a → Qwet routing is NOT supported

Two parts land differently:

- **ACCEPT** — the tiny-tail (H2) rejection is solid. The threshold sweep is clean
  (0 H2 cells up to 0.05 m; max frdp 0.07–0.20 m per cell), so the remaining frost is
  genuinely **material over-persistence**, not a detector artifact. This correctly
  overturns the earlier tiny-tail suspicion.
- **REJECT (as routed)** — "9 H1a → Qwet-dominant" overstates the missing-wet-heat
  case ~3×. The classifier bucketed on surface temperature + warm/wet + retreat, but
  **did not use snow depth**, which it captured (`max_snow_depth_m`) but never consumed
  in `classify_thaw`. The per-day data shows the "cold surface" is dominantly
  **over-deep snow insulation**, not a missing energy term.

## Finding 1 — The H1a "cold surface" is over-deep snow, not missing wet heat

On the warm/wet + `max_surface_temp_c ≤ 0` + material-frost days (the exact rows the
H1a-cold-surface branch fires on), the daily trajectory shows:

| Cell | bucket | stalled days | snow depth on those days (m) | surface heat flux | runoff |
| --- | --- | ---: | --- | --- | ---: |
| W9 1994 | H1a | 32 | **0.394 – 1.035** | 0–3 W/m² | 0 |
| W9 1997 (+84 d) | H1a | 54 | **0.193 – 1.140** | ~0 | 0 |
| W9 2004 | H1a | 25 | **0.432 – 0.909** | 0–3 | 0 |
| W9 2009 (+111 d) | H1a | 63 | 0.043 – 1.125 | ~0 | 0 |
| W9 2011 | H1a | 17 | **0.485 – 0.918** | 0–3 | 0 |
| W9 1996 | H1a | 38 | 0.143 – 0.681 | 0–4 | 0 |
| South 1986 | H1a | 10 | 0.000 – 0.508 | ~0 | 0 |
| South 1987 | H1a | 14 | 0.000 – 0.542 | ~0 | 0 |
| W9 2006 | H1a | 9 | 0.000 – 0.259 | 0–3 | 0 |

The surface is pinned at 0 because it is buried under **up to a meter of modeled snow**;
the surface heat flux is **~0–5 W/m²** (the snowpack chokes conduction), and **runoff =
0** — the warm rain/melt is being absorbed/refrozen in the snowpack, not reaching the
soil. **This is over-insulation. `Qwet` cannot move these cells** because the energy is
blocked by snow before any advective term could act.

## Finding 2 — This routes back to the forcing-limited snow, and undercuts the step-2 sign premise

- The W9 snow is **modeled-over-observed and forcing-limited** (step 1). So the
  snow-buried thaw-late cells are the **downstream consequence of over-deep snow** —
  whose magnitude we established is unresolvable from current authority — **not a
  frost-model defect.**
- It also **retroactively undercuts step 2's sign-coherence assumption.** Step 2
  encoded *deeper snow → earlier thaw* (so late-thaw = candidate defect). The daily
  data shows the opposite in spring: *deeper snow → surface frozen → frost persists →
  later thaw.* These late-thaw cells are therefore partly **snow-forcing**, not cleanly
  frost-model defects. (This does not invalidate step 2 wholesale — the early-onset and
  snow-free cells stand — but the snow-buried late-thaw cells were mis-premised.)

## The honest split (vs the reported 9 H1a)

- **~6 of 9 are snow-buried** — W9 1994/1996/1997/2004/2011 never drop below ~0.14–0.49 m
  snow on the stalled days. Persistence is snow-insulation-driven → route to the snow
  question (Finding 3), not Qwet.
- **~3 have a genuine snow-free component** — South 1986/1987 and W9 2006 reach snow = 0
  on stalled days with frost still persisting → the *real* `Qwet` / missing-thaw-energy
  candidates.
- The 2 H1b cells (surface reached 19–22 °C, no retreat) are genuine and unaffected.

## Finding 3 — We must identify WHY the modeled snow persists (operator, 2026-06-29)

The over-deep snow is the actual lever for most of these cells, so the next question is
**why the modeled snow does not melt off** when it should (the real frost tube thawed —
e.g. W9 2009 observed thaw 2008-12-17 — while the model holds ~1 m of snow into April).
This is decisive because it determines what the residual *is*:

- **(a) over-accumulation (magnitude):** too much snow input — forcing-limited
  (undercatch/lapse), unresolvable from current authority. Frost over-persistence is
  then a forcing artifact, not fixable.
- **(b) spring under-melt (rate):** the melt model ablates too slowly in
  late-winter/spring, so snow lingers. This is a **fixable melt-rate defect** — and it
  connects to the known **spring-melt-realization residual** (SNOWDENSITY-10.3.8
  localized maritime over-accumulation to the Feb–May melt season, canopy-independent).
  If this is operative at Sleepers, **the frost thaw-late residual and the snow
  spring-melt residual are the *same* defect** — and fixing spring melt would help both,
  with no new frost physics.
- **(c) genuinely missing Qwet:** only for the snow-free-persistent subset.

This reframes the program: before any `Qwet` build, decompose the snow persistence into
accumulation vs melt-rate, because (b) is in-scope, dual-benefit, and far cheaper than a
new advective-heat term — and (a) means the cell is not actionable at all.

---

## Recommended diagnostic amendment (cheap, diagnostic-only — not a Qwet build)

Re-run over the existing `thaw_residual_buckets.json` daily rows; no solver/contract/
fixture change.

1. **Snow-depth-controlled re-bucketing.** For each thaw-late cell, on its
   warm/wet + surface ≤ 0 + material-frost days, classify:
   - **SNOW-BURIED** — snow depth above a control threshold (sensitivity-swept, e.g.
     0.05 / 0.10 / 0.20 m; do **not** tune to a result) with surface heat flux ≈ 0 →
     persistence is snow-insulation-driven → route to step 2 below, **not** Qwet.
   - **SNOW-FREE-PERSISTENT** — frost survives warm/wet days at/near zero snow → the
     genuine `Qwet` / missing-thaw-energy candidate.
   Report the per-cell split + the threshold sensitivity (expect ~6 buried / ~3 free).

2. **Snow-persistence decomposition (the WHY).** For the SNOW-BURIED cells:
   - **Modeled vs observed snow depth** on the stalled days (W9 and South are paired
     snow-depth sites — step 1 had 193 and 384 paired rows). Quantify the
     over-prediction.
   - **Accumulation vs melt-rate:** decompose the modeled snow-water trajectory
     (`snow_water_m`, `snow_water_delta_m`) over the carried period — is snow still
     being *added* (over-accumulation, route (a)) or *failing to ablate* in Feb–May
     (under-melt, route (b))? Report the melt-season (Feb–May) snow-water loss rate vs
     observed.
   - **Connect to the spring-melt residual** (SNOWDENSITY-10.3.8 Feb–May
     over-accumulation): is the same under-melt signature present at Sleepers?
   Output: per-cell route (a)/(b)/(c) with the accumulation-vs-melt evidence.

3. **Routing implication.** Only the SNOW-FREE-PERSISTENT cells (≈2–3) justify the
   `Qwet` investigation. The SNOW-BURIED/under-melt cells route to the **snow
   spring-melt** work (dual-benefit, in-scope); the SNOW-BURIED/over-accumulation cells
   are forcing-limited (report-only). Update the GAP-SNOWFREEZE-002 disposition with the
   corrected split and *do not* carry "H1a-dominant → Qwet" forward unamended.

## Discipline

Diagnostic-only; no solver, detector-threshold, contract, default, or fixture change.
Do not adopt a snow-depth control threshold or a thaw threshold by fitting it to the
residual — the snow-depth split must be reported as a sensitivity sweep, and any
eventual threshold anchored to physical reasoning, not the cell count.
