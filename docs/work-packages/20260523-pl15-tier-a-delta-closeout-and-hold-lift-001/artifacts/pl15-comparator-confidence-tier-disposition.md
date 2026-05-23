# PL15 Comparator Confidence-Tier Disposition

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier policy requires unresolved Tier-A strict deltas to remain blocking unless
  formally risk-accepted with explicit approval reference.

Ran:
- Consumed direct PL14 strict replay artifacts:
  - `artifacts/h5_wat_comparator.json`
  - `artifacts/h5_plot_comparator.json`

## PL15 Disposition Records (Direct PL14 Replay Evidence)

| tier | surface_id | delta_signature | first_divergence_surface | first_divergence_timestep | decision | evidence_mode |
|---|---|---|---|---|---|---|
| `Tier-A` | `single-ofe.daily-water-balance.H5.wat.dat` | `structure_diff; line_count_baseline=1123; line_count_candidate=5; line_count_mismatch=1118; numeric_arity_mismatch_lines=1` | `H5.wat.dat` | `line-level structural divergence` | `block` | `Ran` |
| `Tier-A` | `single-ofe.daily-water-balance.H5.plot.dat` | `missing candidate artifact; only_baseline_count=1; only_baseline_examples=["H5.plot.dat"]` | `H5.plot.dat` | `artifact presence check` | `block` | `Ran` |

## Disposition Summary

1. Both required Tier-A include surfaces remain strict failures (`strict_pass=false`).
2. No row was down-classified to Tier-B/Tier-C.
3. Tier-A blocker set is non-empty and remains promotion-blocking.
4. Claude pre-closeout physics review (`KERNEL-GAP-001..012`) is tracked as
   additional scope-honesty governance input and dispositioned via follow-on
   queue rows; it does not down-classify Tier-A comparator blockers.
