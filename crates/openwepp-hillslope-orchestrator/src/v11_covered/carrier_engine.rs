// Shared value-only construction primitives for the covered carrier engine.

struct CoveredCarrierEnvelopeBuildV1<'a> {
    candidate: &'a DirectV10RealConsumerShadow,
    interval_s: f64,
    duration_s_bits: u64,
    covered_destinations: &'a BTreeSet<(OfeId, TileId)>,
    covered_boundaries: &'a BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    open_boundaries: &'a BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    provisional: bool,
    finalize_wb14_parent_interval: bool,
}

struct ProvisionalCoveredIterationEvidenceV1 {
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    corrected_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    transaction_id: TransactionId,
    soil_candidates: Vec<SoilThermalTileCandidate>,
    soil_energy_operands_v2: Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
}

fn complete_stage3_lower_boundaries_v1(
    expected_covered: &BTreeSet<(OfeId, TileId)>,
    covered: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    expected_open: &BTreeSet<(OfeId, TileId)>,
    open: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError> {
    if covered.keys().cloned().collect::<BTreeSet<_>>() != *expected_covered {
        return Err(DirectV11RealConsumerError::Identity(
            "covered Stage-3 lower-boundary topology",
        ));
    }
    if open.keys().cloned().collect::<BTreeSet<_>>() != *expected_open {
        return Err(DirectV11RealConsumerError::Identity(
            "open-snow Stage-3 lower-boundary topology",
        ));
    }
    if expected_covered
        .iter()
        .any(|destination| expected_open.contains(destination))
    {
        return Err(DirectV11RealConsumerError::Identity(
            "covered/open destination forcing intersection",
        ));
    }
    let mut complete = covered.clone();
    for (destination, boundary) in open {
        if complete
            .insert(destination.clone(), boundary.clone())
            .is_some()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered/open destination forcing intersection",
            ));
        }
    }
    Ok(complete)
}

#[derive(Clone)]
pub(crate) struct AcceptedCoveredCarrierEvidenceV1 {
    pub final_boundaries: BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    pub final_lanes: BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    pub component_receipts: BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
    pub wb14_child_receipt_set_sha256: String,
    pub wb14_parent_receipt_set_sha256: Option<String>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
}

#[cfg(test)]
fn accepted_carrier_parent_cadence_beginning_v1(
    discovery: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    installed: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<BTreeMap<u32, DirectSnowStage3PersistentState>, DirectV11RealConsumerError> {
    if discovery.keys().collect::<BTreeSet<_>>() != installed.keys().collect::<BTreeSet<_>>() {
        return Err(DirectV11RealConsumerError::Identity(
            "accepted carrier physical Stage-3 beginning topology",
        ));
    }
    discovery
        .iter()
        .map(|(lane_id, discovery)| {
            let installed = installed
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted carrier physical Stage-3 beginning lane",
                ))?;
            Wb11HydrologyKernel::validate_stage3_persistent_state(discovery).map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "accepted carrier physical Stage-3 discovery beginning",
                )
            })?;
            Wb11HydrologyKernel::validate_stage3_persistent_state(installed).map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "accepted carrier physical Stage-3 installed beginning",
                )
            })?;
            let mut rebound = discovery.clone();
            rebound.next_interval_index = installed.next_interval_index;
            rebound.fingerprint =
                Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&rebound);
            if rebound != *installed {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted carrier physical Stage-3 beginning beyond parent cadence",
                ));
            }
            Ok((*lane_id, rebound))
        })
        .collect()
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    /// Seal accepted carrier evidence solely from the winning exact carrier
    /// values. This function never evaluates Stage 3 or LSE and never writes
    /// publication fields on the owning stack.
    pub(crate) fn seal_accepted_carrier_evidence_v1(
        &self,
        phase: &CoveredCarrierPhaseResultV1,
        input: &V11ImportedV10SegmentInput,
        ending_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<AcceptedCoveredCarrierEvidenceV1, DirectV11RealConsumerError> {
        let physical_support = phase.transition.boundary.support;
        if physical_support.start_ns() < input.support.start_ns()
            || physical_support.end_ns() != input.support.end_ns()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted carrier final-child/envelope support identity",
            ));
        }
        let ending_v8_sha256 = digest32_from_lower_hex(
            &phase
                .carrier_envelope
                .vegetation()
                .ending_state()
                .state_sha256,
        )?;
        let ending_snow_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(ending_stage3)?);
        if phase.beginning_candidates.stage3_by_lane() != &phase.beginning_stage3_by_lane
            || phase.beginning_candidates.joint() != &phase.transition.beginning_joint
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted carrier physical-child beginning identity",
            ));
        }
        let beginning_v11_state_sha256 = digest32_from_lower_hex(
            &phase
                .beginning_candidates
                .shadow()
                .vegetation_state
                .0
                .state_sha256,
        )?;
        // The carrier phase retains the complete covered/open lower-boundary
        // map so its physical candidate is replayable.  Only destinations
        // with a covered-carrier source receipt may enter covered evidence;
        // open-snow evidence is sealed independently below.
        let covered_lower_boundaries = phase
            .complete_lower_boundaries
            .iter()
            .filter(|(destination, _)| phase.carrier_source_receipts.contains_key(*destination))
            .map(|(destination, boundary)| (destination.clone(), boundary.clone()))
            .collect::<BTreeMap<_, _>>();
        if covered_lower_boundaries.len() != phase.carrier_source_receipts.len() {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted covered carrier boundary/source topology",
            ));
        }
        let (_covered_boundaries, covered_receipts) = self.seal_final_covered_boundaries(
            physical_support,
            beginning_v11_state_sha256,
            &phase.beginning_stage3_by_lane,
            &covered_lower_boundaries,
            &phase.carrier_source_receipts,
            &phase.carrier_envelope,
            ending_v8_sha256,
            ending_snow_sha256,
        )?;
        let mut open_boundaries = phase
            .complete_lower_boundaries
            .iter()
            .filter(|(destination, _)| phase.open_snow_candidates.contains_key(*destination))
            .map(|(destination, boundary)| (destination.clone(), boundary.clone()))
            .collect::<BTreeMap<_, _>>();
        let open_receipts = phase
            .open_snow_candidates
            .iter()
            .map(|(destination, candidate)| {
                let mut candidate = candidate.clone();
                let consumed_boundary = open_boundaries.get(destination).ok_or(
                    DirectV11RealConsumerError::Identity(
                        "accepted open-snow carrier boundary topology",
                    ),
                )?;
                candidate.snow_temperature_k = consumed_boundary.snow_temperature_k;
                candidate.latent_heat_j_kg = consumed_boundary.latent_heat_j_kg;
                candidate.sensible_outward_w_m2 = consumed_boundary.sensible_to_canopy_air_w_m2;
                candidate.vapor_outward_kg_m2_s = consumed_boundary.vapor_to_canopy_air_kg_m2_s;
                candidate.latent_energy_outward_j_m2 = candidate.vapor_outward_kg_m2_s
                    * candidate.latent_heat_j_kg
                    * f64::from_bits(candidate.support.duration_s_bits());
                candidate.snow_absorbed_shortwave_w_m2 = consumed_boundary.shortwave_absorbed_w_m2;
                candidate.snow_net_longwave_w_m2 = consumed_boundary.net_longwave_w_m2;
                let receipt =
                    FinalStage3OpenSnowBoundaryReceiptV1::try_new(candidate, ending_snow_sha256)?;
                let boundary = open_boundaries.get_mut(destination).ok_or(
                    DirectV11RealConsumerError::Identity(
                        "accepted open-snow carrier boundary topology",
                    ),
                )?;
                boundary.final_canopy_boundary_receipt_sha256 = Some(
                    Sha256Digest::try_new(digest32_hex(receipt.receipt_sha256)).map_err(|_| {
                        DirectV11RealConsumerError::Identity("accepted open-snow boundary receipt")
                    })?,
                );
                boundary.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("accepted sealed open-snow boundary")
                })?;
                Ok((destination.clone(), receipt))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        let final_boundaries =
            self.complete_final_boundary_receipts(covered_receipts, open_receipts)?;
        let final_lanes = self.final_lane_boundary_receipts(
            physical_support,
            &final_boundaries,
            &phase.precipitation_sets,
        )?;
        let component_receipts = phase
            .covered_lse_states
            .iter()
            .map(|(destination, state)| {
                let boundary = final_boundaries
                    .get(destination)
                    .and_then(|value| match value {
                        FinalStage3TileBoundaryReceiptV1::V11Canopy(value) => Some(value),
                        FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => None,
                    })
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "accepted component carrier destination",
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
        Ok(AcceptedCoveredCarrierEvidenceV1 {
            final_boundaries,
            final_lanes,
            component_receipts,
            wb14_child_receipt_set_sha256: phase.wb14_child_receipt_set_sha256.clone(),
            wb14_parent_receipt_set_sha256: phase.wb14_parent_receipt_set_sha256.clone(),
            wb14_child_replay_bytes: phase.wb14_child_replay_bytes.clone(),
            wb14_parent_replay_bytes: phase.wb14_parent_replay_bytes.clone(),
        })
    }

    /// Build one unpublished carrier envelope from a complete, disjoint
    /// covered/open destination set. The returned value is not adopted.
    fn build_covered_carrier_envelope_value_with_soil_beginning_v1(
        &self,
        inputs: CoveredCarrierEnvelopeBuildV1<'_>,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV11RealConsumerError> {
        let expected_open = self
            .snow_surface_forcing_by_destination
            .iter()
            .filter_map(|(destination, forcing)| {
                forcing.is_open_snow().then_some(destination.clone())
            })
            .collect::<BTreeSet<_>>();
        let mut complete = complete_stage3_lower_boundaries_v1(
            inputs.covered_destinations,
            inputs.covered_boundaries,
            &expected_open,
            inputs.open_boundaries,
        )?;
        if inputs.provisional {
            for boundary in complete.values_mut() {
                boundary.optical_receipt_sha256 = None;
                boundary.reciprocal_longwave_receipt_sha256 = None;
                boundary.final_canopy_boundary_receipt_sha256 = None;
            }
        }
        let envelope = inputs
            .candidate
            .inner
            .construct_covered_interval_envelope_with_duration_and_soil_beginning(
                self.day_index,
                self.interval_index,
                self.interval,
                inputs.interval_s,
                inputs.duration_s_bits,
                inputs.covered_destinations,
                &complete,
                inputs.provisional,
                inputs.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
                unpublished_soil_candidate,
                unpublished_soil_continuation,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        audit_covered_carrier_condensation_credits(envelope.hydrology().condensation_credits());
        Ok(envelope)
    }

    /// Evaluate only the physical covered endpoint needed by one fixed-point
    /// iterate. This posture cannot construct or expose vegetation/BGC owner
    /// candidates, V8 projection receipts, or accepted publication state.
    fn build_covered_carrier_physical_value_with_soil_beginning_v1(
        &self,
        prepared: &PreparedCoveredCanopySoilInputV1,
        inputs: CoveredCarrierEnvelopeBuildV1<'_>,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<
        crate::land_surface_energy_shadow::ProvisionalCoveredV8PhysicalEvaluationV1,
        DirectV11RealConsumerError,
    > {
        if !inputs.provisional {
            return Err(DirectV11RealConsumerError::Identity(
                "physical-only covered endpoint posture",
            ));
        }
        let expected_open = self
            .snow_surface_forcing_by_destination
            .iter()
            .filter_map(|(destination, forcing)| {
                forcing.is_open_snow().then_some(destination.clone())
            })
            .collect::<BTreeSet<_>>();
        let mut complete = complete_stage3_lower_boundaries_v1(
            inputs.covered_destinations,
            inputs.covered_boundaries,
            &expected_open,
            inputs.open_boundaries,
        )?;
        for boundary in complete.values_mut() {
            boundary.optical_receipt_sha256 = None;
            boundary.reciprocal_longwave_receipt_sha256 = None;
            boundary.final_canopy_boundary_receipt_sha256 = None;
        }
        let physical = inputs
            .candidate
            .inner
            .construct_prepared_covered_interval_physical_with_duration_and_soil_beginning(
                self.day_index,
                self.interval_index,
                prepared,
                inputs.interval_s,
                inputs.duration_s_bits,
                inputs.covered_destinations,
                &complete,
                inputs.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
                unpublished_soil_candidate,
                unpublished_soil_continuation,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        audit_covered_carrier_condensation_credits(physical.hydrology().condensation_credits());
        Ok(physical)
    }

    fn build_provisional_covered_iteration_evidence_v1(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        current_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        prepared: &PreparedCoveredCanopySoilInputV1,
        inputs: CoveredCarrierEnvelopeBuildV1<'_>,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<
            &DirectSoilThermalUnpublishedContinuationResultV2,
        >,
    ) -> Result<ProvisionalCoveredIterationEvidenceV1, DirectV11RealConsumerError> {
        #[cfg(test)]
        let force_full = covered_full_provisional_envelope_forced_for_test();
        #[cfg(not(test))]
        let force_full = false;
        let evidence = if force_full {
            let envelope = self.build_covered_carrier_envelope_value_with_soil_beginning_v1(
                inputs,
                unpublished_soil_candidate,
                unpublished_soil_continuation,
            )?;
            let precipitation_sets = self.precipitation_parcel_sets(support, &envelope)?;
            let (corrected_boundaries, _, _) =
                self.corrected_covered_boundaries_from_envelope(current_boundaries, &envelope)?;
            let lse_states = envelope
                .covered_lse_iteration_state_by_destination()
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered provisional LSE iteration state")
                })?;
            ProvisionalCoveredIterationEvidenceV1 {
                precipitation_sets,
                corrected_boundaries,
                lse_states,
                transaction_id: envelope.transaction_id(),
                soil_candidates: envelope.hydrology().soil_thermal_candidates().to_vec(),
                soil_energy_operands_v2:
                    crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
                        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                            envelope.transaction_id(),
                            support.start_ns().get(),
                            support.end_ns().get(),
                            envelope.hydrology().pre_ingress_soil_thermal_candidates(),
                        )
                        .map_err(|error| {
                            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                                DirectV9RealConsumerError::LandSurfaceShadow(error),
                            ))
                        })?,
                        support.start_ns().get(),
                        support.end_ns().get(),
                        &self.beginning.inner.lse_configuration.owner_id,
                        &self.beginning.inner.surface_configuration.owner_id,
                        envelope.hydrology().pre_ingress_soil_thermal_candidates(),
                        envelope.hydrology().surface_ingress(),
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            DirectV9RealConsumerError::LandSurfaceShadow(error),
                        ))
                    })?,
            }
        } else {
            let physical = self.build_covered_carrier_physical_value_with_soil_beginning_v1(
                prepared,
                inputs,
                unpublished_soil_candidate,
                unpublished_soil_continuation,
            )?;
            let precipitation_sets =
                self.precipitation_parcel_sets_from_physical(support, &physical)?;
            let (corrected_boundaries, _, _) =
                self.corrected_covered_boundaries_from_physical(current_boundaries, &physical)?;
            let lse_states = physical
                .covered_lse_iteration_state_by_destination()
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered provisional LSE iteration state")
                })?;
            ProvisionalCoveredIterationEvidenceV1 {
                precipitation_sets,
                corrected_boundaries,
                lse_states,
                transaction_id: physical.transaction_id(),
                soil_candidates: physical.hydrology().soil_thermal_candidates().to_vec(),
                soil_energy_operands_v2:
                    crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
                        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                            physical.transaction_id(),
                            support.start_ns().get(),
                            support.end_ns().get(),
                            physical.hydrology().pre_ingress_soil_thermal_candidates(),
                        )
                        .map_err(|error| {
                            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                                DirectV9RealConsumerError::LandSurfaceShadow(error),
                            ))
                        })?,
                        support.start_ns().get(),
                        support.end_ns().get(),
                        &self.beginning.inner.lse_configuration.owner_id,
                        &self.beginning.inner.surface_configuration.owner_id,
                        physical.hydrology().pre_ingress_soil_thermal_candidates(),
                        physical.hydrology().surface_ingress(),
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            DirectV9RealConsumerError::LandSurfaceShadow(error),
                        ))
                    })?,
            }
        };
        #[cfg(test)]
        record_covered_provisional_physical_audit_v1(CoveredProvisionalPhysicalAuditV1 {
            precipitation_sets: evidence.precipitation_sets.clone(),
            corrected_boundaries: evidence.corrected_boundaries.clone(),
            lse_states: evidence.lse_states.clone(),
            transaction_id: evidence.transaction_id,
            soil_candidates: evidence.soil_candidates.clone(),
        });
        Ok(evidence)
    }

    /// Rebuild the component-resolved LSE carrier from one immutable envelope.
    fn rebuild_covered_lse_carrier_value_v1(
        &self,
        source: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
        ),
        DirectV11RealConsumerError,
    > {
        let (corrected, _, _) =
            self.corrected_covered_boundaries_from_envelope(source, envelope)?;
        let lse = envelope
            .covered_lse_iteration_state_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier LSE state"))?;
        let rebuilt = self.apply_lse_iteration_exchange(&corrected, &lse)?;
        let mut rebuilt = self.merge_latest_stage3_state_operands(&rebuilt, stage3_states)?;
        snow_boundary_state.apply_to_boundaries(
            &self.beginning.inner.surface_configuration.ofe_bindings,
            &mut rebuilt,
        )?;
        Ok((rebuilt, lse))
    }
}

#[cfg(test)]
mod accepted_carrier_evidence_tests {
    use super::*;
    use crate::DirectSnowLayerState;

    fn stage3_state(lane_id: u32) -> DirectSnowStage3PersistentState {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(
            lane_id,
            vec![DirectSnowLayerState::new(0.08, 0.8, 100.0, 12.0)],
        )
        .expect("Stage-3 state")
    }

    fn destination(tile: &str) -> (OfeId, TileId) {
        (
            OfeId::try_new("ofe-1").expect("OFE"),
            TileId::try_new(tile).expect("tile"),
        )
    }

    fn boundary(byte: u8) -> Stage3SnowCoveredLowerBoundary {
        let digest = Sha256Digest::try_new(format!("{byte:02x}").repeat(32)).expect("digest");
        Stage3SnowCoveredLowerBoundary {
            snow_temperature_k: 273.15,
            latent_heat_j_kg: 2_834_000.0,
            sensible_to_canopy_air_w_m2: 0.0,
            vapor_to_canopy_air_kg_m2_s: 0.0,
            net_longwave_w_m2: 0.0,
            shortwave_absorbed_w_m2: 0.0,
            precipitation_advection_w_m2: 0.0,
            carrier_receipt_id: digest.clone(),
            snow_vis_albedo: 0.8,
            snow_nir_albedo: 0.8,
            stage3_albedo_state_sha256: digest.clone(),
            forcing_receipt_sha256: digest,
            optical_receipt_sha256: None,
            reciprocal_longwave_receipt_sha256: None,
            final_canopy_boundary_receipt_sha256: None,
        }
    }

    #[test]
    fn mixed_open_and_covered_join_retains_open_marker_and_rejects_topology_poison() {
        let covered_destination = destination("covered");
        let open_destination = destination("open");
        let expected_covered = BTreeSet::from([covered_destination.clone()]);
        let expected_open = BTreeSet::from([open_destination.clone()]);
        let covered = BTreeMap::from([(covered_destination.clone(), boundary(1))]);
        let open = BTreeMap::from([(open_destination.clone(), boundary(2))]);

        let complete =
            complete_stage3_lower_boundaries_v1(&expected_covered, &covered, &expected_open, &open)
                .expect("complete mixed Stage-3 boundary map");
        assert_eq!(expected_covered, BTreeSet::from([covered_destination]));
        assert!(complete.contains_key(&open_destination));
        assert_eq!(complete.len(), 2);

        assert!(complete_stage3_lower_boundaries_v1(
            &expected_covered,
            &covered,
            &expected_open,
            &BTreeMap::new(),
        )
        .is_err());
        let substituted = BTreeMap::from([(destination("substituted"), boundary(3))]);
        assert!(complete_stage3_lower_boundaries_v1(
            &expected_covered,
            &covered,
            &expected_open,
            &substituted,
        )
        .is_err());
    }

    #[test]
    fn only_finite_cold_covered_boundary_is_refinable() {
        let mut cold = boundary(4);
        cold.snow_temperature_k = 199.0;
        assert!(covered_boundary_only_cold_temperature_is_refinable_v1(
            &cold
        ));

        let mut admitted = cold.clone();
        admitted.snow_temperature_k = 200.0;
        assert!(!covered_boundary_only_cold_temperature_is_refinable_v1(
            &admitted
        ));

        let mut structural_poison = cold;
        structural_poison.latent_heat_j_kg = f64::NAN;
        assert!(!covered_boundary_only_cold_temperature_is_refinable_v1(
            &structural_poison
        ));
    }

    #[test]
    fn accepted_carrier_beginning_rebinds_only_parent_cadence_cursor() {
        let discovery = BTreeMap::from([(1, stage3_state(1)), (2, stage3_state(2))]);
        let mut installed = discovery.clone();
        for state in installed.values_mut() {
            state.next_interval_index = 9;
            state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
        }
        let rebound = accepted_carrier_parent_cadence_beginning_v1(&discovery, &installed)
            .expect("cadence-only replay difference");
        assert_eq!(rebound, installed);
    }

    #[test]
    fn accepted_carrier_beginning_rejects_physical_and_topology_substitution() {
        let discovery = BTreeMap::from([(1, stage3_state(1))]);
        let mut physical_substitution = discovery.clone();
        let state = physical_substitution.get_mut(&1).expect("lane");
        state.layers[0].cold_content_j_m2 += 1.0e-12;
        state.layers[0].temperature_c =
            Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                state.layers[0].mass_swe_m,
                state.layers[0].cold_content_j_m2,
            );
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
        assert!(
            accepted_carrier_parent_cadence_beginning_v1(&discovery, &physical_substitution,)
                .is_err()
        );
        assert!(accepted_carrier_parent_cadence_beginning_v1(
            &discovery,
            &BTreeMap::from([(2, stage3_state(2))]),
        )
        .is_err());
    }

    #[test]
    fn accepted_sealer_is_value_only_and_does_not_publish() {
        let source = include_str!("carrier_engine.rs");
        let body = source
            .split("pub(crate) fn seal_accepted_carrier_evidence_v1")
            .nth(1)
            .expect("accepted sealer")
            .split("fn build_covered_carrier_envelope_value_with_soil_beginning_v1")
            .next()
            .expect("accepted sealer body");
        for forbidden in [
            "evaluate_stage3_persistent_support",
            "evaluate_stage3_terminal_support",
            "construct_covered_interval_envelope_with_duration(\n                self.day_index",
            "last_final_boundary_receipts =",
            "last_wb14_",
            "self.ending =",
            "candidate.beginning_stage3_state_sha256 =",
        ] {
            assert!(
                !body.contains(forbidden),
                "accepted sealer used {forbidden}"
            );
        }
    }

    #[test]
    fn provisional_envelope_clears_all_accepted_boundary_identities() {
        let source = include_str!("carrier_engine.rs");
        assert!(source.contains("boundary.optical_receipt_sha256 = None"));
        assert!(source.contains("boundary.reciprocal_longwave_receipt_sha256 = None"));
        assert!(source.contains("boundary.final_canopy_boundary_receipt_sha256 = None"));
    }
}
