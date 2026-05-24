# PL14S Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Added PL14S contract-derived integration test target:
  - `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
  - registered in `Cargo.toml` as `pl14s_tier_a_candidate_emission_and_replay_contract`.
- Test coverage implemented:
  1. `pl14s_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence`
  - validates Tier-A routing authority remains `HigherConfidence` for single-OFE daily lanes.
  2. `pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers`
  - validates required PL14S schema/guard markers in semantic comparator, replay suite, and suite README authority text.
  3. `pl14s_contract_conformance_enforces_strict_lane_required_vs_skipped_modes`
  - validates strict comparator branch policy (`.dat` required, `.parquet` skipped).
  4. `pl14s_contract_conformance_rejects_duplicate_row_keys_in_semantic_lane_inputs`
  - executes semantic comparator with synthetic duplicate `(OFE,J,Y)` rows and validates hard-fail behavior.

## Ran
- Command
```bash
cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture
```
- Result: **pass** (`4 passed`).
