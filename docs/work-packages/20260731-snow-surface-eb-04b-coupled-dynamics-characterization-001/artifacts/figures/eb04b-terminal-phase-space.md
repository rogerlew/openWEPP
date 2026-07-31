# EB-04B Terminal Phase Space

## Caption

Retained cold content versus ice mass for all 22 thermal-family rejected
control volumes. The line is the independently calculated `0 K` boundary,
`Q_cc = m c_i 273.15`. Circles crossed the boundary; triangles were rejected
slightly earlier when saturation vapor pressure underflowed.

## How To Read This Figure

Distance above the line means the cold-content-to-heat-capacity ratio implies a
temperature below absolute zero. All points are rejected micro-volume slices,
not a claim about complete seasonal pack mass. Color identifies the enabled cell, while marker
shape identifies the exact typed rejection.

## Provenance And Limits

Generated from EB-04A typed snapshots by `tools/run_analysis.py`. Temperature
is independently reconstructed with the canonical `2100 J kg^-1 K^-1` ice
heat capacity. The plot proves the proximate state; it does not select a
corrective extinction or phase-change formulation.
