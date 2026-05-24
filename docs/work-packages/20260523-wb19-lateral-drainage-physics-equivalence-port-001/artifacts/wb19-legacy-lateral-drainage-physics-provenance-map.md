# WB19 Legacy Lateral Drainage Physics Provenance Map

Status: `completed`
Evidence mode: `Static`

## Legacy Authority Anchor
- Baseline worktree: `/workdir/wepp-forest_260430_baseline`
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Legacy source files:
  - `/workdir/wepp-forest_260430_baseline/src/watbal.for`
  - `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Equation/Behavior Mapping
| Legacy authority | Legacy lines | WB19 contract authority | openWEPP runtime mapping |
| --- | --- | --- | --- |
| Saturated-thickness detection and water-table depth (`watbl`, `unsdep`) | `watbal.for` 530-552 | `SC-SUBHYD-001` WB19 algorithm + branch table | `run_drainage` water-table branch, lines 4638-4663 |
| Layer-weighted saturated conductivity and anisotropy lateral flux (`latk`, `latqcc`) | `watbal.for` 607-706 | `SC-SUBHYD-001` Eq. [6.2.4]-style lateral clause; `SC-WATBAL-001` WB19 lane rows | `run_lateral_transfer` conductivity weighting + `q_potential`, lines 4419-4464 |
| Top-down lateral withdrawal from layers above field-capacity | `watbal.for` 727-785 | `SC-SUBHYD-001` WB19 withdrawal sequencing | `wb19_withdraw_top_down`, lines 2181-2196; applied in lateral path line 4478 |
| Tile layer detection from cumulative depth | `drain.for` 101-110 | `SC-SUBHYD-001` WB19 drainage branch semantics | `run_drainage` tile-layer detection, lines 4663-4672 |
| Drain-zone conductivity averaging below water table (`dranks`) | `drain.for` 117-154 | `SC-SUBHYD-001` Eq. [6.2.10]-[6.2.11] preconditions | `run_drainage` `dranks` calculation, lines 4673-4688 |
| Equivalent depth branch equations and geometry checks (`de`) | `drain.for` 160-186 | `SC-SUBHYD-001` Eq. [6.2.10]-[6.2.11] equivalent-depth clause | `run_drainage` `equivalent_depth_cm` branch, lines 4706-4743 |
| Drainage flux equation + cm/h to m/day conversion | `drain.for` 189-203 | `SC-SUBHYD-001` drainage flux clause | `run_drainage` `drainage_cm_h` and conversion, lines 4754-4768 |
| Drainage capacity cap (`drainc`) | `drain.for` 205-206 | `SC-SUBHYD-001` INV-SUBHYD-011 and WB19 cap requirement | `q_drainage = min(q_potential, wb11_drainage_coefficient, available_pool)`, lines 4779-4781 |
| Tile-layer-to-surface drainage withdrawal order | `drain.for` 217-260 | `SC-SUBHYD-001` WB19 drainage withdrawal sequencing | `wb19_withdraw_tile_to_surface`, lines 2198-2221; applied at lines 4782-4787 |

## Status-ID Continuity
WB19 retains legacy WB11 kernel-family status IDs for guard and success
surfaces while changing underlying physics authority to the WB19 layer-aware
implementation.
