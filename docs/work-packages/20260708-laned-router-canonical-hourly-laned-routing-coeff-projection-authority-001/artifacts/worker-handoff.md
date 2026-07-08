# Worker Handoff

Status: final handoff.

## Next Package

M-T2B may proceed after this package closes, but it must treat
coefficient-absent legacy cropland as legacy/off unless a later bridge contract
ratifies all five Lane D route coefficients.

## Binding Rules

- Keep explicit native `routing_coefficients` and authorized explicit producer
  fields as the only accepted static Lane D coefficient authority.
- Preserve no-coefficient legacy/off fallback.
- Preserve mixed-authority fail-closed behavior.
- Do not implement hidden projection from `rrc`, `rrough`, row/rill geometry,
  cover/residue/canopy-cover fields, aggregate friction factors, erosion
  delivery ratios, or diagnostics.
- Current Rust comments/errors still name native `routing_coefficients`
  because no implementation broadening occurred in this package. If a future
  package makes another explicit producer user-visible in runtime diagnostics,
  update the runtime wording and tests to say source-authorized route
  coefficients without weakening the fail-closed behavior.
- Distinguish generated zero, disabled process, missing authority, and
  explicit-disable/rollback in any groundwater/baseflow publication work.

## Reopening Route

If production needs coefficient-complete activation for legacy cropland without
explicit route coefficients, scaffold a separate bridge-authority package. That
package must supply all five static operands, provenance, bounds, tests, manifest
labels, and predeclared multi-case fidelity gates before implementation.
