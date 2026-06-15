# CQR05 Quality Plan Report

Evidence: Static + Ran.

Objective: perform a behavior-preserving private-helper extraction in
`hydrology_phase_erod14.rs` so every eligible target-module function has CRAP
`<= 30`.

Plan executed:

- Freeze public surface and focused EROD14 contract behavior before edits.
- Capture target coverage/LCOV and CRAP baseline.
- Extract cohesive private helpers for input loading, domain validation, case
  semantics, class-state loading, transport projection, reproportioning,
  enrichment, and writeback construction.
- Preserve formula text, arithmetic grouping, constants, thresholds, typed
  guard families, symbol names, and writeback order.
- Re-run focused tests, coverage/LCOV, CRAP, and required Rust closure gates.

Closure:

- CRAP target met. Final target-file maximum CRAP is `23.0`.
- Target `#[allow(clippy::too_many_lines)]` suppression removed.
- Workspace clippy passed with `-D warnings`.
- Warning retained for coverage below science-tier threshold.
