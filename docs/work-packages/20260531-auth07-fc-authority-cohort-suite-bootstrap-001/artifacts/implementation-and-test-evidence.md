# AUTH07 Implementation And Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement and validate independent FC-authority cohort suite scaffolding with
  reproducible fixture metadata and contract-derived checks.

## Static

1. Added AUTH07 package scaffold and artifacts under:
   - `docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/`
2. Added external-authority suite spec:
   - `docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md`
3. Added AUTH07 cohort fixture root and sidecars:
   - `tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/`
4. Added AUTH07 integration test:
   - `tests/integration/auth07_fc_authority_cohort_contract.rs`
5. Added contract/index/registry references and package index entries:
   - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
   - `docs/specifications/science-contracts/index.md`
   - `docs/specifications/external-authority/registry.yaml`
   - `docs/work-packages/README.md`
6. Added fixture EOL normalization rules for constitutive fixtures:
   - `.gitattributes`

## Ran

1. `cargo fmt --check`
   - initial run: fail (AUTH07 test formatting drift)
2. `cargo fmt`
   - pass
3. `cargo fmt --check`
   - pass
4. `cargo test --test auth07_fc_authority_cohort_contract`
   - pass
5. `cargo test --test auth05_level4_constitutive_authority_hardening_contract --test auth07_fc_authority_cohort_contract`
   - pass
6. `cd tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001 && sha256sum --check --strict fixtures.sha256`
   - pass
7. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001 --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md --path docs/specifications/science-contracts/contracts/SC-SOIL-001.md --path docs/specifications/science-contracts/index.md`
   - pass
8. `markdown-doc validate --path docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/package.md --path docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/artifacts --path docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md --path docs/specifications/science-contracts/contracts/SC-SOIL-001.md --path docs/specifications/science-contracts/index.md`
   - pass
