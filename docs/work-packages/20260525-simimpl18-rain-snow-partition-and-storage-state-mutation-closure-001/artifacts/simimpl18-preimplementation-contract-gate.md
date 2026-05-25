# simimpl18-preimplementation-contract-gate

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Contract-first sequence checkpoint:
1. Canonical `SC-*` amendments complete for SIMIMPL18 authority.
2. Contract-derived tests added.
3. Pre-implementation failure capture executed before production edits.

## Ran
- Command:
  - `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture`
- Result:
  - `FAILED` (expected pre-implementation signal).
- Failing tests (pre-fix blockers confirmed):
  - `pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers`
    (`run_pl14s_legacy_suite.py` missing required SIMIMPL18 baseline-policy/full-span markers).
  - `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
    (observed `RM=4.4` on cold all-snow day; expected `RM=0`).
  - `simimpl18_contract_requires_multi_day_storage_state_mutation`
    (published storage tuple invariant across day-1/day-2 forcing).
- Gate decision:
  - `PASS` for pre-implementation contract gate purpose (tests fail on
    pre-fix behavior as required); production edits authorized next.
