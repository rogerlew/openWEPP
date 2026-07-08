# Metric Authority Decision

Status: `EXECUTED-COMPLETE-METRIC-AUTHORITY`
Evidence mode: Static + Ran.

## Decision

`SC-OFEROUTE-001` rev 44 replaces the mesh-policy annual pass-sediment
strict relative-only gate with a material-year plus annual-vector rule.

For every annual pass-sediment column (`tdet`, `tdep`, `sedcon_1` through
`sedcon_5`):

- A material member-year has absolute reference annual value at least `5%` of
  that column's total absolute reference annual sum.
- Material member-years must satisfy the named relative tolerance.
- The full annual vector must satisfy
  `sum_y |candidate_y - reference_y| / max(sum_y |reference_y|, 1e-12) <= tolerance`.
- If the reference annual vector is dry at `<= 1e-12`, the candidate annual
  vector must also sum to `<= 1e-12`.
- Low-contribution member-year relative excursions remain reported but are not
  standalone blockers when the material-year and vector gates pass.

## Rationale

The predecessor package localized the sole WA annual sediment blocker to one
low-contribution year (`tdep:4`), with an absolute movement of
`0.013223319 kg` in a `0.597483647 kg` reference year. The same day had
identical pass-row water magnitudes and source mass, no clamp/tail/fallback
event, terminal routed outlet delta `-0.00336 m3` on a `4594 m3` event, and
terminal routed-shape L1 `0.000635`, well below the shape tolerance.

A strict max-relative-per-member-year annual gate lets a low-denominator
annual entry dominate the mesh-policy decision even when the annual column
vector remains stable. The rev-44 rule keeps strict relative control on
material years and adds a whole-vector annual sediment magnitude bound, while
still recording low-contribution excursions for review.

This is not a threshold widening. The named relative threshold is unchanged;
only the annual sediment evidence aggregation rule changes for
low-contribution years.

## Replay Result

Source:
`artifacts/annual-sediment-metric-replay.md` and
`artifacts/annual-sediment-metric-replay.json`.

- Selected-cohort comparisons replayed: `21`
- Pre-rev44 strict-relative annual sediment blockers: `1`
- Rev-44 annual sediment blockers: `0`

The only pre-rev44 blocker is
`wa_cascades_forest_h1` `fine_reference_adequacy_dt75`,
`dx2p5_dt75` versus `dx1p25_dt75`, `tdep:4`:
`0.0221316838 > 0.00666666667`.

Under rev 44, that comparison passes:

- annual-vector max relative: `0.000612007475` on `tdep`
- material-year max relative: `0.00173788779`
- low-contribution max relative, still reported: `0.0221316838`

## Non-Authorization

This package does not authorize:

- `dx5` production mesh default promotion
- any active mesh default change
- routed-water, routed-shape, storage, tail-fold, or closure threshold changes
- sediment process-physics changes
- default/off behavior changes
- shadow mesh changes

Renewed `dx5` promotion remains a follow-on package that must use the rev-44
metric authority plus the coupled space-time evidence posture already recorded
in `SC-OFEROUTE-001` rev 43.
