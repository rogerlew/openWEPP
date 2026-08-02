# Kernel Profile Compliance

Evidence mode: **Static + Ran**.

- Authority preceded implementation: contract version 121 and the failing
  contract test were written before production Rust.
- Units are explicit at the boundary: physical snow depth is `m` snow; rain,
  snowfall SWE, and melt terms are `m` water equivalent; phase fractions are
  dimensionless; hydrometeor temperature is `degC`.
- Closure uses produced operands and the contract tolerance; no state value is
  canonicalized to make the diagnostic close.
- The existing sum of legacy-inch CoE terms is converted and passed to the
  existing cap/mutation path. Individually converted terms are diagnostic only.
- Typed guards reject non-finite/out-of-domain phase metadata, noncomplementary
  wet-hour fractions, nonzero dry-hour fractions, and component mismatch.
- The real downstream JSONL consumer reads the new fields. Producer-only or
  shadow evidence is not used for closure.
- The observation cohort remains `DIAGNOSTIC_ONLY` under ADR-0042. No fitting,
  efficacy threshold, or promotion decision is admitted.

Result: **COMPLIANT** for behavior-neutral diagnostic observability.
