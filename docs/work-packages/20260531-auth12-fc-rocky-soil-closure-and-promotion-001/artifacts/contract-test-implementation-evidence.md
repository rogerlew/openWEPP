# AUTH12 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: Static

## Contract-Derived Test and Fixture Updates

1. `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/`
   - Added rocky anchor fixture: `h1_real_rocky_p1_authority.sol`.
   - Updated `cohort_case.json`:
     - `valid_9002_reference` expected status set to `within`.
     - Added `h1_real_rocky_authority` with expected status `within`.
   - Regenerated `fixtures.sha256` and updated `fixtures.provenance.yaml`.
2. `docs/specifications/external-authority/required-suite-obligations.json`
   - Added new required fixture and required case binding.
   - Increased minimum case count to 4.
   - Updated required threshold expectation for `valid_9002_reference` to
     `within`.
3. `tests/integration/auth07_fc_authority_cohort_contract.rs`
   - Added rocky-anchor fixture binding guard.
   - Updated cohort comparison to use layer-authoritative ProfileFCStore lineage
     (`Σ(thetfc_i * dg_i) * 1000`).
   - Updated lane/contract metadata assertions for AUTH12 posture.
4. `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
   - Updated posture checks to support pre-closure and post-closure states.
   - Added post-promotion requirement that closure package state is `complete`.
5. `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
   - Updated independent authority comparator to mirror AUTH12 measured-theta
     FC/WP producer/runtime paired multiplier semantics.
