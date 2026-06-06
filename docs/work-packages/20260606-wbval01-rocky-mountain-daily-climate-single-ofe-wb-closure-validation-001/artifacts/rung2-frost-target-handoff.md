# Rung-2 Frost Target Handoff

Status: complete

Evidence mode: Ran

Ran:

WBVAL01 names frost as the next mechanism rung and keeps the HPHYS0298->0320
snow/`RM` comparator route suspended behind
`docs/backlog/20260605-snow-code-deferred-science-review.md`.

Prioritized conservation-break targets from emitted WAT ledgers:

| Priority | Prefix | Max abs full-year R (mm) | First break year | Dominant magnitude pattern |
|---:|---|---:|---:|---|
| 1 | p19 | 78.740 | 2 | Ep-dominant |
| 2 | p22 | 61.861 | 2 | Ep-dominant |
| 3 | p16 | 58.961 | 2 | mixed Ep/latqcc |
| 4 | p5 | 58.848 | 2 | latqcc early, Ep in year 5 |
| 5 | p21 | 54.229 | 2 | Ep-dominant |
| 6 | p3 | 51.689 | 2 | latqcc early, Ep in year 5 |
| 7 | p8 | 51.440 | 2 | latqcc early, Ep in year 5 |
| 8 | p15 | 51.307 | 2 | Ep-dominant |
| 9 | p10 | 49.839 | 2 | Ep-dominant |
| 10 | p12 | 49.774 | 2 | Ep-dominant |
| 11 | p1 | 49.153 | 2 | latqcc early, Ep in year 5 |
| 12 | p13 | 47.431 | 2 | latqcc early, Ep in year 5 |

Required preflight blockers before declaring full-run frost closure:

| Blocker | Hillslopes | Evidence |
|---|---|---|
| `CLIM-RUNTIME-E-017` | `p2`, `p4`, `p6`, `p9`, `p14`, `p17` | Hourly radiation value exceeds physical hourly extraterrestrial radiation bound. |
| `HKERNEL-WB11-PERC-E-003` | `p7`, `p11`, `p18`, `p20` | Percolation deep seepage phase fails closed at `1990` Julian day `95`. |
| Missing pre-day-1 storage row | all emitted WAT ledgers | Year 1 cannot be full-calendar-year classified without an initial storage surface. |

Recommended next work-package shape:

1. Next mechanism rung remains **frost** on the emitted conservation-break set,
   starting with `p19`, `p22`, `p16`, and `p5`.
2. A prerequisite/domain-unblock package should either run before or be bundled
   into the frost package scope if full-population closure is required. It must
   preserve fail-closed behavior and must not canonicalize the radiation or
   percolation domain violations away.
3. Do not reopen the snow/`RM` comparator route from this evidence.
