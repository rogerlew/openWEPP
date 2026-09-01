impl DirectV11SnowCoveredRealConsumerStack<'_> {
    fn snow_soil_heat_receipts_v2(
        &self,
        support: TimeSupport,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &DirectSoilThermalCandidate,
    ) -> Result<BTreeMap<u32, SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        self.snow_soil_heat_receipts_for_read_view_v2(support, trial_stage3, trial_soil.read_view())
    }

    fn snow_soil_heat_receipts_for_read_view_v2(
        &self,
        support: TimeSupport,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        ending_soil: DirectSoilThermalReadView<'_>,
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
            digest32_from_lower_hex(self.beginning.inner.soil_thermal.state_sha256().as_str())?;
        let beginning_soil = self.beginning.inner.soil_thermal.read_view();
        let mut receipts = BTreeMap::new();
        for (lane_id, ofe_id) in lane_to_ofe {
            let inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("V2 snow-soil Stage-3 inputs"),
            )?;
            let beginning_stage = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("V2 snow-soil beginning snow owner"),
            )?;
            let ending_stage =
                trial_stage3
                    .get(&lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V2 snow-soil trial snow owner",
                    ))?;
            let beginning_terminal =
                crate::hydrology::stage3_is_terminal_event_domain(beginning_stage);
            if !crate::hydrology::stage3_is_resolved_thermal_domain(beginning_stage)
                && !beginning_terminal
            {
                continue;
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
                } else {
                    continue;
                };
            let configured_ofe = self
                .beginning
                .inner
                .lse_configuration
                .ofes
                .iter()
                .find(|value| value.ofe_id == ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V2 snow-soil configured OFE",
                ))?;
            let configured_top = configured_ofe.soil_interface_layers.first().ok_or(
                DirectV11RealConsumerError::Identity("V2 snow-soil configured top layer"),
            )?;
            let beginning_ofe = beginning_soil
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V2 snow-soil beginning OFE",
                ))?;
            let ending_ofe = ending_soil
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V2 snow-soil ending OFE",
                ))?;
            let beginning_top = beginning_ofe.ordered_layers().into_iter().next().ok_or(
                DirectV11RealConsumerError::Identity("V2 snow-soil beginning top"),
            )?;
            let ending_top = ending_ofe.ordered_layers().into_iter().next().ok_or(
                DirectV11RealConsumerError::Identity("V2 snow-soil ending top"),
            )?;
            if beginning_top.layer_id() != &configured_top.layer_id
                || ending_top.layer_id() != &configured_top.layer_id
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "V2 snow-soil top-layer identity",
                ));
            }
            let (beginning_heat, ending_heat, accepted_heat) =
                crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
                    0.5 * beginning_bottom.thickness_m,
                    beginning_bottom.thermal_conductivity_w_m_k,
                    0.5 * configured_top.thickness_m,
                    configured_top.thermal_conductivity_w_m_k,
                    beginning_bottom.temperature_k,
                    beginning_top.temperature_k(),
                    ending_bottom_temperature_k,
                    ending_top.temperature_k(),
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                })?;
            let accepted_j = accepted_heat * f64::from_bits(support.duration_s_bits());
            let ending_snow_sha256 =
                digest_bytes(&serde_json::to_vec(ending_stage).map_err(|_| {
                    DirectV11RealConsumerError::Identity("V2 snow-soil trial snow seal")
                })?);
            let ending_soil_sha256 = digest_bytes(
                &match ending_ofe {
                    DirectSoilThermalOfeReadView::V1(value) => serde_json::to_vec(value),
                    DirectSoilThermalOfeReadView::V2(value) => serde_json::to_vec(value),
                }
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("V2 snow-soil trial soil seal")
                })?,
            );
            let receipt = SnowSoilHeatReceiptV1 {
                schema_version: 1,
                model_identity_sha256: digest_bytes(b"SC-SNOWENERGY-001@18-SNOW-SOIL-CN-V1"),
                support,
                support_duration_ns: support.duration_ns(),
                lane_id,
                ofe_id: ofe_id.clone(),
                ofe_ground_basis: true,
                topology_identity_sha256,
                configuration_identity_sha256,
                beginning_snow_owner_identity_sha256: beginning_bottom
                    .beginning_stage3_state_sha256,
                beginning_soil_owner_identity_sha256,
                bottom_snow_layer_id: u32::try_from(beginning_stage.layers.len().saturating_sub(1))
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("V2 snow bottom layer ordinal")
                    })?,
                first_soil_layer_id: configured_top.layer_id.clone(),
                bottom_snow_half_thickness_m: 0.5 * beginning_bottom.thickness_m,
                bottom_snow_conductivity_w_m_k: beginning_bottom.thermal_conductivity_w_m_k,
                top_soil_half_thickness_m: 0.5 * configured_top.thickness_m,
                top_soil_conductivity_w_m_k: configured_top.thermal_conductivity_w_m_k,
                beginning_bottom_snow_temperature_k: beginning_bottom.temperature_k,
                beginning_top_soil_temperature_k: beginning_top.temperature_k(),
                ending_bottom_snow_temperature_k: ending_bottom_temperature_k,
                ending_top_soil_temperature_k: ending_top.temperature_k(),
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
            .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
            receipts.insert(lane_id, receipt);
        }
        Ok(receipts)
    }

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
        let last_phase =
            endpoint
                .carrier_phase_chain
                .last()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "precomputed terminal empty carrier chain",
                ))?;
        if last_phase.transition.boundary.support
            != endpoint.carrier_phase.transition.boundary.support
            || last_phase.transition.probe_child_identity.receipt_sha256
                != endpoint
                    .carrier_phase
                    .transition
                    .probe_child_identity
                    .receipt_sha256
            || last_phase.ending_candidates.joint().receipt_sha256()
                != endpoint
                    .carrier_phase
                    .ending_candidates
                    .joint()
                    .receipt_sha256()
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
                || chain
                    .windows(2)
                    .any(|pair| pair[0].support.end_ns() != pair[1].support.start_ns())
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
            TerminalPublicationPostureV1::RetainFinalWithDeferredNativeV2Soil {
                pre_event_authority_sha256,
            } if pre_event_authority_sha256 == endpoint.pre_event_authority_sha256 => {
                AcceptedPublicationFinalizationPostureV1::RetainFinalWithDeferredNativeV2Soil {
                    pre_event_authority_sha256,
                }
            }
            TerminalPublicationPostureV1::RetainFinalWithDeferredNativeV2Soil { .. } => {
                return Err(DirectV11RealConsumerError::Identity(
                    "terminal final deferred-soil publication authority substitution",
                ));
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
                .read_view()
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &trial.ofe_id)
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
            if !snow_soil_receipt_reseal_roundoff_within_bound_v1(heat_reseal_residual_j_m2, 0.0) {
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
                ending_soil_owner_sha256: digest_bytes(
                    &match ending_soil {
                        DirectSoilThermalOfeReadView::V1(value) => serde_json::to_vec(value),
                        DirectSoilThermalOfeReadView::V2(value) => serde_json::to_vec(value),
                    }
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "accepted terminal ending soil identity",
                        )
                    })?,
                ),
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
        let mut persistent_receipts = match endpoint.carrier_phase.soil_candidate.v1() {
            Ok(v1_candidate) => self.snow_soil_heat_receipts(
                input.support,
                &endpoint.ending_stage3_by_lane,
                v1_candidate,
            )?,
            Err(_) if endpoint.carrier_phase.soil_candidate.v2().is_ok() => self
                .snow_soil_heat_receipts_v2(
                    input.support,
                    &endpoint.ending_stage3_by_lane,
                    &endpoint.carrier_phase.soil_candidate,
                )?,
            Err(error) => {
                return Err(DirectV11RealConsumerError::Runtime(
                    DirectV10RealConsumerError::Runtime(error),
                ));
            }
        };
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
                soil_candidate: endpoint.carrier_phase.soil_candidate.clone(),
                soil_continuation: endpoint
                    .carrier_phase
                    .ending_candidates
                    .soil_continuation()
                    .cloned(),
                ending_snow_owner_bytes: ending_snow_owner_bytes.clone(),
                soil_top_boundary_credits: soil_credits.clone(),
            })
        } else {
            None
        };
        let (output, candidate, support_receipt) =
            finalize_v11_imported_segment_with_soil_continuation(
                &self.beginning,
                input,
                &endpoint.carrier_phase.carrier_envelope,
                Some(&compositional_envelopes),
                Some(endpoint.carrier_phase.ending_candidates.shadow()),
                Some(&endpoint.carrier_phase.soil_candidate),
                endpoint.carrier_phase.ending_candidates.soil_continuation(),
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
                | AcceptedPublicationFinalizationPostureV1::RetainFinalWithDeferredNativeV2Soil {
                    ..
                }
        ));
        Ok(output)
    }
}
