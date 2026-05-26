# MOFE09 Runtime Fallback Implementation Report

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Implemented runtime soil theta source precedence in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`:
  - `theta_r_rosetta.or(wp_measured)` for `thetdr`
  - `fc_rosetta.or(fc_measured)` for `thetfc`
- Applied precedence for both primary-layer validation and per-layer projection.
- Preserved strict typed failure posture when neither source exists.
- Updated typed error display text to reflect dual-source requirement.

Ran:
- New and existing seam tests pass with fallback behavior.
