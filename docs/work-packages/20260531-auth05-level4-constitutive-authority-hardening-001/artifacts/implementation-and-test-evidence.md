# AUTH05 Implementation And Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement and validate AUTH05 Level-4 constitutive authority hardening.

## Static

1. AUTH05 package scaffold and registry entry added:
   - `docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/**`
   - `docs/work-packages/README.md`
2. Level-4 external-authority suite docs now cite only constitutive external
   references (no legacy parity authority IDs):
   - `cas_l4_soil_fc_minus33_001.md`
   - `cas_l4_soil_wp_minus1500_001.md`
   - `cas_l4_watbal_relax_to_fc_001.md`
3. Registry points all AUTH03 Level-4 suites at AUTH05 hardened target:
   - `docs/specifications/external-authority/registry.yaml`
4. Hardened integration target implements runtime authority comparison and
   fail-closed negative perturbation checks:
   - `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
5. Relax fixture assertion schema now requires explicit branch expectations:
   - `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json`

## Ran

1. `cargo test --test auth03_level4_constitutive_gate_contract --test auth05_level4_constitutive_authority_hardening_contract`
   - pass
2. `cargo fmt --check`
   - pass
3. `markdown-doc lint --path docs/specifications/external-authority/registry.yaml --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/work-packages/README.md --path docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001`
   - pass (`23 files validated, 0 errors, 0 warnings`)
4. `markdown-doc validate --path docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md --path docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md --path docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md --path docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/package.md --path docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/artifacts`
   - pass (`18 files validated, 0 errors`)
