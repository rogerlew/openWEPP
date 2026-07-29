# CANOPY-CAL-06 Human-Interpretation Figure Contract

Status: `FULFILLED — CAL-06 COMPLETE / BOUNDED CHARACTERIZATION`

Evidence class: `Static`

## Purpose

CAL-06 must publish figures that let a human inspect within-site canopy
gradients without collapsing CAL-04B uncertainty, hiding CAL-05 source
incompleteness, or promoting downstream effects before their upstream process
cells pass.

The original experiment map is
[`figures/cal06-experiment-map.svg`](figures/cal06-experiment-map.svg). It is a
design artifact, not a result.

The plot-only result figures, same-basename Markdown caption sidecars, exact
source-table bindings, and renderer are retained in
[`CANOPY-CAL-06 artifacts/figures`](../work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/artifacts/figures/README.md).
All six required views are present. Ancillary information records the Harvard
SWE exclusion, unavailable predictive litter sources, and gated downstream
cells distinctly from measured zero without placing prose inside the plots.

## Required result figures

The CAL-06 work package must bind exact source tables and reproduce at least:

1. **Canopy chronology by site and stratum.** Daily canopy cover, LAI, and GSI
   summaries for open, deciduous, mixed, and conifer/evergreen lanes, with the
   complete retained CAL-04B ensemble shown as a band or member distribution.
2. **Seasonal ordering and amplitude.** Winter floor, summer closure, seasonal
   amplitude, and observed comparison intervals where compatible observations
   exist. Ordering alone is insufficient.
3. **Snow response small multiples.** Accumulation, peak SWE, density, melt-out
   date, and timing for matched within-site strata, with canopy-independent
   residuals labeled rather than attributed to phenology.
4. **Litter, residue, and frost chronology.** Tissue-separated daily sources,
   source completeness, surface residue, residue depth, frost onset, and thaw.
   Missing predictive needle or fine-wood sources must remain visibly null.
5. **Downstream consequence panels.** Interception, ET, runoff, and erosion
   only for lanes whose upstream canopy, snow, residue, and frost cells pass.
   Failed upstream cells receive a visible `NOT ADVANCED`, not a zero.
6. **Congruence verdict matrix.** Every prespecified quantitative cell by site
   and stratum, including `PASS`, bounded contrary evidence, failure, missing
   input, and nonapplicability.

## Visual integrity requirements

- Calibration and independent-validation observations remain visually
  distinct.
- Ensemble uncertainty is not replaced by a selected best member.
- Units, aggregation period, site, stratum, and observation role appear on the
  plot or in its adjacent Markdown sidecar/machine-readable manifest.
- Missing, unrepresented, not-applicable, and measured zero are distinct.
- Every plot has a same-basename Markdown sidecar linking the exact tidy source
  table and deterministic renderer.
- Color is not the only carrier of status or stratum identity.
- Accessible SVG is the canonical vector format; raster exports are derivative
  conveniences only.

## Advancement use

Figures support interpretation but do not replace machine-readable gates,
independent reconstruction, or consumer-path evidence. A visually plausible
gradient cannot override a failed quantitative cell.
