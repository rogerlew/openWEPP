# Implementation Evidence

Status: executed.
Evidence mode: Static.

## Changed Files

- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/`

## Spec Update

Static:

- Bumped the plant-file spec metadata to `spec_version` `0.3.0` and
  `last_updated_utc` `2026-07-08T17:13:26Z`.
- Added `ow-lanuse-1` to the executable parser datver allowlist.
- Added an end-user-facing `openWEPP-Native ow-lanuse-1 Profile` section that
  explains:
  - first-line `datver` controls landuse-code interpretation;
  - under native `ow-lanuse-1`, `landuse=3` is native forest and `landuse=4` is
    native cropland;
  - native cropland reuses cropland grammar across the plant, operation,
    initial, surface, contour, drain, and yearly sections;
  - native forest has plant, initial, and yearly records only;
  - `routing_coefficients` / `routing_coefficients_v1` is a two-line plant
    extension with exactly five explicit Lane D coefficients.
- Added native forest plant, initial, and yearly payload layouts.
- Added native carve-out notes to the affected legacy landuse branch headings so
  the manual-derived roads/forest text no longer contradicts native
  `ow-lanuse-1` behavior.
- Added `ow-lanuse-1` to the Information Section `datver` list.

## Scope Control

Static:

- No Rust source, tests, fixtures, `SC-*` contracts, or WEPPpy files were edited
  by this package.
- No coefficient values or defaults were introduced.
- No routing activation, mesh policy, or runtime consumer behavior was changed.
