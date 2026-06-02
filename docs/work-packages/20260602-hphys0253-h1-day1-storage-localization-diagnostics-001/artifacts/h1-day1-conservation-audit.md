# H1 Day-1 Conservation Audit

Status: complete

Evidence mode: ran

Ran:

- Trace rows:
  `/tmp/hphys0253_20260602T203448Z/hillslope_output/H1.hphys0253.trace.jsonl`.
- Formula: input minus `ET + Dp + latqcc + Q + delta-storage`.
- `ET` is `Ep + Es + Er`; candidate `delta-storage` is actual trace
  `post_wb13 wb11 - post_seed wb11`.

| Quantity | mm | Notes |
|---|---:|---|
| candidate actual `P` residual | -0.000000 | `P - (ET + Dp + latqcc + Q + actual ΔS)` |
| candidate actual `RM` residual | -0.000000 | `RM - (ET + Dp + latqcc + Q + actual ΔS)` |
| candidate inferred initial from `P` | 323.346740 | `S1 + ET + Dp + latqcc + Q - P` |
| candidate actual post_seed | 323.346740 | trace post_seed `wb11_soil_water` |
| candidate inferred minus actual | 0.000000 | WAT/trace day-1 accounting closes |
| baseline inferred initial from `P` | 343.500000 | baseline WAT-derived t=0 proxy |
| baseline inferred ΔS from `P` | -0.430000 | `S1 - inferred S0` |
| baseline inferred ΔS from `RM` | -0.430000 | `S1 - inferred S0` |

Interpretation:

- Candidate day-1 accounting is internally balanced to the reported precision.
- The storage deficit is therefore not a hidden WB13 publication imbalance.
- The actionable residual is the pre-scheduler candidate storage deficit
  relative to the baseline WAT-derived t=0 proxy.
