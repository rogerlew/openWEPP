# Kernel-Profile Compliance Checklist

Status: complete / revision-60 review and terminal verification PASS

Evidence mode: **Static + Ran**

- [x] Canonical `SC-SED-001` authority is updated; revisions 56–57 preceded
      production correction and revisions 58–60 close profile documentation only.
- [x] Every required profile schema surface is present or explicitly mapped:
      purpose/scope, authority, variables/units, algorithm state, numbered
      algorithm, branch/guard table, invariant map, aliases, constants,
      unit governance, tolerances, calibration posture, test vectors, Binding
      Exposure Index, and gap register.
- [x] Algorithm inputs, outputs, private mutated zone state, branch priority,
      zero-/one-interval degenerate behavior, and step-local pre/postconditions are explicit in
      `SC-SED-001#EB-04W2C-Kernel-Process-Profile-Conformance`.
- [x] Guard/error mapping aligns exact closure with
      `erosion.wave1.publication_closure` and diagnostic refusal with
      `erosion.wave1.flux_closure`; only the latter is consumer-recoverable.
- [x] Constants/provenance table binds `32 * f64::EPSILON`, unchanged
      `1e-9`/`5e-3` factors, and Newton–Cotes weights as fixed numerical—not
      user-calibratable—values.
- [x] Unit-governance rows use the required symbol/units/registry/conversion/
      scalar-exception/publication columns and name `wave1_totals`,
      `wave1_flux_closure`, `wave1_integrate_rate_block`, and the EROD16
      denormalization reconstruction as conversion/ownership paths.
- [x] `CALIBRATION_NOT_APPLICABLE` is canonical, and
      `calibration-readiness-matrix.md` binds the exact three ADR-0042 fields
      and dispositions all ten readiness obligations individually with
      `PASS`/`NOT_APPLICABLE`, evidence path, and rationale.
- [x] The six-row Binding Exposure Index maps every active/historical addendum;
      active EROD13 diagnostic residue maps to `INV-SED-016`, and the canonical
      checker and its strict mode pass.
- [x] Contract-derived test authored and observed red before implementation.
- [x] Exact mass identity remains separate and hard.
- [x] Diagnostic tolerance, typed refusal, zero-contribution rule, and refusal
      counter remain unchanged.
- [x] No coefficient tuning, result-aware population narrowing, or snow rollback.
- [x] Numerical sub-march provenance prevents quadrature across coefficient,
      critical-shear, solution-family, region, or clamp boundaries.
- [x] Curved, odd/even/single, boundary, injected-error, and real hourly-
      consumer test vectors exercise the corrected path.
- [x] Independent per-cell load-ledger reconstruction checks accepted-solve
      conservation and rejects the detachment-only alias.
- [x] Focused, owning-crate, quick, frost, warnings-denied clippy, formatting,
      assurance, and Critical full gates executed.
- [x] Initial two independent reviews completed with findings.
- [x] Every initial finding dispositioned and corrected.
- [x] Fresh independent review accepts revision 60 and the terminal-governance
      correction.
- [x] Two independent terminal verifications accept the exact terminal tree.

Delegation is explicitly authorized in the package and active prompt. All
binding closure gates pass. No external constitutive-suite
metadata, cohort binding, or required-case posture changed, so the authority-
suite anti-evasion commands are not applicable to revisions 58–60.
