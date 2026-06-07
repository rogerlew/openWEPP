# Disposition

Evidence mode: `Static:` and `Ran:`.

Status: `executed-hold-boundary`.

The in-envelope soil defect `FQ1-HS-RUNTIME-E-062-SOIL-CORRECTED-LAYER-COVERAGE`
is corrected:

- `SC-SOIL-001` v23 defines `INV-SOIL-017`.
- Parser-layer corrected diagnostic symbols now extend the deepest normalized
  corrected interval to valid parser profile bottom.
- Focused tests pass and preserve invalid-layer fail-closed behavior.
- Post-fix p1-p43 validation has zero `HS-RUNTIME-E-062` failures.

The package cannot close as fully complete because acceptance asked for all 43
single-OFE hillslopes to emit `H.wat.parquet` + `H.hbp` or be reclassified as
genuinely invalid soil. Post-fix validation emitted WAT/HBP for `42/43`; `p11`
now fails later at `HKERNEL-WB11-PERC-E-003` in `percolation_deep_seepage` on
`1990-162`. That mechanism is outside the declared soil parse/runtime mapping
envelope and was not normalized or bypassed.

Disposition: retain the soil correction and hand off the remaining p11 blocker
as a downstream percolation/snow/runoff authority defect. FROSTVAL01 can treat
FQ1 as no longer the population-scale soil parser/runtime blocker, but the frost
rung still has one p11 WAT-publication blocker before a strict 43/43 frost run.
