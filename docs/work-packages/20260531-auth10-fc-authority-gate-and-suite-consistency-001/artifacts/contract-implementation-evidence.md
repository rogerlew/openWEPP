# AUTH10 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

Static:
- Closed AUTH09 follow-on provenance mismatch in active Level-3 WB19 suite:
  - `docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
  - `tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001/fixtures.provenance.yaml`
  - Active metadata now references `cas_l3_*` root/hash directly.
- Promoted independent direct-theta FC cohort authority from Level-5 periodic
  monitor to Level-4 required constitutive gate:
  - `docs/specifications/external-authority/registry.yaml`
  - moved suite spec:
    - `docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md`
    - -> `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
  - lane/failure posture is now `required` / `hard-fail`.
- Updated canonical contract authority text to reflect AUTH10 gate posture:
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `docs/specifications/science-contracts/index.md`
