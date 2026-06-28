# Authority Gap And Disposition

Evidence mode: `Static + Ran`

## Authority Checked

Sturm 2010 Table 4 and Eq. 6 provide a class-density trajectory form:

`rho = (rho_max - rho_0) * (1 - exp(-k1 * depth_cm - k2 * day_of_year)) + rho_0`

The available local table covers:

| Class | rho_max | rho_0 | k1 | k2 |
|---|---:|---:|---:|---:|
| alpine | 0.5975 | 0.2237 | 0.0012 | 0.0038 |
| maritime | 0.5979 | 0.2578 | 0.0010 | 0.0038 |
| prairie | 0.5940 | 0.2332 | 0.0016 | 0.0031 |
| tundra | 0.3630 | 0.2425 | 0.0029 | 0.0049 |
| taiga | 0.2170 | 0.2170 | 0.0000 | 0.0000 |

Ephemeral is part of the six-class Sturm snow-class system, but the local
Sturm 2010 authority states ephemeral measurements were excluded and supplies no
parameter row.

The numeric Sturm 1995 binary decision-tree thresholds are now source-verified
from `references/copyrighted/sturm1995.pdf` and recorded in
`sturm-thresholds-source-verification.md`. The verified thresholds are:

- `Tc=10 degC`
- `CDM=30/125 degC-month`
- `SPR=2 mm day^-1`
- wind low/high evidence bracketed by `0.5-2.0 m s^-1`

Sturm/Liston 2021 is a cross-check only: it changes the ephemeral threshold to
`61 degC-month`, the precipitation threshold to `4 mm day^-1`, and renames
Taiga/Alpine to Boreal Forest/Montane Forest.

## Runtime Boundaries

- Sturm 1995 brackets actual wind separation but does not select a single
  numeric wind cutoff. The candidate therefore fails closed for wind-dependent
  branches inside `0.5 < wind < 2.0 m s^-1`.
- The rare deep-tundra/deep-taiga branches fail closed because they are not
  standard six-class labels.
- Ephemeral has no Sturm 2010 density row; the candidate uses the documented
  process-first fresh-snow/Anderson fallback rather than fabricated parameters.
- Promotion requires the real cross-SNOTEL primary rubric, bidirectional
  densification flip, persistence guardrail, and conservation evidence.

## Rerun Gate Result

The real cross-SNOTEL+cancov direct-production rerun completed after source
verification:

- activated default: `15` robust fails / `179` robust score;
- climate-class candidate: `16` robust fails / `168` robust score;
- candidate robust improvements: `4`;
- candidate robust regressions: `13`;
- bidirectional densification flip: failed, with
  `harvard_open:seasonal_densification_trajectory:-1`;
- conservation: passed, with `159986` candidate trace rows, max snow-state
  residual `4.440892098500626e-16 m`, and max partition residual
  `5.551115123125783e-17 m` under `1e-9 m`.

## Disposition

The selector is reserved and fail-closed. The source authority gap is closed,
but the candidate is not promoted and the default is not changed because the
primary observed-data rubric, bidirectional densification flip, and persistence
guardrail failed.
