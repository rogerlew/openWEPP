# Verification Agent A

Status: `PASS`

Evidence: `Static + Ran (read-only audit)`

All package claims independently verify:

- EB-04, EB-04A, binary, executable-source-diff, and frozen-manifest identities
  match the current retained inputs.
- The manifest contains 24 unique targets with the exact `17/5/2` taxonomy;
  every trace ends on the day before rejection, and all 22 B companions reach
  the corresponding pre-failure day.
- The deterministic GZIP contains 83,232 data rows, 51 fields, 24 cases,
  83,208 successful rows, and one typed terminal row per case.
- Independent temperature reconstruction closes `22/22` with zero maximum
  residual. Seventeen states cross `0 K`; five remain valid Kelvin inputs that
  underflow in saturation-vapor-pressure evaluation.
- All 20 S/LS thermal cases have prior-day positive sublimation and negative
  latent energy, while two L-only contrary cases prove sublimation is not
  necessary. The rejected-slice role and crossing-substep driver remain
  correctly inconclusive.
- Both geometry residuals reconstruct from one filtered SWE fragment whose
  density-derived physical depth exceeds the independent depth tolerance.
- Five SVGs parse, have complete one-to-one sidecars, and retain the corrected
  rejected-slice semantics.
- Direct documentation lint, exact identity, review disposition, protected-
  boundary, and diff-hygiene checks pass. Rust-gate reuse is valid because the
  executable-source diff is unchanged from validated EB-04A.

One stale catalog reference was found during verification, accepted, corrected,
and rechecked. No blocker remains.
