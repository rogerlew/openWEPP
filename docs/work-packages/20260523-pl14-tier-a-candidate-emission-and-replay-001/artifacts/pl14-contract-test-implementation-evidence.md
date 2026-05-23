# PL14 Contract-Test Implementation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implemented Contract-Derived PL14 Target

- Added test target registration in `Cargo.toml`:
  - `name = "pl14_tier_a_candidate_replay_contract"`
  - `path = "tests/integration/pl14_tier_a_candidate_replay_contract.rs"`

- Added integration contract tests:
  - `pl14_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence`
  - `pl14_contract_conformance_rejects_missing_or_mismatched_single_ofe_metadata`
  - `pl14_contract_conformance_emits_replay_staging_rows_with_canonical_25_columns`
  - `pl14_contract_conformance_rejects_missing_required_replay_symbol`

## Command and Result

```bash
cargo test --test pl14_tier_a_candidate_replay_contract -- --nocapture
```

Result: `ok` (`4 passed`, `0 failed`).
