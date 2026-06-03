# Verification Agent A

Status: completed-local

Evidence mode: Ran

Ran:

- `cargo test -p openwepp-hillslope-orchestrator hphys0264 -- --nocapture`
  passed.
- `cargo test -p openwepp-summary-accumulator wb13_row_snaps_roundoff_negative_soil_evaporation_only_for_evappm_pmet_branch -- --nocapture` passed.
- `cargo test -p openwepp-summary-accumulator wb13_row -- --nocapture` passed.
- `cargo fmt --check` passed after review disposition.
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-summary-accumulator --all-targets -- -D warnings`
  passed after review disposition.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage -- --nocapture` passed.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_multi_day_storage_state_mutation -- --nocapture` passed.

Conclusion:

- Contract-derived seam vectors and adjacent WB13 publication guards pass.
