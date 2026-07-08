# Crate Ownership Assessment

Status: draft recommendation.

## Recommendation

Create a publishable shared schema crate:

```text
crates/openwepp-management-schema
```

This crate should own canonical management YAML Rust types, serde
serialization/deserialization, extension policy helpers, typed validation
errors, schema-version dispatch, and optional generated schema artifacts.

`openwepp-input-contract` should remain the legacy/parser-contract crate for
flat WEPP input surfaces. It may depend on `openwepp-management-schema` only if
contract tests need shared types, but it should not become the owner of the new
native YAML schema.

`openwepp-landuse-migrate` should depend on `openwepp-management-schema` to emit
and validate YAML. The hillslope orchestrator/runtime intake should depend on
the same crate to consume YAML. This keeps producer and consumer on one type
surface.

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

The YAML authorization package should disposition this recommendation before
code lands. Deviating from the dedicated crate path requires a recorded review
finding explaining why the chosen owner remains publishable, shared by producer
and consumer, and does not entangle legacy flat-parser compatibility with native
YAML schema ownership.
