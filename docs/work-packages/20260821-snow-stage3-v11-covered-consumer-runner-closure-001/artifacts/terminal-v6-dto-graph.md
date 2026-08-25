# Terminal V6 closed purpose-built DTO graph

Authority: `Terminal diagnostic correlation V6 compiler-indexed projection authority`. Root: `RejectedPrefixEvidenceV6`. Reachable DTO nodes: 20; declared DTO nodes: 20. No live carrier type is embedded wholesale.

## `Digest32V6`

0. `bytes`: `bytes`; nested `primitive`; order `array index 0..31`.

## `DiagnosticF64V6`

0. `bits`: `u64`; nested `primitive`; order `scalar`.
1. `semantic_finite`: `bool`; nested `primitive`; order `scalar`.

## `TimeSupportV6`

0. `start_ns`: `u128`; nested `primitive`; order `scalar`.
1. `end_ns`: `u128`; nested `primitive`; order `scalar`.

## `PrefixIdentityV6`

0. `parent`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
2. `lane_id`: `u32`; nested `primitive`; order `scalar`.

## `PairPositionV6`

0. `tag`: `u16`; nested `primitive`; order `COARSE=0,FINE_1=1,FINE_2=2`.

## `LiveProviderRoleV6`

0. `tag`: `u16`; nested `primitive`; order `FULL=0,HALF_1=1,HALF_2=2,RETRY=3,BRACKET_LOWER=4,BRACKET_UPPER=5,ROOT=6`.

## `CarrierPhaseKeyV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `support`: `TimeSupportV6`; nested `TimeSupportV6`; order `scalar`.
2. `role`: `LiveProviderRoleV6`; nested `LiveProviderRoleV6`; order `scalar`.
3. `attempt`: `u32`; nested `primitive`; order `scalar`.
4. `coupling`: `u32`; nested `primitive`; order `scalar`.
5. `beginning_joint`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
6. `carrier_ending_joint`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
7. `provider_call`: `u64`; nested `primitive`; order `scalar`.
8. `arena_index`: `u64`; nested `primitive`; order `scalar`.

## `ZeroTerminalIngressEvidenceV6`

0. `hydrology_terminal_liquid_supply`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
1. `wb14_terminal_liquid_credit`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
2. `surface_liquid_terminal_ingress`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.

## `CarrierPhaseEvidenceV6`

0. `key`: `CarrierPhaseKeyV6`; nested `CarrierPhaseKeyV6`; order `scalar`.
1. `request_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
2. `child_identity_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
3. `ending_joint_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
4. `trial_snow_soil_receipt_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
5. `precipitation_set_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
6. `carrier_envelope_transaction`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
7. `lower_boundary_set_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
8. `carrier_source_receipt_set_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
9. `covered_lse_state_set_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
10. `soil_candidate_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
11. `soil_top_boundary_credit_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
12. `wb14_child_receipt_set_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
13. `wb14_child_replay_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
14. `zero_terminal_ingress`: `ZeroTerminalIngressEvidenceV6`; nested `ZeroTerminalIngressEvidenceV6`; order `scalar`.

## `TerminalStateV6`

0. `ice_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
1. `liquid_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
2. `cold_content_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.

## `TerminalLedgerV6`

0. `complete_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
1. `cold_energy_change_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
2. `refrozen_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
3. `deposition_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
4. `sublimation_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
5. `melt_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
6. `unallocated_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
7. `shortwave_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
8. `longwave_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
9. `sensible_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
10. `latent_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
11. `advected_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
12. `snow_soil_heat_energy_j_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
13. `external_liquid_kg_m2`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.

## `CouplingIterationEvidenceV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `support`: `TimeSupportV6`; nested `TimeSupportV6`; order `scalar`.
2. `role`: `LiveProviderRoleV6`; nested `LiveProviderRoleV6`; order `scalar`.
3. `attempt`: `u32`; nested `primitive`; order `scalar`.
4. `coupling`: `u32`; nested `primitive`; order `scalar`.
5. `carrier_key`: `CarrierPhaseKeyV6`; nested `CarrierPhaseKeyV6`; order `scalar`.
6. `flux_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
7. `preview`: `TerminalStateV6`; nested `TerminalStateV6`; order `scalar`.
8. `incoming_hint_present`: `bool`; nested `primitive`; order `scalar`.
9. `incoming_hint_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
10. `outgoing_hint_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
11. `component_checks_digest`: `Digest32V6`; nested `Digest32V6`; order `ordered ice,liquid,cold_content,surface_temperature`.
12. `combined_converged`: `bool`; nested `primitive`; order `scalar`.

## `CouplingSelectionEvidenceV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `support`: `TimeSupportV6`; nested `TimeSupportV6`; order `scalar`.
2. `role`: `LiveProviderRoleV6`; nested `LiveProviderRoleV6`; order `scalar`.
3. `attempt`: `u32`; nested `primitive`; order `scalar`.
4. `iteration_count`: `u32`; nested `primitive`; order `scalar`.
5. `ordered_iteration_keys`: `CarrierPhaseKeyV6`; nested `CarrierPhaseKeyV6`; order `coupling ordinal ascending sequence`.
6. `selected_iteration_key`: `CarrierPhaseKeyV6`; nested `CarrierPhaseKeyV6`; order `scalar`.
7. `selected_carrier_key`: `CarrierPhaseKeyV6`; nested `CarrierPhaseKeyV6`; order `scalar`.
8. `selected_coupling`: `u32`; nested `primitive`; order `scalar`.
9. `returned_flux_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
10. `returned_preview_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
11. `returned_carrier_joint_digest`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
12. `selected_converged`: `bool`; nested `primitive`; order `scalar`.

## `SelectedTerminalTrialEvidenceV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `pair_position`: `PairPositionV6`; nested `PairPositionV6`; order `scalar`.
2. `role`: `LiveProviderRoleV6`; nested `LiveProviderRoleV6`; order `scalar`.
3. `attempt`: `u32`; nested `primitive`; order `scalar`.
4. `support`: `TimeSupportV6`; nested `TimeSupportV6`; order `scalar`.
5. `beginning_state`: `TerminalStateV6`; nested `TerminalStateV6`; order `scalar`.
6. `ending_state`: `TerminalStateV6`; nested `TerminalStateV6`; order `scalar`.
7. `ledger`: `TerminalLedgerV6`; nested `TerminalLedgerV6`; order `scalar`.
8. `selection`: `CouplingSelectionEvidenceV6`; nested `CouplingSelectionEvidenceV6`; order `scalar`.
9. `hydrology_complete_ending_joint`: `Digest32V6`; nested `Digest32V6`; order `scalar`.

## `PairComponentErrorV6`

0. `component`: `u16`; nested `primitive`; order `ice=0,liquid=1,cold=2,complete_energy=3,unallocated_energy=4`.
1. `coarse`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
2. `refined`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
3. `delta`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
4. `absolute_tolerance`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
5. `relative_tolerance`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
6. `denominator`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
7. `scaled`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.

## `PairDecisionV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `pair_ordinal`: `u32`; nested `primitive`; order `scalar`.
2. `coarse`: `SelectedTerminalTrialEvidenceV6`; nested `SelectedTerminalTrialEvidenceV6`; order `role FULL or RETRY; position COARSE`.
3. `fine_1`: `SelectedTerminalTrialEvidenceV6`; nested `SelectedTerminalTrialEvidenceV6`; order `role HALF_1; position FINE_1`.
4. `fine_2`: `SelectedTerminalTrialEvidenceV6`; nested `SelectedTerminalTrialEvidenceV6`; order `role HALF_2; position FINE_2`.
5. `component_errors`: `PairComponentErrorV6`; nested `PairComponentErrorV6`; order `exact five-component order`.
6. `maximum_scaled`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `exact left fold`.
7. `diagnostic_winner`: `u16`; nested `primitive`; order `first bitwise-equal component`.
8. `decision`: `u16`; nested `primitive`; order `ACCEPT=0,REJECT_RETRY=1`.
9. `current_duration`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
10. `proposed_next_present`: `bool`; nested `primitive`; order `scalar`.
11. `proposed_next`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.

## `BelowCarrierDomainOutcomeV6`

0. `outer_variant`: `u16`; nested `primitive`; order `DirectSnowStage3EvaluationError::TerminalNumerics=2`.
1. `inner_variant`: `u16`; nested `primitive`; order `SnowTerminalNumericsFailure::BelowCarrierDomain=2`.

## `TrialAdmissionV6`

0. `prefix`: `Digest32V6`; nested `Digest32V6`; order `scalar`.
1. `ordinal`: `u32`; nested `primitive`; order `scalar`.
2. `proposed_support`: `TimeSupportV6`; nested `TimeSupportV6`; order `scalar`.
3. `proposed_duration`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
4. `required_half_duration`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `scalar`.
5. `minimum_carrier_duration`: `DiagnosticF64V6`; nested `DiagnosticF64V6`; order `exact 0.6000000000000000 seconds`.
6. `decision`: `u16`; nested `primitive`; order `ADMIT=0,BELOW_CARRIER_DOMAIN=1,DOMAIN_OR_NONFINITE=2`.
7. `outcome_present`: `bool`; nested `primitive`; order `scalar`.
8. `outcome`: `BelowCarrierDomainOutcomeV6`; nested `BelowCarrierDomainOutcomeV6`; order `scalar`.
9. `provider_calls_before`: `u64`; nested `primitive`; order `scalar`.
10. `provider_calls_after`: `u64`; nested `primitive`; order `scalar`.

## `NoninterferenceSnapshotV6`

0. `beginning`: `Digest32V6`; nested `Digest32V6`; order `canonical owner-set bytes`.
1. `stage3_beginning_by_lane`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
2. `ending`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
3. `ending_stage3_by_lane`: `Digest32V6`; nested `Digest32V6`; order `ascending lane option framing`.
4. `last_support_receipt`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
5. `last_final_boundary_receipts`: `Digest32V6`; nested `Digest32V6`; order `ascending destination`.
6. `last_lane_boundary_receipts`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
7. `last_component_carrier_receipts`: `Digest32V6`; nested `Digest32V6`; order `ascending destination`.
8. `last_snow_soil_heat_receipts`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
9. `last_terminal_snow_soil_heat_receipts`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
10. `last_precipitation_parcel_sets`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
11. `last_physical_outcome_ledgers`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
12. `last_terminal_events`: `Digest32V6`; nested `Digest32V6`; order `ascending lane`.
13. `pending_terminal_parcels`: `Digest32V6`; nested `Digest32V6`; order `ascending digest`.
14. `precomputed_terminal_accepted`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
15. `last_wb14_child_receipt_set_sha256`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
16. `last_wb14_parent_receipt_set_sha256`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
17. `last_wb14_child_replay_bytes`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
18. `last_wb14_parent_replay_bytes`: `Digest32V6`; nested `Digest32V6`; order `scalar option framing`.
19. `terminal_endpoint_mode`: `bool`; nested `primitive`; order `scalar`.
20. `day_index`: `u64`; nested `primitive`; order `checked usize-to-u64 conversion`.
21. `interval_index`: `u64`; nested `primitive`; order `checked usize-to-u64 conversion`.
22. `finalize_wb14_parent_interval`: `bool`; nested `primitive`; order `scalar`.
23. `initial_joint`: `Digest32V6`; nested `Digest32V6`; order `caller-local exact joint identity`.
24. `clock`: `Digest32V6`; nested `Digest32V6`; order `caller-local canonical clock bytes`.
25. `provider_call_count`: `u64`; nested `primitive`; order `caller-local exact counter`.

## `RejectedPrefixEvidenceV6`

0. `schema`: `u16`; nested `primitive`; order `constant 6`.
1. `identity`: `PrefixIdentityV6`; nested `PrefixIdentityV6`; order `scalar`.
2. `before`: `NoninterferenceSnapshotV6`; nested `NoninterferenceSnapshotV6`; order `scalar`.
3. `admissions`: `TrialAdmissionV6`; nested `TrialAdmissionV6`; order `admission ordinal ascending; final member zero-call BelowCarrierDomain`.
4. `carrier_arena`: `CarrierPhaseEvidenceV6`; nested `CarrierPhaseEvidenceV6`; order `arena index ascending immutable`.
5. `iterations`: `CouplingIterationEvidenceV6`; nested `CouplingIterationEvidenceV6`; order `attempt then coupling ordinal`.
6. `selections`: `CouplingSelectionEvidenceV6`; nested `CouplingSelectionEvidenceV6`; order `attempt ordinal`.
7. `selected_trials`: `SelectedTerminalTrialEvidenceV6`; nested `SelectedTerminalTrialEvidenceV6`; order `trial ordinal`.
8. `pair_decisions`: `PairDecisionV6`; nested `PairDecisionV6`; order `pair ordinal`.
9. `outcome`: `BelowCarrierDomainOutcomeV6`; nested `BelowCarrierDomainOutcomeV6`; order `scalar`.
10. `after`: `NoninterferenceSnapshotV6`; nested `NoninterferenceSnapshotV6`; order `scalar`.
11. `unchanged`: `bool`; nested `primitive`; order `exact before/after DTO bytes equality`.
