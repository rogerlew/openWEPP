# Marcell EF, MN — short grass (open) (canopy stratum)

Fixture `marcell_open_mn` — per-stratum hillslope for canopy-stratified melt/`cancov` validation.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/ju/juvenile-separatist` |
| Hillslope | TopazID `42` -> wepp_id `p6` |
| Disturbed luse / stratum | `short grass (open)` |
| Climate | 1980-2024 (DAYMET + GRIDMET + CLIGEN + PRISM) |
| Location | ~47.53, -93.47, ~422 m |
| Pairs with observed stratum | open stratum (USFS RDA 10.2737/RDS-2021-0016) |

## Modification
- `ksflag` -> `1` (frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. Only
  change from the as-built inputs.

## Contents
`p6.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill marcell_open_mn p6.run
```
