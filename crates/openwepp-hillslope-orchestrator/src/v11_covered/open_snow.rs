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
        let evaluate_stage3 = |destination_receipts: &BTreeMap<(OfeId, TileId), Digest32>,
                               boundaries: &BTreeMap<
            (OfeId, TileId),
            Stage3SnowCoveredLowerBoundary,
        >,
                               final_lane_receipts: Option<
            &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
        >| {
            let terms = self.lane_stage3_terms_from_boundaries(
                destination_receipts,
                boundaries,
                interval_s,
            )?;
            let mut ending_stage3 = self.stage3_beginning_by_lane.clone();
            for lane_id in terms.keys() {
                let beginning = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                    DirectV11RealConsumerError::Identity("active Stage-3 beginning lane"),
                )?;
                let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                    DirectV11RealConsumerError::Identity("covered Stage-3 input lane"),
                )?;
                let stage3_forcing = self.stage3_forcing_by_lane.get(lane_id).copied().ok_or(
                    DirectV11RealConsumerError::Identity("covered Stage-3 forcing lane"),
                )?;
                let lane_terms = terms
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane terms",
                    ))?;
                let beginning_stage3_digest =
                    Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning)
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered beginning active-volume surface",
                            )
                        })?
                        .beginning_stage3_state_sha256;
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
                            carrier_receipt_sha256: lane_terms.provisional_carrier_receipt_sha256,
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
                        precipitation_advection_j_m2: 0.0,
                        latent_heat_j_kg,
                        beginning_stage3_state_sha256: beginning_stage3_digest,
                        identity,
                    },
                )?;
                let result = Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                    stage3_inputs,
                    beginning,
                    *lane_id,
                    beginning.next_interval_index,
                    stage3_forcing,
                    boundary,
                )?;
                let flux_tolerance = 1.0e-6_f64;
                let evaluation = &result.evaluation;
                if (evaluation.complete_arm_sensible_j_m2 - boundary.sensible_energy_j_m2).abs()
                    > flux_tolerance
                    || (evaluation.complete_arm_shortwave_j_m2 - boundary.shortwave_energy_j_m2)
                        .abs()
                        > flux_tolerance
                    || (evaluation.complete_arm_latent_j_m2 - boundary.latent_energy_j_m2).abs()
                        > flux_tolerance
                    || (evaluation.complete_arm_longwave_j_m2 - boundary.net_longwave_energy_j_m2)
                        .abs()
                        > flux_tolerance
                    || (evaluation.complete_arm_advected_j_m2
                        - boundary.precipitation_advection_j_m2)
                        .abs()
                        > flux_tolerance
                    || (evaluation.complete_arm_vapor_mass_exchange_kg_m2
                        - boundary.vapor_mass_kg_m2)
                        .abs()
                        > 1.0e-9
                    || result.evaluation.evaluated_seconds.to_bits() != interval_s.to_bits()
                    || result.lifecycle != "active"
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "Stage-3 covered boundary/result ledger join",
                    ));
                }
                if result.terminal_event.is_some() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered adopter received terminal event before terminal chronology",
                    ));
                }
                ending_stage3.insert(*lane_id, result.state);
            }
            Ok::<_, DirectV11RealConsumerError>(ending_stage3)
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
        let mut previous_stage3_states: Option<BTreeMap<u32, DirectSnowStage3PersistentState>> =
            None;
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
        let (
            candidate,
            envelope,
            ending_stage3,
            _final_lower_boundaries,
            final_boundary_receipts,
            final_lane_boundary_receipts,
            _final_destination_receipts,
            final_component_carrier_receipts,
            _final_shortwave_by_lane,
            _final_longwave_by_lane,
        ) = 'fixed_point: {
            for _iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
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
                let mut current_execution_boundaries = current_boundaries.clone();
                for (destination, boundary) in &open_boundaries {
                    current_execution_boundaries.insert(destination.clone(), boundary.clone());
                }
                let mut provisional_candidate = self.beginning.clone();
                provisional_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let provisional_envelope = provisional_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &current_execution_boundaries,
                        true,
                        self.finalize_wb14_parent_interval,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let (next_boundaries, _next_shortwave_by_lane, _next_longwave_by_lane) = self
                    .corrected_covered_boundaries_from_envelope(
                        &current_boundaries,
                        &provisional_envelope,
                    )?;
                let lse_states = provisional_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered provisional LSE iteration state",
                        )
                    })?;
                let next_covered_boundaries =
                    self.apply_lse_iteration_exchange(&next_boundaries, &lse_states)?;
                let mut next_boundaries = next_covered_boundaries.clone();
                for (destination, boundary) in open_boundaries {
                    if next_boundaries.insert(destination, boundary).is_some() {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered/open boundary intersection",
                        ));
                    }
                }
                let next_boundaries = self.merge_latest_stage3_state_operands(
                    &next_boundaries,
                    &iteration_stage3_states,
                )?;
                let stage3_candidate =
                    evaluate_stage3(&destination_receipts, &next_boundaries, None)?;
                let lse_converged = previous_lse_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_lse_states_equal(previous, &lse_states)
                });
                let stage3_converged = previous_stage3_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_stage3_states_equal(previous, &stage3_candidate)
                });
                let boundary_converged =
                    previous_complete_boundaries
                        .as_ref()
                        .is_some_and(|previous| {
                            covered_fixed_point_boundaries_equal(previous, &next_boundaries)
                        });
                let converged = lse_converged && stage3_converged && boundary_converged;
                if !converged {
                    previous_lse_states = Some(lse_states);
                    previous_stage3_states = Some(stage3_candidate.clone());
                    iteration_stage3_states = stage3_candidate;
                    iteration_boundaries = Some(next_covered_boundaries);
                    previous_complete_boundaries = Some(next_boundaries);
                    continue;
                }

                let mut final_candidate = self.beginning.clone();
                final_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let final_input_boundaries = self.merge_latest_stage3_state_operands(
                    &next_covered_boundaries,
                    &stage3_candidate,
                )?;
                let mut final_execution_boundaries = final_input_boundaries.clone();
                for (destination, boundary) in self
                    .open_snow_boundaries_by_destination(&stage3_candidate)?
                    .1
                {
                    final_execution_boundaries.insert(destination, boundary);
                }
                let final_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &final_execution_boundaries,
                        false,
                        self.finalize_wb14_parent_interval,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
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
                let (final_open_diagnostics, final_open_boundaries, _) =
                    self.open_snow_boundaries_by_destination(&stage3_candidate)?;
                let mut final_complete_boundaries = final_rebuilt_boundaries.clone();
                let mut final_next_destination_receipts = initial_diagnostic_receipts.clone();
                for (destination, digest) in final_open_diagnostics {
                    final_next_destination_receipts.insert(destination, digest);
                }
                for (destination, boundary) in final_open_boundaries {
                    final_complete_boundaries.insert(destination, boundary);
                }
                let final_complete_boundaries = self.merge_latest_stage3_state_operands(
                    &final_complete_boundaries,
                    &stage3_candidate,
                )?;
                let final_stage3_candidate = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_boundaries,
                    None,
                )?;
                if !covered_fixed_point_boundaries_equal(
                    &final_input_boundaries,
                    &final_rebuilt_boundaries,
                ) || !covered_fixed_point_lse_states_equal(&lse_states, &final_lse_states)
                    || !covered_fixed_point_stage3_states_equal(
                        &stage3_candidate,
                        &final_stage3_candidate,
                    )
                {
                    previous_lse_states = Some(final_lse_states);
                    previous_stage3_states = Some(final_stage3_candidate.clone());
                    iteration_stage3_states = final_stage3_candidate;
                    iteration_boundaries = Some(final_rebuilt_boundaries);
                    previous_complete_boundaries = Some(final_complete_boundaries);
                    continue;
                }
                let sealed_source_input_boundaries = self.merge_latest_stage3_state_operands(
                    &final_rebuilt_boundaries,
                    &final_stage3_candidate,
                )?;
                let sealed_source_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &{
                            let mut complete = sealed_source_input_boundaries.clone();
                            for (destination, boundary) in self
                                .open_snow_boundaries_by_destination(&final_stage3_candidate)?
                                .1
                            {
                                complete.insert(destination, boundary);
                            }
                            complete
                        },
                        false,
                        self.finalize_wb14_parent_interval,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
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
                        input,
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
                let final_lane_boundary_receipts =
                    self.final_lane_boundary_receipts(input, &final_boundary_receipts)?;
                let final_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &{
                            let mut complete = final_covered_lower_boundaries.clone();
                            for (destination, boundary) in &final_open_lower_boundaries {
                                complete.insert(destination.clone(), boundary.clone());
                            }
                            complete
                        },
                        false,
                        self.finalize_wb14_parent_interval,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
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
                let final_ending_stage3 = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_lower_boundaries,
                    Some(&final_lane_boundary_receipts),
                )?;
                if !covered_fixed_point_stage3_states_equal(
                    &final_stage3_candidate,
                    &final_ending_stage3,
                ) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "final Stage-3 lane receipt self-reconstruction",
                    ));
                }
                // The retained receipts must describe the candidate that is
                // actually installed, not the tolerance-equivalent precursor
                // used to discover the fixed point.  Re-seal from the replay
                // outputs, then prove that receipt metadata cannot perturb any
                // physical result.
                let installed_v8_digest = digest32_from_lower_hex(
                    &final_envelope.vegetation().ending_state().state_sha256,
                )?;
                let installed_stage3_digest = digest_bytes(&canonical_stage3_snow_owner_bytes_v11(
                    &final_ending_stage3,
                )?);
                let (installed_covered_lower_boundaries, installed_covered_boundary_receipts) =
                    self.seal_final_covered_boundaries(
                        input,
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
                let installed_lane_boundary_receipts =
                    self.final_lane_boundary_receipts(input, &installed_boundary_receipts)?;
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
                let installed_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &{
                            let mut complete = installed_covered_lower_boundaries.clone();
                            for (destination, boundary) in &installed_open_lower_boundaries {
                                complete.insert(destination.clone(), boundary.clone());
                            }
                            complete
                        },
                        false,
                        self.finalize_wb14_parent_interval,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
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
                let installed_stage3 = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_complete_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                )?;
                if installed_stage3 != final_ending_stage3 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed Stage-3 replay exact physical identity",
                    ));
                }
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
                ));
            }
            Err(DirectV11RealConsumerError::CoveredBoundary(
                SnowStage3HandoffError::FixedPointIterationLimit,
            ))
        }?;
        let ending_snow_owner_bytes = canonical_stage3_snow_owner_bytes_v11_with_receipts(
            &ending_stage3,
            &final_lane_boundary_receipts,
            &final_boundary_receipts,
        )?;
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &candidate,
            input,
            &envelope,
            ending_snow_owner_bytes,
            self.day_index,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(final_boundary_receipts);
        self.last_lane_boundary_receipts = Some(final_lane_boundary_receipts);
        self.last_component_carrier_receipts = Some(final_component_carrier_receipts);
        self.ending_stage3_by_lane = Some(ending_stage3);
        self.ending = Some(candidate);
        Ok(output)
    }
}

#[cfg(test)]
mod component_carrier_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    fn make_boundary(optical: u8) -> FinalStage3CanopyBoundaryReceiptV1 {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_000_000_000))
            .expect("support");
        FinalStage3CanopyBoundaryReceiptV1::try_new(FinalStage3CanopyBoundaryReceiptInputs {
            support,
            destination: (
                OfeId::try_new("ofe-1").expect("OFE"),
                TileId::try_new("forest").expect("tile"),
            ),
            beginning_v11_state_sha256: Digest32::from_bytes([1; 32]),
            beginning_stage3_state_sha256: Digest32::from_bytes([2; 32]),
            ending_v8_physical_candidate_sha256: Digest32::from_bytes([3; 32]),
            ending_stage3_state_sha256: Digest32::from_bytes([4; 32]),
            provisional_carrier_receipt_sha256: Digest32::from_bytes([5; 32]),
            optical_receipt_sha256: Digest32::from_bytes([optical; 32]),
            reciprocal_longwave_receipt_sha256: Digest32::from_bytes([7; 32]),
            sensible_to_canopy_air_w_m2: 2.0,
            vapor_to_canopy_air_kg_m2_s: 1.0e-6,
            latent_energy_to_canopy_air_j_m2: 2.5,
            snow_temperature_k: 270.0,
            latent_heat_j_kg: 2_500_000.0,
            snow_absorbed_shortwave_w_m2: 10.0,
            snow_net_longwave_w_m2: -5.0,
        })
        .expect("boundary")
    }

    fn state() -> CoveredLseIterationState {
        CoveredLseIterationState {
            canopy_air_temperature_k: 290.0,
            canopy_air_specific_humidity_kg_kg: 0.01,
            snow_temperature_k: 270.0,
            snow_sensible_w_m2: 2.0,
            snow_vapor_kg_m2_s: 1.0e-6,
            snow_latent_w_m2: 2.5,
            snow_net_longwave_w_m2: -5.0,
            component_temperatures_k: vec![("canopy".into(), [292.0; 4])],
            component_carrier_surfaces: (0_u8..4)
                .map(|component_ordinal| CoveredCarrierComponentState {
                    vertical_occupancy_ordinal: 0,
                    occupancy_id: "canopy".into(),
                    component_ordinal,
                    surface_area_m2_m2_tile: 0.25,
                    emissive_area_m2_m2_tile: 0.25,
                    heat_conductance_m_s_tile: 0.25,
                    vapor_conductance_m_s_tile: if component_ordinal == 3 { 0.0 } else { 0.25 },
                    vapor_authorization_kg_m2_tile_s: None,
                    temperature_k: 292.0,
                    specific_humidity_kg_kg: 0.011,
                    sensible_to_canopy_air_w_m2: 0.75,
                    vapor_to_canopy_air_kg_m2_s: if component_ordinal == 3 {
                        0.0
                    } else if component_ordinal == 2 {
                        1.0e-6
                    } else {
                        0.5e-6
                    },
                })
                .collect(),
            canopy_sensible_w_m2: 3.0,
            canopy_vapor_kg_m2_s: 2.0e-6,
            sensible_to_reference_air_w_m2: 5.0,
            vapor_to_reference_air_kg_m2_s: 3.0e-6,
        }
    }

    #[test]
    fn component_carrier_rejects_stale_inner_seal_and_fresh_boundary_substitution() {
        let boundary = make_boundary(6);
        let mut receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        receipt.components[0].temperature_k += 1.0;
        assert!(receipt.validate(&boundary).is_err());

        let alternate_boundary = make_boundary(8);
        let receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        assert!(receipt.validate(&alternate_boundary).is_err());
    }

    #[test]
    fn component_carrier_uses_vertical_order_not_lexical_occupancy_order() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let upper = physical
            .component_carrier_surfaces
            .iter()
            .cloned()
            .map(|mut component| {
                component.occupancy_id = "z-upper".into();
                component.surface_area_m2_m2_tile *= 0.5;
                component.emissive_area_m2_m2_tile *= 0.5;
                component.heat_conductance_m_s_tile *= 0.5;
                component.vapor_conductance_m_s_tile *= 0.5;
                component.sensible_to_canopy_air_w_m2 *= 0.5;
                component.vapor_to_canopy_air_kg_m2_s *= 0.5;
                component
            })
            .collect::<Vec<_>>();
        let lower = upper
            .iter()
            .cloned()
            .map(|mut component| {
                component.vertical_occupancy_ordinal = 1;
                component.occupancy_id = "a-lower".into();
                component
            })
            .collect::<Vec<_>>();
        physical.component_carrier_surfaces = upper.into_iter().chain(lower).collect();
        ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &physical,
            &boundary,
        )
        .expect("physical vertical order is authoritative");
    }

    #[test]
    fn component_carrier_rejects_duplicate_occupancy_across_vertical_ordinals() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let mut duplicate = physical.component_carrier_surfaces.clone();
        for component in &mut duplicate {
            component.vertical_occupancy_ordinal = 1;
        }
        physical.component_carrier_surfaces.extend(duplicate);
        physical.canopy_sensible_w_m2 *= 2.0;
        physical.canopy_vapor_kg_m2_s *= 2.0;
        physical.sensible_to_reference_air_w_m2 =
            physical.canopy_sensible_w_m2 + physical.snow_sensible_w_m2;
        physical.vapor_to_reference_air_kg_m2_s =
            physical.canopy_vapor_kg_m2_s + physical.snow_vapor_kg_m2_s;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &physical,
                &boundary,
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod covered_convergence_policy_tests {
    use super::*;
    use crate::DirectSnowLayerState;

    fn state() -> DirectSnowStage3PersistentState {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(
            7,
            vec![DirectSnowLayerState::new(0.1, 0.2, 500.0, 3.0)],
        )
        .expect("persistent state")
    }

    fn equal(
        left: DirectSnowStage3PersistentState,
        right: DirectSnowStage3PersistentState,
    ) -> bool {
        covered_fixed_point_stage3_states_equal(
            &BTreeMap::from([(7, left)]),
            &BTreeMap::from([(7, right)]),
        )
    }

    fn reseal(state: &mut DirectSnowStage3PersistentState) {
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
    }

    #[test]
    fn structural_fingerprint_and_count_fields_are_exact() {
        let original = state();
        let mut changed = original.clone();
        changed.fingerprint ^= 1;
        assert!(!equal(original.clone(), changed));
        let mut changed = original.clone();
        changed.layers[0].settle_day_count =
            f64::from_bits(changed.layers[0].settle_day_count.to_bits() + 1);
        reseal(&mut changed);
        assert!(!equal(original, changed));
    }

    #[test]
    fn unit_specific_state_tolerances_do_not_share_one_scale() {
        let original = state();
        let mut within = original.clone();
        within.layers[0].mass_swe_m += 0.5e-9;
        within.layers[0].temperature_c += 0.5e-8;
        within.layers[0].cold_content_j_m2 += 0.5e-6;
        reseal(&mut within);
        assert!(equal(original.clone(), within));
        let mut outside = original.clone();
        outside.layers[0].cold_content_j_m2 += 2.0e-6;
        reseal(&mut outside);
        assert!(!equal(original, outside));
    }

    #[test]
    fn density_is_exact_after_each_state_fingerprint_is_reconstructed() {
        let original = state();
        let mut changed = original.clone();
        changed.layers[0].density_kg_m3 =
            f64::from_bits(changed.layers[0].density_kg_m3.to_bits() + 1);
        reseal(&mut changed);
        assert!(!equal(original, changed));
    }

    #[test]
    fn cumulative_mass_uses_its_area_mass_tolerance() {
        let original = state();
        let mut within = original.clone();
        within.cumulative_snowfall_kg_m2 += 0.5e-6;
        reseal(&mut within);
        assert!(equal(original.clone(), within));

        let mut outside = original.clone();
        outside.cumulative_snowfall_kg_m2 += 2.0e-6;
        reseal(&mut outside);
        assert!(!equal(original, outside));
    }

    #[test]
    fn immutable_initial_mass_lineage_is_bitwise_exact() {
        let original = state();
        let mutations: [fn(&mut DirectSnowStage3PersistentState); 2] = [
            |state: &mut DirectSnowStage3PersistentState| {
                state.initial_ice_kg_m2 = f64::from_bits(state.initial_ice_kg_m2.to_bits() + 1);
            },
            |state: &mut DirectSnowStage3PersistentState| {
                state.initial_retained_liquid_kg_m2 =
                    f64::from_bits(state.initial_retained_liquid_kg_m2.to_bits() + 1);
            },
        ];
        for mutate in mutations {
            let mut changed = original.clone();
            mutate(&mut changed);
            reseal(&mut changed);
            assert!(!equal(original.clone(), changed));
        }
    }
}
