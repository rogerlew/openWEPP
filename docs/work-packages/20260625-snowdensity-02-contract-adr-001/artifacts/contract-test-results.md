# Contract Test Results

Ran:

- `cargo test --test snowdensity02_contract_adr_guard`

Result:

- PASS.

Iteration notes:

- Initial run before `Cargo.toml` registration failed because the workspace
  enumerates integration test targets explicitly.
- Registered `snowdensity02_contract_adr_guard` in `Cargo.toml`.
- Second run exposed a weak handoff marker; the handoff now states
  `No per-site constants.`
- Final run passed `3` tests.

Guard coverage:

- `SC-SNOWFREEZE-001` v75 and `INV-SNOWFREEZE-051`.
- `snow_model = legacy_wepp | physics_bulk`.
- `legacy_wepp` default status.
- no-site-tuning / `ssd` non-promotion.
- Anderson-1976/SNOBAL candidate-envelope wording.
- `OBL-SNOWFREEZE-P-026`.
- ADR-0027 status and non-decision boundaries.
- Package-local no-production-runtime and handoff language.
