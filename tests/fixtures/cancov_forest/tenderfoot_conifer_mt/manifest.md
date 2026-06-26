# Tenderfoot Creek Experimental Forest, MT (lodgepole / spruce-fir)

Fixture `tenderfoot_conifer_mt` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/as/askance-regularity` |
| Hillslope | TopazID `22` -> wepp_id `p2` |
| Canopy type | coniferous (evergreen) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~46.92, -110.85 |
| Snow climate | N. Rockies continental |
| Observation source | on-forest SNOTEL `1008:MT:SNTL` (Onion Park) / `1009:MT:SNTL` (Stringer Creek) |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.

## Contents
`p2.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill tenderfoot_conifer_mt p2.run
```
