# Terminal bounded observation seam V2 validators and test matrix

Status: `CANDIDATE / NO SOURCE AUTHORITY`

All validators are executable crate-unit functions over owned DTOs and return
typed errors. They run only after the physical result has returned.

- every `DiagnosticF64` recomputes finiteness from `f64::from_bits`;
- component `delta` is bitwise the live `refined - coarse`; denominator and
  scaled error are recomputed with the live formula;
- component order is ice, liquid, cold, complete energy, unallocated energy;
  the fold is exactly five entries and winner is the first component bitwise
  equal to the maximum (separate from cross-pair `>=` retention);
- pair roles are exactly `COARSE/FULL`, `FINE_1/HALF_1`, `FINE_2/HALF_2` and
  supports join contiguously; all beginning states/joints and half joins match
  field-by-field;
- selection receives the full iteration arena, requires a nonempty bounded
  range, exactly one selected ordinal inside it, matching carrier, final
  convergence and no later iteration for that key;
- provider ordinals are contiguous from zero and provider-call count equals
  the number of closure entries, including failed returns; successful carrier
  records and selected coupling records reference those ordinals exactly once;
- `REJECT_RETRY` pair is immediately followed by the separate attempted
  `0.9375 / 2 = 0.46875` admission, typed `BelowCarrierDomain`, with identical
  before/after provider counts and minimum `0.6` seconds;
- the three ingress witnesses have their required distinct source tags and
  empty exact terminal-kind match vectors;
- NoEvidence/CaptureEvidence snapshots compare every named location and the
  physical error variant/fields bitwise.

Frozen expected bits are: 1.875 `0x3ffe000000000000`; 0.9375
`0x3fee000000000000`; 0.46875 `0x3fde000000000000`; 0.6
`0x3fe3333333333333`; signed refined-minus-coarse complete-energy difference
27.2131278332233 J/m2 `0x403b368f8bb18fb9`; scaled LTE
1918111.5296775517 `0x413d449f8798f2b2`. The capture test must additionally
freeze and assert the two captured complete-energy operand bits; the historical
difference alone is not accepted as a substitute.

The focused fixture is the existing crate-unit
`v9_real_consumer_shadow_wb14_tests::interior_terminal_event_runs_covered_event_and_snow_free_remainder`.
Its capture twin must reuse its setup function and inputs, not duplicate a
descriptive approximation.

Required tests after GO are: DTO construction/closure; each validator positive
and one-field poison cases; all six role/position poisons; first-equal winner;
selection empty/out-of-range/stale poisons; provider exhaustive relation;
three terminal-kind positive poisons; exact pair/floor fixture; and two fresh
fixture executions through NoEvidence and CaptureEvidence with field-by-field
noninterference. Run rustfmt, crate check/tests, V20/V21 structural guards and
diff hygiene. Only after this matrix and real capture execute may final
v21/v11/v139/v6 review start.
