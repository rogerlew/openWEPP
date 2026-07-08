# openWEPP Landuse Migration CLI

## Header Metadata

- `spec_id`: `SPEC-TOOL-LANDUSE-MIGRATION-CLI-001`
- `surface_id`: `tool-landuse-migrate`
- `status`: `draft`
- `owner`: `openWEPP`
- `spec_version`: `0.2.0`
- `last_updated_utc`: `2026-07-08T23:45:00Z`
- `evidence_mode`: `Static`

## Purpose

Define the public command-line and library behavior for an offline Rust landuse
migration tool that converts frozen legacy WEPP management flat files and flat
`ow-lanuse-1` management files into canonical typed openWEPP management YAML.

The tool exists to support crates.io distribution and reproducible user
workflows without optional sidecars. Its output is a standalone `.yaml` file
that openWEPP can consume directly once the canonical YAML authorization package
lands.

When no explicit output path is supplied, the producer appends `.yaml` to the
input filename. A flat source such as `field.man` therefore defaults to
`field.man.yaml`. The `.man` segment is an informal provenance convention, not a
consumer requirement.

## Package And Binary

Planned Rust package:

- workspace member crate: `crates/openwepp-landuse-migrate`
- library crate name: `openwepp_landuse_migrate`
- binary name: `openwepp-landuse-migrate`

The library owns migration planning, argument discovery, validation, output
serialization, and report generation. The binary is a thin CLI wrapper around
that library.

## Source And Target Policy

The legacy management flat-file parser is an ingest-only migration reader. The
implementation carries no obligation to support new legacy flat-file dialects
beyond the current parser's accepted source set and the flat `ow-lanuse-1`
source surface.

Initial source set:

- legacy flat `.man` datvers accepted by the current Rust management parser;
- flat `ow-lanuse-1` as a source-only native flat-file bridge.

Initial target set:

- canonical `openwepp-management-yaml` schema version 1 with `datver:
  ow-lanuse-1`;
- `latest`, initially resolving to that same YAML schema/datver pair.

The migrator does not write native flat `.man` output. It does not require a
Rust management flat-file writer.

## Non-Negotiable Policy

Legacy datver migration to `ow-lanuse-1` must produce a coefficient-complete
native YAML file when the source contains legacy cropland scheduled for
migration. There is no compatibility-only migration mode for pre-native datvers.

Rules:

- No optional sidecar may be required for the migrated YAML file to run with the
  intended physics.
- Legacy `landuse=1` cropland does not itself authorize Lane D route
  coefficients.
- Pre-native legacy cropland migration to `ow-lanuse-1` requires explicit
  disturbed-class authority sufficient to write all five Lane D
  `routing_coefficients`.
- The tool must not infer route coefficients from `rrc`, `rrough`, row/rill
  geometry, cover, residue, aggregate friction, erosion delivery, or diagnostic
  fields.
- Unknown class, partial class map, unsupported source landuse, unsupported
  target requirement, or invalid coefficient row is a hard error.
- Existing native YAML `ow-lanuse-N` to newer native YAML datver migration does
  not require a disturbed class unless the target version introduces a new
  required authority surface.

## Command Surface

### Required Argument Discovery

```text
openwepp-landuse-migrate <input> --args-for-migration-to <target>
```

`<target>` may be a concrete native datver such as `ow-lanuse-1` or the alias
`latest`. The emitted format is canonical management YAML.

The command must inspect the actual input file and print the required arguments
for that input and target. It must not print only generic help.

Output formats:

```text
--format text
--format json
--format toml
```

Minimum discovery fields:

- source format and datver;
- target YAML schema and datver;
- detected landuse sections and scheduled crop slots;
- required migration arguments;
- accepted class-map key types for the detected file;
- blocking unsupported source landuses;
- whether a single global `--disturbed-class` is admissible.

### Validation

```text
openwepp-landuse-migrate <input> --validate [--to <target>] \
  [--disturbed-class <class>] \
  [--disturbed-class-map <path>] \
  [--args-file <path>] \
  [--format text|json|toml]
```

Validation modes:

- For native management YAML with no `--to`, validate schema, datver, required
  route-coefficient authority, and runtime eligibility.
- For flat legacy `.man` or flat `ow-lanuse-1` with `--to`, validate whether the
  input can migrate to the target with the supplied migration authority.
- For dry migration planning, perform the same parse, class resolution,
  coefficient validation, and report construction as migration without writing
  output.

`--validate` writes no migrated management file. It returns nonzero for
malformed input, invalid YAML, unsupported source/target, missing migration
authority, missing target-required coefficients, or any source surface that
cannot become standalone native YAML.

### Migration

```text
openwepp-landuse-migrate <input.man> \
  --to <target> \
  [--output <output.yaml>] \
  [--disturbed-class <class>] \
  [--disturbed-class-map <path>] \
  [--args-file <path>] \
  [--dry-run] \
  [--report <path>] \
  [--report-format text|json|toml]
```

Semantics:

- `--to latest` resolves to the latest native YAML datver supported by the
  crate.
- If `--output` is omitted, the output path is the input path plus `.yaml`;
  `field.man` becomes `field.man.yaml`.
- Explicit and derived output paths must end in lowercase `.yaml`. The producer
  must not emit `.yml`, `.YML`, or `.YAML`.
- The `.man` portion of default `.man.yaml` names is only an informal signal
  that the YAML was migrated from a flat management source.
- `--disturbed-class <class>` applies one class to every migrated legacy
  cropland route-coefficient site only when discovery says a global class is
  admissible.
- `--disturbed-class-map <path>` supplies per-scenario or per-schedule class
  authority.
- `--args-file <path>` supplies the same migration arguments in a structured
  file for reproducible migration pipelines. It is a migration-time input, not
  a runtime sidecar, and is not needed after the output YAML is written.
- `--dry-run` performs parse, planning, class resolution, coefficient
  validation, and report generation but does not write output YAML.
- `--report` writes an audit report. The output YAML remains standalone even if
  the report is discarded.

## Class Map Contract

The class map must be explicit enough to assign a disturbed class to every
legacy cropland site that will become native coefficient-complete. Accepted key
forms are target-specific and must be reported by `--args-for-migration-to`.

Initial `ow-lanuse-1` key forms:

- `plant_scenario_name`;
- `plant_index`;
- `schedule_slot`;
- `ofe_index` only when it resolves unambiguously to scheduled crop slots.

The CLI must fail if two class-map entries assign different classes to the same
scheduled crop slot unless the user supplies an explicit conflict-resolution
argument added by a later spec revision. The initial revision has no conflict
resolution flag.

Example TOML:

```toml
[plant_scenario_name."Corn Year 1"]
disturbed_class = "agriculture crops"

[plant_scenario_name."Small Grain Rotation"]
disturbed_class = "agriculture small grain"
```

## Disturbed Route Coefficient Table

For crates.io distribution, the Rust crate must include a versioned copy of the
Disturbed route-coefficient table equivalent to:

```text
/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py
```

Authority:

- ADR-0014 Disturbed openWEPP route coefficients;
- work package
  `20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001`;
- management-lanuse authority contract rev 2;
- `SC-OFEROUTE-001` rev 49 native datver authority.

The table must be embedded in the crate source or generated into checked-in Rust
source. It must not be fetched at runtime.

The migration report and emitted YAML must include enough provenance to identify
the table row used for each migrated coefficient site:

- table id;
- table version;
- table checksum;
- source authority string;
- normalized disturbed class;
- emitted five coefficient values.

## Output Requirements

For pre-native legacy cropland to `ow-lanuse-1` YAML:

- output format is `openwepp-management-yaml`;
- output schema version is `1`;
- output `datver` is `ow-lanuse-1`;
- migrated cropland records use native cropland semantics equivalent to
  `landuse=4`;
- every scheduled native cropland plant record that participates in the
  migrated run has explicit typed `routing_coefficients`;
- all five route-coefficient values are finite and in-domain;
- the output does not require a sidecar, report, or original `.man` file to
  recover coefficient authority;
- unsupported legacy rangeland, forest, road, or ambiguous source modes fail
  closed in the initial implementation.

For flat `ow-lanuse-1` source to canonical YAML:

- preserve existing explicit route coefficients and native landuse semantics;
- fail closed on missing target-required authority;
- emit YAML that validates against the canonical typed schema.

For native YAML `ow-lanuse-N` to latest:

- preserve existing explicit route coefficients unless a target-version rule
  requires a deterministic schema rename or unit-preserving rewrite;
- fail closed on missing target-required authority;
- emit a report of every schema step applied.

## Error Posture

The CLI must return nonzero and write no output file for:

- malformed source management file;
- malformed or schema-invalid YAML;
- explicit output path with any terminal extension other than lowercase
  `.yaml`;
- unsupported source datver;
- unsupported target datver;
- legacy migration requiring disturbed class authority when none was supplied;
- partial class map;
- unknown disturbed class;
- invalid route coefficient row;
- unsupported source landuse for the requested target;
- output path overwrite unless `--force` is introduced by a later revision;
- native target output that would lack target-required route coefficients.

Error messages must name the missing argument or unsupported source surface and
must suggest `--args-for-migration-to <target>` when applicable.

## Library API Shape

The implementation package should define a versioned migration API similar to:

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

Required library concepts:

- `Datver`;
- `SourceManagement`;
- `CanonicalManagementYaml`;
- `MigrationPlan`;
- `MigrationArgSpec`;
- `MigrationArgs`;
- `ClassMap`;
- `DisturbedRouteCoefficientTable`;
- `ValidationReport`;
- `MigrationReport`;
- typed `LanduseMigrationError`.

Production code must use typed errors, not broad boxed errors.

## Schema Finalization Gate

Before implementation closure, this spec or a directly linked normative schema
appendix must replace the schema open questions with final formats for:

- `--disturbed-class-map`;
- `--args-file`;
- migration reports;
- canonical management YAML output.

The implementation must include tests for class-map conflict detection, partial
map failure, global disturbed-class admissibility, and `--validate` success and
failure modes. Rust code must not land as complete while these schemas remain
only descriptive or unresolved.

## Test Obligations

Minimum tests for the implementation package:

- `--args-for-migration-to ow-lanuse-1` on legacy cropland reports required
  disturbed-class authority and YAML target format.
- `--validate --to ow-lanuse-1` on legacy cropland without disturbed-class
  authority fails closed.
- legacy cropland with a global admissible disturbed class writes canonical
  YAML with `datver: ow-lanuse-1`, native cropland semantics, and explicit route
  coefficients.
- omitted `--output` for `field.man` writes `field.man.yaml`.
- explicit producer output using `.yml`, `.YML`, or `.YAML` fails closed.
- partial class map fails closed and names missing sites.
- unknown disturbed class fails closed.
- legacy rangeland/forest/roads fail closed in initial implementation.
- flat `ow-lanuse-1` source with coefficients migrates to equivalent native
  YAML.
- migrated output validates through the canonical management YAML schema.
- migrated output is consumed by the real openWEPP runtime path that projects
  all five route symbols into PL schedule surfaces.
- native YAML `ow-lanuse-1` with coefficients to `latest` preserves
  coefficients.
- dry-run writes no output YAML and still produces a report.
- overwrite without a future `--force` flag fails.

## Crates.io Distribution Requirements

- The package intended for publication must set package metadata appropriate for
  crates.io.
- Runtime behavior must not depend on `/home/workdir/wepppy`, network access, or
  generated files outside the crate.
- Include license and repository metadata through the workspace package
  metadata.
- Keep the binary usable independently of the openWEPP workspace checkout.

## Open Questions

- Exact class-map schema for schedule slots should be finalized during
  implementation after inspecting the parsed management schedule model.
- Whether the first implementation should support native forest migration from
  flat `ow-lanuse-1` source is unresolved; default posture is fail closed unless
  the YAML authorization package ratifies it.
- The latest native datver alias initially resolves to `ow-lanuse-1` YAML;
  future native datvers must add explicit migrator steps.
