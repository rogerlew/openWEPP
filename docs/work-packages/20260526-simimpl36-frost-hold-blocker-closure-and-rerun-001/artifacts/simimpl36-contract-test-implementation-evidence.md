# SIMIMPL36 Contract Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Contract-derived test additions:
  - soil parser compatibility coverage for quoted `9002` policy-first header
    form with optional `avke` omission.
  - WB14 runoff reconciliation near-zero normalization guard behavior.
  - PL14S comparator/suite schema marker coverage for new partition and
    year-offset controls.

## Ran
- `cargo test --test infile_soil_parser_contract compatibility_accepts_quoted_9002_policy_first_header_form -- --exact`
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract wb14_contract_conformance_normalizes_within_tolerance_negative_runoff_before_writeback -- --exact`
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers -- --exact`
