# CQR09 Numeric Equivalence

Static: no decomposition formula, unit conversion, threshold, or floating-point
expression was changed. The refactor moves already-computed values and existing
zero-state validations into private helpers.

Static: the annual control output still uses the same conversion helpers:

- `usize_to_u8_for_decomposition` for `resmgt`.
- `usize_to_u16_for_decomposition` for `jdherb`, `jdburn`, `jdslge`, `jdcut`,
  and `jdmove`.

Static: the scalar payload fields `fbrnag`, `fbrnog`, `frcut`, and `frmove`
are copied from the same required state values into the same control fields.

Static: the active-action branch selector still matches `resmgt` values
`1..=6` to Herbicide, Burn, Silage, Cut, Remove, and None respectively, with
the same runtime-day equality checks for action activation.

Ran: characterization and workspace tests passed after the refactor; see
`cqr09-implementation-and-test-evidence.md` and `gate-results.md`.

Status: numeric and branch behavior preserved by static diff review and test
evidence.
