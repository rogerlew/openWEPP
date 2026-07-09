# Implementation

Evidence label: Static/Ran.

Status: `COMPLETE`

Implemented behavior-preserving CQR in
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`.

Production changes:

- Extracted `impoundment_outflow_at_stage` into cohesive private helpers for
  drop-spillway, culvert-family, rockfill, emergency-spillway, filter-fence,
  perforated-riser, and total-outflow validation logic.
- Added private `Ws12CulvertFamilyParams` to parameterize the two existing
  culvert families without changing symbol names, index order, guard classes,
  or expression grouping.
- Preserved the original call order and accumulation order:
  drop spillway -> culvert #1 -> culvert #2 -> rockfill -> emergency spillway
  -> filter fence -> perforated riser -> total validation.
- Removed the stale `#[allow(clippy::too_many_lines)]` from
  `impoundment_outflow_at_stage` after decomposition made it unnecessary.

Test changes:

- Added root-level `#[test]` functions in the include file rather than a
  trailing `#[cfg(test)] mod` block, preserving the include-order constraint
  documented in the package.
- Characterized outlet-family numeric assembly, adaptive retry behavior, route
  wrapper behavior, and fail-closed guard outcomes.

No public API, serialization, runtime-symbol, threshold, tolerance, or science
contract changes were made.
