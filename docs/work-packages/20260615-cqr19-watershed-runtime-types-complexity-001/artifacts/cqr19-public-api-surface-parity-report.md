# CQR19 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in watershed
runtime type formatting/code paths. No public API change is authorized.

Static: final production edits are private helper extraction only. The following
public items are preserved:

- `WatershedRuntimeInputError`
- `WatershedRuntimeInputError::code`
- `impl fmt::Display for WatershedRuntimeInputError`
- `impl Error for WatershedRuntimeInputError`
- `WatershedClimateRuntimeRequest`
- `WatershedHillslopeClimateAssignment`
- `WatershedClimateRuntimeInputError`
- `WatershedClimateRuntimeInputError::code`
- `impl fmt::Display for WatershedClimateRuntimeInputError`
- `impl Error for WatershedClimateRuntimeInputError`

Static: no enum variants, field names, derives, visibility modifiers, public
method signatures, type aliases, trait impls, stable error IDs, or display text
were intentionally changed.

Ran: focused characterization tests assert all public stable error IDs and all
`Display` strings for the touched public error types.
