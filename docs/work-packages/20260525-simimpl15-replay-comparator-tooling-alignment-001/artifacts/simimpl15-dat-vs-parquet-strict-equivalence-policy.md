# simimpl15-dat-vs-parquet-strict-equivalence-policy

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- `.dat` lane policy is `strict-required` (raw strict comparator required).
- `.parquet` lane policy is `strict-equivalent-required` (semantic lane required).
- Semantic strict-equivalence blockers are enforced:
- semantic schema version mismatch.
- missing required investigation columns.
- zero emitted semantic column statistics.
- When blocker list is non-empty for parquet strict-equivalent mode, suite exits hard-fail.

## Ran
- `pl14s_contract_conformance_enforces_strict_lane_required_vs_strict_equivalent_modes` passed.
- `strict_lane_policy_mode_is_deterministic_by_candidate_extension` passed.
- Workspace tests and targeted SIMIMPL15 tests passed with policy checks active.
