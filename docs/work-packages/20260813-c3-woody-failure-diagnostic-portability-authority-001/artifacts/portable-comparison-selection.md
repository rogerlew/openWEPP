# Portable Rejected-Failure Diagnostic Comparison Selection

Evidence: `Static + Ran`

## Observed Pair

| Source | `step_norm` |
|---|---:|
| frozen CPython V5 oracle | `3925.8532969524972` |
| Rust production diagnostic | `3925.8544224384018` |
| absolute delta | `0.0011254859045948251` |

The relative delta against the larger magnitude is approximately
`2.86686e-7`. Failure identity, iterations, backtracking count, active-cap
order, candidate absence, and rollback agree exactly.

## Selected Rule

After all exact eligibility checks pass, two values `a` and `b` compare equal
when:

```text
abs(a-b) <= 3e-7 * max(abs(a), abs(b))
```

The decimal constant is evaluated in binary64. No absolute tolerance exists.
The observed allowance is `0.0011777563267315204`, so the measured delta
uses about 95.6% of the permitted envelope.

## Eligibility and Anti-Laundering

The tolerance is unavailable unless all of the following compare exactly:

- model, transaction, occupancy, pass, solve, and diagnostic field identity;
- optional presence/null shape;
- typed failure and candidate absence;
- iteration/backtracking counts, ordered bounds/caps, branches, and array order;
- finite classification, sign class, and zero/nonzero class;
- byte-identical rollback evidence.

NaN, either infinity, and any negative `step_norm` are invalid evidence. Zero
cannot compare equal to a nonzero value;
`+0.0` and `-0.0` form one exact zero class. The rule cannot accept a solve,
modify a residual threshold, repair
a wrong unit or field, or compare accepted state/flux/authorization/closure.
