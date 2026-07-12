# DC CQR HB-01 — Erod13 Strict-Positive Threshold

Status: `ACTIVE`

## Objective

Close `DC-CQR-HB01-001`: `compute_direct_erod13` accepts `te_s = 0` even
though `SC-SED-001` requires positive effective runoff duration and positive
denominators/forcing surfaces before the Wave-1 solve.

## Correction Authority Envelope

- Canonical authority: `SC-SED-001#INV-SED-004/005/007`, its Wave-1 algorithm
  (`te > 0` and finite denominators), and the existing typed hard-fail posture.
  No contract amendment is required.
- Observed mechanism: `validate_min(value, WB11_ZERO_THRESHOLD)` subtracts the
  same tolerance inside its comparison, making its effective lower bound zero.
- Production write set:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`,
  limited to Erod13 strict-positive input validation and mechanical helper
  decomposition required to resume HB-01.
- Test write set:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/tests/erosion_hb01.rs`
  and its registration in the target production module.
- Allowed correction: add an Erod13-local finite strict-positive validator and
  apply it to every SC-SED-required positive input while preserving the exact
  existing field order, error variant, and names.
- Excluded: generic `validate_min` semantics, tolerance constants, erosion
  equations, branch thresholds, formulas, accumulation/grouping, schemas,
  downstream consumers, and other process families.
- Acceptance: zero and non-finite values fail with the exact typed field;
  nominal/boundary/deposition vectors and both watdur/sediment closures retain
  behavior; the real R7D6 consumer passes; focused tests and HB-01 remeasurement
  pass with every target/slice row CRAP at most 30.
- Security impact: none; this strengthens fail-closed numeric admission.

Conversion rule: the mechanism is reproduced, in-envelope, canonically
authorized, safe, and directly testable. This package must land the correction
and may not close as HOLD for implementation effort.

## Progress

- [x] Reproduce zero-duration acceptance before production correction.
- [x] Confirm canonical strict-positive authority and local mechanism.
- [ ] Land the Erod13-local strict-positive correction.
- [ ] Resume mechanical HB-01 decomposition and focused measurement.
- [ ] Complete focused validation and dual review/verification disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared production, tests, and evidence. Expected outputs are the correction,
focused metrics, finding dispositions, verification, and terminal disposition.

## Outcomes

Pending execution.
