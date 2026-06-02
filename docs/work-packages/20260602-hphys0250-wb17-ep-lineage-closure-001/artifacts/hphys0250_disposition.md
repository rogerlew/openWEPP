# HPHYS0250 Disposition

Status: HOLD

Evidence mode: static + ran

Decision:

- HOLD. HPHYS0250 corrected the missing/zero `Ep` lineage class and passed
  code/contract gates, but full `H1..H39` semantic parity remains `0/39`.

What closed:

- PL scheduler activation is preserved for management-present runs.
- Established perennial `jdplt=0` slots dispatch under baseline-compatible
  `ptgrp` semantics.
- H1-style initial live canopy now seeds live PL state and nonzero root depth.
- WB11 growth/decomposition transitions publish scheduler-computed state.
- WB13 final `Ep` publication consumes post-WB19 flux-surface `Ep`.
- WB15 near-zero `I`/liquid roundoff is canonicalized before writeback while
  material negatives remain typed failures.

What remains open:

| Family | Status | Evidence |
|---|---|---|
| `Ep` | open | `0/39`, fail count `56230`, mean abs mean `1.683414`, max `7.778432` |
| `Total-Soil` / `SoilWaterTotal` | open | `0/39`, mean abs mean `168.130627`, max `619.184688` |
| `Snow-Water` | open | `0/39`, mean abs mean `58.195696`, max `562.470000` |
| `RM` / `Q` | open | `RM 0/39`, `Q 0/39`, H6 worst storm/runoff timing residual |
| `Dp` | open | `0/39`, mean abs mean `0.171118`, max `0.240000` |
| `Es` | nearly closed | `38/39`, fail count `1165`, worst H6 |

Promotion rule:

- Do not mark HPHYS0250 `GO`; unresolved semantic residuals are known
  contract and continuation blockers.
