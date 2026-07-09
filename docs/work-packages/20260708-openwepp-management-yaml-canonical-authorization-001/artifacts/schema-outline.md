# Schema Outline

Status: implemented v1.

Canonical YAML identity:

```yaml
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
```

Implemented crate:

- `crates/openwepp-management-schema`
- `publish = true`
- owns serde types, YAML parse/emission, extension policy helpers, and typed
  validation errors.

Implemented top-level schema families:

- `topology`: `nofes`, `total_years`;
- `metadata`: management name, three-line description, optional provenance;
- `plants`: tagged native landuse plant records;
- `operations`;
- `initial_conditions`;
- `surface_effects`;
- `contours`;
- `drains`;
- `yearly_scenarios`;
- `schedule`: one-based author-facing rotation/year/OFE references plus yearly
  references.

Implemented native plant route-coefficient object:

```yaml
routing_coefficients:
  k_o: 500.0
  form_c_d: 1.25
  d_r_m: 0.06
  lambda: 0.2
  vegetation_c_d: 0.7
  authority:
    source: disturbed-route-coefficients
    version: 2026-07-08
    checksum: fixture
    disturbed_class: high_severity_fire
```

Validation:

- rejects unknown `format`;
- rejects unsupported `schema_version`;
- rejects unsupported `datver`;
- rejects empty required section families;
- rejects invalid topology/schedule counts;
- rejects dangling one-based references;
- rejects missing route-coefficient objects;
- rejects negative/non-finite coefficient values;
- rejects empty route-coefficient authority fields.

Extension policy:

- producers accept lowercase `.yaml` only;
- default migrated path appends `.yaml` to the full source filename, giving
  `.man.yaml` for flat `.man` sources;
- consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML` before schema
  validation.

Execution evidence:

- `cargo test -p openwepp-management-schema`
- `cargo clippy -p openwepp-management-schema --all-targets -- -D warnings`
