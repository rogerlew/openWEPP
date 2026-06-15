# CQR12 Numeric Equivalence

Status: complete.

Static: production changes are private helper extraction and import expansion.
No constants, formulas, unit conversions, float expression grouping, or output
publication formulas were intentionally changed.

Static: value-order preservation:

- header insertion and validation order remains enabled, element count, system
  type, schedule type, minimum depth, optional maximum depth, period count;
- period insertion and validation order remains element id, depletion trigger,
  start/end date fields, then sprinkler/furrow-specific fields;
- sprinkler fields retain rate, depth ratio, and nozzle factor validation and
  projection meanings;
- furrow fields retain end element, supply rate, supply duration, and fill
  ratio validation and projection meanings.

Ran:

- focused CQR12 projection/guard characterization before and after refactor,
  exit `0`;
- `cargo test --workspace`, exit `0`.

Conclusion: behavior-preserving decomposition only; no numeric-equivalence
delta identified.
