# Hubbard Brook Experimental Forest, NH (northern hardwood, leaf-off)

Fixture `hubbardbrook_deciduous_nh` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/sc/scabby-demographic` |
| Hillslope | TopazID `62` -> wepp_id `p10` |
| Canopy type | deciduous (NLCD 41; deciduous mgmt) |
| Climate | 1980-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~43.94, -71.72 |
| Snow climate | N. Appalachian humid continental |
| Observation source | EDI snow course `knb-lter-hbr.27` + SCAN `2069:NH:SCAN` |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.

## Contents
`p10.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill hubbardbrook_deciduous_nh p10.run
```
