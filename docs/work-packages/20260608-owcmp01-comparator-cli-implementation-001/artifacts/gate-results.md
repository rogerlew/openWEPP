# Gate Results

Status: complete
Evidence mode: Ran

## Focused Iteration Gates

| Command | Exit | Result |
|---|---:|---|
| `python3 -m py_compile tools/owcmp/semantic_wat.py tools/owcmp/pl14s_suite.py tools/owcmp/summary.py tools/owcmp/owcmp` | 0 | PASS |
| `cargo test --test owcmp_cli_contract` | 0 | PASS, 7 passed |
| `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract` | 0 | PASS, 8 passed |
| Manual `tools/owcmp/owcmp wat semantic` plus `tools/owcmp/owcmp summarize` smoke on temporary `.dat` fixtures | 0 | PASS, compact verdict `PASS` |
| Dynamic `tools/owcmp/owcmp pl14s run` smoke with fake baseline replay and fake strict comparator | 0 | PASS, emitted `pl14s-legacy-suite-v2` provenance with strict and semantic lanes |

## Pre-Handoff Sanity

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --check` | 0 | PASS |
| `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract` | 0 | PASS, 8 passed |
| `cargo test --test owcmp_cli_contract` | 0 | PASS, 7 passed |
| `git diff --check` | 0 | PASS |

## Selected Output

`cargo test --test owcmp_cli_contract`:

```text
running 7 tests
test owcmp_declares_pl14s_contract_markers_and_deferred_observe_boundary ... ok
test owcmp_observe_normalize_is_deferred ... ok
test owcmp_pl14s_run_emits_provenance_with_strict_and_semantic_lanes ... ok
test owcmp_summarize_reports_failed_commands_as_failed_verdict ... ok
test owcmp_summarize_reports_policy_skipped_commands_as_skipped ... ok
test owcmp_wat_semantic_rejects_duplicate_row_keys ... ok
test owcmp_summarize_emits_compact_json_and_markdown ... ok

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
test simimpl18_contract_requires_multi_day_storage_state_mutation ... ok
test simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage ... ok

test result: ok. 8 passed; 0 failed
```

Manual CLI smoke:

```text
{"summary_json": "/tmp/tmp.dtitUFxjs1/summary/summary.json", "summary_md": "/tmp/tmp.dtitUFxjs1/summary/summary.md", "verdict": "PASS"}
```

Dynamic `owcmp pl14s run` smoke:

```text
test owcmp_pl14s_run_emits_provenance_with_strict_and_semantic_lanes ... ok
```

The fixture uses a fake baseline replay script that writes `H1.wat.dat` and a
fake strict comparator that writes the requested strict JSON, then asserts the
resulting `pl14s_provenance_manifest.json` contains `pl14s-legacy-suite-v2`,
`strict-required`, `native-runtime-dat`, `semantic_pass: true`, and the
`tools/owcmp` tolerance path.

## Line-Count Governance

| File | Lines | Disposition |
|---|---:|---|
| `tools/owcmp/owcmp` | 80 | OK |
| `tools/owcmp/semantic_wat.py` | 496 | OK |
| `tools/owcmp/pl14s_suite.py` | 618 | OK |
| `tools/owcmp/summary.py` | 222 | OK |
| `tests/integration/owcmp_cli_contract.rs` | 417 | OK; below 2000-line warning threshold |

## Broader Gates Not Run

- `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
  --workspace`, and `cargo deny check` were not run. OWCMP01 is a tooling-local
  package with no Rust production/kernel edits, and `package.md` defines focused
  validation plus pre-handoff sanity gates for this slice.
- No authority-suite anti-evasion guard was run because this package does not
  edit external-authority suite posture, cohort fixtures, or required-case
  bindings. It only adds the parallel `owcmp` path and keeps the existing
  legacy-suite contract test unchanged.
