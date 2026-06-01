# AUTH11 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

Static:
- Restored anchored discrepancy fixture coverage in direct-theta cohort:
  - added `valid_9002.sol` back to
    `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/`
  - updated `cohort_case.json` with explicit
    `expected_threshold_status` classification.
  - updated `fixtures.sha256` and `fixtures.provenance.yaml`.
- Updated AUTH07 contract-derived test to enforce anchor bindings and threshold
  status classification:
  - `tests/integration/auth07_fc_authority_cohort_contract.rs`
- Added machine-checking test for obligation guards:
  - `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
- Added source-level anti-evasion review script:
  - `tools/release/check_authority_suite_antievasion.sh`
  - documented in `tools/release/README.md`.

Ran:
- `bash tools/release/check_authority_suite_antievasion.sh --base-ref 0dc1788 --head-ref HEAD`
  - result: pass
