# WSHEDIMPL39 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Added watershed CLI contract-derived applicability vectors in
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`:
  - `watershed_cli_rejects_missing_applicability_selector_block`
  - `watershed_cli_rejects_disallowed_perennial_stream_selector`
- Updated fixture runfile generator in the same file to include required
  `inputs.applicability` table for nominal vectors.
- Updated downstream cross-contract posture assertions:
  - `tests/integration/erod11_alias_boundary_ownership_contract.rs`:
    `GAP-ROUTE-005` expected status now `closed`,
  - `tests/integration/erod12_cross_domain_contract_closure_contract.rs`:
    `GAP-ROUTE-005` expected status now `closed`,
    `GAP-SYSTEM-001` expected status now `promotable-with-risk`.

## Ran
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> pass
- `cargo test -p openwepp --test erod11_alias_boundary_ownership_contract --test erod12_cross_domain_contract_closure_contract` -> pass
