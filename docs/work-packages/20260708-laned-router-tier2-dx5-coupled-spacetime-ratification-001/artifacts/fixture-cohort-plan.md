# Fixture Cohort Plan

Evidence mode: Static.

## Cohort

Real selected-cohort members:

- `mn_corn_h4`
- `n_idaho_forest_h1`
- `wa_cascades_forest_h1`

Synthetic stress member:

- `h2637` is retained as non-blocking stress evidence under ADR-0037 and must
  not carry fleet-general production-promotion authority.

## Predeclared Ladder

Production-cap rungs:

- `baseline_fixed10_dt300`
- `dx5_dt300`
- `dx2p5_dt300`
- `dx1p25_dt300`

Refined diagnostic max-substep rungs:

- `dx5_dt75`
- `dx2p5_dt75`
- `dx1p25_dt75`

## Binding Comparisons

- Fine-reference adequacy:
  - `dx2p5_dt300` vs `dx1p25_dt300`
  - `dx2p5_dt75` vs `dx1p25_dt75`
- Candidate-vs-reference:
  - `dx5_dt300` vs `dx2p5_dt300`
  - `dx5_dt75` vs `dx2p5_dt75`
- Timestep controls:
  - `dx5_dt300` vs `dx5_dt75`
  - `dx2p5_dt300` vs `dx2p5_dt75`
  - `dx1p25_dt300` vs `dx1p25_dt75`

Promotion requires `dx5_dt300` to pass candidate-vs-reference tolerances and
not be disqualified by the same-`dx` timestep controls. If only `dx5_dt75`
passes, the package holds because changing production max-substep policy is not
in this package's default-flip authority.
