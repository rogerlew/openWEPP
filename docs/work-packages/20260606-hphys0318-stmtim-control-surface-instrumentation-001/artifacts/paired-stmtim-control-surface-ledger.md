# Paired Stmtim Control-Surface Ledger

Status: complete

Evidence mode: Static

Static:

route_id: `paired-fixed-baseline-stmtim-observe-hold`
carried_rows_total: `57`
HPHYS0315_spring_2014_rows: `24`
HPHYS0316_spring_2016_rows: `33`
key: `2013 day 11 hour 11`
fixed_baseline_hrsnow_m: `0.0007454545120708644`
openwepp_snow_hourly_snowfall_m_0011: `0.0`
openwepp_stmtim_trace_implemented: `true`
fixed_baseline_stmtim_observe_available: `false`
production_physics_edit_authorized: `false`

Required control surfaces:

| Surface | Fixed baseline | OpenWEPP | Status |
|---|---:|---:|---|
| Surface | Fixed baseline | OpenWEPP trace alias | Status |
|---|---:|---:|---|
| `rain` | missing | `snow.hourly.stmtim.rain_m_0011` | OpenWEPP instrumented; fixed baseline missing |
| `stmdur` | missing | `snow.hourly.stmtim.stmdur_s_0011` | OpenWEPP instrumented; fixed baseline missing |
| rounded `wntdur` | missing | `snow.hourly.stmtim.wntdur_h_0011` | OpenWEPP instrumented; fixed baseline missing |
| adjusted `wnttim` | missing | `snow.hourly.stmtim.wnttim_h_0011` | OpenWEPP instrumented; fixed baseline missing |
| `hrtemp` | missing | `snow.hourly.stmtim.hrtemp_c_0011` | OpenWEPP instrumented; fixed baseline missing |
| `rst` | missing | `snow.hourly.stmtim.rst_c_0011` | OpenWEPP instrumented; fixed baseline missing |
| `hrrain` | missing | `snow.hourly.stmtim.hrrain_m_0011` | OpenWEPP instrumented; fixed baseline missing |
| `hrsnow` | `0.0007454545120708644 m` | `snow.hourly.stmtim.hrsnow_m_0011` | OpenWEPP instrumented; fixed baseline output known, controlling side incomplete |
| active interval membership | missing | `snow.hourly.stmtim.active_interval_0011` | OpenWEPP instrumented; fixed baseline missing |
| rain branch choice | missing | `snow.hourly.stmtim.rain_branch_0011` | OpenWEPP instrumented; fixed baseline missing |
| snow branch choice | missing | `snow.hourly.stmtim.snow_branch_0011` | OpenWEPP instrumented; fixed baseline missing |

Classification:

- ADR0017 verdict: `UNRESOLVED`.
- Owner: `HPHYS0319`.
- Remaining blocker: fixed-baseline paired `stmtim` observe values.
- Disallowed actions: precipitation-phase edit, snow-producer edit,
  branch-predicate edit, melt-term edit, WB13/WB17/WB18/WB19/WB12
  compensation.
