# Implementation Plan

Status: executed complete.

## Crate

Created `crates/openwepp-landuse-migrate` as a publishable workspace member
with:

- library crate `openwepp_landuse_migrate`;
- binary `openwepp-landuse-migrate`;
- dependency on `openwepp-input-contract` for frozen flat management parsing;
- dependency on `openwepp-management-schema`, the shared canonical management
  YAML schema surface ratified by
  `20260708-openwepp-management-yaml-canonical-authorization-001`;
- checked-in Disturbed route-coefficient table equivalent to WEPPpy
  `route_coefficients.py`.

## Sequence Disposition

1. Canonical management YAML authorization: complete before implementation
   closure.
2. Crate skeleton and workspace membership: complete.
3. Typed migration planning and `--args-for-migration-to`: complete.
4. `--validate` for native YAML and flat-source migratability: complete.
5. Normative class-map, `--args-file`, validation-report, and migration-report
   schemas: finalized in the CLI spec.
6. Legacy cropland to `ow-lanuse-1` YAML writer: complete.
7. Default output naming and lowercase `.yaml` producer enforcement: complete.
8. Flat `ow-lanuse-1` to canonical YAML bridge: complete.
9. YAML schema validation and runtime consumption proof: complete.
10. Native YAML `ow-lanuse-1` to `latest` pass-through migrator: complete.
11. CLI tests and integration tests: complete.
12. Closure gates: recorded in `gate-results.md`.

## Protected Behavior

- No runtime hillslope default change in this package.
- No sidecars.
- No legacy-field coefficient projection.
- No compatibility-only native output for pre-native datvers.
- No native flat `.man` writer.
- No producer output extensions other than lowercase `.yaml`.

## Closure Disposition

The prior schema-finalization blocker is closed. The CLI spec now defines the
class-map, `--args-file`, report schemas, and canonical output schema
relationship. Tests cover discovery, validation, migration, missing/unknown/
partial class authority, default `.man.yaml` naming, explicit uppercase YAML
rejection, flat-native migration, and real runtime consumption.
