# Gate Results

Status: completed

Evidence mode: Ran

Ran:

- `cargo test -p openwepp-summary-accumulator wb13_row_snaps_roundoff_negative_soil_evaporation_only_for_evappm_pmet_branch -- --nocapture` passed.
- `cargo test -p openwepp-summary-accumulator wb13_row -- --nocapture` passed
  after Claude Code review disposition.
- `cargo fmt --check` passed after Claude Code review disposition.
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-summary-accumulator --all-targets -- -D warnings`
  passed after Claude Code review disposition.
- `cargo test -p openwepp-hillslope-orchestrator hphys0264 -- --nocapture` passed.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage -- --nocapture` passed.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_multi_day_storage_state_mutation -- --nocapture` passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with warnings only: duplicate `getrandom`,
  duplicate `hashbrown`, duplicate `twox-hash`, and unmatched license allowance
  warnings for `ISC` and `Unicode-DFS-2016`.
- Full H1..H39 diagnostics ran at `/tmp/hphys0264_20260603T083941Z` and recorded
  semantic pass `0/39`.

Skipped:

- No external systems or network gates were required by this package.
