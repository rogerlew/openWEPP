# openWEPP Hillslope CLI Specification (`RUNNER-HILL-CLI-001`)

Status: `draft-normative`  
Queue anchor: `CLI04-openwepp-output-hillslope-wat-parquet-architecture-001`  
Evidence mode: `Static`  
Ran evidence: none

## Purpose

Define the canonical executable contract for `openwepp-cli-hill` so openWEPP
can produce simulation-driven, provenance-valid partitioned interchange outputs
for wepppy/wepppyo3 consumer workflows.

## Normative Scope

- CLI boundary and invocation rules for hillslope runs.
- Required schema-versioned `.run` ingestion and validation policy.
- Required/optional output families for hillslope runs.
- Dedicated shared output crate organization boundary for output contracts,
  serializers, and output-surface tests.
- WAT parquet metadata parity requirements for WEPPpy/WEPPpyo3 consumers.
- Run-level provenance metadata artifact requirements.
- Build/release binary metadata sidecar requirements aligned with existing
  WEPP sidecar practice.

## Out of Scope

- Watershed CLI (`openwepp-cli-watershed`) detailed surface.
- Replay CLI (`openwepp-replay`) detailed surface.

## Authority Anchors

- `docs/decisions/0004-subprocess-hillslope-orchestration.md`
- `docs/decisions/0006-three-binaries-incl-replay.md`
- `docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/decisions/0005-parquet-via-wepppyo3-interchange.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`

## Binary Identity Model

| Role | Logical command identity | Release artifact name policy | Required sidecar |
|---|---|---|---|
| hillslope run driver | `openwepp-cli-hill` | `^openwepp_[0-9]{6}[a-z0-9_-]*_hill$` | `<binary>.json` |

Notes:
- `openwepp-cli-hill` is the executable role identity.
- Release artifact naming and sidecar schema validation are governed by
  `openwepp-binary-release-contract.md`.

## Schema-Versioned `.run` Mode (Required)

`openwepp-cli-hill` must consume a single schema-versioned `.run` file that
conforms to `openwepp-hillslope-runfile-v1`:

1. File format must be TOML 1.0 (`UTF-8`).
2. Top-level required keys:
   - `schema` (exactly `openwepp-hillslope-runfile-v1`)
   - `run_name`
   - `unit_system` (exactly `metric`)
3. `[inputs]` required keys:
   - `soil`
   - `management`
   - `slope`
   - `climate`
4. `[inputs]` optional keys:
   - `wepp_ui` (`bool`)
   - `pmetpara` (`string` path)
5. `[inputs.snow]` optional override keys:
   - `rst`
   - `newsnw`
   - `ssd`
6. `[inputs.frost]` optional override keys:
   - `wintRed`
   - `fineTop`
   - `fineBot`
   - `ksnowf`
   - `kresf`
   - `ksoilf`
   - `kfactor1`
   - `kfactor2`
   - `kfactor3`
7. `[outputs]` keys:
   - required: `pass` (`.hbp` path), `loss` (`.json` path)
   - optional string parquet paths: `wat`, `soil`, `plot`, `ebe`, `element`
8. Path semantics:
   - absolute paths are accepted;
   - relative paths are resolved against the directory containing the `.run`
     file;
   - shell interpolation and glob expansion are prohibited.
9. Invalid schema/version, missing required keys, non-metric unit selection,
   unresolved required inputs,
   and unwritable output destinations are typed hard failures.
10. Legacy line-oriented WEPP `.run` stdin recipes are out of contract.
11. When `outputs.wat` is configured, emitted parquet must preserve:
   - field metadata keys `units` and `description`,
   - dataset metadata keys `dataset_version`, `dataset_version_major`,
     `dataset_version_minor`, `schema_version`.

Sidecar semantics:

- `snow` and `frost` are override surfaces and do not gate whether snow/frost
  routines execute.
- Legacy compatibility mode `--legacy-sidecar-discovery` discovers
  `snow.txt`, `frost.txt`, `wepp_ui.txt`, and `pmetpara.txt` as optional
  run-directory files.
- In `--legacy-sidecar-discovery` mode, discovered sidecars are authoritative;
  `.run` sidecar override keys (`inputs.wepp_ui`, `inputs.pmetpara`,
  `inputs.snow`, `inputs.frost`) are ignored.

## Invocation Contract

`openwepp-cli-hill` execution must be launcher-safe and deterministic:

1. The production launcher boundary is in-repo `open_wepp_runner`; wepppy integrates
   through this boundary and must not depend on shell interpolation.
2. Invocation uses explicit argument arrays only (`std::process::Command`).
3. `open_wepp_runner` is openWEPP-only and does not accept legacy-engine
   selectors.
4. Legacy WEPP orchestration belongs to `wepppy/wepp_runner`.

Minimum execution inputs:
- `.run` control file satisfying `openwepp-hillslope-runfile-v1`
- hillslope core input files declared in `.run` (`.sol`, `.man`, `.slp`, `.cli`)
- optional override inputs declared in `.run` (`wepp_ui`, `pmetpara`,
  `[inputs.snow]`, `[inputs.frost]`) or discovered via
  `--legacy-sidecar-discovery`
- metric unit declaration (`unit_system = "metric"`)

## Output Contract

For simulation-driven CLI03/CLI04 runs, `openwepp-cli-hill` must emit:

Required outputs:

1. pass output (`.hbp`) at `.run` `outputs.pass`
2. loss output (`.json`) at `.run` `outputs.loss`

Optional parquet outputs when configured in `.run`:

1. `outputs.wat` (`.parquet`)
2. `outputs.soil` (`.parquet`)
3. `outputs.plot` (`.parquet`)
4. `outputs.ebe` (`.parquet`)
5. `outputs.element` (`.parquet`)

`outputs.wat` metadata parity requirements:

1. `outputs.wat` must preserve WEPPpy/WEPPpyo3 field metadata keys
   `units` and `description`.
2. `outputs.wat` schema metadata must include
   `dataset_version`, `dataset_version_major`,
   `dataset_version_minor`, and `schema_version`.
3. WAT schema authority is the WB13 canonical projection plus coordinated
   openWEPP/wepppyo3 interchange extensions. Dataset version `1.4` adds the
   required `frdp` frost-front depth column in `mm`; earlier post-`wepp_260430`
   lineage exceptions remain optional producer-authoritative `Interception` and
   `InterceptionStorage` parquet terms.

`crop` output is intentionally out of scope for this revision until its
columnar/output contract authority is defined.

Output requirements:
- Output files must be generated by openWEPP runtime execution from accepted
  runtime surfaces, not copied/substituted from legacy WEPP lanes.
- Bootstrap-synthesized placeholder outputs are prohibited as acceptance
  semantics.
- Omission of required outputs is a typed hard failure.

## Output Organization Boundary (CLI04 Required)

For CLI04 implementation scope, output behavior must be organized under a
dedicated crate boundary:

- target shared crate path: `crates/openwepp-output/`
- CLI03 predecessor path (transition-only): `crates/openwepp-hillslope-output/`
- ownership: hillslope and watershed output contracts, output-path validation,
  serializers, and output-surface checksum helpers
- non-ownership: process launch orchestration and child-process lifecycle
  management (remain in runner boundary)
- parquet dependency policy for new CLI04 implementation work:
  - required `arrow-rs` ecosystem crates:
    `parquet` + `arrow-array` + `arrow-schema`
  - `arrow-schema` is a companion crate within `arrow-rs`, not an alternative
  - `arrow2` adoption is prohibited for new implementation work in this package

Minimum crate modules:
- `contracts` (typed output config and invariants)
- `writers` (pass/loss/parquet emission paths)
- `manifest` (output checksum mapping and manifest payload assembly)

Test minimums for this crate:
- unit tests for extension/path and required/optional output invariants
- serializer tests for pass/loss and optional parquet path handling
- manifest checksum tests proving required coverage and deterministic ordering
- WAT parquet schema tests proving metadata parity for
  `units`/`description` (field metadata) and dataset version keys
  (schema metadata)
- dependency policy tests/static gates proving no new `arrow2` adoption for
  CLI04 implementation surfaces
- integration wiring tests from runner -> outputs crate surface

## Run Provenance Manifest (Required)

Each successful hillslope run must emit a provenance manifest (JSON), default:

`<launcher-managed output location>/openwepp_hillslope_run_manifest.json`

Manifest path selection is launcher-managed and is not a `.run` output key.

Required fields:
- `schema` (manifest schema id)
- `engine` (`openwepp`)
- `binary_path`
- `binary_sha256`
- `binary_sidecar_path`
- `binary_sidecar_sha256`
- `source_commit`
- `invoked_utc`
- `argv` (explicit argument vector)
- `run_dir`
- `run_file`
- `input_checksums` (file -> sha256)
- `output_checksums` (file -> sha256)

At minimum, `output_checksums` must include required pass/loss outputs
(`.hbp`, `.json`) and any optional parquet outputs that were configured for the
run.

Manifest schema id for this revision:
- `openwepp-hillslope-run-manifest-v1`

Determinism requirements:
1. `argv` order must match actual process invocation order.
2. `input_checksums` and `output_checksums` must be key-sorted by path string.
3. `invoked_utc` must be RFC 3339 UTC with `Z` suffix.

## Build Metadata Sidecar Contract (Required)

Build/release artifacts for the hillslope binary must include:
- binary file matching release naming policy
- metadata sidecar at `<binary>.json`

Sidecar constraints:
1. schema id must be `openwepp-binary-release-metadata-v1`.
2. required field set must satisfy
   `docs/contracts/openwepp-binary-release-contract.md`.
3. overlapping fields with WEPP sidecars from `/workdir/wepp-forest` must keep
   compatible semantics and types (`binary_name`, `binary_role`, `release_tag`,
   `source_repo`, `source_commit`, `built_utc`, `sha256`, `features`,
   `validation`).
4. sidecar absence or schema violation blocks release.

## Required Invariants

| Invariant ID | Requirement | Failure posture |
|---|---|---|
| `RUNNER-HILL-INV-001` | Required hillslope outputs are simulation-driven, openWEPP-emitted interchange artifacts, never legacy substitutions or bootstrap synth placeholders. | hard-fail + release/comparator hold |
| `RUNNER-HILL-INV-002` | `.run` input must declare explicit required input bindings and a schema value of `openwepp-hillslope-runfile-v1`. | hard-fail |
| `RUNNER-HILL-INV-003` | Missing required run outputs (`outputs.pass` `.hbp`, `outputs.loss` `.json`) blocks acceptance. | hard-fail |
| `RUNNER-HILL-INV-004` | Binary metadata sidecar `<binary>.json` is present and schema-valid for every release build. | hard-fail |
| `RUNNER-HILL-INV-005` | Invocation boundary uses explicit args, is openWEPP-only, and rejects legacy-engine selector flags. | hard-fail |
| `RUNNER-HILL-INV-006` | Relative paths in `.run` are resolved against the `.run` parent directory; unresolved required paths or unwritable output destinations block runtime start. | hard-fail |
| `RUNNER-HILL-INV-007` | `snow`/`frost` surfaces are parameter overrides and do not toggle snow/frost routine execution. | hard-fail |
| `RUNNER-HILL-INV-008` | openWEPP hillslope `.run` unit system is metric-only (`unit_system = "metric"`); non-metric unit selections are rejected. | hard-fail |
| `RUNNER-HILL-INV-009` | Output-family serialization/validation logic is implemented via dedicated shared outputs crate boundary (`crates/openwepp-output/` target; `crates/openwepp-hillslope-output/` transition-only predecessor) with crate-owned tests. | hard-fail + package hold |
| `RUNNER-HILL-INV-010` | When configured, `outputs.wat` emission preserves WEPPpy/WEPPpyo3 metadata parity (`units`/`description` field metadata + required dataset metadata version keys). | hard-fail |
| `RUNNER-HILL-INV-011` | `outputs.wat` schema authority preserves WB13 canonical daily projection and permits optional producer-authoritative `Interception` and `InterceptionStorage` terms per post-`wepp_260430` consumer-lineage authority. | hard-fail + review hold |
| `RUNNER-HILL-INV-012` | New CLI04 parquet implementation work uses `arrow-rs` crate stack (`parquet`, `arrow-array`, `arrow-schema`) and does not adopt `arrow2`. | hard-fail + package hold |

## Implementation Sequencing Requirement

For CLI03/CLI04 code-authoring work where contract authority applies:
1. amend/ratify canonical contracts and this subsystem spec,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Contract-Test Minimums (CLI03/CLI04)

1. Cargo metadata shows a binary target for hillslope CLI role.
2. `.run` parse/validation tests enforce:
   - schema/version match,
   - required key presence,
   - relative/absolute path resolution rules,
   - hard-fail on unresolved required paths.
3. Fixture integration test emits required run outputs
   (`outputs.pass`, `outputs.loss`) and configured optional parquet outputs
   at their configured `.run` paths, then writes a manifest with checksums.
4. Release artifact test validates `<binary>.json` sidecar presence and schema
   conformance.
5. Output crate tests validate required/optional output contract behavior and
   deterministic manifest checksum assembly.
6. WAT parquet schema tests validate presence/values of metadata keys:
   - field metadata keys: `units`, `description`,
   - schema metadata keys: `dataset_version`, `dataset_version_major`,
     `dataset_version_minor`, `schema_version`.
7. WAT schema parity tests validate canonical WB13 projection compatibility and
   optional `Interception` / `InterceptionStorage` handling.
8. CLI04 dependency posture tests/static gates enforce shared-boundary parquet
   stack policy (`parquet`, `arrow-array`, `arrow-schema`) and reject new
   `arrow2` adoption.
