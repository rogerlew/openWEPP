# SIMIMPL36 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Production/runtime/tooling implementation closure in scope:
  - WB14 near-zero runoff canonicalization before writeback publication.
  - Scheduler phase report now carries explicit decision-violation vectors.
  - Runner failure detail now includes last phase/message/decision context.
  - Soil parser compatibility quoted-header extension across disturbed datver
    variants.
  - Comparator and suite support candidate partition controls + candidate
    year-offset mapping.

## Ran
- Replay and comparator execution bundle:
  - `artifacts/replay-run-20260526T164400Z/`
- Required gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Targeted contract-derived tests:
  - `compatibility_accepts_quoted_9002_policy_first_header_form`
  - `wb14_contract_conformance_normalizes_within_tolerance_negative_runoff_before_writeback`
  - `pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers`
