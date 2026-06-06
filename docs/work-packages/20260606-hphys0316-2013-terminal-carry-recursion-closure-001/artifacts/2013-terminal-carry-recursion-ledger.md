# 2013 Terminal Carry Recursion Ledger

Status: complete

Evidence mode: Static

Static:

- Source route: HPHYS0313 `recursive-year-start-inherited-state-hold`.
- HPHYS0316 route: `2013-hourly-snowfall-input-surface-parity-hold`.
- ADR0017 verdict: `UNRESOLVED`.
- owner: `HPHYS0317`.
- production_edit_authorized=false.
- Total carried rows: `33`.
- Inherited continuity key: `2014 day 1 hour 1`.
- First-material source key: `2013 day 11 hour 11`.
- Baseline source value: `hrsnow = 0.0007454545120708644 m`.
- OpenWEPP homologous value: `snow.hourly.snowfall_m_0011 = 0.0 m`.

| Hillslope | Window | Carried rows | 2014 day-1 h1 depth delta (m) | 2013 terminal depth delta (m) | 2013 first material key | First source lane | Owner | Production edit |
|---|---|---:|---:|---:|---|---|---|---|
| H1 | spring-2016 | 15 | 0.013144251023522513 | 0.013144251023522124 | `2013 day 11 hour 11` | `2013-hourly-snowfall-input-surface-parity-hold` | `HPHYS0317` | `false` |
| H7 | spring-2016 | 9 | 0.015279465660242741 | 0.015279465660242408 | `2013 day 11 hour 11` | `2013-hourly-snowfall-input-surface-parity-hold` | `HPHYS0317` | `false` |
| H39 | spring-2016 | 9 | 0.0147979087518893 | 0.014797908751889022 | `2013 day 11 hour 11` | `2013-hourly-snowfall-input-surface-parity-hold` | `HPHYS0317` | `false` |

Interpretation:

The spring-2016 rows are not a separate branch-predicate, melt-term, WB13,
WB17, WB18, WB19, or WB12 route. They inherit the 2013 terminal snowpack delta
already localized by HPHYS0312/HPHYS0313 to the 2013 day 11 hour 11
branch-gated positive-`hrsnow` route. Because HPHYS0315 did not prove paired
input-surface ownership for that key, the inherited spring-2016 rows remain
`UNRESOLVED`.
