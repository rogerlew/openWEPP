# Disposition

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Static + Ran.

Decision: retain active production fixed `10 cells/OFE`; hold/reject
target-`dx` production promotion under this package's declared exit criteria.

Completed:
- Contract authority updated to `SC-OFEROUTE-001` rev 39.
- Diagnostic target-`dx` selector implemented and bounded.
- Diagnostic active trace output implemented and manifest-checksummed.
- Package-local ladder harness isolated outputs from source fixtures.
- Full selected-cohort + H2637 ladder executed.
- Case-4 dimensionless tests executed.
- Focused selector/trace tests, fmt, and clippy executed.

Finding disposition:
- Pre-execution review H findings were resolved before execution by defining
  candidate-vs-fine-reference error basis, reference adequacy, and Case-4's
  dimensionless role.
- Review Agent A M1 accepted: trace-only selector misuse now fails at startup
  before output setup, with a regression in `laned_shadow_h2637`.
- Review Agent A M2 accepted: contract/package wording now records the
  executed floor-plus-fail-closed-cap mesh rule, not an upper clamp.
- Review Agent A residual test concern dispositioned: full trace emission and
  manifest checksum inclusion are evidenced by the package ladder outputs;
  the always-run regression covers the default/off safety risk.
- Verification Agent B blocker accepted: package status changed from the
  earlier complete/rejected wording to
  `EXECUTED-HOLD-DX-REFERENCE-ADEQUACY`, gate statuses normalized, and the
  hold audit completed.
- Execution evidence blocks promotion: WA fine-reference rungs fail active
  closure at day 1122; H2637 stress fails shape/sediment adequacy; target
  `dx10/dx5` behavior on WA is non-promotable.

Remaining risk:
- The WA day-1122 high-resolution closure/magnitude pathology is a numerics
  investigation item and is the named hold-lift path. It does not justify a
  production mesh-policy change inside this package.

Next action:
- Optional follow-on: scaffold a narrow WA day-1122 high-resolution active
  router numerics investigation package.
