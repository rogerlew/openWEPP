# CAL-05 Interpretation Figures

Status: `REPRODUCIBLE POST-CLOSURE VISUALIZATION SUPPLEMENT`

Evidence class: `Ran`

These figures visualize the frozen CAL-05 synthetic results for human
interpretation. They do not add observations, change an execution axis, select
a parameter, or upgrade calibration or predictive-source status.

## Figures

1. [`cal05-terminal-stock-response.svg`](cal05-terminal-stock-response.svg)
   shows the 4-by-4 terminal-stock response to the frozen annual-input and
   decay axes. It makes the joint source/decay control visible.
2. [`cal05-daily-recovery.svg`](cal05-daily-recovery.svg) shows that the
   complete daily stock series uniquely recovers `S020-K050` within the frozen
   grid while every other candidate has positive error.
3. [`cal05-source-decay-ridge.svg`](cal05-source-decay-ridge.svg) shows the
   five analytically constructed source/rate pairs and their distinct
   trajectories to the same year-20 stock.

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
timestamp or environment-dependent metadata.
