# Diagnostic — Total-Soil Over-Drainage Root Cause (WB18 Deep Percolation)

Author: Claude Code
Date (UTC): 2026-06-01
Status: diagnostic — single-hillslope trajectory attribution of the
`Total-Soil`/`SoilWaterTotal` over-drainage residual. Names the mechanism and
the kernel lane; does not prescribe a fix.
Evidence: **Ran** (duckdb over the candidate and legacy baseline parquet for H1).

---

## 1. Result

The `Total-Soil` over-drainage (39/39 fail, mean-abs-diff ~140.7 mm, frozen
across HPHYS0223/0224/0229) is caused by the candidate **deep-percolation (`Dp`)
kernel over-percolating in the early simulation days**: it drains the profile
from ~250 mm to its cutoff (~77 mm) in the first ~6 days at `Dp` of 40–82 mm/day
with near-zero precipitation, then `Dp` → 0 and the soil sits at ~77 mm. Legacy
percolates a steady ~0.24 mm/day and retains ~340 mm. The mechanism is in the
WB18 / deep-percolation lane (`Dp`, the daily water-balance `D` term), **not
WB19** (lateral/drainage).

## 2. Evidence — H1 daily candidate vs legacy (year 1)

| day | J | P | Q | Dp cand | Dp legacy | Total-Soil cand | Total-Soil legacy |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 0.0 | 0.0 | **39.53** | 0.24 | 254.0 | 343.1 |
| 2 | 2 | 0.9 | 0.0 | **82.52** | 0.24 | 171.2 | 342.5 |
| 3 | 3 | 0.8 | 0.0 | **47.99** | 0.24 | 123.1 | 341.7 |
| 4 | 4 | 1.1 | 0.0 | 14.62 | 0.24 | 109.0 | 340.9 |
| 5 | 5 | 0.5 | 0.0 | 25.59 | 0.24 | 82.8 | 340.1 |
| 6 | 6 | 3.0 | 0.0 | 5.11 | 0.24 | 78.3 | 339.2 |
| 7+ | … | … | 0.0 | **~0.0** | 0.24 | **~77** | **~340** |

Year-1 aggregates (H1): candidate **ΣDp = 216 mm** vs legacy **78 mm**; candidate
**max daily Dp = 82.5** vs legacy **0.24**; candidate Total-Soil min/avg = 68/75 mm
vs legacy 36/247 mm. Worst-row (HPHYS0229 semantic `top_divergent_rows`, J=145):
Total-Soil legacy 645.6 / candidate 71.8; `Dp` legacy 0.24 / candidate 0.0.

Behavioral signature: candidate percolation is a **fast drain-to-field-capacity**
model (large `Dp` until soil reaches ~FC cutoff, then 0); legacy is a **slow
rate-limited seepage** (~0.24 mm/day, soil persistently far above FC).

## 3. What this explains

- **Why WB19 packages moved nothing.** HPHYS0224–0228 (WB19 soil-water cap,
  available-pool, FC/WP coca/watyld authority) changed `Total-Soil` by exactly
  0.000 mm (HPHYS0229 rerun, independently re-counted). The defect is WB18
  percolation; WB19 is the wrong lane. HPHYS0229's null result ruled WB19 out;
  this attribution names the correct lane.
- **Why `Dp` looked near-closed.** The reported `Dp` mean-abs-diff (0.325) is
  averaged over 1,461 days; the divergence is concentrated in ~6 days/year
  (40–82 vs 0.24 mm/day) and ~0 elsewhere. A large, physically-decisive
  divergence hidden by a small mean — the same summary-statistic trap as the FC
  mean.
- **Why FC was a false lead.** The soil is not drying because field capacity is
  low; it is drying because percolation *rate* is ~100–400× too high in the
  early transient. FC sets where percolation stops (~77 mm); the rate is the
  defect. (See the withdrawn AUTH05 FC worked example.)

## 4. Lane (finding) and open questions (not prescriptions)

Lane: WB18 deep percolation (the `Dp`/`D` term; `wb18_perc_*` state and the
percolation kernel). Solid and evidenced: candidate `Dp` is 2–3 orders of
magnitude too high in the early-transient days; legacy's ~0.24 mm/day is the
target the contract requires matching.

Not asserted here (the FC lesson — verify before remediating):
1. Is legacy's steady ~0.24 mm/day the `perc.for`/WB18 contract behavior openWEPP
   must replicate, or itself an artifact? (Confirm against the baseline
   percolation routine.)
2. What drives the candidate burst — an unrate-limited free-drainage formula, a
   too-high effective Ksat/drainage coefficient feeding percolation, or a
   different percolation model than legacy? (WB14 `ksatadj`, which 0228 touched
   in tests only, feeds percolation rate and is adjacent.)
3. Is initial soil water also offset (day-1 candidate 254 vs legacy 343)? Small
   relative to the percolation burst, but present.

## 5. Validation criterion for remediation

A correct WB18 percolation fix must, on H1, **collapse early-day `Dp` from
40–82 mm/day toward legacy's ~0.24 mm/day** (ΣDp year-1 from 216 → ~78 mm), which
should hold `Total-Soil` near legacy (~340 mm) instead of draining to ~77 mm.
This H1 `Dp` trajectory is the acceptance trace; it does not require the legacy
binary as authority beyond the matched-contract question in §4(1).

## 6. Reproduction

```
CAND=/tmp/hphys0229_20260601T175346Z/parity/hillslope_output/H1.wat.parquet
BASE=/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet
# candidate uses column "Total-Soil"; baseline uses "Total-Soil Water"; align by sim_day_index.
duckdb "select sim_day_index, julian, P, Q, Dp, \"Total-Soil\" from '$CAND' order by sim_day_index limit 14"
duckdb "select sim_day_index, julian, P, Q, Dp, \"Total-Soil Water\" from '$BASE' order by sim_day_index limit 14"
```

Baseline source recorded in each `reports/semantic/H*.semantic.json` `inputs.baseline_wat`.
