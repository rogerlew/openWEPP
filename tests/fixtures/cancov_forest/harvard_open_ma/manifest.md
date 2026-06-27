# Harvard Forest, MA — short grass (open) (canopy stratum)

Fixture `harvard_open_ma` — per-stratum hillslope for canopy-stratified melt/`cancov` validation.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/un/undescended-conserve` |
| Hillslope | TopazID `31` -> wepp_id `p3` |
| Disturbed luse / stratum | `short grass (open)` |
| Climate | 1980-2024 (DAYMET + GRIDMET + CLIGEN + PRISM) |
| Location | ~42.537, -72.173, 348 m |
| Pairs with observed stratum | open stratum (HF237) |

## Modification
- `ksflag` -> `1` (frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. Only
  change from the as-built inputs.

## Contents
`p3.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill harvard_open_ma p3.run
```
