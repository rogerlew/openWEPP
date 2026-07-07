# Final Disposition

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

## Result

D16 hybrid active-path default promotion is held before implementation.

The current hybrid is operationally attractive but not yet default-promotable:

- Active plain H2637: `39.73 s` user / `0:39.75` wall.
- Active explicit hybrid H2637: `33.45 s` user / `0:33.47` wall.
- Case-4 hybrid ladder: PASS.
- Active hybrid closure residuals: machine-scale.
- Blocking evidence: H2637 active plain-vs-hybrid publication deltas lack a
  ratified production tolerance (`-0.4396 %` routed outlet; `-6.474 %`
  pass sediment sums).

No partial default flip landed. Current behavior remains:

- `OPENWEPP_LANED_ACTIVE=1` selects active Lane-D routing.
- `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` selects the hybrid stepper.
- Implicit unset remains plain active.

## Status

`EXECUTED-HOLD-FIDELITY-TOLERANCE`.

## Verification

- Review: Lorentz and Euclid returned `GO-WITH-AMENDMENTS`; all findings were
  accepted and fixed in artifacts.
- Verification: Hilbert and Anscombe returned `GO`.
- Local hygiene: `git diff --check`, scoped markdown lint, and
  `cargo fmt --check` passed.
