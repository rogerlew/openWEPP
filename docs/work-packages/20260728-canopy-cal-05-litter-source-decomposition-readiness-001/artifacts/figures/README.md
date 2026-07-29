# CAL-05 Interpretation Figures

Status: `REPRODUCIBLE POST-CLOSURE VISUALIZATION SUPPLEMENT`

Evidence class: `Ran`

These figures visualize the frozen CAL-05 synthetic results for human
interpretation. They do not add observations, change an execution axis, select
a parameter, or upgrade calibration or predictive-source status.

## Figures

| Figure | Markdown sidecar | Interpretation |
| --- | --- | --- |
| [`cal05-terminal-stock-response.svg`](cal05-terminal-stock-response.svg) | [`cal05-terminal-stock-response.md`](cal05-terminal-stock-response.md) | The 4×4 terminal-stock response to the frozen annual-input and decay axes. |
| [`cal05-daily-recovery.svg`](cal05-daily-recovery.svg) | [`cal05-daily-recovery.md`](cal05-daily-recovery.md) | Complete daily stock uniquely recovers `S020-K050` within the frozen grid. |
| [`cal05-source-decay-ridge.svg`](cal05-source-decay-ridge.svg) | [`cal05-source-decay-ridge.md`](cal05-source-decay-ridge.md) | Five source/rate pairs follow distinct paths to the same year-20 stock. |

Each same-basename sidecar is written for readers who encounter the figure
outside the work-package narrative. It explains why the figure exists, how to
read it, the plain-language takeaway, units and quantitative context, exact
source links, and the interpretation/authority boundaries that must accompany
the plot.

All quantities are synthetic and retain the package's
`ASSUMED_FOR_EXECUTION` classification. The figures are explanatory evidence,
not empirical calibration, natural source authority, probability, or
physiological bounds.

## Reproduction

From the repository root:

```bash
.venv/bin/python \
  docs/work-packages/20260728-canopy-cal-05-litter-source-decomposition-readiness-001/tools/plot_results.py
```

The renderer uses only the Python standard library and reads:

- `deterministic-design.csv`;
- `reconstruction-results.csv`;
- `synthetic-recovery.csv`;
- `terminal-stock-ridge-design.csv`; and
- `ridge-producer-results.csv`.

The SVG output contains accessible titles and descriptions and has no
timestamp or environment-dependent metadata. The Markdown sidecars are
authored companion documents and are not rewritten by the renderer.
