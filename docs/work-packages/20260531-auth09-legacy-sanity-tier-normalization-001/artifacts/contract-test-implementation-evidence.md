# AUTH09 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

Static:
- Updated `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`:
  - suite spec path `cas_l4_* -> cas_l3_*`,
  - registry assertion `authority_level: 5 -> 3`,
  - fixture root and fixture `suite_id` assertions to `cas_l3_*`,
  - posture wording to Level-3 legacy/sanity evidence.
- Updated `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`:
  - fixture/suite arrays reference new `cas_l3_*` branch suite path/root,
  - function renamed to reflect mixed Level-4 + Level-3 coverage.
- Renamed fixture root:
  - `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/`
  - -> `tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001/`
- Updated fixture metadata:
  - `solwpv_fcdep_branch_cases.json` suite ID to `cas_l3_*`,
  - `fixtures.sha256` hash updated to
    `6efea65c268cd8cd632340666528d1a207165c3c84bf25f15bc435cc45ae7175`,
  - `fixtures.provenance.yaml` suite ID/hash/transform note updated.

Ran:
- `cargo test --workspace` passed, including:
  - `auth06_active_level4_and_level3_suites_publish_fixture_hashes_and_provenance_sidecars`
  - `auth08_suite_registry_and_contract_addendum_are_present`
  - `auth08_solwpv_branch_fixture_cases_enforce_fcdep_mutation_scope`
