# Berthoud Summit / Fraser EF band, CO (Engelmann spruce / subalpine fir / lodgepole)

Fixture `berthoud_conifer_co` — canopy-stratified snow melt / `cancov` validation hillslope.

| Field | Value |
|---|---|
| Source wepp.cloud run | `/wc1/runs/ol/old-fluorosis` |
| Hillslope | TopazID `32` -> wepp_id `p4` |
| Canopy type | coniferous (evergreen) |
| Climate | 1986-2024 (DAYMET daily + GRIDMET wind + CLIGEN + PRISM) |
| Location | ~39.80, -105.78 |
| Snow climate | CO continental subalpine |
| Observation source | SNOTEL `05K14S:CO:SNTL` (SWE + depth + soil-T) |

## Modification
- `ksflag` `1 0` → `1 1` (forest default 0 → frost active); comment `# ksflag -> 0` -> `# ksflag -> 1`. This is the **only**
  change from the as-built wepp.cloud inputs.
- **RAP_TS-adjusted cancov** applied in the source build.
- Operator satellite-imagery check on 2026-06-26 found the low
  `cancov = 0.05` consistent with the site imagery. Treat this as
  site-specific sparse/open conifer evidence, not an automatic mismatch against
  the generic evergreen `0.90` projection.

## Contents
`p4.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt` (copied from the source `wepp/runs/`).

## Run
```
openwepp-cli-hill berthoud_conifer_co p4.run
```
