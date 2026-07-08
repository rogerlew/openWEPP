# Implementation Plan

Status: scaffolded and amended for YAML-only output.

## Crate

Create `crates/openwepp-landuse-migrate` as a publishable workspace member with:

- library crate `openwepp_landuse_migrate`;
- binary `openwepp-landuse-migrate`;
- dependency on `openwepp-input-contract` for frozen flat management parsing;
- dependency on `openwepp-management-schema` or the shared canonical management
  YAML schema surface ratified by
  `20260708-openwepp-management-yaml-canonical-authorization-001`;
- checked-in Disturbed route-coefficient table equivalent to WEPPpy
  `route_coefficients.py`.

## Sequence

1. Wait for, or coordinate with, canonical management YAML authorization.
2. Add crate skeleton and workspace membership.
3. Implement typed migration planning and `--args-for-migration-to`.
4. Implement `--validate` for YAML inputs and flat-source migratability.
5. Finalize normative class-map, `--args-file`, validation-report, and
   migration-report schemas, then add class-map parser and Disturbed table
   validation.
6. Implement legacy cropland to `ow-lanuse-1` YAML writer.
7. Implement default output naming that appends `.yaml` to the input filename
   and enforce lowercase `.yaml` for producer output.
8. Implement flat `ow-lanuse-1` to canonical YAML bridge.
9. Validate migrated YAML through the canonical schema and prove runtime
   consumption.
10. Add native YAML `ow-lanuse-1` to `latest` pass-through migrator.
11. Add CLI tests and integration tests.
12. Run closure gates.

## Protected Behavior

- No runtime hillslope default change in this package.
- No sidecars.
- No legacy-field coefficient projection.
- No compatibility-only native output for pre-native datvers.
- No native flat `.man` writer.
- No producer output extensions other than lowercase `.yaml`.

## Closure Blocker

Rust implementation closure is blocked until the class-map, `--args-file`,
validation-report, migration-report, and canonical management YAML schemas are
final in the specs or directly linked normative appendices, with tests for
class-map conflicts, partial maps, global-class admissibility, and `--validate`
success/failure behavior. Producer extension behavior must be closed by tests:
default `.man.yaml` naming for flat `.man` inputs and fail-closed rejection of
explicit `.yml`, `.YML`, or `.YAML` output paths.
