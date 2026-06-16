# CQR29 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`.

Static: protected boundaries are public enum variants, stable error IDs,
display text, boundary class mapping, typed error behavior, and science-contract
behavior.

Ran: live baseline identified `Wb11HydrologyKernelGuardError::fmt` at line 167
as the CQR29 target with CC `16`, coverage `0%`, and CRAP `272.0`.

Ran: added focused characterization coverage before production refactor for all
15 public guard-error variants, asserting exact `code()`, `boundary_class()`,
and `to_string()` outputs.

Ran: production refactor is private-only: `Display::fmt` now delegates to
private display-part helpers. The public enum variants, `code()` output,
`boundary_class()` mapping, and display strings are preserved.

Ran: final metrics close the target and helpers:

- `Wb11HydrologyKernelGuardError::fmt`: CRAP `1.0`.
- `Wb11HydrologyKernelGuardError::display_parts`: CRAP `5.0`.
- `Wb11HydrologyKernelGuardError::phase_display_parts`: CRAP
  `8.000751314800901`.
- `Wb11HydrologyKernelGuardError::erod13_display_parts`: CRAP
  `5.003644846187491`.
- `Wb11HydrologyKernelGuardError::erod14_display_parts`: CRAP
  `5.003644846187491`.
- `Wb11HydrologyKernelGuardError::erod18_display_parts`: CRAP
  `5.00205473822635`.
- `HydrologyGuardErrorDisplayParts::fmt_with_code`: CRAP `7.0`.

Warn: `cargo crap` emitted 126 LCOV source-map warnings; this matches the
known warning class seen in prior CQR rows and did not block target closure.
