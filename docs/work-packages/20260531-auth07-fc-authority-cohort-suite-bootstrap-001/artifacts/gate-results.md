# AUTH07 Gate Results

Status: completed  
Evidence mode: Ran

## Scope
- Execute AUTH07 validation gates for FC-authority cohort suite bootstrap.

## Commands run

1. `cargo fmt --check`
   - initial run: fail (format drift in AUTH07 integration test)
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

## Gate decision
- pass
