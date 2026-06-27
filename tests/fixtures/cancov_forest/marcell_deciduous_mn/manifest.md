# Marcell EF, MN — deciduous forest (canopy stratum)

Fixture `marcell_deciduous_mn` — per-stratum hillslope for canopy-stratified melt/`cancov` validation.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/ju/juvenile-separatist` |
| Hillslope | TopazID `73` -> wepp_id `p15` |
| Disturbed luse / stratum | `deciduous forest` |
| Climate | 1980-2024 (DAYMET + GRIDMET + CLIGEN + PRISM) |
| Location | ~47.53, -93.47, ~422 m |
| Pairs with observed stratum | deciduous stratum (USFS RDA 10.2737/RDS-2021-0016) |

## Modification
- `ksflag` -> `1` (frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. Only
  change from the as-built inputs.

## Contents
`p15.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill marcell_deciduous_mn p15.run
```
