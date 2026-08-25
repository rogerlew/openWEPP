# Terminal V5 generated canonical wire

Authority: `Terminal diagnostic correlation V5 checked-schema authority`. Framing: exact live `framed_sha256` primitive.

## `rejected_prefix_v5` domain `terminal-rejected-prefix-v5`

0. `schema` <- `constant 5`; type `u16`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `diagnostic` / `terminal_v5_capture`; private `false`.
1. `prefix_identity` <- `derived prefix identity`; type `PrefixIdentityV5`; mode `explicit-adapter`; nested `prefix_identity_v5`; order `none`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.
2. `beginning_snapshots` <- `explicit owner snapshot set before trial prefix`; type `SnapshotSetV5`; mode `explicit-adapter`; nested `snapshot_set_v5`; order `declared owner order`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.
3. `admissions` <- `ordered admission records`; type `Vec<TrialAdmissionV5>`; mode `explicit-adapter`; nested `trial_admission_v5`; order `admission ordinal ascending`; variants `none`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
4. `carrier_arena` <- `immutable carrier evidence arena`; type `Vec<CarrierPhaseV5>`; mode `explicit-adapter`; nested `carrier_phase_v5`; order `arena index ascending`; variants `none`; owner `provider-carrier` / `v11_covered::carrier_phase`; private `true`.
5. `iterations` <- `coupling evaluations`; type `Vec<CouplingIterationV5>`; mode `explicit-adapter`; nested `coupling_iteration_v5`; order `attempt then coupling ordinal`; variants `none`; owner `coupling-evaluation` / `stage3_solver::evaluation`; private `true`.
6. `selections` <- `coupling selections`; type `Vec<CouplingSelectionV5>`; mode `explicit-adapter`; nested `coupling_selection_v5`; order `attempt ordinal`; variants `none`; owner `coupling-selection` / `stage3_solver`; private `true`.
7. `pair_decisions` <- `adaptive pair decisions`; type `Vec<PairDecisionV5>`; mode `explicit-adapter`; nested `pair_decision_v5`; order `pair ordinal`; variants `ACCEPT=0,REJECT_RETRY=1`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
8. `outcome` <- `match Err(DirectSnowStage3EvaluationError::TerminalNumerics(SnowTerminalNumericsFailure::BelowCarrierDomain))`; type `BelowCarrierOutcomeV5`; mode `explicit-projection`; nested `below_carrier_outcome_v5`; order `none`; variants `TerminalNumerics=2,BelowCarrierDomain=2`; owner `terminal-solver` / `hydrology::02_guard_errors`; private `false`.
9. `ending_snapshots` <- `same explicit owner snapshot set after retained result`; type `SnapshotSetV5`; mode `explicit-adapter`; nested `snapshot_set_v5`; order `declared owner order`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.
10. `unchanged` <- `bitwise comparison of beginning and ending snapshot wires`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `diagnostic` / `terminal_v5_capture`; private `false`.

## `prefix_identity_v5` domain `terminal-prefix-identity-v5`

0. `parent` <- `parent transaction digest`; type `[u8;32]`; mode `primitive`; nested `none`; order `array index`; variants `none`; owner `identity` / `snow_stage3_v11_terminal_execution`; private `true`.
1. `prefix` <- `rejected prefix digest`; type `[u8;32]`; mode `derived-expression`; nested `none`; order `array index`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.
2. `lane_id` <- `terminal lane id`; type `u32`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.

## `snapshot_set_v5` domain `terminal-snapshot-set-v5`

0. `owner_set` <- `canonical seven-owner bytes`; type `BTreeMap<String,Vec<u8>>`; mode `explicit-adapter`; nested `none`; order `ascending owner id; exactly vegetation,snow,land_surface_energy,hydrology,bgc,soil_thermal,surface_liquid`; variants `none`; owner `owner-set` / `snow_stage3_v11_terminal_execution`; private `true`.
1. `joint` <- `CoveredTerminalJointTrialStateV1 exact adapter`; type `CoveredTerminalJointTrialStateV1`; mode `explicit-adapter`; nested `none`; order `authority fields then ascending owner bytes then receipt`; variants `none`; owner `terminal-solver` / `hydrology::runoff_reconciliation`; private `true`.
2. `clock` <- `named terminal clock snapshot`; type `u128`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `chronology` / `snow_stage3_v11_terminal_execution`; private `true`.
3. `provider_cursor_and_calls` <- `named provider cursor plus ordered call log`; type `(u64,Vec<ProviderCallV5>)`; mode `explicit-projection`; nested `none`; order `call ordinal`; variants `none`; owner `provider` / `snow_stage3_v11_terminal_execution`; private `true`.
4. `receipts` <- `named receipt collections`; type `Vec<NamedBytesV5>`; mode `explicit-projection`; nested `none`; order `explicit schema name order`; variants `none`; owner `receipt-owners` / `v11_covered::receipt_sets`; private `true`.
5. `parcels` <- `named precipitation and terminal parcel collections`; type `Vec<NamedBytesV5>`; mode `explicit-projection`; nested `none`; order `explicit schema name order`; variants `none`; owner `provider-carrier` / `snow_stage3_v11_precipitation`; private `true`.
6. `execution_cursor` <- `named execution cursor`; type `u64`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `snow_stage3_v11_terminal_execution`; private `true`.
7. `named_noninterference_fields` <- `explicit list in owner access plan; no lexical discovery`; type `Vec<NamedBytesV5>`; mode `explicit-projection`; nested `none`; order `schema-declared name order`; variants `none`; owner `multiple` / `owner access plan`; private `true`.

## `trial_admission_v5` domain `terminal-trial-admission-v5`

0. `ordinal` <- `admission ordinal`; type `u32`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
1. `support_start` <- `proposed support.start`; type `u128`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
2. `support_end` <- `proposed support.end`; type `u128`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
3. `proposed_duration_bits` <- `proposed_duration.to_bits()`; type `u64`; mode `derived-expression`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
4. `proposed_duration_finite` <- `proposed_duration.is_finite()`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
5. `decision` <- `admission decision`; type `u16`; mode `explicit-projection`; nested `none`; order `none`; variants `ADMIT=0,BELOW_CARRIER_DOMAIN=1,DOMAIN_OR_NONFINITE=2`; owner `terminal-solver` / `stage3_solver::support`; private `true`.
6. `below_carrier_outcome` <- `optional exact outcome witness`; type `Option<BelowCarrierOutcomeV5>`; mode `explicit-projection`; nested `below_carrier_outcome_v5`; order `presence then payload`; variants `absent=0,present=1`; owner `terminal-solver` / `hydrology::02_guard_errors`; private `false`.
7. `provider_calls_before` <- `provider call counter before admission`; type `u64`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `provider` / `snow_stage3_v11_terminal_execution`; private `true`.
8. `provider_calls_after` <- `provider call counter after admission`; type `u64`; mode `primitive`; nested `none`; order `none`; variants `none`; owner `provider` / `snow_stage3_v11_terminal_execution`; private `true`.

## `below_carrier_outcome_v5` domain `terminal-below-carrier-outcome-v5`

0. `outer_variant` <- `DirectSnowStage3EvaluationError::TerminalNumerics`; type `u16`; mode `explicit-projection`; nested `none`; order `none`; variants `TerminalNumerics=2`; owner `terminal-solver` / `hydrology::02_guard_errors`; private `false`.
1. `inner_variant` <- `SnowTerminalNumericsFailure::BelowCarrierDomain`; type `u16`; mode `explicit-projection`; nested `none`; order `none`; variants `BelowCarrierDomain=2`; owner `terminal-solver` / `hydrology::02_guard_errors`; private `false`.

## `carrier_phase_v5` domain `terminal-carrier-phase-v5`

0. `key` <- `fixed carrier key fields: prefix,support,role,attempt,coupling,joint digests,provider call,arena index`; type `CarrierKeyV5`; mode `explicit-projection`; nested `none`; order `listed field order`; variants `FULL=0,HALF_1=1,HALF_2=2,RETRY=3,BRACKET_LOWER=4,BRACKET_UPPER=5,ROOT=6`; owner `provider-carrier` / `stage3_solver::evaluation`; private `true`.
1. `request` <- `CoveredTerminalTrialRequestV1 exact fields`; type `CoveredTerminalTrialRequestV1`; mode `explicit-adapter`; nested `none`; order `declaration order; optional hint presence; joint map ascending`; variants `role tags as key`; owner `provider-carrier` / `hydrology::runoff_reconciliation`; private `true`.
2. `child` <- `CoveredProbeChildIdentityV1 exact fields`; type `CoveredProbeChildIdentityV1`; mode `explicit-adapter`; nested `none`; order `declaration order`; variants `role tags as key`; owner `provider-carrier` / `hydrology::runoff_reconciliation`; private `true`.
3. `projection` <- `TerminalCarrierPhaseProjectionV5 from CoveredCarrierPhaseResultV1`; type `TerminalCarrierPhaseProjectionV5`; mode `explicit-projection`; nested `none`; order `carrier_projection field disposition order`; variants `none`; owner `provider-carrier` / `v11_covered::carrier_phase`; private `true`.
4. `terminal_parcel_absent` <- `terminal parcel collection is empty`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `diagnostic` / `terminal_v5_capture`; private `false`.
5. `hydrology_ingress_bits` <- `terminal liquid hydrology ingress.to_bits()`; type `u64`; mode `derived-expression`; nested `none`; order `none`; variants `none`; owner `diagnostic` / `terminal_v5_capture`; private `false`.
6. `hydrology_ingress_finite` <- `terminal liquid hydrology ingress.is_finite()`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `diagnostic` / `terminal_v5_capture`; private `false`.

## `coupling_iteration_v5` domain `terminal-coupling-iteration-v5`

0. `identity` <- `prefix,support,live role,attempt,coupling,carrier key`; type `CouplingIterationIdentityV5`; mode `explicit-projection`; nested `none`; order `listed field order`; variants `live role tags`; owner `coupling-evaluation` / `stage3_solver::evaluation`; private `true`.
1. `flux` <- `TerminalFluxIntegral all nine fields`; type `TerminalFluxIntegral`; mode `explicit-adapter`; nested `none`; order `declaration order; each f64 bits then finite byte`; variants `none`; owner `coupling-evaluation` / `stage3_solver::terminal_event`; private `true`.
2. `preview` <- `TerminalState all three fields`; type `TerminalState`; mode `explicit-adapter`; nested `none`; order `declaration order; each f64 bits then finite byte`; variants `none`; owner `coupling-evaluation` / `stage3_solver::terminal_event`; private `true`.
3. `incoming_hint` <- `optional incoming CoveredTerminalEndingSnowHintV1 all four fields`; type `Option<CoveredTerminalEndingSnowHintV1>`; mode `explicit-adapter`; nested `none`; order `presence then declaration order`; variants `absent=0,present=1`; owner `coupling-evaluation` / `hydrology::runoff_reconciliation`; private `true`.
4. `outgoing_hint` <- `outgoing CoveredTerminalEndingSnowHintV1 all four fields`; type `CoveredTerminalEndingSnowHintV1`; mode `explicit-adapter`; nested `none`; order `declaration order`; variants `none`; owner `coupling-evaluation` / `hydrology::runoff_reconciliation`; private `true`.
5. `component_checks` <- `ice,liquid,cold_content,surface_temperature delta/tolerance/absolute_delta/within`; type `[CouplingCheckV5;4]`; mode `derived-expression`; nested `none`; order `ice,liquid,cold_content,surface_temperature`; variants `none`; owner `coupling-evaluation` / `stage3_solver::evaluation`; private `true`.
6. `combined_converged` <- `combined convergence predicate`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `coupling-evaluation` / `stage3_solver::evaluation`; private `true`.

## `coupling_selection_v5` domain `terminal-coupling-selection-v5`

0. `identity` <- `prefix,support,live role,attempt`; type `CouplingSelectionIdentityV5`; mode `explicit-projection`; nested `none`; order `listed field order`; variants `live role tags`; owner `coupling-selection` / `stage3_solver`; private `true`.
1. `iteration_keys` <- `evaluated iteration keys`; type `Vec<CouplingIterationKeyV5>`; mode `explicit-projection`; nested `none`; order `coupling ordinal ascending`; variants `none`; owner `coupling-selection` / `stage3_solver`; private `true`.
2. `selected_keys` <- `selected iteration and carrier keys`; type `(CouplingIterationKeyV5,CarrierKeyV5)`; mode `explicit-projection`; nested `none`; order `tuple order`; variants `none`; owner `coupling-selection` / `stage3_solver`; private `true`.
3. `returned_digests` <- `returned flux,preview,carrier-joint digests`; type `[[u8;32];3]`; mode `derived-expression`; nested `none`; order `flux,preview,carrier-joint`; variants `none`; owner `coupling-selection` / `stage3_solver`; private `true`.
4. `selected_converged` <- `selected convergence`; type `bool`; mode `derived-expression`; nested `none`; order `none`; variants `false=0,true=1`; owner `coupling-selection` / `stage3_solver`; private `true`.

## `pair_component_error_v5` domain `terminal-pair-component-error-v5`

0. `component` <- `canonical component index`; type `u16`; mode `primitive`; nested `none`; order `none`; variants `ice=0,liquid=1,cold=2,complete_energy=3,unallocated_energy=4`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
1. `values` <- `coarse,refined,delta,abs_tol,rel_tol,denominator,scaled each as bits+finite`; type `[DiagnosticF64V5;7]`; mode `derived-expression`; nested `none`; order `coarse,refined,delta,abs_tol,rel_tol,denominator,scaled`; variants `none`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.

## `pair_decision_v5` domain `terminal-pair-decision-v5`

0. `identity` <- `prefix,pair ordinal and coarse/fine trial digests`; type `PairIdentityV5`; mode `explicit-projection`; nested `none`; order `prefix,ordinal,coarse,fine1,fine2`; variants `COARSE=0,FINE_1=1,FINE_2=2`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
1. `refined_digests` <- `refined state and ledger digests`; type `[[u8;32];2]`; mode `derived-expression`; nested `none`; order `state,ledger`; variants `none`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
2. `component_errors` <- `five exact component errors`; type `[PairComponentErrorV5;5]`; mode `explicit-adapter`; nested `pair_component_error_v5`; order `ice,liquid,cold,complete_energy,unallocated_energy`; variants `component tags`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
3. `maximum_scaled` <- `exact left fold over scaled binary64 values`; type `DiagnosticF64V5`; mode `derived-expression`; nested `none`; order `none`; variants `none`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
4. `winner` <- `first bitwise-equal canonical component`; type `u16`; mode `derived-expression`; nested `none`; order `none`; variants `component tags`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
5. `decision` <- `pair decision`; type `u16`; mode `explicit-projection`; nested `none`; order `none`; variants `ACCEPT=0,REJECT_RETRY=1`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
6. `current_duration` <- `current duration bits+finite`; type `DiagnosticF64V5`; mode `derived-expression`; nested `none`; order `bits then finite`; variants `none`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
7. `proposed_next` <- `optional proposed next bits+finite`; type `Option<DiagnosticF64V5>`; mode `derived-expression`; nested `none`; order `presence,bits,finite`; variants `absent=0,present=1`; owner `terminal-solver` / `stage3_solver::terminal_event`; private `true`.
