# Disposition

Status: complete

Evidence mode: Ran

Ran:

Final disposition: `executed-hold`

production_physics_edit_authorized: `false`

HPHYS0319 scaffolded and executed fixed-baseline `stmtim` observe recovery for
H1/H7/H39 at the 2013 day 11 hour 11 key. The temporary pinned-baseline
instrumentation recovered the missing fixed-baseline lane:

- `rain = 0.00082 m`
- `stmdur = 38040 s`
- rounded `wntdur = 11 h`
- adjusted `wnttim = 1 h`
- active interval membership = `1`
- snow branch = `1`
- `hrsnow = 0.00074545 m`

Regenerated OpenWEPP HPHYS0245 traces published matching `rain`, rounded
`wntdur`, and near-identical `hrtemp`/`rst`, but preserved `wnttim = 0`,
active interval membership = `0`, snow branch = `0`, and
`snow.hourly.stmtim.hrsnow_m_0011 = 0`.

The combined `57` carried rows remain ADR0017 `UNRESOLVED`/owned `HOLD` as
`stmtim-active-interval-divergence-hold`. HPHYS0320 owns source-line
classification for the baseline adjusted `wnttim = 1` versus OpenWEPP
`wnttim = 0` divergence. No precipitation-phase, snow-producer,
branch-predicate, melt-term, WB13, WB17, WB18, WB19, or WB12 edit is
authorized by HPHYS0319.
