# Baseline Code Map

Status: queued placeholder.

Normative code authority:
`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Surfaces To Map

| Surface | Baseline file(s) | Required output |
|---|---|---|
| `gwcoeff.txt` parse | `src/main.for` | line map for `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `lr_bf`; missing-file branch behavior |
| daily groundwater update | `src/contin.for` | equations, update order, units, storage carry, recharge from deep percolation |
| hillslope pass payload | `src/wshpas.for`, `src/wshdrv.for` | `gwbfv`/`gwdsv` serialization and readback behavior |
| watershed/channel consumption | `src/wshchr.for`, `src/wshcqi.for` | side/top hillslope baseflow injection, threshold-area branch, phosphorus coupling if relevant |
| publication/accounting | `src/watbalprint.for` | water-balance baseflow behavior under `lr_bf=0` versus `lr_bf=1` |
| namespace separation | `src/main.for`, `src/wshchr.for`, `SC-INFILE-CHANINP-001` | `gwcoeff.bfcoeff` versus `chan.inp cbase` |

## Known Starting Points

Static reconnaissance found these baseline locations:

- `main.for`: `gwcoeff.txt` parse at lines near `120-136` and `450-465`.
- `contin.for`: Srivastava groundwater update at lines near `1089-1116`.
- `wshcqi.for`: `lr_bf` branch and `tmpgwbfv` consumption at lines near
  `86-114`.
- `wshchr.for`: side/top baseflow channel injection at lines near `133-220`;
  `lr_bf=1` channel water-balance behavior near `696`.

Execution must replace this reconnaissance with exact file:line citations in
reviewable artifacts.
