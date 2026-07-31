# Complete Chronology Evidence

Evidence: `Ran`

`complete-chronology.csv.gz` contains all 83,208 successful daily rows from the
24 hash-bound EB-04A traces plus one typed terminal row for each target: 83,232
data rows total. It includes every prospectively named available operand:

- SWE, depth, density, layer count, and layer-temperature range;
- cold content before/after/export and an independent whole-pack temperature
  diagnostic;
- sublimation, snowpack loss, shortwave, longwave, latent, surface energy,
  refreeze, and routed melt;
- active/lower mass and depth;
- maximum and peak requested/applied/rejected `G_0`, active/lower peak
  temperature, conductivity, resistance, and timestep;
- exact thermal-snapshot or geometry-terminal operands.

The gzip stream is generated with `mtime=0` and a fixed SVG hash salt. Two
consecutive executions produce identical JSON, CSV, compressed chronology,
SVG, and sidecar hashes.

`case-dynamics-summary.csv` aggregates the complete chronology by target. Across
the 24 targets, 21 histories contain a lower volume, the maximum requested,
applied, and rejected absolute `G_0` values are `730.80`, `32.72`, and
`721.50 W m^-2`, and the minimum selected substep is `60 s`. These quantities
describe the retained approach but do not identify the unpublished terminal
crossing substep.
