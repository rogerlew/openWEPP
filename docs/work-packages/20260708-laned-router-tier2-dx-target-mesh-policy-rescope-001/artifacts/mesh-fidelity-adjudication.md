# Mesh Fidelity Adjudication

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Ran.

## Decision

Reject production target-`dx` promotion for this package. Retain the active
production default fixed `10 cells/OFE`.

## Evidence Basis

The package landed the missing diagnostic trace surface and reran the ladder
with candidate-vs-reference comparisons over the declared surfaces:
- daily terminal routed-outlet mass vector,
- D13 routed hourly erosion-shape weights,
- annual pass-sediment sums,
- active closure hard guards,
- `routed_end_window_storage_m3`,
- `routed_tail_fold_m3`,
- `days_uniform_shape`,
- `lane_days_erosion_source_shape_degenerate`.

`mn_corn_h4` and `n_idaho_forest_h1` passed the fine-reference adequacy and
candidate comparisons. They do not justify a production change by themselves:
the current fixed baseline also passes the same comparisons, and target-`dx`
adds cost on long OFEs.

`wa_cascades_forest_h1` blocks promotion:
- `dx2p5` failed the active closure guard at day 1122.
- `dx1p25` failed the active closure guard at day 1122.
- `dx10` and `dx5` completed but produced non-promotable routed outlet/storage
  magnitudes before the fine-reference rungs could close.

H2637 remains synthetic stress only and failed the adequacy tolerance on routed
shape and annual sediment surfaces.

## Candidate Verdicts

| Policy | Verdict |
|--------|---------|
| Fixed `10 cells/OFE` | Retained as production default. |
| Target `dx20` | Not promoted; no real-cohort benefit sufficient to justify change, and WA fine reference did not close. |
| Target `dx10` | Rejected; WA outlet/storage magnitudes are non-promotable. |
| Target `dx5` | Rejected; WA outlet/storage magnitudes are non-promotable. |
| Target `dx2p5` / `dx1p25` | Rejected as production/reference rungs for WA under current closure guard. |

## Follow-On

No immediate mesh-policy hold-lift package is recommended. The next useful
work is investigation of the WA day-1122 high-resolution closure/magnitude
pathology under a narrow numerics bug package, not a production policy flip.
