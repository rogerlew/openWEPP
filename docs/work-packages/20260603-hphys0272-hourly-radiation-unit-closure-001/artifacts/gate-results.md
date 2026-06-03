# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: gate results are scoped to HPHYS0272 edits and known repository-level
residuals.

Ran:

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | pass | no formatting changes required |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | workspace clippy clean |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests --lib` | pass | `47 passed` |
| `cargo test --workspace` | fail/HOLD | existing SIMIMPL18 fixture failures with `HKERNEL-WB11-ET-E-003` in `pl14s_tier_a_candidate_emission_and_replay_contract`; unrelated to HPHYS0272 radiation seam |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | authority suite anti-evasion checks passed |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | pass | `2 passed` |
| `cargo deny check` | pass with warnings | duplicate/unmatched-license warnings only; advisories/bans/licenses/sources ok |
| `markdown-doc lint --path ...` | pass | `28 files validated, 0 errors, 0 warnings` |

Workspace failure detail:

- `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
  failed during fixture execution with `HKERNEL-WB11-ET-E-003`.
- `simimpl18_contract_requires_multi_day_storage_state_mutation` failed with
  the same `HKERNEL-WB11-ET-E-003`.
