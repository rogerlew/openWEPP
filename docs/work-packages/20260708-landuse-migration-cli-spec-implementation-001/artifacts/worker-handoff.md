# Worker Handoff

Status: scaffolded and amended for YAML-only output.

First implementation actions:

1. Confirm the canonical management YAML authorization package has closed, or
   coordinate schema work with it.
2. Add `crates/openwepp-landuse-migrate` to the workspace.
3. Implement `--args-for-migration-to ow-lanuse-1` for legacy cropland inputs
   with YAML target reporting.
4. Implement `--validate` for native YAML and flat-source migratability.
5. Embed/version the Disturbed route-coefficient table.
6. Implement fail-closed legacy cropland to coefficient-complete `ow-lanuse-1`
   YAML migration.
7. Implement default `.man.yaml` naming for flat `.man` inputs and reject
   explicit producer outputs that do not end in lowercase `.yaml`.
8. Add schema and runtime consumer tests proving migrated YAML is native and
   coefficient-complete.

Do not add sidecars, compatibility-only native output for pre-native datvers, or
a native flat `.man` writer. Do not emit `.yml`, `.YML`, or `.YAML` from
openWEPP producer tools.
