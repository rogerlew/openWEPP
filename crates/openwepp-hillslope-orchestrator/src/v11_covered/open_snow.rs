const OUTCOME_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

include!("open_snow_receipt_reseal_helpers.rs");
include!("terminal_composition.rs");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CoveredOrdinaryPhysicalAuthorityV1 {
    physical_input_sha256: Digest32,
    beginning_authority_sha256: Digest32,
    accepted_authorization_sha256: Digest32,
}

fn covered_ordinary_physical_authority_v1(
    input: &V11ImportedV10SegmentInput,
) -> Result<CoveredOrdinaryPhysicalAuthorityV1, DirectV11RealConsumerError> {
    fn frame<T: Serialize>(value: &T) -> Result<Digest32, DirectV11RealConsumerError> {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            DirectV11RealConsumerError::Identity("covered ordinary physical authority framing")
        })?;
        Ok(digest_bytes(&bytes))
    }
    Ok(CoveredOrdinaryPhysicalAuthorityV1 {
        physical_input_sha256: frame(&(
            input.parent_transaction_id,
            input.support,
            input.duration_s_bits,
            &input.configuration,
        ))?,
        beginning_authority_sha256: frame(&(&input.beginning, &input.staged_resource_owners))?,
        accepted_authorization_sha256: frame(&(
            input.accepted_slab_receipt.parent_transaction_id(),
            input.accepted_slab_receipt.segment_id(),
            input.accepted_slab_receipt.slab_ordinal(),
            input.accepted_slab_receipt.support(),
            input.accepted_slab_receipt.duration_s_bits(),
        ))?,
    })
}

fn validate_covered_ordinary_physical_reuse_gate_v1(
    seed: CoveredOrdinaryPhysicalAuthorityV1,
    candidate: CoveredOrdinaryPhysicalAuthorityV1,
    terminal_endpoint_mode: bool,
) -> Result<(), DirectV11RealConsumerError> {
    if terminal_endpoint_mode || seed != candidate {
        return Err(DirectV11RealConsumerError::Identity(
            "covered ordinary physical reuse identity",
        ));
    }
    Ok(())
}

fn validate_covered_terminal_reuse_trial_binding_v1(
    observed: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    accepted: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    trial_support: TimeSupport,
    replay_trial_sha256: Digest32,
    replay_beginning_owner_set_sha256: Digest32,
) -> Result<(), DirectV11RealConsumerError> {
    let expected = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        accepted_slab_sha256: *replay_trial_sha256.as_bytes(),
        parent_beginning_complete_owner_set_sha256: *replay_beginning_owner_set_sha256.as_bytes(),
        child_support_start_ns: trial_support.start_ns().get(),
        child_support_end_ns: trial_support.end_ns().get(),
        ..accepted
    };
    if observed != expected {
        return Err(DirectV11RealConsumerError::Identity(
            "covered terminal reuse WB14 trial authorization",
        ));
    }
    Ok(())
}

#[cfg(test)]
include!("open_snow_reuse_gate_tests.rs");

fn reconstruct_stage3_phase_forcing_v1(
    liquid_mass_kg_m2: f64,
    solid_mass_kg_m2: f64,
) -> Result<(f64, f64, f64), DirectV11RealConsumerError> {
    if !liquid_mass_kg_m2.is_finite()
        || !solid_mass_kg_m2.is_finite()
        || liquid_mass_kg_m2 < 0.0
        || solid_mass_kg_m2 < 0.0
    {
        return Err(DirectV11RealConsumerError::Identity(
            "precipitation phase/mass same-set join",
        ));
    }
    let rain_m = liquid_mass_kg_m2 / 1_000.0;
    let snowfall_m = solid_mass_kg_m2 / 100.0;
    let active_precipitation_m = (liquid_mass_kg_m2 + solid_mass_kg_m2) / 1_000.0;
    if (active_precipitation_m - (rain_m + snowfall_m * 0.1)).abs() > 1.0e-12 {
        return Err(DirectV11RealConsumerError::Identity(
            "precipitation phase active-volume reconstruction",
        ));
    }
    Ok((rain_m, snowfall_m, active_precipitation_m))
}

#[cfg(test)]
mod stage3_phase_forcing_reconstruction_tests {
    use super::*;

    #[test]
    fn solid_geometric_depth_reconstructs_water_equivalent_and_rejects_raw_depth_alias() {
        let liquid_mass = 2.854_810_069_486_077;
        let solid_mass = 7.145_189_930_513_923;
        let (rain_m, snowfall_m, active_m) =
            reconstruct_stage3_phase_forcing_v1(liquid_mass, solid_mass)
                .expect("phase reconstruction");
        assert_eq!(rain_m.to_bits(), (liquid_mass / 1_000.0).to_bits());
        assert_eq!(snowfall_m.to_bits(), (solid_mass / 100.0).to_bits());
        assert_eq!(
            active_m.to_bits(),
            ((liquid_mass + solid_mass) / 1_000.0).to_bits(),
        );
        assert!((active_m - 0.01).abs() <= f64::EPSILON);
        assert_ne!(active_m.to_bits(), (rain_m + snowfall_m).to_bits());
        assert!(reconstruct_stage3_phase_forcing_v1(-f64::MIN_POSITIVE, solid_mass).is_err());
    }
}

#[derive(Clone, Copy, Debug)]
struct TerminalEndpointExternalLedgerV1 {
    energy_j_m2: [f64; 6],
    vapor_kg_m2: f64,
}

impl TerminalEndpointExternalLedgerV1 {
    fn ordered_add(self, ending: Self) -> Self {
        Self {
            energy_j_m2: std::array::from_fn(|index| {
                self.energy_j_m2[index] + ending.energy_j_m2[index]
            }),
            vapor_kg_m2: self.vapor_kg_m2 + ending.vapor_kg_m2,
        }
    }

    fn matches(self, other: Self) -> bool {
        self.energy_j_m2
            .iter()
            .zip(other.energy_j_m2)
            .all(|(actual, expected)| (actual - expected).abs() <= 1.0e-6)
            && (self.vapor_kg_m2 - other.vapor_kg_m2).abs() <= 1.0e-9
    }
}

fn accepted_terminal_endpoint_timing_v1(
    entry_seconds: f64,
    terminal_seconds: f64,
    unevaluated_seconds: f64,
    hour_offset_seconds: f64,
    complete_evaluated_seconds: f64,
    support_seconds: f64,
) -> bool {
    unevaluated_seconds.abs() <= 1.0e-6
        && (entry_seconds + terminal_seconds - support_seconds).abs() <= 1.0e-6
        && (hour_offset_seconds - support_seconds).abs() <= 1.0e-6
        && (complete_evaluated_seconds - support_seconds).abs() <= 1.0e-6
}

fn validate_accepted_terminal_endpoint_composition_v1(
    evaluation: &crate::hydrology::DirectSnowStage3EvaluationDiagnostics,
    reconciliation: &crate::hydrology::DirectSnowStage3OperatorReconciliation,
    event: &DirectSnowTerminalEventResult,
    support_seconds: f64,
) -> Result<bool, DirectV11RealConsumerError> {
    if !event.event_occurred {
        return Ok(false);
    }
    if !accepted_terminal_endpoint_timing_v1(
        event.terminal_entry_offset_seconds,
        event.evaluated_seconds,
        event.unevaluated_seconds,
        event.hour_offset_seconds,
        evaluation.evaluated_seconds,
        support_seconds,
    ) {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix/terminal chronology",
        ));
    }
    let mut prefix_seconds = 0.0;
    let mut prefix = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [0.0; 6],
        vapor_kg_m2: 0.0,
    };
    for tuple in reconciliation.tuples.iter().filter(|tuple| {
        tuple.applicable
            && tuple.elapsed_start_seconds + tuple.duration_seconds
                <= event.terminal_entry_offset_seconds + 1.0e-9
    }) {
        prefix_seconds += tuple.duration_seconds;
        prefix = prefix.ordered_add(TerminalEndpointExternalLedgerV1 {
            energy_j_m2: [
                tuple.sensible_flux_w_m2 * tuple.duration_seconds,
                tuple.net_shortwave_w_m2 * tuple.duration_seconds,
                tuple.latent_flux_w_m2 * tuple.duration_seconds,
                tuple.net_longwave_w_m2 * tuple.duration_seconds,
                tuple.precipitation_advected_flux_w_m2 * tuple.duration_seconds,
                tuple.snow_soil_heat_flux_w_m2 * tuple.duration_seconds,
            ],
            vapor_kg_m2: tuple.vapor_mass_exchange_kg_m2,
        });
    }
    if (prefix_seconds - event.terminal_entry_offset_seconds).abs() > 1.0e-6 {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix chronology",
        ));
    }
    let terminal = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [
            event.sensible_energy_j_m2,
            event.shortwave_energy_j_m2,
            event.latent_energy_j_m2,
            event.longwave_energy_j_m2,
            event.advected_energy_j_m2,
            event.snow_soil_heat_energy_j_m2,
        ],
        vapor_kg_m2: event.deposition_kg_m2 - event.sublimation_kg_m2,
    };
    let complete = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [
            evaluation.complete_arm_sensible_j_m2,
            evaluation.complete_arm_shortwave_j_m2,
            evaluation.complete_arm_latent_j_m2,
            evaluation.complete_arm_longwave_j_m2,
            evaluation.complete_arm_advected_j_m2,
            evaluation.complete_arm_snow_soil_heat_j_m2,
        ],
        vapor_kg_m2: evaluation.complete_arm_vapor_mass_exchange_kg_m2,
    };
    if !prefix.ordered_add(terminal).matches(complete) {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix/terminal ledger composition",
        ));
    }
    Ok(true)
}

include!("open_snow_physical_reconstruction_helpers.rs");

struct SnowSoilReceiptInputs<'a> {
    support: openwepp_coupled_time::TimeSupport,
    duration_s: f64,
    lane_id: u32,
    ofe_id: &'a OfeId,
    topology_identity_sha256: Digest32,
    configuration_identity_sha256: Digest32,
    beginning_soil_owner_identity_sha256: Digest32,
    beginning_stage: &'a DirectSnowStage3PersistentState,
    ending_stage: &'a DirectSnowStage3PersistentState,
    ending_ofe: &'a openwepp_land_surface_energy::SoilThermalOfeSnapshot,
    configured_top: &'a openwepp_land_surface_energy::SoilInterfaceLayer,
    beginning_top: &'a openwepp_land_surface_energy::SoilThermalLayerSnapshot,
    ending_top: &'a openwepp_land_surface_energy::SoilThermalLayerSnapshot,
    beginning_bottom_temperature_k: f64,
    beginning_bottom_thickness_m: f64,
    beginning_bottom_conductivity_w_m_k: f64,
    beginning_snow_owner_sha256: Digest32,
    ending_bottom_temperature_k: f64,
}

fn seal_snow_soil_heat_receipt(
    inputs: &SnowSoilReceiptInputs<'_>,
) -> Result<SnowSoilHeatReceiptV1, DirectV11RealConsumerError> {
    let (beginning_heat, ending_heat, accepted_heat) =
        crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
            0.5 * inputs.beginning_bottom_thickness_m,
            inputs.beginning_bottom_conductivity_w_m_k,
            0.5 * inputs.configured_top.thickness_m,
            inputs.configured_top.thermal_conductivity_w_m_k,
            inputs.beginning_bottom_temperature_k,
            inputs.beginning_top.temperature_k,
            inputs.ending_bottom_temperature_k,
            inputs.ending_top.temperature_k,
        )
        .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
    let accepted_j = accepted_heat * inputs.duration_s;
    let ending_snow_sha256 = digest_bytes(
        &serde_json::to_vec(inputs.ending_stage)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow-soil trial snow seal"))?,
    );
    let ending_soil_sha256 = digest_bytes(
        &serde_json::to_vec(inputs.ending_ofe)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow-soil trial soil seal"))?,
    );
    SnowSoilHeatReceiptV1 {
        schema_version: 1,
        model_identity_sha256: digest_bytes(b"SC-SNOWENERGY-001@18-SNOW-SOIL-CN-V1"),
        support: inputs.support,
        support_duration_ns: inputs.support.duration_ns(),
        lane_id: inputs.lane_id,
        ofe_id: inputs.ofe_id.clone(),
        ofe_ground_basis: true,
        topology_identity_sha256: inputs.topology_identity_sha256,
        configuration_identity_sha256: inputs.configuration_identity_sha256,
        beginning_snow_owner_identity_sha256: inputs.beginning_snow_owner_sha256,
        beginning_soil_owner_identity_sha256: inputs.beginning_soil_owner_identity_sha256,
        bottom_snow_layer_id: u32::try_from(inputs.beginning_stage.layers.len().saturating_sub(1))
            .map_err(|_| DirectV11RealConsumerError::Identity("snow bottom layer ordinal"))?,
        first_soil_layer_id: inputs.configured_top.layer_id.clone(),
        bottom_snow_half_thickness_m: 0.5 * inputs.beginning_bottom_thickness_m,
        bottom_snow_conductivity_w_m_k: inputs.beginning_bottom_conductivity_w_m_k,
        top_soil_half_thickness_m: 0.5 * inputs.configured_top.thickness_m,
        top_soil_conductivity_w_m_k: inputs.configured_top.thermal_conductivity_w_m_k,
        beginning_bottom_snow_temperature_k: inputs.beginning_bottom_temperature_k,
        beginning_top_soil_temperature_k: inputs.beginning_top.temperature_k,
        ending_bottom_snow_temperature_k: inputs.ending_bottom_temperature_k,
        ending_top_soil_temperature_k: inputs.ending_top.temperature_k,
        beginning_heat_flux_w_m2_ofe_ground: beginning_heat,
        ending_heat_flux_w_m2_ofe_ground: ending_heat,
        accepted_heat_flux_w_m2_ofe_ground: accepted_heat,
        accepted_heat_j_m2_ofe_ground: accepted_j,
        snow_candidate_heat_j_m2_ofe_ground: -accepted_j,
        soil_candidate_heat_j_m2_ofe_ground: accepted_j,
        snow_candidate_ending_identity_sha256: ending_snow_sha256,
        soil_candidate_ending_identity_sha256: ending_soil_sha256,
        receipt_sha256: Digest32::zero(),
    }
    .seal()
    .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))
}

struct PhysicalOutcomeLedgerInputs<'a> {
    support: openwepp_coupled_time::TimeSupport,
    ending: &'a BTreeMap<u32, DirectSnowStage3PersistentState>,
    lanes: &'a BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    destinations: &'a BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    precipitation: &'a BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    soil: &'a BTreeMap<u32, SnowSoilHeatReceiptV1>,
    terminal_soil: &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>,
    adaptive_trial_soil: &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
    terminal_events: &'a BTreeMap<u32, DirectSnowTerminalEventResult>,
    /// Vapor material enthalpy, active/lower interlayer custody, and the
    /// independently reported Stage-3 snow--soil energy, respectively.
    diagnostics: &'a BTreeMap<u32, (f64, f64, f64, f64)>,
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    #[allow(clippy::too_many_lines)]
    fn execute_precomputed_terminal_accepted_endpoint(
        &mut self,
        input: &V11ImportedV10SegmentInput,
        endpoint: PrecomputedTerminalAcceptedEndpointV1,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let terminal_lanes = endpoint
            .terminal_events
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let adaptive_lanes = endpoint
            .terminal_snow_soil_trial_receipts
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        if adaptive_lanes.is_empty() || !terminal_lanes.is_subset(&adaptive_lanes) {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal accepted lane topology",
            ));
        }
        if endpoint.accepted_slab_sha256 != input.accepted_slab_receipt.slab_id().digest()
            || endpoint.accepted_envelope_support != input.support
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal accepted envelope identity",
            ));
        }
        let final_child_support = endpoint.carrier_phase.transition.boundary.support;
        if final_child_support.start_ns() < endpoint.accepted_envelope_support.start_ns()
            || final_child_support.end_ns() != endpoint.accepted_envelope_support.end_ns()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal final-child support identity",
            ));
        }
        if endpoint.beginning_pending_terminal_parcels != self.pending_terminal_parcels {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal beginning parcel identity",
            ));
        }
        if endpoint
            .terminal_snow_soil_trial_receipt_chains_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != adaptive_lanes
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal trial-chain topology",
            ));
        }
        let accepted_child_count = endpoint
            .terminal_snow_soil_trial_receipt_chains_by_lane
            .values()
            .next()
            .map(Vec::len)
            .ok_or(DirectV11RealConsumerError::Identity(
                "precomputed terminal accepted child count",
            ))?;
        if endpoint
            .terminal_snow_soil_trial_receipt_chains_by_lane
            .values()
            .any(|chain| chain.len() != accepted_child_count)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal accepted child-count topology",
            ));
        }
        if endpoint.carrier_phase_chain.len() != accepted_child_count {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal carrier/trial child-count identity",
            ));
        }
        for (index, phase) in endpoint.carrier_phase_chain.iter().enumerate() {
            let expected_ordinal = input
                .accepted_slab_receipt
                .slab_ordinal()
                .checked_add(u32::try_from(index).map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "precomputed terminal carrier child-count width",
                    )
                })?)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "precomputed terminal carrier child ordinal overflow",
                ))?;
            if phase.transition.probe_child_identity.physical_child_ordinal != expected_ordinal
                || endpoint
                    .terminal_snow_soil_trial_receipt_chains_by_lane
                    .iter()
                    .any(|(lane_id, chain)| {
                        chain.get(index).is_none_or(|receipt| {
                            receipt.support != phase.transition.boundary.support
                                || phase
                                    .batch_terminal_snow_soil_trial_receipts_by_lane
                                    .get(lane_id)
                                    != Some(receipt)
                        })
                    })
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "precomputed terminal carrier/trial child identity",
                ));
            }
        }
        let last_phase = endpoint.carrier_phase_chain.last().ok_or(
            DirectV11RealConsumerError::Identity("precomputed terminal empty carrier chain"),
        )?;
        if last_phase.transition.boundary.support != endpoint.carrier_phase.transition.boundary.support
            || last_phase.transition.probe_child_identity.receipt_sha256
                != endpoint.carrier_phase.transition.probe_child_identity.receipt_sha256
            || last_phase.ending_candidates.joint().receipt_sha256()
                != endpoint.carrier_phase.ending_candidates.joint().receipt_sha256()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal final carrier identity",
            ));
        }
        let final_physical_child_ordinal = input
            .accepted_slab_receipt
            .slab_ordinal()
            .checked_add(u32::try_from(accepted_child_count - 1).map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "precomputed terminal accepted child-count width",
                )
            })?)
            .ok_or(DirectV11RealConsumerError::Identity(
                "precomputed terminal accepted child ordinal overflow",
            ))?;
        if endpoint
            .carrier_phase
            .transition
            .probe_child_identity
            .physical_child_ordinal
            != final_physical_child_ordinal
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal final-child ordinal identity",
            ));
        }
        let mut integrated_snow_soil_heat_by_lane = BTreeMap::new();
        for (lane_id, final_receipt) in &endpoint.terminal_snow_soil_trial_receipts {
            let chain = endpoint
                .terminal_snow_soil_trial_receipt_chains_by_lane
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "precomputed terminal trial-chain lane",
                ))?;
            if chain.is_empty()
                || chain.first().is_none_or(|receipt| {
                    receipt.support.start_ns() != endpoint.accepted_envelope_support.start_ns()
                })
                || chain.last() != Some(final_receipt)
                || final_receipt.support != final_child_support
                || chain.windows(2).any(|pair| {
                    pair[0].support.end_ns() != pair[1].support.start_ns()
                })
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "precomputed terminal trial-chain support",
                ));
            }
            let mut snow_heat_j_m2 = 0.0;
            for receipt in chain {
                receipt.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "precomputed terminal trial-chain receipt seal",
                    )
                })?;
                if receipt.lane_id != *lane_id {
                    return Err(DirectV11RealConsumerError::Identity(
                        "precomputed terminal trial-chain lane identity",
                    ));
                }
                snow_heat_j_m2 += receipt.snow_heat_j_m2;
            }
            integrated_snow_soil_heat_by_lane.insert(*lane_id, snow_heat_j_m2);
        }
        let phase_child =
            digest32_from_lower_hex(&endpoint.carrier_phase.wb14_child_receipt_set_sha256)?;
        let phase_parent = endpoint
            .carrier_phase
            .wb14_parent_receipt_set_sha256
            .as_deref()
            .map(digest32_from_lower_hex)
            .transpose()?;
        if phase_child != endpoint.wb14_child_receipt_set_sha256
            || phase_parent != endpoint.wb14_parent_receipt_set_sha256
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal WB14 binding",
            ));
        }
        let beginning_owner_states = input
            .staged_resource_owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()?;
        let beginning_owner_set_sha256 = openwepp_coupled_time::complete_owner_set_digest(
            &beginning_owner_states,
        )
        .map_err(|_| {
            DirectV11RealConsumerError::Identity("precomputed terminal beginning owner set")
        })?;
        let recomputed_pre_event_authority =
            precomputed_terminal_pre_event_authority_sha256_v1(&endpoint)?;
        if endpoint.beginning_owner_set_sha256 != beginning_owner_set_sha256
            || endpoint.pre_event_authority_sha256 != recomputed_pre_event_authority
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precomputed terminal pre-event authority",
            ));
        }
        let publication_posture = match self.terminal_publication_posture {
            TerminalPublicationPostureV1::RetainFinal => {
                AcceptedPublicationFinalizationPostureV1::RetainFinal
            }
            TerminalPublicationPostureV1::DeferProvisional {
                pre_event_authority_sha256,
            } if pre_event_authority_sha256 == endpoint.pre_event_authority_sha256 => {
                AcceptedPublicationFinalizationPostureV1::DeferTerminalProvisional {
                    pre_event_authority_sha256,
                }
            }
            TerminalPublicationPostureV1::DeferProvisional { .. } => {
                return Err(DirectV11RealConsumerError::Identity(
                    "terminal provisional publication authority substitution",
                ));
            }
        };
        for (lane_id, trial) in &endpoint.terminal_snow_soil_trial_receipts {
            let ending = endpoint.ending_stage3_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("precomputed terminal ending lane"),
            )?;
            trial.validate().map_err(|_| {
                DirectV11RealConsumerError::Identity("precomputed terminal trial seal")
            })?;
            let terminal = endpoint.terminal_events.get(lane_id);
            let endpoint_is_valid = terminal.map_or_else(
                || {
                    crate::hydrology::stage3_is_resolved_thermal_domain(ending)
                        || crate::hydrology::stage3_is_terminal_event_domain(ending)
                },
                |event| {
                    event.event_occurred
                        && event.unevaluated_seconds.abs() <= 1.0e-6
                        && !crate::hydrology::stage3_has_represented_ice(ending)
                        && ending.layers.is_empty()
                        && ending.detached_retained_liquid_kg_m2.to_bits() == 0.0_f64.to_bits()
                },
            );
            if trial.support != final_child_support
                || trial.lane_id != *lane_id
                || !endpoint_is_valid
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "precomputed adaptive physical endpoint",
                ));
            }
        }

        let composed_precipitation = compose_terminal_precipitation_sets_v1(
            &endpoint.carrier_phase_chain,
            endpoint.accepted_envelope_support,
        )?;

        let mut evidence = self.seal_accepted_carrier_evidence_v1(
            &endpoint.carrier_phase,
            input,
            &endpoint.ending_stage3_by_lane,
        )?;
        for (lane_id, event) in &endpoint.terminal_events {
            let event_result_sha256 =
                crate::snow_stage3_v11_attachment::canonical_terminal_event_result_digest(event)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "accepted terminal bounded-vapor event identity",
                        )
                    })?;
            let raw_lane = evidence.final_lanes.get(lane_id).cloned().ok_or(
                DirectV11RealConsumerError::Identity("accepted terminal bounded-vapor lane"),
            )?;
            let actual_vapor_to_canopy_air_kg_m2 = *endpoint
                .final_child_actual_vapor_to_canopy_air_kg_m2_by_lane
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted terminal final-child bounded-vapor lane",
                ))?;
            let actual_latent_energy_to_canopy_air_j_m2 =
                actual_vapor_to_canopy_air_kg_m2 * raw_lane.aggregate_latent_heat_j_kg;
            let bounded_lane = raw_lane
                .with_terminal_bounded_vapor(
                    event_result_sha256,
                    actual_vapor_to_canopy_air_kg_m2,
                    actual_latent_energy_to_canopy_air_j_m2,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("accepted terminal bounded-vapor receipt")
                })?;
            evidence.final_lanes.insert(*lane_id, bounded_lane);
        }
        let mut terminal_receipts = BTreeMap::new();
        for lane_id in &terminal_lanes {
            let trial = endpoint
                .terminal_snow_soil_trial_receipts
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted terminal trial lane",
                ))?;
            let beginning = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("accepted terminal beginning lane"),
            )?;
            let ending = endpoint.ending_stage3_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("accepted terminal ending lane"),
            )?;
            let ending_soil = endpoint
                .carrier_phase
                .soil_candidate
                .ofes
                .iter()
                .find(|value| value.ofe_id == trial.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted terminal ending soil OFE",
                ))?;
            let integrated_snow_heat_j_m2 = *integrated_snow_soil_heat_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity(
                    "accepted terminal integrated snow-soil heat lane",
                ),
            )?;
            let event_snow_heat_j_m2 = endpoint
                .terminal_events
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted terminal integrated event lane",
                ))?
                .snow_soil_heat_energy_j_m2;
            let heat_reseal_residual_j_m2 =
                (integrated_snow_heat_j_m2 - event_snow_heat_j_m2).abs();
            if !snow_soil_receipt_reseal_roundoff_within_bound_v1(
                heat_reseal_residual_j_m2,
                0.0,
            ) {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted terminal integrated snow-soil heat identity",
                ));
            }
            crate::snow_stage3_v11_attachment::record_covered_receipt_reseal_roundoff_v1(
                heat_reseal_residual_j_m2,
                0.0,
            );
            let receipt = physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1 {
                support: input.support,
                lane_id: *lane_id,
                ofe_id: trial.ofe_id.clone(),
                beginning_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
                ),
                ending_dormant_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending)?,
                ),
                ending_soil_owner_sha256: digest_bytes(&serde_json::to_vec(ending_soil).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "accepted terminal ending soil identity",
                        )
                    },
                )?),
                limiting_boundary_receipt_sha256: trial.receipt_sha256,
                snow_heat_j_m2: integrated_snow_heat_j_m2,
                soil_heat_j_m2: -integrated_snow_heat_j_m2,
                receipt_sha256: Digest32::zero(),
            }
            .seal()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("accepted terminal snow-soil receipt")
            })?;
            terminal_receipts.insert(*lane_id, receipt);
        }
        let diagnostics = endpoint
            .terminal_snow_soil_trial_receipts
            .keys()
            .map(|lane_id| {
                let integrated = integrated_snow_soil_heat_by_lane.get(lane_id).ok_or(
                    DirectV11RealConsumerError::Identity(
                        "accepted terminal diagnostic snow-soil heat lane",
                    ),
                )?;
                Ok((*lane_id, (0.0, 0.0, 0.0, *integrated)))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        let mut persistent_receipts = self.snow_soil_heat_receipts(
            input.support,
            &endpoint.ending_stage3_by_lane,
            &endpoint.carrier_phase.soil_candidate,
        )?;
        // Retain exactly the persistent receipt domain used by final owner
        // validation. A terminal lane can reach dormant state before its event
        // receipt is installed into the accepted package, so the endpoint
        // thermal domain—not only the event map—owns this partition.
        persistent_receipts.retain(|lane, _| {
            let Some(beginning) = self.stage3_beginning_by_lane.get(lane) else {
                return false;
            };
            let Some(ending) = endpoint.ending_stage3_by_lane.get(lane) else {
                return false;
            };
            crate::hydrology::stage3_is_resolved_thermal_domain(beginning)
                && (crate::hydrology::stage3_is_resolved_thermal_domain(ending)
                    || crate::hydrology::stage3_is_terminal_event_domain(ending))
        });
        if persistent_receipts
            .keys()
            .any(|lane| terminal_lanes.contains(lane))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "adaptive snow-soil persistent/terminal receipt overlap",
            ));
        }
        let physical_ledgers = self.physical_outcome_ledgers(&PhysicalOutcomeLedgerInputs {
            support: input.support,
            ending: &endpoint.ending_stage3_by_lane,
            lanes: &evidence.final_lanes,
            destinations: &evidence.final_boundaries,
            precipitation: &composed_precipitation,
            soil: &persistent_receipts,
            terminal_soil: &terminal_receipts,
            adaptive_trial_soil: &endpoint.terminal_snow_soil_trial_receipts,
            terminal_events: &endpoint.terminal_events,
            diagnostics: &diagnostics,
        })?;
        if !adaptive_lanes
            .iter()
            .all(|lane_id| physical_ledgers.contains_key(lane_id))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted terminal physical ledger set",
            ));
        }
        let ending_snow_owner_bytes =
            canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
                &endpoint.ending_stage3_by_lane,
                &endpoint.beginning_pending_terminal_parcels,
                &evidence.final_lanes,
                &evidence.final_boundaries,
            )?;
        let mut soil_credits = Vec::new();
        if endpoint
            .carrier_phase
            .batch_soil_top_boundary_credits_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != adaptive_lanes
        {
            return Err(DirectV11RealConsumerError::Identity(
                "adaptive terminal batch soil-credit topology",
            ));
        }
        soil_credits.extend(
            endpoint
                .carrier_phase
                .batch_soil_top_boundary_credits_by_lane
                .values()
                .cloned(),
        );
        let compositional_envelopes = endpoint
            .carrier_phase_chain
            .iter()
            .map(|phase| phase.carrier_envelope.clone())
            .collect::<Vec<_>>();
        let terminal_physical_reuse_seed = if matches!(
            publication_posture,
            AcceptedPublicationFinalizationPostureV1::DeferTerminalProvisional { .. }
        ) {
            Some(CoveredTerminalPhysicalReuseSeedV1 {
                physical_authority_sha256: precomputed_terminal_physical_authority_sha256_v1(
                    &endpoint,
                )?,
                envelope: endpoint.carrier_phase.carrier_envelope.clone(),
                compositional_envelopes: compositional_envelopes.clone(),
                ending_snow_owner_bytes: ending_snow_owner_bytes.clone(),
                soil_top_boundary_credits: soil_credits.clone(),
            })
        } else {
            None
        };
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &self.beginning,
            input,
            &endpoint.carrier_phase.carrier_envelope,
            Some(&compositional_envelopes),
            Some(endpoint.carrier_phase.ending_candidates.shadow()),
            ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &soil_credits,
            &physical_ledgers,
            publication_posture,
        )?;

        // Publication fields are assigned only after every acceptance and
        // finalization guard above has succeeded.
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(evidence.final_boundaries);
        self.last_lane_boundary_receipts = Some(evidence.final_lanes);
        self.last_component_carrier_receipts = Some(evidence.component_receipts);
        self.last_snow_soil_heat_receipts = Some(persistent_receipts);
        self.last_terminal_snow_soil_heat_receipts = Some(terminal_receipts);
        self.last_adaptive_terminal_snow_soil_trial_receipts =
            Some(endpoint.terminal_snow_soil_trial_receipts);
        self.last_precipitation_parcel_sets = Some(composed_precipitation);
        self.last_physical_outcome_ledgers = Some(physical_ledgers);
        self.last_terminal_events = Some(endpoint.terminal_events);
        self.last_wb14_child_receipt_set_sha256 = Some(evidence.wb14_child_receipt_set_sha256);
        self.last_wb14_parent_receipt_set_sha256 = evidence.wb14_parent_receipt_set_sha256;
        self.last_wb14_child_replay_bytes = Some(evidence.wb14_child_replay_bytes);
        self.last_wb14_parent_replay_bytes = evidence.wb14_parent_replay_bytes;
        self.ending_stage3_by_lane = Some(endpoint.ending_stage3_by_lane);
        self.ending = Some(candidate);
        self.terminal_physical_reuse_seed = terminal_physical_reuse_seed;
        self.last_publication_retained = Some(matches!(
            publication_posture,
            AcceptedPublicationFinalizationPostureV1::RetainFinal
        ));
        Ok(output)
    }

    #[allow(clippy::too_many_lines)]
    fn physical_outcome_ledgers(
        &self,
        inputs: &PhysicalOutcomeLedgerInputs<'_>,
    ) -> Result<
        BTreeMap<u32, physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>,
        DirectV11RealConsumerError,
    > {
        use physical_outcome_ledger::{
            Stage3LanePhysicalOutcomeExpectationV1, Stage3LanePhysicalOutcomeLedgerV1,
        };
        let support = inputs.support;
        let duration_s = f64::from_bits(support.duration_s_bits());
        let mut result = BTreeMap::new();
        for (lane_id, beginning) in &self.stage3_beginning_by_lane {
            let ending = inputs
                .ending
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger ending lane",
                ))?;
            let terminal_ending = self.terminal_endpoint_mode
                && ending.layers.is_empty()
                && !crate::hydrology::stage3_has_represented_ice(ending)
                && ending.detached_retained_liquid_kg_m2.to_bits() == 0.0_f64.to_bits();
            let reappearance = beginning.layers.is_empty()
                && crate::hydrology::stage3_has_represented_ice(ending)
                && inputs.precipitation.get(lane_id).is_some_and(|set| {
                    set.parcels.iter().any(|parcel| {
                        parcel.phase == Stage3PrecipitationPhaseV1::Solid
                            && parcel.mass_kg_m2_tile_ground > 0.0
                    })
                });
            if (!crate::hydrology::stage3_is_resolved_thermal_domain(beginning)
                && !crate::hydrology::stage3_is_terminal_event_domain(beginning)
                && !reappearance)
                || (!crate::hydrology::stage3_is_resolved_thermal_domain(ending)
                    && !crate::hydrology::stage3_is_terminal_event_domain(ending)
                    && !terminal_ending)
            {
                continue;
            }
            let lane = inputs
                .lanes
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger boundary lane",
                ))?;
            let parcels =
                inputs
                    .precipitation
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "physical ledger precipitation lane",
                    ))?;
            let soil = inputs.soil.get(lane_id);
            let adaptive_trial_soil = inputs.adaptive_trial_soil.get(lane_id);
            if soil.is_none() && adaptive_trial_soil.is_none() && !terminal_ending && !reappearance
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "physical ledger soil lane",
                ));
            }
            let beginning_digest = digest_bytes(
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
            );
            let ending_digest = digest_bytes(
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending)?,
            );
            let mut solid = 0.0;
            let mut liquid = 0.0;
            let mut advection = 0.0;
            for parcel in &parcels.parcels {
                let destination = parcels
                    .destinations
                    .iter()
                    .find(|value| value.topology_index == parcel.destination_topology_index)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "physical ledger precipitation destination",
                    ))?;
                let mass = parcel.mass_kg_m2_tile_ground * destination.fraction_of_ofe;
                let specific_enthalpy = match parcel.enthalpy_provider {
                    Stage3PrecipitationEnthalpyProviderV1::Temperature {
                        temperature_k,
                        reference_temperature_k,
                        specific_heat_j_kg_k,
                        ..
                    } => specific_heat_j_kg_k * (temperature_k - reference_temperature_k),
                    Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                        specific_enthalpy_j_kg,
                        ..
                    } => specific_enthalpy_j_kg,
                };
                advection += mass * specific_enthalpy;
                match parcel.phase {
                    Stage3PrecipitationPhaseV1::Solid => solid += mass,
                    Stage3PrecipitationPhaseV1::Liquid => liquid += mass,
                }
            }
            let beginning_ice = beginning
                .layers
                .iter()
                .map(|v| v.mass_swe_m * 1_000.0)
                .sum::<f64>();
            let ending_ice = ending
                .layers
                .iter()
                .map(|v| v.mass_swe_m * 1_000.0)
                .sum::<f64>();
            let beginning_liquid = beginning
                .layers
                .iter()
                .map(|v| v.liquid_water_m * 1_000.0)
                .sum::<f64>()
                + beginning.detached_retained_liquid_kg_m2;
            let ending_liquid = ending
                .layers
                .iter()
                .map(|v| v.liquid_water_m * 1_000.0)
                .sum::<f64>()
                + ending.detached_retained_liquid_kg_m2;
            let deposition =
                ending.cumulative_deposition_kg_m2 - beginning.cumulative_deposition_kg_m2;
            let sublimation =
                ending.cumulative_sublimation_kg_m2 - beginning.cumulative_sublimation_kg_m2;
            let melt = ending.cumulative_melt_kg_m2 - beginning.cumulative_melt_kg_m2;
            // Reconstruct phase conversion from the independent ice owner.
            // The per-layer refrozen counter is diagnostic and does not
            // include every rain-on-snow conversion represented by the
            // accepted ending ice mass.
            let reconstructed_refreeze =
                ending_ice - (beginning_ice + solid + deposition - sublimation - melt);
            // Refreeze is a nonnegative process amount. Preserve the original
            // independently reconstructed ice equation: if a negative value
            // is material, replacing only the process amount with zero leaves
            // that discrepancy in `ice_residual_kg_m2` and closure fails at
            // the unchanged ledger tolerance.
            let refreeze = reconstructed_refreeze.max(0.0);
            let terminal = ending.cumulative_unresolved_liquid_kg_m2
                - beginning.cumulative_unresolved_liquid_kg_m2;
            let terminal_liquid_sensible_enthalpy = ending
                .cumulative_terminal_unallocated_energy_j_m2
                - beginning.cumulative_terminal_unallocated_energy_j_m2;
            // Independently reconstruct the lane exchange from the immutable
            // destination records in their sealed semantic topology order.
            // The lane aggregate is an audited result, never ledger authority.
            let mut destination_aggregate = [0.0_f64; 5];
            for (destination, receipt) in inputs
                .destinations
                .iter()
                .filter(|(destination, _)| destination.0 == lane.ofe_id)
            {
                let physical = receipt.physical_operands();
                let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
                destination_aggregate[0] += fraction * physical[0];
                destination_aggregate[1] += fraction * physical[2];
                destination_aggregate[2] += fraction * physical[3];
                destination_aggregate[3] += fraction * physical[4];
                destination_aggregate[4] += fraction * physical[1];
            }
            let (receipt_vapor, receipt_latent) =
                lane.terminal_bounded_vapor_receipt.as_ref().map_or(
                    (
                        lane.aggregate_vapor_to_canopy_air_kg_m2_s,
                        lane.aggregate_latent_energy_to_canopy_air_j_m2,
                    ),
                    |receipt| {
                        (
                            receipt.raw_vapor_to_canopy_air_kg_m2_s,
                            receipt.raw_latent_energy_to_canopy_air_j_m2,
                        )
                    },
                );
            let aggregate_receipt = [
                lane.aggregate_sensible_to_canopy_air_w_m2,
                receipt_latent,
                lane.aggregate_snow_absorbed_shortwave_w_m2,
                lane.aggregate_snow_net_longwave_w_m2,
                receipt_vapor,
            ];
            validate_destination_reconstruction_against_lane_aggregate(
                destination_aggregate,
                aggregate_receipt,
            )?;
            let terminal_event = inputs.terminal_events.get(lane_id);
            let sensible = terminal_event.map_or(-destination_aggregate[0] * duration_s, |event| {
                event.sensible_energy_j_m2
            });
            let latent = terminal_event.map_or(-lane.aggregate_latent_energy_to_canopy_air_j_m2, |event| {
                event.latent_energy_j_m2
            });
            let shortwave = terminal_event.map_or(destination_aggregate[2] * duration_s, |event| {
                event.shortwave_energy_j_m2
            });
            let longwave = terminal_event.map_or(destination_aggregate[3] * duration_s, |event| {
                event.longwave_energy_j_m2
            });
            // Reconstruct material enthalpy from immutable owner state, never
            // from the accepted external-energy sum being audited.
            let beginning_cold = beginning
                .layers
                .iter()
                .map(|v| v.cold_content_j_m2)
                .sum::<f64>();
            let ending_cold = ending
                .layers
                .iter()
                .map(|v| v.cold_content_j_m2)
                .sum::<f64>();
            let beginning_enthalpy =
                -beginning_cold + OUTCOME_LATENT_HEAT_FUSION_J_KG * beginning_liquid;
            let ending_enthalpy = -ending_cold + OUTCOME_LATENT_HEAT_FUSION_J_KG * ending_liquid;
            let (vapor_material_enthalpy, interlayer_active, interlayer_lower, soil_heat) = *inputs
                .diagnostics
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger diagnostic lane",
                ))?;
            let soil_receipt_sha256 = if let Some(receipt) = adaptive_trial_soil {
                receipt.receipt_sha256
            } else if terminal_ending {
                inputs
                    .terminal_soil
                    .get(lane_id)
                    .map_or(Digest32::zero(), |receipt| receipt.receipt_sha256)
            } else if reappearance {
                digest_bytes(b"SC-SNOWENERGY-001@22-REAPPEARANCE-NO-SNOW-SOIL-EXCHANGE-V1")
            } else {
                soil.map_or(Digest32::zero(), |receipt| receipt.receipt_sha256)
            };
            let expected = Stage3LanePhysicalOutcomeExpectationV1 {
                support,
                lane_id: *lane_id,
                ofe_id: lane.ofe_id.clone(),
                topology_sha256: lane.topology_configuration_sha256,
                beginning_snow_owner_sha256: beginning_digest,
                ending_snow_owner_sha256: ending_digest,
                precipitation_set_sha256: parcels.receipt_sha256,
                source_receipts_sha256: [
                    lane.optical_receipt_sha256,
                    lane.reciprocal_longwave_receipt_sha256,
                    lane.provisional_carrier_receipt_sha256,
                    lane.final_destination_receipt_sha256,
                    soil_receipt_sha256,
                    lane.receipt_sha256,
                ],
            };
            let (destination_liquid_outcomes, terminal, terminal_liquid_sensible_enthalpy) =
                super::physical_outcome_ledger::seal_destination_liquid_outcomes_v1(
                    &lane.ofe_id,
                    lane.ordered_destinations
                        .iter()
                        .map(|destination| (&destination.tile_id, destination.tile_fraction)),
                    terminal,
                    terminal_liquid_sensible_enthalpy,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "physical outcome destination liquid authority",
                    )
                })?;
            let value = Stage3LanePhysicalOutcomeLedgerV1 {
                support,
                lane_id: *lane_id,
                ofe_id: lane.ofe_id.clone(),
                area_basis: Stage3LaneAreaBasisV1::OfeGround,
                topology_sha256: lane.topology_configuration_sha256,
                beginning_snow_owner_sha256: beginning_digest,
                ending_snow_owner_sha256: ending_digest,
                precipitation_set_sha256: parcels.receipt_sha256,
                source_receipts_sha256: expected.source_receipts_sha256,
                beginning_ice_kg_m2: beginning_ice,
                beginning_liquid_kg_m2: beginning_liquid,
                beginning_cold_content_j_m2: beginning_cold,
                beginning_enthalpy_j_m2: beginning_enthalpy,
                ending_ice_kg_m2: ending_ice,
                ending_liquid_kg_m2: ending_liquid,
                ending_cold_content_j_m2: ending_cold,
                ending_enthalpy_j_m2: ending_enthalpy,
                solid_precipitation_kg_m2: solid,
                liquid_precipitation_kg_m2: liquid,
                precipitation_advection_j_m2: terminal_event.map_or(advection, |event| {
                    event.advected_energy_j_m2
                }),
                deposition_kg_m2: deposition,
                sublimation_kg_m2: sublimation,
                vapor_transfer_kg_m2: deposition - sublimation,
                latent_heat_j_kg: lane.aggregate_latent_heat_j_kg,
                snow_surface_temperature_k: lane.aggregate_snow_temperature_k,
                vapor_material_enthalpy_j_m2: vapor_material_enthalpy,
                melt_kg_m2: melt,
                refreeze_kg_m2: refreeze,
                terminal_liquid_kg_m2: terminal,
                terminal_liquid_sensible_enthalpy_j_m2: terminal_liquid_sensible_enthalpy,
                destination_liquid_outcomes,
                retained_liquid_kg_m2: ending_liquid,
                shortwave_j_m2: shortwave,
                longwave_j_m2: longwave,
                sensible_j_m2: sensible,
                latent_j_m2: latent,
                soil_heat_j_m2: soil_heat,
                interlayer_active_conduction_j_m2: interlayer_active,
                interlayer_lower_conduction_j_m2: interlayer_lower,
                interlayer_conduction_j_m2: interlayer_active + interlayer_lower,
                refreeze_fusion_j_m2: 0.0,
                mass_residual_kg_m2: 0.0,
                ice_residual_kg_m2: 0.0,
                liquid_residual_kg_m2: 0.0,
                vapor_residual_kg_m2: 0.0,
                energy_residual_j_m2: 0.0,
                ending_liquid_residual_kg_m2: 0.0,
                receipt_sha256: Digest32::zero(),
            };
            let ledger = Stage3LanePhysicalOutcomeLedgerV1::try_new(value.clone(), &expected)
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("physical outcome ledger closure")
                })?;
            result.insert(*lane_id, ledger);
        }
        Ok(result)
    }

    fn snow_soil_heat_receipts(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &SoilThermalSnapshot,
    ) -> Result<BTreeMap<u32, SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        let lane_to_ofe = self.covered_lane_to_ofe(&self.stage3_beginning_by_lane)?;
        let mut topology_bytes = Vec::new();
        for binding in &self.beginning.inner.surface_configuration.ofe_bindings {
            topology_bytes.extend_from_slice(&binding.production_lane_id.to_be_bytes());
            topology_bytes.extend_from_slice(binding.ofe_id.as_str().as_bytes());
            topology_bytes.push(0);
        }
        let topology_identity_sha256 = digest_bytes(&topology_bytes);
        let configuration_identity_sha256 = digest32_from_lower_hex(
            self.beginning
                .inner
                .lse_configuration
                .configuration_sha256
                .as_str(),
        )?;
        let beginning_soil_owner_identity_sha256 =
            digest32_from_lower_hex(self.beginning.inner.soil_thermal.state_sha256.as_str())?;
        let mut receipts = BTreeMap::new();
        for (lane_id, ofe_id) in lane_to_ofe {
            if let Some(receipt) = self.snow_soil_heat_receipt_for_lane(
                support,
                lane_id,
                &ofe_id,
                trial_stage3,
                trial_soil,
                topology_identity_sha256,
                configuration_identity_sha256,
                beginning_soil_owner_identity_sha256,
            )? {
                receipts.insert(lane_id, receipt);
            }
        }
        Ok(receipts)
    }

    fn retain_terminal_limiting_snow_soil_receipts(
        &self,
        mut next: BTreeMap<u32, SnowSoilHeatReceiptV1>,
        previous: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
        if self.terminal_endpoint_mode {
            for (lane_id, state) in trial_stage3 {
                if state.layers.is_empty()
                    && !crate::hydrology::stage3_has_represented_ice(state)
                    && state.detached_retained_liquid_kg_m2.to_bits() == 0.0_f64.to_bits()
                {
                    if let Some(limiting) = previous.get(lane_id) {
                        next.insert(*lane_id, limiting.clone());
                    }
                }
            }
        }
        next
    }

    fn terminal_snow_soil_heat_receipts(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        ending: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        installed_soil: &SoilThermalSnapshot,
        limiting: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        diagnostics: &BTreeMap<u32, (f64, f64, f64, f64)>,
    ) -> Result<
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>,
        DirectV11RealConsumerError,
    > {
        let mut result = BTreeMap::new();
        for (lane_id, ending_state) in ending {
            if !self.terminal_endpoint_mode
                || !ending_state.layers.is_empty()
                || crate::hydrology::stage3_has_represented_ice(ending_state)
                || ending_state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            {
                continue;
            }
            let beginning = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("terminal snow-soil beginning lane"),
            )?;
            let limiting_receipt =
                limiting
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "terminal snow-soil limiting receipt",
                    ))?;
            let ending_soil_ofe = installed_soil
                .ofes
                .iter()
                .find(|value| value.ofe_id == limiting_receipt.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "terminal snow-soil ending OFE",
                ))?;
            let snow_heat = diagnostics
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "terminal snow-soil event-integrated operand",
                ))?
                .3;
            let receipt = physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1 {
                support,
                lane_id: *lane_id,
                ofe_id: limiting_receipt.ofe_id.clone(),
                beginning_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
                ),
                ending_dormant_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending_state)?,
                ),
                ending_soil_owner_sha256: digest_bytes(
                    &serde_json::to_vec(ending_soil_ofe).map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "terminal snow-soil ending soil identity",
                        )
                    })?,
                ),
                limiting_boundary_receipt_sha256: limiting_receipt.receipt_sha256,
                snow_heat_j_m2: snow_heat,
                soil_heat_j_m2: -snow_heat,
                receipt_sha256: Digest32::zero(),
            }
            .seal()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal snow-soil receipt closure")
            })?;
            result.insert(*lane_id, receipt);
        }
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn snow_soil_heat_receipt_for_lane(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        lane_id: u32,
        ofe_id: &OfeId,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &SoilThermalSnapshot,
        topology_identity_sha256: Digest32,
        configuration_identity_sha256: Digest32,
        beginning_soil_owner_identity_sha256: Digest32,
    ) -> Result<Option<SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        let inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("snow-soil Stage-3 inputs"),
        )?;
        let beginning_stage = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("snow-soil beginning snow owner"),
        )?;
        let ending_stage =
            trial_stage3
                .get(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil trial snow owner",
                ))?;
        // An unresolved lane has no admissible snow thermal boundary.
        // Preserve its carried owner unchanged and emit no heat receipt or
        // soil credit; fabricating a bottom temperature/conductivity would
        // violate the canonical Stage-3 lifecycle boundary.
        let beginning_terminal = crate::hydrology::stage3_is_terminal_event_domain(beginning_stage);
        if !crate::hydrology::stage3_is_resolved_thermal_domain(beginning_stage)
            && !beginning_terminal
        {
            return Ok(None);
        }
        let beginning_bottom = if beginning_terminal {
            Wb11HydrologyKernel::project_stage3_terminal_bottom_volume_v1(
                beginning_stage,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        } else {
            Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
                beginning_stage,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        };
        let ending_bottom_temperature_k =
            if crate::hydrology::stage3_is_resolved_thermal_domain(ending_stage) {
                Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
                    ending_stage,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?
                .temperature_k
            } else if crate::hydrology::stage3_is_terminal_event_domain(ending_stage) {
                Wb11HydrologyKernel::project_stage3_terminal_bottom_volume_v1(
                    ending_stage,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?
                .temperature_k
            } else if self.terminal_endpoint_mode {
                // Dormancy has no projectable snow node.  A terminal-specific
                // event-integrated receipt must carry this custody; the
                // persistent Crank--Nicolson receipt cannot be completed by
                // fabricating an endpoint temperature.
                return Ok(None);
            } else {
                return Ok(None);
            };
        let configured_ofe = self
            .beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-soil configured OFE",
            ))?;
        let configured_top = configured_ofe.soil_interface_layers.first().ok_or(
            DirectV11RealConsumerError::Identity("snow-soil configured top layer"),
        )?;
        let beginning_ofe = self
            .beginning
            .inner
            .soil_thermal
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-soil beginning OFE",
            ))?;
        let ending_ofe = trial_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity("snow-soil trial OFE"))?;
        let beginning_top =
            beginning_ofe
                .ordered_layers
                .first()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil beginning top layer",
                ))?;
        let ending_top =
            ending_ofe
                .ordered_layers
                .first()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil trial top layer",
                ))?;
        if beginning_top.layer_id != configured_top.layer_id
            || ending_top.layer_id != configured_top.layer_id
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-soil ordered top-layer identity",
            ));
        }
        seal_snow_soil_heat_receipt(&SnowSoilReceiptInputs {
            support,
            duration_s: f64::from_bits(self.interval.lse_forcing.interval_s.to_bits()),
            lane_id,
            ofe_id,
            topology_identity_sha256,
            configuration_identity_sha256,
            beginning_soil_owner_identity_sha256,
            beginning_stage,
            ending_stage,
            ending_ofe,
            configured_top,
            beginning_top,
            ending_top,
            beginning_bottom_temperature_k: beginning_bottom.temperature_k,
            beginning_bottom_thickness_m: beginning_bottom.thickness_m,
            beginning_bottom_conductivity_w_m_k: beginning_bottom.thermal_conductivity_w_m_k,
            beginning_snow_owner_sha256: beginning_bottom.beginning_stage3_state_sha256,
            ending_bottom_temperature_k,
        })
        .map(Some)
    }

    fn soil_top_boundary_credits(
        &self,
        receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    ) -> Result<Vec<SoilThermalTopBoundaryCreditV1>, DirectV11RealConsumerError> {
        receipts
            .values()
            .map(|receipt| {
                crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
                    .map_err(|error| {
                        DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                    })?;
                let beginning_ofe = self
                    .beginning
                    .inner
                    .soil_thermal
                    .ofes
                    .iter()
                    .find(|value| value.ofe_id == receipt.ofe_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "soil credit beginning OFE",
                    ))?;
                let first_layer = beginning_ofe.ordered_layers.first().ok_or(
                    DirectV11RealConsumerError::Identity("soil credit first layer"),
                )?;
                crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt_installed_join(
                    receipt,
                    &first_layer.layer_id,
                    receipt.snow_candidate_ending_identity_sha256,
                    receipt.soil_candidate_ending_identity_sha256,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                })?;
                Ok(SoilThermalTopBoundaryCreditV1 {
                    lane_id: receipt.lane_id,
                    ofe_id: receipt.ofe_id.clone(),
                    first_layer_id: first_layer.layer_id.clone(),
                    beginning_owner_id: self.beginning.inner.soil_thermal.owner_id.clone(),
                    beginning_configuration_sha256: self
                        .beginning
                        .inner
                        .soil_thermal
                        .configuration_sha256
                        .clone(),
                    beginning_state_sha256: self.beginning.inner.soil_thermal.state_sha256.clone(),
                    support_start_ns: i64::try_from(receipt.support.start_ns().get()).map_err(
                        |_| DirectV11RealConsumerError::Identity("soil heat support start"),
                    )?,
                    support_end_ns: i64::try_from(receipt.support.end_ns().get()).map_err(
                        |_| DirectV11RealConsumerError::Identity("soil heat support end"),
                    )?,
                    accepted_positive_downward_j_m2_ofe_ground: receipt
                        .accepted_heat_j_m2_ofe_ground,
                    soil_thermal_credit_j_m2_ofe_ground: receipt
                        .soil_candidate_heat_j_m2_ofe_ground,
                    snow_soil_heat_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                        receipt.receipt_sha256,
                    ))
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("soil heat receipt digest")
                    })?,
                })
            })
            .collect()
    }

    fn validate_snow_soil_heat_receipt_iterate_joins(
        &self,
        receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &SoilThermalSnapshot,
    ) -> Result<(), DirectV11RealConsumerError> {
        for (lane_id, receipt) in receipts {
            let stage3 = trial_stage3
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil iterate lane join",
                ))?;
            if self.terminal_endpoint_mode
                && !crate::hydrology::stage3_has_represented_ice(stage3)
                && stage3.layers.is_empty()
            {
                // Terminal dormancy has no snow node. Its last limiting
                // receipt is joined separately by the terminal receipt path.
                continue;
            }
            let soil_ofe = trial_soil
                .ofes
                .iter()
                .find(|value| value.ofe_id == receipt.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil iterate OFE join",
                ))?;
            let first_layer =
                soil_ofe
                    .ordered_layers
                    .first()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "snow-soil iterate first-layer join",
                    ))?;
            let snow_sha256 = digest_bytes(&serde_json::to_vec(stage3).map_err(|_| {
                DirectV11RealConsumerError::Identity("snow-soil iterate snow serialization")
            })?);
            let soil_sha256 = digest_bytes(&serde_json::to_vec(soil_ofe).map_err(|_| {
                DirectV11RealConsumerError::Identity("snow-soil iterate soil serialization")
            })?);
            crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt_installed_join(
                receipt,
                &first_layer.layer_id,
                snow_sha256,
                soil_sha256,
            )
            .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
        }
        Ok(())
    }
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    fn execute_terminal_physical_reuse(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let endpoint = self.precomputed_terminal_accepted.take().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse endpoint"),
        )?;
        let seed = self.terminal_physical_reuse_seed.take().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse physical seed"),
        )?;
        let beginning_owner_states = input
            .staged_resource_owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()?;
        let beginning_owner_set_sha256 =
            openwepp_coupled_time::complete_owner_set_digest(&beginning_owner_states).map_err(
                |_| DirectV11RealConsumerError::Identity("covered terminal reuse beginning owner"),
            )?;
        if endpoint.accepted_slab_sha256 != input.accepted_slab_receipt.slab_id().digest() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse accepted slab",
            ));
        }
        if endpoint.accepted_envelope_support != input.support {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse support",
            ));
        }
        if endpoint.beginning_owner_set_sha256 != beginning_owner_set_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse beginning owner",
            ));
        }
        if endpoint.beginning_pending_terminal_parcels != self.pending_terminal_parcels {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse pending parcels",
            ));
        }
        if endpoint.pre_event_authority_sha256
            != precomputed_terminal_pre_event_authority_sha256_v1(&endpoint)?
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse pre-event seal",
            ));
        }
        if seed.physical_authority_sha256
            != precomputed_terminal_physical_authority_sha256_v1(&endpoint)?
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse physical authority",
            ));
        }
        let child_replay = self.last_wb14_child_replay_bytes.as_deref().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse WB14 child replay"),
        )?;
        let trial_binding = crate::direct_runtime::wb14_child_replay_binding(child_replay)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered terminal reuse WB14 trial binding")
            })?;
        validate_covered_terminal_reuse_trial_binding_v1(
            trial_binding,
            self.wb14_coupled_child_binding,
            endpoint.carrier_phase.transition.boundary.support,
            endpoint.wb14_replay_trial_sha256,
            endpoint.wb14_replay_beginning_owner_set_sha256,
        )?;
        crate::direct_runtime::validate_wb14_child_replay_binding(child_replay, trial_binding)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered terminal reuse WB14 child validation")
            })?;
        if let Some(parent) = self.last_wb14_parent_replay_bytes.as_deref() {
            crate::direct_runtime::validate_wb14_parent_replay(child_replay, parent).map_err(
                |_| DirectV11RealConsumerError::Identity("covered terminal reuse WB14 parent"),
            )?;
        }
        let child_sha256 = digest_bytes(child_replay);
        let parent_sha256 = self
            .last_wb14_parent_replay_bytes
            .as_deref()
            .map(digest_bytes);
        if endpoint.wb14_child_receipt_set_sha256 != child_sha256
            || endpoint.wb14_parent_receipt_set_sha256 != parent_sha256
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal reuse WB14 authorization",
            ));
        }
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &self.beginning,
            input,
            &seed.envelope,
            Some(&seed.compositional_envelopes),
            Some(self.ending.as_ref().ok_or(
                DirectV11RealConsumerError::Identity(
                    "covered terminal reuse precomputed physical ending",
                ),
            )?),
            seed.ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &seed.soil_top_boundary_credits,
            self.last_physical_outcome_ledgers.as_ref().ok_or(
                DirectV11RealConsumerError::Identity(
                    "covered terminal reuse physical outcome ledgers",
                ),
            )?,
            AcceptedPublicationFinalizationPostureV1::RetainFinal,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        Ok(output)
    }

    fn execute_ordinary_physical_reuse(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let seed = self.ordinary_physical_reuse_seed.take().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary physical reuse seed"),
        )?;
        validate_covered_ordinary_physical_reuse_gate_v1(
            seed.physical_authority,
            covered_ordinary_physical_authority_v1(input)?,
            self.terminal_endpoint_mode,
        )?;
        let child_replay = self.last_wb14_child_replay_bytes.as_deref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child replay"),
        )?;
        let (rebound_child, rebound_parent) =
            crate::direct_runtime::rebind_wb14_replay_to_accepted_slab(
                child_replay,
                self.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
            )
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child reseal")
            })?;
        crate::direct_runtime::validate_wb14_child_replay_binding(
            &rebound_child,
            self.wb14_coupled_child_binding,
        )
        .map_err(|_| {
            DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child validation")
        })?;
        if let Some(parent) = rebound_parent.as_deref() {
            crate::direct_runtime::validate_wb14_parent_replay(&rebound_child, parent).map_err(
                |_| {
                    DirectV11RealConsumerError::Identity(
                        "covered ordinary reuse WB14 parent validation",
                    )
                },
            )?;
        }
        self.last_wb14_child_receipt_set_sha256 = Some(digest32_hex(digest_bytes(&rebound_child)));
        self.last_wb14_parent_receipt_set_sha256 = rebound_parent
            .as_ref()
            .map(|bytes| digest32_hex(digest_bytes(bytes)));
        self.last_wb14_child_replay_bytes = Some(rebound_child);
        self.last_wb14_parent_replay_bytes = rebound_parent;
        let ending_stage3 =
            self.ending_stage3_by_lane
                .as_ref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered ordinary reuse Stage-3 ending",
                ))?;
        let lanes = self.last_lane_boundary_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse lane receipts"),
        )?;
        let destinations = self.last_final_boundary_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse boundary receipts"),
        )?;
        let soil = self.last_snow_soil_heat_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse snow-soil receipts"),
        )?;
        let ending_snow_owner_bytes =
            canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
                ending_stage3,
                &self.pending_terminal_parcels,
                lanes,
                destinations,
            )?;
        let soil_credits = self.soil_top_boundary_credits(soil)?;
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &self.beginning,
            input,
            &seed.envelope,
            None,
            None,
            ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &soil_credits,
            self.last_physical_outcome_ledgers.as_ref().ok_or(
                DirectV11RealConsumerError::Identity(
                    "covered ordinary reuse physical outcome ledgers",
                ),
            )?,
            AcceptedPublicationFinalizationPostureV1::RetainFinal,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        Ok(output)
    }
}

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
             precipitation_sets: &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>| {
                terminal_events.borrow_mut().clear();
                let terms = self.lane_stage3_terms_from_boundaries(
                    destination_receipts,
                    boundaries,
                    interval_s,
                )?;
                let mut ending_stage3 = self.stage3_beginning_by_lane.clone();
                let mut outcome_diagnostics_by_lane = BTreeMap::new();
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
                            snow_soil_heat_j_m2: match snow_soil_receipts.get(lane_id) {
                                Some(receipt) => receipt.snow_candidate_heat_j_m2_ofe_ground,
                                None if beginning.layers.is_empty()
                                    && stage3_forcing.forcing.snowfall_m > 0.0 =>
                                {
                                    0.0
                                }
                                None => {
                                    return Err(DirectV11RealConsumerError::Identity(
                                        "covered lane snow-soil heat receipt",
                                    ));
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
                Ok::<_, DirectV11RealConsumerError>((ending_stage3, outcome_diagnostics_by_lane))
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
        let mut previous_soil_state: Option<SoilThermalSnapshot> = None;
        let mut iteration_soil_state = self.beginning.inner.soil_thermal.clone();
        let support_duration_ns = input.support.duration_ns();
        let minimum_support_ns =
            crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
        let exact_floor_support = support_duration_ns == minimum_support_ns;
        let coarse_support_relaxation_enabled = support_duration_ns > minimum_support_ns;
        let mut exact_floor_period_two_relaxation_enabled = false;
        let mut accepted_snow_soil_receipts = self.snow_soil_heat_receipts(
            input.support,
            &iteration_stage3_states,
            &iteration_soil_state,
        )?;
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
            installed_precipitation_sets,
            installed_cold_content_export_by_lane,
            installed_soil_preview,
        ) = 'fixed_point: {
            use crate::snow_stage3_v11_attachment::{
                begin_adaptive_parent_fixed_point_phase_v1 as phase_start,
                record_adaptive_parent_fixed_point_phase_v1 as phase_record,
                record_adaptive_parent_profile_detail_v1 as profile_record,
            };
            for iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
                let operand_started = phase_start();
                let next_snow_soil_receipts = self.snow_soil_heat_receipts(
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
                    &iteration_soil_state,
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
                let mut provisional_candidate = self.beginning.clone();
                provisional_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
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
                )?;
                let provisional_precipitation_sets = provisional.precipitation_sets;
                let next_boundaries = provisional.corrected_boundaries;
                let lse_states = provisional.lse_states;
                let provisional_transaction_id = provisional.transaction_id;
                let provisional_soil_candidates = provisional.soil_candidates;
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
                phase_record("envelope", envelope_started);
                let stage3_started = phase_start();
                let (stage3_candidate, _) = evaluate_stage3(
                    &destination_receipts,
                    &next_boundaries,
                    None,
                    &accepted_snow_soil_receipts,
                    &provisional_precipitation_sets,
                )?;
                phase_record("stage3", stage3_started);
                let soil_started = phase_start();
                let soil_credits = self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?;
                let soil_candidate = aggregate_soil_thermal_ending_with_top_boundary_credits(
                    &self.beginning.inner.soil_thermal,
                    &self.beginning.inner.lse_configuration,
                    provisional_transaction_id,
                    &provisional_soil_candidates,
                    &soil_credits,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?
                .ending;
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
                let stage3_converged = if relaxation_enabled {
                    covered_fixed_point_stage3_states_equal(
                        &iteration_stage3_states,
                        &stage3_candidate,
                    )
                } else {
                    previous_stage3_states.as_ref().is_some_and(|previous| {
                        covered_fixed_point_stage3_states_equal(previous, &stage3_candidate)
                    })
                };
                let soil_converged = if relaxation_enabled {
                    covered_fixed_point_soil_states_equal(&iteration_soil_state, &soil_candidate)
                } else {
                    previous_soil_state.as_ref().is_some_and(|previous| {
                        covered_fixed_point_soil_states_equal(previous, &soil_candidate)
                    })
                };
                let boundary_converged =
                    previous_complete_boundaries
                        .as_ref()
                        .is_some_and(|previous| {
                            covered_fixed_point_boundaries_equal(previous, &next_boundaries)
                        });
                let converged =
                    lse_converged && stage3_converged && soil_converged && boundary_converged;
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
                let converged = converged && !covered_fixed_point_exhaustion_forced_for_test();
                if !converged {
                    // Apply under-relaxed Picard to the current iterate. The
                    // support scaling damps stronger coarse-support coupling,
                    // while the 0.25 contraction floor prevents ordinary long
                    // supports from exhausting the bounded solve solely due
                    // to vanishing iterate progress. The exact fallback floor
                    // remains raw unless an authentic A/B/A Stage-3 cycle has
                    // enabled the bounded 0.5 candidate weight.
                    let relaxation_weight = covered_fixed_point_relaxation_weight_v1(
                        input.support.duration_ns(),
                        exact_floor_period_two_relaxation_enabled,
                    );
                    let next_iteration_stage3 = relaxation_weight.and_then(|weight| {
                        covered_fixed_point_stage3_underrelaxed_iterate_v1(
                            &iteration_stage3_states,
                            &stage3_candidate,
                            weight,
                        )
                    });
                    let next_iteration_soil = relaxation_weight.and_then(|weight| {
                        covered_fixed_point_soil_underrelaxed_iterate_v1(
                            &iteration_soil_state,
                            &soil_candidate,
                            weight,
                        )
                    });
                    previous_lse_states = Some(lse_states);
                    previous_previous_stage3_states = previous_stage3_states.take();
                    previous_stage3_states = Some(stage3_candidate.clone());
                    iteration_stage3_states = next_iteration_stage3.unwrap_or(stage3_candidate);
                    previous_soil_state = Some(soil_candidate.clone());
                    iteration_soil_state = next_iteration_soil.unwrap_or(soil_candidate);
                    iteration_boundaries = Some(next_covered_boundaries);
                    previous_complete_boundaries = Some(next_boundaries);
                    continue;
                }

                // Re-seal from the converged candidate endpoints. These are
                // the identities retained by the parent join and replayed for
                // exact installation; the preceding receipt was only the
                // fixed-point operand generated from the prior trial.
                let finalization_started = phase_start();
                let finalization_candidate_started = phase_start();
                let final_snow_soil_receipts = self.snow_soil_heat_receipts(
                    input.support,
                    &stage3_candidate,
                    &soil_candidate,
                )?;
                accepted_snow_soil_receipts = self.retain_terminal_limiting_snow_soil_receipts(
                    final_snow_soil_receipts,
                    &accepted_snow_soil_receipts,
                    &stage3_candidate,
                );

                let mut final_candidate = self.beginning.clone();
                final_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let final_input_boundaries = self.merge_latest_stage3_state_operands(
                    &next_covered_boundaries,
                    &stage3_candidate,
                )?;
                let final_input_open_boundaries = self
                    .open_snow_boundaries_by_destination(&stage3_candidate)?
                    .1;
                let final_envelope =
                    self.build_covered_carrier_envelope_value_v1(CoveredCarrierEnvelopeBuildV1 {
                        candidate: &final_candidate,
                        interval_s,
                        duration_s_bits: input.duration_s_bits,
                        covered_destinations: &covered_destinations,
                        covered_boundaries: &final_input_boundaries,
                        open_boundaries: &final_input_open_boundaries,
                        provisional: false,
                        finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
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
                let (final_stage3_candidate, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_boundaries,
                    None,
                    &accepted_snow_soil_receipts,
                    &final_precipitation_sets,
                )?;
                profile_record("finalization candidate", finalization_candidate_started);
                if !covered_fixed_point_boundaries_equal(
                    &final_input_boundaries,
                    &final_rebuilt_boundaries,
                ) || !covered_fixed_point_lse_states_equal(&lse_states, &final_lse_states)
                    || !covered_fixed_point_stage3_states_equal(
                        &stage3_candidate,
                        &final_stage3_candidate,
                    )
                {
                    if iteration + 1 == COVERED_FIXED_POINT_POLICY.max_iterations {
                        crate::snow_stage3_v11_attachment::record_covered_fixed_point_limit_detail_v1(
                            crate::snow_stage3_v11_attachment::CoveredFixedPointLimitDetailV1 {
                            stage: crate::snow_stage3_v11_attachment::CoveredFixedPointLimitStageV1::Finalization,
                            lse_converged: covered_fixed_point_lse_states_equal(
                                &lse_states,
                                &final_lse_states,
                            ),
                            stage3_converged: covered_fixed_point_stage3_states_equal(
                                &stage3_candidate,
                                &final_stage3_candidate,
                            ),
                            soil_converged: true,
                            boundary_converged: covered_fixed_point_boundaries_equal(
                                &final_input_boundaries,
                                &final_rebuilt_boundaries,
                            ),
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
                    iteration_stage3_states = final_stage3_candidate;
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
                let sealed_source_envelope =
                    self.build_covered_carrier_envelope_value_v1(CoveredCarrierEnvelopeBuildV1 {
                        candidate: &final_candidate,
                        interval_s,
                        duration_s_bits: input.duration_s_bits,
                        covered_destinations: &covered_destinations,
                        covered_boundaries: &sealed_source_input_boundaries,
                        open_boundaries: &sealed_source_open_boundaries,
                        provisional: false,
                        finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
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
                let final_envelope =
                    self.build_covered_carrier_envelope_value_v1(CoveredCarrierEnvelopeBuildV1 {
                        candidate: &final_candidate,
                        interval_s,
                        duration_s_bits: input.duration_s_bits,
                        covered_destinations: &covered_destinations,
                        covered_boundaries: &final_covered_lower_boundaries,
                        open_boundaries: &final_open_lower_boundaries,
                        provisional: false,
                        finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
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
                let (final_ending_stage3, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_lower_boundaries,
                    Some(&final_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
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
                let installed_envelope =
                    self.build_covered_carrier_envelope_value_v1(CoveredCarrierEnvelopeBuildV1 {
                        candidate: &final_candidate,
                        interval_s,
                        duration_s_bits: input.duration_s_bits,
                        covered_destinations: &covered_destinations,
                        covered_boundaries: &installed_covered_lower_boundaries,
                        open_boundaries: &installed_open_lower_boundaries,
                        provisional: false,
                        finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
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
                let (installed_stage3, installed_cold_content_export_by_lane) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_complete_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    &installed_precipitation_sets,
                )?;
                if installed_stage3 != final_ending_stage3 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed Stage-3 replay exact physical identity",
                    ));
                }
                let installed_soil = aggregate_soil_thermal_ending_with_top_boundary_credits(
                    &self.beginning.inner.soil_thermal,
                    &self.beginning.inner.lse_configuration,
                    installed_envelope.transaction_id(),
                    installed_envelope.hydrology().soil_thermal_candidates(),
                    &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?
                .ending;
                let installed_snow_soil_receipts = self
                    .retain_terminal_limiting_snow_soil_receipts(
                        self.snow_soil_heat_receipts(
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
                    &installed_soil,
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
                        covered_fixed_point_soil_underrelaxed_iterate_v1(
                            &iteration_soil_state,
                            &installed_soil,
                            weight,
                        )
                    });
                    phase_record("finalization", finalization_started);
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
                    &installed_soil,
                )?;
                let (identity_replayed_stage3, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_complete_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    &installed_precipitation_sets,
                )?;
                let identity_replayed_soil =
                    aggregate_soil_thermal_ending_with_top_boundary_credits(
                        &self.beginning.inner.soil_thermal,
                        &self.beginning.inner.lse_configuration,
                        installed_envelope.transaction_id(),
                        installed_envelope.hydrology().soil_thermal_candidates(),
                        &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?
                    .ending;
                profile_record("finalization identity replay", identity_replay_started);
                if identity_replayed_stage3 != installed_stage3
                    || identity_replayed_soil != installed_soil
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "snow-soil identity-only receipt reseal replay",
                    ));
                }
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
        }?;
        self.validate_snow_soil_heat_receipt_iterate_joins(
            &accepted_snow_soil_receipts,
            &ending_stage3,
            &installed_soil_preview,
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
            ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
            &physical_outcome_ledgers,
            AcceptedPublicationFinalizationPostureV1::RetainFinal,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(final_boundary_receipts);
        self.last_lane_boundary_receipts = Some(final_lane_boundary_receipts);
        self.last_component_carrier_receipts = Some(final_component_carrier_receipts);
        self.last_snow_soil_heat_receipts = Some(accepted_snow_soil_receipts);
        self.last_precipitation_parcel_sets = Some(installed_precipitation_sets);
        self.last_physical_outcome_ledgers = Some(physical_outcome_ledgers);
        self.last_terminal_snow_soil_heat_receipts = Some(terminal_snow_soil_heat_receipts);
        self.last_adaptive_terminal_snow_soil_trial_receipts = Some(BTreeMap::new());
        self.last_terminal_events = Some(terminal_events.into_inner());
        self.ending_stage3_by_lane = Some(ending_stage3);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        self.ordinary_physical_reuse_seed = Some(CoveredOrdinaryPhysicalReuseSeedV1 {
            physical_authority: covered_ordinary_physical_authority_v1(input)?,
            envelope,
        });
        Ok(output)
    }
}

#[cfg(test)]
include!("open_snow_tail_tests.rs");
