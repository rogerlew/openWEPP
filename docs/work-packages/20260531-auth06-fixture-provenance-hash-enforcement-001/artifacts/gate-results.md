# AUTH06 Gate Results

Status: completed  
Evidence mode: Ran

## Scope
- Execute AUTH06 validation gates for fixture provenance/hash enforcement.

## Commands run

1. `cargo test --test auth06_fixture_provenance_hash_enforcement_contract`
   - pass
2. `cargo test --test auth05_level4_constitutive_authority_hardening_contract --test auth06_fixture_provenance_hash_enforcement_contract`
   - pass
3. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth06 --release-dir /tmp/openwepp_auth06_release --skip-stability`
   - initial run: fail at `cargo fmt --check` (AUTH06 test formatting drift)
4. `cargo fmt`
   - pass
5. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth06 --release-dir /tmp/openwepp_auth06_release --skip-stability`
   - pass
   - authority report:
     - `/tmp/openwepp_auth06_release/authority_suite_results.md`
6. `cargo fmt --check`
   - pass
7. `markdown-doc lint --path docs/specifications/correctness-authority-model.md --path docs/specifications/external-authority/README.md --path docs/specifications/external-authority/suite-schema.md --path docs/specifications/external-authority/suite-template.md --path docs/specifications/external-authority/registry-template.yaml --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/README.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001`
   - pass
8. `markdown-doc validate --path docs/specifications/correctness-authority-model.md --path docs/specifications/external-authority/README.md --path docs/specifications/external-authority/suite-schema.md --path docs/specifications/external-authority/suite-template.md --path docs/specifications/external-authority/registry-template.yaml --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/package.md --path docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/artifacts`
   - pass

## Gate decision
- pass
