# Worker Handoff

Status: complete

Evidence mode: Static

Next package: `HPHYS0319`

Carry-forward facts:

- HPHYS0318 preserves the combined `57` carried rows.
- Current route: `paired-fixed-baseline-stmtim-observe-hold`.
- Production edit authorization remains `false`.
- Key: `2013 day 11 hour 11`.
- Fixed baseline records `hrsnow = 0.0007454545120708644 m`.
- OpenWEPP records `snow.hourly.snowfall_m_0011 = 0.0 m`.
- OpenWEPP now publishes `snow.hourly.stmtim.*_####` runtime diagnostics and
  `snow_hourly_stmtim_*` HPHYS0245 trace maps.
- Fixed-baseline paired values for `rain`, `stmdur`, rounded `wntdur`,
  adjusted `wnttim`, `hrtemp`, `rst`, `hrrain`, active interval membership,
  and branch choice remain unavailable.

HPHYS0319 should instrument or recover fixed-baseline `stmtim.for` observe
values at the same key and same lineage before any producer or downstream
water-balance edit is considered.
