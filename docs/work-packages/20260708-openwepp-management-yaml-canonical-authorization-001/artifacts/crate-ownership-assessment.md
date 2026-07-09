# Crate Ownership Assessment

Status: implemented disposition.

## Recommendation

Create a publishable shared schema crate:

```text
crates/openwepp-management-schema
```

Implemented. This crate owns canonical management YAML Rust types, serde
serialization/deserialization, extension policy helpers, typed validation
errors, schema-version dispatch, and optional generated schema artifacts.

`openwepp-input-contract` remains the legacy/parser-contract crate for flat WEPP
input surfaces and now depends on `openwepp-management-schema` for the YAML
adapter. It does not own the native YAML schema API.

`openwepp-landuse-migrate` should depend on `openwepp-management-schema` to emit
and validate YAML. The current runtime consumer path reaches the schema through
`openwepp-input-contract::parsers::management::parse_management_document_from_path`.
This keeps producer and consumer on one type surface without making the runner
own schema details directly.

## Rationale

- The schema must be used by both producer tools and runtime consumers.
- The schema should be publishable for crates.io workflows; the existing
  `openwepp-input-contract` crate is currently `publish = false`.
- Keeping YAML schema ownership separate avoids mixing frozen legacy flat-file
  parser compatibility with the forward native producer format.
- A dedicated crate gives the migration CLI, runner/orchestrator, tests, and
  downstream tools a stable API without pulling in runtime orchestration.

## Boundary

`openwepp-management-schema` should own:

- `CanonicalManagementYaml`;
- schema-version and datver enums;
- route-coefficient typed objects and validation;
- file-extension policy helpers:
  - producer emits lowercase `.yaml`;
  - default flat-source output appends `.yaml`, giving `.man.yaml`;
  - consumer accepts `.yaml`, `.YAML`, `.yml`, and `.YML`;
- serde YAML parsing/emission;
- typed validation errors with field paths;
- optional JSON Schema or equivalent generated schema artifacts.

It should not own:

- legacy flat `.man` parsing;
- Disturbed route-coefficient source acquisition;
- migration planning or class-map resolution;
- hillslope runtime scheduling;
- Lane D routing numerics.

## Alternatives

`openwepp-input-contract` ownership:

- Rejected as the default long-term path because it is legacy/parser-contract
  oriented and currently not publishable. It can host contract tests and bridge
  checks, but should not own the native schema API.

`openwepp-landuse-migrate` ownership:

- Rejected because the runtime consumer would then depend on a CLI/migration
  crate, coupling production intake to producer tooling.

`openwepp-sim-contract` ownership:

- Rejected because the management YAML format is an input schema surface, not a
  simulation output/state contract.

## Implementation Note

The YAML authorization package adopted the dedicated crate path. No deviation
was taken.
