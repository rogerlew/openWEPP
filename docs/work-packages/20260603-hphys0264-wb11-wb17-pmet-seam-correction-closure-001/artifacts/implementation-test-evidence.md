# Implementation/Test Evidence

Status: completed

Evidence mode: Static + Ran

Static:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  now detects `wb11_et_seed_branch_evappm` and requires finite PMET component
  symbols when that branch is active.
- PMET branch execution bypasses non-PMET stage/PT partition behavior,
  publishes `Etp = pmet.ep_m`, consumes soil/residue evaporation from
  `pmet.es_m`, and allows signed ET only under the EVAPPM branch marker.
- Signed negative PMET `es` is published as signed `Es`, `Er` remains
  non-negative, and the storage deficit is returned to top layer state.
- `crates/openwepp-runner/src/hillslope/mod.rs` and
  `crates/openwepp-summary-accumulator/src/lib.rs` pass and consume the EVAPPM
  branch marker so WB13/WAT publication can allow signed `Es` only for PMET
  EVAPPM lineage.

Ran:

- `cargo test -p openwepp-summary-accumulator wb13_row_allows_negative_soil_evaporation_only_for_evappm_pmet_branch -- --nocapture` passed.
- `cargo test -p openwepp-hillslope-orchestrator hphys0264 -- --nocapture`
  passed with both HPHYS0264 tests.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage -- --nocapture` passed.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires_multi_day_storage_state_mutation -- --nocapture` passed.
