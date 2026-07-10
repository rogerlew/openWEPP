# Implementation

Evidence label: Static/Ran.

Status: `EXECUTED`

Target:

- `crates/openwepp-input-contract/src/parsers/management.rs`

## Summary

Static:

- Decomposed `parse_management_from_str` into private orchestration helpers:
  `parse_management_header`, `parse_management_sections`,
  `parse_management_meta`, and `parse_management_schedule`.
- Added small private carrier structs for the parsed header, section bundle,
  metadata, and schedule.
- Added `parse_positive_required` to preserve existing zero-count rejection
  while avoiding repeated guard blocks.
- Decomposed operation scenario parsing into private helpers for:
  landuse validation, MFO line parsing, operation-code parsing, pcode allowlist
  validation, `cltpos` parsing, effect-line parsing, and extension-line capture.
- Decomposed initial-condition parsing into:
  `parse_initial_scenario`, `parse_initial_cropland`, and
  `parse_initial_terminal_line`.
- Decomposed contour parsing into:
  `parse_contour_scenario`, `parse_contour_values`, and
  `parse_optional_contours_perm`.
- Review response: reused `validate_non_forest_cropland_landuse` from the
  surface and drain sections as well, preserving their original section names
  and option-field IDs (`iseq`, `dcont`) while eliminating contract-sensitive
  duplicate guard logic.

## Behavior-Preservation Notes

Static:

- Public parser functions, public data models, and error enum variants were not
  changed.
- The section parse order remains:
  plants, operations, initials, surfaces, contours, drains, yearlies, metadata,
  management schedule, cross-section reference validation, trailing-input
  check.
- The operation parser preserves the same field read order:
  scenario metadata, MFO line, pcode/cltpos line, effect line, optional
  extension lines.
- The initial parser preserves the native forest carve and the cropland field
  read order.
- The contour parser preserves the same datver-gated `contours_perm` behavior.
- Surface and drain landuse validation now share the same private helper as
  operation, initial, and contour parsing, but their typed error fields are
  unchanged.
- No floating-point expression was regrouped and no accumulation order was
  introduced or changed. The touched paths parse and assign scalar values.

## CQR Result

Ran:

- Targeted after-CRAP evidence:
  `/tmp/openwepp-cqr-nightly-07-management-targeted-crap.json`.
- Every deduplicated eligible production function in the target module is now
  at CRAP `<= 30`.
- Max target CRAP after refactor: `28.136080592592595`.

Disposition:

- Implementation is behavior-preserving CQR only.
- No parser contract, plant-file spec, `lanuse` authority, datver allowlist,
  fail-closed behavior, serialization, or public output meaning was amended.
