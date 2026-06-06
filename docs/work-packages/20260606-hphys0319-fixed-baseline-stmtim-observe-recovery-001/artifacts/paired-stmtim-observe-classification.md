# Paired Stmtim Observe Classification

Status: complete

Evidence mode: Ran

Ran:
- paired_fixed_baseline_openwepp_stmtim_values: `true`
- production_physics_edit_authorized: `false`
- carried_rows_total: `57`
- next_owner: `HPHYS0320`
- next_route: `paired-stmtim-source-line-classification-hold`

| Hill | baseline hrsnow m | openwepp snow.hourly.stmtim.hrsnow_m_0011 | baseline active | openwepp snow.hourly.stmtim.active_interval_0011 | baseline snow branch | openwepp snow branch | classification |
| --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 0.00074545 | 0 | 1 | 0 | 1 | 0 | stmtim-active-interval-divergence-hold |
| H7 | 0.00074545 | 0 | 1 | 0 | 1 | 0 | stmtim-active-interval-divergence-hold |
| H39 | 0.00074545 | 0 | 1 | 0 | 1 | 0 | stmtim-active-interval-divergence-hold |

HPHYS0319 recovers the missing fixed-baseline observe lane but does not by itself prove a production defect. HPHYS0320 owns source-line classification for the paired divergence.
