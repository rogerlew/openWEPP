# PL15R Comparator Confidence-Tier Disposition

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier policy remains strict: unresolved Tier-A deltas block promotion unless
  explicitly risk-accepted.
- PL15R supersession evidence must be provenance-valid for
  `openWEPP-vs-legacy` classification. Legacy-vs-legacy substitution is
  non-authoritative for PL08 hold-lift.

Ran:
- Reviewed PL14R artifacts:
  - `h5_wat_comparator_schema_aligned.json`
  - `h5_plot_comparator_schema_aligned.json`
  - `h5_wat_day_by_day_schema_aligned.json`
  - `pl14r-comparator-run-provenance-manifest.md`
  - `pl14r-schema-aligned-day-by-day-retest.md`

## PL15R Disposition Records (Reversal)

| tier | surface_id | delta_signature | first_divergence_surface | first_divergence_timestep | decision | evidence_mode |
|---|---|---|---|---|---|---|
| `Tier-A` | `single-ofe.daily-water-balance.H5.wat.dat` | `schema_aligned_strict_pass=true; provenance_invalid=openwepp_candidate_not_used; legacy_candidate_substitution_detected` | `candidate provenance chain` | `N/A` | `block` | `Static + Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.H5.plot.dat` | `schema_aligned_strict_pass=true; provenance_invalid=openwepp_candidate_not_used; legacy_candidate_substitution_detected` | `candidate provenance chain` | `N/A` | `block` | `Static + Ran` |

## Disposition Summary

1. Schema-aligned strict-pass artifacts exist but are not authoritative for
   openWEPP parity claims because candidate provenance is legacy-substituted.
2. Active Tier-A blocker set is non-empty due provenance validity failure.
3. PL08 hold-lift remains blocked pending provenance-valid openWEPP replay lane
   evidence and physics-parity package closure.
