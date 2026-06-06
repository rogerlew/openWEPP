# Snow Carry Source-Line Parity Summary

Status: complete

Evidence mode: ran

## Counts

- Affected HPHYS0309 rows represented: `58`
- HPHYS0310 groups represented: `7`
- Production edit authorized groups: `0`

## Route Counts

- `fixed-observe-precision-hold`: `1`
- `prior-year-terminal-state-hold`: `6`

## Group Routes

| Hillslope | Window | Year | Rows | Route | Finding |
|---|---|---:|---:|---|---|
| H1 | first-abs-storage-ge-10mm | 2013 | 1 | `fixed-observe-precision-hold` | baseline/openWEPP previous-hour states are near-identical, but H305_S_OUT exposes rounded post-hour depth/density and omits baseline wdayct; source-line equation defect is not proven |
| H1 | spring-2014 | 2014 | 8 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |
| H1 | spring-2016 | 2016 | 15 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |
| H7 | spring-2014 | 2014 | 7 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |
| H7 | spring-2016 | 2016 | 9 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |
| H39 | spring-2014 | 2014 | 9 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |
| H39 | spring-2016 | 2016 | 9 | `prior-year-terminal-state-hold` | day-1 h01 delta equals prior-year terminal delta; source-line carry-forward is parity and the residual is inherited from the prior-year terminal snowpack state |

## Interpretation

Six day-1 groups carry the exact prior-year terminal depth and density
deltas into the new year; the source-line carry-forward path itself is parity and the
residual remains inherited prior-year terminal snowpack state. The single
H1 2013 density/settling group has near-identical previous-hour states,
but the available fixed observe lane is rounded and lacks baseline
`wdayct`, so no production settling equation edit is authorized.
