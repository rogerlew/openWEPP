# PL15R Semantic-Parity Direction Assessment

Status: `complete`
Evidence mode: `Static + Ran`
Assessment: `ACCEPTANCE-POSITIVE (Tier-A required surfaces closed)`

Static:
- Semantic parity (not bitwise parity) remains project target.
- Tier-A single-OFE daily water-balance surfaces remain high-confidence
  acceptance-direction signals.

Ran:
- PL14R schema-aligned strict replay artifacts show:
  - `H5.wat.dat`: `strict_pass=true` with `identical=1`
  - `H5.plot.dat`: `strict_pass=true` with `identical=1`
- Day-by-day keyed parity artifact shows:
  - `all_columns_exact=true`
  - `common_row_count=1095`

## Direction Verdict

`ACCEPTANCE-POSITIVE`

Why:

1. Both required Tier-A surfaces pass strict replay in the refreshed evidence
   lane.
2. `H5.wat.dat` parity is exact on all 25 canonical measures day-by-day under
   shared keyspace.
3. No unresolved Tier-A blocker remains after supersession classification.

## Scope Note

This assessment is scoped to PL15R Tier-A required surfaces and does not claim
that all non-Tier-A lanes are closed as release gates.
