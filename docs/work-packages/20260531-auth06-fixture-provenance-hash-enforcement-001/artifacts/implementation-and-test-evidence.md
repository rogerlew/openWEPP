# AUTH06 Implementation And Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement and validate fixture reproducibility enforcement for active
  external-authority suites.

## Static

1. Added lock/provenance sidecars for active Level-4 suites:
   - `tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/{fixtures.sha256,fixtures.provenance.yaml}`
   - `tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/{fixtures.sha256,fixtures.provenance.yaml}`
   - `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/{fixtures.sha256,fixtures.provenance.yaml}`
2. Added fixture hash/provenance metadata to active suite docs.
3. Added registry pointers for fixture lock/provenance sidecars:
   - `docs/specifications/external-authority/registry.yaml`
4. Added blocking fixture-integrity enforcement in:
   - `tools/release/run_release_candidate_gates.sh`
5. Updated operator/runbook docs:
   - `tools/release/README.md`
   - `docs/governance/openwepp-release-procedure-draft.md`

## Ran

1. `cargo test --test auth06_fixture_provenance_hash_enforcement_contract`
   - pass
2. `cargo test --test auth05_level4_constitutive_authority_hardening_contract --test auth06_fixture_provenance_hash_enforcement_contract`
   - pass
3. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth06 --release-dir /tmp/openwepp_auth06_release --skip-stability`
   - initial run: fail at `cargo fmt --check` (formatting in AUTH06 test)
4. `cargo fmt`
   - pass
5. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth06 --release-dir /tmp/openwepp_auth06_release --skip-stability`
   - pass
   - included clippy-driven cleanup for:
     - `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
     - `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
   - authority report generated at:
     - `/tmp/openwepp_auth06_release/authority_suite_results.md`
6. `cargo fmt --check`
   - pass
7. `markdown-doc lint --path docs/specifications/correctness-authority-model.md --path docs/specifications/external-authority/README.md --path docs/specifications/external-authority/suite-schema.md --path docs/specifications/external-authority/suite-template.md --path docs/specifications/external-authority/registry-template.yaml --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/README.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001`
   - pass
8. `markdown-doc validate --path docs/specifications/correctness-authority-model.md --path docs/specifications/external-authority/README.md --path docs/specifications/external-authority/suite-schema.md --path docs/specifications/external-authority/suite-template.md --path docs/specifications/external-authority/registry-template.yaml --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/package.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/artifacts`
   - pass
