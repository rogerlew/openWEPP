# Magnitude Attribution

Status: EXECUTED
Evidence mode: Ran.

## Dominant Clamp Rows

| Rung | Rank | Day | Lane | Clamp m3 | Source m3 | Outlet m3 | Storage m3 |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline_fixed10 | 1 | 1418 | 5 | 145554.778351 | 2914.7262742 | 156178.946938 | 6.38734563086 |
| baseline_fixed10 | 2 | 1167 | 4 | 783.871996027 | 2223.36483363 | 7092.33782749 | 6.2629444433 |
| baseline_fixed10 | 3 | 1418 | 4 | 426.945523395 | 2926.11151005 | 7715.82965812 | 5.12748111849 |
| baseline_fixed10 | 4 | 1790 | 5 | 99.3503490848 | 2019.87257734 | 5112.95041902 | 2.54779418398 |
| baseline_fixed10 | 5 | 1167 | 3 | 75.7506097797 | 1981.0614631 | 4091.36394228 | 5.0687475612 |
| dx20 | 1 | 1418 | 5 | 145554.778351 | 2914.7262742 | 156178.946938 | 6.38734563086 |
| dx20 | 2 | 1167 | 4 | 783.871996027 | 2223.36483363 | 7092.33782749 | 6.2629444433 |
| dx20 | 3 | 1418 | 4 | 426.945523395 | 2926.11151005 | 7715.82965812 | 5.12748111849 |
| dx20 | 4 | 1790 | 5 | 99.3503490848 | 2019.87257734 | 5112.95041902 | 2.54779418398 |
| dx20 | 5 | 1167 | 3 | 75.7506097797 | 1981.0614631 | 4091.36394228 | 5.0687475612 |
| dx10 | 1 | 1418 | 5 | 457540698.111 | 2914.7262742 | 454321724.743 | 3229738.0758 |
| dx10 | 2 | 1167 | 4 | 1445.53552013 | 2223.36483363 | 7784.23042905 | 6.06460576272 |
| dx10 | 3 | 1418 | 4 | 561.193421539 | 2926.11151005 | 7849.98182158 | 5.18274530363 |
| dx10 | 4 | 1790 | 5 | 140.182240742 | 2019.87257734 | 5153.7698739 | 2.54911206905 |
| dx10 | 5 | 1167 | 3 | 105.847734844 | 1981.0614631 | 4121.39468105 | 5.14205017239 |
| dx5 | 1 | 1418 | 5 | 27708994361.1 | 2914.7262742 | 27659171303 | 65123121.8217 |
| dx5 | 2 | 1167 | 4 | 71717608.09 | 2223.36483363 | 9938289.86445 | 62069299.1887 |
| dx5 | 3 | 1167 | 5 | 22394919.9152 | 2211.67341068 | 17417884.8305 | 14917536.6226 |
| dx5 | 4 | 1418 | 4 | 18780169.8977 | 2926.11151005 | 15297148.963 | 3490314.80442 |
| dx5 | 5 | 1790 | 5 | 2344968.5307 | 2019.87257734 | 681276.217717 | 1668708.41257 |

## Day-1418 Climate

| Surface | Value |
|---|---:|
| Date | 2003-11-18 |
| Precipitation mm | 83.8 |
| Duration h | 13.3 |
| Peak-intensity input | 9.6 |

## Hydrology-Source Delta Check

Maximum absolute delta from `baseline_fixed10` in `H1.wat.parquet` for
the inspected days. Zero deltas mean the source producer is not changing
across mesh rungs.

| Rung | Day | Max hydrology delta |
|---|---:|---:|
| baseline_fixed10 | 1122 | 0 |
| baseline_fixed10 | 1167 | 0 |
| baseline_fixed10 | 1418 | 0 |
| dx20 | 1122 | 0 |
| dx20 | 1167 | 0 |
| dx20 | 1418 | 0 |
| dx10 | 1122 | 0 |
| dx10 | 1167 | 0 |
| dx10 | 1418 | 0 |
| dx5 | 1122 | 0 |
| dx5 | 1167 | 0 |
| dx5 | 1418 | 0 |

## Attribution

- The largest completed-rung magnitudes localize to day 1418, lane 5,
  not to day 1122.
- The active hydrology source rows are unchanged across completed rungs
  for the inspected days, so the amplification is router-internal.
- `dx20` is identical to `baseline_fixed10` because the 10-cell floor
  controls 108.34 m OFEs; finer target-`dx` rungs increase cell counts
  and expose the clamp/storage amplification.
