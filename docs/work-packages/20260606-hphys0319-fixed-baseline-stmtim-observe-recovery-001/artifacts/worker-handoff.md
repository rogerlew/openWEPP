# Worker Handoff

Status: complete

Evidence mode: Static

Static:

Next package: `HPHYS0320`

Recommended route: `paired-stmtim-source-line-classification-hold`

Carry-forward facts:

- HPHYS0319 recovers fixed-baseline paired `stmtim` observe values for H1/H7/H39
  at 2013 day 11 hour 11.
- Combined carried rows remain `57`.
- Fixed baseline records `wntdur = 11`, adjusted `wnttim = 1`, active interval
  `1`, snow branch `1`, and `hrsnow = 0.00074545 m`.
- OpenWEPP records `wntdur = 11`, `wnttim = 0`, active interval `0`, snow
  branch `0`, and `snow.hourly.stmtim.hrsnow_m_0011 = 0`.
- The immediate paired classification is
  `stmtim-active-interval-divergence-hold`.
- Production edit authorization remains `false`.

HPHYS0320 should classify why OpenWEPP preserves/publicizes `wnttim = 0` while
baseline `stmtim.for` adjusts `wnttim` to `1` for this event before any
producer or downstream water-balance edit.
