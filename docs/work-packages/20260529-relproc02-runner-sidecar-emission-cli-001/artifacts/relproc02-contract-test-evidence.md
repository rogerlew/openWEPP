# relproc02-contract-test-evidence

Status: complete  
Evidence mode: Static

## Tests Added

`crates/openwepp-runner/src/bin/open_wepp_runner.rs`:

1. `release_sidecar_command_writes_sidecar_for_requested_role`
   - Asserts `release sidecar` writes `<binary>.json`.
   - Asserts emitted payload `binary_role` matches requested role.
2. `release_sidecar_command_rejects_unsupported_role_value`
   - Asserts unsupported `--role` is rejected with typed missing-argument
     error surface (`RUNNER-E-001`).

These tests directly assert the newly documented command-surface contract for
explicit path/role sidecar emission.
