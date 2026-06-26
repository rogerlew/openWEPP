# Physics-Bulk Snowbench Adjudication

- Schema: `snowdensity04-physics-bulk-adjudication-v1`
- Disposition: `NON-PROMOTION`
- Best variant: `density_compaction_v1`
- Beats openWEPP as-built: `False`
- Beats legacy as-built: `False`
- No site constants: `True`
- Runtime coupling: `none; offline snowbench candidate only`

## Comparator Summary

| Model | Robust fail | Robust score | Density fail | Density score | Robust counts |
|---|---:|---:|---:|---:|---|
| `openwepp_as_built` | 9 | 84 | 9 | 16 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `legacy_as_built` | 9 | 84 | 9 | 16 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `pysnobal` | 28 | 11 | 12 | 4 | `{"fail": 28, "marginal": 6, "pass": 1, "strong": 1, "unavailable": 24}` |

## Candidate Summary

| Variant | Model | Robust fail | Robust score | Density fail | Density score | Robust counts | vs openWEPP | vs legacy |
|---|---|---:|---:|---:|---:|---|---|---|
| `density_compaction_v1` | `physics_bulk_density_compaction_v1` | 18 | 46 | 7 | 22 | `{"fail": 18, "marginal": 15, "pass": 5, "strong": 7, "unavailable": 15}` | `{"better": 8, "equal": 13, "unpaired": 0, "worse": 24}` | `{"better": 8, "equal": 13, "unpaired": 0, "worse": 24}` |

Disposition rule: a candidate must reduce robust fail count and preserve or improve robust ordinal score against both openWEPP as-built and legacy as-built. SNOWDENSITY-06 additionally requires the density/densification robust-cell profile to improve without melt retuning. Comparator agreement is flag evidence only under ADR-0017.
