# Caption

**CAL-06 congruence verdict matrix.** Categorical status and advancement
outcome for every prespecified canopy, snow, litter, residue, frost,
interception/ET, runoff, and erosion cell.

Paired figure: `cal06-congruence-verdict-matrix.svg`.

## Ancillary information

- Triangle symbols denote `BOUNDED`; dashes denote `NOT_EVALUATED`.
- The matrix contains all 13 cells frozen before result-bearing execution.
- `NOT_EVALUATED`, `NOT_ADVANCED`, authority-missing null, unbound
  observation, and measured zero remain distinct states.
- The package-level result is
  `COMPLETE / BOUNDED GRADIENT CHARACTERIZATION / DOWNSTREAM ADVANCEMENT
  WITHHELD`.
- Quantitative results and rationales remain in the machine-readable verdict
  table rather than inside the plot.

## Source data

- [`cell-contract.csv`](../cell-contract.csv)
- [`verdict-matrix.csv`](../verdict-matrix.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)

