# 2013 Terminal Carry Source Lineage

Status: complete

Evidence mode: Static

Static:

| Lane | Source | HPHYS0316 use |
|---|---|---|
| Day-start carry | `winter.for:193` | Baseline assigns `snodpt(iplane)=snodpy(iplane)`, carrying prior terminal snowpack state into day 1. |
| Hourly snow state seed | `snowd.for:50-53` | Baseline initializes `snodep`, `snodpt`, `densgy`, and `densgt` from carried `snodpy`/`densg`. |
| Settle-day count update | `snowd.for:61-65` | Baseline increments day count at hour 1 and resets it when `hrsnow(hour) > 0.0`. |
| Cold settling | `snowd.for:122-139` | Baseline computes `setf`, density, cap, and settled depth. |
| Snowing branch | `snowd.for:166-172` | Baseline positive-`hrsnow` branch adds snowfall depth to settled depth and updates density. |
| Post-hour carry writeback | `snowd.for:303-312` | Baseline writes `snodep`/`densgt` back to `snodpt`, `snodpy`, and `densg`. |
| OpenWEPP day-count handling | `03_kernel_support_00_support_helpers.rs:3872-3877` | Homologous settle-day count increment/reset lane. |
| OpenWEPP cold settling | `03_kernel_support_00_support_helpers.rs:3901-3912` | Homologous `setf`, density cap, and settled-depth lane. |
| OpenWEPP snowing branch | `03_kernel_support_00_support_helpers.rs:3914-3924` | Homologous positive-hourly-snowfall depth/density update lane. |
| Prior terminal scan | `HPHYS0312` | Scanned 2013 and proved the spring-2014 terminal deltas that feed 2014 day 1. |
| Split-route reconstruction | `HPHYS0313` | Reconstructed the 2013 day 11 hour 11 positive-`hrsnow` branch and ruled out settling equation/drift ownership. |
| Hourly input blocker | `HPHYS0315` | Preserved the 2013 day 11 hour 11 input-surface parity gap and assigned follow-on ownership to `HPHYS0317`. |

Continuity:

- H1 spring-2016: 2014 day 1 hour 1 depth delta
  `0.013144251023522513 m`; matching 2013 terminal depth delta
  `0.013144251023522124 m`.
- H7 spring-2016: 2014 day 1 hour 1 depth delta
  `0.015279465660242741 m`; matching 2013 terminal depth delta
  `0.015279465660242408 m`.
- H39 spring-2016: 2014 day 1 hour 1 depth delta
  `0.0147979087518893 m`; matching 2013 terminal depth delta
  `0.014797908751889022 m`.

Remaining gap:

HPHYS0316 connects the inherited spring-2016 rows to the already localized 2013
day 11 hour 11 positive-`hrsnow` route. It does not prove paired
fixed-baseline/openWEPP input surfaces for `rain`, `stmdur`, `wntdur`,
`wnttim`, `hrtemp`, `rst`, `hrsnow`, and `hrrain`. That blocker remains owned
by `HPHYS0317`, and no production edit is authorized.
