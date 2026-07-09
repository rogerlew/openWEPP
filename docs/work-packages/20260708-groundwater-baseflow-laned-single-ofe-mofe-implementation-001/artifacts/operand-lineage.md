# Operand Lineage

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

| Operand | Units | Source | Destination | Alias rejected | Status |
|---|---|---|---|---|---|
| `D_i` groundwater recharge | `m^3` over daily timestep | `hydrology_projection.deep_percolation_m * lane.area_m2`, summed over Lane D lanes | Groundwater recurrence | generated `gwdsv`; `latqcc`; surface runoff | mapped |
| `S_i` groundwater storage | `m^3` | Run-level groundwater carry after current recharge and prior outflows | Next-day recurrence and diagnostics | soil water storage; snow storage | mapped |
| `Qb_i` / `gwbfv` | `m^3` over daily timestep | `bfcoeff * S_i` | Direct WAT `Base`, watershed WAT `baseflow_mm` / `channel_baseflow_m3`, active summary total | `latqcc`; `cbase`; active surface source | mapped |
| `Qs_i` / `gwdsv` | `m^3` over daily timestep | `dscoeff * S_i` | Direct runtime output and active summary total; real downstream consumer held | current soil `Dp`; `latqcc`; `Base` | HOLD |
| `latqcc` | `mm` publication depth / outlet volume | `SC-SUBHYD-001` lateral export, terminal lane only for watershed aggregate | Lane D bypass/export ledger and WAT `latqcc` | `gwbfv`; `gwdsv`; `cbase` | protected |
| `ui_SCrunf` | active-router source depth/volume | Return-flow exfiltration seam in hourly subsurface carry | Lane D surface source series | `gwbfv`; `gwdsv`; `latqcc` | protected |
| `cbase` | `m^3 s^-1 m^-2` | `chan.inp` channel branch | Separate watershed/channel default branch only when `lr_bf=0` | `bfcoeff`; `gwbfv`; WAT `Base` when generated | protected |
