# Variant Ledger

Static:

- All variants are global named candidates.
- No variant accepts per-site constants.
- No variant changes production runtime activation, default behavior, parser
  configuration, compatibility runtime, or output publication.

## Variants

| Variant | Model ID | Purpose | Candidate-only constants |
|---|---|---|---|
| `candidate_v1` | `physics_bulk_candidate_v1` | SNOWDENSITY-03 baseline | fresh snow min/base `50 kg m^-3`, max `200 kg m^-3`; degree melt `0.18 kg m^-2 degC^-1 h^-1`; solar melt efficiency `0.02`; dry/wet compaction multipliers `1/1`. |
| `slow_melt_v1` | `physics_bulk_slow_melt_v1` | Test whether first-candidate failures are dominated by excessive melt/early ablation. | candidate constants except degree melt `0.05`, solar melt efficiency `0.005`. |
| `dense_slow_melt_v1` | `physics_bulk_dense_slow_melt_v1` | Test slower melt plus globally denser fresh snow and stronger densification. | fresh snow min/base `75 kg m^-3`, max `250 kg m^-3`; degree melt `0.05`; solar melt efficiency `0.005`; dry/wet compaction multipliers `4/2`. |
| `cold_dense_slow_melt_v1` | `physics_bulk_cold_dense_slow_melt_v1` | Test a colder, more conservative pack with dense slow melt. | dense-slow-melt constants except degree melt `0.03`, solar melt efficiency `0.002`, cold-content relaxation `0.03 h^-1`. |

## Promotion-Candidate Rule

The SNOWDENSITY-04 rule is profile-based and comparator-aware:

```text
candidate robust_fail_count < comparator robust_fail_count
and candidate robust_ordinal_score >= comparator robust_ordinal_score
```

The candidate must satisfy that rule against both `openwepp_as_built` and
`legacy_as_built`. This is not default activation and does not make legacy a
correctness target; it is the package-local offline adjudication threshold for
whether a variant earns the next runtime opt-in package.
