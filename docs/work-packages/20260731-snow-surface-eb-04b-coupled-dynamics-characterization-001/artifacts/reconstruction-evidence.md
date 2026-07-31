# Independent Reconstruction Evidence

Evidence: `Ran`

The package tool independently evaluated all 22 rejected thermal snapshots with
`rho_w = 1000 kg m^-3` and `c_i = 2100 J kg^-1 K^-1`. Maximum absolute residual
against the typed control-volume temperature was exactly `0.0 degC` at the
serialized precision.

Anti-alias self-checks reject:

- the wrong temperature sign;
- physical layer depth substituted for SWE mass;
- acceptance with any required quantitative predicate forced false.

The two geometry snapshots were independently parsed. Reapplying the production
`mass_swe_m > 1e-9 m` filter exactly reproduces each reported aggregate, while
including the excluded fragment reproduces the expected prior depth. Excluded
fragment depth matches the signed residual within `2e-15 m`.

The EB-04A mass and energy ledgers remain unchanged and hash-bound. EB-04B adds
no conservation-sensitive producer or publication field.
