# AUTH12 Implementation and Test Evidence

Status: complete  
Evidence mode: Static + Ran

## Production Implementation

1. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
   - Added datver-policy-aware FC/WP correction policy for measured-theta
     families (`7777/7778/9002/9003/9005`) aligned to WEPPpy producer contract.
   - Preserved corrected-lineage processing and applied paired runtime `cpm`
     multiplication for measured FC/WP payloads (legacy `scon.for` basis).
   - Threaded policy through seed expansion, normalized-layer mapping, and WB13
     profile symbol computation.
2. `crates/openwepp-runner/src/hillslope/mod.rs`
   - Updated WB11 seeding saturation-floor and theta-store coupling to retain
     paired `por*cpm` saturation semantics for measured-theta datvers.
   - Added unit tests:
     `auth12_wb11_seed_applies_cpm_for_disturbed_measured_fcwp_lineage` and
     `auth12_wb11_seed_applies_cpm_for_legacy_measured_theta_fcwp_lineage`.
3. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   - Updated normalized corrected-layer test seed construction to include the
     datver-aware FC/WP policy field and producer-contract policy guards.

## Green-State Confirmation

Ran:

```bash
cargo test --test auth07_fc_authority_cohort_contract
cargo test --test auth11_required_suite_obligation_guards_contract
cargo test --test auth05_level4_constitutive_authority_hardening_contract
cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires
```

All above passed after implementation closure.
