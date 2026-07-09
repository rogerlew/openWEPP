# Implementation

Status: `COMPLETE`

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Implemented as behavior-preserving CQR:

- Added module-local characterization tests for private WS12 impoundment
  projection, discharge, interpolation, regression, and guard helpers.
- Extracted `derive_riser_apr_coefficients_from_points` from
  `derive_riser_apr_coefficients` so the regression math can be covered without
  forcing every test through riser sampling.
- Extracted the active-projection culvert-like branch into
  `derive_culvert_like_active_projection` and
  `collect_culvert_like_stage_thresholds`.
- Extracted the active-projection riser branch into
  `derive_riser_active_projection` and
  `derive_riser_coefficient_from_reference`.
- Removed three stale `#[allow(clippy::too_many_lines)]` suppressions after the
  decomposition made them unnecessary.

Preservation notes:

- No public type, function, serialization, parser, or runtime API was changed.
- Floating-point expression grouping inside moved blocks was preserved.
- The active-projection discharge accumulation order was preserved:
  culvert 1, culvert 2, rockfill, emergency, filter.
- Existing typed fail-closed error classes, symbol names, and rule strings were
  retained; package-local tests now assert representative symbols/rules.
- No `SC-IMPOUND-001` contract authority, threshold, unit, or science formula
  was changed.
