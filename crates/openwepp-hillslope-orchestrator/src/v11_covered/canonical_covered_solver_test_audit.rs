#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CanonicalCoveredAuditRoleV1 {
    Initial,
    FixedPointAdjudication,
    MultisecantAdjudication(u8),
    UnchargedSentinel,
}

#[cfg(test)]
impl From<CanonicalCoveredMapRoleV1> for CanonicalCoveredAuditRoleV1 {
    fn from(value: CanonicalCoveredMapRoleV1) -> Self {
        match value {
            CanonicalCoveredMapRoleV1::Initial => Self::Initial,
            CanonicalCoveredMapRoleV1::FixedPointAdjudication => Self::FixedPointAdjudication,
            CanonicalCoveredMapRoleV1::MultisecantAdjudication(trial) => {
                Self::MultisecantAdjudication(trial)
            }
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredErrorClassV1 {
    CoupledTime,
    Runtime,
    Vegetation,
    V8OwnerIdentity,
    V8Vegetation,
    V8Biogeochemistry,
    Serialization,
    Stage3,
    Identity,
    SurfaceLiquidReplay,
    ZeroDurationSnowLiquid,
    OpenSnowLowerBoundaryDomain,
    ComponentCarrierReferenceFluxCustody,
    AdaptiveRefinement,
    CoveredBoundary,
    Stage3PrecipitationCustody,
    Stage3SnowSoilHeatCustody,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredRejectionStageV1 {
    RoleOrdinal,
    PreflightIdentity,
    Physical,
    PhysicalValidation,
    V8Persistent,
    V8VegetationCandidate,
    V8Biogeochemistry,
    V8EnvelopeValidation,
    EndingJoint,
    CompleteOwnerSet,
    RestartHistory,
    AdjudicationConvergence,
    MultisecantProposal,
    PublicationSupport,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredPhysicalParityCaseV1 {
    Ordinary,
    NativeFrozen,
    NativeMixedPhase,
    NativeThawRefreeze,
    NativeWetCanopy,
    NativeMultiOfe,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredPhysicalParityPoisonV1 {
    RoleOrdinal,
    Support,
    Transaction,
    Topology,
    LowerBoundary,
    Precipitation,
    SoilCandidate,
    BeginningOwner,
    NativeOrdinarySubstitution,
    HalfNativeCustody,
    PhysicalOneUlp,
    RoleOrdinalAndPhysicalOneUlp,
    SupportAndPhysicalOneUlp,
    LowerBoundaryAndV8Persistent,
    V8Persistent,
    V8VegetationCandidate,
    V8Biogeochemistry,
    V8EnvelopeValidation,
    EndingJoint,
    CompleteOwnerSet,
    PublicationSupport,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct CanonicalCoveredPoisonTargetV1 {
    pub role: CanonicalCoveredAuditRoleV1,
    pub ordinal: u32,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredFinalConvergencePoisonV1 {
    OuterNonclosure,
    DependentNonclosure,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredMultisecantProposalPoisonV1 {
    ExtremeFiniteDepth,
    FiniteDensityAboveCap,
    FiniteAboveFreezingTemperature,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CanonicalCoveredPhysicalEndpointAuditV1 {
    evidence: Vec<CoveredProvisionalPhysicalAuditV1>,
    pub includes_stage3_endings: bool,
    pub includes_surface_wb14_custody: bool,
    pub retains_inactive_native_litter_wb14_bytes: bool,
    pub realizes_named_regime: bool,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CanonicalCoveredPhysicalParityAuditV1 {
    pub physical_only: CanonicalCoveredPhysicalEndpointAuditV1,
    pub forced_complete: CanonicalCoveredPhysicalEndpointAuditV1,
    pub native_snow_free_litter_physics_call_count: u32,
    pub native_snow_free_surface_physics_call_count: u32,
    pub native_snow_free_wb14_physics_call_count: u32,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalCoveredPhysicalPoisonRejectionV1 {
    pub typed_error: CanonicalCoveredErrorClassV1,
    pub rejection_stage: CanonicalCoveredRejectionStageV1,
    pub rollback_byte_identical: bool,
    pub completed_final_envelope_count: u32,
    pub accepted_parent_publication_count: u32,
    pub charged_map_attempt_count: u32,
    pub validated_physical_endpoint_count: u32,
    pub validated_iteration_endpoint_count: u32,
    pub validated_final_physical_endpoint_count: u32,
    pub final_constructor_attempt_count: u32,
    pub v8_receipt_constructor_attempt_count: u32,
    pub vegetation_persistent_constructor_attempt_count: u32,
    pub vegetation_material_constructor_attempt_count: u32,
    pub biogeochemistry_constructor_attempt_count: u32,
    pub ending_joint_constructor_attempt_count: u32,
    pub complete_owner_set_constructor_attempt_count: u32,
    pub restart_owner_constructor_attempt_count: u32,
    pub publication_support_constructor_attempt_count: u32,
    pub map_local_publication_attempt_count: u32,
    pub charged_roles: Vec<(CanonicalCoveredAuditRoleV1, u32)>,
    pub validated_pending_adjudication_count: u32,
    pub history_disposition_count: u32,
    pub dependent_rejection_disposition_count: u32,
    pub final_disposition_count: u32,
}

#[cfg(test)]
pub(crate) fn canonical_covered_physical_parity_for_test(
    case: CanonicalCoveredPhysicalParityCaseV1,
) -> Result<CanonicalCoveredPhysicalParityAuditV1, DirectV11RealConsumerError> {
    let (
        physical_only,
        forced_complete,
        physical_only_native,
        forced_complete_native,
        _physical_only_map,
        _forced_complete_map,
    ) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            canonical_covered_physical_parity_fixture_v1(case);
    if physical_only.is_empty() || forced_complete.is_empty() {
        return Err(DirectV11RealConsumerError::Identity(
            "covered physical parity missing actual endpoint evidence",
        ));
    }
    let native = !matches!(case, CanonicalCoveredPhysicalParityCaseV1::Ordinary);
    let endpoint =
        |evidence: Vec<CoveredProvisionalPhysicalAuditV1>,
         native_audit: crate::v9_real_consumer_shadow::CoveredNativePhysicalPathAuditV1|
         -> Result<CanonicalCoveredPhysicalEndpointAuditV1, DirectV11RealConsumerError> {
            let includes_stage3_endings = evidence.iter().all(|value| {
                value
                    .ending_stage3_by_lane
                    .as_ref()
                    .is_some_and(|ending| !ending.is_empty())
            });
            let includes_surface_wb14_custody = evidence.iter().all(|value| {
                value.opaque_physical_projection.sha256 != Digest32::zero()
                    && value
                        .complete_physical_projection_sha256
                        .is_some_and(|digest| digest != Digest32::zero())
                    && value.opaque_physical_projection.lse_destination_count
                        == value.lse_states.len()
                    && value.opaque_physical_projection.release_destination_count
                        == value.lse_states.len()
                    && value.physical_endpoint_captured
                    && value.soil_candidate.is_some()
                    && value.soil_continuation.is_some()
                    && value
                        .batch_boundaries_by_lane
                        .as_ref()
                        .is_some_and(|value| !value.is_empty())
                    && value.carrier_source_receipts.is_some()
                    && value.open_snow_candidates.is_some()
                    && value
                        .terminal_soil_trials
                        .as_ref()
                        .is_some_and(|value| !value.is_empty())
                    && value
                        .terminal_soil_credits
                        .as_ref()
                        .is_some_and(|value| !value.is_empty())
                    && value.surface_custody.is_some()
                    && !value.wb14_child_replay_bytes.is_empty()
                    && digest32_hex(digest_bytes(&value.wb14_child_replay_bytes))
                        == value.wb14_child_receipt_set_sha256
                    && value
                        .wb14_parent_replay_bytes
                        .as_ref()
                        .map(|bytes| digest32_hex(digest_bytes(bytes)))
                        == value.wb14_parent_receipt_set_sha256
            });
            let native_physical_count = evidence
                .iter()
                .filter(|value| value.stage3_covered_native)
                .count();
            let retained_maps = &native_audit.represented_snow_retention_by_map;
            let retains_inactive_native_litter_wb14_bytes = native
                && native_physical_count == evidence.len()
                && retained_maps.len() == evidence.len()
                && retained_maps.iter().all(|value| {
                    value.beginning_v3_sha256 == value.ending_v3_sha256
                        && value.beginning_v4_sha256 == value.ending_v4_sha256
                        && value.beginning_v3_sha256 != Digest32::zero()
                        && value.beginning_v4_sha256 != Digest32::zero()
                });
            let layers = evidence
                .iter()
                .flat_map(|value| {
                    value.beginning_stage3_by_lane.values().chain(
                        value
                            .ending_stage3_by_lane
                            .iter()
                            .flat_map(|states| states.values()),
                    )
                })
                .flat_map(|state| state.layers.iter())
                .collect::<Vec<_>>();
            let realizes_named_regime =
                match case {
                    CanonicalCoveredPhysicalParityCaseV1::Ordinary => native_physical_count == 0,
                    CanonicalCoveredPhysicalParityCaseV1::NativeFrozen => {
                        !layers.is_empty()
                            && layers.iter().all(|layer| {
                                layer.temperature_c < 0.0 && layer.liquid_water_m == 0.0
                            })
                    }
                    CanonicalCoveredPhysicalParityCaseV1::NativeMixedPhase => layers
                        .iter()
                        .any(|layer| layer.liquid_water_m > 0.0 && layer.mass_swe_m > 0.0),
                    CanonicalCoveredPhysicalParityCaseV1::NativeThawRefreeze => {
                        evidence.iter().any(|value| {
                            value
                                .stage3_refreeze_by_lane
                                .values()
                                .any(|amount| *amount > 0.0)
                        })
                    }
                    CanonicalCoveredPhysicalParityCaseV1::NativeWetCanopy => {
                        evidence.iter().any(|value| {
                            value.precipitation_sets.values().flat_map(|set| &set.parcels).any(
                    |parcel| {
                        parcel.source == Stage3PrecipitationSourceV1::VegetationTerminalThroughfall
                            && parcel.mass_kg_m2_tile_ground > 0.0
                            && value.wet_canopy_destinations.contains(&(
                                parcel.destination_ofe_id.clone(),
                                parcel.destination_tile_id.clone(),
                            ))
                    },
                )
                        })
                    }
                    CanonicalCoveredPhysicalParityCaseV1::NativeMultiOfe => {
                        let ofes = evidence
                            .iter()
                            .flat_map(|value| {
                                value
                                    .stage3_surface_destinations
                                    .iter()
                                    .map(|key| key.0.clone())
                            })
                            .collect::<std::collections::BTreeSet<_>>();
                        let tiles = evidence
                            .iter()
                            .flat_map(|value| {
                                value
                                    .stage3_surface_destinations
                                    .iter()
                                    .map(|key| key.1.clone())
                            })
                            .collect::<std::collections::BTreeSet<_>>();
                        let ending_lanes = evidence
                            .iter()
                            .filter_map(|value| value.ending_stage3_by_lane.as_ref())
                            .flat_map(|ending| ending.keys().copied())
                            .collect::<std::collections::BTreeSet<_>>();
                        ofes.len() >= 2 && tiles.len() >= 2 && ending_lanes.len() >= 2
                    }
                };
            if !includes_stage3_endings || !includes_surface_wb14_custody {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered physical parity incomplete actual endpoint custody",
                ));
            }
            if !realizes_named_regime {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered physical parity named regime was not realized",
                ));
            }
            Ok(CanonicalCoveredPhysicalEndpointAuditV1 {
                evidence,
                includes_stage3_endings,
                includes_surface_wb14_custody,
                retains_inactive_native_litter_wb14_bytes,
                realizes_named_regime,
            })
        };
    let native_snow_free_litter_physics_call_count = physical_only_native
        .snow_free_litter_physics_call_count
        .saturating_add(forced_complete_native.snow_free_litter_physics_call_count);
    let native_snow_free_surface_physics_call_count = physical_only_native
        .snow_free_surface_physics_call_count
        .saturating_add(forced_complete_native.snow_free_surface_physics_call_count);
    let native_snow_free_wb14_physics_call_count = physical_only_native
        .snow_free_wb14_physics_call_count
        .saturating_add(forced_complete_native.snow_free_wb14_physics_call_count);
    Ok(CanonicalCoveredPhysicalParityAuditV1 {
        physical_only: endpoint(physical_only, physical_only_native)?,
        forced_complete: endpoint(forced_complete, forced_complete_native)?,
        native_snow_free_litter_physics_call_count,
        native_snow_free_surface_physics_call_count,
        native_snow_free_wb14_physics_call_count,
    })
}

#[cfg(test)]
pub(crate) fn canonical_covered_physical_parity_poison_for_test(
    case: CanonicalCoveredPhysicalParityCaseV1,
    poison: CanonicalCoveredPhysicalParityPoisonV1,
) -> CanonicalCoveredPhysicalPoisonRejectionV1 {
    let (audit, rejection) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            canonical_covered_physical_poison_fixture_v1(case, poison);
    let (typed_error, rejection_stage, rollback_byte_identical) =
        rejection.expect("real covered poison path must record its propagated typed rejection");
    let solve = audit.solves.last();
    CanonicalCoveredPhysicalPoisonRejectionV1 {
        typed_error,
        rejection_stage,
        rollback_byte_identical,
        completed_final_envelope_count: solve
            .map_or(0, |solve| solve.completed_final_envelope_count),
        accepted_parent_publication_count: audit.accepted_parent_publication_count,
        charged_map_attempt_count: solve.map_or(0, |solve| solve.charged_map_attempt_count),
        validated_physical_endpoint_count: solve
            .map_or(0, |solve| solve.validated_physical_endpoint_count),
        validated_iteration_endpoint_count: solve
            .map_or(0, |solve| solve.validated_iteration_endpoint_count),
        validated_final_physical_endpoint_count: solve
            .map_or(0, |solve| solve.validated_final_physical_endpoint_count),
        final_constructor_attempt_count: solve
            .map_or(0, |solve| solve.final_constructor_attempt_count),
        v8_receipt_constructor_attempt_count: solve
            .map_or(0, |solve| solve.v8_receipt_constructor_attempt_count),
        vegetation_persistent_constructor_attempt_count: solve.map_or(0, |solve| {
            solve.vegetation_persistent_constructor_attempt_count
        }),
        vegetation_material_constructor_attempt_count: solve.map_or(0, |solve| {
            solve.vegetation_material_constructor_attempt_count
        }),
        biogeochemistry_constructor_attempt_count: solve
            .map_or(0, |solve| solve.biogeochemistry_constructor_attempt_count),
        ending_joint_constructor_attempt_count: solve
            .map_or(0, |solve| solve.ending_joint_constructor_attempt_count),
        complete_owner_set_constructor_attempt_count: solve.map_or(0, |solve| {
            solve.complete_owner_set_constructor_attempt_count
        }),
        restart_owner_constructor_attempt_count: solve
            .map_or(0, |solve| solve.restart_owner_constructor_attempt_count),
        publication_support_constructor_attempt_count: solve.map_or(0, |solve| {
            solve.publication_support_constructor_attempt_count
        }),
        map_local_publication_attempt_count: audit.map_local_publication_attempt_count,
        charged_roles: solve.map_or_else(Vec::new, |solve| solve.charged_roles.clone()),
        validated_pending_adjudication_count: solve
            .map_or(0, |solve| solve.validated_pending_adjudication_count),
        history_disposition_count: solve.map_or(0, |solve| solve.history_disposition_count),
        dependent_rejection_disposition_count: solve
            .map_or(0, |solve| solve.dependent_rejection_disposition_count),
        final_disposition_count: solve.map_or(0, |solve| solve.final_disposition_count),
    }
}

#[cfg(test)]
pub(crate) fn canonical_covered_targeted_failure_for_test(
    case: CanonicalCoveredPhysicalParityCaseV1,
    poison: CanonicalCoveredPhysicalParityPoisonV1,
    target: CanonicalCoveredPoisonTargetV1,
) -> CanonicalCoveredPhysicalPoisonRejectionV1 {
    let (audit, rejection) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            canonical_covered_physical_poison_fixture_at_map_v1(case, poison, target);
    let (typed_error, rejection_stage, rollback_byte_identical) = rejection
        .expect("targeted covered poison must propagate through the selected authentic map");
    let solve = audit.solves.last();
    CanonicalCoveredPhysicalPoisonRejectionV1 {
        typed_error,
        rejection_stage,
        rollback_byte_identical,
        completed_final_envelope_count: solve
            .map_or(0, |value| value.completed_final_envelope_count),
        accepted_parent_publication_count: audit.accepted_parent_publication_count,
        charged_map_attempt_count: solve.map_or(0, |value| value.charged_map_attempt_count),
        validated_physical_endpoint_count: solve
            .map_or(0, |value| value.validated_physical_endpoint_count),
        validated_iteration_endpoint_count: solve
            .map_or(0, |value| value.validated_iteration_endpoint_count),
        validated_final_physical_endpoint_count: solve
            .map_or(0, |value| value.validated_final_physical_endpoint_count),
        final_constructor_attempt_count: solve
            .map_or(0, |value| value.final_constructor_attempt_count),
        v8_receipt_constructor_attempt_count: solve
            .map_or(0, |value| value.v8_receipt_constructor_attempt_count),
        vegetation_persistent_constructor_attempt_count: solve.map_or(0, |value| {
            value.vegetation_persistent_constructor_attempt_count
        }),
        vegetation_material_constructor_attempt_count: solve.map_or(0, |value| {
            value.vegetation_material_constructor_attempt_count
        }),
        biogeochemistry_constructor_attempt_count: solve
            .map_or(0, |value| value.biogeochemistry_constructor_attempt_count),
        ending_joint_constructor_attempt_count: solve
            .map_or(0, |value| value.ending_joint_constructor_attempt_count),
        complete_owner_set_constructor_attempt_count: solve.map_or(0, |value| {
            value.complete_owner_set_constructor_attempt_count
        }),
        restart_owner_constructor_attempt_count: solve
            .map_or(0, |value| value.restart_owner_constructor_attempt_count),
        publication_support_constructor_attempt_count: solve.map_or(0, |value| {
            value.publication_support_constructor_attempt_count
        }),
        map_local_publication_attempt_count: audit.map_local_publication_attempt_count,
        charged_roles: solve.map_or_else(Vec::new, |solve| solve.charged_roles.clone()),
        validated_pending_adjudication_count: solve
            .map_or(0, |solve| solve.validated_pending_adjudication_count),
        history_disposition_count: solve.map_or(0, |solve| solve.history_disposition_count),
        dependent_rejection_disposition_count: solve
            .map_or(0, |solve| solve.dependent_rejection_disposition_count),
        final_disposition_count: solve.map_or(0, |solve| solve.final_disposition_count),
    }
}

#[cfg(test)]
pub(crate) fn canonical_covered_targeted_final_nonclosure_for_test(
    case: CanonicalCoveredPhysicalParityCaseV1,
    poison: CanonicalCoveredFinalConvergencePoisonV1,
) -> CanonicalCoveredPhysicalPoisonRejectionV1 {
    let (audit, rejection) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            canonical_covered_convergence_poison_fixture_v1(case, poison);
    let (typed_error, rejection_stage, rollback_byte_identical) =
        rejection.expect("targeted final nonclosure must propagate from the stable final map");
    let solve = audit.solves.last();
    CanonicalCoveredPhysicalPoisonRejectionV1 {
        typed_error,
        rejection_stage,
        rollback_byte_identical,
        completed_final_envelope_count: solve
            .map_or(0, |value| value.completed_final_envelope_count),
        accepted_parent_publication_count: audit.accepted_parent_publication_count,
        charged_map_attempt_count: solve.map_or(0, |value| value.charged_map_attempt_count),
        validated_physical_endpoint_count: solve
            .map_or(0, |value| value.validated_physical_endpoint_count),
        validated_iteration_endpoint_count: solve
            .map_or(0, |value| value.validated_iteration_endpoint_count),
        validated_final_physical_endpoint_count: solve
            .map_or(0, |value| value.validated_final_physical_endpoint_count),
        final_constructor_attempt_count: solve
            .map_or(0, |value| value.final_constructor_attempt_count),
        v8_receipt_constructor_attempt_count: solve
            .map_or(0, |value| value.v8_receipt_constructor_attempt_count),
        vegetation_persistent_constructor_attempt_count: solve.map_or(0, |value| {
            value.vegetation_persistent_constructor_attempt_count
        }),
        vegetation_material_constructor_attempt_count: solve.map_or(0, |value| {
            value.vegetation_material_constructor_attempt_count
        }),
        biogeochemistry_constructor_attempt_count: solve
            .map_or(0, |value| value.biogeochemistry_constructor_attempt_count),
        ending_joint_constructor_attempt_count: solve
            .map_or(0, |value| value.ending_joint_constructor_attempt_count),
        complete_owner_set_constructor_attempt_count: solve.map_or(0, |value| {
            value.complete_owner_set_constructor_attempt_count
        }),
        restart_owner_constructor_attempt_count: solve
            .map_or(0, |value| value.restart_owner_constructor_attempt_count),
        publication_support_constructor_attempt_count: solve.map_or(0, |value| {
            value.publication_support_constructor_attempt_count
        }),
        map_local_publication_attempt_count: audit.map_local_publication_attempt_count,
        charged_roles: solve.map_or_else(Vec::new, |solve| solve.charged_roles.clone()),
        validated_pending_adjudication_count: solve
            .map_or(0, |solve| solve.validated_pending_adjudication_count),
        history_disposition_count: solve.map_or(0, |solve| solve.history_disposition_count),
        dependent_rejection_disposition_count: solve
            .map_or(0, |solve| solve.dependent_rejection_disposition_count),
        final_disposition_count: solve.map_or(0, |solve| solve.final_disposition_count),
    }
}

#[cfg(test)]
#[test]
fn canonical_covered_stable_two_map_chronology_is_physically_reachable() {
    use CanonicalCoveredAuditRoleV1 as Role;

    let (stable_panicked, stable) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            run_equilibrium_with_canonical_covered_role_audit();
    assert!(!stable_panicked, "stable covered solve fixture panicked");
    assert!(!stable.solves.is_empty(), "missing stable covered solve");
    for solve in &stable.solves {
        assert!(solve.completed, "stable covered solve did not complete");
        assert_eq!(
            solve.charged_roles,
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
            "stable support must consume the second charged map as the final disposition",
        );
        assert_eq!(solve.charged_map_attempt_count, 2);
        assert_eq!(solve.validated_physical_endpoint_count, 2);
        assert_eq!(solve.validated_pending_adjudication_count, 1);
        assert_eq!(solve.history_disposition_count, 0);
        assert_eq!(solve.dependent_rejection_disposition_count, 0);
        assert_eq!(solve.final_disposition_count, 1);
        assert_eq!(solve.final_constructor_attempt_count, 1);
        assert_eq!(solve.completed_final_envelope_count, 1);
    }
}

#[cfg(test)]
#[test]
fn canonical_covered_two_map_adjudication_failures_never_replay_or_fall_through() {
    use CanonicalCoveredAuditRoleV1 as Role;
    use CanonicalCoveredErrorClassV1 as ErrorClass;
    use CanonicalCoveredPhysicalParityCaseV1 as Case;
    use CanonicalCoveredPhysicalParityPoisonV1 as Poison;
    use CanonicalCoveredRejectionStageV1 as Stage;

    let target = CanonicalCoveredPoisonTargetV1 {
        role: Role::FixedPointAdjudication,
        ordinal: 1,
    };
    for (poison, expected_error, expected_stage, expected_counts, expected_dispositions) in [
        (
            Poison::Precipitation,
            ErrorClass::Stage3PrecipitationCustody,
            Stage::Physical,
            (2, 1, 0),
            (0, 0, 0, 0),
        ),
        (
            Poison::V8Persistent,
            ErrorClass::V8Vegetation,
            Stage::V8Persistent,
            (2, 2, 1),
            (1, 0, 0, 1),
        ),
    ] {
        let rejection = canonical_covered_targeted_failure_for_test(Case::Ordinary, poison, target);
        assert_eq!(rejection.typed_error, expected_error, "{poison:?}");
        assert_eq!(rejection.rejection_stage, expected_stage, "{poison:?}");
        assert_eq!(
            (
                rejection.charged_map_attempt_count,
                rejection.validated_physical_endpoint_count,
                rejection.final_constructor_attempt_count,
            ),
            expected_counts,
            "{poison:?}",
        );
        let expected_endpoint_split = if poison == Poison::Precipitation {
            (1, 0)
        } else {
            (1, 1)
        };
        assert_eq!(
            (
                rejection.validated_iteration_endpoint_count,
                rejection.validated_final_physical_endpoint_count,
            ),
            expected_endpoint_split,
            "{poison:?}",
        );
        assert_eq!(
            (
                rejection.validated_pending_adjudication_count,
                rejection.history_disposition_count,
                rejection.dependent_rejection_disposition_count,
                rejection.final_disposition_count,
            ),
            expected_dispositions,
            "{poison:?}",
        );
        assert_eq!(
            rejection.charged_roles,
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
            "the failed adjudication must not replay or charge a later map",
        );
        assert_eq!(rejection.completed_final_envelope_count, 0, "{poison:?}");
        assert_eq!(
            rejection.map_local_publication_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(rejection.accepted_parent_publication_count, 0, "{poison:?}");
        assert!(rejection.rollback_byte_identical, "{poison:?}");
    }

    let poison = CanonicalCoveredFinalConvergencePoisonV1::DependentNonclosure;
    for (case, expected_counts, expected_dispositions, expected_roles) in [
        (
            Case::Ordinary,
            (2, 2, 0),
            (1, 0, 1, 0),
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
        ),
        (
            Case::NativeMixedPhase,
            (3, 3, 0),
            (2, 1, 1, 0),
            vec![
                (Role::Initial, 0),
                (Role::FixedPointAdjudication, 1),
                (Role::MultisecantAdjudication(1), 2),
            ],
        ),
    ] {
        let rejection = canonical_covered_targeted_final_nonclosure_for_test(case, poison);
        assert_eq!(
            rejection.typed_error,
            ErrorClass::AdaptiveRefinement,
            "{poison:?}"
        );
        assert_eq!(
            rejection.rejection_stage,
            Stage::AdjudicationConvergence,
            "{poison:?}"
        );
        assert_eq!(
            (
                rejection.charged_map_attempt_count,
                rejection.validated_physical_endpoint_count,
                rejection.final_constructor_attempt_count,
            ),
            expected_counts,
            "{poison:?}",
        );
        assert_eq!(
            (
                rejection.validated_iteration_endpoint_count,
                rejection.validated_final_physical_endpoint_count,
            ),
            (expected_counts.0 - 1, 0),
            "{poison:?}",
        );
        assert_eq!(
            (
                rejection.validated_pending_adjudication_count,
                rejection.history_disposition_count,
                rejection.dependent_rejection_disposition_count,
                rejection.final_disposition_count,
            ),
            expected_dispositions,
        );
        assert_eq!(rejection.charged_roles, expected_roles);
        assert_eq!(rejection.completed_final_envelope_count, 0, "{poison:?}");
        assert_eq!(
            rejection.map_local_publication_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(rejection.accepted_parent_publication_count, 0, "{poison:?}");
        assert!(rejection.rollback_byte_identical, "{poison:?}");
    }
}

#[cfg(test)]
#[test]
fn canonical_covered_outer_nonclosure_consumes_pending_map_to_history() {
    use CanonicalCoveredAuditRoleV1 as Role;

    let (panicked, audit) =
        crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
            run_transient_with_canonical_covered_role_audit(Some(
                CanonicalCoveredFinalConvergencePoisonV1::OuterNonclosure,
            ));
    assert!(
        !panicked,
        "outer nonclosure must enter authentic history rather than reject or replay a final map",
    );
    let solve = audit.solves.last().expect("outer-nonclosure solve audit");
    assert!(solve.completed);
    assert!((3..=7).contains(&solve.charged_map_attempt_count));
    assert_eq!(
        solve.validated_physical_endpoint_count,
        solve.charged_map_attempt_count,
    );
    assert_eq!(
        solve.validated_pending_adjudication_count + 1,
        solve.charged_map_attempt_count,
    );
    assert_eq!(
        solve.charged_roles.get(..3),
        Some(
            [
                (Role::Initial, 0),
                (Role::FixedPointAdjudication, 1),
                (Role::MultisecantAdjudication(1), 2),
            ]
            .as_slice(),
        ),
    );
    assert_eq!(
        solve.history_disposition_count + 2,
        solve.charged_map_attempt_count,
    );
    assert_eq!(solve.dependent_rejection_disposition_count, 0);
    assert_eq!(solve.final_disposition_count, 1);
    assert_eq!(solve.final_constructor_attempt_count, 1);
    assert_eq!(solve.completed_final_envelope_count, 1);
    assert_eq!(audit.map_local_publication_attempt_count, 0);
}

#[cfg(test)]
#[test]
fn canonical_covered_invalid_finite_multisecant_proposals_reject_before_next_charge() {
    use CanonicalCoveredAuditRoleV1 as Role;

    for poison in [
        CanonicalCoveredMultisecantProposalPoisonV1::ExtremeFiniteDepth,
        CanonicalCoveredMultisecantProposalPoisonV1::FiniteDensityAboveCap,
        CanonicalCoveredMultisecantProposalPoisonV1::FiniteAboveFreezingTemperature,
    ] {
        let (panicked, audit, rejection) =
            crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
                run_transient_with_canonical_covered_multisecant_poison_audit(
                    CanonicalCoveredFinalConvergencePoisonV1::OuterNonclosure,
                    poison,
                );
        assert!(
            panicked,
            "{poison:?} must retain its typed adaptive rejection"
        );
        assert_eq!(
            rejection,
            Some((
                CanonicalCoveredErrorClassV1::AdaptiveRefinement,
                CanonicalCoveredRejectionStageV1::MultisecantProposal,
                false,
            )),
            "{poison:?}",
        );
        let solve = audit.solves.last().expect("proposal rejection solve audit");
        assert!(solve.terminated, "{poison:?}");
        assert!(!solve.completed, "{poison:?}");
        assert_eq!(
            solve.charged_roles,
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
            "{poison:?}: complete proposal validation must precede the successor map charge",
        );
        assert_eq!(solve.charged_map_attempt_count, 2, "{poison:?}");
        assert_eq!(solve.validated_physical_endpoint_count, 2, "{poison:?}");
        assert_eq!(solve.validated_iteration_endpoint_count, 2, "{poison:?}");
        assert_eq!(
            solve.validated_final_physical_endpoint_count, 0,
            "{poison:?}"
        );
        assert_eq!(solve.validated_pending_adjudication_count, 1, "{poison:?}");
        assert_eq!(solve.history_disposition_count, 1, "{poison:?}");
        assert_eq!(solve.dependent_rejection_disposition_count, 0, "{poison:?}");
        assert_eq!(solve.final_disposition_count, 0, "{poison:?}");
        assert_eq!(solve.final_constructor_attempt_count, 0, "{poison:?}");
        assert_eq!(solve.completed_final_envelope_count, 0, "{poison:?}");
        assert_eq!(solve.v8_receipt_constructor_attempt_count, 0, "{poison:?}");
        assert_eq!(
            solve.vegetation_persistent_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.vegetation_material_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.biogeochemistry_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.ending_joint_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.complete_owner_set_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.restart_owner_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(
            solve.publication_support_constructor_attempt_count, 0,
            "{poison:?}"
        );
        assert_eq!(audit.map_local_publication_attempt_count, 0, "{poison:?}");
        assert_eq!(audit.accepted_parent_publication_count, 0, "{poison:?}");
    }
}

#[cfg(test)]
#[test]
fn canonical_covered_convergence_poison_is_execution_thread_scoped() {
    use CanonicalCoveredAuditRoleV1 as Role;

    let (outer, stable) = std::thread::scope(|scope| {
        let outer = scope.spawn(|| {
            crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
                run_transient_with_canonical_covered_role_audit(Some(
                    CanonicalCoveredFinalConvergencePoisonV1::OuterNonclosure,
                ))
        });
        let stable = scope.spawn(|| {
            crate::v9_real_consumer_shadow::tests::adaptive_production_path_coverage::
                run_equilibrium_with_canonical_covered_role_audit()
        });
        (
            outer.join().expect("parallel outer-poison audit"),
            stable.join().expect("parallel unpoisoned audit"),
        )
    });
    assert!(
        !outer.0,
        "scoped outer poison rejected its production solve"
    );
    let outer = outer.1.solves.last().expect("scoped outer solve");
    assert_eq!(
        outer.charged_roles.get(..3),
        Some(
            [
                (Role::Initial, 0),
                (Role::FixedPointAdjudication, 1),
                (Role::MultisecantAdjudication(1), 2),
            ]
            .as_slice(),
        ),
    );
    assert!(outer.history_disposition_count >= 1);

    assert!(!stable.0, "parallel unpoisoned production solve failed");
    for solve in &stable.1.solves {
        assert_eq!(
            solve.charged_roles,
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
            "the outer poison escaped its worker execution thread",
        );
        assert_eq!(solve.history_disposition_count, 0);
        assert_eq!(solve.final_disposition_count, 1);
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CanonicalCoveredSolveAuditV1 {
    pub completed: bool,
    pub terminated: bool,
    pub charged_map_attempt_count: u32,
    pub charged_roles: Vec<(CanonicalCoveredAuditRoleV1, u32)>,
    pub validated_physical_endpoint_count: u32,
    pub validated_iteration_endpoint_count: u32,
    pub validated_final_physical_endpoint_count: u32,
    pub validated_pending_adjudication_count: u32,
    pub history_disposition_count: u32,
    pub dependent_rejection_disposition_count: u32,
    pub final_disposition_count: u32,
    pub final_constructor_attempt_count: u32,
    pub completed_final_envelope_count: u32,
    pub v8_receipt_constructor_attempt_count: u32,
    pub vegetation_persistent_constructor_attempt_count: u32,
    pub vegetation_material_constructor_attempt_count: u32,
    pub biogeochemistry_constructor_attempt_count: u32,
    pub ending_joint_constructor_attempt_count: u32,
    pub complete_owner_set_constructor_attempt_count: u32,
    pub restart_owner_constructor_attempt_count: u32,
    pub publication_support_constructor_attempt_count: u32,
}

#[cfg(test)]
impl CanonicalCoveredSolveAuditV1 {
    fn new() -> Self {
        Self {
            completed: false,
            terminated: false,
            charged_map_attempt_count: 0,
            charged_roles: Vec::new(),
            validated_physical_endpoint_count: 0,
            validated_iteration_endpoint_count: 0,
            validated_final_physical_endpoint_count: 0,
            validated_pending_adjudication_count: 0,
            history_disposition_count: 0,
            dependent_rejection_disposition_count: 0,
            final_disposition_count: 0,
            final_constructor_attempt_count: 0,
            completed_final_envelope_count: 0,
            v8_receipt_constructor_attempt_count: 0,
            vegetation_persistent_constructor_attempt_count: 0,
            vegetation_material_constructor_attempt_count: 0,
            biogeochemistry_constructor_attempt_count: 0,
            ending_joint_constructor_attempt_count: 0,
            complete_owner_set_constructor_attempt_count: 0,
            restart_owner_constructor_attempt_count: 0,
            publication_support_constructor_attempt_count: 0,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CanonicalCoveredMapRoleAuditV1 {
    pub solves: Vec<CanonicalCoveredSolveAuditV1>,
    pub map_local_publication_attempt_count: u32,
    pub successful_history_append_count: u32,
    pub accepted_parent_publication_count: u32,
}

#[cfg(test)]
std::thread_local! {
    static CANONICAL_COVERED_MAP_ROLE_AUDIT_V1: std::cell::RefCell<Option<CanonicalCoveredMapRoleAuditV1>> = const { std::cell::RefCell::new(None) };
    static CANONICAL_COVERED_PARITY_POISON_V1: std::cell::Cell<Option<CanonicalCoveredPhysicalParityPoisonV1>> = const { std::cell::Cell::new(None) };
    static CANONICAL_COVERED_PARITY_POISON_TARGET_V1: std::cell::Cell<Option<CanonicalCoveredPoisonTargetV1>> = const { std::cell::Cell::new(None) };
    static CANONICAL_COVERED_CURRENT_MAP_V1: std::cell::Cell<Option<CanonicalCoveredPoisonTargetV1>> = const { std::cell::Cell::new(None) };
    static CANONICAL_COVERED_PARITY_REJECTION_V1: std::cell::RefCell<Option<(CanonicalCoveredErrorClassV1, CanonicalCoveredRejectionStageV1, bool)>> = const { std::cell::RefCell::new(None) };
    static CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1: std::cell::Cell<Option<CanonicalCoveredFinalConstructorStageV1>> = const { std::cell::Cell::new(None) };
    static CANONICAL_COVERED_SUCCESSFUL_HISTORY_APPEND_DIGESTS_V1: std::cell::RefCell<Vec<Digest32>> = const { std::cell::RefCell::new(Vec::new()) };
    static CANONICAL_COVERED_FINAL_CONVERGENCE_POISON_V1: std::cell::Cell<Option<CanonicalCoveredFinalConvergencePoisonV1>> = const { std::cell::Cell::new(None) };
    static CANONICAL_COVERED_MULTISECANT_PROPOSAL_POISON_V1: std::cell::Cell<Option<CanonicalCoveredMultisecantProposalPoisonV1>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(crate) struct CanonicalCoveredFinalConvergencePoisonGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredFinalConvergencePoisonGuardV1 {
    fn drop(&mut self) {
        CANONICAL_COVERED_FINAL_CONVERGENCE_POISON_V1.with(|slot| slot.set(None));
    }
}

#[cfg(test)]
pub(crate) fn force_canonical_covered_final_convergence_poison_for_test(
    poison: CanonicalCoveredFinalConvergencePoisonV1,
) -> CanonicalCoveredFinalConvergencePoisonGuardV1 {
    CANONICAL_COVERED_FINAL_CONVERGENCE_POISON_V1.with(|slot| slot.set(Some(poison)));
    CanonicalCoveredFinalConvergencePoisonGuardV1
}

#[cfg(test)]
fn take_canonical_covered_final_convergence_poison_v1(
) -> Option<CanonicalCoveredFinalConvergencePoisonV1> {
    CANONICAL_COVERED_FINAL_CONVERGENCE_POISON_V1.with(|slot| slot.replace(None))
}

#[cfg(test)]
pub(crate) struct CanonicalCoveredMultisecantProposalPoisonGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredMultisecantProposalPoisonGuardV1 {
    fn drop(&mut self) {
        CANONICAL_COVERED_MULTISECANT_PROPOSAL_POISON_V1.with(|slot| slot.set(None));
    }
}

#[cfg(test)]
pub(crate) fn force_canonical_covered_multisecant_proposal_poison_for_test(
    poison: CanonicalCoveredMultisecantProposalPoisonV1,
) -> CanonicalCoveredMultisecantProposalPoisonGuardV1 {
    CANONICAL_COVERED_MULTISECANT_PROPOSAL_POISON_V1.with(|slot| slot.set(Some(poison)));
    CanonicalCoveredMultisecantProposalPoisonGuardV1
}

#[cfg(test)]
fn take_canonical_covered_multisecant_proposal_poison_v1(
) -> Option<CanonicalCoveredMultisecantProposalPoisonV1> {
    CANONICAL_COVERED_MULTISECANT_PROPOSAL_POISON_V1.with(|slot| slot.replace(None))
}

#[cfg(test)]
struct CanonicalCoveredCompleteOwnerScopeGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredCompleteOwnerScopeGuardV1 {
    fn drop(&mut self) {
        CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1.with(|active| active.set(false));
    }
}

#[cfg(test)]
fn begin_canonical_covered_complete_owner_scope_v1() -> CanonicalCoveredCompleteOwnerScopeGuardV1 {
    CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1.with(|active| active.set(true));
    CanonicalCoveredCompleteOwnerScopeGuardV1
}

#[cfg(test)]
fn canonical_covered_complete_owner_set_boundary_v1() {
    if CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1.with(std::cell::Cell::get) {
        canonical_covered_final_constructor_boundary_v1(
            CanonicalCoveredFinalConstructorStageV1::CompleteOwnerSet,
        );
    }
}

#[cfg(test)]
pub(crate) struct CanonicalCoveredParityPoisonGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredParityPoisonGuardV1 {
    fn drop(&mut self) {
        CANONICAL_COVERED_PARITY_POISON_V1.with(|slot| slot.set(None));
        CANONICAL_COVERED_PARITY_POISON_TARGET_V1.with(|slot| slot.set(None));
        CANONICAL_COVERED_CURRENT_MAP_V1.with(|slot| slot.set(None));
    }
}

#[cfg(test)]
pub(crate) fn force_canonical_covered_parity_poison_for_test(
    poison: CanonicalCoveredPhysicalParityPoisonV1,
) -> CanonicalCoveredParityPoisonGuardV1 {
    CANONICAL_COVERED_PARITY_POISON_V1.with(|slot| slot.set(Some(poison)));
    CANONICAL_COVERED_PARITY_REJECTION_V1.with(|slot| *slot.borrow_mut() = None);
    CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1.with(|slot| slot.set(None));
    CanonicalCoveredParityPoisonGuardV1
}

#[cfg(test)]
pub(crate) fn force_canonical_covered_targeted_parity_poison_for_test(
    poison: CanonicalCoveredPhysicalParityPoisonV1,
    target: CanonicalCoveredPoisonTargetV1,
) -> CanonicalCoveredParityPoisonGuardV1 {
    let guard = force_canonical_covered_parity_poison_for_test(poison);
    CANONICAL_COVERED_PARITY_POISON_TARGET_V1.with(|slot| slot.set(Some(target)));
    guard
}

#[cfg(test)]
struct CanonicalCoveredCurrentMapGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredCurrentMapGuardV1 {
    fn drop(&mut self) {
        CANONICAL_COVERED_CURRENT_MAP_V1.with(|slot| slot.set(None));
    }
}

#[cfg(test)]
fn enter_canonical_covered_current_map_v1(
    role: CanonicalCoveredMapRoleV1,
    ordinal: u32,
) -> CanonicalCoveredCurrentMapGuardV1 {
    CANONICAL_COVERED_CURRENT_MAP_V1.with(|slot| {
        slot.set(Some(CanonicalCoveredPoisonTargetV1 {
            role: role.into(),
            ordinal,
        }));
    });
    CanonicalCoveredCurrentMapGuardV1
}

#[cfg(test)]
pub(crate) fn canonical_covered_current_map_for_test() -> Option<CanonicalCoveredPoisonTargetV1> {
    CANONICAL_COVERED_CURRENT_MAP_V1.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(crate) fn canonical_covered_parity_poison_selected_for_current_map_v1(
    poison: CanonicalCoveredPhysicalParityPoisonV1,
) -> bool {
    if canonical_covered_parity_poison_v1() != Some(poison) {
        return false;
    }
    let target = CANONICAL_COVERED_PARITY_POISON_TARGET_V1.with(std::cell::Cell::get);
    target.is_none() || target == CANONICAL_COVERED_CURRENT_MAP_V1.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(crate) fn canonical_covered_parity_poison_active_for_test() -> bool {
    CANONICAL_COVERED_PARITY_POISON_V1.with(|slot| slot.get().is_some())
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_parity_rollback_for_test(rollback: bool) {
    CANONICAL_COVERED_PARITY_REJECTION_V1.with(|slot| {
        if let Some(value) = slot.borrow_mut().as_mut() {
            value.2 = rollback;
        }
    });
}

#[cfg(test)]
pub(crate) fn canonical_covered_parity_poison_v1() -> Option<CanonicalCoveredPhysicalParityPoisonV1>
{
    CANONICAL_COVERED_PARITY_POISON_V1.with(std::cell::Cell::get)
}

#[cfg(test)]
fn canonical_covered_record_parity_rejection_v1(
    error: &DirectV11RealConsumerError,
    stage: CanonicalCoveredRejectionStageV1,
) {
    let class = match error {
        DirectV11RealConsumerError::CoupledTime(_) => CanonicalCoveredErrorClassV1::CoupledTime,
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
            DirectV9RealConsumerError::OwnerEnvelope(
                crate::land_surface_energy_shadow::CoveredV8OwnerEnvelopeError::Identity(_),
            ),
        )) => CanonicalCoveredErrorClassV1::V8OwnerIdentity,
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
            DirectV9RealConsumerError::OwnerEnvelope(
                crate::land_surface_energy_shadow::CoveredV8OwnerEnvelopeError::Vegetation(_),
            ),
        )) => CanonicalCoveredErrorClassV1::V8Vegetation,
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
            DirectV9RealConsumerError::OwnerEnvelope(
                crate::land_surface_energy_shadow::CoveredV8OwnerEnvelopeError::Biogeochemistry(_),
            ),
        )) => CanonicalCoveredErrorClassV1::V8Biogeochemistry,
        DirectV11RealConsumerError::Runtime(_) => CanonicalCoveredErrorClassV1::Runtime,
        DirectV11RealConsumerError::Vegetation(_) => CanonicalCoveredErrorClassV1::Vegetation,
        DirectV11RealConsumerError::Identity(_) => CanonicalCoveredErrorClassV1::Identity,
        DirectV11RealConsumerError::AdaptiveRefinement(_) => {
            CanonicalCoveredErrorClassV1::AdaptiveRefinement
        }
        DirectV11RealConsumerError::CoveredBoundary(_) => {
            CanonicalCoveredErrorClassV1::CoveredBoundary
        }
        DirectV11RealConsumerError::Stage3PrecipitationCustody(_) => {
            CanonicalCoveredErrorClassV1::Stage3PrecipitationCustody
        }
        DirectV11RealConsumerError::Stage3SnowSoilHeatCustody(_) => {
            CanonicalCoveredErrorClassV1::Stage3SnowSoilHeatCustody
        }
        DirectV11RealConsumerError::Stage3(_) => CanonicalCoveredErrorClassV1::Stage3,
        DirectV11RealConsumerError::Serialization(_) => CanonicalCoveredErrorClassV1::Serialization,
        DirectV11RealConsumerError::SurfaceLiquidReplay(_) => {
            CanonicalCoveredErrorClassV1::SurfaceLiquidReplay
        }
        DirectV11RealConsumerError::ZeroDurationSnowLiquid(_) => {
            CanonicalCoveredErrorClassV1::ZeroDurationSnowLiquid
        }
        DirectV11RealConsumerError::OpenSnowLowerBoundaryDomain { .. } => {
            CanonicalCoveredErrorClassV1::OpenSnowLowerBoundaryDomain
        }
        DirectV11RealConsumerError::ComponentCarrierReferenceFluxCustody { .. } => {
            CanonicalCoveredErrorClassV1::ComponentCarrierReferenceFluxCustody
        }
    };
    CANONICAL_COVERED_PARITY_REJECTION_V1
        .with(|slot| *slot.borrow_mut() = Some((class, stage, false)));
}

#[cfg(test)]
fn canonical_covered_parity_rejection_v1(
    error: DirectV11RealConsumerError,
    stage: CanonicalCoveredRejectionStageV1,
) -> DirectV11RealConsumerError {
    canonical_covered_record_parity_rejection_v1(&error, stage);
    error
}

#[cfg(test)]
pub(crate) fn take_canonical_covered_parity_rejection_for_test() -> Option<(
    CanonicalCoveredErrorClassV1,
    CanonicalCoveredRejectionStageV1,
    bool,
)> {
    CANONICAL_COVERED_PARITY_REJECTION_V1.with(|slot| slot.borrow_mut().take())
}

#[cfg(test)]
pub(crate) struct CanonicalCoveredMapRoleAuditGuardV1;

#[cfg(test)]
pub(crate) fn begin_canonical_covered_map_role_audit_v1() -> CanonicalCoveredMapRoleAuditGuardV1 {
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        *audit.borrow_mut() = Some(CanonicalCoveredMapRoleAuditV1::default());
    });
    CANONICAL_COVERED_SUCCESSFUL_HISTORY_APPEND_DIGESTS_V1
        .with(|digests| digests.borrow_mut().clear());
    CanonicalCoveredMapRoleAuditGuardV1
}

#[cfg(test)]
pub(crate) fn take_canonical_covered_map_role_audit_v1() -> CanonicalCoveredMapRoleAuditV1 {
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_successful_history_append_v1(receipt_sha256: Digest32) {
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.successful_history_append_count =
                audit.successful_history_append_count.saturating_add(1);
            CANONICAL_COVERED_SUCCESSFUL_HISTORY_APPEND_DIGESTS_V1
                .with(|digests| digests.borrow_mut().push(receipt_sha256));
        }
    });
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_accepted_parent_adoption_v1(
    adopted_support_sha256: Digest32,
) {
    let matched_append = CANONICAL_COVERED_SUCCESSFUL_HISTORY_APPEND_DIGESTS_V1.with(|digests| {
        let mut digests = digests.borrow_mut();
        digests
            .iter()
            .rposition(|digest| *digest == adopted_support_sha256)
            .map(|index| digests.remove(index))
            .is_some()
    });
    if !matched_append {
        return;
    }
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.accepted_parent_publication_count =
                audit.accepted_parent_publication_count.saturating_add(1);
        }
    });
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_publication_retain_entry_v1() {
    CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1.with(|slot| {
        slot.set(Some(
            CanonicalCoveredFinalConstructorStageV1::PublicationSupport,
        ))
    });
    if !CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1.with(std::cell::Cell::get) {
        return;
    }
    canonical_covered_audit_update_v1(|solve| {
        solve.publication_support_constructor_attempt_count = solve
            .publication_support_constructor_attempt_count
            .saturating_add(1);
    });
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.map_local_publication_attempt_count =
                audit.map_local_publication_attempt_count.saturating_add(1);
        }
    });
}

#[cfg(test)]
fn canonical_covered_audit_update_v1(update: impl FnOnce(&mut CanonicalCoveredSolveAuditV1)) {
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        if audit.solves.is_empty() {
            audit.solves.push(CanonicalCoveredSolveAuditV1::new());
        }
        if let Some(solve) = audit.solves.last_mut() {
            update(solve);
        }
    });
}

#[cfg(test)]
struct CanonicalCoveredSolveTerminationGuardV1;

#[cfg(test)]
impl Drop for CanonicalCoveredSolveTerminationGuardV1 {
    fn drop(&mut self) {
        canonical_covered_audit_update_v1(|solve| {
            if !solve.completed {
                solve.terminated = true;
            }
        });
    }
}

#[cfg(test)]
fn begin_canonical_covered_solve_termination_guard_v1() -> CanonicalCoveredSolveTerminationGuardV1 {
    CanonicalCoveredSolveTerminationGuardV1
}

#[cfg(test)]
fn canonical_covered_audit_charge_v1(role: CanonicalCoveredMapRoleV1, ordinal: u32) {
    CANONICAL_COVERED_MAP_ROLE_AUDIT_V1.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        if audit
            .solves
            .last()
            .is_none_or(|solve| solve.completed || solve.terminated)
        {
            audit.solves.push(CanonicalCoveredSolveAuditV1::new());
        }
        if let Some(solve) = audit.solves.last_mut() {
            solve.charged_map_attempt_count = solve.charged_map_attempt_count.saturating_add(1);
            solve.charged_roles.push((role.into(), ordinal));
        }
    });
}

#[cfg(not(test))]
fn canonical_covered_audit_charge_v1(_: CanonicalCoveredMapRoleV1, _: u32) {}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalCoveredFinalConstructorStageV1 {
    V8Receipt,
    VegetationPersistent,
    VegetationMaterial,
    Biogeochemistry,
    EndingJoint,
    CompleteOwnerSet,
    RestartOwner,
    PublicationSupport,
}

#[cfg(test)]
pub(crate) fn canonical_covered_final_constructor_boundary_v1(
    stage: CanonicalCoveredFinalConstructorStageV1,
) {
    use CanonicalCoveredFinalConstructorStageV1 as Stage;
    if stage == Stage::RestartOwner
        && !CANONICAL_COVERED_COMPLETE_OWNER_SCOPE_V1.with(std::cell::Cell::get)
    {
        return;
    }
    CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1.with(|slot| slot.set(Some(stage)));
    canonical_covered_audit_update_v1(|solve| match stage {
        Stage::V8Receipt => {
            solve.final_constructor_attempt_count =
                solve.final_constructor_attempt_count.saturating_add(1);
            solve.v8_receipt_constructor_attempt_count =
                solve.v8_receipt_constructor_attempt_count.saturating_add(1);
        }
        Stage::VegetationPersistent => {
            solve.vegetation_persistent_constructor_attempt_count = solve
                .vegetation_persistent_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::VegetationMaterial => {
            solve.vegetation_material_constructor_attempt_count = solve
                .vegetation_material_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::Biogeochemistry => {
            solve.biogeochemistry_constructor_attempt_count = solve
                .biogeochemistry_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::EndingJoint => {
            solve.ending_joint_constructor_attempt_count = solve
                .ending_joint_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::CompleteOwnerSet => {
            solve.complete_owner_set_constructor_attempt_count = solve
                .complete_owner_set_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::RestartOwner => {
            solve.restart_owner_constructor_attempt_count = solve
                .restart_owner_constructor_attempt_count
                .saturating_add(1);
        }
        Stage::PublicationSupport => {
            solve.publication_support_constructor_attempt_count = solve
                .publication_support_constructor_attempt_count
                .saturating_add(1);
        }
    });
}

#[cfg(test)]
pub(crate) fn canonical_covered_final_validation_boundary_v1(
    stage: CanonicalCoveredFinalConstructorStageV1,
) {
    CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1.with(|slot| slot.set(Some(stage)));
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_outer_rejection_for_test(
    error: &DirectV11RealConsumerError,
) {
    let Some(stage) = canonical_covered_last_rejection_stage_v1() else {
        return;
    };
    canonical_covered_record_parity_rejection_v1(error, stage);
}

#[cfg(test)]
fn canonical_covered_last_rejection_stage_v1() -> Option<CanonicalCoveredRejectionStageV1> {
    match CANONICAL_COVERED_LAST_CONSTRUCTOR_STAGE_V1.with(std::cell::Cell::get) {
        Some(CanonicalCoveredFinalConstructorStageV1::V8Receipt) => {
            Some(CanonicalCoveredRejectionStageV1::V8EnvelopeValidation)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::VegetationPersistent) => {
            Some(CanonicalCoveredRejectionStageV1::V8Persistent)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::VegetationMaterial) => {
            Some(CanonicalCoveredRejectionStageV1::V8VegetationCandidate)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::Biogeochemistry) => {
            Some(CanonicalCoveredRejectionStageV1::V8Biogeochemistry)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::EndingJoint) => {
            Some(CanonicalCoveredRejectionStageV1::EndingJoint)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::CompleteOwnerSet) => {
            Some(CanonicalCoveredRejectionStageV1::CompleteOwnerSet)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::RestartOwner) => {
            Some(CanonicalCoveredRejectionStageV1::RestartHistory)
        }
        Some(CanonicalCoveredFinalConstructorStageV1::PublicationSupport) => {
            Some(CanonicalCoveredRejectionStageV1::PublicationSupport)
        }
        None => None,
    }
}

#[cfg(test)]
pub(crate) fn record_canonical_covered_outer_attachment_rejection_for_test(
    error: &crate::snow_stage3_v11_attachment::DirectSnowStage3V11AttachmentError,
) {
    use crate::snow_stage3_v11_attachment::DirectSnowStage3V11AttachmentError as Attachment;
    match error {
        Attachment::Owner(error) => record_canonical_covered_outer_rejection_for_test(error),
        Attachment::V11(openwepp_vegetation::v11::V11ExecutionError::Executor(error)) => {
            record_canonical_covered_outer_rejection_for_test(error);
        }
        Attachment::AdaptiveTrial { source, .. } => {
            record_canonical_covered_outer_attachment_rejection_for_test(source);
        }
        Attachment::CoupledTime(_) => {
            if let Some(stage) = canonical_covered_last_rejection_stage_v1() {
                CANONICAL_COVERED_PARITY_REJECTION_V1.with(|slot| {
                    *slot.borrow_mut() =
                        Some((CanonicalCoveredErrorClassV1::CoupledTime, stage, false));
                });
            }
        }
        _ => {}
    }
}
