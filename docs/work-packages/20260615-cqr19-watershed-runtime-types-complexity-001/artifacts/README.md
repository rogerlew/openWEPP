# CQR19 Artifacts

Artifact set for CQR19. Evidence entries must label `Static:` versus `Ran:`.

Status: complete pending package commit and push.

Static: package scope is behavior-preserving CRAP/cyclomatic-complexity
decomposition for
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`.

Static: primary target identity from live baseline was
`WatershedClimateRuntimeInputError::fmt` at line `330`, CRAP `420.0`, CC
`20.0`, coverage `0.0%`.

Ran: before and after LCOV plus `cargo crap` JSON are stored in this directory:

- `lcov_before.info`
- `crap_before.json`
- `lcov_after.info`
- `crap_after.json`

Static: final target CRAP is `6.0`; highest remaining target-file CRAP row is
`WatershedClimateRuntimeInputError::code` at `19.0`.
