# Pre-Implementation Gate

Evidence class: **Ran + Static**

## Reproduction

`cargo nextest run -p openwepp-hillslope-orchestrator --lib hb01_` ran seven
tests: six passed and
`hb01_b_d_h_threshold_domain_guards_fail_closed_with_exact_fields` failed.
The failure proves `te_s = 0` returned a nominal state instead of
`DirectRuntimeError::DirectDomainViolation` naming
`erosion.erod13.te_s`. Production had not been changed.

## Authority And Mechanism

`SC-SED-001#INV-SED-004` and the canonical Wave-1 algorithm require positive
effective runoff duration. `INV-SED-005/007` likewise require finite,
non-singular shear/normalization denominators. The target currently passes the
constant `WB11_ZERO_THRESHOLD` into a helper that accepts values down to
`minimum - WB11_ZERO_THRESHOLD`; using the same constant for both terms admits
zero.

The correction is local and fail-closed: an Erod13-specific strict-positive
guard will reject non-finite or `<= 0` values before the solver without changing
the generic tolerance helper or any formula. The seven DC conversion criteria
(reproduction, mechanism, ownership, authority, safety, testability, bounded
impact) all pass.

Disposition: `PASS` to production correction inside the declared envelope.

## In-Execution Envelope Amendment

The strict-positive correction passed all eight HB-01 tests but made the shared
profile expose `r7g_zero_upstream_lane_publishes_no_erosion_inflow_intake`:
R7D6 invoked pointwise Erod13 with authoritative `q_runoff_m = 0`, producing the
new correct strict-positive error instead of the contract-authorized inactive
event. `SC-SED-001` expressly allows no-runoff erosion inactivity and requires
positive drivers only when erosion computation is invoked.

Two independent reviewers accept a same-file envelope amendment. Both require
an exact equality gate, never `<=`, tolerance, or passby logic; malformed runoff
must still reach validation, and hourly/inter-OFE continuity must not be gated.
One reviewer proposed gating any zero pointwise hydrology scalar, while the
other limited approval to the reproduced `q_runoff_m == 0.0` mechanism and
flagged duration policy as separate authority. The binding disagreement default
selects the narrower runoff-only gate. No formula, generic helper, or continuity
activation changes.

Disposition: `PASS` to the amended bounded correction.
