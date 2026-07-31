# Final Disposition

Status: `executed / complete / pass`

Evidence mode: `Static + Ran`

## Decision

EB-03A implements the authoritative Marks/SNOBAL active/lower thermal provider
and closes the identified snow-physics defect. Technical/science disposition:
`PASS`.

Overall package disposition: `PASS`.

## Passed Current-Scope Evidence

- canonical version-3/119 contract authority and strict governance checks;
- exact libsnobal `KTS+efcon` and harmonic active/lower conduction;
- physical, conservative, persistent active/lower state projection;
- 24/24 terminal focused tests and 22/22 meteorology tests;
- frost profile 324/324;
- real direct-production B absent/empty/B/L/S/LS, all exit zero;
- independent same-substep `G_0`, resistance, and carrier reconstruction;
- active, lower, whole-pack, vapor/latent, and liquid closure;
- default rollback identity;
- formatting, Clippy, documentation, assurance validation, and 92-file
  rendered-review checks; and
- dual independent technical reviews and terminal verification.

## Historical Hold And Resolution

The package prospectively required the full quick profile, and ADR-0043 also
requires the Critical full profile. Quick fails in a CQR exact-head
self-test that reproduced alone and under independent verification. Full
reached two assurance publication-matrix timeouts at `720 s`. The amended
assurance inputs may be consumed by those tests; dependency independence was
not established. Neither blocker authorizes this snow package to edit CQR or
assurance publication machinery.

SNOW-SURFACE-EB-03B corrected both out-of-envelope validation defects without
changing snow physics, assurance authority, timeout configuration, or quality
thresholds. Quick 2109/2109, frost 324/324, and Critical full 2158/2158 pass.

The historical hold is lifted. EB-04 is admitted for scaffolding from the
frozen factorial design. No further snow calibration, coefficient, clamp, or
physics change is indicated before that result-bearing package.
