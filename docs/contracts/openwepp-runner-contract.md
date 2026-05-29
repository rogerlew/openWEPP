# open_wepp_runner Contract

Status: `normative`

This contract defines integration rules for launching openWEPP binaries.

Canonical runner identifier for this contract surface: `open_wepp_runner`
(in-repo openWEPP runner). Legacy spelling `openwepp_runner` is non-canonical.

## Scope

- openWEPP binary selection and invocation.
- Failure behavior for contract mismatch.

## Hard rules

1. `open_wepp_runner` is the only launcher for openWEPP binaries.
2. `wepppy` treats openWEPP as an external engine boundary; it does not depend
   on `wepp_runner` internals for openWEPP invocation semantics.
3. Legacy WEPP binary orchestration is owned by `wepppy/wepp_runner`, not
   `open_wepp_runner`.
4. No silent fallback across engine families or pass-family contracts.

## Invocation safety

- Use explicit argument arrays only; no shell interpolation.
- Emit binary identity and sidecar identity in launch telemetry.
- Reject mixed-family binary tuples (watershed/hillslope mismatch).
- Keep output-surface serialization/validation logic in a dedicated outputs
  crate; `open_wepp_runner` owns launch orchestration and policy enforcement,
  not inline output-family implementation.

## CLI03/CLI04 Runner Command Surface

`open_wepp_runner` must expose at least:

- `open_wepp_runner run-hillslope ...`
- `open_wepp_runner release lint --release-dir <path>`
- `open_wepp_runner release sidecar --binary <path> --role <role>`

`run-hillslope` requirements:

1. Invocation must execute `openwepp-cli-hill` with explicit argv capture.
2. Runner must propagate non-zero child exit as typed hard failure.
3. `.run` input must satisfy
   `openwepp-hillslope-runfile-contract.md` (`schema =
   openwepp-hillslope-runfile-v1`) with `unit_system = "metric"`, required
   core input bindings,
   optional sidecar override controls, and explicit output configuration:
   required `outputs.pass` (`.hbp`), required `outputs.loss` (`.json`), and
   optional parquet output paths (`outputs.wat`, `outputs.soil`,
   `outputs.plot`, `outputs.ebe`, `outputs.element`).
4. Legacy line-oriented stdin `.run` recipes are out of contract for this
   surface.
5. Required hillslope outputs are `outputs.pass` (`.hbp`) and `outputs.loss`
   (`.json`); optional parquet outputs are configured as optional `.run`
   `outputs` paths (`wat`, `soil`, `plot`, `ebe`, `element`).
6. Bootstrap-synthesized placeholder include surfaces are prohibited as
   production acceptance semantics.
7. `run-hillslope` does not accept or negotiate legacy-engine selectors.
8. Required/optional hillslope output serialization and manifest checksum
   assembly are delegated to a dedicated outputs crate boundary with crate-owned
   typed contracts and tests.
9. Output serialization is delegated to dedicated crate boundaries:
   - hillslope: `crates/openwepp-hillslope-output/`,
   - watershed: `crates/openwepp-watershed-output/`.
10. When `.run` `outputs.wat` is configured, emitted `H.wat.parquet` must
    preserve WEPPpy/WEPPpyo3 metadata parity:
    - field metadata keys `units` and `description`,
    - dataset metadata keys `dataset_version`, `dataset_version_major`,
      `dataset_version_minor`, and `schema_version`.
11. WAT schema/metadata authority uses default baseline with explicit exception:
    - default legacy comparator baseline remains
      `/workdir/wepp-forest_260430_baseline` at
      `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`,
    - WAT output semantics for consumer parity (including optional
      `InterceptionStorage`) follow post-`wepp_260430` `wepp-forest`/WEPPpy
      lineage per CLI04 stakeholder authority.

`release lint` requirements:

1. Enumerate candidate binaries in release directory.
2. Require `<binary>.json` sidecar for each candidate.
3. Validate sidecar schema id and required fields from
   `openwepp-binary-release-contract.md`.
4. Reject mixed/invalid watershed-hillslope pairing claims.

`release sidecar` requirements:

1. Require explicit `--binary <path>` and
   `--role <watershed|hillslope|replay>`.
2. Write `<binary_path>.json` sidecar using
   `openwepp-binary-release-metadata-v1` required fields.
3. Validate emitted sidecar before command success return.
4. Reject unsupported roles and missing required flags as hard errors.
5. Surface metadata IO/validation failures as typed errors; no silent fallback.

## Typed Runner Failure IDs

Runner boundary failures use stable IDs:

- `RUNNER-E-001`: missing required runner argument or unsupported runner flag
- `RUNNER-E-002`: hillslope binary path missing/unreadable
- `RUNNER-E-003`: launch failed before subprocess execution
- `RUNNER-E-004`: launched process exited non-zero
- `RUNNER-E-005`: release sidecar missing/invalid
- `RUNNER-E-006`: release binary naming contract violation

Release metadata emission failures use stable IDs:

- `RELMD-E-001`: sidecar/binary path IO failure
- `RELMD-E-002`: sidecar JSON serialization failure
- `RELMD-E-003`: emitted sidecar JSON parse failure
- `RELMD-E-004`: required sidecar field missing
- `RELMD-E-005`: sidecar field invalid

## CLI03/CLI04 Output Guard IDs

Required output-surface failures use stable existing guard IDs:

- `CLIHILL-E-013`: required hillslope CLI output surface missing.
- `OPEN_RUNNER-E-018`: missing required hillslope output surface(s).
- `CLIHILL-E-010`: output-contract mismatch (including `.run` output-shape
  violations and output-surface contract validation failures).

## Error posture

Contract mismatches are hard errors, including:

- unsupported runner flags (including engine-selector flags);
- required binary release sidecar missing or invalid;
- invalid `.run` schema/version or unresolved required `.run` paths;
- non-metric `.run` unit-system selection;
- binary naming contract violations;
- incompatible watershed/hillslope release pairings;
- missing `H.wat.parquet` metadata parity keys when `outputs.wat` is configured;
- unauthorized parquet dependency posture for new CLI04 implementation work
  (`arrow2` adoption where `parquet` + `arrow-array` + `arrow-schema` are
  required).
