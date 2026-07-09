# Disposition

Status: executed complete.

Accepted decisions:

- `openwepp-management-yaml` is a first-class management input surface, not a
  sidecar.
- `openwepp-management-yaml` is the canonical native producer document for
  `ow-lanuse-1+`.
- Flat native `.man` remains a parseable source/compatibility bridge.
- openWEPP does not need a native flat `.man` writer for producer-side native
  evolution.
- Producers emit lowercase `.yaml`; migrated flat `.man` defaults to
  `.man.yaml` naming.
- Consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML` only as dispatch
  extensions before schema validation.
- `crates/openwepp-management-schema` is the shared publishable schema owner.
- The real runner intake consumes YAML directly through the input-contract
  adapter, and the existing PL projection consumes YAML-derived route
  coefficients.

Implemented files:

- `crates/openwepp-management-schema/**`
- `crates/openwepp-input-contract/src/parsers/management.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `tests/integration/infile_management_yaml_contract.rs`
- `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml`

Authority updates:

- `SC-INFILE-MANAGEMENT-YAML-001`
- `SC-OFEROUTE-001` rev 50
- management-lanuse authority `LANUSE-AUTH-8`
- `management-yaml.spec.md`
- input-surface registry row promoted to `active`

Review disposition:

- Rust reviewer findings were addressed by tightening coefficient domains,
  enforcing schedule coverage, and adding runner-intake YAML dispatch proof.
- QA reviewer findings were addressed by removing stale package text, aligning
  spec/contract coefficient domains with implementation, recording line-count
  governance, and recording publish-risk posture.

Line-count governance:

- `crates/openwepp-input-contract/src/parsers/management.rs` is 2851 lines after
  this package. This file was already a large flat-management parser monolith;
  the package added a bounded YAML adapter to the existing parser boundary to
  avoid inventing a second runtime management model.
- Decomposition during this package would have broadened the write set into a
  mechanical parser split unrelated to YAML authorization. The follow-on split
  intent is to move YAML adapter/conversion helpers into a sibling
  `management_yaml.rs` parser module or similar during a targeted parser
  maintainability package, preserving public parse APIs and fixture identity.

Publish-risk disposition:

- `openwepp-management-schema` is marked `publish = true` and passes local
  package dry-run once recorded in `gate-results.md`.
- The crate uses `serde_yaml 0.9.34+deprecated`, which brings
  `unsafe-libyaml`. `cargo deny check` currently accepts the dependency set.
  This is acceptable for this authorization package because the schema crate is
  internal-first and the next producer package can revisit the YAML backend
  before public crate release if crates.io policy or maintenance posture
  requires it.

Known residuals:

- The migration CLI implementation remains out of scope and is owned by
  `20260708-landuse-migration-cli-spec-implementation-001`.
- JSON Schema artifact generation was not added in this package; the Rust serde
  schema is the implemented authority surface for v1.
- Full legacy flat-to-YAML conversion is intentionally not implemented here.
