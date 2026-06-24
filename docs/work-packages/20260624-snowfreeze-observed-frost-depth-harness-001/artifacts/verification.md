# Verification

Evidence class: Ran.

Status: focused gates passed; broad workspace gates partially run.

## Focused Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Python compile | `PASS` | `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py` |
| Network acquisition | `PASS` | `.venv/bin/python tools/snowfreeze_observed/observed_harness.py --cache target/snowfreeze_observed fetch` |
| Corpus regeneration | `PASS` | `.venv/bin/python tools/snowfreeze_observed/observed_harness.py --cache target/snowfreeze_observed normalize --observations-dir tests/fixtures/snowfreeze_observed/observations` |
| Observation schema/provenance/checksum validation | `PASS` | `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations` |
| Rust observation contract | `PASS` | `cargo test --test snowfreeze_observed_frost_depth_contract` => 3 passed |

## Direct Comparison Matrix

| Site | Command result | Verdict | Evidence |
| --- | --- | --- | --- |
| `site1_sleepers_south_field_vt` | `PASS` | `UNRESOLVED` | `target/snowfreeze_observed_compare_site1_direct/comparison_report.json` |
| `site2_sleepers_w9_hardwood_vt` | `PASS` | `UNRESOLVED` | `target/snowfreeze_observed_compare_site2_direct/comparison_report.json` |
| `site3_scan_mandan_nd` | `runner failed` | `HARNESS-SURFACE-MISMATCH` | direct runtime lane 1 day 487 negative `storage_reconciliation.frost_storage_projection_theta_m` |
| `site4_ggd498_morris_mn` | `runner failed` | `HARNESS-SURFACE-MISMATCH` | direct runtime lane 1 day 10727 negative `storage_reconciliation.frost_storage_projection_theta_m` |
| `site5_reynolds_creek_us_rls_id` | `PASS` | `UNRESOLVED` | `target/snowfreeze_observed_compare_site5_direct/comparison_report.json` |

## Workspace Gates

| Gate | Result | Note |
| --- | --- | --- |
| `cargo fmt --check` | `PASS` | Ran after `cargo fmt`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | not run | Broad workspace gate; package touched Python/docs/one Rust contract test. |
| `cargo test --workspace` | not run | Broad workspace gate; focused package test passed. |
| `cargo deny check` | not run | Broad workspace gate. |
| `git diff --check` | `PASS` | No whitespace errors. |

## External-Authority / Anti-Evasion Gates

| Gate | Result | Note |
| --- | --- | --- |
| `bash tools/release/check_authority_suite_antievasion.sh` | not run | No authority-suite binding changes. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | not run | No required-suite binding changes. |

## Focused Test Transcript

```text
running 3 tests
test snowfreeze_observed_manifest_binds_external_authority_and_source_statuses ... ok
test snowfreeze_observed_harness_documents_no_defect_without_snow_depth_control ... ok
test snowfreeze_observed_csv_schema_preserves_measurement_correspondence ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```
