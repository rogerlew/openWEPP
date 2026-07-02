# Rubric Verdict — the Acceptance Bar (operator-ratified)

Evidence class: Ran (both captures executed by this package; before at branch
`39061021` in `rubric-before/`, after on the single-solve build in
`rubric-after/`; same harness, same observation corpus, same five sites).

## Side-by-side (classify_residuals)

| Site | Cell | Before | After | Reading |
|---|---|---:|---:|---|
| site1 Sleepers South Field (frost tube) | max abs frost residual | 0.24687 m | 0.24663 m | better |
| site1 | matched/residual/snow counts, mean signed snow | 392/392/218, 0.156069 | identical (0.156067) | unchanged |
| site2 Sleepers W9 (frost tube) | max abs frost residual | 0.390314 m | 0.390296 m | ~unchanged |
| site2 | all counts | identical | identical | unchanged |
| site3 SCAN Mandan (isotherm) | isotherm exceedances | 3,658 | 3,715 | **worse (+57 / 10,643 rows)** |
| site4 GGD498 Morris (frost tube) | max abs frost residual | 0.78686 m | 0.73763 m | **better (−4.9 cm)** |
| site4 | all counts | identical | identical | unchanged |
| site5 Reynolds Creek (isotherm) | isotherm exceedances | 125 | 125 | unchanged |
| all | primary classifications / families | 3× SNOW-CONTROL-FAILED, 2× INCONCLUSIVE | identical | unchanged |
| all | defect-eligible / OPENWEPP-DEFECTIVE | 0 / 0 | 0 / 0 | unchanged |

Snow-side columns (pairs, failures, signed bias, over/under, SWE-alias) are
identical on every site — an internal control confirming the change is
isolated to the frost trajectory (the snow model was untouched).

## Verdict

**No-worse holds in every verdict-bearing cell — the bar passes.** The three
frost-tube magnitude sites (the depth-magnitude authority per
INV-SNOWFREEZE-047(a)) improved (site4, −4.9 cm max residual; site1
marginally) or held (site2). The single adverse cell is Mandan's isotherm
upper-bound exceedance count (+57, a +1.6% relative increase over a
10,643-row record) at a site the rubric routes INCONCLUSIVE for magnitude
(no paired observed snow rows; INV-SNOWFREEZE-047(b)/-048): that cell is an
upper-bound/timing surface that cannot carry a defect verdict under the
ratified taxonomy. It is recorded here as a bounded residual delta for
review rather than silently absorbed.

## Endpoint and closure evidence (same build)

- Full workspace suite 1283/1283 (the seven snowbench sentinels that caught
  the interim mis-shape included).
- H2637 completes with all in-run conservation/closure guards green:
  **32.54 s, 76,176 KiB** (single rep; ≈3.4× the 9.65 s quiet legacy anchor,
  from 71.4 s at the package's start-of-day baseline).
- First divergence vs the WP-1 protected baseline: last-bit `Q` at
  year 1 / OFE 2, broadening as the single frost trajectory propagates
  (235,939 / 235,961 WAT rows differ somewhere; `frdp` max delta 0.244 m,
  consistent with the Stage-1 paired-solve divergence scale of 0.245 m).

## Exit timing and re-profile (quiet window, load 1.6–3.5)

| Measure | Value |
|---|---|
| H2637 3-rep | 33.74 / 32.77 / 32.58 s (median **32.77 s**) |
| Same-window legacy `wepp_260430_hill` | 9.30 s |
| **Ratio** | **3.52×** |
| Max RSS | ~76.6 MiB |

Day's arc on the same fixture and host: 71.4 s (7.40×) at the program's
start → 46.69 s (4.80×) after WP-1 → **32.77 s (3.52×)** after WP-2 —
under the 45.6 s `<=5x` viability budget with ~30% margin.

Exit re-profile (perf-wp2exit.data): the double-solve structure is gone —
`compute_direct_winter_frost_partition` shows one caller at 4.0% self;
frost hourly machinery 8.3% (was 11.7%); the distribution is now
physics-dominated (percolation 4.3%, lateral 3.3%, Harder–Pomeroy phase
4.0%, transcendentals ~4%). Top remaining non-physics candidates for any
future work: `round` inside `derived_frost_depths_from_fine_state`
(3.7% — the F4 target, unchanged in this package) and memmove 6.1%.
