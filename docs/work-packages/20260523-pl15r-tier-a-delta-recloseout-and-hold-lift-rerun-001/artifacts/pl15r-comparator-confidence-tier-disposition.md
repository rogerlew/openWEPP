# PL15R Comparator Confidence-Tier Disposition

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier policy remains strict: unresolved Tier-A deltas block promotion unless
  explicitly risk-accepted.
- PL15R supersession authority requires active blocker classification from the
  latest schema-aligned replay evidence set.

Ran:
- Consumed PL14R schema-aligned strict replay artifacts:
  - `h5_wat_comparator_schema_aligned.json`
  - `h5_plot_comparator_schema_aligned.json`
  - `h5_wat_day_by_day_schema_aligned.json`

## PL15R Disposition Records

| tier | surface_id | delta_signature | first_divergence_surface | first_divergence_timestep | decision | evidence_mode |
|---|---|---|---|---|---|---|
| `Tier-A` | `single-ofe.daily-water-balance.H5.wat.dat` | `strict_pass=true; identical=1; day_by_day_all_columns_exact=true; common_row_count=1095` | `none` | `none` | `accept` | `Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.H5.plot.dat` | `strict_pass=true; identical=1` | `none` | `none` | `accept` | `Ran` |

## Disposition Summary

1. Required Tier-A include surfaces now pass strict replay in the schema-aligned
   evidence lane.
2. Residual Tier-A blocker set is empty.
3. Pre-alignment PL14/PL14R failure signatures are retained as superseded
   historical context and are not active blockers in PL15R.
