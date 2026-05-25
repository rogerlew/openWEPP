# simimpl18-contract-test-implementation-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Contract-derived tests added before production edits:
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
  - Added SIMIMPL18 cold-day partition contract test:
    `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`.
  - Added SIMIMPL18 multi-day storage mutation contract test:
    `simimpl18_contract_requires_multi_day_storage_state_mutation`.
  - Tightened replay-suite schema marker test to require baseline-year policy
    and full-span comparability markers in
    `run_pl14s_legacy_suite.py` (`--baseline-year-policy`,
    `--expected-common-row-count`, `baseline_year_policy`,
    `expected_common_row_count`, `full_span_policy_ready`).

## Ran
- Command:
  - `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture`
- Result on current package state:
  - replay-suite schema marker assertions pass;
  - SIMIMPL18 physics assertions remain failing (expected unresolved blocker):
    - `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
    - `simimpl18_contract_requires_multi_day_storage_state_mutation`
