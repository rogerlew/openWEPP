# DC CQR HA-07 — Non-Finite Growth Authority

Status: `TERMINAL-PASS`

## Objective

Close `DC-CQR-HA07-001`: `direct_production_typed_growth_crop_authority`
accepts a required `bbb = NaN` scalar and returns typed crop authority
containing NaN, violating `SC-PLANT-001#INV-PLANT-021`.

## Correction Authority Envelope

- Canonical authority: confirmed `SC-PLANT-001#INV-PLANT-021` requires every
  required growth symbol to be present, finite, and domain-valid or fail with a
  typed boundary error. No contract amendment is required.
- Production write set:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`, limited to required-scalar finite validation.
- Test write set:
  `crates/openwepp-runner/src/hillslope/tests03/cqr_growth_authority.rs` and its
  existing registration.
- Allowed edit: reject a present non-finite required scalar with
  `HillslopeCliError`, preserving exact symbol identity, existing missing-value
  behavior, all finite values, integral/range validation, and consumer order.
- Excluded: growth equations, domain thresholds for finite scalars, schemas,
  schedule precedence, fallback/default values, and other process families.
- Acceptance: NaN and infinity regression tests fail before and pass after;
  missing/finite/integral behavior remains green; focused runner tests, Clippy,
  format, diff, and HA-07 remeasurement pass.
- Security impact: none; the change strengthens fail-closed numeric validation.

Conversion rule: the reproduced mechanism is in-envelope, canonically
authorized, safe, and directly testable, so this package must land the
production correction and may not close as HOLD for implementation effort.

## Progress

- [x] Reproduce `bbb = NaN` acceptance.
- [x] Confirm `INV-PLANT-021` finite required-symbol authority.
- [x] Add contract-derived NaN/infinity regressions.
- [x] Correct the production required-scalar helper.
- [x] Run focused validation and HA-07 remeasurement.
- [x] Complete dual review and verification/disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared source/tests and evidence. Expected outputs are the correction,
focused results, reviews, verification, and terminal disposition.

## Outcomes

The required-scalar helper now rejects every present non-finite value with a
typed error retaining the exact symbol and observed value. Missing and finite
values retain prior behavior; integral/range validation remains downstream.
Focused tests pass `4/4`, runner profile passes `108/108`, Clippy/format/diff
pass, and the HA-07 slice has zero CRAP rows above 30. Two independent reviews
and verifications pass with no unresolved finding. Disposition:
`TERMINAL-PASS`; defect `DC-CQR-HA07-001` is closed.
