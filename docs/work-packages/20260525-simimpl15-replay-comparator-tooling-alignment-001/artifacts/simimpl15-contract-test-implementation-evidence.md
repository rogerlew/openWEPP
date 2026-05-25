# simimpl15-contract-test-implementation-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Contract-derived tests implemented/updated:
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- Assertions cover:
- v2 schema markers (`pl14s-semantic-wat-v2`, `pl14s-legacy-suite-v2`).
- required `--candidate-surface-source-class` argument and strict policy markers.
- strict source promotability hold behavior.
- `Total-Soil` canonical investigation-column continuity.

## Ran
- Targeted suite command passed:
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract --test pl14_tier_a_candidate_replay_contract --test pl14r_tier_a_replay_rerun_contract --test comparator_tier_routing_metadata -- --nocapture`.
