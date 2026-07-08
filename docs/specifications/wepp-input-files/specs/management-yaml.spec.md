# openWEPP Management YAML

## Header Metadata

- `spec_id`: `SPEC-INFILE-MANAGEMENT-YAML-001`
- `surface_id`: `infile-management-yaml`
- `status`: `draft`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-07-08T23:45:00Z`
- `evidence_mode`: `Static`

## Purpose

Define the planned canonical typed YAML management format for openWEPP native
management inputs. This format is the producer-side successor to legacy WEPP
flat `.man` management files for `ow-lanuse-1` and later native datvers.

The YAML format is a primary input surface, not a sidecar. A valid native YAML
management file must contain all management operands required for its declared
datver and physics eligibility.

## Format Identity

Initial file surface:

```yaml
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
```

Rules:

- `format` is required and must be `openwepp-management-yaml`.
- `schema_version` is required and increments for YAML schema changes.
- `datver` is required and carries the native management semantic version.
- `ow-lanuse-1` is the initial native datver; future `ow-lanuse-N` datvers must
  have explicit schema migration rules.

## File Extension Policy

Producer requirements:

- openWEPP producers must emit lowercase `.yaml` as the terminal extension.
- For migrated flat management inputs, the default output path appends `.yaml`
  to the full source filename. A source such as `hillslope.man` therefore
  defaults to `hillslope.man.yaml`.
- The `.man` portion of `.man.yaml` is an informal provenance-oriented naming
  convention for migrated flat management sources, not a schema or consumer
  requirement.
- Producers must not emit `.yml`, `.YML`, or `.YAML`.

Consumer requirements:

- Consumers must accept terminal extensions `.yaml`, `.YAML`, `.yml`, and
  `.YML`.
- Extension acceptance is only a dispatch convenience. The document must still
  validate `format`, `schema_version`, `datver`, and all target-required typed
  operands.
- Consumers must reject extensionless management YAML paths unless a later
  contract revision authorizes explicit content-sniffing behavior.

## Relationship To Flat Management Files

Legacy flat `.man` files remain source-only migration inputs and protected
compatibility/validation/rollback surfaces. openWEPP carries no obligation to
author new flat `.man` outputs for native landuse evolution.

Flat `ow-lanuse-1` may remain a source-only bridge for migration from the
current parser surface to canonical YAML. New producer work should target YAML
directly once this specification is authorized and implemented.

## Canonical Policy

For new Lane D/native landuse physics:

- canonical producer output is typed management YAML with `datver:
  ow-lanuse-1` or later;
- optional sidecars are not production route-coefficient authority;
- legacy cropland fields do not authorize route coefficients;
- route coefficients must be explicit typed operands in the YAML document;
- missing, partial, mixed, or invalid coefficient authority fails closed before
  runtime projection.

## Typed Schema Requirements

The implementation package must define Rust types and a machine-readable schema
for the YAML surface. The schema must be strict by default.

Minimum top-level concepts:

- document identity: `format`, `schema_version`, `datver`;
- management metadata: name, description, provenance;
- typed plant scenarios;
- typed operation scenarios;
- typed initial-condition scenarios;
- typed surface-effect scenarios;
- typed contour and drainage scenarios where present;
- typed yearly scenarios;
- typed management schedule with OFE, rotation, year, and crop-slot references;
- validation metadata sufficient to report source/migration provenance.

## Route Coefficient Schema

Native cropland records that participate in Lane D/new-physics production must
carry all five routing coefficients explicitly.

Required route coefficient object:

```yaml
routing_coefficients:
  authority: disturbed-route-coefficients
  authority_version: "<version>"
  authority_checksum: "<checksum>"
  disturbed_class: "agriculture crops"
  k_o: 0.0
  form_c_d: 0.0
  d_r_m: 0.0
  lambda: 0.0
  vegetation_c_d: 0.0
```

The exact numeric values above are placeholders in this illustrative fragment.
The normative implementation must embed the authorized Disturbed coefficient
table and write the real class row values.

## Producer Obligations

A YAML producer must:

- emit lowercase `.yaml` as the terminal output extension;
- emit a schema-valid document;
- emit native landuse variants explicitly rather than encoding native behavior
  through legacy sentinel ambiguity;
- include all target-required operands for the declared datver;
- preserve or embed route-coefficient provenance when coefficients are required;
- fail rather than emit a YAML file that would need a sidecar or external state
  to recover target-required authority.

## Consumer Obligations

The openWEPP runtime consumer must:

- accept `.yaml`, `.YAML`, `.yml`, and `.YML` terminal extensions for YAML input
  paths;
- parse the YAML through the shared typed schema;
- reject unknown schema versions unless an explicit migrator exists;
- reject missing target-required route coefficients;
- project route coefficients into the same PL schedule surfaces used by flat
  `ow-lanuse-1`;
- prove that production runtime eligibility is based on the YAML document, not
  on the original `.man`, optional reports, or sidecars.

## Validation

Validation must support:

- schema validation;
- datver support validation;
- coefficient completeness validation;
- coefficient domain validation;
- source/migration provenance validation where present;
- runtime eligibility validation for Lane D/native landuse physics.

Validation failures must be typed and name the missing or invalid field path.

## Migration Relationship

`openwepp-landuse-migrate` is the initial producer for this format. It reads
frozen legacy `.man` sources or flat `ow-lanuse-1` source and emits canonical
management YAML. It must not emit a native flat `.man` as its production output.

## Open Questions

- Final review/disposition of the recommended `openwepp-management-schema`
  crate ownership before implementation.
- Exact YAML layout for operation, initial-condition, and yearly schedule
  records after mapping the current parsed management model.
- Whether YAML schema publication should include generated JSON Schema in the
  crate artifact.
