# HJ Andrews Experimental Forest, OR (Douglas-fir / W. hemlock)

Fixture `hjandrews_conifer_or` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/jo/joyous-armchair` |
| Hillslope | TopazID `22` -> wepp_id `p2` |
| Canopy type | coniferous (evergreen) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~44.23, -122.17 |
| Snow climate | Pacific maritime |
| Observation source | EDI `MS007` under-canopy + SNOTEL `719:OR:SNTL` (Roaring River) |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.

## Contents
`p2.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill hjandrews_conifer_or p2.run
```
