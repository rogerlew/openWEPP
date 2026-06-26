# Marcell Experimental Forest, MN (aspen/birch + pine/spruce + tamarack, peatland)

Fixture `marcell_mixed_mn` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/ju/juvenile-separatist` |
| Hillslope | TopazID `61` -> wepp_id `p10` |
| Canopy type | mixed (NLCD 43; mixed-forest mgmt, `xmxlai=9.5`) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~47.53, -93.47, ~422 m |
| Snow climate | Laurentian Mixed Forest, cold continental |
| Observation source | USFS RDA `10.2737/RDS-2021-0016` (biweekly SWE/depth/frost by conifer/deciduous/open cover type, 1962-) |

## Modification
- `ksflag` `1 0` -> `1 1` (forest default 0 -> frost active); comment
  `# ksflag -> 0` -> `# ksflag -> 1`. Only change from the as-built inputs.

## Contents
`p10.{run,man,slp,sol,cli}` + sidecars `snow.txt`, `pmetpara.txt`, `gwcoeff.txt`.

## Run
```
openwepp-cli-hill marcell_mixed_mn p10.run
```
