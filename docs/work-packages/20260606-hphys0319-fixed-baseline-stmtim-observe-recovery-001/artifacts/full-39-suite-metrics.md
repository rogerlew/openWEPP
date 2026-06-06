# Full 39 Suite Metrics

Status: complete

Evidence mode: Static

Static:

- H1..H39 semantic metrics are an observe-recovery only carry-forward from the
  current HPHYS0318 route state.
- Semantic pass count: `0/39`.
- HPHYS0319 did not change precipitation-phase physics, snowpack equations,
  water-balance equations, or comparator outputs.
- The combined `57` carried rows remain failing/owned `HOLD` under
  `stmtim-active-interval-divergence-hold`.
- HPHYS0320 owns source-line classification for baseline adjusted
  `wnttim = 1` versus OpenWEPP `wnttim = 0`.
