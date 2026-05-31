# AUTH07 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Add contract-derived integration coverage for independent direct
  `theta_fc(-33kPa)` profile-store comparison and cohort classification.

## Static

1. Added integration test target:
   - `tests/integration/auth07_fc_authority_cohort_contract.rs`
2. Registered target in `Cargo.toml`:
   - `auth07_fc_authority_cohort_contract`
3. Assertions cover:
   - AUTH07 package/suite/registry/contract registration checks,
   - strict soil parse + runtime publication extraction,
   - direct authority `Σ(theta_fc * thickness)` reconstruction,
   - explicit relative-threshold classification, and
   - weighted rock-fragment bucket classification.

## Ran

1. `cargo test --test auth07_fc_authority_cohort_contract`
   - pass
