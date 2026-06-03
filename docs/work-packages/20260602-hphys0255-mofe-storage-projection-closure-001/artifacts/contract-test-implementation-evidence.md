# Contract-Test Implementation Evidence

Status: complete
Evidence mode: static + ran

Static: added tests

- `tests/integration/wb11_storage_projection_kernel_contract.rs`
  - Added `hphys0255_contract_authority_sections_exist`.
  - Added
    `hphys0255_mofe_seed_projection_separates_scoped_ofe_soil_from_active_wb11_state`.
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - Extended
    `cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area`
    to require `"storage_lineage_policy": "single-runtime-wb11-state"`.

Ran: pre-production tests

- `cargo test --test wb11_storage_projection_kernel_contract hphys0255 -- --nocapture`
  passed.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe04_multiofe_publication_uses_canonicalized_oferow_and_total_area -- --nocapture`
  failed before production edit because the manifest field was absent.

Ran: post-production tests

- Both targeted commands passed after adding publication provenance.
