# Worker Handoff

Status: `ready`

EB-04A is an observability-only increment. It preserves the canonical rejected
step while adding typed replay state and independently reconstructable Stage 3
diagnostics. It does not alter a process equation, coefficient, threshold,
forcing, selector, fixture, or user-facing schema.

The targeted replay attempted and classified all 24 frozen EB-04 failures:

- 17 rejected a temperature below absolute zero;
- 5 rejected saturation-vapor-pressure evaluation below its supported
  temperature domain;
- 2 rejected layer-depth reconciliation by approximately one nanometre.

All replay snapshots are complete and remain fail-closed. Independent mass,
surface-energy, latent-energy, vapor-mass, shortwave, and longwave audits pass
their declared tolerances. The package-local diagnostic JSON is bound to the
current executable and executable-source diff by SHA-256.

The quick, frost, and full workspace profiles, focused contract tests, clippy,
formatting, documentation, SVG, security, and exact-diff checks pass. Dual
technical review is complete. The next scientific increment is EB-04B:
controlled regime characterization using the newly retained state; EB-04A
does not authorize a corrective physics change.
