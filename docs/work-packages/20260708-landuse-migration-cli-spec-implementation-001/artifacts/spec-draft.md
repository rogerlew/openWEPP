# Spec Draft

Status: implemented and finalized.

Spec path:
`docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`.

The implemented specification defines:

- package and binary names;
- frozen flat `.man` and flat `ow-lanuse-1` source policy;
- YAML-only native output policy;
- default output naming that appends `.yaml` to the input filename, producing
  `.man.yaml` for flat `.man` inputs;
- `--args-for-migration-to` input-specific argument discovery;
- `--validate` mode;
- migration command syntax;
- required disturbed-class authority for pre-native legacy cropland to
  `ow-lanuse-1` YAML;
- no sidecar and no compatibility-only policy;
- class-map contract;
- `--args-file`, validation-report, migration-report, and YAML output schema
  relationships;
- embedded Disturbed route-coefficient table requirements and checksum
  strategy;
- output, error, library API, test, and crates.io-readiness requirements.

Implemented schema resolutions:

- `--disturbed-class-map` accepts `.toml`, `.json`, `.yaml`, and `.yml`.
- `schedule_slot` keys use one-based
  `rotation_index:year_in_rotation:ofe_index:crop_slot`.
- `--args-file` carries optional `target`, `disturbed_class`, and
  `disturbed_class_map` fields.
- Discovery, validation, and migration reports have explicit field lists and
  `text`/`json`/`toml` rendering.
- Canonical output YAML is exactly `management-yaml.spec.md` schema version 1;
  the migration tool defines no separate YAML dialect.

Key policy: the migration output must be standalone YAML and
coefficient-complete for new-physics production. The tool fails closed rather
than writing an `ow-lanuse-1` YAML file with missing route coefficients for
migrated legacy cropland. The producer emits lowercase `.yaml` only; `.man.yaml`
is the default migrated-flat-source naming convention, not a consumer
requirement.
