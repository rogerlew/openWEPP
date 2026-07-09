# Crate Design

Status: implemented.

## Public Types

Implemented API concepts:

- `SourceManagement`
- `MigrationTarget`
- `MigrationAuthority`
- `ClassMap`
- `MigrationArgSpec`
- `ValidationReport`
- `MigrationReport`
- `MigrationRequest`
- `MigrationOutputYaml`
- `LanduseMigrationError`
- `LanduseMigrator`

## Trait Shape

```rust
pub trait LanduseMigrator {
    fn source_versions(&self) -> &[&'static str];
    fn target_version(&self) -> MigrationTarget;
    fn required_args(
        &self,
        source: &SourceManagement,
    ) -> Result<MigrationArgSpec, LanduseMigrationError>;
    fn validate(
        &self,
        source: &SourceManagement,
        args: &MigrationAuthority,
    ) -> Result<ValidationReport, LanduseMigrationError>;
    fn migrate(
        &self,
        source: SourceManagement,
        request: &MigrationRequest,
    ) -> Result<MigrationOutputYaml, LanduseMigrationError>;
}
```

## Implemented Migrators

- legacy cropland flat datvers to `ow-lanuse-1` YAML with required
  disturbed-class authority;
- flat `ow-lanuse-1` with explicit routing coefficients to canonical YAML;
- native YAML `ow-lanuse-1` to `latest` pass-through while `latest` resolves to
  `ow-lanuse-1`.

## Schema Dependency

The migration crate depends on the shared schema owner ratified by
`20260708-openwepp-management-yaml-canonical-authorization-001`:
`crates/openwepp-management-schema`.

The migration crate does not own canonical YAML structs itself; runtime intake
uses the same schema crate.

## Error Model

Implemented typed error enum: `LanduseMigrationError`. Production code does not
use broad boxed errors or silent defaulting for migration authority.

Errors identify:

- source file parse failure;
- YAML schema validation failure;
- unsupported datver;
- unsupported landuse;
- missing disturbed class authority;
- unknown disturbed class;
- partial class map;
- invalid route coefficient row;
- output path exists;
- invalid producer output extension;
- invalid structured args/class-map files.

## Crates.io Disposition

- `openwepp-landuse-migrate`: `publish = true`.
- `openwepp-management-schema`: already `publish = true`.
- `openwepp-input-contract`: changed to `publish = true` because the migration
  crate's distributable parser path depends on it.
- Runtime behavior has no WEPPpy checkout or network dependency.
