# Gates

Evidence class: `[DIRECT][Ran] + [INFERENCE][Static]`.

| Gate | Status | Evidence |
|---|---|---|
| Focused runtime-selection tests | PASS | `cargo test -p openwepp-runner r7e_default_candidate -- --nocapture` passed: `3` tests, including `r7e_default_candidate_legacy_sidecar_discovery_uses_direct_manifest`. The manifest selected `direct-production-executor`, reported no fallback reason, and emitted `direct_runtime_counters/compatibility_edge_invocations = 0`. |
| Multi-OFE/Wave-2 direct cutover | PASS | `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection -- --nocapture` passed: `1` test. The no-env default selected direct production and the manifest reported `erod14_wave2_enabled = true`, `erod14_wave2_kernel_status_seen = true`, and water-transfer-only qin policy. |
| Public WAT inter-OFE handoff | PASS | `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows -- --nocapture` passed: `1` test. Downstream `UpStrmQ` now equals upstream public `QOFE` scaled by the upstream/downstream area ratio. |
| Direct transfer unit contract | PASS | `cargo test -p openwepp-hillslope-orchestrator r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j -- --nocapture` passed: `1` test. Raw hourly carry stays lane-buffer-local and published transfer/input accounting uses the area-scaled downstream input. |
| Legacy sidecar-discovery direct replay | PASS | `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` passed: `8` tests. The legacy sidecar-discovery replay now closes under direct production. |
| Frost storage source isolation | PASS | `cargo test -p openwepp-hillslope-orchestrator r7h_explicit_frost_storage_source_does_not_rewrite_r4a_layer_projection -- --nocapture` passed: `1` test. |
| Direct frame size guard | PASS | `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture` passed: `1` test. `DirectDayFrame=12400` remains under the ratified `12416` bound. |
| Format and whitespace | PASS | `cargo fmt --check` passed. `git diff --check` passed. |
| Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` passed. |
| Full Rust closure gate | PASS | `cargo nextest run --workspace --profile full` passed: `1865` tests passed, `1` skipped, `2` slow, elapsed `670.281s`. This parallel full-suite pass also exercised the release-sidecar atomic-write fix with no JSON sidecar parse failures. |
| Dependency policy | PASS | `cargo deny check` passed: advisories, bans, licenses, and sources OK. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh` passed. `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` passed: `2` tests. |
