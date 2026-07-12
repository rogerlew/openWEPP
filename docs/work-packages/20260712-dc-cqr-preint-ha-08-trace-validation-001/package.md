# DC CQR HA-08 — Active Trace Numeric Validation

Status: `TERMINAL-PASS`

## Objective And Envelope

Close `DC-CQR-HA08-001`: active trace output accepts non-finite volumes/detail
values (serialized as JSON null), negative routed weights, and nonclosing
weights, rather than failing before publication.

- Authority: `SC-OFEROUTE-001#INV-OFEROUTE-008`, `INV-OFEROUTE-012`, and
  `INV-OFEROUTE-013` require manifest-checksummed trace rows to carry finite
  non-negative volumes/weights and the D13 routed shape; positive-source shapes
  close to unit sum and zero-source shapes are all zero.
- Production write set:
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`,
  limited to typed trace pre-write validation.
- Test write set: the existing HA-08 dedicated test module/registration.
- Allowed edit: validate every numeric field serialized by the row/detail/step
  JSON helpers for finiteness and its existing non-negative domain; validate
  routed weights as finite/non-negative and zero-or-unit closing according to
  source authority; return existing typed runner errors before `fs::write`.
- Protected: JSON keys/schema/order/index bases, numeric values for valid rows,
  routing physics, tolerance authority, selector behavior, and output paths.
- Acceptance: malformed cases fail with field identity and create no output;
  nominal/dry/full-detail rows remain byte-meaning equivalent; focused tests,
  Clippy/format/diff, HA-08 metrics, and two reviews/verifications pass.

The defect is reproduced, in-envelope, canonically authorized, safe, and
testable. The conversion rule requires correction; HOLD for effort is invalid.

Subagent authorization: this package explicitly authorizes the bounded HA-08
implementer and two read-only review/verification agents for this write set.

## Progress

- [x] Reproduce null/negative/nonclosing publication.
- [x] Confirm canonical active-shape/trace authority.
- [x] Add pre-fix-failing typed regressions.
- [x] Implement complete pre-write validation.
- [x] Run focused validation and HA-08 remeasurement.
- [x] Complete dual review/verification and disposition.

## Outcomes

Complete typed traversal now validates every serialized numeric row/detail/
step/limiter/TVD field and routed zero-or-unit shape before file creation. Valid
output schema/bytes remain unchanged. Focused tests pass `12/12`, the shared
runner profile passes `120/120`, and the expanded HA-08 slice clears coverage,
floor, and CRAP gates. Two independent reviews/verifications pass with no
unresolved finding. Disposition: `TERMINAL-PASS`; `DC-CQR-HA08-001` is closed.
