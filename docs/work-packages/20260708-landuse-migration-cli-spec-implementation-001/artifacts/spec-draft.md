# Spec Draft

Status: drafted and amended for YAML-only output.

Spec path:
`docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`.

The draft specification defines:

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
- implementation-closure schema gate for class-map, `--args-file`, validation
  report, migration report, and YAML output formats;
- embedded Disturbed route-coefficient table requirements;
- output, error, library API, test, and crates.io-readiness requirements.

Key policy: the migration output must be standalone YAML and
coefficient-complete for new-physics production. The tool must fail closed
rather than write an `ow-lanuse-1` YAML file with missing route coefficients for
migrated legacy cropland. The producer emits lowercase `.yaml` only; `.man.yaml`
is the default migrated-flat-source naming convention, not a consumer
requirement.
