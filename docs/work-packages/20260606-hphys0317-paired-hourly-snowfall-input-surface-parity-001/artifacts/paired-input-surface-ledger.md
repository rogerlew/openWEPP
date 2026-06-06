# Paired Input Surface Ledger

Status: complete

Evidence mode: Static

Static:

- Source routes: HPHYS0315 spring-2014 hourly snowfall input-lineage hold and
  HPHYS0316 spring-2016 2013 terminal carry-recursion hold.
- HPHYS0317 route: `paired-input-surface-instrumentation-hold`.
- ADR0017 verdict: `UNRESOLVED`.
- owner: `HPHYS0318`.
- production_edit_authorized=false.
- Total carried rows: `57`.
- Spring-2014 carried rows: `24`.
- Spring-2016 carried rows: `33`.
- Key: `2013 day 11 hour 11`.
- Baseline source value: `hrsnow = 0.0007454545120708644 m`.
- OpenWEPP homologous value: `snow.hourly.snowfall_m_0011 = 0.0 m`.

| Hillslope | Window | Carried rows | Key | Baseline `hrsnow` depth (m) | OpenWEPP `snow.hourly.snowfall_m_0011` (m) | Delta openWEPP-baseline (m) | Classification | Owner | Production edit |
|---|---|---:|---|---:|---:|---:|---|---|---|
| H1 | spring-2014 | 8 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |
| H7 | spring-2014 | 7 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |
| H39 | spring-2014 | 9 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |
| H1 | spring-2016 | 15 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |
| H7 | spring-2016 | 9 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |
| H39 | spring-2016 | 9 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `paired-input-surface-instrumentation-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0318` | `false` |

Required paired controlling surfaces before production ownership:

| Surface | Baseline status | OpenWEPP status | HPHYS0317 classification |
|---|---|---|---|
| `rain` | Required; not published as a paired key-hour value in available HPHYS0315/0316 artifacts | Required; not published as a paired key-hour value in available HPHYS0315/0316 artifacts | missing paired evidence |
| `stmdur` | Required; not published as a paired key-hour value in available artifacts | Required; not published as a paired key-hour value in available artifacts | missing paired evidence |
| `wntdur` | Required rounded duration from `stmtim.for:48-56`; not published as paired value | Required rounded duration from openWEPP helper; not published as paired value | missing paired evidence |
| `wnttim` | Required adjusted active start from `stmtim.for:58-60`; not published as paired value | Required adjusted active start from openWEPP helper; not published as paired value | missing paired evidence |
| `hrtemp` | Required branch input; not published as paired key-hour value in available artifacts | Required branch input; not published as paired key-hour value in available artifacts | missing paired evidence |
| `rst` | Required branch threshold; not published as paired key-hour value in available artifacts | Required branch threshold; not published as paired key-hour value in available artifacts | missing paired evidence |
| `hrsnow` | Published as output mismatch: `0.0007454545120708644 m` | Published as output mismatch: `0.0 m` | same-unit output mismatch, not source ownership |
| `hrrain` | Required branch output; not published as paired key-hour value in available artifacts | Required branch output; not published as paired key-hour value in available artifacts | missing paired evidence |
| active interval | Required predicate result; inferred from positive baseline `hrsnow`, not independently paired | Required predicate result; inferred from zero openWEPP snowfall, not independently paired | missing paired evidence |
| branch choice | Required rain/snow branch result; inferred from positive baseline `hrsnow`, not independently paired | Required rain/snow branch result; inferred from zero openWEPP snowfall, not independently paired | missing paired evidence |

Conclusion:

The package preserves the combined `57` carried rows and the same-unit
`hrsnow`/`snow.hourly.snowfall_m_0011` mismatch. Existing evidence does not
publish the controlling paired input surfaces required by ADR0017 and
`SC-CLIMATE-001#INV-CLIMATE-015`; therefore production edit authorization
remains `false`.
