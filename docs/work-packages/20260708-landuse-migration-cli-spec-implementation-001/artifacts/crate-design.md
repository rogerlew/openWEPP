# Crate Design

Status: draft and amended for YAML-only output.

## Public Types

Initial API concepts:

- `Datver`
- `SourceManagement`
- `CanonicalManagementYaml`
- `MigrationPlan`
- `MigrationArgSpec`
- `MigrationArgs`
- `ClassMap`
- `DisturbedRouteCoefficientTable`
- `ValidationReport`
- `MigrationReport`
- `LanduseMigrationError`

## Trait Shape

```rust
pub trait LanduseMigrator {
    fn source_versions(&self) -> &[Datver];
    fn target_version(&self) -> Datver;
    fn required_args(&self, parsed: &SourceManagement) -> MigrationArgSpec;
    fn validate(
        &self,
        parsed: &SourceManagement,
        args: &MigrationArgs,
    ) -> Result<ValidationReport, LanduseMigrationError>;
    fn migrate(
        &self,
        parsed: SourceManagement,
        args: MigrationArgs,
    ) -> Result<MigrationOutputYaml, LanduseMigrationError>;
}
```

## Initial Migrators

- `LegacyCroplandToOwLanuse1Yaml`
- `FlatOwLanuse1ToYaml`
- `OwLanuse1YamlToLatest`

## Schema Dependency

The migration crate should depend on the shared schema owner ratified by
`20260708-openwepp-management-yaml-canonical-authorization-001`. Current
recommendation is a dedicated publishable crate:
`crates/openwepp-management-schema`.

The migration crate should not own canonical YAML structs itself; runtime intake
must use the same schema crate.

## Error Model

Use a typed error enum. Errors should identify:

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
- runtime consumer projection failure.
