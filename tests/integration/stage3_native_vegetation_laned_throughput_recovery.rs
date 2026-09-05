use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_sources(path: &Path, out: &mut Vec<PathBuf>) {
    for entry in
        fs::read_dir(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
    {
        let entry = entry.expect("read directory entry");
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

fn explicitly_nonproduction_historical_test(relative: &str) -> bool {
    const ALLOWLIST: &[&str] = &[
        "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_superseded_historical_solver_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v44_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v45_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v46_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v51_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v52_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v53_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v54_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v55_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v56_tests.rs",
    ];
    ALLOWLIST.contains(&relative)
}

#[test]
fn validated_handoff_static_production_seams_are_required() {
    let repository = root();
    let frozen = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v3_adoption.rs",
    ))
    .expect("read frozen-litter V3 adoption");
    let frozen_v4 = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v4_adoption.rs",
    ))
    .expect("read frozen-litter V4 adoption");
    let outer_transition = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization_v50_transition.rs",
    ))
    .expect("read covered outer-owner transition");
    let surface = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_adapter.rs",
    ))
    .expect("read surface-liquid V2 ingress adapter");
    let vegetation = fs::read_to_string(repository.join("crates/openwepp-vegetation/src/v11.rs"))
        .expect("read V11 vegetation executor");

    assert!(
        frozen.contains("ValidatedFrozenLitterV3ResidentHandoffV1"),
        "expected-red: frozen-litter install lacks the private validated resident handoff"
    );
    let install = frozen
        .split("fn install_frozen_litter_v3_resident")
        .nth(1)
        .expect("frozen-litter install function");
    assert!(
        !install.contains("accepted_publication_supports_canonical_bytes")
            && !install.contains("restore_accepted_publication_supports_canonical_bytes"),
        "expected-red: unchanged resident install still serializes and restores its retained history"
    );
    assert!(
        frozen.contains("restore_restart_authority"),
        "restart must retain complete semantic history restoration"
    );
    assert!(
        frozen_v4.contains("install_validated_frozen_litter_v4_residents"),
        "trusted covered outer transition requires the private validated V3/V4 pair installer"
    );
    let outer_transition_body = outer_transition
        .split("fn authenticate_v50_covered_v8_outer_owner_transition_v1")
        .nth(1)
        .expect("covered outer-owner transition body");
    assert!(
        outer_transition_body.contains("install_validated_frozen_litter_v4_residents")
            && !outer_transition_body.contains("install_restored_frozen_litter_v4_residents"),
        "trusted in-process charged candidates must not enter the restart replay installer"
    );
    assert!(
        frozen_v4.contains("complete_owner_projection_canonical_bytes.clone()"),
        "V4 install must consume the private candidate's already-replayed canonical projection"
    );
    let v4_accept = frozen_v4
        .split("fn accept_runtime_candidate")
        .nth(1)
        .expect("V4 accepted-candidate install")
        .split("#[cfg(test)]")
        .next()
        .expect("V4 accepted-candidate body");
    assert!(
        !v4_accept.contains("SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes")
            && !v4_accept.contains(".canonical_bytes(physical.surface_configuration())"),
        "already-replayed V4 candidates must not be serialized and parsed again at install"
    );

    assert!(
        surface.contains("ValidatedSurfaceLiquidResourceCandidateV2"),
        "expected-red: surface ingress lacks a private revision-bound validated resource candidate"
    );
    assert!(
        surface.contains("consume_validated_surface_liquid_resource_candidate_v2"),
        "expected-red: trusted ingress does not consume the validated candidate typestate"
    );

    assert!(
        vegetation.contains("ValidatedV10SegmentEndingV1"),
        "expected-red: vegetation parent finalization lacks a private validated ending-state handoff"
    );
    assert!(
        vegetation.contains("output.ending.validate(&input.configuration)"),
        "untrusted V11 executor output must retain independent full validation"
    );
}

#[test]
fn validated_handoff_runtime_proof_groups_are_required() {
    let repository = root();
    let frozen_tests = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v3_publication_retention_tests.rs",
    ))
    .expect("read frozen-litter publication retention tests");
    let surface_tests = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_tests.rs",
    ))
    .expect("read surface-liquid V2 ingress tests");
    let chronology_tests = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_parent_chronology_tests.rs",
    ))
    .expect("read V11 parent chronology tests");

    for proof in [
        "install_frozen_litter_v3_resident_moves_validated_history_without_roundtrip",
        "validated_resident_append_checks_only_new_tail_and_preserves_prefix",
        "frozen_litter_restart_revalidates_complete_publication_history",
        "validated_resident_tail_poison_rolls_back",
        "validated_resident_install_cost_is_history_length_independent",
    ] {
        assert!(
            frozen_tests.contains(proof),
            "expected-red: missing frozen-litter validated-handoff runtime proof {proof}"
        );
    }
    for proof in [
        "surface_resource_candidate_validates_each_revision_once",
        "validated_surface_ingress_avoids_owner_reserialization",
        "surface_resource_mutation_invalidates_validation_proof",
        "surface_resource_wrong_configuration_or_nested_owner_rejects",
        "validated_surface_resource_output_and_rollback_are_unchanged",
    ] {
        assert!(
            surface_tests.contains(proof),
            "expected-red: missing surface-resource validated-handoff runtime proof {proof}"
        );
    }
    for proof in [
        "parent_finalization_reuses_one_validated_vegetation_image",
        "untrusted_v11_executor_ending_is_independently_validated_once",
        "lineage_mutation_requires_new_digest_and_validation",
        "vegetation_restart_reparses_and_revalidates_complete_state",
        "validated_vegetation_proof_is_not_transferable",
    ] {
        assert!(
            chronology_tests.contains(proof),
            "expected-red: missing vegetation validated-handoff runtime proof {proof}"
        );
    }
}

#[test]
fn coupled_slab_acceptance_consumes_one_private_revision_proof() {
    let repository = root();
    let contract = fs::read_to_string(
        repository.join("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md"),
    )
    .expect("read coupled-time contract");
    assert!(contract.contains("contract_version: 16"));
    assert!(contract.contains("INV-COUPLEDTIME-028"));
    assert!(contract.contains("OBL-COUPLEDTIME-011"));

    let transaction =
        fs::read_to_string(repository.join("crates/openwepp-coupled-time/src/transaction.rs"))
            .expect("read coupled-time transaction source");
    let body = transaction
        .split("pub fn accept_slab(")
        .nth(1)
        .expect("slab acceptance")
        .split("#[derive(Debug, Clone, PartialEq, Serialize)]")
        .next()
        .expect("slab acceptance body");
    assert!(body.contains("validation_proof"));
    for forbidden in [
        "CoupledSlabCandidateV1::new",
        "owner_set_digest(",
        "ledger_digest(",
        "serde_json",
        ".clone()",
        ".accepted_slab_receipts\n        .iter()",
    ] {
        assert!(
            !body.contains(forbidden),
            "trusted slab acceptance repeated forbidden work: {forbidden}"
        );
    }
}

#[test]
fn canonical_stage3_production_source_has_no_historical_solver_family() {
    let repository = root();
    let mut sources = Vec::new();
    rust_sources(
        &repository.join("crates/openwepp-hillslope-orchestrator/src"),
        &mut sources,
    );
    rust_sources(&repository.join("crates/openwepp-runner/src"), &mut sources);

    let forbidden = [
        "COVERED_FIXED_POINT_POLICY",
        "CoveredFixedPointPolicy",
        "CoveredConvergenceAdmissionV1",
        "CoveredPhysicalEvaluationBudgetV1",
        "PhaseConsistentCoupled",
        "phase_consistent_coupled_",
        "phase_consistent_parity_",
        "phase_consistent_branch_entry_seen",
        "covered_stable_monotone_",
        "stable_monotone_",
        "covered_frozen_temperature_primary_",
        "FrozenTemperaturePrimary",
        "covered_private_q_lattice_",
        "private_q_lattice",
        "covered_authentic_receipt_stabil",
        "covered_authentic_receipt_cycle",
        "receipt_cycle_endpoint_witness",
        "phase_consistent_coupled_root_polish",
        "root_polish",
        "post_root_transition",
        "pre_root_refusal",
        "exact_authentic_cycle",
        "for iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations",
        "max_iterations: 96",
        "V58",
        "v58",
    ];

    for path in sources {
        let relative = path
            .strip_prefix(&repository)
            .expect("source below repository")
            .to_string_lossy()
            .replace('\\', "/");
        if explicitly_nonproduction_historical_test(&relative) {
            continue;
        }
        let source = fs::read_to_string(&path).expect("read Rust source");
        for symbol in forbidden {
            assert!(
                !source.contains(symbol),
                "superseded solver family `{symbol}` remains outside the exact historical-test allowlist in {relative}",
            );
        }
    }
}

#[test]
fn superseded_solver_module_paths_are_deleted_not_hidden_or_renamed_as_tests() {
    let repository = root();
    for relative in [
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_coupled_solve.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_temperature_primary.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_private_q_lattice.rs",
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/stable_monotone.rs",
    ] {
        assert!(
            !repository.join(relative).exists(),
            "superseded production solver module must be deleted: {relative}",
        );
    }

    for (relative, include) in [
        (
            "crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs",
            "include!(\"phase_consistent_coupled_solve.rs\")",
        ),
        (
            "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs",
            "include!(\"stable_monotone.rs\")",
        ),
    ] {
        let source = fs::read_to_string(repository.join(relative)).expect("read include owner");
        assert!(
            !source.contains(include),
            "superseded module include remains in {relative}: {include}",
        );
    }
}

#[test]
fn exact_surface_receipt_seals_parent_local_partial_vs_final_chronology() {
    let repository = root();
    let exact_owner = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs",
    ))
    .expect("read exact surface-enthalpy owner");
    let v4_runtime = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_execution.rs",
    ))
    .expect("read frozen-litter V4 runtime");
    let combined = format!("{exact_owner}\n{v4_runtime}");
    for required in [
        "LseSurfaceEnthalpyEndingPostureV1",
        "ParentLocalPartial",
        "PersistentParentFinal",
        "parent_support_start_ns",
        "parent_support_end_ns",
        "ending_posture",
    ] {
        assert!(
            combined.contains(required),
            "exact-surface receipt/runtime is missing parent-local chronology token `{required}`",
        );
    }
}

#[test]
fn represented_snow_native_covered_envelope_production_seam_is_required() {
    let repository = root();
    let classification = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_multitile_adoption.rs",
    ))
    .expect("read native V3 multi-tile classification");
    let native_v4 = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v4_adoption.rs",
    ))
    .expect("read native V4 adoption");
    let carrier = fs::read_to_string(
        repository.join("crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_engine.rs"),
    )
    .expect("read covered carrier engine");
    let covered_owner = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs",
    ))
    .expect("read covered owner projection");
    let production = format!("{classification}\n{native_v4}\n{carrier}\n{covered_owner}");

    let mut defects = Vec::new();
    if !classification.contains("Stage3CoveredNative") {
        defects.push("missing typed Stage3CoveredNative tile/regime classification".to_owned());
    }
    for required in [
        "represented_snow_native_column_uses_standard_covered_solver_once",
        "represented_snow_native_column_skips_frozen_litter_v3_v4_and_wb14",
        "represented_snow_native_column_retains_exact_optical_and_lower_boundary_receipts",
        "represented_snow_native_column_does_not_construct_second_inner_envelope",
        "represented_snow_native_column_retains_frozen_litter_v3_v4_bytes",
        "represented_snow_native_column_transitions_to_snow_free_litter_after_terminal_split",
        "represented_snow_native_column_rolls_back_complete_owner_on_failure",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing production behavior proof {required}"));
        }
    }
    for required in [
        "Stage3SnowOpticalBoundaryReceiptV1",
        "Stage3SnowCoveredLowerBoundary",
        "CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing exact Stage3 boundary custody token {required}"
            ));
        }
    }
    if carrier.contains(".map(|(candidate, _native_envelope)| candidate)") {
        defects.push("native charged envelope is still discarded".to_owned());
    }

    assert!(
        defects.is_empty(),
        "represented-snow native covered-envelope seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn unpublished_soil_continuation_enters_v3_only_as_typed_candidate_beginning() {
    let repository = root();
    let producer = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs",
    ))
    .expect("read unpublished soil-thermal continuation producer");
    let producer_continuation = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_unpublished_continuation.rs",
    ))
    .expect("read unpublished soil-thermal continuation include");
    let producer_behavior_proofs = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_v49_tests.rs",
    ))
    .expect("read unpublished soil-thermal continuation behavior proofs");
    let runtime = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_execution.rs",
    ))
    .expect("read frozen-litter V3 runtime");
    let projection = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_projection.rs",
    ))
    .expect("read surface-liquid V3 projection");
    let adoption = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v3_adoption.rs",
    ))
    .expect("read frozen-litter V3 adoption");
    let production = format!(
        "{producer}\n{producer_continuation}\n{producer_behavior_proofs}\n{runtime}\n{projection}\n{adoption}"
    );

    let mut defects = Vec::new();
    for required in [
        "FrozenLitterV3SoilBeginningV1",
        "CandidateOnlyUnpublishedSoil",
        "SoilThermalUnpublishedPhysicalBeginningV2",
        "compose_soil_thermal_accepted_from_unpublished_v2",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing typed candidate-only soil-beginning token {required}"
            ));
        }
    }
    for required in [
        "unpublished_soil_beginning_authenticates_exact_contiguous_child",
        "unpublished_soil_beginning_never_emits_owner_or_restart_bytes",
        "unpublished_soil_beginning_rejects_support_rebind",
        "unpublished_soil_beginning_rejects_predecessor_trial_substitution",
        "unpublished_soil_beginning_final_acceptance_replays_outer_owner_once",
        "unpublished_soil_beginning_rolls_back_without_intermediate_install",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing production behavior proof {required}"));
        }
    }

    assert!(
        defects.is_empty(),
        "typed candidate-only unpublished-soil V3 seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn litter_phase_capacity_spill_is_typed_and_routes_once_through_wb14() {
    let repository = root();
    let production_paths = [
        "crates/openwepp-land-surface-energy/src/litter_phase.rs",
        "crates/openwepp-land-surface-energy/src/litter_phase_output.rs",
        "crates/openwepp-land-surface-energy/src/litter_phase_closure.rs",
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_input_projection.rs",
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_execution.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_adapter.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs",
    ];
    let test_paths = [
        "crates/openwepp-land-surface-energy/src/litter_phase_tests.rs",
        "crates/openwepp-land-surface-energy/src/transaction_v3_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_tests.rs",
    ];
    let production = production_paths
        .iter()
        .map(|relative| {
            fs::read_to_string(repository.join(relative)).expect("read production seam")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let tests = test_paths
        .iter()
        .map(|relative| fs::read_to_string(repository.join(relative)).expect("read focused tests"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut defects = Vec::new();
    for required in [
        "LitterPhaseCapacitySpillV1",
        "LitterPhaseOverflow",
        "LitterPhaseCapacitySpillEnergy",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing typed phase-spill production token {required}"
            ));
        }
    }
    for required in [
        "litter_phase_capacity_boundary_does_not_spill",
        "melt_created_litter_phase_capacity_spill_preserves_mass_and_enthalpy",
        "litter_phase_capacity_spill_routes_once_through_wb14",
        "litter_phase_capacity_spill_rejects_condensation_alias",
        "litter_phase_capacity_spill_rejects_transaction_support_receipt_substitution",
        "litter_phase_capacity_spill_rolls_back_all_owners",
    ] {
        if !tests.contains(required) {
            defects.push(format!("missing focused behavior proof {required}"));
        }
    }
    if production.contains("ending.liquid_kg_m2_tile.min(")
        || production.contains("liquid_kg_m2_tile: ending.liquid_kg_m2_tile.min(")
    {
        defects.push("phase ending still uses capacity clipping/min normalization".to_owned());
    }

    assert!(
        defects.is_empty(),
        "exact litter phase-capacity spill seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn heterogeneous_v3_surface_resource_join_applies_ordinary_finalized_uses_once() {
    let repository = root();
    let production_paths = [
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_adapter.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs",
    ];
    let test_paths = [
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs",
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_tests.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_tests.rs",
    ];
    let production = production_paths
        .iter()
        .map(|relative| {
            fs::read_to_string(repository.join(relative)).expect("read production seam")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let tests = test_paths
        .iter()
        .map(|relative| fs::read_to_string(repository.join(relative)).expect("read focused tests"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut defects = Vec::new();
    for required in [
        "SurfaceLiquidV2HeterogeneousResourceJoinV1",
        "apply_ordinary_finalized_uses_to_phase_adjusted_v2",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing typed heterogeneous-resource production token {required}"
            ));
        }
    }
    for required in [
        "heterogeneous_v3_resource_join_debits_ordinary_finalized_use_once",
        "heterogeneous_v3_resource_join_retains_native_phase_and_spill_custody",
        "heterogeneous_v3_resource_join_rejects_native_vapor_replay_as_ordinary_use",
        "heterogeneous_v3_resource_join_rejects_foreign_or_duplicate_finalized_use",
        "heterogeneous_v3_resource_join_executes_one_ingress",
        "heterogeneous_v3_resource_join_rolls_back_all_owners",
    ] {
        if !tests.contains(required) {
            defects.push(format!("missing focused behavior proof {required}"));
        }
    }
    if production
        .contains("let surface_resource = accepted.surface_resource.liquid_arithmetic().clone();")
    {
        defects.push(
            "V3 heterogeneous seam still replaces the resource wholesale with accepted native arithmetic"
                .to_owned(),
        );
    }

    assert!(
        defects.is_empty(),
        "exact heterogeneous V3 surface-resource join is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn wat5_replays_exact_typed_additional_supply_segments_without_retiming() {
    let repository = root();
    let production_paths = [
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_wat5.rs",
    ];
    let test_paths = [
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_tests_tail.rs",
        "crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs",
        "crates/openwepp-runner/src/hillslope/tests03/wat5_output_transaction.rs",
    ];
    let production = production_paths
        .iter()
        .map(|relative| fs::read_to_string(repository.join(relative)).expect("read WAT5 seam"))
        .collect::<Vec<_>>()
        .join("\n");
    let tests = test_paths
        .iter()
        .map(|relative| fs::read_to_string(repository.join(relative)).expect("read WAT5 tests"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut defects = Vec::new();
    for required in [
        "Wat5AdditionalSupplySourceKindV1",
        "Wat5AdditionalSupplySegmentV1",
        "accepted_wat5_additional_supply_segments",
        "wat5_piecewise_combined_supply",
        "rainfall_and_exact_typed_additional_segments_saturation_hourly_zero_order_hold",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing exact WAT5 segment token {required}"));
        }
    }
    for required in [
        "wat5_exact_segments_keep_rain_and_snow_terminal_distinct",
        "wat5_exact_segments_preserve_routed_runon_receipt_support",
        "wat5_exact_segments_preserve_litter_overflow_phase_receipt_support",
        "wat5_overlapping_sources_advance_piecewise_wb14_once",
        "wat5_exact_segments_reconstruct_hourly_additional_supply_without_double_count",
        "wat5_exact_segments_reject_unknown_retimed_or_substituted_source",
        "wat5_exact_segments_preserve_wat_pass_hbp_manifest_for_1_10_19",
        "wat5_exact_segments_roll_back_complete_output_set",
    ] {
        if !tests.contains(required) {
            defects.push(format!("missing focused WAT5 proof {required}"));
        }
    }
    if production.contains("accepted WAT5 precipitation source kind")
        || production.contains("let mut hourly = accepted.hourly_snow_terminal_liquid_m;")
    {
        defects.push(
            "WAT5 still rejects/aggregates accepted non-rain supply instead of retaining typed segments"
                .to_owned(),
        );
    }

    assert!(
        defects.is_empty(),
        "exact typed WAT5 additional-segment seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn wat5_bounded_partition_reconciliation_preserves_positive_accepted_hour() {
    let repository = root();
    let production = [
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs",
    ]
    .iter()
    .map(|relative| fs::read_to_string(repository.join(relative)).expect("read WAT5 replay"))
    .collect::<Vec<_>>()
    .join("\n");
    let tests = [
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs",
        "crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs",
        "crates/openwepp-runner/src/hillslope/tests03/wat5_output_transaction.rs",
    ]
    .iter()
    .map(|relative| fs::read_to_string(repository.join(relative)).expect("read WAT5 tests"))
    .collect::<Vec<_>>()
    .join("\n");

    let mut defects = Vec::new();
    for required in [
        "Wat5BoundedPartitionLedgerReconciliationV1",
        "reconcile_wat5_zero_raw_generation_hour_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing bounded WAT5 reconciliation token {required}"
            ));
        }
    }
    for required in [
        "wat5_bounded_reconciliation_places_on_latest_positive_source_piece",
        "wat5_bounded_reconciliation_preserves_raw_supply_infiltration_closure",
        "wat5_bounded_reconciliation_preserves_authoritative_positive_hour",
        "wat5_bounded_reconciliation_is_source_order_independent",
        "wat5_bounded_reconciliation_rejects_first_uniform_or_duplicate_placement",
        "wat5_bounded_reconciliation_rejects_zero_foreign_or_missing_source_support",
        "wat5_bounded_reconciliation_accepts_exact_tolerance_boundary",
        "wat5_bounded_reconciliation_rejects_first_value_above_tolerance",
        "wat5_bounded_reconciliation_rolls_back_complete_output_set",
    ] {
        if !tests.contains(required) {
            defects.push(format!(
                "missing bounded WAT5 reconciliation proof {required}"
            ));
        }
    }
    if production.contains("WAT5-E-002 positive authoritative WB14 hour has zero raw support") {
        defects.push(
            "WAT5 still unconditionally rejects a source-complete bounded zero-raw hour".to_owned(),
        );
    }

    assert!(
        defects.is_empty(),
        "bounded source-supported WAT5 reconciliation seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn wat5_stage3_projects_sealed_receipt_dispositions_without_a_second_physical_solve() {
    let repository = root();
    let production = [
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_wat5.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs",
    ]
    .into_iter()
    .map(|relative| fs::read_to_string(repository.join(relative)).expect("read WAT5 v5 seam"))
    .collect::<Vec<_>>()
    .join("\n");
    let tests = fs::read_to_string(
        repository.join(
            "crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_tests_tail.rs",
        ),
    )
    .expect("read WAT5 v5 focused tests");

    for required in [
        "accepted_wat5_receipt_sources",
        "accepted_wat5_receipt_profile",
        "wat5_source_profile_with_exact_segments",
        "run_wat5_subhourly_generation_with_accepted_profile",
        "DirectSurfaceLiquidReceiptDisposition::Infiltration",
        "DirectSurfaceLiquidReceiptDisposition::RetainedSurface",
        "DirectSurfaceLiquidReceiptDisposition::RoutedRunoff",
        "DirectSurfaceLiquidReceiptDisposition::OutletRunoff",
        "WAT5-E-001 accepted receipt source/disposition closure",
    ] {
        assert!(
            production.contains(required),
            "missing receipt-complete WAT5 production token {required}",
        );
    }
    for required in [
        "accepted_wat5_receipt_profile_preserves_source_and_disposition_ledgers_without_second_solve",
        "accepted_wat5_receipt_profile_rejects_source_disposition_omission_or_duplication",
    ] {
        assert!(
            tests.contains(required),
            "missing receipt-complete WAT5 proof {required}",
        );
    }
}

#[test]
fn wat5_v4_output_consumer_validates_typed_supply_and_bounded_closing() {
    let repository = root();
    let output_path =
        repository.join("crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs");
    let output = fs::read_to_string(&output_path).expect("read WAT5 output consumer");
    let focused = [
        "crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs",
        "tests/integration/subhourly_water_output_roundtrip.rs",
        "tests/integration/subhourly_generation_properties.rs",
    ]
    .iter()
    .map(|relative| fs::read_to_string(repository.join(relative)).expect("read output tests"))
    .collect::<Vec<_>>()
    .join("\n");

    let mut defects = Vec::new();
    for required in [
        "WAT5_V4_SOURCE_COMPLETENESS_CODE",
        "rainfall_and_exact_typed_additional_segments_saturation_hourly_zero_order_hold",
        "validate_v4_bounded_closing_reconciliation",
    ] {
        if !output.contains(required) {
            defects.push(format!("missing WAT5 v4 output binding {required}"));
        }
    }
    for required in [
        "output_accepts_exact_typed_additional_supply_source_code",
        "output_validates_combined_rain_and_additional_raw_closure",
        "output_validates_single_bounded_closing_reconciliation_on_latest_positive_source_bin",
        "output_rejects_bounded_reconciliation_without_positive_typed_source",
        "output_rejects_duplicate_or_nonlatest_bounded_reconciliation",
        "output_rejects_bounded_reconciliation_above_tolerance",
        "output_preserves_hour_and_run_output_set_on_failure",
    ] {
        if !focused.contains(required) {
            defects.push(format!("missing focused WAT5 v4 output proof {required}"));
        }
    }
    if output.contains("row.additional_supply_depth_mm != 0.0") {
        defects.push("output still rejects every positive typed additional-supply row".to_owned());
    }
    if output.contains("approximately_equal(row.rainfall_depth_mm, raw_accounted_mm)") {
        defects.push("output raw closure still omits additional supply".to_owned());
    }
    if output.contains("rainfall_complete_saturation_hourly_zero_order_hold")
        && !output.contains("WAT5_V4_SOURCE_COMPLETENESS_CODE")
    {
        defects.push("output still binds the stale rain-only source code".to_owned());
    }

    assert!(
        defects.is_empty(),
        "WAT5 v4 output-consumer binding is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn exact_surface_owner_uses_authenticated_topology_rank_for_opaque_ofe_ids() {
    let repository = root();
    let production_path = repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs",
    );
    let production = fs::read_to_string(&production_path).expect("read exact-surface owner");
    let reconstruction = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_execution.rs",
    ))
    .expect("read exact-surface operand reconstruction");
    let retained_credit_replay = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs",
    ))
    .expect("read retained exact-surface credit replay");
    let focused =
        fs::read_to_string(repository.join(
            "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_tests.rs",
        ))
        .expect("read exact-surface focused tests");

    let mut defects = Vec::new();
    for required in [
        "validate_topology_ranked_exact_surface_order",
        "topology_rank",
        "configuration_sha256",
    ] {
        if !production.contains(required) {
            defects.push(format!(
                "missing topology-ranked exact-owner binding {required}"
            ));
        }
    }
    for required in [
        "exact_surface_owner_accepts_configured_ofe_9_then_ofe_10_topology",
        "exact_surface_owner_accepts_opaque_nonlexical_ofe_topology",
        "exact_surface_owner_preserves_within_ofe_and_operand_order",
        "exact_surface_owner_rejects_duplicate_omitted_or_substituted_topology_keys",
        "exact_surface_owner_rejects_topology_relative_or_within_ofe_reorder",
        "exact_surface_owner_rejects_stale_configuration_digest",
        "exact_surface_owner_topology_failure_rolls_back_all_bytes",
    ] {
        if !focused.contains(required) {
            defects.push(format!("missing exact-owner topology proof {required}"));
        }
    }
    if production.contains("key >= &record.surface_key") {
        defects.push("bare owner validation still imposes lexical surface-key order".to_owned());
    }
    if production.contains("prior >= &identity") {
        defects.push("accepted operands still impose lexical OFE order".to_owned());
    }
    for required in [
        "V16 exact-surface operand topology",
        "topology_rank[&left.surface_key]",
        "topology_rank[&right.surface_key]",
    ] {
        if !reconstruction.contains(required) {
            defects.push(format!(
                "missing topology-ranked exact-surface reconstruction binding {required}"
            ));
        }
    }
    if reconstruction.contains("(&left.surface_key, &left.kind, left.ordinal).cmp") {
        defects.push("exact-surface producer still imposes lexical surface-key order".to_owned());
    }
    for required in [
        "for record in &configuration.parent().records",
        "grouped.remove(store_key)",
        "ordinal: u32::try_from(credits.len())",
    ] {
        if !retained_credit_replay.contains(required) {
            defects.push(format!(
                "missing topology-ranked retained-credit replay binding {required}"
            ));
        }
    }
    if retained_credit_replay.contains("for (ordinal, (store_key, mut receipts)) in grouped") {
        defects
            .push("retained-credit replay still assigns ordinals through lexical keys".to_owned());
    }

    assert!(
        defects.is_empty(),
        "topology-ranked exact-surface owner seam is absent:\n{}",
        defects.join("\n")
    );
}

#[test]
fn stage3_lane_d_qualification_reads_canonical_manifest_provenance() {
    let repository = root();
    let qualification = fs::read_to_string(
        repository
            .join("crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs"),
    )
    .expect("read Stage-3 Lane-D qualification source");
    let wat_schema = fs::read_to_string(
        repository.join("crates/openwepp-hillslope-output/src/hillslope_wat.rs"),
    )
    .expect("read canonical WAT schema source");
    let wat5_schema = fs::read_to_string(
        repository.join("crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs"),
    )
    .expect("read canonical WAT5 schema source");
    let canonical = "manifest[\"execution_provenance\"][\"laned_active\"]";
    let stale = "manifest[\"execution\"][\"laned_active\"]";

    assert_eq!(
        qualification.matches(canonical).count(),
        4,
        "every Lane-D qualification manifest read must use canonical execution provenance"
    );
    assert!(
        !qualification.contains(stale),
        "the retired non-schema execution path must not return to Lane-D qualification"
    );
    for column in ["OFE", "QOFE", "Area", "Snow-Water", "frozwt"] {
        assert!(
            wat_schema.contains(&format!("\"{column}\"")),
            "qualification-requested WAT column {column} must exist in the canonical schema"
        );
        assert!(
            qualification.contains(&format!("\"{column}\"")),
            "qualification must request canonical WAT column {column}"
        );
    }
    for stale_column in ["ofe", "qofe", "area", "snow_water"] {
        let stale_argument = format!(", \"{stale_column}\")");
        assert!(
            !qualification.contains(&stale_argument),
            "qualification must not request stale WAT column alias {stale_column}"
        );
    }
    for sparse_binding in [
        "reconstruct_sparse_wat5_hourly_sources",
        "value.unwrap_or(0.0)",
        "hour != subinterval / 12",
        "wat5_sparse_and_dense_hourly_source_reconstruction_are_equivalent",
        "wat5_hourly_source_reconstruction_rejects_duplicate_or_noncanonical_rows",
    ] {
        assert!(
            qualification.contains(sparse_binding),
            "missing sparse WAT5 qualification binding {sparse_binding}"
        );
    }
    assert!(
        wat5_schema.contains("\"omitted_bins\".to_string()")
            && wat5_schema.contains("\"exact_zero\".to_string()"),
        "canonical WAT5 metadata must bind omitted bins to exact zero"
    );
}

#[test]
fn stage3_lane_d_hbp_reads_authenticated_public_hourly_surfaces() {
    let repository = root();
    let executor = format!(
        "{}\n{}",
        fs::read_to_string(
            repository
                .join("crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs")
        )
        .expect("read direct executor source"),
        fs::read_to_string(repository.join(
            "crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor_tests.rs"
        ))
        .expect("read direct executor test source"),
    );
    let hbp = fs::read_to_string(
        repository.join("crates/openwepp-runner/src/hillslope/04_direct_publication.rs"),
    )
    .expect("read HBP publication source");

    let stage3_branch = executor
        .split("if stage3_v11_owned_day {")
        .nth(1)
        .and_then(|tail| {
            tail.split("archive_committed_day(frame, day_index)?;")
                .next()
        })
        .expect("locate committed Stage-3 publication branch");
    let committed_clone = stage3_branch
        .find(".committed_snow_stage3_publication_day(day_index)?")
        .expect("committed accepted day lookup");
    let authority_stability = stage3_branch
        .find("if frame.laned_active.is_some() != laned_active_owned_day {")
        .expect("Lane-D authority selector stability guard");
    let ledger_validation = stage3_branch
        .find("Self::validate_optional_laned_active_public_day_ledger(")
        .expect("posture-aware authenticated Lane-D public ledger validation");
    let row_construction = stage3_branch
        .find("DirectPublicationDayRow::from_day_frame(&day_frame, day_input, lane)?")
        .expect("committed accepted row construction");
    let active_binding_guard = stage3_branch
        .find("if laned_active_owned_day {")
        .expect("authoritative active-posture hourly binding guard");
    let hourly_binding = stage3_branch
        .find("Self::bind_laned_active_public_hourly_surfaces(&day_frame, &mut row)?")
        .expect("committed accepted active-posture hourly binding");
    let sink_consume = stage3_branch
        .find("consume_row(&row, &day_frame)?")
        .expect("streaming sink consumption");
    assert!(
        committed_clone < authority_stability
            && authority_stability < ledger_validation
            && ledger_validation < row_construction
            && row_construction < active_binding_guard
            && active_binding_guard < hourly_binding
            && hourly_binding < sink_consume,
        "the exact committed Stage-3 frame must preserve the Lane-D authority selector, validate the posture-aware public ledger, and bind HBP timing only for the active posture before the sink clones it"
    );
    assert_eq!(
        executor
            .matches("Self::bind_laned_active_public_hourly_surfaces(")
            .count(),
        1,
        "the Stage-3 public row must have exactly one Lane-D HBP attachment call"
    );

    for binding in [
        "bind_laned_active_public_hourly_surfaces(&day_frame, &mut row)?",
        ".laned_active_routing",
        "Some(routing.routed_weights)",
        "laned_active authenticated routed hourly surface",
        "laned_active Wave-1 hourly sediment surface",
        "actual.to_bits() != expected.to_bits()",
        "candidate.hourly_sediment_mass_kg = Some([0.0; 24])",
        "laned_active_public_hourly_pair_preserves_all_24_bins_and_peak",
        "laned_active_public_hourly_pair_preserves_matching_nonzero_sediment",
        "laned_active_public_hourly_pair_mismatch_rolls_back",
        "laned_active_public_hourly_pair_missing_sources_roll_back",
        "laned_active_public_hourly_pair_invalid_sediment_rolls_back",
        "candidate.runvol_m3 = routing.outlet_m3",
        ".map(|weight| weight * routing.outlet_m3)",
        "/ 3_600.0",
        "laned_active.publication.source_clamp_outlet_storage",
        "laned_active_terminal_public_runoff_uses_exact_outlet_and_hourly_peak",
        "laned_active_nonterminal_public_runoff_preserves_accepted_scalars",
        "laned_active_terminal_public_runoff_invalid_weights_roll_back",
        "laned_active_public_day_ledger_closes_storage_and_clamp_or_rejects",
    ] {
        assert!(
            executor.contains(binding),
            "missing authenticated Lane-D public hourly binding {binding}"
        );
    }
    assert!(
        executor.contains("day_frame.laned_active_routing =")
            && executor.contains("routed_weights: [0.0; 24]"),
        "exact-zero active days must retain their authenticated 24-bin routing record"
    );

    for consumer in [
        "sediment_row.erosion.hourly_runoff_fraction",
        "sediment_row.erosion.hourly_sediment_mass_kg",
        "*slot = fraction * runvol_m3",
        "hourly_runoff_volume_m3: surfaces.hourly_runoff_volume_m3",
    ] {
        assert!(
            hbp.contains(consumer),
            "HBP must consume the paired Lane-D public hourly surface through {consumer}"
        );
    }
}

#[test]
fn representative_long_run_recipe_binds_real_runner_without_narrowing() {
    let repository = root();
    let recipe: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(repository.join(
            "docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/representative-10ofe-100year-workload.json",
        ))
        .expect("read representative workload recipe"),
    )
    .expect("parse representative workload recipe");
    assert_eq!(recipe["ofe_count"].as_u64(), Some(10));
    assert_eq!(
        recipe["century"]["expected_day_count"].as_u64(),
        Some(36_525),
    );
    assert_eq!(
        recipe["century"]["expected_leap_day_count"].as_u64(),
        Some(25),
    );
    assert_eq!(
        recipe["prohibited_shortcuts"]
            .as_array()
            .expect("prohibited shortcut population")
            .len(),
        5,
    );

    let registration =
        fs::read_to_string(repository.join("crates/openwepp-runner/src/hillslope/03_tests.rs"))
            .expect("read runner test registration");
    assert!(registration.contains("include!(\"tests03/stage3_long_run_qualification.rs\")"));
    let harness = fs::read_to_string(
        repository
            .join("crates/openwepp-runner/src/hillslope/tests03/stage3_long_run_qualification.rs"),
    )
    .expect("read representative workload harness");
    for binding in [
        "REPRESENTATIVE_LONG_RUN_OFE_COUNT: usize = 10",
        "REPRESENTATIVE_LONG_RUN_CENTURY_DAYS: usize = 36_525",
        "year_count * REPRESENTATIVE_LONG_RUN_OFE_COUNT",
        "author_stage3_v11_owner_seed_fixture",
        "HillslopeRuntimeSelection::DirectProductionExecutor",
        "execute_hillslope_run_with_runtime_policy",
        "archive[\"record_count\"]",
        "active[\"days_seen\"]",
        "active[\"days_routed\"]",
        "total_source_m3",
        "total_routed_outlet_m3",
        "representative_ten_ofe_complete_year_real_runner",
        "representative_ten_ofe_hundred_year_real_runner",
    ] {
        assert!(
            harness.contains(binding),
            "representative workload harness missing {binding}",
        );
    }
}

#[test]
fn active_lane_d_projects_surface_liquid_to_local_only_before_stage3_bootstrap() {
    let repository = root();
    let runner = fs::read_to_string(
        repository.join("crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs"),
    )
    .expect("read runner execution source");
    let seed = fs::read_to_string(
        repository.join("crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs"),
    )
    .expect("read Stage-3 production seed");
    let identity = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/identity_validation.rs",
    ))
    .expect("read SurfaceLiquid identity validator");

    for binding in [
        "let pre_bootstrap_laned_active_enabled = resolve_laned_active_enabled(&day_input_builder)",
        "bootstrap_with_laned_active_surface_owner",
        "active Lane-D ownership changed across Stage-3 bootstrap",
    ] {
        assert!(
            runner.contains(binding),
            "missing pre-bootstrap Lane-D binding {binding}"
        );
    }
    for binding in [
        "project_laned_active_day_zero_surface_owner",
        "INV-OFEROUTE-015 active surface-owner projection requires an untouched day-zero state",
        "record.runon_destination_ofe_id = None",
        "record.runon_destination_tile_id = None",
        "active_laned_bootstrap_removes_surface_runon_and_preserves_day_zero_liquid_bits",
        "active_laned_bootstrap_rejects_non_day_zero_owner_without_mutation",
    ] {
        assert!(
            seed.contains(binding),
            "missing Lane-D local-owner proof {binding}"
        );
    }
    assert!(
        identity.contains("let lane_d_local = route_by_ofe")
            && identity.contains("if lane_d_local"),
        "SurfaceLiquid identity must admit only the complete all-local routing posture"
    );
}

#[test]
fn snow_free_native_litter_stages_the_current_wb14_parent_before_physics() {
    let source = fs::read_to_string(
        root().join("crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs"),
    )
    .expect("read native real-consumer source");
    let body = source
        .split("install_imported_v10_snow_free_authority(&mut candidate);")
        .nth(1)
        .expect("snow-free authority installation")
        .split("let deferred_v4_prepared_soil")
        .next()
        .expect("snow-free native-parent staging region");
    assert!(
        body.contains("stage_frozen_litter_wb14_parent_from_inner_v1()"),
        "snow-free native V3/V4 must transactionally adopt or finalize the current inner WB14 parent before preparing physical execution"
    );
}

#[test]
fn snow_free_final_receipt_reseal_is_contract_bound_and_production_reachable() {
    let repository = root();
    for (path, version, invariant, obligation) in [
        (
            "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
            "contract_version: 16",
            "INV-COUPLEDTIME-029",
            "OBL-COUPLEDTIME-012",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md",
            "contract_version: 18",
            "INV-VEGTRANSACTION-018",
            "OBL-VEGTRANSACTION-C-006",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "contract_version: 31",
            "INV-LANDSURFACEENERGY-160",
            "OBL-LANDSURFACEENERGY-C-015",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
            "contract_version: 29",
            "INV-SURFACELIQUID-033",
            "OBL-SURFACELIQUID-C-023",
        ),
    ] {
        let contract = fs::read_to_string(repository.join(path)).expect("read contract");
        assert!(contract.contains(version), "{path} missing {version}");
        assert!(contract.contains(invariant), "{path} missing {invariant}");
        assert!(contract.contains(obligation), "{path} missing {obligation}");
        assert!(contract.to_ascii_lowercase().contains("replay fallback"));
    }

    let execution = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_real_parent_execution.rs",
    ))
    .expect("read snow-free V11 execution");
    assert!(execution.contains("SnowFreePhysicalReuseSeedV1"));
    assert!(execution.contains("prepare_snow_free_physical_reuse"));
    assert!(execution.contains("snow-free physical reuse ending owners"));
}

#[test]
fn native_inactive_wb14_prefix_transition_is_contract_bound_and_production_reachable() {
    let repository = root();
    for (path, version, invariant, obligation, exposure) in [
        (
            "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
            "contract_version: 29",
            "INV-SURFACELIQUID-035",
            "OBL-SURFACELIQUID-C-025",
            "SURFACELIQUID-V28-NATIVE-INACTIVE-PREFIX-TRANSITION",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
            "contract_version: 16",
            "INV-COUPLEDTIME-031",
            "OBL-COUPLEDTIME-014",
            "CT-NATIVE-INACTIVE-PREFIX-TRANSITION",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
            "contract_version: 60",
            "INV-SNOWENERGY-087",
            "OBL-SNOWENERGY-C-055",
            "SNOWENERGY-ADR0044-NATIVE-INACTIVE-PREFIX-TRANSITION",
        ),
    ] {
        let contract = fs::read_to_string(repository.join(path)).expect("read contract");
        for binding in [version, invariant, obligation, exposure] {
            assert!(contract.contains(binding), "{path} missing {binding}");
        }
    }

    let production_sources = [
        "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_wb14.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v2_ingress_adapter.rs",
    ]
    .into_iter()
    .map(|path| fs::read_to_string(repository.join(path)).expect("read production source"))
    .collect::<Vec<_>>()
    .join("\n");
    for required in [
        "ValidatedNativeInactiveWb14PrefixV1",
        "validate_native_inactive_wb14_prefix_v1",
        "begin_after_native_inactive_prefix",
        "openwepp-wb14-parent-receipt-chain-native-prefix-v1",
        "inactive_prefix_sha256",
    ] {
        assert!(
            production_sources.contains(required),
            "expected-red: missing authenticated native inactive-prefix production seam {required}"
        );
    }
}

#[test]
fn canonical_covered_pending_adjudication_is_contract_bound() {
    let repository = root();
    for (path, version, invariant, obligation, exposure) in [
        (
            "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
            "contract_version: 60",
            "INV-SNOWENERGY-086",
            "OBL-SNOWENERGY-C-054",
            "SNOWENERGY-ADR0044-NONFINAL-PHYSICAL-ONLY",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "contract_version: 31",
            "INV-LANDSURFACEENERGY-161",
            "OBL-LANDSURFACEENERGY-C-016",
            "LSE-V27-PENDING-ADJUDICATION",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md",
            "contract_version: 18",
            "INV-VEGTRANSACTION-019",
            "OBL-VEGTRANSACTION-P-005",
            "BEI-VEGTRANSACTION-COVERED-NONFINAL-PHYSICAL-ONLY",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
            "contract_version: 29",
            "INV-SURFACELIQUID-034",
            "OBL-SURFACELIQUID-C-024",
            "SURFACELIQUID-V29-COVERED-PENDING-ADJUDICATION",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
            "contract_version: 16",
            "INV-COUPLEDTIME-030",
            "OBL-COUPLEDTIME-013",
            "CT-COVERED-NONFINAL-PHYSICAL-ONLY",
        ),
    ] {
        let contract = fs::read_to_string(repository.join(path)).expect("read contract");
        for binding in [version, invariant, obligation, exposure] {
            assert!(contract.contains(binding), "{path} missing {binding}");
        }
        if path.ends_with("SC-VEGETATIONTRANSACTION-001.md") {
            assert!(
                contract.contains("OBL-VEGTRANSACTION-C-007"),
                "{path} missing OBL-VEGTRANSACTION-C-007"
            );
        }
    }

    let solver = fs::read_to_string(repository.join(
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/canonical_covered_solver.rs",
    ))
    .expect("read canonical covered solver");
    let snowenergy = fs::read_to_string(
        repository.join("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"),
    )
    .expect("read snow-energy contract");
    let coupled_time = fs::read_to_string(
        repository.join("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md"),
    )
    .expect("read coupled-time contract");
    for contract in [&snowenergy, &coupled_time] {
        for authority in [
            "Initial@0",
            "FixedPointAdjudication@1",
            "FinalAccepted",
            "M=N+2",
            "2<=M<=7",
        ] {
            assert!(
                contract.contains(authority),
                "canonical covered contract missing pending-adjudication authority `{authority}`",
            );
        }
    }
    assert!(
        snowenergy.contains("candidate against its own map output")
            && snowenergy.contains("dependent-output stability")
            && snowenergy.contains("consumed exactly once"),
        "snow-energy contract missing exclusive pending-adjudication authority",
    );
    assert!(
        coupled_time.contains("exactly one disposition")
            && coupled_time.contains("not an additional charged role")
            && coupled_time.contains("Dependent-only nonclosure"),
        "coupled-time contract missing exclusive pending-adjudication disposition authority",
    );
    let carrier = fs::read_to_string(
        repository.join("crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs"),
    )
    .expect("read covered carrier phase");
    let production = format!("{solver}\n{carrier}");

    for binding in [
        "CanonicalCoveredIterationMapV1",
        "CanonicalCoveredPendingAdjudicationMapV1",
        "CanonicalCoveredFinalMapV1",
        "execute_canonical_covered_iteration_map_v1",
        "execute_canonical_covered_pending_adjudication_map_v1",
        "consume_canonical_covered_pending_as_history_v1",
        "consume_canonical_covered_pending_as_adaptive_rejection_v1",
        "consume_canonical_covered_pending_as_final_v1",
        "execute_covered_carrier_physical_phase_v1",
    ] {
        assert!(
            production.contains(binding),
            "expected-red: canonical covered execution lacks role-specific private result boundary {binding}"
        );
    }

    let pending_declaration = production
        .lines()
        .find(|line| line.contains("struct CanonicalCoveredPendingAdjudicationMapV1"))
        .expect("pending adjudication map declaration");
    assert_eq!(
        pending_declaration.trim_start(),
        "struct CanonicalCoveredPendingAdjudicationMapV1 {",
        "pending adjudication result must remain module-private"
    );
    let pending_declaration_offset = production
        .find("struct CanonicalCoveredPendingAdjudicationMapV1")
        .expect("pending adjudication map declaration offset");
    let pending_attribute_window = production[..pending_declaration_offset]
        .lines()
        .rev()
        .take(6)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !production.contains("Serialize for CanonicalCoveredPendingAdjudicationMapV1")
            && !production.contains("Deserialize for CanonicalCoveredPendingAdjudicationMapV1")
            && !production.contains("Clone for CanonicalCoveredPendingAdjudicationMapV1")
            && !production.contains("Copy for CanonicalCoveredPendingAdjudicationMapV1")
            && !production.contains("From<CanonicalCoveredPendingAdjudicationMapV1")
            && !production.contains("TryFrom<CanonicalCoveredPendingAdjudicationMapV1")
            && !pending_attribute_window.contains("Serialize")
            && !pending_attribute_window.contains("Deserialize")
            && !pending_attribute_window.contains("Clone")
            && !pending_attribute_window.contains("Copy"),
        "pending adjudication must be move-only and expose no wire or promotion conversion"
    );

    let execution = solver
        .split("fn execute_canonical_covered_production_v1")
        .nth(1)
        .expect("canonical covered production body");
    for binding in [
        "execute_canonical_covered_iteration_map_v1",
        "execute_canonical_covered_pending_adjudication_map_v1",
        "consume_canonical_covered_pending_as_history_v1",
        "consume_canonical_covered_pending_as_adaptive_rejection_v1",
        "consume_canonical_covered_pending_as_final_v1",
    ] {
        assert!(
            execution.contains(binding),
            "canonical production does not consume {binding}"
        );
    }

    let iteration = solver
        .split("fn execute_canonical_covered_iteration_map_v1")
        .nth(1)
        .expect("iteration map function")
        .split("fn execute_canonical_covered_pending_adjudication_map_v1")
        .next()
        .expect("iteration map body");
    assert!(iteration.contains("execute_covered_carrier_physical_phase_v1"));
    assert!(
        !iteration.contains("execute_covered_carrier_batch_phase_v2"),
        "nonfinal roles must not enter the complete envelope constructor"
    );
}
