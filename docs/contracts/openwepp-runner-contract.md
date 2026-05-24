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

## CLI03 Runner Command Surface

`open_wepp_runner` must expose at least:

- `open_wepp_runner run-hillslope ...`
- `open_wepp_runner release lint --release-dir <path>`

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
   assembly are delegated to a dedicated outputs crate boundary (for CLI03,
   `crates/openwepp-hillslope-output/`) with crate-owned typed contracts and
   tests.

`release lint` requirements:

1. Enumerate candidate binaries in release directory.
2. Require `<binary>.json` sidecar for each candidate.
3. Validate sidecar schema id and required fields from
   `openwepp-binary-release-contract.md`.
4. Reject mixed/invalid watershed-hillslope pairing claims.

## Typed Runner Failure IDs

Runner boundary failures use stable IDs:

- `RUNNER-E-001`: missing required runner argument or unsupported runner flag
- `RUNNER-E-002`: hillslope binary path missing/unreadable
- `RUNNER-E-003`: launch failed before subprocess execution
- `RUNNER-E-004`: launched process exited non-zero
- `RUNNER-E-005`: release sidecar missing/invalid
- `RUNNER-E-006`: release binary naming contract violation

## CLI03 Output Guard IDs

Required CLI03 output-surface failures use stable existing guard IDs:

- `CLIHILL-E-013`: required hillslope CLI output surface missing.
- `OPEN_RUNNER-E-018`: missing required hillslope output surface(s).

## Error posture

Contract mismatches are hard errors, including:

- unsupported runner flags (including engine-selector flags);
- required binary release sidecar missing or invalid;
- invalid `.run` schema/version or unresolved required `.run` paths;
- non-metric `.run` unit-system selection;
- binary naming contract violations;
- incompatible watershed/hillslope release pairings.
