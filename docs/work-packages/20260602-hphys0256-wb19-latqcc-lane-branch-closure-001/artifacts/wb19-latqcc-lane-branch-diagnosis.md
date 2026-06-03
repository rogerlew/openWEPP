# WB19 Latqcc Lane-Branch Diagnosis

Status: completed

Evidence mode: static

- Static: pinned daily baseline `watbal.for` computes `hk(i) =
  -2.655 / alog10(fc/ul)` at lines 286-304 and uses daily lateral logic at
  lines 570-709.
- Static: daily `solwpv>=2006` uses frozen-adjusted `fzdrfc`, `fzul`,
  `sstz = st/fzul`, `fffx = sstz**hk` floored at `0.002`, and
  `latk = 86400 * (totk/totdg)` without the hourly bottom-contiguous
  `meblfc` gate.
- Static: daily `solwpv<2006` uses a top-contiguous above-threshold block and
  applies an average `avstt/avul/avhk` conductivity multiplier.
- Static: pinned hourly baseline `watbal_hourly.for` uses a bottom-contiguous
  `meblfc` gate at lines 629-665, conductivity on `drfc` at lines 695-715,
  and `latk = 3600 * (totk/totdg)` at lines 725-735.
- Static: openWEPP had applied the hourly lateral law to default daily lanes,
  producing `q_lateral=0` for a state where daily baseline authority requires
  positive lateral flow.
