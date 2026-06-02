# WB18 Baseline Lineage Map

Status: completed

Evidence mode: Static

Static:
- Pinned baseline:
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `watbal_hourly.for:540-545`: calls `purk` each hourly substep and
  accumulates `deepSeep = deepSeep + sep(iplane)`.
- `purk.for:167-188`: in hourly mode (`ui_run.eq.1`), mutates current and
  lower-layer `st` by `sep(iplane)/ui_LFtstp`; bottom seepage remembered as
  `sepsav = sep(iplane)/ui_LFtstp`.
- `perc.for:186-214`: for bottom layer in hourly mode, sets lower boundary
  conductivity from restrictive-layer `kslast` when `slflag=1`, then computes
  thickness-weighted `sscz = (dg(k1)+ui_bdrkth)/(dg(k1)/ssc(k1)+ui_bdrkth/ssc(k1+1))`.
- `perc.for:227-228`: computes `sep = min(vv, 86400*fx*sscz)`.
- H39 `p39.sol` restrictive footer is `1 10000.0 0.01`, so
  `ui_bdrkth=10 m` and `kslast=0.01 mm/h`; this matches the legacy day-1
  baseline `Dp=0.24 mm/day` order of magnitude.
