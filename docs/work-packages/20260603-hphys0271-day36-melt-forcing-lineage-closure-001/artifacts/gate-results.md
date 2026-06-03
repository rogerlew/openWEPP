# Gate Results

Status: completed/HOLD
Evidence mode: ran

Ran:

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | pass | Passed after `cargo fmt`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | Passed after scoped Clippy allowances for migrated melt helper length and trace-builder melt-term similar names. |
| `cargo test -p openwepp-runner hphys0271_trace_row_captures_melt_term_hourly_forcing_maps --lib -- --nocapture` | pass | `1 passed`. |
| `cargo test --test clim05_snow_runtime_kernel_contract hphys0271_contract_conformance_records_melt_terms_and_hourly_forcing -- --nocapture` | pass | `1 passed`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | `PASS: authority suite anti-evasion checks passed.` |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | pass | `2 passed`. |
| `cargo test --workspace` | fail | Known unrelated SIMIMPL18 fixture failures in `pl14s_tier_a_candidate_emission_and_replay_contract`: `HKERNEL-WB11-ET-E-003` domain violation. All HPHYS0271/CLIM05 tests passed. |
| `cargo deny check` | pass with warnings | Advisories/bans/licenses/sources ok; existing warnings for unmatched license allowances and duplicate dependency versions. |
| `markdown-doc lint --path ...` | pass | `28 files validated, 0 errors, 0 warnings`. |

Static: The workspace test failures pre-existed in this line of work and are not caused by HPHYS0271 trace publication.
