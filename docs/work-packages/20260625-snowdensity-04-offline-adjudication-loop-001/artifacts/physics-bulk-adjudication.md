# SNOWDENSITY-04 Physics-Bulk Adjudication

- Schema: `snowdensity04-physics-bulk-adjudication-v1`
- Disposition: `PROMOTION-CANDIDATE`
- Best variant: `dense_slow_melt_v1`
- Beats openWEPP as-built: `True`
- Beats legacy as-built: `True`
- No site constants: `True`
- Runtime coupling: `none; offline snowbench candidate only`

## Comparator Summary

| Model | Robust fail | Robust score | Robust counts |
|---|---:|---:|---|
| `openwepp_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `legacy_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `pysnobal` | 28 | 11 | `{"fail": 28, "marginal": 6, "pass": 1, "strong": 1, "unavailable": 24}` |

## Candidate Summary

| Variant | Model | Robust fail | Robust score | Robust counts | vs openWEPP | vs legacy |
|---|---|---:|---:|---|---|---|
| `candidate_v1` | `physics_bulk_candidate_v1` | 24 | 34 | `{"fail": 24, "marginal": 13, "pass": 3, "strong": 5, "unavailable": 15}` | `{"better": 4, "equal": 15, "unpaired": 0, "worse": 26}` | `{"better": 4, "equal": 14, "unpaired": 0, "worse": 27}` |
| `slow_melt_v1` | `physics_bulk_slow_melt_v1` | 6 | 95 | `{"fail": 6, "marginal": 8, "pass": 3, "strong": 27, "unavailable": 16}` | `{"better": 14, "equal": 25, "unpaired": 0, "worse": 5}` | `{"better": 15, "equal": 23, "unpaired": 0, "worse": 6}` |
| `dense_slow_melt_v1` | `physics_bulk_dense_slow_melt_v1` | 6 | 102 | `{"fail": 6, "marginal": 3, "pass": 6, "strong": 29, "unavailable": 16}` | `{"better": 17, "equal": 24, "unpaired": 0, "worse": 3}` | `{"better": 18, "equal": 22, "unpaired": 0, "worse": 4}` |
| `cold_dense_slow_melt_v1` | `physics_bulk_cold_dense_slow_melt_v1` | 15 | 65 | `{"fail": 15, "marginal": 4, "pass": 5, "strong": 17, "unavailable": 19}` | `{"better": 12, "equal": 12, "unpaired": 0, "worse": 17}` | `{"better": 13, "equal": 10, "unpaired": 0, "worse": 18}` |

Disposition rule: a candidate must reduce robust fail count and preserve or improve robust ordinal score against both openWEPP as-built and legacy as-built. Comparator agreement is flag evidence only under ADR-0017.
