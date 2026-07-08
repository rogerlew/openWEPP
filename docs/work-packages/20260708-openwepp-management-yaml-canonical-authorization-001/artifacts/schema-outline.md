# Schema Outline

Status: draft.

Initial canonical YAML identity:

```yaml
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
```

Required schema families:

- document identity and provenance;
- typed plant, operation, initial-condition, surface-effect, contour, drainage,
  yearly, and management schedule records;
- explicit native landuse variants;
- explicit route-coefficient object for Lane D/native production;
- validation metadata and migration provenance where present.

Extension policy:

- producers emit lowercase `.yaml` only;
- migrated flat management defaults append `.yaml` to the source filename, so
  `field.man` becomes `field.man.yaml`;
- the `.man` portion of `.man.yaml` is informal provenance, not a consumer
  requirement;
- consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML`, then validate document
  identity and schema content.

The implementation must use strict typed Rust structs. Unknown schema versions,
missing required fields, and missing target-required route coefficients fail
closed.
