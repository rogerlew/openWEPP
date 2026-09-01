const OUTCOME_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

#[inline(never)]
fn covered_boxed_execution_v1<T, E>(execute: impl FnOnce() -> Result<T, E>) -> Result<Box<T>, E> {
    execute().map(Box::new)
}

include!("open_snow_receipt_reseal_helpers.rs");
include!("terminal_composition.rs");
include!("stable_monotone.rs");

include!("open_snow_convergence_metrics.rs");

#[allow(clippy::too_many_arguments)]
fn record_covered_limiter_sample_v1(
    support: TimeSupport,
    iteration: usize,
    stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1,
    convergence: (bool, bool, bool, bool),
    deltas: (f64, f64, f64, f64, f64),
) {
    crate::snow_stage3_v11_attachment::record_covered_fixed_point_limiter_sample_v1(
        crate::snow_stage3_v11_attachment::CoveredFixedPointLimiterSampleV1 {
            support,
            iteration,
            stage,
            lse_converged: convergence.0,
            stage3_converged: convergence.1,
            soil_converged: convergence.2,
            boundary_converged: convergence.3,
            lse_max_normalized_delta_bits: deltas.0.to_bits(),
            stage3_max_normalized_delta_bits: deltas.1.to_bits(),
            soil_enthalpy_max_normalized_delta_bits: deltas.2.to_bits(),
            soil_temperature_max_normalized_delta_bits: deltas.3.to_bits(),
            boundary_max_normalized_delta_bits: deltas.4.to_bits(),
        },
    );
}

include!("open_snow_physical_support.rs");

impl crate::v11_vegetation_consumer::DirectV11ImportedStack
    for DirectV11SnowCoveredRealConsumerStack<'_>
{
    type Error = DirectV11RealConsumerError;

    #[allow(clippy::too_many_lines)]
    fn execute_imported_v10_stack(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error> {
        if input.configuration != self.beginning.vegetation_configuration
            || input.beginning != self.beginning.vegetation_state
            || input.duration_s_bits != input.support.duration_s_bits()
            || self.interval.lse_forcing.interval_s.to_bits() != input.duration_s_bits
            || self.snow_surface_forcing_by_destination.is_empty()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered support / DirectV10 beginning join",
            ));
        }
        if self.terminal_physical_reuse_seed.is_some() {
            return self.execute_terminal_physical_reuse(input);
        }
        if self.ordinary_physical_reuse_seed.is_some() {
            return self.execute_ordinary_physical_reuse(input);
        }
        if let Some(endpoint) = self.precomputed_terminal_accepted.take() {
            return self.execute_precomputed_terminal_accepted_endpoint(input, endpoint);
        }
        let interval_s = f64::from_bits(input.duration_s_bits);
        for stage3_forcing in self.stage3_forcing_by_lane.values() {
            if stage3_forcing.duration_seconds.to_bits() != interval_s.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3/V11 support duration",
                ));
            }
        }
        let (_, initial_vegetation_state) = project_v9_runtime_to_v8(
            &self.beginning.inner.vegetation_configuration,
            &self.beginning.inner.vegetation_state,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::V9(error),
            ))
        })?;
        let terminal_events = std::cell::RefCell::new(BTreeMap::new());
        let evaluate_stage3 =
            |destination_receipts: &BTreeMap<(OfeId, TileId), Digest32>,
             boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
             final_lane_receipts: Option<&BTreeMap<u32, LaneStage3BoundaryReceiptV1>>,
             snow_soil_receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
             unpublished_cn_trial_operands: Option<
                &BTreeMap<u32, CoveredPhaseConsistentCnTrialOperandV1>,
            >,
             coupled_evaluation_kind: Option<CoveredPhaseConsistentPhysicalEvaluationKindV1>,
             precipitation_sets: &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>| {
                terminal_events.borrow_mut().clear();
                let terms = self.lane_stage3_terms_from_boundaries(
                    destination_receipts,
                    boundaries,
                    interval_s,
                )?;
                let mut ending_stage3 = self.stage3_beginning_by_lane.clone();
                let mut outcome_diagnostics_by_lane = BTreeMap::new();
                let mut phase_support_images_by_lane = BTreeMap::new();
                for lane_id in terms.keys() {
                    let beginning = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                        DirectV11RealConsumerError::Identity("active Stage-3 beginning lane"),
                    )?;
                    let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                        DirectV11RealConsumerError::Identity("covered Stage-3 input lane"),
                    )?;
                    let mut stage3_forcing =
                        self.stage3_forcing_by_lane.get(lane_id).copied().ok_or(
                            DirectV11RealConsumerError::Identity("covered Stage-3 forcing lane"),
                        )?;
                    let precipitation_set = precipitation_sets.get(lane_id).ok_or(
                        DirectV11RealConsumerError::Identity(
                            "covered precipitation parcel-set lane",
                        ),
                    )?;
                    let (precipitation_mass, precipitation_advection_j_m2) =
                        reconstruct_precipitation_mass_and_advected_heat(precipitation_set)
                            .map_err(|error| {
                                DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                            })?;
                    let mut liquid_mass = 0.0;
                    let mut solid_mass = 0.0;
                    for parcel in &precipitation_set.parcels {
                        let fraction = precipitation_set
                            .destinations
                            .get(parcel.destination_topology_index as usize)
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "precipitation destination index",
                            ))?
                            .fraction_of_ofe;
                        match parcel.phase {
                            Stage3PrecipitationPhaseV1::Solid => {
                                solid_mass += fraction * parcel.mass_kg_m2_tile_ground;
                            }
                            Stage3PrecipitationPhaseV1::Liquid => {
                                liquid_mass += fraction * parcel.mass_kg_m2_tile_ground;
                            }
                        }
                    }
                    if !precipitation_mass.is_finite() {
                        return Err(DirectV11RealConsumerError::Identity(
                            "precipitation phase/mass same-set join",
                        ));
                    }
                    let (rain_m, snowfall_m, active_precipitation_m) =
                        reconstruct_stage3_phase_forcing_v1(liquid_mass, solid_mass)?;
                    stage3_forcing.forcing.rain_m = rain_m;
                    stage3_forcing.forcing.snowfall_m = snowfall_m;
                    stage3_forcing.forcing.active_precipitation_m = active_precipitation_m;
                    let lane_terms =
                        terms
                            .get(lane_id)
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "covered Stage-3 lane terms",
                            ))?;
                    let beginning_stage3_digest =
                        if beginning.layers.is_empty() && stage3_forcing.forcing.snowfall_m > 0.0 {
                            digest_bytes(
                                &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)
                                    .map_err(|_| {
                                        DirectV11RealConsumerError::Identity(
                                            "covered reappearance beginning state",
                                        )
                                    })?,
                            )
                        } else if crate::hydrology::stage3_is_terminal_event_domain(beginning) {
                            Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(beginning)
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "covered beginning active-volume surface",
                                    )
                                })?
                                .beginning_stage3_state_sha256
                        } else {
                            Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning)
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "covered beginning active-volume surface",
                                    )
                                })?
                                .beginning_stage3_state_sha256
                        };
                    let (
                        sensible_to_canopy_air_w_m2,
                        vapor_to_canopy_air_kg_m2_s,
                        latent_energy_to_canopy_air_j_m2,
                        snow_absorbed_shortwave_w_m2,
                        snow_net_longwave_w_m2,
                        latent_heat_j_kg,
                        identity,
                    ) = if let Some(receipts) = final_lane_receipts {
                        let receipt =
                            receipts
                                .get(lane_id)
                                .ok_or(DirectV11RealConsumerError::Identity(
                                    "covered final lane boundary receipt",
                                ))?;
                        if receipt.aggregate_sensible_to_canopy_air_w_m2.to_bits()
                            != lane_terms.sensible_to_canopy_air_w_m2.to_bits()
                            || receipt.aggregate_vapor_to_canopy_air_kg_m2_s.to_bits()
                                != lane_terms.vapor_to_canopy_air_kg_m2_s.to_bits()
                            || receipt.aggregate_latent_energy_to_canopy_air_j_m2.to_bits()
                                != lane_terms.latent_energy_to_canopy_air_j_m2.to_bits()
                            || receipt.aggregate_snow_absorbed_shortwave_w_m2.to_bits()
                                != lane_terms.snow_absorbed_shortwave_w_m2.to_bits()
                            || receipt.aggregate_snow_net_longwave_w_m2.to_bits()
                                != lane_terms.snow_net_longwave_w_m2.to_bits()
                            || receipt.aggregate_snow_temperature_k.to_bits()
                                != lane_terms.snow_temperature_k.to_bits()
                            || receipt.aggregate_latent_heat_j_kg.to_bits()
                                != lane_terms.latent_heat_j_kg.to_bits()
                            || receipt.precipitation_parcel_set_sha256
                                != precipitation_set.receipt_sha256
                        {
                            return Err(DirectV11RealConsumerError::Identity(
                                "covered lane receipt boundary reconstruction",
                            ));
                        }
                        (
                            receipt.aggregate_sensible_to_canopy_air_w_m2,
                            receipt.aggregate_vapor_to_canopy_air_kg_m2_s,
                            receipt.aggregate_latent_energy_to_canopy_air_j_m2,
                            receipt.aggregate_snow_absorbed_shortwave_w_m2,
                            receipt.aggregate_snow_net_longwave_w_m2,
                            receipt.aggregate_latent_heat_j_kg,
                            Stage3BoundaryIdentity::Final {
                                provisional_carrier_receipt_sha256: receipt
                                    .provisional_carrier_receipt_sha256,
                                optical_receipt_sha256: receipt.optical_receipt_sha256,
                                reciprocal_longwave_receipt_sha256: receipt
                                    .reciprocal_longwave_receipt_sha256,
                                final_destination_receipt_sha256: receipt
                                    .final_destination_receipt_sha256,
                                final_lane_receipt_sha256: receipt.receipt_sha256,
                            },
                        )
                    } else {
                        (
                            lane_terms.sensible_to_canopy_air_w_m2,
                            lane_terms.vapor_to_canopy_air_kg_m2_s,
                            lane_terms.latent_energy_to_canopy_air_j_m2,
                            lane_terms.snow_absorbed_shortwave_w_m2,
                            lane_terms.snow_net_longwave_w_m2,
                            lane_terms.latent_heat_j_kg,
                            Stage3BoundaryIdentity::Provisional {
                                carrier_receipt_sha256: lane_terms
                                    .provisional_carrier_receipt_sha256,
                            },
                        )
                    };
                    let (sensible_into_snow_j_m2, vapor_into_snow_kg_m2, latent_into_snow_j_m2) =
                        outward_snow_fluxes_to_stage3(
                            sensible_to_canopy_air_w_m2,
                            vapor_to_canopy_air_kg_m2_s,
                            latent_energy_to_canopy_air_j_m2,
                            interval_s,
                        );
                    let boundary = Stage3SnowSurfaceBoundaryReceiptV1::try_new(
                        Stage3SnowSurfaceBoundaryReceiptInputs {
                            support: input.support,
                            sensible_energy_j_m2: sensible_into_snow_j_m2,
                            vapor_mass_kg_m2: vapor_into_snow_kg_m2,
                            latent_energy_j_m2: latent_into_snow_j_m2,
                            shortwave_energy_j_m2: snow_absorbed_shortwave_w_m2 * interval_s,
                            net_longwave_energy_j_m2: snow_net_longwave_w_m2 * interval_s,
                            precipitation_advection_j_m2,
                            // The reappearance microstep begins without a snow
                            // thermal node. Its canonical zero-duration domain
                            // creation therefore carries no snow--soil
                            // conduction; ordinary conduction starts with the
                            // next accepted microstep, when a represented bottom
                            // snow volume exists.
                            snow_soil_heat_j_m2: if let Some(kind) = coupled_evaluation_kind {
                                if final_lane_receipts.is_some() {
                                    return Err(DirectV11RealConsumerError::Identity(
                                        "phase-consistent CN trial final receipt alias",
                                    ));
                                }
                                covered_phase_consistent_cn_consumption_v1(
                                    kind,
                                    unpublished_cn_trial_operands
                                        .and_then(|operands| operands.get(lane_id)),
                                    snow_soil_receipts.get(lane_id),
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "phase-consistent typed CN consumption",
                                    )
                                })?
                                .snow_candidate_heat_j_m2_ofe_ground
                            } else {
                                match (
                                    unpublished_cn_trial_operands
                                        .and_then(|operands| operands.get(lane_id)),
                                    snow_soil_receipts.get(lane_id),
                                ) {
                                    (Some(trial), Some(receipt)) => {
                                        if final_lane_receipts.is_some() {
                                            return Err(DirectV11RealConsumerError::Identity(
                                                "phase-consistent CN trial final receipt alias",
                                            ));
                                        }
                                        trial.validate_against(receipt)?;
                                        trial.snow_candidate_heat_j_m2_ofe_ground
                                    }
                                    (Some(_), None) => {
                                        return Err(DirectV11RealConsumerError::Identity(
                                            "phase-consistent CN trial immutable receipt",
                                        ));
                                    }
                                    (None, Some(receipt)) => {
                                        receipt.snow_candidate_heat_j_m2_ofe_ground
                                    }
                                    (None, None)
                                        if beginning.layers.is_empty()
                                            && stage3_forcing.forcing.snowfall_m > 0.0 =>
                                    {
                                        0.0
                                    }
                                    (None, None) => {
                                        return Err(DirectV11RealConsumerError::Identity(
                                            "covered lane snow-soil heat receipt",
                                        ));
                                    }
                                }
                            },
                            latent_heat_j_kg,
                            beginning_stage3_state_sha256: beginning_stage3_digest,
                            identity,
                        },
                    )?;
                    let mut result = if self.terminal_endpoint_mode
                        && beginning.schema_version == 2
                        && beginning.terminal_event_model.is_some()
                    {
                        Wb11HydrologyKernel::evaluate_stage3_terminal_support_with_boundary_v1(
                            stage3_inputs,
                            beginning,
                            *lane_id,
                            beginning.next_interval_index,
                            stage3_forcing,
                            boundary,
                        )?
                    } else {
                        Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                            stage3_inputs,
                            beginning,
                            *lane_id,
                            beginning.next_interval_index,
                            stage3_forcing,
                            boundary,
                        )?
                    };
                    Wb11HydrologyKernel::project_stage3_parent_cadence_result(
                        beginning,
                        &mut result,
                        self.finalize_wb14_parent_interval,
                    )?;
                    let flux_tolerance = 1.0e-6_f64;
                    let evaluation = &result.evaluation;
                    let accepted_terminal_endpoint = if self.terminal_endpoint_mode {
                        match result.terminal_event.as_ref() {
                            Some(event) => validate_accepted_terminal_endpoint_composition_v1(
                                evaluation,
                                &result.reconciliation,
                                event,
                                interval_s,
                            )?,
                            None => false,
                        }
                    } else {
                        false
                    };
                    let complete_terminal_interval =
                        result.terminal_event.as_ref().is_some_and(|event| {
                            accepted_terminal_endpoint_timing_v1(
                                event.terminal_entry_offset_seconds,
                                event.evaluated_seconds,
                                event.unevaluated_seconds,
                                event.hour_offset_seconds,
                                evaluation.evaluated_seconds,
                                interval_s,
                            )
                        });
                    let joined = [
                        (
                            evaluation.complete_arm_sensible_j_m2,
                            boundary.sensible_energy_j_m2,
                        ),
                        (
                            evaluation.complete_arm_shortwave_j_m2,
                            boundary.shortwave_energy_j_m2,
                        ),
                        (
                            evaluation.complete_arm_latent_j_m2,
                            boundary.latent_energy_j_m2,
                        ),
                        (
                            evaluation.complete_arm_longwave_j_m2,
                            boundary.net_longwave_energy_j_m2,
                        ),
                        (
                            evaluation.complete_arm_advected_j_m2,
                            boundary.precipitation_advection_j_m2,
                        ),
                        (
                            evaluation.complete_arm_snow_soil_heat_j_m2,
                            boundary.snow_soil_heat_j_m2,
                        ),
                    ];
                    if joined
                        .iter()
                        .any(|(actual, expected)| (actual - expected).abs() > flux_tolerance)
                        || (evaluation.complete_arm_vapor_mass_exchange_kg_m2
                            - boundary.vapor_mass_kg_m2)
                            .abs()
                            > 1.0e-9
                        || (!accepted_terminal_endpoint
                            && result.evaluation.evaluated_seconds.to_bits()
                                != interval_s.to_bits())
                        || (!matches!(result.lifecycle, "active" | "reappeared")
                            && !accepted_terminal_endpoint)
                    {
                        if result
                            .terminal_event
                            .as_ref()
                            .is_some_and(|event| event.event_occurred)
                            && !self.terminal_endpoint_mode
                        {
                            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                                "covered terminal event requires terminal chronology",
                            ));
                        }
                        return Err(DirectV11RealConsumerError::Identity(
                            "Stage-3 covered boundary/result ledger join",
                        ));
                    }
                    if input.support.duration_ns() >= COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS {
                        phase_support_images_by_lane.insert(
                            *lane_id,
                            CoveredExactFloorTerminalPhaseSupportImageV1 {
                                parent_start_ns: input.support.start_ns().get(),
                                parent_end_ns: input.support.end_ns().get(),
                                support_start_ns: input.support.start_ns().get(),
                                support_end_ns: input.support.end_ns().get(),
                                actual_vapor_kg_m2: evaluation
                                    .complete_arm_vapor_mass_exchange_kg_m2,
                                deposition_kg_m2: result.deposition_kg_m2,
                                sublimation_kg_m2: result.sublimation_kg_m2,
                                snowfall_kg_m2: result.snowfall_kg_m2,
                                external_liquid_kg_m2: result.external_liquid_kg_m2,
                                complete_energy_j_m2: evaluation.complete_arm_total_j_m2,
                                cold_content_export_j_m2: evaluation
                                    .complete_arm_cold_content_export_j_m2,
                                ordered_energy_components_j_m2: [
                                    evaluation.complete_arm_shortwave_j_m2,
                                    evaluation.complete_arm_longwave_j_m2,
                                    evaluation.complete_arm_sensible_j_m2,
                                    evaluation.complete_arm_latent_j_m2,
                                    evaluation.complete_arm_advected_j_m2,
                                    evaluation.complete_arm_snow_soil_heat_j_m2,
                                    evaluation.complete_arm_internal_active_lower_conduction_j_m2,
                                ],
                                source_receipt_fingerprints: [
                                    evaluation.source_fingerprint,
                                    evaluation.forcing_fingerprint,
                                    evaluation.geometry_fingerprint,
                                    evaluation.non_formulation_fingerprint,
                                    evaluation.surface_arm_non_formulation_fingerprint,
                                    evaluation.complete_arm_non_formulation_fingerprint,
                                ],
                            },
                        );
                    }
                    if let Some(event) = result.terminal_event.as_ref() {
                        if self.terminal_endpoint_mode
                            && event.event_occurred
                            && accepted_terminal_endpoint
                        {
                            terminal_events.borrow_mut().insert(*lane_id, event.clone());
                            outcome_diagnostics_by_lane.insert(
                                *lane_id,
                                (
                                    result.evaluation.complete_arm_cold_content_export_j_m2,
                                    0.0,
                                    0.0,
                                    result.evaluation.complete_arm_snow_soil_heat_j_m2,
                                ),
                            );
                            ending_stage3.insert(*lane_id, result.state);
                            continue;
                        }
                        if event.event_occurred {
                            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                                "covered terminal event requires terminal chronology",
                            ));
                        }
                        if !complete_terminal_interval {
                            return Err(DirectV11RealConsumerError::Identity(
                                "covered terminal interval chronology",
                            ));
                        }
                    }
                    let mut interlayer_active = 0.0;
                    let mut interlayer_lower = 0.0;
                    for tuple in &result.reconciliation.tuples {
                        if tuple.applicable {
                            let lower_before = tuple.lower_cold_before_conduction_j_m2.ok_or(
                                DirectV11RealConsumerError::Identity(
                                    "lower interlayer beginning owner state",
                                ),
                            )?;
                            let lower_after = tuple.lower_cold_after_conduction_j_m2.ok_or(
                                DirectV11RealConsumerError::Identity(
                                    "lower interlayer ending owner state",
                                ),
                            )?;
                            // The lower control volume has no external boundary:
                            // independently reconstruct its received conduction
                            // from the immutable before/after material state.
                            let reported_active = tuple
                                .internal_active_lower_conduction_j_m2
                                .ok_or(DirectV11RealConsumerError::Identity(
                                    "active interlayer diagnostic",
                                ))?;
                            let reported_lower = tuple.lower_cold_energy_change_j_m2.ok_or(
                                DirectV11RealConsumerError::Identity("lower interlayer diagnostic"),
                            )?;
                            let (reconstructed_active, reconstructed_lower) =
                                reconstruct_interlayer_from_owner_states(
                                    lower_before,
                                    lower_after,
                                    reported_active,
                                    reported_lower,
                                )?;
                            interlayer_active += reconstructed_active;
                            interlayer_lower += reconstructed_lower;
                        }
                    }
                    outcome_diagnostics_by_lane.insert(
                        *lane_id,
                        (
                            result.evaluation.complete_arm_cold_content_export_j_m2,
                            interlayer_active,
                            interlayer_lower,
                            result.evaluation.complete_arm_snow_soil_heat_j_m2,
                        ),
                    );
                    ending_stage3.insert(*lane_id, result.state);
                }
                Ok::<_, DirectV11RealConsumerError>((
                    ending_stage3,
                    outcome_diagnostics_by_lane,
                    phase_support_images_by_lane,
                ))
            };
        let _interval_index = u8::try_from(self.interval_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 interval index overflow"))?;
        let iteration_vegetation_state = initial_vegetation_state;
        let mut iteration_stage3_states = self.stage3_beginning_by_lane.clone();
        let mut iteration_boundaries: Option<
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        > = None;
        let mut previous_lse_states: Option<BTreeMap<(OfeId, TileId), CoveredLseIterationState>> =
            None;
        let mut previous_previous_stage3_states: Option<
            BTreeMap<u32, DirectSnowStage3PersistentState>,
        > = None;
        let mut previous_stage3_states: Option<BTreeMap<u32, DirectSnowStage3PersistentState>> =
            None;
        let mut iteration_phase_support_images: Option<
            BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
        > = None;
        let mut phase_consistent_root_support_images: Option<
            BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
        > = None;
        let mut phase_consistent_branch_entry_seen = false;
        let mut phase_consistent_branch_entry_sides: Option<Vec<i8>> = None;
        let mut phase_consistent_coupled_active = false;
        let mut phase_consistent_parity_monotone_trace =
            Vec::<CoveredParityMonotoneActiveSetResetV1>::new();
        let mut stable_monotone_trace = Vec::<CoveredStableMonotoneRawAuthenticMapV1>::new();
        let mut stable_monotone_pre_root_refusal_disabled = false;
        let mut previous_soil_state: Option<DirectSoilThermalCandidate> = None;
        let mut iteration_soil_state =
            self.initial_unpublished_soil_iteration_candidate_v1(input.support)?;
        let support_duration_ns = input.support.duration_ns();
        let minimum_support_ns =
            crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
        let exact_floor_support = support_duration_ns == minimum_support_ns;
        let coarse_support_relaxation_enabled = support_duration_ns > minimum_support_ns;
        let mut exact_floor_period_two_relaxation_enabled = false;
        let mut finalization_stabilization = CoveredFinalizationStabilizationV1::default();
        let mut physical_evaluation_budget =
            CoveredPhysicalEvaluationBudgetV1::new(0).map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "phase-consistent physical evaluation budget",
                )
            })?;
        let mut accepted_snow_soil_receipts = self.snow_soil_heat_receipts_for_candidate_v1(
            input.support,
            &iteration_stage3_states,
            &iteration_soil_state,
        )?;
        let mut accepted_snow_enthalpy_material_owner =
            self.beginning_snow_enthalpy_material_owner.clone();
        let mut previous_complete_boundaries: Option<
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        > = None;
        // The legacy reduced carrier is evaluated once, solely to seed the
        // nonlinear iteration.  Its receipt never changes with candidate
        // state and is not the accepted component-carrier authority.
        let initial_guess_receipts = self.carrier_receipts_by_destination(
            interval_s,
            &iteration_vegetation_state,
            &iteration_stage3_states,
            self.stage3_forcing_by_lane,
        )?;
        let initial_guess_boundaries = self.stage3_lower_boundaries_by_destination(
            &initial_guess_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let initial_diagnostic_receipts = initial_guess_receipts
            .iter()
            .map(|(destination, receipt)| (destination.clone(), receipt.diagnostic_sha256))
            .collect::<BTreeMap<_, _>>();
        let covered_destinations = initial_guess_receipts
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        let prepared_covered_input = DirectV9RealConsumerShadow::prepare_covered_canopy_soil_input(
            self.interval,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
        let fixed_point = covered_boxed_execution_v1(|| 'fixed_point: {
            use crate::snow_stage3_v11_attachment::{
                begin_adaptive_parent_fixed_point_phase_v1 as phase_start,
                record_adaptive_parent_fixed_point_phase_v1 as phase_record,
                record_adaptive_parent_profile_detail_v1 as profile_record,
            };
            for iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
                covered_physical_evaluation_budget_charge_v1(&mut physical_evaluation_budget)
                    .map_err(|_| {
                        DirectV11RealConsumerError::AdaptiveRefinement(
                            "phase-consistent physical evaluation budget",
                        )
                    })?;
                let operand_started = phase_start();
                let next_snow_soil_receipts = self.snow_soil_heat_receipts_for_candidate_v1(
                    input.support,
                    &iteration_stage3_states,
                    &iteration_soil_state,
                )?;
                accepted_snow_soil_receipts = self.retain_terminal_limiting_snow_soil_receipts(
                    next_snow_soil_receipts,
                    &accepted_snow_soil_receipts,
                    &iteration_stage3_states,
                );
                self.validate_snow_soil_heat_receipt_iterate_joins(
                    &accepted_snow_soil_receipts,
                    &iteration_stage3_states,
                    iteration_soil_state.read_view(),
                )?;
                let (open_diagnostics, open_boundaries, _) =
                    self.open_snow_boundaries_by_destination(&iteration_stage3_states)?;
                let mut destination_receipts = initial_diagnostic_receipts.clone();
                for (destination, digest) in open_diagnostics {
                    if destination_receipts.insert(destination, digest).is_some() {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered/open destination forcing intersection",
                        ));
                    }
                }
                let carrier_boundaries = initial_guess_boundaries.clone();
                // The reduced carrier supplies only the first numerical guess.
                // After one LSE evaluation, the complete component-resolved
                // boundary is the sole iterate consumed by Stage 3.
                let flux_boundaries = iteration_boundaries.as_ref().unwrap_or(&carrier_boundaries);
                let current_boundaries = self.merge_latest_stage3_state_operands(
                    flux_boundaries,
                    &iteration_stage3_states,
                )?;
                phase_record("operands", operand_started);
                let envelope_started = phase_start();
                let mut provisional_candidate = covered_boxed_execution_v1(|| {
                    Ok::<_, DirectV11RealConsumerError>(self.beginning.clone())
                })?;
                provisional_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let unpublished_soil_candidate =
                    matches!(&iteration_soil_state, DirectSoilThermalCandidate::V2(_))
                        .then_some(&iteration_soil_state);
                let provisional = self.build_provisional_covered_iteration_evidence_v1(
                    input.support,
                    &current_boundaries,
                    &prepared_covered_input,
                    CoveredCarrierEnvelopeBuildV1 {
                        candidate: &provisional_candidate,
                        interval_s,
                        duration_s_bits: input.duration_s_bits,
                        covered_destinations: &covered_destinations,
                        covered_boundaries: &current_boundaries,
                        open_boundaries: &open_boundaries,
                        provisional: true,
                        finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
                    },
                    unpublished_soil_candidate,
                    None,
                )?;
                let provisional_precipitation_sets = provisional.precipitation_sets;
                let next_boundaries = provisional.corrected_boundaries;
                let mut lse_states = provisional.lse_states;
                let provisional_transaction_id = provisional.transaction_id;
                let provisional_soil_candidates = provisional.soil_candidates;
                let mut next_covered_boundaries =
                    self.apply_lse_iteration_exchange(&next_boundaries, &lse_states)?;
                let mut next_boundaries = next_covered_boundaries.clone();
                for (destination, boundary) in open_boundaries {
                    if next_boundaries.insert(destination, boundary).is_some() {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered/open boundary intersection",
                        ));
                    }
                }
                let mut next_boundaries = self.merge_latest_stage3_state_operands(
                    &next_boundaries,
                    &iteration_stage3_states,
                )?;
                phase_record("envelope", envelope_started);
                let stage3_started = phase_start();
                let (mut stage3_candidate, _, stage3_support_images) = evaluate_stage3(
                    &destination_receipts,
                    &next_boundaries,
                    None,
                    &accepted_snow_soil_receipts,
                    None,
                    None,
                    &provisional_precipitation_sets,
                )?;
                phase_record("stage3", stage3_started);
                let soil_started = phase_start();
                let soil_credits = self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?;
                let mut soil_candidate = self.unpublished_soil_candidate_for_covered_iteration_v1(
                    input.support,
                    provisional_transaction_id,
                    &provisional_soil_candidates,
                    &provisional.soil_energy_operands_v2,
                    &soil_credits,
                )?;
                if exact_floor_support
                    && !exact_floor_period_two_relaxation_enabled
                    && covered_fixed_point_exact_floor_period_two_detected_v1(
                        previous_previous_stage3_states.as_ref(),
                        previous_stage3_states.as_ref(),
                        &stage3_candidate,
                    )
                {
                    exact_floor_period_two_relaxation_enabled = true;
                }
                let relaxation_enabled =
                    coarse_support_relaxation_enabled || exact_floor_period_two_relaxation_enabled;
                let lse_converged = previous_lse_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_lse_states_equal(previous, &lse_states)
                });
                let stage3_comparison = if relaxation_enabled {
                    Some(&iteration_stage3_states)
                } else {
                    previous_stage3_states.as_ref()
                };
                let stage3_converged = stage3_comparison.is_some_and(|previous| {
                    covered_fixed_point_stage3_states_equal(previous, &stage3_candidate)
                });
                let soil_comparison = if relaxation_enabled {
                    Some(&iteration_soil_state)
                } else {
                    previous_soil_state.as_ref()
                };
                let soil_converged = soil_comparison.is_some_and(|previous| {
                    covered_fixed_point_soil_candidates_equal(previous, &soil_candidate)
                });
                let boundary_converged =
                    previous_complete_boundaries
                        .as_ref()
                        .is_some_and(|previous| {
                            covered_fixed_point_boundaries_equal(previous, &next_boundaries)
                        });
                if crate::snow_stage3_v11_attachment::covered_fixed_point_limiter_audit_enabled_v1()
                {
                    let (soil_enthalpy_delta, soil_temperature_delta) =
                        covered_soil_max_normalized_deltas_v1(soil_comparison, &soil_candidate);
                    record_covered_limiter_sample_v1(
                        input.support,
                        iteration + 1,
                        crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::Picard,
                        (
                            lse_converged,
                            stage3_converged,
                            soil_converged,
                            boundary_converged,
                        ),
                        (
                            covered_lse_max_normalized_delta_v1(
                                previous_lse_states.as_ref(),
                                &lse_states,
                            ),
                            covered_stage3_max_normalized_delta_v1(
                                stage3_comparison,
                                &stage3_candidate,
                            ),
                            soil_enthalpy_delta,
                            soil_temperature_delta,
                            covered_boundary_max_normalized_delta_v1(
                                previous_complete_boundaries.as_ref(),
                                &next_boundaries,
                            ),
                        ),
                    );
                }
                let coupled_map_converged =
                    lse_converged && stage3_converged && soil_converged && boundary_converged;
                let mut converged = CoveredConvergenceAdmissionV1::Picard.admits(
                    finalization_stabilization
                        .picard_accepts_convergence(coupled_map_converged, relaxation_enabled),
                    true,
                    true,
                );
                let mut coupled_finalization_inputs = None;
                let picard_relaxation_weight = (!converged)
                    .then(|| {
                        covered_fixed_point_relaxation_weight_v1(
                            input.support.duration_ns(),
                            exact_floor_period_two_relaxation_enabled,
                        )
                    })
                    .flatten();
                if iteration + 1 == COVERED_FIXED_POINT_POLICY.max_iterations && !converged {
                    crate::snow_stage3_v11_attachment::record_covered_fixed_point_limit_detail_v1(
                        crate::snow_stage3_v11_attachment::CoveredFixedPointLimitDetailV1 {
                        stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::Picard,
                        lse_converged,
                        stage3_converged,
                        soil_converged,
                        boundary_converged,
                        stage3_first_difference: if relaxation_enabled {
                            covered_stage3_state_first_difference_v1(
                                &iteration_stage3_states,
                                &stage3_candidate,
                            )
                        } else {
                            previous_stage3_states.as_ref().and_then(|previous| {
                                covered_stage3_state_first_difference_v1(
                                    previous,
                                    &stage3_candidate,
                                )
                            })
                        },
                    });
                }
                phase_record("soil", soil_started);
                #[cfg(test)]
                {
                    converged = converged && !covered_fixed_point_exhaustion_forced_for_test();
                }
                if !converged {
                    let mut stable_monotone_eligibility = None;
                    if !stable_monotone_pre_root_refusal_disabled {
                        let raw_stable_map = covered_stable_monotone_raw_authentic_map_v1(
                            input.support,
                            &self.stage3_beginning_by_lane,
                            &iteration_stage3_states,
                            &stage3_candidate,
                            &stage3_support_images,
                            &accepted_snow_soil_receipts,
                            self.beginning.inner.soil_thermal.read_view(),
                            &iteration_soil_state,
                            &soil_candidate,
                            &self.beginning.inner.lse_configuration,
                            self.stage3_inputs_by_lane,
                            physical_evaluation_budget.used,
                        );
                        stable_monotone_eligibility = match raw_stable_map {
                            Ok(map) => covered_stable_monotone_observe_raw_authentic_map_v1(
                                &mut stable_monotone_trace,
                                map,
                                &physical_evaluation_budget,
                            ),
                            Err(_) => {
                                stable_monotone_trace.clear();
                                None
                            }
                        };
                    }
                    let relaxation_weight = picard_relaxation_weight;
                    let active_set_iterate = iteration_phase_support_images
                        .as_ref()
                        .and_then(|current_support| {
                            covered_vapor_active_set_transition_v1(
                                current_support,
                                &stage3_support_images,
                            )
                            .map(|transition| (current_support, transition))
                        })
                        .map(|(current_support, transition)| {
                            if transition == CoveredVaporActiveSetTransitionV1::Interface {
                                for (lane_id, current_image) in current_support {
                                    covered_vapor_active_set_interface_v1(
                                        current_image,
                                        &stage3_support_images[lane_id],
                                    )
                                    .map_err(|error| {
                                        DirectV11RealConsumerError::AdaptiveRefinement(
                                            covered_vapor_active_set_error_detail_v1(error),
                                        )
                                    })?;
                                }
                            }
                            covered_vapor_active_set_iterate_v1(
                                &iteration_stage3_states,
                                &stage3_candidate,
                                &self.stage3_beginning_by_lane,
                                current_support,
                                &stage3_support_images,
                                transition,
                            )
                            .map_err(|error| {
                                DirectV11RealConsumerError::AdaptiveRefinement(
                                    covered_vapor_active_set_error_detail_v1(error),
                                )
                            })
                        })
                        .transpose()?;
                    if active_set_iterate.is_none() {
                        phase_consistent_parity_monotone_trace.clear();
                    }
                    if let Some(active_set) = active_set_iterate.as_ref() {
                        stable_monotone_trace.clear();
                        stable_monotone_eligibility = None;
                        stable_monotone_pre_root_refusal_disabled = true;
                        match active_set.transition {
                            CoveredVaporActiveSetTransitionV1::Interface => {
                                if phase_consistent_branch_entry_seen {
                                    let root_support = phase_consistent_root_support_images
                                        .as_ref()
                                        .ok_or(DirectV11RealConsumerError::AdaptiveRefinement(
                                            "phase-consistent retained root image",
                                        ))?;
                                    let root_joins = root_support
                                        .values()
                                        .flat_map(|image| image.source_receipt_fingerprints)
                                        .collect::<Vec<_>>();
                                    let reset_joins = active_set
                                        .support_images
                                        .values()
                                        .flat_map(|image| image.source_receipt_fingerprints)
                                        .collect::<Vec<_>>();
                                    let coordinates_and_predicates =
                                        |support: &BTreeMap<
                                            u32,
                                            CoveredExactFloorTerminalPhaseSupportImageV1,
                                        >|
                                         -> Result<
                                            (Vec<u64>, Vec<u8>),
                                            DirectV11RealConsumerError,
                                        > {
                                            let mut coordinates = Vec::new();
                                            let mut predicates = Vec::new();
                                            for (lane_id, image) in support {
                                                let (water, enthalpy) =
                                                phase_consistent_support_coordinates_v1(
                                                    &self.stage3_beginning_by_lane[lane_id],
                                                    image,
                                                )
                                                .map_err(|_| {
                                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                                        "phase-consistent transition coordinates",
                                                    )
                                                })?;
                                                coordinates
                                                    .extend([water.to_bits(), enthalpy.to_bits()]);
                                                predicates.push(if enthalpy <= 0.0 {
                                                0
                                            } else if enthalpy
                                                < crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
                                                    * water
                                            {
                                                1
                                            } else {
                                                2
                                            });
                                            }
                                            Ok((coordinates, predicates))
                                        };
                                    let (root_coordinates_bits, root_branch_predicates) =
                                        coordinates_and_predicates(root_support)?;
                                    let (reset_coordinates_bits, reset_branch_predicates) =
                                        coordinates_and_predicates(&active_set.support_images)?;
                                    let reset_trace = CoveredPhaseConsistentTransitionResetV1 {
                                        root_join_fingerprints: root_joins,
                                        reset_join_fingerprints: reset_joins,
                                        root_coordinates_bits,
                                        reset_coordinates_bits,
                                        root_branch_predicates,
                                        reset_branch_predicates,
                                        branch_entry_vapor_sides:
                                            phase_consistent_branch_entry_sides
                                                .clone()
                                                .unwrap_or_default(),
                                        opposite_raw_vapor_sides: stage3_support_images
                                            .values()
                                            .filter_map(|image| {
                                                image.pure_vapor_side().map(|side| match side {
                                                    std::cmp::Ordering::Greater => 1,
                                                    std::cmp::Ordering::Less => -1,
                                                    std::cmp::Ordering::Equal => 0,
                                                })
                                            })
                                            .collect(),
                                        raw_authentic_continuous_owner_bits: stage3_candidate
                                            .values()
                                            .flat_map(|state| {
                                                state.layers.iter().flat_map(|layer| {
                                                    [
                                                        layer.mass_swe_m.to_bits(),
                                                        layer.liquid_water_m.to_bits(),
                                                        layer.cold_content_j_m2.to_bits(),
                                                    ]
                                                })
                                            })
                                            .collect(),
                                    };
                                    let exact_reset =
                                        phase_consistent_coupled_active_set_transition_window_v1(
                                            &mut phase_consistent_root_support_images,
                                            &mut phase_consistent_branch_entry_seen,
                                            &mut phase_consistent_branch_entry_sides,
                                            &active_set.support_images,
                                            &reset_trace,
                                        );
                                    if exact_reset {
                                        phase_consistent_parity_monotone_trace.clear();
                                        phase_consistent_coupled_active = true;
                                    } else {
                                        let minimum_solver_reserve =
                                            3 * self.stage3_beginning_by_lane.len()
                                                + 2 * self
                                                    .beginning
                                                    .inner
                                                    .soil_thermal
                                                    .read_view()
                                                    .ordered_ofes()
                                                    .len()
                                                + 4;
                                        let parity_monotone_observation =
                                            covered_parity_monotone_active_set_observe_v1(
                                                &mut phase_consistent_parity_monotone_trace,
                                                CoveredParityMonotoneActiveSetResetV1 {
                                                    support_start_ns: input
                                                        .support
                                                        .start_ns()
                                                        .get(),
                                                    support_end_ns: input.support.end_ns().get(),
                                                    reset: reset_trace,
                                                    physical_evaluation_ordinal:
                                                        physical_evaluation_budget.used,
                                                    publication_eligible: false,
                                                },
                                                &physical_evaluation_budget,
                                                minimum_solver_reserve,
                                            );
                                        phase_consistent_coupled_active = match
                                            parity_monotone_observation
                                        {
                                            Ok(eligibility) => eligibility.is_some(),
                                            Err(_) => {
                                                match covered_one_way_phase_boundary_eligibility_v1(
                                                    &phase_consistent_parity_monotone_trace,
                                                    &physical_evaluation_budget,
                                                    minimum_solver_reserve,
                                                ) {
                                                    Ok(eligibility) => {
                                                        eligibility.canonical_boundary_crossings == 1
                                                    }
                                                    Err(
                                                        PhaseConsistentCoupledSolveErrorV1::NonDescent,
                                                    ) => {
                                                        covered_one_way_post_crossing_contraction_eligibility_v1(
                                                            &phase_consistent_parity_monotone_trace,
                                                            &physical_evaluation_budget,
                                                            minimum_solver_reserve,
                                                        )
                                                        .map_err(|_| {
                                                            DirectV11RealConsumerError::AdaptiveRefinement(
                                                                "phase-consistent post-crossing contraction eligibility",
                                                            )
                                                        })?
                                                        .canonical_boundary_crossings
                                                            == 1
                                                    }
                                                    Err(_) => return Err(
                                                        DirectV11RealConsumerError::AdaptiveRefinement(
                                                            "phase-consistent one-way phase-boundary eligibility",
                                                        ),
                                                    ),
                                                }
                                            }
                                        };
                                    }
                                } else {
                                    phase_consistent_parity_monotone_trace.clear();
                                    phase_consistent_root_support_images =
                                        Some(active_set.support_images.clone());
                                    phase_consistent_branch_entry_seen = false;
                                    phase_consistent_branch_entry_sides = None;
                                }
                            }
                            CoveredVaporActiveSetTransitionV1::BranchEntry => {
                                phase_consistent_branch_entry_seen = true;
                                phase_consistent_branch_entry_sides = Some(
                                    active_set
                                        .support_images
                                        .values()
                                        .filter_map(|image| {
                                            image.pure_vapor_side().map(|side| match side {
                                                std::cmp::Ordering::Greater => 1,
                                                std::cmp::Ordering::Less => -1,
                                                std::cmp::Ordering::Equal => 0,
                                            })
                                        })
                                        .collect(),
                                );
                            }
                        }
                    }
                    let mut coupled_artifacts = None;
                    let stable_monotone_coupled_active = stable_monotone_eligibility.is_some();
                    if phase_consistent_coupled_active || stable_monotone_coupled_active {
                        let endpoint_seed_stage3 = stage3_candidate.clone();
                        let endpoint_seed_soil = soil_candidate.clone();
                        let endpoint_seed_snow_soil_receipts = self
                            .snow_soil_heat_receipts_for_candidate_v1(
                                input.support,
                                &endpoint_seed_stage3,
                                &endpoint_seed_soil,
                            )?;
                        let endpoint_seed_boundaries = next_covered_boundaries.clone();
                        let coupled_root_exists = std::cell::Cell::new(false);
                        let frozen_temperature_primary_committed = std::cell::Cell::new(false);
                        let coupled_result = covered_boxed_execution_v1(|| -> Result<
                            (
                                CoveredPhaseConsistentPhysicalArtifactsV1,
                                BTreeMap<u32, SnowSoilHeatReceiptV1>,
                                CoveredFinalizationEquivalentReplayInputsV1,
                            ),
                            DirectV11RealConsumerError,
                        > {
                        let root_support = active_set_iterate
                            .as_ref()
                            .map(|active_set| active_set.support_images.clone())
                            .unwrap_or_else(|| stage3_support_images.clone());
                        let lane_ids = root_support.keys().copied().collect::<Vec<_>>();
                        let beginning_soil_ofes = self
                            .beginning
                            .inner
                            .soil_thermal
                            .read_view()
                            .ordered_ofes();
                        let legacy_seed_coordinates = stable_monotone_eligibility
                            .as_ref()
                            .map(|eligibility| eligibility.seed_coordinates.clone())
                            .unwrap_or_default();
                        let mut legacy_initial_coordinates = Vec::new();
                        if legacy_seed_coordinates.is_empty() {
                            legacy_initial_coordinates.reserve(
                                3 * lane_ids.len() + 2 * beginning_soil_ofes.len(),
                            );
                            for lane_id in &lane_ids {
                                let beginning = &self.stage3_beginning_by_lane[lane_id];
                                let coordinates = phase_consistent_support_coordinates_v1(
                                    beginning,
                                    &root_support[lane_id],
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "phase-consistent root coordinates",
                                    )
                                })?;
                                let density_kg_m3 = iteration_stage3_states
                                    .get(lane_id)
                                    .and_then(|state| state.layers.first())
                                    .map(|layer| layer.density_kg_m3)
                                    .ok_or(DirectV11RealConsumerError::AdaptiveRefinement(
                                        "phase-consistent density coordinate",
                                    ))?;
                                legacy_initial_coordinates.extend([
                                    coordinates.0,
                                    coordinates.1,
                                    density_kg_m3,
                                ]);
                            }
                            for ofe in iteration_soil_state.read_view().ordered_ofes() {
                                let top = ofe.ordered_layers().into_iter().next().ok_or(
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "phase-consistent soil coordinate",
                                    ),
                                )?;
                                legacy_initial_coordinates.extend([
                                    covered_soil_layer_enthalpy_coordinate_v1(top)?,
                                    top.temperature_k(),
                                ]);
                            }
                        } else {
                            let legacy_expected =
                                3 * lane_ids.len() + 2 * beginning_soil_ofes.len();
                            if legacy_seed_coordinates.len() != legacy_expected {
                                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                                    "phase-consistent legacy seed coordinate shape",
                                ));
                            }
                            legacy_initial_coordinates = legacy_seed_coordinates;
                        }
                        let frozen_temperature_primary =
                            covered_frozen_temperature_primary_eligibility_v1(
                                &lane_ids,
                                &self.stage3_beginning_by_lane,
                                &endpoint_seed_stage3,
                                &root_support,
                                &legacy_initial_coordinates,
                                beginning_soil_ofes.len(),
                                &physical_evaluation_budget,
                                legacy_initial_coordinates.len() + 1
                                    + COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1,
                            )
                            .map_err(|_| {
                                DirectV11RealConsumerError::AdaptiveRefinement(
                                    "frozen temperature-primary eligibility",
                                )
                            })?;
                        let initial_coordinate_posture = if frozen_temperature_primary.is_some() {
                            CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                        } else {
                            CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
                        };
                        if initial_coordinate_posture
                            == CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                        {
                            frozen_temperature_primary_committed.set(true);
                        }
                        let initial_coordinates = if let Some(eligibility) =
                            frozen_temperature_primary.as_ref()
                        {
                            eligibility.seed_coordinates.clone()
                        } else {
                            covered_phase_consistent_same_map_cn_heat_seed_v1(
                                &legacy_initial_coordinates,
                                &lane_ids,
                                beginning_soil_ofes.len(),
                                &endpoint_seed_snow_soil_receipts,
                            )
                            .map_err(|_| {
                                DirectV11RealConsumerError::AdaptiveRefinement(
                                    "phase-consistent same-map CN heat seed",
                                )
                            })?
                        };
                        let frozen_temperature_primary_beginning_carries =
                            if initial_coordinate_posture
                                == CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                            {
                                Some(covered_frozen_temperature_primary_beginning_carries_v1(
                                    &self.stage3_beginning_by_lane,
                                    &lane_ids,
                                    self.beginning_snow_enthalpy_material_owner.as_ref(),
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "V56 beginning exact snow enthalpy carry",
                                    )
                                })?)
                            } else {
                                None
                            };
                        let active_coordinate_posture = std::cell::Cell::new(initial_coordinate_posture);
                        let frozen_temperature_primary_beginning_carries =
                            std::cell::RefCell::new(frozen_temperature_primary_beginning_carries);
                        let authentic_replay_input_exchange = std::cell::RefCell::new(None::<
                            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
                        >);
                        let expected_density_model_branch = stable_monotone_eligibility
                            .as_ref()
                            .map(|eligibility| eligibility.density_model_branch.clone());
                        let lane_to_ofe = self
                            .covered_lane_to_ofe(&self.stage3_beginning_by_lane)?;
                        let physical_evaluate = |
                            coordinates: &[f64],
                            budget: &mut CoveredPhysicalEvaluationBudgetV1,
                            authentic_receipts: Option<&BTreeMap<u32, SnowSoilHeatReceiptV1>>,
                            evaluation_kind: CoveredPhaseConsistentPhysicalEvaluationKindV1,
                        |
                         -> Result<
                            CoveredPhaseConsistentPhysicalEvaluationV1,
                            PhaseConsistentCoupledSolveErrorV1,
                        > {
                            let coordinate_posture = active_coordinate_posture.get();
                            let charged_map =
                                covered_phase_consistent_finalization_equivalent_map_v1(
                                    budget,
                                    || {
                                        if evaluation_kind.requires_authentic_receipts()
                                            != authentic_receipts.is_some()
                                        {
                                            return Err(
                                                PhaseConsistentCoupledSolveErrorV1::Structure,
                                            );
                                        }
                            let closure_posture =
                                covered_phase_consistent_carrier_closure_posture_v1(
                                    evaluation_kind,
                                );
                            let projected_soil_consumption =
                                CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly;
                            if closure_posture.requires_strict_weighted_ofe_closure()
                                != authentic_receipts.is_some()
                            {
                                return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
                            }
                            let soil_coordinate_offset =
                                coordinate_posture.soil_coordinate_offset(lane_ids.len())?;
                            let expected = soil_coordinate_offset
                                .checked_add(2 * beginning_soil_ofes.len())
                                .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                            if coordinates.len() != expected {
                                return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
                            }
                            let (proposed_stage3, phases, frozen_lane_enthalpies) =
                                match coordinate_posture {
                                    CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                                        let mut legacy_coordinates = Vec::with_capacity(
                                            3 * lane_ids.len() + 2 * beginning_soil_ofes.len(),
                                        );
                                        for lane_index in 0..lane_ids.len() {
                                            legacy_coordinates.extend_from_slice(
                                                &coordinates
                                                    [4 * lane_index..4 * lane_index + 3],
                                            );
                                        }
                                        legacy_coordinates.extend_from_slice(
                                            &coordinates[soil_coordinate_offset..],
                                        );
                                        let (projected, phases) =
                                            covered_phase_consistent_project_stage3_coordinates_v1(
                                                &endpoint_seed_stage3,
                                                &lane_ids,
                                                &legacy_coordinates,
                                            )?;
                                        (projected, Some(phases), Vec::new())
                                    }
                                    CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                                        let (projected, enthalpies) =
                                            covered_frozen_temperature_primary_project_stage3_coordinates_v1(
                                                &endpoint_seed_stage3,
                                                &lane_ids,
                                                coordinates,
                                            )?;
                                        (projected, None, enthalpies)
                                    }
                                };
                            let carrier_input_exchange =
                                covered_phase_consistent_carrier_input_exchange_v1(
                                    evaluation_kind,
                                    &endpoint_seed_boundaries,
                                    authentic_replay_input_exchange.borrow().as_ref(),
                                )?;
                            let trial_boundaries = self
                                .merge_latest_stage3_state_operands(
                                    &carrier_input_exchange,
                                    &proposed_stage3,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let mut trial_candidate = covered_boxed_execution_v1(|| {
                                    Ok::<_, DirectV11RealConsumerError>(self.beginning.clone())
                                })
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            trial_candidate.inner.authority =
                                CoveredColumnAuthority::V11SnowCovered;
                            let (trial_open_diagnostics, trial_open_boundaries, _) = self
                                .open_snow_boundaries_by_destination(&proposed_stage3)
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let mut projected_soil_coordinates = Vec::new();
                            for (soil_index, ofe) in endpoint_seed_soil
                                .read_view()
                                .ordered_ofes()
                                .iter()
                                .enumerate()
                            {
                                let layers = ofe.ordered_layers();
                                let top = layers
                                    .first()
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                projected_soil_coordinates.push(
                                    openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2 {
                                        ofe_id: ofe.ofe_id().clone(),
                                        layer_id: top.layer_id().clone(),
                                        proposed_total_enthalpy_j_m2_ofe_ground: coordinates
                                            [soil_coordinate_offset + 2 * soil_index],
                                        proposed_temperature_k: coordinates
                                            [soil_coordinate_offset + 2 * soil_index + 1],
                                    },
                                );
                            }
                            let prepared_soil = self
                                .beginning
                                .prepare_next_soil_thermal_support_v2(
                                    input.support.start_ns().get(),
                                    input.support.end_ns().get(),
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let endpoint_seed_soil_trial = endpoint_seed_soil
                                .v2()
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            if endpoint_seed_soil_trial.transaction_id()
                                != prepared_soil.beginning_owner().transaction_id
                                || endpoint_seed_soil_trial.support_start_ns()
                                    != input.support.start_ns().get()
                                || endpoint_seed_soil_trial.support_end_ns()
                                    != input.support.end_ns().get()
                            {
                                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                            }
                            let proposed_soil = self
                                .beginning
                                .project_soil_thermal_unpublished_top_layer_coordinates_v2(
                                    &prepared_soil,
                                    &projected_soil_coordinates,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let final_envelope = self
                                .build_covered_carrier_envelope_value_with_soil_beginning_v1(
                                    CoveredCarrierEnvelopeBuildV1 {
                                        candidate: &trial_candidate,
                                        interval_s,
                                        duration_s_bits: input.duration_s_bits,
                                        covered_destinations: &covered_destinations,
                                        covered_boundaries: &trial_boundaries,
                                        open_boundaries: &trial_open_boundaries,
                                        provisional: closure_posture.carrier_is_provisional(),
                                        finalize_wb14_parent_interval: self
                                            .finalize_wb14_parent_interval,
                                    },
                                    Some(&proposed_soil),
                                    None,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let precipitation_sets = self
                                .precipitation_parcel_sets(input.support, &final_envelope)
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let (carrier_corrected_boundaries, _, _) = self
                                .corrected_covered_boundaries_from_envelope(
                                    &trial_boundaries,
                                    &final_envelope,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let carrier_lse_states = final_envelope
                                .covered_lse_iteration_state_by_destination()
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let provisional = ProvisionalCoveredIterationEvidenceV1 {
                                precipitation_sets,
                                corrected_boundaries: carrier_corrected_boundaries,
                                lse_states: carrier_lse_states,
                                transaction_id: final_envelope.transaction_id(),
                                soil_candidates: final_envelope
                                    .hydrology()
                                    .soil_thermal_candidates()
                                    .to_vec(),
                                soil_energy_operands_v2:
                                    crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
                                        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                                            final_envelope.transaction_id(),
                                            input.support.start_ns().get(),
                                            input.support.end_ns().get(),
                                            final_envelope
                                                .hydrology()
                                                .pre_ingress_soil_thermal_candidates(),
                                        )
                                        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
                                        input.support.start_ns().get(),
                                        input.support.end_ns().get(),
                                        &self.beginning.inner.lse_configuration.owner_id,
                                        &self.beginning.inner.surface_configuration.owner_id,
                                        final_envelope
                                            .hydrology()
                                            .pre_ingress_soil_thermal_candidates(),
                                        final_envelope.hydrology().surface_ingress(),
                                    )
                                    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
                            };
                            let mut destination_receipts = initial_diagnostic_receipts.clone();
                            for (destination, digest) in trial_open_diagnostics {
                                if destination_receipts.insert(destination, digest).is_some() {
                                    return Err(
                                        PhaseConsistentCoupledSolveErrorV1::SideConstraint,
                                    );
                                }
                            }
                            if provisional.lse_states.values().any(|state| {
                                !state.shared_heat_residual_w_m2.is_finite()
                                    || !state.shared_heat_tolerance_w_m2.is_finite()
                                    || state.shared_heat_residual_w_m2.abs()
                                        > state.shared_heat_tolerance_w_m2
                                    || !state.shared_vapor_residual_kg_m2_s.is_finite()
                                    || !state.shared_vapor_tolerance_kg_m2_s.is_finite()
                                    || state.shared_vapor_residual_kg_m2_s.abs()
                                        > state.shared_vapor_tolerance_kg_m2_s
                            }) {
                                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                            }
                            let corrected_boundaries = self
                                .apply_lse_iteration_exchange(
                                    &provisional.corrected_boundaries,
                                    &provisional.lse_states,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let mut complete_trial_boundaries = corrected_boundaries.clone();
                            for (destination, boundary) in &trial_open_boundaries {
                                if complete_trial_boundaries
                                    .insert(destination.clone(), boundary.clone())
                                    .is_some()
                                {
                                    return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                                }
                            }
                            let complete_trial_boundaries = self
                                .merge_latest_stage3_state_operands(
                                    &complete_trial_boundaries,
                                    &proposed_stage3,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let mut cn_trial_operands = BTreeMap::new();
                            let mut projected_soil_cn_consumed_lanes = Vec::new();
                            for (lane_index, lane_id) in lane_ids.iter().enumerate() {
                                let ofe_id = lane_to_ofe
                                    .get(lane_id)
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let soil_index = beginning_soil_ofes
                                    .iter()
                                    .position(|ofe| ofe.ofe_id() == ofe_id)
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let receipt = accepted_snow_soil_receipts
                                    .get(lane_id)
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let trial = match coordinate_posture {
                                    CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                                        projected_soil_consumption
                                            .cn_heat_coordinate_trial_operand(
                                                coordinates,
                                                lane_index,
                                                receipt,
                                            )?
                                    }
                                    CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                                        projected_soil_consumption.cn_trial_operand(
                                            coordinates,
                                            lane_ids.len(),
                                            soil_index,
                                            receipt,
                                            coordinates[3 * lane_index + 1],
                                            interval_s,
                                        )?
                                    }
                                };
                                if let Some(authentic) = authentic_receipts {
                                    let sealed = authentic
                                        .get(lane_id)
                                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                    crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(
                                        sealed,
                                    )
                                    .map_err(|_| {
                                        PhaseConsistentCoupledSolveErrorV1::ReplayMismatch
                                    })?;
                                }
                                cn_trial_operands.insert(*lane_id, trial);
                                projected_soil_cn_consumed_lanes.push(*lane_id);
                            }
                            covered_phase_consistent_projected_soil_exact_once_v1(
                                &lane_ids,
                                &projected_soil_cn_consumed_lanes,
                                &[],
                            )?;
                            let stage3 = evaluate_stage3(
                                &destination_receipts,
                                &complete_trial_boundaries,
                                None,
                                authentic_receipts.unwrap_or(&accepted_snow_soil_receipts),
                                Some(&cn_trial_operands),
                                Some(evaluation_kind),
                                &provisional.precipitation_sets,
                            )
                            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            for image in stage3.2.values() {
                                image
                                    .validate()
                                    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            }
                            let credit_receipts =
                                authentic_receipts.unwrap_or(&accepted_snow_soil_receipts);
                            let credits = credit_receipts
                                .values()
                                .map(|receipt| {
                                    let consumption = covered_phase_consistent_cn_consumption_v1(
                                        evaluation_kind,
                                        cn_trial_operands.get(&receipt.lane_id),
                                        Some(receipt),
                                    )?;
                                    let beginning_ofe = beginning_soil_ofes
                                        .iter()
                                        .find(|ofe| ofe.ofe_id() == &receipt.ofe_id)
                                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                    let first = beginning_ofe
                                        .ordered_layers()
                                        .into_iter()
                                        .next()
                                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                    Ok(SoilThermalTopBoundaryCreditV1 {
                                        lane_id: receipt.lane_id,
                                        ofe_id: receipt.ofe_id.clone(),
                                        first_layer_id: first.layer_id().clone(),
                                        beginning_owner_id: self
                                            .beginning
                                            .inner
                                            .soil_thermal
                                            .owner_id()
                                            .clone(),
                                        beginning_configuration_sha256: self
                                            .beginning
                                            .inner
                                            .soil_thermal
                                            .configuration_sha256()
                                            .clone(),
                                        beginning_state_sha256: self
                                            .beginning
                                            .inner
                                            .soil_thermal
                                            .state_sha256()
                                            .clone(),
                                        support_start_ns: i64::try_from(
                                            receipt.support.start_ns().get(),
                                        )
                                        .map_err(|_| {
                                            PhaseConsistentCoupledSolveErrorV1::Structure
                                        })?,
                                        support_end_ns: i64::try_from(
                                            receipt.support.end_ns().get(),
                                        )
                                        .map_err(|_| {
                                            PhaseConsistentCoupledSolveErrorV1::Structure
                                        })?,
                                        accepted_positive_downward_j_m2_ofe_ground: consumption
                                            .soil_candidate_heat_j_m2_ofe_ground,
                                        soil_thermal_credit_j_m2_ofe_ground: consumption
                                            .soil_candidate_heat_j_m2_ofe_ground,
                                        snow_soil_heat_receipt_sha256: Sha256Digest::try_new(
                                            digest32_hex(receipt.receipt_sha256),
                                        )
                                        .map_err(|_| {
                                            PhaseConsistentCoupledSolveErrorV1::Structure
                                        })?,
                                    })
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            let soil_candidate = self
                                .unpublished_soil_candidate_for_covered_iteration_v1(
                                    input.support,
                                    provisional.transaction_id,
                                    &provisional.soil_candidates,
                                    &provisional.soil_energy_operands_v2,
                                    &credits,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let physical_snow_soil_receipts = self
                                .snow_soil_heat_receipts_for_candidate_v1(
                                    input.support,
                                    &stage3.0,
                                    &soil_candidate,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                            let physical_snow_candidate_cn_heat_j_m2 = lane_ids
                                .iter()
                                .map(|lane_id| {
                                    physical_snow_soil_receipts
                                        .get(lane_id)
                                        .map(|receipt| {
                                            receipt.snow_candidate_heat_j_m2_ofe_ground
                                        })
                                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            let mut beginning_water = Vec::new();
                            let mut beginning_enthalpy = Vec::new();
                            let mut beginning_exact_enthalpy_hi = Vec::new();
                            let mut beginning_enthalpy_carry = Vec::new();
                            let mut delta_water = Vec::new();
                            let mut complete_energy = Vec::new();
                            let mut ordered_energy_operands = Vec::new();
                            let mut physical_ice = Vec::new();
                            let mut physical_density = Vec::new();
                            let mut physical_thickness = Vec::new();
                            let mut exact_density_settling_branch_satisfied = Vec::new();
                            let mut actual_density_model_branch = Vec::new();
                            for lane_id in &lane_ids {
                                let image = &stage3.2[lane_id];
                                let ending = phase_consistent_support_coordinates_v1(
                                    &self.stage3_beginning_by_lane[lane_id],
                                    image,
                                )?;
                                let delta = image.snowfall_kg_m2
                                    + image.external_liquid_kg_m2
                                    + image.deposition_kg_m2
                                    - image.sublimation_kg_m2;
                                beginning_water.push(ending.0 - delta);
                                beginning_enthalpy.push(ending.1 - image.complete_energy_j_m2);
                                delta_water.push(delta);
                                complete_energy.push(image.complete_energy_j_m2);
                                let mut exact_operands =
                                    image.ordered_energy_components_j_m2.to_vec();
                                exact_operands.push(image.cold_content_export_j_m2);
                                ordered_energy_operands.push(exact_operands);
                                if coordinate_posture
                                    == CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                                {
                                    let beginning_carries =
                                        frozen_temperature_primary_beginning_carries.borrow();
                                    let carry_state = beginning_carries
                                        .as_ref()
                                        .and_then(|carries| carries.get(beginning_exact_enthalpy_hi.len()))
                                        .filter(|carry| carry.lane_id() == *lane_id)
                                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                    beginning_exact_enthalpy_hi.push(
                                        carry_state.enthalpy_hi_j_m2_ofe_ground(),
                                    );
                                    beginning_enthalpy_carry
                                        .push(carry_state.enthalpy_carry().clone());
                                }
                                let physical_state = stage3
                                    .0
                                    .get(lane_id)
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let branch_layer = iteration_stage3_states
                                    .get(lane_id)
                                    .and_then(|state| state.layers.first())
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let physical_layer = covered_terminal_density_physical_layer_v1(
                                    physical_state,
                                    branch_layer.settle_day_count,
                                )?;
                                actual_density_model_branch.extend_from_slice(
                                    &covered_terminal_density_constitutive_branch_v1(
                                        *lane_id,
                                        &self.stage3_beginning_by_lane[lane_id],
                                        &self.stage3_inputs_by_lane[lane_id],
                                        branch_layer.settle_day_count,
                                    )
                                    .map_err(|_| {
                                        PhaseConsistentCoupledSolveErrorV1::SideConstraint
                                    })?,
                                );
                                physical_ice.push(physical_layer.mass_swe_m * 1_000.0);
                                physical_density.push(physical_layer.density_kg_m3);
                                physical_thickness.push(physical_layer.thickness_m);
                                exact_density_settling_branch_satisfied.push(true);
                            }
                            if expected_density_model_branch
                                .as_ref()
                                .is_some_and(|expected| expected != &actual_density_model_branch)
                            {
                                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                            }
                            let mut beginning_soil_enthalpy = Vec::new();
                            let mut soil_delta = Vec::new();
                            let mut owner_soil_temperature = Vec::new();
                            for (soil_index, beginning_ofe) in
                                beginning_soil_ofes.iter().enumerate()
                            {
                                let beginning_top = beginning_ofe
                                    .ordered_layers()
                                    .into_iter()
                                    .next()
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let ending_top = soil_candidate
                                    .read_view()
                                    .ordered_ofes()
                                    .into_iter()
                                    .find(|ofe| ofe.ofe_id() == beginning_ofe.ofe_id())
                                    .and_then(|ofe| ofe.ordered_layers().into_iter().next())
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let configured_top = self
                                    .beginning
                                    .inner
                                    .lse_configuration
                                    .ofes
                                    .iter()
                                    .find(|ofe| &ofe.ofe_id == beginning_ofe.ofe_id())
                                    .and_then(|ofe| ofe.soil_interface_layers.first())
                                    .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
                                let coordinate = soil_coordinate_offset + 2 * soil_index;
                                let beginning_energy = covered_soil_layer_enthalpy_coordinate_v1(
                                    beginning_top,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                                let ending_energy = covered_soil_layer_enthalpy_coordinate_v1(
                                    ending_top,
                                )
                                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
                                beginning_soil_enthalpy.push(beginning_energy);
                                soil_delta.push(
                                    ending_energy - beginning_energy,
                                );
                                owner_soil_temperature.push(
                                    beginning_top.temperature_k()
                                        + (coordinates[coordinate] - beginning_energy)
                                            / configured_top.areal_heat_capacity_j_m2_k,
                                );
                            }
                            let mut tolerances = Vec::with_capacity(expected);
                            for lane_index in 0..lane_ids.len() {
                                let coordinate = coordinate_posture.snow_stride() * lane_index;
                                let geometry = match coordinate_posture {
                                    CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                                        let phase = phase_consistent_canonical_phase_projection_v1(
                                            coordinates[coordinate],
                                            coordinates[coordinate + 1],
                                            coordinates[coordinate + 2],
                                        )?;
                                        CoveredTerminalDensityGeometryCoordinateV1::from_canonical_phase(
                                            &phase,
                                        )?
                                    }
                                    CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                                        CoveredTerminalDensityGeometryCoordinateV1 {
                                            rho_1_kg_m3: coordinates[coordinate + 2],
                                            ice_1_kg_m2: coordinates[coordinate],
                                            z_1_m: coordinates[coordinate]
                                                / coordinates[coordinate + 2],
                                        }
                                    }
                                };
                                let lane_tolerances = [
                                    COVERED_FIXED_POINT_POLICY.mass_abs_kg_m2,
                                    COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
                                    geometry.density_absolute_tolerance_kg_m3(),
                                ];
                                tolerances.extend(lane_tolerances);
                                if coordinate_posture
                                    == CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
                                {
                                    tolerances.push(COVERED_FIXED_POINT_POLICY.energy_abs_j_m2);
                                }
                            }
                            for _ in &beginning_soil_ofes {
                                let soil_tolerances = [
                                    COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
                                    COVERED_FIXED_POINT_POLICY.state_temperature_abs_k,
                                ];
                                tolerances.extend(soil_tolerances);
                            }
                            let evaluation = match coordinate_posture {
                                CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                                    let mut legacy_coordinates = Vec::with_capacity(
                                        3 * lane_ids.len() + 2 * beginning_soil_ofes.len(),
                                    );
                                    let mut legacy_tolerances = Vec::with_capacity(
                                        3 * lane_ids.len() + 2 * beginning_soil_ofes.len(),
                                    );
                                    for lane_index in 0..lane_ids.len() {
                                        legacy_coordinates.extend_from_slice(
                                            &coordinates
                                                [4 * lane_index..4 * lane_index + 3],
                                        );
                                        legacy_tolerances.extend_from_slice(
                                            &tolerances
                                                [4 * lane_index..4 * lane_index + 3],
                                        );
                                    }
                                    legacy_coordinates
                                        .extend_from_slice(&coordinates[soil_coordinate_offset..]);
                                    legacy_tolerances.extend_from_slice(
                                        &tolerances[soil_coordinate_offset..],
                                    );
                                    let base_evaluation =
                                        covered_phase_consistent_residual_assemble_v1(
                                            CoveredPhaseConsistentResidualInputsV1 {
                                                coordinates: legacy_coordinates,
                                                beginning_snow_water_kg_m2: beginning_water,
                                                beginning_snow_enthalpy_j_m2: beginning_enthalpy,
                                                physical_delta_water_kg_m2: delta_water,
                                                physical_complete_energy_j_m2: complete_energy,
                                                physical_ice_kg_m2: physical_ice,
                                                physical_density_kg_m3: physical_density,
                                                physical_thickness_m: physical_thickness,
                                                exact_density_settling_branch_satisfied,
                                                beginning_soil_enthalpy_j_m2:
                                                    beginning_soil_enthalpy,
                                                physical_soil_delta_energy_j_m2: soil_delta,
                                                owner_soil_temperature_k:
                                                    owner_soil_temperature,
                                                absolute_tolerances: legacy_tolerances,
                                                algebraic_side_constraints_satisfied: true,
                                            },
                                        )?;
                                    let snow_candidate_cn_heat_j_m2 = lane_ids
                                        .iter()
                                        .enumerate()
                                        .map(|(lane_index, _)| coordinates[4 * lane_index + 3])
                                        .collect::<Vec<_>>();
                                    covered_cn_heat_coordinate_residual_evaluate_v1(
                                        base_evaluation,
                                        coordinates.to_vec(),
                                        &snow_candidate_cn_heat_j_m2,
                                        &physical_snow_candidate_cn_heat_j_m2,
                                        tolerances,
                                    )?
                                }
                                CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                                    covered_frozen_temperature_primary_residual_assemble_v1(
                                        CoveredFrozenTemperaturePrimaryResidualInputsV1 {
                                            coordinates: coordinates.to_vec(),
                                            lane_ids: lane_ids.clone(),
                                            beginning_snow_water_kg_m2: beginning_water,
                                            beginning_snow_enthalpy_hi_j_m2:
                                                beginning_exact_enthalpy_hi,
                                            beginning_snow_enthalpy_carry:
                                                beginning_enthalpy_carry,
                                            physical_delta_water_kg_m2: delta_water,
                                            ordered_physical_energy_operands_j_m2:
                                                ordered_energy_operands,
                                            physical_ice_kg_m2: physical_ice,
                                            physical_density_kg_m3: physical_density,
                                            physical_thickness_m: physical_thickness,
                                            exact_density_settling_branch_satisfied,
                                            beginning_soil_enthalpy_j_m2:
                                                beginning_soil_enthalpy,
                                            physical_soil_delta_energy_j_m2: soil_delta,
                                            owner_soil_temperature_k:
                                                owner_soil_temperature,
                                            absolute_tolerances: tolerances,
                                            algebraic_side_constraints_satisfied:
                                                frozen_lane_enthalpies.len() == lane_ids.len(),
                                        },
                                    )?
                                }
                            };
                            let finalization_inputs =
                                CoveredFinalizationEquivalentReplayInputsV1 {
                                    proposed_stage3,
                                    proposed_soil,
                                    input_covered_boundaries: corrected_boundaries.clone(),
                                    input_open_boundaries: trial_open_boundaries,
                                    destination_receipts,
                                };
                            let phase_branch = lane_ids
                                .iter()
                                .map(|lane_id| {
                                    if coordinate_posture
                                        == CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                                    {
                                        Ok(0)
                                    } else {
                                        let phase = phases
                                            .as_ref()
                                            .and_then(|phases| phases.get(lane_id))
                                            .ok_or(
                                                PhaseConsistentCoupledSolveErrorV1::Structure,
                                            )?;
                                        covered_canonical_phase_predicate_v1(
                                            phase.water_kg_m2,
                                            phase.enthalpy_j_m2,
                                        )
                                    }
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            let branch_identity =
                                CoveredPhaseConsistentPhysicalBranchIdentityV1 {
                                    phase_branch,
                                    density_model_branch: actual_density_model_branch,
                                };
                            let snow_enthalpy_material_owner = if coordinate_posture
                                == CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
                            {
                                Some(covered_frozen_temperature_primary_compound_owner_v1(
                                    input.support,
                                    provisional.transaction_id,
                                    &lane_ids,
                                    &self.stage3_beginning_by_lane,
                                    stage3.0.clone(),
                                    &frozen_lane_enthalpies,
                                    &stage3.2,
                                    &physical_snow_soil_receipts,
                                    &branch_identity,
                                    self.beginning_snow_enthalpy_material_owner.as_ref(),
                                )?)
                            } else {
                                None
                            };
                            let artifacts = CoveredPhaseConsistentPhysicalArtifactsV1 {
                                stage3_candidate: stage3.0,
                                stage3_support_images: stage3.2,
                                corrected_boundaries,
                                lse_states: provisional.lse_states,
                                precipitation_sets: provisional.precipitation_sets,
                                transaction_id: provisional.transaction_id,
                                soil_candidates: provisional.soil_candidates,
                                soil_candidate,
                                cn_trial_operands,
                                snow_enthalpy_material_owner,
                            };
                            Ok((
                                evaluation,
                                artifacts,
                                finalization_inputs,
                                branch_identity,
                            ))
                                    },
                                )?;
                            charged_map.validate()?;
                            let (
                                residual,
                                artifacts,
                                finalization_inputs,
                                branch_identity,
                            ) = charged_map.value;
                            Ok(CoveredPhaseConsistentPhysicalEvaluationV1 {
                                residual,
                                artifacts,
                                finalization_inputs,
                                branch_identity,
                                coordinate_posture,
                                physical_evaluation_ordinal:
                                    charged_map.physical_evaluation_ordinal,
                            })
                        };
                        let (mut solve_root, mut solve_trust_radius) = match initial_coordinate_posture {
                            CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                                let solve = covered_frozen_temperature_primary_solve_v1(
                                    initial_coordinates,
                                    &mut physical_evaluation_budget,
                                    |coordinates, budget| {
                                        physical_evaluate(
                                            coordinates,
                                            budget,
                                            None,
                                            CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
                                        )
                                    },
                            )
                            .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "frozen temperature-primary safeguarded physical solve",
                                    )
                                })?;
                                (solve.root, solve.trust_radius)
                            }
                            CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                                let solve = phase_consistent_coupled_physical_solve_v1(
                                    initial_coordinates,
                                    &mut physical_evaluation_budget,
                                    |coordinates, budget| {
                                        physical_evaluate(
                                            coordinates,
                                            budget,
                                            None,
                                            CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
                                        )
                                    },
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "phase-consistent safeguarded physical solve",
                                    )
                                })?;
                                (solve.root, solve.trust_radius)
                            }
                        };
                        if initial_coordinate_posture
                            == CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
                        {
                            let post_root_transition =
                                covered_frozen_temperature_primary_post_root_transition_v1(
                                    &solve_root,
                                    &lane_ids,
                                    &self.stage3_beginning_by_lane,
                                    beginning_soil_ofes.len(),
                                    &physical_evaluation_budget,
                                    3 * lane_ids.len() + 2 * beginning_soil_ofes.len() + 1
                                        + COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1,
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "frozen temperature-primary post-root eligibility",
                                    )
                                })?;
                            if let Some(transition) = post_root_transition {
                                frozen_temperature_primary_committed.set(true);
                                let beginning_carries =
                                    covered_frozen_temperature_primary_beginning_carries_v1(
                                        &self.stage3_beginning_by_lane,
                                        &lane_ids,
                                        self.beginning_snow_enthalpy_material_owner.as_ref(),
                                    )
                                    .map_err(|_| {
                                        DirectV11RealConsumerError::Identity(
                                            "V57 post-root beginning exact snow enthalpy carry",
                                        )
                                    })?;
                                frozen_temperature_primary_beginning_carries
                                    .replace(Some(beginning_carries));
                                active_coordinate_posture.set(
                                    CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary,
                                );
                                let solve = covered_frozen_temperature_primary_solve_v1(
                                    transition.seed_coordinates,
                                    &mut physical_evaluation_budget,
                                    |coordinates, budget| {
                                        physical_evaluate(
                                            coordinates,
                                            budget,
                                            None,
                                            CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
                                        )
                                    },
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "frozen temperature-primary post-root safeguarded physical solve",
                                    )
                                })?;
                                solve_root = solve.root;
                                solve_trust_radius = solve.trust_radius;
                            }
                        }
                        coupled_root_exists.set(true);
                        let polished = phase_consistent_coupled_root_polish_v1(
                            solve_root,
                            solve_trust_radius,
                            &mut physical_evaluation_budget,
                            |coordinates, budget| {
                                physical_evaluate(
                                    coordinates,
                                    budget,
                                    None,
                                    CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
                                )
                            },
                        )
                        .map_err(|_| {
                            DirectV11RealConsumerError::AdaptiveRefinement(
                                "phase-consistent coupled physical root polishing",
                            )
                        })?;
                        let private_replay_inputs =
                            polished.evaluation.finalization_inputs.clone();
                        authentic_replay_input_exchange.replace(Some(
                            private_replay_inputs.input_covered_boundaries,
                        ));
                        let private_root = &polished.evaluation.artifacts;
                        let authentic_receipts = self.snow_soil_heat_receipts_for_candidate_v1(
                            input.support,
                            &private_root.stage3_candidate,
                            &private_root.soil_candidate,
                        )?;
                        let stabilization_outcome = covered_authentic_receipt_stabilize_or_cycle_v1(
                            authentic_receipts,
                            &mut physical_evaluation_budget,
                            |kind, receipts, budget| {
                                let evaluation = physical_evaluate(
                                    &polished.evaluation.residual.coordinates,
                                    budget,
                                    Some(receipts),
                                    kind,
                                )?;
                                let reconstructed_receipts = self
                                    .snow_soil_heat_receipts_for_candidate_v1(
                                        input.support,
                                        &evaluation.artifacts.stage3_candidate,
                                        &evaluation.artifacts.soil_candidate,
                                    )
                                    .map_err(|_| {
                                        PhaseConsistentCoupledSolveErrorV1::ReplayMismatch
                                    })?;
                                Ok((
                                    evaluation.residual,
                                    evaluation.artifacts,
                                    evaluation.finalization_inputs,
                                    reconstructed_receipts,
                                ))
                            },
                        )
                        .map_err(|_| {
                            DirectV11RealConsumerError::AdaptiveRefinement(
                                "phase-consistent authentic receipt stabilization",
                            )
                        })?;
                        let stabilization = match stabilization_outcome {
                            CoveredAuthenticReceiptStabilizationOutcomeV1::Stabilized(
                                stabilized,
                            ) => *stabilized,
                            CoveredAuthenticReceiptStabilizationOutcomeV1::ExactCycle(cycle) => {
                                let expected_branch =
                                    polished.evaluation.branch_identity.clone();
                                covered_authentic_receipt_cycle_endpoint_witness_v1(
                                    &cycle,
                                    &mut physical_evaluation_budget,
                                    &expected_branch,
                                    |member| {
                                        covered_receipt_cycle_endpoint_coordinates_v1(
                                            member,
                                            &lane_ids,
                                        )
                                    },
                                    |kind, coordinates, receipts, budget| {
                                        let evaluation = physical_evaluate(
                                            coordinates,
                                            budget,
                                            Some(receipts),
                                            kind,
                                        )?;
                                        let reconstructed_receipts = self
                                            .snow_soil_heat_receipts_for_candidate_v1(
                                                input.support,
                                                &evaluation.artifacts.stage3_candidate,
                                                &evaluation.artifacts.soil_candidate,
                                            )
                                            .map_err(|_| {
                                                PhaseConsistentCoupledSolveErrorV1::ReplayMismatch
                                            })?;
                                        Ok((evaluation, reconstructed_receipts))
                                    },
                                )
                                .map_err(|_| {
                                    DirectV11RealConsumerError::AdaptiveRefinement(
                                        "phase-consistent authentic receipt-cycle endpoint witness",
                                    )
                                })?
                            }
                        };
                        if stabilization.publication_eligible
                            || stabilization.independent_replay_count != 1
                            || !CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(
                                false,
                                stabilization.residual.scaled_merit <= 1.0
                                    && stabilization
                                        .residual
                                        .algebraic_side_constraints_satisfied,
                                true,
                            )
                        {
                            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                                "phase-consistent coupled authentic admission",
                            ));
                        }
                        Ok((
                            stabilization.artifacts,
                            stabilization.stabilized_receipts,
                            stabilization.finalization_inputs,
                        ))
                        });
                        match coupled_result {
                            Ok(coupled) => {
                                accepted_snow_soil_receipts = coupled.1.clone();
                                coupled_artifacts = Some(coupled.0.clone());
                                coupled_finalization_inputs = Some(coupled.2.clone());
                            }
                            Err(_)
                                if stable_monotone_coupled_active
                                    && !coupled_root_exists.get()
                                    && !frozen_temperature_primary_committed.get()
                                    && physical_evaluation_budget.used
                                        < physical_evaluation_budget.maximum =>
                            {
                                // Version 34 does not admit or retain any
                                // private pre-root state. Resume the ordinary
                                // raw authentic map with only the already
                                // depleted shared budget.
                                covered_stable_monotone_disable_after_pre_root_refusal_v1(
                                    &mut stable_monotone_trace,
                                    &mut stable_monotone_pre_root_refusal_disabled,
                                );
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    if let Some(artifacts) = coupled_artifacts {
                        if let Some(owner) = artifacts.snow_enthalpy_material_owner.as_ref() {
                            owner.validate().map_err(|_| {
                                DirectV11RealConsumerError::Identity(
                                    "V56 accepted compound snow material owner",
                                )
                            })?;
                            accepted_snow_enthalpy_material_owner = Some(owner.clone());
                        }
                        stage3_candidate = artifacts.stage3_candidate;
                        next_covered_boundaries = artifacts.corrected_boundaries.clone();
                        next_boundaries = artifacts.corrected_boundaries;
                        lse_states = artifacts.lse_states;
                        soil_candidate = artifacts.soil_candidate;
                        converged = true;
                    }
                    if converged {
                        // Continue below into the unchanged authentic
                        // finalization path with the replayed physical root.
                    } else {
                        let next_iteration_stage3 = active_set_iterate
                            .as_ref()
                            .map(|active_set| active_set.iterate.clone())
                            .or_else(|| {
                                relaxation_weight.and_then(|weight| {
                                    covered_fixed_point_stage3_underrelaxed_iterate_v1(
                                        &iteration_stage3_states,
                                        &stage3_candidate,
                                        weight,
                                    )
                                })
                            });
                        let next_iteration_soil = relaxation_weight.and_then(|weight| {
                            covered_fixed_point_soil_candidate_underrelaxed_iterate_v1(
                                &iteration_soil_state,
                                &soil_candidate,
                                weight,
                            )
                        });
                        previous_lse_states = Some(lse_states);
                        previous_previous_stage3_states = previous_stage3_states.take();
                        previous_stage3_states = Some(stage3_candidate.clone());
                        iteration_phase_support_images = Some(
                            active_set_iterate
                                .map(|active_set| active_set.support_images)
                                .unwrap_or(stage3_support_images),
                        );
                        iteration_stage3_states = next_iteration_stage3.unwrap_or(stage3_candidate);
                        previous_soil_state = Some(soil_candidate.clone());
                        iteration_soil_state = next_iteration_soil.unwrap_or(soil_candidate);
                        iteration_boundaries = Some(next_covered_boundaries);
                        previous_complete_boundaries = Some(next_boundaries);
                        continue;
                    }
                }

                // Re-seal from the converged candidate endpoints. These are
                // the identities retained by the parent join and replayed for
                // exact installation; the preceding receipt was only the
                // fixed-point operand generated from the prior trial.
                let finalization_started = phase_start();
                let finalization_candidate_started = phase_start();
                let final_snow_soil_receipts = self.snow_soil_heat_receipts_for_candidate_v1(
                    input.support,
                    &stage3_candidate,
                    &soil_candidate,
                )?;
                accepted_snow_soil_receipts = self.retain_terminal_limiting_snow_soil_receipts(
                    final_snow_soil_receipts,
                    &accepted_snow_soil_receipts,
                    &stage3_candidate,
                );

                let mut final_candidate = covered_boxed_execution_v1(|| {
                    Ok::<_, DirectV11RealConsumerError>(self.beginning.clone())
                })?;
                final_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let (
                    final_input_boundaries,
                    final_input_open_boundaries,
                    final_map_stage3,
                    final_map_soil,
                    final_next_destination_receipts,
                ) = if let Some(replay) = coupled_finalization_inputs.as_ref() {
                    (
                        replay.input_covered_boundaries.clone(),
                        replay.input_open_boundaries.clone(),
                        replay.proposed_stage3.clone(),
                        replay.proposed_soil.clone(),
                        replay.destination_receipts.clone(),
                    )
                } else {
                    let input_boundaries = self.merge_latest_stage3_state_operands(
                        &next_covered_boundaries,
                        &stage3_candidate,
                    )?;
                    let (open_diagnostics, open_boundaries, _) =
                        self.open_snow_boundaries_by_destination(&stage3_candidate)?;
                    let mut destination_receipts = initial_diagnostic_receipts.clone();
                    for (destination, digest) in open_diagnostics {
                        if destination_receipts.insert(destination, digest).is_some() {
                            return Err(DirectV11RealConsumerError::Identity(
                                "covered/open finalization receipt intersection",
                            ));
                        }
                    }
                    (
                        input_boundaries,
                        open_boundaries,
                        stage3_candidate.clone(),
                        soil_candidate.clone(),
                        destination_receipts,
                    )
                };
                let final_unpublished_soil_candidate =
                    matches!(&final_map_soil, DirectSoilThermalCandidate::V2(_))
                        .then_some(&final_map_soil);
                let final_envelope = covered_boxed_execution_v1(|| {
                    self.build_covered_carrier_envelope_value_with_soil_beginning_v1(
                        CoveredCarrierEnvelopeBuildV1 {
                            candidate: &final_candidate,
                            interval_s,
                            duration_s_bits: input.duration_s_bits,
                            covered_destinations: &covered_destinations,
                            covered_boundaries: &final_input_boundaries,
                            open_boundaries: &final_input_open_boundaries,
                            provisional: false,
                            finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
                        },
                        final_unpublished_soil_candidate,
                        None,
                    )
                })?;
                let final_precipitation_sets =
                    self.precipitation_parcel_sets(input.support, &final_envelope)?;
                let (final_corrected_boundaries, final_shortwave_by_lane, final_longwave_by_lane) =
                    self.corrected_covered_boundaries_from_envelope(
                        &final_input_boundaries,
                        &final_envelope,
                    )?;
                let final_lse_states = final_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("covered final LSE iteration state")
                    })?;
                let final_rebuilt_boundaries = self
                    .apply_lse_iteration_exchange(&final_corrected_boundaries, &final_lse_states)?;
                let mut final_complete_boundaries = final_rebuilt_boundaries.clone();
                for (destination, boundary) in final_input_open_boundaries {
                    final_complete_boundaries.insert(destination, boundary);
                }
                let final_complete_boundaries = self.merge_latest_stage3_state_operands(
                    &final_complete_boundaries,
                    &final_map_stage3,
                )?;
                let (final_stage3_candidate, _, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_boundaries,
                    None,
                    &accepted_snow_soil_receipts,
                    None,
                    None,
                    &final_precipitation_sets,
                )?;
                profile_record("finalization candidate", finalization_candidate_started);
                let final_lse_converged =
                    covered_fixed_point_lse_states_equal(&lse_states, &final_lse_states);
                let final_stage3_converged = covered_fixed_point_stage3_states_equal(
                    &stage3_candidate,
                    &final_stage3_candidate,
                );
                let final_boundary_converged = covered_fixed_point_boundaries_equal(
                    &final_input_boundaries,
                    &final_rebuilt_boundaries,
                );
                if !final_boundary_converged || !final_lse_converged || !final_stage3_converged {
                    if crate::snow_stage3_v11_attachment::covered_fixed_point_limiter_audit_enabled_v1()
                    {
                        record_covered_limiter_sample_v1(
                            input.support,
                            iteration + 1,
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::Finalization,
                            (
                                final_lse_converged,
                                final_stage3_converged,
                                true,
                                final_boundary_converged,
                            ),
                            (
                                covered_lse_max_normalized_delta_v1(
                                    Some(&lse_states),
                                    &final_lse_states,
                                ),
                                covered_stage3_max_normalized_delta_v1(
                                    Some(&stage3_candidate),
                                    &final_stage3_candidate,
                                ),
                                0.0,
                                0.0,
                                covered_boundary_max_normalized_delta_v1(
                                    Some(&final_input_boundaries),
                                    &final_rebuilt_boundaries,
                                ),
                            ),
                        );
                    }
                    if iteration + 1 == COVERED_FIXED_POINT_POLICY.max_iterations {
                        crate::snow_stage3_v11_attachment::record_covered_fixed_point_limit_detail_v1(
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitDetailV1 {
                            stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::Finalization,
                            lse_converged: final_lse_converged,
                            stage3_converged: final_stage3_converged,
                            soil_converged: true,
                            boundary_converged: final_boundary_converged,
                            stage3_first_difference: covered_stage3_state_first_difference_v1(
                                &stage3_candidate,
                                &final_stage3_candidate,
                            ),
                        });
                    }
                    phase_record("finalization", finalization_started);
                    previous_lse_states = Some(final_lse_states);
                    previous_previous_stage3_states = previous_stage3_states.take();
                    previous_stage3_states = Some(final_stage3_candidate.clone());
                    let next_iteration_stage3 = covered_fixed_point_finalization_stage3_iterate_v1(
                        &stage3_candidate,
                        &final_stage3_candidate,
                        input.support.duration_ns(),
                        exact_floor_period_two_relaxation_enabled,
                    );
                    finalization_stabilization
                        .observe_restart(next_iteration_stage3 != final_stage3_candidate);
                    covered_stable_monotone_clear_on_finalization_restart_v1(
                        &mut stable_monotone_trace,
                        &mut stable_monotone_pre_root_refusal_disabled,
                    );
                    phase_consistent_parity_monotone_trace.clear();
                    iteration_stage3_states = next_iteration_stage3;
                    previous_soil_state = Some(soil_candidate.clone());
                    iteration_soil_state = soil_candidate;
                    iteration_boundaries = Some(final_rebuilt_boundaries);
                    previous_complete_boundaries = Some(final_complete_boundaries);
                    continue;
                }
                let sealed_source_started = phase_start();
                let sealed_source_input_boundaries = self.merge_latest_stage3_state_operands(
                    &final_rebuilt_boundaries,
                    &final_stage3_candidate,
                )?;
                let sealed_source_open_boundaries = self
                    .open_snow_boundaries_by_destination(&final_stage3_candidate)?
                    .1;
                let sealed_source_envelope = covered_boxed_execution_v1(|| {
                    self.build_covered_carrier_envelope_value_with_soil_beginning_v1(
                        CoveredCarrierEnvelopeBuildV1 {
                            candidate: &final_candidate,
                            interval_s,
                            duration_s_bits: input.duration_s_bits,
                            covered_destinations: &covered_destinations,
                            covered_boundaries: &sealed_source_input_boundaries,
                            open_boundaries: &sealed_source_open_boundaries,
                            provisional: false,
                            finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
                        },
                        final_unpublished_soil_candidate,
                        None,
                    )
                })?;
                let (sealed_source_corrected_boundaries, _, _) = self
                    .corrected_covered_boundaries_from_envelope(
                        &sealed_source_input_boundaries,
                        &sealed_source_envelope,
                    )?;
                let sealed_source_lse_states = sealed_source_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered sealed-source LSE iteration state",
                        )
                    })?;
                let sealed_source_boundaries = self.apply_lse_iteration_exchange(
                    &sealed_source_corrected_boundaries,
                    &sealed_source_lse_states,
                )?;
                let sealed_source_boundaries = self.merge_latest_stage3_state_operands(
                    &sealed_source_boundaries,
                    &final_stage3_candidate,
                )?;
                let ending_v8_physical_candidate_sha256 = digest32_from_lower_hex(
                    &sealed_source_envelope
                        .vegetation()
                        .ending_state()
                        .state_sha256,
                )?;
                let ending_stage3_state_sha256 = digest_bytes(
                    &canonical_stage3_snow_owner_bytes_v11(&final_stage3_candidate)?,
                );
                let (final_covered_lower_boundaries, final_covered_boundary_receipts) = self
                    .seal_final_covered_boundaries(
                        input.support,
                        digest32_from_lower_hex(&input.beginning.0.state_sha256)?,
                        &self.stage3_beginning_by_lane,
                        &sealed_source_boundaries,
                        &initial_guess_receipts,
                        &sealed_source_envelope,
                        ending_v8_physical_candidate_sha256,
                        ending_stage3_state_sha256,
                    )?;
                let (final_open_lower_boundaries, final_open_boundary_receipts) = self
                    .seal_final_open_snow_boundaries(
                        &final_stage3_candidate,
                        ending_stage3_state_sha256,
                    )?;
                let final_boundary_receipts = self.complete_final_boundary_receipts(
                    final_covered_boundary_receipts,
                    final_open_boundary_receipts,
                )?;
                let final_lane_boundary_receipts = self.final_lane_boundary_receipts(
                    input.support,
                    &final_boundary_receipts,
                    &final_precipitation_sets,
                )?;
                let final_envelope = covered_boxed_execution_v1(|| {
                    self.build_covered_carrier_envelope_value_with_soil_beginning_v1(
                        CoveredCarrierEnvelopeBuildV1 {
                            candidate: &final_candidate,
                            interval_s,
                            duration_s_bits: input.duration_s_bits,
                            covered_destinations: &covered_destinations,
                            covered_boundaries: &final_covered_lower_boundaries,
                            open_boundaries: &final_open_lower_boundaries,
                            provisional: false,
                            finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
                        },
                        final_unpublished_soil_candidate,
                        None,
                    )
                })?;
                let (self_reconstructed_boundaries, _, _) = self
                    .corrected_covered_boundaries_from_envelope(
                        &final_covered_lower_boundaries,
                        &final_envelope,
                    )?;
                let self_reconstructed_lse_states = final_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("covered sealed LSE iteration state")
                    })?;
                let self_reconstructed_boundaries = self.apply_lse_iteration_exchange(
                    &self_reconstructed_boundaries,
                    &self_reconstructed_lse_states,
                )?;
                if !covered_fixed_point_boundaries_equal(
                    &final_covered_lower_boundaries,
                    &self_reconstructed_boundaries,
                ) || !covered_fixed_point_lse_states_equal(
                    &sealed_source_lse_states,
                    &self_reconstructed_lse_states,
                ) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "final covered boundary self-reconstruction",
                    ));
                }
                let mut final_complete_lower_boundaries = final_covered_lower_boundaries.clone();
                for (destination, boundary) in final_open_lower_boundaries {
                    final_complete_lower_boundaries.insert(destination, boundary);
                }
                let (final_ending_stage3, _, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_lower_boundaries,
                    Some(&final_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    None,
                    None,
                    &final_precipitation_sets,
                )?;
                #[cfg(test)]
                let final_ending_stage3 = {
                    let mut reconstructed = final_ending_stage3;
                    apply_covered_receipt_reseal_density_perturbation_for_test(&mut reconstructed);
                    reconstructed
                };
                profile_record("finalization sealed source", sealed_source_started);
                if !covered_fixed_point_stage3_states_equal(
                    &final_stage3_candidate,
                    &final_ending_stage3,
                ) {
                    if crate::snow_stage3_v11_attachment::covered_fixed_point_limiter_audit_enabled_v1()
                    {
                        record_covered_limiter_sample_v1(
                            input.support,
                            iteration + 1,
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::ReceiptReplay,
                            (true, false, true, true),
                            (
                                0.0,
                                covered_stage3_max_normalized_delta_v1(
                                    Some(&final_stage3_candidate),
                                    &final_ending_stage3,
                                ),
                                0.0,
                                0.0,
                                0.0,
                            ),
                        );
                    }
                    if iteration + 1 == COVERED_FIXED_POINT_POLICY.max_iterations {
                        crate::snow_stage3_v11_attachment::record_covered_fixed_point_limit_detail_v1(
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitDetailV1 {
                            stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::ReceiptReplay,
                            lse_converged: true,
                            stage3_converged: false,
                            soil_converged: true,
                            boundary_converged: true,
                            stage3_first_difference: covered_stage3_state_first_difference_v1(
                                &final_stage3_candidate,
                                &final_ending_stage3,
                            ),
                        });
                    }
                    // The accepted lane receipts are part of the physical
                    // fixed-point mapping. A tolerance-equivalent candidate
                    // is not converged until replaying those sealed receipts
                    // reconstructs the exact Stage-3 owner state. Feed that
                    // reconstructed state back as the next iterate; the
                    // existing iteration cap remains the fail-closed guard
                    // for a cycle or noncontracting receipt mapping.
                    phase_record("finalization", finalization_started);
                    covered_stable_monotone_clear_on_finalization_restart_v1(
                        &mut stable_monotone_trace,
                        &mut stable_monotone_pre_root_refusal_disabled,
                    );
                    phase_consistent_parity_monotone_trace.clear();
                    previous_lse_states = Some(self_reconstructed_lse_states);
                    previous_previous_stage3_states = previous_stage3_states.take();
                    previous_stage3_states = Some(final_ending_stage3.clone());
                    iteration_stage3_states = final_ending_stage3;
                    previous_soil_state = Some(soil_candidate.clone());
                    iteration_soil_state = soil_candidate;
                    iteration_boundaries = Some(self_reconstructed_boundaries);
                    previous_complete_boundaries = Some(final_complete_lower_boundaries);
                    continue;
                }
                // The retained receipts must describe the candidate that is
                // actually installed, not the tolerance-equivalent precursor
                // used to discover the fixed point.  Re-seal from the replay
                // outputs, then prove that receipt metadata cannot perturb any
                // physical result.
                let install_started = phase_start();
                let installed_v8_digest = digest32_from_lower_hex(
                    &final_envelope.vegetation().ending_state().state_sha256,
                )?;
                let installed_stage3_digest = digest_bytes(&canonical_stage3_snow_owner_bytes_v11(
                    &final_ending_stage3,
                )?);
                let (installed_covered_lower_boundaries, installed_covered_boundary_receipts) =
                    self.seal_final_covered_boundaries(
                        input.support,
                        digest32_from_lower_hex(&input.beginning.0.state_sha256)?,
                        &self.stage3_beginning_by_lane,
                        &self_reconstructed_boundaries,
                        &initial_guess_receipts,
                        &final_envelope,
                        installed_v8_digest,
                        installed_stage3_digest,
                    )?;
                let (installed_open_lower_boundaries, installed_open_boundary_receipts) = self
                    .seal_final_open_snow_boundaries(
                        &final_stage3_candidate,
                        installed_stage3_digest,
                    )?;
                let installed_boundary_receipts = self.complete_final_boundary_receipts(
                    installed_covered_boundary_receipts,
                    installed_open_boundary_receipts,
                )?;
                let installed_component_carrier_receipts = self_reconstructed_lse_states
                    .iter()
                    .map(|(destination, state)| {
                        let boundary = installed_boundary_receipts
                            .get(destination)
                            .and_then(|value| match value {
                                FinalStage3TileBoundaryReceiptV1::V11Canopy(value) => Some(value),
                                FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => None,
                            })
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "installed component carrier boundary destination",
                            ))?;
                        Ok((
                            destination.clone(),
                            ComponentResolvedCarrierReceiptV1::try_new(
                                destination.clone(),
                                state,
                                boundary,
                            )?,
                        ))
                    })
                    .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
                let installed_envelope = covered_boxed_execution_v1(|| {
                    self.build_covered_carrier_envelope_value_with_soil_beginning_v1(
                        CoveredCarrierEnvelopeBuildV1 {
                            candidate: &final_candidate,
                            interval_s,
                            duration_s_bits: input.duration_s_bits,
                            covered_destinations: &covered_destinations,
                            covered_boundaries: &installed_covered_lower_boundaries,
                            open_boundaries: &installed_open_lower_boundaries,
                            provisional: false,
                            finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
                        },
                        final_unpublished_soil_candidate,
                        None,
                    )
                })?;
                let installed_precipitation_sets =
                    self.precipitation_parcel_sets(input.support, &installed_envelope)?;
                let installed_lane_boundary_receipts = self.final_lane_boundary_receipts(
                    input.support,
                    &installed_boundary_receipts,
                    &installed_precipitation_sets,
                )?;
                let installed_lse_states = installed_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "installed covered LSE iteration state",
                        )
                    })?;
                if installed_lse_states != self_reconstructed_lse_states
                    || installed_envelope.vegetation().ending_state()
                        != final_envelope.vegetation().ending_state()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed covered replay exact physical identity",
                    ));
                }
                let mut installed_complete_lower_boundaries =
                    installed_covered_lower_boundaries.clone();
                for (destination, boundary) in installed_open_lower_boundaries {
                    installed_complete_lower_boundaries.insert(destination, boundary);
                }
                let (installed_stage3, installed_cold_content_export_by_lane, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_complete_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    None,
                    None,
                    &installed_precipitation_sets,
                )?;
                if installed_stage3 != final_ending_stage3 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed Stage-3 replay exact physical identity",
                    ));
                }
                let installed_soil = self.unpublished_soil_candidate_for_covered_envelope_v1(
                    input.support,
                    &installed_envelope,
                    &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
                )?;
                let installed_snow_soil_receipts = self
                    .retain_terminal_limiting_snow_soil_receipts(
                        self.snow_soil_heat_receipts_for_candidate_v1(
                            input.support,
                            &installed_stage3,
                            &installed_soil,
                        )?,
                        &accepted_snow_soil_receipts,
                        &installed_stage3,
                    );
                self.validate_snow_soil_heat_receipt_iterate_joins(
                    &installed_snow_soil_receipts,
                    &installed_stage3,
                    installed_soil.read_view(),
                )?;
                if installed_snow_soil_receipts.len() != accepted_snow_soil_receipts.len()
                    || installed_snow_soil_receipts
                        .keys()
                        .ne(accepted_snow_soil_receipts.keys())
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "snow-soil installed receipt lane topology",
                    ));
                }
                let installed_receipt_topology_changed =
                    installed_snow_soil_receipts
                        .iter()
                        .any(|(lane_id, installed)| {
                            accepted_snow_soil_receipts
                                .get(lane_id)
                                .is_none_or(|accepted| {
                                    installed.lane_id != accepted.lane_id
                                        || installed.ofe_id != accepted.ofe_id
                                        || installed.bottom_snow_layer_id
                                            != accepted.bottom_snow_layer_id
                                        || installed.first_soil_layer_id
                                            != accepted.first_soil_layer_id
                                })
                        });
                if installed_receipt_topology_changed {
                    return Err(DirectV11RealConsumerError::Identity(
                        "snow-soil installed receipt node topology",
                    ));
                }
                let installed_receipt_max_abs_temperature_residual = installed_snow_soil_receipts
                    .iter()
                    .try_fold(0.0_f64, |maximum, (lane_id, installed)| {
                        let accepted = accepted_snow_soil_receipts.get(lane_id).ok_or(
                            DirectV11RealConsumerError::Identity(
                                "snow-soil installed receipt temperature lane",
                            ),
                        )?;
                        Ok::<_, DirectV11RealConsumerError>(
                            maximum
                                .max(
                                    (installed.ending_bottom_snow_temperature_k
                                        - accepted.ending_bottom_snow_temperature_k)
                                        .abs(),
                                )
                                .max(
                                    (installed.ending_top_soil_temperature_k
                                        - accepted.ending_top_soil_temperature_k)
                                        .abs(),
                                ),
                        )
                    })?;
                let installed_receipt_max_abs_energy_residual = installed_snow_soil_receipts
                    .iter()
                    .try_fold(0.0_f64, |maximum, (lane_id, installed)| {
                        let accepted = accepted_snow_soil_receipts.get(lane_id).ok_or(
                            DirectV11RealConsumerError::Identity(
                                "snow-soil installed receipt energy lane",
                            ),
                        )?;
                        Ok::<_, DirectV11RealConsumerError>(
                            maximum
                                .max(
                                    (installed.accepted_heat_j_m2_ofe_ground
                                        - accepted.accepted_heat_j_m2_ofe_ground)
                                        .abs(),
                                )
                                .max(
                                    (installed.snow_candidate_heat_j_m2_ofe_ground
                                        - accepted.snow_candidate_heat_j_m2_ofe_ground)
                                        .abs(),
                                )
                                .max(
                                    (installed.soil_candidate_heat_j_m2_ofe_ground
                                        - accepted.soil_candidate_heat_j_m2_ofe_ground)
                                        .abs(),
                                ),
                        )
                    })?;
                profile_record("finalization install", install_started);
                if !snow_soil_receipt_reseal_roundoff_within_bound_v1(
                    installed_receipt_max_abs_energy_residual,
                    installed_receipt_max_abs_temperature_residual,
                ) {
                    if crate::snow_stage3_v11_attachment::covered_fixed_point_limiter_audit_enabled_v1()
                    {
                        let (soil_enthalpy_delta, soil_temperature_delta) =
                            covered_soil_max_normalized_deltas_v1(
                                Some(&iteration_soil_state),
                                &installed_soil,
                            );
                        record_covered_limiter_sample_v1(
                            input.support,
                            iteration + 1,
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::ReceiptReplay,
                            (true, false, false, true),
                            (
                                0.0,
                                covered_stage3_max_normalized_delta_v1(
                                    Some(&iteration_stage3_states),
                                    &installed_stage3,
                                ),
                                soil_enthalpy_delta,
                                soil_temperature_delta,
                                0.0,
                            ),
                        );
                    }
                    if iteration + 1 == COVERED_FIXED_POINT_POLICY.max_iterations {
                        crate::snow_stage3_v11_attachment::record_covered_fixed_point_limit_detail_v1(
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitDetailV1 {
                                stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::ReceiptReplay,
                                lse_converged: true,
                                stage3_converged: false,
                                soil_converged: false,
                                boundary_converged: true,
                                stage3_first_difference: covered_stage3_state_first_difference_v1(
                                    &iteration_stage3_states,
                                    &installed_stage3,
                                ),
                            },
                        );
                    }
                    let relaxation_weight = covered_fixed_point_relaxation_weight_v1(
                        input.support.duration_ns(),
                        exact_floor_period_two_relaxation_enabled,
                    );
                    // Receipt resealing is itself a coupled endpoint map and
                    // uses the same guarded owner relaxation. The exact floor
                    // remains raw unless the authentic Stage-3 cycle detector
                    // has already enabled contraction. Cap exhaustion remains
                    // fail-closed.
                    let next_iteration_stage3 = relaxation_weight.and_then(|weight| {
                        covered_fixed_point_stage3_underrelaxed_iterate_v1(
                            &iteration_stage3_states,
                            &installed_stage3,
                            weight,
                        )
                    });
                    let next_iteration_soil = relaxation_weight.and_then(|weight| {
                        covered_fixed_point_soil_candidate_underrelaxed_iterate_v1(
                            &iteration_soil_state,
                            &installed_soil,
                            weight,
                        )
                    });
                    phase_record("finalization", finalization_started);
                    covered_stable_monotone_clear_on_finalization_restart_v1(
                        &mut stable_monotone_trace,
                        &mut stable_monotone_pre_root_refusal_disabled,
                    );
                    phase_consistent_parity_monotone_trace.clear();
                    previous_lse_states = Some(installed_lse_states);
                    previous_previous_stage3_states = previous_stage3_states.take();
                    previous_stage3_states = Some(installed_stage3.clone());
                    iteration_stage3_states = next_iteration_stage3.unwrap_or(installed_stage3);
                    previous_soil_state = Some(installed_soil.clone());
                    iteration_soil_state = next_iteration_soil.unwrap_or(installed_soil);
                    iteration_boundaries = Some(final_rebuilt_boundaries);
                    previous_complete_boundaries = Some(installed_complete_lower_boundaries);
                    continue;
                }
                crate::snow_stage3_v11_attachment::record_covered_receipt_reseal_roundoff_v1(
                    installed_receipt_max_abs_energy_residual,
                    installed_receipt_max_abs_temperature_residual,
                );
                let identity_replay_started = phase_start();
                // Keep the exact equal/opposite heat that both solvers
                // actually consumed. The reconstructed endpoint receipt is a
                // convergence audit, not a replacement physical credit. Once
                // its residual is within the explicit roundoff bounds, bind
                // the consumed receipt to the exact installed candidate
                // identities and reseal its complete digest.
                for (lane_id, accepted) in &mut accepted_snow_soil_receipts {
                    let installed = installed_snow_soil_receipts.get(lane_id).ok_or(
                        DirectV11RealConsumerError::Identity(
                            "snow-soil installed receipt reseal lane",
                        ),
                    )?;
                    accepted.snow_candidate_ending_identity_sha256 =
                        installed.snow_candidate_ending_identity_sha256;
                    accepted.soil_candidate_ending_identity_sha256 =
                        installed.soil_candidate_ending_identity_sha256;
                    *accepted = accepted.clone().seal().map_err(|error| {
                        DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                    })?;
                }
                self.validate_snow_soil_heat_receipt_iterate_joins(
                    &accepted_snow_soil_receipts,
                    &installed_stage3,
                    installed_soil.read_view(),
                )?;
                let (identity_replayed_stage3, _, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_complete_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    None,
                    None,
                    &installed_precipitation_sets,
                )?;
                let identity_replayed_soil = self
                    .unpublished_soil_candidate_for_covered_envelope_v1(
                        input.support,
                        &installed_envelope,
                        &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
                    )?;
                profile_record("finalization identity replay", identity_replay_started);
                if identity_replayed_stage3 != installed_stage3 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "snow-soil identity-only receipt reseal replay",
                    ));
                }
                let installed_soil = select_identity_replayed_soil_candidate_v1(
                    &installed_soil,
                    identity_replayed_soil,
                )?;
                crate::snow_stage3_v11_attachment::record_covered_fixed_point_iteration_audit_v1(
                    input.support,
                    iteration + 1,
                    true,
                );
                phase_record("finalization", finalization_started);
                break 'fixed_point Ok::<_, DirectV11RealConsumerError>((
                    final_candidate,
                    installed_envelope,
                    installed_stage3,
                    installed_complete_lower_boundaries,
                    installed_boundary_receipts,
                    installed_lane_boundary_receipts,
                    final_next_destination_receipts,
                    installed_component_carrier_receipts,
                    final_shortwave_by_lane,
                    final_longwave_by_lane,
                    installed_precipitation_sets,
                    installed_cold_content_export_by_lane,
                    installed_soil,
                ));
            }
            crate::snow_stage3_v11_attachment::record_covered_fixed_point_iteration_audit_v1(
                input.support,
                COVERED_FIXED_POINT_POLICY.max_iterations,
                false,
            );
            Err(DirectV11RealConsumerError::CoveredBoundary(
                SnowStage3HandoffError::FixedPointIterationLimit,
            ))
        })?;
        let candidate = &fixed_point.0;
        let envelope = &fixed_point.1;
        let ending_stage3 = &fixed_point.2;
        let final_boundary_receipts = &fixed_point.4;
        let final_lane_boundary_receipts = &fixed_point.5;
        let final_component_carrier_receipts = &fixed_point.7;
        let installed_precipitation_sets = &fixed_point.10;
        let installed_cold_content_export_by_lane = &fixed_point.11;
        let installed_soil_preview = &fixed_point.12;
        self.validate_snow_soil_heat_receipt_iterate_joins(
            &accepted_snow_soil_receipts,
            &ending_stage3,
            installed_soil_preview.read_view(),
        )?;
        let ending_snow_owner_bytes =
            canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
                &ending_stage3,
                &self.pending_terminal_parcels,
                &final_lane_boundary_receipts,
                &final_boundary_receipts,
            )?;
        let terminal_snow_soil_heat_receipts = self.terminal_snow_soil_heat_receipts(
            input.support,
            &ending_stage3,
            &installed_soil_preview,
            &accepted_snow_soil_receipts,
            &installed_cold_content_export_by_lane,
        )?;
        let physical_outcome_ledgers =
            self.physical_outcome_ledgers(&PhysicalOutcomeLedgerInputs {
                support: input.support,
                ending: &ending_stage3,
                lanes: &final_lane_boundary_receipts,
                destinations: &final_boundary_receipts,
                precipitation: &installed_precipitation_sets,
                soil: &accepted_snow_soil_receipts,
                terminal_soil: &terminal_snow_soil_heat_receipts,
                adaptive_trial_soil: &BTreeMap::new(),
                terminal_events: &BTreeMap::new(),
                diagnostics: &installed_cold_content_export_by_lane,
            })?;
        let (accepted_wb14_child, accepted_wb14_parent) =
            crate::direct_runtime::rebind_wb14_replay_to_accepted_slab(
                envelope
                    .hydrology()
                    .surface_ingress()
                    .wb14_child_replay_bytes(),
                self.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
            )
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered accepted WB14 replay reseal")
            })?;
        self.last_wb14_child_receipt_set_sha256 =
            Some(digest32_hex(digest_bytes(&accepted_wb14_child)));
        self.last_wb14_parent_receipt_set_sha256 = accepted_wb14_parent
            .as_ref()
            .map(|bytes| digest32_hex(digest_bytes(bytes)));
        self.last_wb14_child_replay_bytes = Some(accepted_wb14_child);
        self.last_wb14_parent_replay_bytes = accepted_wb14_parent;
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &candidate,
            input,
            &envelope,
            None,
            None,
            None,
            ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
            &physical_outcome_ledgers,
            AcceptedPublicationFinalizationPostureV1::RetainFinal,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(final_boundary_receipts.clone());
        self.last_lane_boundary_receipts = Some(final_lane_boundary_receipts.clone());
        self.last_component_carrier_receipts = Some(final_component_carrier_receipts.clone());
        self.last_snow_soil_heat_receipts = Some(accepted_snow_soil_receipts);
        self.last_snow_enthalpy_material_owner = accepted_snow_enthalpy_material_owner;
        self.last_precipitation_parcel_sets = Some(installed_precipitation_sets.clone());
        self.last_physical_outcome_ledgers = Some(physical_outcome_ledgers);
        self.last_terminal_snow_soil_heat_receipts = Some(terminal_snow_soil_heat_receipts);
        self.last_adaptive_terminal_snow_soil_trial_receipts = Some(BTreeMap::new());
        self.last_terminal_events = Some(terminal_events.into_inner());
        self.ending_stage3_by_lane = Some(ending_stage3.clone());
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        self.ordinary_physical_reuse_seed = Some(CoveredOrdinaryPhysicalReuseSeedV1 {
            physical_authority: covered_ordinary_physical_authority_v1(input)?,
            envelope: (**envelope).clone(),
        });
        Ok(output)
    }
}

#[cfg(test)]
include!("open_snow_tail_tests.rs");
