# WEPPcloud Source Recovery

Recovery date: `2026-07-26`

Evidence class: `Live read-only production inspection plus byte-preserving copy`

## Recovered projects

| Site | Run/config | Production path | Selected surface | Topaz ID | Centroid | Elevation |
| --- | --- | --- | --- | --- | --- | --- |
| Hubbard Brook | `unassailable-sensuousness/disturbed9002` | `/geodata/wc1/runs/un/unassailable-sensuousness` | `p1` / `H1` | `22` | `-71.725899, 43.948767` | `551.599976 m` |
| Santee | `clean-burning-griddle/disturbed9002` | `/geodata/wc1/runs/cl/clean-burning-griddle` | `p2` / `H2` | `23` | `-79.791594, 33.148681` | `9.30002 m` |

The production paths are host paths. Read-only inspection inside the WEPPcloud
container used the corresponding `/wc1/runs/...` paths.

The selected surfaces follow Bill's report. The WEPP-ID-to-Topaz-ID mapping,
centroids, and elevations come from each recovered `watershed/hillslopes.parquet`.

## Retained subset

The repository fixture retains, for each selected surface:

- the exact `.cli`, `.man`, `.run`, `.slp`, `.sol`, and `.err` inputs;
- groundwater, Penman-Monteith, snow, channel, and channel-type sidecars;
- exact event, element, loss, pass, plot, soil, and water outputs;
- both source return-period JSON files;
- the root run, climate, soil, land-use, watershed, WEPP, disturbed, and unit
  controller records plus the WEPP log; and
- the source hillslope table.

The combined fixture contains 60 source-native files totaling `33,389,530`
bytes. Each site has an independent `SHA256SUMS` manifest.

## Run identity

Both selected run files encode 100 simulation years, and both synthetic climate
records span simulation years 1 through 100. Both climate files report CLIGEN
`5.32300` and seed `12345`. Hubbard uses station file `nh275639.par`; Santee
uses `sc381544.par`.

The source land-use controllers identify 0.9 canopy cover:

- Hubbard: Deciduous Forest, class `41`;
- Santee: Mixed Forest, class `43`.

The selected source soils are SSURGO-derived sand loams:

- Hubbard: mukey `665220`;
- Santee: mukey `131976`.

## Boundary

This bundle recreates the exact report-linked WEPPcloud source surfaces. It
does not recreate Bill's manually transcribed Windows WEPP project. Those
converted slope, soil, management/run, and output files were not stored in the
WEPPcloud projects. The BLARHG executable is documented separately so CAL-02
can run controlled Windows comparisons without conflating those comparisons
with Bill's missing original project.
