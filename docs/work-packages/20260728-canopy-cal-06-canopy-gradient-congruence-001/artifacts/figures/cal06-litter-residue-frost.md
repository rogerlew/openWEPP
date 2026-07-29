# Caption

**CAL-06 litter, residue, and frost chronology.** Daily leaf-litter transfer,
surface-residue mass, residue depth, and frost depth for the Marcell canopy
strata. Lines show ensemble medians and shaded bands span the frozen timing
ensemble.

Paired figure: `cal06-litter-residue-frost.svg`.

## Ancillary information

- Leaf litter is expressed in kg/m²/day, surface residue in kg/m², residue
  depth in meters, and frost depth in millimeters.
- Predictive needle and fine-woody sources remain
  `NULL_AUTHORITY_MISSING` in all 259 forest/member runs; null is not zero.
- Total litter and decay adequacy remain unevaluated. Residue and frost cells
  are `NOT_ADVANCED`.
- Frost onset and thaw distributions are retained in
  `ensemble-summary.csv`; chronology alone does not establish adequacy of the
  incomplete source composition.

## Source data

- [`daily-climatology.csv`](../daily-climatology.csv)
- [`ensemble-summary.csv`](../ensemble-summary.csv)
- [`verdict-matrix.csv`](../verdict-matrix.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)

