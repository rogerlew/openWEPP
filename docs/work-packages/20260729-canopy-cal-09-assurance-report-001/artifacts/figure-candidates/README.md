# CAL-09 Candidate Figures

Status: `retained report assets — figure contract frozen`

These operator-reviewed figures retain the frozen scientific content,
timescale, aggregation, and layout. All eight figures, their sidecars, and
their plotted source/derived tables are declared public-safe research objects
in the unadmitted CAL-09 report source.

| Candidate | Sidecar | Main question |
| --- | --- | --- |
| [`f1-coefficient-response.svg`](f1-coefficient-response.svg) | [`f1-coefficient-response.md`](f1-coefficient-response.md) | How accepted coefficient combinations change seasonal model dynamics |
| [`f2-forest-class-seasonality.svg`](f2-forest-class-seasonality.svg) | [`f2-forest-class-seasonality.md`](f2-forest-class-seasonality.md) | How forest classes differ through the year |
| [`f3-litter-residue-frost.svg`](f3-litter-residue-frost.svg) | [`f3-litter-residue-frost.md`](f3-litter-residue-frost.md) | How leaf-off propagates into residue and frost state |
| [`f4-temperate-observed-modeled-timing.svg`](f4-temperate-observed-modeled-timing.svg) | [`f4-temperate-observed-modeled-timing.md`](f4-temperate-observed-modeled-timing.md) | Whether temperate calibration timing transfers |
| [`f5-source-decay-trajectories.svg`](f5-source-decay-trajectories.svg) | [`f5-source-decay-trajectories.md`](f5-source-decay-trajectories.md) | Why source and decay remain confounded |
| [`f6-canopy-gradient-snow-response.svg`](f6-canopy-gradient-snow-response.svg) | [`f6-canopy-gradient-snow-response.md`](f6-canopy-gradient-snow-response.md) | How canopy gradients and observed snow seasonality compare |
| [`f7-hemisphere-seasonality.svg`](f7-hemisphere-seasonality.svg) | [`f7-hemisphere-seasonality.md`](f7-hemisphere-seasonality.md) | How Southern Hemisphere observed and modeled trends align |
| [`f8-beza-observed-modeled.svg`](f8-beza-observed-modeled.svg) | [`f8-beza-observed-modeled.md`](f8-beza-observed-modeled.md) | Where the Bezà chronology contradiction appears |

Supporting rows:

- `f1-exemplar-coefficients.csv`
- `f4-temperate-timing-summary.csv`
- `f6-observed-snow-climatology.csv`
- `f7-relative-seasonality.csv`
- `source-manifest.csv`

Rebuild from the repository root:

```console
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  docs/work-packages/20260729-canopy-cal-09-assurance-report-001/tools/build_candidate_figures.py
```
