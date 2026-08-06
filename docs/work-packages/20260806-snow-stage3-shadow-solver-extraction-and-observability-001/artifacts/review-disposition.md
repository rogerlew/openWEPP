# Review Disposition

Status: second remediation implemented; independent re-review pending.

All first-round findings were accepted. The remediation:

- moved the JSON object terminator after the optional evaluation suffix and
  added a complete-row schema-v4 golden plus full schema-v5 parse;
- passes an explicit evaluator context only from evaluator-owned calls, with
  `None` on authoritative calls and a filtered-capture invalid-geometry test;
- constructs and validates `Stage3EvaluationTag` before sequential clone
  allocation and retains tagged zero coverage for empty packs;
- initializes all 24 requested hours, reconstructs evaluated support, and
  converts hourly weighted fluxes with the full `3,600 s` basis;
- independently computes paired-arm non-formulation fingerprints over complete
  working cold content, layer state, hourly forcing, albedo, radiation,
  pressure, geometry, and shared tag/support IDs;
- restores the public `complete_carrier_shadow` field as a typed sequential
  compatibility spelling with an explicit conflict guard;
- makes surface/complete/terminal applicability explicit, removes sequential
  surface-arm values, and defines available ice as the maximum pre-debit value;
- adds runtime component/support/fingerprint guards; and
- replaces the closure proof with solver-produced paired and truncated
  sequential rows through the actual writer, exhaustive field reads,
  independent daily/hourly reconstruction, and all required anti-alias checks.

Focused remediation evidence: `53/53` six-binary Stage 3 tests, `150/150`
runner unit tests, and warnings-denied two-crate all-target Clippy pass.

The first re-review accepted the numerical, custody, schema, and consumer
remediation, but QA correctly retained `HOLD`: fields still extended two
existing exhaustive public diagnostics structs, a variant extended the public
kernel error enum, and no actual protected-output byte comparison existed.
That finding is accepted. The second remediation:

- restores the exact pre-package field/variant shapes of
  `DirectSnowSurfaceEnergyHourDiagnostics`, `DirectSnowStage3Diagnostics`, and
  `Wb11HydrologyKernelGuardError`;
- carries evaluator-only hourly data, the optional evaluator result, and typed
  turbulent failure on new additive types;
- keeps the authoritative partition outside the evaluation payload and proves
  it exactly equals disabled execution, including verbose diagnostics; and
- projects that authoritative partition through the real WAT, HBP, and PASS
  writers and compares exact enabled-versus-disabled bytes.

Second-remediation focused evidence: `53/53` six-binary Stage 3 tests, `2/2`
evaluator validation tests, `5/5` runner evaluation tests including exact
WAT/HBP/PASS bytes, and warnings-denied two-crate all-target Clippy pass.
