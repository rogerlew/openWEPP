# Harvard Forest, MA (red oak / red maple + white pine / hemlock)

Fixture `harvard_mixed_ma` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/un/undescended-conserve` |
| Hillslope | TopazID `43` -> wepp_id `p8` |
| Canopy type | mixed (NLCD 43; mixed-forest mgmt) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~42.54, -72.17, 348 m |
| Snow climate | NE transitional |
| Observation source | snow-pillow SWE `HF155` + canopy-stratified depth/density `HF237` |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.

## Contents
`p8.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill harvard_mixed_ma p8.run
```
