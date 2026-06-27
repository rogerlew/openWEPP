# Hubbard Brook EF, NH — mixed forest (canopy stratum)

Fixture `hubbardbrook_mixed_nh` — per-stratum hillslope for canopy-stratified melt/`cancov` validation.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/sc/scabby-demographic` |
| Hillslope | TopazID `33` -> wepp_id `p4` |
| Disturbed luse / stratum | `mixed forest` |
| Climate | 1980-2024 (DAYMET + GRIDMET + CLIGEN + PRISM) |
| Location | ~43.945, -71.720 |
| Pairs with observed stratum | mixed (complements deciduous; knb-lter-hbr.27) |

## Modification
- `ksflag` -> `1` (frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. Only
  change from the as-built inputs.

## Contents
`p4.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill hubbardbrook_mixed_nh p4.run
```
