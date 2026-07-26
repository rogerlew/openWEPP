# Elliot Reproduction Source Fixtures

These fixtures preserve the exact report-linked WEPPcloud source surfaces used
to reconstruct William J. Elliot's April 2026 hardwood and mixed-forest
analysis.

| Site | WEPPcloud run | Config | Selected surface | Topaz ID | Hillslope centroid | Elevation | Land cover | Soil |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Hubbard Brook | `unassailable-sensuousness` | `disturbed9002` | `p1` / `H1` | `22` | `-71.725899, 43.948767` | `551.599976 m` | Deciduous Forest (`41`), 0.9 canopy cover | SSURGO mukey `665220`, sand loam |
| Santee | `clean-burning-griddle` | `disturbed9002` | `p2` / `H2` | `23` | `-79.791594, 33.148681` | `9.30002 m` | Mixed Forest (`43`), 0.9 canopy cover | SSURGO mukey `131976`, sand loam |

The report-level run coordinates are `-71.7253057471, 43.9537833388` for
Hubbard Brook and `-79.7923328685, 33.1513542598` for Santee. They identify the
WEPPcloud projects; the table records the selected hillslope centroids from
each source `hillslopes.parquet`.

Both run controls encode 100 years, and the synthetic climate records span
simulation years 1 through 100. Their CLIGEN headers identify version `5.32300`,
seed `12345`, and station files `nh275639.par` and `sc381544.par`.

Each site directory contains:

- exact selected hillslope inputs and sidecars from `wepp/runs/`;
- exact selected `H*.dat` outputs and both WEPPcloud return-period JSON files;
- compact root controller metadata sufficient to identify configuration,
  coordinates, land use, soils, watershed, and WEPP settings; and
- the source hillslope table used to resolve WEPP `p*` IDs to Topaz IDs,
  centroids, and elevations; and
- a `SHA256SUMS` manifest over every retained source-native file.

The exact `p1.man` and `p2.man` inputs are the site-specific constant-cover
comparators. Both disable plant growth, senescence, and decomposition while
holding canopy, rill cover, and interrill cover fixed. Their only byte-level
parameter difference is initial residue mass: `1.00 kg/m2` at Hubbard Brook
and `1.45 kg/m2` at Santee. `constant-cover-comparators.json` provides the
machine-readable path, identity, operands, and invariants for the CAL-02
harness.

The fixture does not claim to be Bill's WEPP Windows project. Bill manually
transcribed WEPPcloud slope and soil information through the Windows file
builder; those converted files and his Windows output files were not present in
the report-linked runs.

Large `.cli`, `.dat`, and `.parquet` files are routed through path-confined Git
LFS rules.
