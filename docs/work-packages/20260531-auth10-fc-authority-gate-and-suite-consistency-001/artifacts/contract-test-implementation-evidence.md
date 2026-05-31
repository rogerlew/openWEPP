# AUTH10 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

Static:
- Updated AUTH07 contract-derived integration suite to enforce direct
  threshold gate semantics (non-inverted):
  - `tests/integration/auth07_fc_authority_cohort_contract.rs`
  - removed `expect_exceeds_threshold` expectation pinning.
  - gate now fails when `relative_error > max_relative_error_threshold`.
- Updated AUTH07 fixture root and metadata to Level-4 suite identity:
  - moved fixture root:
    - `tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/`
    - -> `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/`
  - updated `cohort_case.json`, `fixtures.sha256`, `fixtures.provenance.yaml`.
  - removed unused `valid_9002` cohort fixture from this gate suite.
- Expanded AUTH06 fixture-integrity contract to include the promoted Level-4
  direct-theta FC suite:
  - `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`.

Ran:
- `cargo test --workspace` passed and includes:
  - `auth06_active_level4_and_level3_suites_publish_fixture_hashes_and_provenance_sidecars`
  - `auth07_package_and_suite_authority_sections_exist`
  - `auth07_profile_fc_authority_cohort_threshold_and_rock_bucket_classification`
