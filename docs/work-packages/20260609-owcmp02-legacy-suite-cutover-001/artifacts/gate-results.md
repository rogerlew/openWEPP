# Gate Results

Status: complete
Evidence mode: Ran

| Command | Exit | Result |
|---|---:|---|
| `python3 -m py_compile tools/owcmp/semantic_wat.py tools/owcmp/pl14s_suite.py tools/owcmp/summary.py tools/owcmp/owcmp` | 0 | PASS |
| `cargo test --test owcmp_cli_contract` | 0 | PASS, 7 passed |
| `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract` | 0 | PASS, 8 passed |
| `cargo fmt --check` | 0 | PASS |
| `git diff --check` | 0 | PASS |
| `test ! -e tools/legacy_comparison_suite` | 0 | PASS, `legacy_suite_absent` |
| `find tools/owcmp tests/integration docs/work-packages/20260609-owcmp02-legacy-suite-cutover-001 -type d -name __pycache__ -print` | 0 | PASS after cleanup, no output |
| `rg -l "legacy_comparison_suite|semantic_hillslope_wat_compare|run_pl14s_legacy_suite" -g '!target/**'` | 0 | PASS with disposition; 140 files, no active blockers |

## Selected Output

`cargo test --test owcmp_cli_contract`:

```text
running 7 tests
test owcmp_declares_pl14s_contract_markers_and_deferred_observe_boundary ... ok
test owcmp_observe_normalize_is_deferred ... ok
test owcmp_summarize_reports_policy_skipped_commands_as_skipped ... ok
test owcmp_summarize_reports_failed_commands_as_failed_verdict ... ok
test owcmp_wat_semantic_rejects_duplicate_row_keys ... ok
test owcmp_summarize_emits_compact_json_and_markdown ... ok
test owcmp_pl14s_run_emits_provenance_with_strict_and_semantic_lanes ... ok

test result: ok. 7 passed; 0 failed
```

`cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`:

```text
running 8 tests
test pl14s_contract_conformance_classifies_candidate_source_provenance ... ok
test pl14s_contract_conformance_enforces_strict_lane_required_vs_strict_equivalent_modes ... ok
test pl14s_contract_conformance_requires_conversion_dat_row_consistency_for_evidence_readiness ... ok
test pl14s_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence ... ok
test pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers ... ok
test pl14s_contract_conformance_rejects_duplicate_row_keys_in_semantic_lane_inputs ... ok
test simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage ... ok
test simimpl18_contract_requires_multi_day_storage_state_mutation ... ok

test result: ok. 8 passed; 0 failed
```
