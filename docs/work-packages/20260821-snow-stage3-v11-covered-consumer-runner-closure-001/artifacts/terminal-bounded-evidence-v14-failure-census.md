# V14 exact affected-failure census

Current HEAD run: nextest `875ee602-1e45-4a2a-8010-0387f238f6cc`, independently
reproduced with full failure output at `521986ae` in
`/tmp/openwepp-v13-affected.log`: 844 passed, 11 failed, 1 skipped.

Pre-seam comparison: detached `2d34d1082ae0d6e324a33a2e2b9159f99f5f8ace`
run `acd21032-6069-4d34-aa3f-5e307a99d0d0`: 842 passed, the identical 11
failed, 1 skipped. The two additional V12 tests pass at current HEAD. Every
failure name and error signature below predates `ce58080c`; none is an
evidence-seam regression.

All tests are in the `openwepp-hillslope-orchestrator` lib-test binary.

| Fully qualified test suffix | First relevant failure | Before `ce58080c` | Classification | Required action |
|---|---|---:|---|---|
| `persistent_tests::persistent_support_evaluator_runs_one_admitted_parent_support` | expected actual terminal solver; got `TerminalNumerics(BelowCarrierDomain)` | yes, identical | expected terminal-success obligation | retain until carrier floor is authorized |
| `persistent_tests::terminal_event_request_is_state_bound_and_censors_remaining_time` | unwrap of `TerminalNumerics(BelowCarrierDomain)` | yes, identical | expected terminal-success obligation | retain until carrier floor is authorized |
| `persistent_tests::terminal_no_event_refreeze_closes_persistent_day` | unwrap of `TerminalNumerics(BelowCarrierDomain)` | yes, identical | expected terminal-success obligation | retain until carrier floor is authorized |
| `v9_real_consumer_shadow::tests::interior_terminal_event_runs_covered_event_and_snow_free_remainder` | `Stage3(TerminalNumerics(BelowCarrierDomain))` | yes, identical | expected terminal-success obligation | do not relabel; later event authority must satisfy |
| `v9_real_consumer_shadow::tests::coupled_hard_boundary_truncates_selected_900_second_child` | `snow.stage3_terminal_persistent_identity_or_model`, value 1 outside exact 0 | yes, identical | stale structural/source guard | resolve under later persistent-model authority, not V14 |
| `v9_real_consumer_shadow::tests::latest_accepted_stage3_state_changes_next_wb14_proposal` | same persistent identity/model error | yes, identical | stale structural/source guard | same |
| `v9_real_consumer_shadow::tests::one_1800_second_child_matches_complete_historical_candidate` | same persistent identity/model error | yes, identical | stale structural/source guard | same |
| `v9_real_consumer_shadow::tests::two_900_second_complete_owner_children_publish_one_parent` | same persistent identity/model error | yes, identical | stale structural/source guard | same |
| `v9_real_consumer_shadow::tests::resolved_snow_and_snow_free_lanes_publish_one_atomic_parent` | `Executor(Identity("covered carrier lane/OFE set"))` | yes, identical | stale structural/source guard | later mixed-lane identity authority |
| `v9_real_consumer_shadow::tests::two_resolved_snow_lanes_choose_common_earliest_cadence` | same lane/OFE-set identity error | yes, identical | stale structural/source guard | later mixed-lane identity authority |
| `v9_real_consumer_shadow::tests::v10_midnight_failure_rolls_back_every_shadow_owner_exactly` | expected legacy `Unsupported`; actual variant differs | yes, identical | stale structural/source guard | update only under separate V10 error-contract authority |

`CHILD1-TERM-EVIDENCE-016`: the prior count-only baseline could not prove
regression nonintroduction. This census closes that uncertainty: the complete
before/after failure sets and signatures are identical.
