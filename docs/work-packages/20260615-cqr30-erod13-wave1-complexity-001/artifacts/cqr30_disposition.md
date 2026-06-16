# CQR30 Disposition

Disposition: accept.

Accepted changes:

- Behavior-preserving private decomposition of
  `Wb11HydrologyKernel::run_erod13_wave1_core`.
- Existing EROD13 contract vector retained as characterization coverage.
- Target CRAP reduced from `265.2636791582994` to `8.0`.
- Highest extracted helper CRAP is `29.0`.
- Previous target-level `clippy::too_many_lines` suppression removed.

Open findings: none.

Warnings:

- `cargo crap` reported the established `126` source-map warnings for LCOV
  entries on both before and after runs.
- `erod13_process_inputs` is close to the CQR threshold at CRAP `29.0`; future
  edits should avoid adding logic there.

Status: accepted pending package push.
