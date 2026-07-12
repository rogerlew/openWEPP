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
