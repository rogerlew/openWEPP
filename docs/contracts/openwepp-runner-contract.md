# openwepp_runner Contract

Status: `normative`

This contract defines integration rules for launching openWEPP binaries.

## Scope

- openWEPP binary selection and invocation.
- Engine selection at caller boundaries.
- Failure behavior for contract mismatch.

## Hard rules

1. `openwepp_runner` is the only launcher for openWEPP binaries.
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

## Error posture

Contract mismatches are hard errors, including:

- selector missing or unsupported;
- required sidecar missing or invalid;
- binary naming contract violations;
- incompatible watershed/hillslope release pairings.
