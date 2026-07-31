# eb04-failure-chronology

Status: `complete`

Evidence mode: `Ran`

Figure: [`eb04-failure-chronology.svg`](eb04-failure-chronology.svg)

## Caption

When did each failed cell first reach its typed domain rejection? Every failed cell in the single fixed round. One-based simulation day index.

## Question

When did each failed cell first reach its typed domain rejection?

## Population

Every failed cell in the single fixed round.

## Units

One-based simulation day index.

## Processing

The index is one day after the last successfully published trace row.

## Uncertainty

Fixture start dates differ, so day index supports within-run chronology but not cross-site calendar attribution.

## Exclusions

Completed cells are absent because they have no failure day.

## Interpretation

Late failures show why short smoke fixtures did not establish population-wide runtime admissibility.

## Limitation

Twenty-two failures use a wrapper that reports layer density as the rejected conductivity-path value; two are prior-layer thickness reconciliation failures. EB-04 does not infer the hidden meteorology sub-error.
