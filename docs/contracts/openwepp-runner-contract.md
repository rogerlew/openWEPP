# open_wepp_runner Contract

Status: `normative`

This contract defines integration rules for launching openWEPP binaries.

Canonical runner identifier for this contract surface: `open_wepp_runner`
(in-repo openWEPP runner). Legacy spelling `openwepp_runner` is non-canonical.

## Scope

- openWEPP binary selection and invocation.
- Engine selection at caller boundaries.
- Failure behavior for contract mismatch.

## Hard rules

1. `open_wepp_runner` is the only launcher for openWEPP binaries.
2. `wepppy` treats openWEPP as an external engine boundary; it does not depend
   on `wepp_runner` internals for openWEPP invocation semantics.
3. Engine selection is explicit at request/config boundaries.
4. No silent fallback between engines or pass-family contracts.

## Engine selector

Required selector values:

- `legacy_wepp`
- `openwepp`

If selector is missing at a boundary that supports both engines, callers must
fail with a typed configuration error. Defaulting by implicit binary discovery
is not permitted for dual-engine paths.

## Invocation safety

- Use explicit argument arrays only; no shell interpolation.
- Emit binary identity and sidecar identity in launch telemetry.
- Reject mixed-family binary tuples (watershed/hillslope mismatch).

## CLI01 Runner Command Surface

`open_wepp_runner` must expose at least:

- `open_wepp_runner run-hillslope ...`
- `open_wepp_runner release lint --release-dir <path>`

`run-hillslope` requirements:

1. Engine selector is required and explicit.
2. CLI01 launch path supports `openwepp` selector without fallback.
3. Invocation must execute `openwepp-cli-hill` with explicit argv capture.
4. Runner must propagate non-zero child exit as typed hard failure.

`release lint` requirements:

1. Enumerate candidate binaries in release directory.
2. Require `<binary>.json` sidecar for each candidate.
3. Validate sidecar schema id and required fields from
   `openwepp-binary-release-contract.md`.
4. Reject mixed/invalid watershed-hillslope pairing claims.

## CLI01 Typed Runner Failure IDs

Runner boundary failures use stable IDs:

- `RUNNER-E-001`: missing/unsupported engine selector
- `RUNNER-E-002`: hillslope binary path missing/unreadable
- `RUNNER-E-003`: launch failed before subprocess execution
- `RUNNER-E-004`: launched process exited non-zero
- `RUNNER-E-005`: release sidecar missing/invalid
- `RUNNER-E-006`: release binary naming contract violation

## Error posture

Contract mismatches are hard errors, including:

- selector missing or unsupported;
- required sidecar missing or invalid;
- binary naming contract violations;
- incompatible watershed/hillslope release pairings.
