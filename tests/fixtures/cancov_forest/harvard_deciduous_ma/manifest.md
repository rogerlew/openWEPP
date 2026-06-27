# Harvard Forest, MA — deciduous forest (hardwood) (canopy stratum)

Fixture `harvard_deciduous_ma` — per-stratum hillslope for canopy-stratified melt/`cancov` validation.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/un/undescended-conserve` |
| Hillslope | TopazID `41` -> wepp_id `p6` |
| Disturbed luse / stratum | `deciduous forest (hardwood)` |
| Climate | 1980-2024 (DAYMET + GRIDMET + CLIGEN + PRISM) |
| Location | ~42.537, -72.173, 348 m |
| Pairs with observed stratum | hardwood stratum (HF237) |

## Modification
- `ksflag` -> `1` (frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. Only
  change from the as-built inputs.

## Contents
`p6.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill harvard_deciduous_ma p6.run
```
