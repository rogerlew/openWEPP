# Sleepers River RW, VT (open / pasture)

Fixture `sleepers_pasture_vt` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/in/interconnected-fit` |
| Hillslope | TopazID `23` -> wepp_id `p3` |
| Canopy type | pasture / ag (low canopy) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~44.46, -72.09 |
| Snow climate | NE Vermont glaciated upland |
| Observation source | USGS DOI 10.5066/P9NMQX70 (ties to non-SNOTEL frost blocker, Sleepers South) |

## Modification
- `ksflag` already `1` (ag/pasture default); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.

## Contents
`p3.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill sleepers_pasture_vt p3.run
```
