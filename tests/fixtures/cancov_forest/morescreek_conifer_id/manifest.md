# Mores Creek Summit, ID (ponderosa / lodgepole)

Fixture `morescreek_conifer_id` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/pr/praetorian-talcum` |
| Hillslope | TopazID `42` -> wepp_id `p6` |
| Canopy type | coniferous (evergreen) |
| Climate | 1986-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~43.93, -115.67 |
| Snow climate | Idaho intermountain |
| Observation source | SNOTEL `15F01S:ID:SNTL` (SWE + depth + soil-T since 1992) |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.
- **RAP_TS-adjusted cancov** applied in the source build.

## Contents
`p6.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill morescreek_conifer_id p6.run
```
