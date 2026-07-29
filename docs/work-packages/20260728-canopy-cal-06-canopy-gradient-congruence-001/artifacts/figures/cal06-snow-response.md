# Caption

**CAL-06 snow response.** Daily snow-water-equivalent climatology for the
Marcell, Harvard, and Hubbard Brook canopy strata. Lines show ensemble medians
and shaded bands span the complete frozen timing ensemble.

Paired figure: `cal06-snow-response.svg`.

## Ancillary information

- SWE is plotted in millimeters.
- Exact-date depth, SWE, and bulk-density residuals are retained in the
  observation-score tables without an invented acceptance threshold.
- Harvard SWE is excluded as
  `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`.
- Harvard density compares modeled bulk snow density only with HF237-01 daily
  bulk density. Vertical profiles remain
  `NOT_EVALUATED_SCALE_MISMATCH`; the mixed/hemlock comparison is unbound.
- Peak SWE, peak depth, peak bulk density, and melt-out distributions are
  retained in `ensemble-summary.csv`. Common snow residuals are not attributed
  to canopy phenology.

## Source data

- [`daily-climatology.csv`](../daily-climatology.csv)
- [`ensemble-summary.csv`](../ensemble-summary.csv)
- [`observation-score-summary.csv`](../observation-score-summary.csv)
- [`observation-operator-disposition.md`](../observation-operator-disposition.md)
- [`verdict-matrix.csv`](../verdict-matrix.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)

