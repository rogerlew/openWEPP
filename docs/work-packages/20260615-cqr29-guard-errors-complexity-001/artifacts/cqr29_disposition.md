# CQR29 Disposition

Status: complete-with-warnings.

Accepted changes:

- Added public-surface characterization for all guard-error variants.
- Replaced the long `Display::fmt` branch matcher with private helper
  decomposition.
- Removed the target-file `clippy::too_many_lines` suppression.

Closure:

- Before target: `Wb11HydrologyKernelGuardError::fmt`, CRAP `272.0`.
- After target: `Wb11HydrologyKernelGuardError::fmt`, CRAP `1.0`.
- Max extracted-helper CRAP: `8.000751314800901`.

Warnings:

- `cargo crap` emitted 126 LCOV source-map warnings.

No open review findings remain.
