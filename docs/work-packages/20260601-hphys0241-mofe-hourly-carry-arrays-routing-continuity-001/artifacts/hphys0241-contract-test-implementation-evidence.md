# HPHYS0241 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static + ran

Static: contract-derived tests were added or amended before production-code
edits.

- `tests/integration/wb11_hydrology_kernel_contract.rs`: added HPHYS0241
  scheduler vectors proving array-derived carryover overrides stale
  compatibility runon, publishes current arrays, copy-forwards current arrays
  to upstream arrays, and rejects negative upstream carry payloads.
- `tests/integration/cli03_runner_contract_derived_tests.rs`: added manifest
  and watershed-validator source assertions for `mofe_hourly_carry`, 24-slot
  policy, required arrays, active/inactive posture, and carry totals.
- `tests/integration/mofe04_publication_contract_authority_closure_contract.rs`:
  added contract-surface assertions for WATBAL/SYSTEM HPHYS0241 authority.
- `tests/integration/mofe05_watershed_contributor_metadata_contract_authority_closure_contract.rs`:
  added contract-surface assertions for SYSTEM/ROUTE metadata gates.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: updated
  the valid multi-OFE manifest fixture to include active HPHYS0241
  carry-metadata, preserving existing MOFE05 negative vectors.

Ran:
- `cargo test --test wb11_hydrology_kernel_contract hphys0241` passed.
- `cargo test --test cli03_runner_contract_derived_tests hphys0241` passed.
- `cargo test --test mofe04_publication_contract_authority_closure_contract` passed.
- `cargo test --test mofe05_watershed_contributor_metadata_contract_authority_closure_contract` passed.
- `cargo test --test cli03_runner_contract_derived_tests mofe04` passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_mofe05` passed.
