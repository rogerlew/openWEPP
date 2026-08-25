const OUTCOME_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

fn validate_destination_reconstruction_against_lane_aggregate(
    reconstructed: [f64; 5],
    aggregate: [f64; 5],
) -> Result<(), DirectV11RealConsumerError> {
    if reconstructed
        .iter()
        .zip(aggregate)
        .any(|(reconstructed, aggregate)| (reconstructed - aggregate).abs() > 1.0e-6)
    {
        return Err(DirectV11RealConsumerError::Identity(
            "physical ledger lane aggregate substitution",
        ));
    }
    Ok(())
}

fn reconstruct_interlayer_from_owner_states(
    lower_before: f64,
    lower_after: f64,
    reported_active: f64,
    reported_lower: f64,
) -> Result<(f64, f64), DirectV11RealConsumerError> {
    let reconstructed_lower = lower_before - lower_after;
    if (reported_lower - reconstructed_lower).abs() > 1.0e-9
        || (reported_active + reconstructed_lower).abs() > 1.0e-9
    {
        return Err(DirectV11RealConsumerError::Identity(
            "interlayer owner-state reconstruction",
        ));
    }
    Ok((-reconstructed_lower, reconstructed_lower))
}

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
    diagnostics: &'a BTreeMap<u32, (f64, f64, f64)>,
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
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
            if !crate::hydrology::stage3_is_resolved_thermal_domain(beginning)
                || !crate::hydrology::stage3_is_resolved_thermal_domain(ending)
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
            let soil = inputs
                .soil
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger soil lane",
                ))?;
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
            let aggregate_receipt = [
                lane.aggregate_sensible_to_canopy_air_w_m2,
                lane.aggregate_latent_energy_to_canopy_air_j_m2,
                lane.aggregate_snow_absorbed_shortwave_w_m2,
                lane.aggregate_snow_net_longwave_w_m2,
                lane.aggregate_vapor_to_canopy_air_kg_m2_s,
            ];
            validate_destination_reconstruction_against_lane_aggregate(
                destination_aggregate,
                aggregate_receipt,
            )?;
            let sensible = -destination_aggregate[0] * duration_s;
            let latent = -destination_aggregate[1];
            let shortwave = destination_aggregate[2] * duration_s;
            let longwave = destination_aggregate[3] * duration_s;
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
                    soil.receipt_sha256,
                    lane.receipt_sha256,
                ],
            };
            let (vapor_material_enthalpy, interlayer_active, interlayer_lower) = *inputs
                .diagnostics
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger diagnostic lane",
                ))?;
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
                precipitation_advection_j_m2: advection,
                deposition_kg_m2: deposition,
                sublimation_kg_m2: sublimation,
                vapor_transfer_kg_m2: deposition - sublimation,
                latent_heat_j_kg: lane.aggregate_latent_heat_j_kg,
                snow_surface_temperature_k: lane.aggregate_snow_temperature_k,
                vapor_material_enthalpy_j_m2: vapor_material_enthalpy,
                melt_kg_m2: melt,
                refreeze_kg_m2: refreeze,
                terminal_liquid_kg_m2: terminal,
                retained_liquid_kg_m2: ending_liquid,
                shortwave_j_m2: shortwave,
                longwave_j_m2: longwave,
                sensible_j_m2: sensible,
                latent_j_m2: latent,
                soil_heat_j_m2: soil.snow_candidate_heat_j_m2_ofe_ground,
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
            let ledger =
                Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected).map_err(|_| {
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
        if !crate::hydrology::stage3_is_resolved_thermal_domain(beginning_stage)
            || !crate::hydrology::stage3_is_resolved_thermal_domain(ending_stage)
        {
            return Ok(None);
        }
        let beginning_bottom = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
            beginning_stage,
            inputs.surface_energy_options.atmospheric_pressure_pa,
        )?;
        let ending_bottom = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
            ending_stage,
            inputs.surface_energy_options.atmospheric_pressure_pa,
        )?;
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
            ending_bottom_temperature_k: ending_bottom.temperature_k,
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
        >,
                               snow_soil_receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
                               precipitation_sets: &BTreeMap<
            u32,
            Stage3PrecipitationPhaseParcelSetV1,
        >| {
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
                let mut stage3_forcing = self.stage3_forcing_by_lane.get(lane_id).copied().ok_or(
                    DirectV11RealConsumerError::Identity("covered Stage-3 forcing lane"),
                )?;
                let precipitation_set =
                    precipitation_sets
                        .get(lane_id)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "covered precipitation parcel-set lane",
                        ))?;
                let (precipitation_mass, precipitation_advection_j_m2) =
                    reconstruct_precipitation_mass_and_advected_heat(precipitation_set).map_err(
                        |error| DirectV11RealConsumerError::from_stage3_physical_custody(&error),
                    )?;
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
                if !precipitation_mass.is_finite()
                    || !liquid_mass.is_finite()
                    || !solid_mass.is_finite()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "precipitation phase/mass same-set join",
                    ));
                }
                stage3_forcing.forcing.rain_m = liquid_mass / 1_000.0;
                stage3_forcing.forcing.snowfall_m = solid_mass / 100.0;
                stage3_forcing.forcing.active_precipitation_m =
                    stage3_forcing.forcing.rain_m + stage3_forcing.forcing.snowfall_m;
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
                        precipitation_advection_j_m2,
                        snow_soil_heat_j_m2: snow_soil_receipts
                            .get(lane_id)
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "covered lane snow-soil heat receipt",
                            ))?
                            .snow_candidate_heat_j_m2_ofe_ground,
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
                    || (evaluation.complete_arm_snow_soil_heat_j_m2 - boundary.snow_soil_heat_j_m2)
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
                        let reported_active = tuple.internal_active_lower_conduction_j_m2.ok_or(
                            DirectV11RealConsumerError::Identity("active interlayer diagnostic"),
                        )?;
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
        let mut previous_stage3_states: Option<BTreeMap<u32, DirectSnowStage3PersistentState>> =
            None;
        let mut previous_soil_state: Option<SoilThermalSnapshot> = None;
        let mut iteration_soil_state = self.beginning.inner.soil_thermal.clone();
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
        ) = 'fixed_point: {
            for _iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
                accepted_snow_soil_receipts = self.snow_soil_heat_receipts(
                    input.support,
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
                        self.wb14_coupled_child_binding,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let provisional_precipitation_sets =
                    self.precipitation_parcel_sets(input.support, &provisional_envelope)?;
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
                let (stage3_candidate, _) = evaluate_stage3(
                    &destination_receipts,
                    &next_boundaries,
                    None,
                    &accepted_snow_soil_receipts,
                    &provisional_precipitation_sets,
                )?;
                let soil_credits = self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?;
                let soil_candidate = aggregate_soil_thermal_ending_with_top_boundary_credits(
                    &self.beginning.inner.soil_thermal,
                    &self.beginning.inner.lse_configuration,
                    provisional_envelope.transaction_id(),
                    provisional_envelope.hydrology().soil_thermal_candidates(),
                    &soil_credits,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?
                .ending;
                let lse_converged = previous_lse_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_lse_states_equal(previous, &lse_states)
                });
                let stage3_converged = previous_stage3_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_stage3_states_equal(previous, &stage3_candidate)
                });
                let soil_converged = previous_soil_state.as_ref().is_some_and(|previous| {
                    covered_fixed_point_soil_states_equal(previous, &soil_candidate)
                });
                let boundary_converged =
                    previous_complete_boundaries
                        .as_ref()
                        .is_some_and(|previous| {
                            covered_fixed_point_boundaries_equal(previous, &next_boundaries)
                        });
                let converged =
                    lse_converged && stage3_converged && soil_converged && boundary_converged;
                if !converged {
                    previous_lse_states = Some(lse_states);
                    previous_stage3_states = Some(stage3_candidate.clone());
                    iteration_stage3_states = stage3_candidate;
                    previous_soil_state = Some(soil_candidate.clone());
                    iteration_soil_state = soil_candidate;
                    iteration_boundaries = Some(next_covered_boundaries);
                    previous_complete_boundaries = Some(next_boundaries);
                    continue;
                }

                // Re-seal from the converged candidate endpoints. These are
                // the identities retained by the parent join and replayed for
                // exact installation; the preceding receipt was only the
                // fixed-point operand generated from the prior trial.
                accepted_snow_soil_receipts = self.snow_soil_heat_receipts(
                    input.support,
                    &stage3_candidate,
                    &soil_candidate,
                )?;

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
                        self.wb14_coupled_child_binding,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
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
                    previous_soil_state = Some(soil_candidate.clone());
                    iteration_soil_state = soil_candidate;
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
                        self.wb14_coupled_child_binding,
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
                let final_lane_boundary_receipts = self.final_lane_boundary_receipts(
                    input,
                    &final_boundary_receipts,
                    &final_precipitation_sets,
                )?;
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
                        self.wb14_coupled_child_binding,
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
                let (final_ending_stage3, _) = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_complete_lower_boundaries,
                    Some(&final_lane_boundary_receipts),
                    &accepted_snow_soil_receipts,
                    &final_precipitation_sets,
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
                let installed_lane_boundary_receipts = self.final_lane_boundary_receipts(
                    input,
                    &installed_boundary_receipts,
                    &final_precipitation_sets,
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
                        self.wb14_coupled_child_binding,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let installed_precipitation_sets =
                    self.precipitation_parcel_sets(input.support, &installed_envelope)?;
                if installed_precipitation_sets
                    .iter()
                    .map(|(lane, set)| (*lane, set.receipt_sha256))
                    .collect::<BTreeMap<_, _>>()
                    != final_precipitation_sets
                        .iter()
                        .map(|(lane, set)| (*lane, set.receipt_sha256))
                        .collect::<BTreeMap<_, _>>()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "installed precipitation parcel-set substitution",
                    ));
                }
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
                ));
            }
            Err(DirectV11RealConsumerError::CoveredBoundary(
                SnowStage3HandoffError::FixedPointIterationLimit,
            ))
        }?;
        let installed_soil_preview = aggregate_soil_thermal_ending_with_top_boundary_credits(
            &self.beginning.inner.soil_thermal,
            &self.beginning.inner.lse_configuration,
            envelope.transaction_id(),
            envelope.hydrology().soil_thermal_candidates(),
            &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?
        .ending;
        for (lane_id, receipt) in &mut accepted_snow_soil_receipts {
            let installed_snow =
                ending_stage3
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "snow-soil installed lane reseal",
                    ))?;
            let installed_soil_ofe = installed_soil_preview
                .ofes
                .iter()
                .find(|value| value.ofe_id == receipt.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil installed OFE reseal",
                ))?;
            let installed_top = installed_soil_ofe.ordered_layers.first().ok_or(
                DirectV11RealConsumerError::Identity("snow-soil installed top-node reseal"),
            )?;
            if installed_top.layer_id != receipt.first_soil_layer_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-soil installed typed node reseal",
                ));
            }
            receipt.snow_candidate_ending_identity_sha256 =
                digest_bytes(&serde_json::to_vec(installed_snow).map_err(|_| {
                    DirectV11RealConsumerError::Identity("snow-soil installed snow identity")
                })?);
            receipt.soil_candidate_ending_identity_sha256 =
                digest_bytes(&serde_json::to_vec(installed_soil_ofe).map_err(|_| {
                    DirectV11RealConsumerError::Identity("snow-soil installed soil identity")
                })?);
            *receipt = receipt.clone().seal().map_err(|error| {
                DirectV11RealConsumerError::from_stage3_physical_custody(&error)
            })?;
        }
        let ending_snow_owner_bytes = canonical_stage3_snow_owner_bytes_v11_with_receipts(
            &ending_stage3,
            &final_lane_boundary_receipts,
            &final_boundary_receipts,
        )?;
        let physical_outcome_ledgers =
            self.physical_outcome_ledgers(&PhysicalOutcomeLedgerInputs {
                support: input.support,
                ending: &ending_stage3,
                lanes: &final_lane_boundary_receipts,
                destinations: &final_boundary_receipts,
                precipitation: &installed_precipitation_sets,
                soil: &accepted_snow_soil_receipts,
                diagnostics: &installed_cold_content_export_by_lane,
            })?;
        self.last_wb14_child_receipt_set_sha256 = Some(
            envelope
                .hydrology()
                .surface_ingress()
                .wb14_child_receipt_set_sha256()
                .to_string(),
        );
        self.last_wb14_parent_receipt_set_sha256 = envelope
            .hydrology()
            .surface_ingress()
            .wb14_parent_receipt_set_sha256()
            .map(ToString::to_string);
        self.last_wb14_child_replay_bytes = Some(
            envelope
                .hydrology()
                .surface_ingress()
                .wb14_child_replay_bytes()
                .to_vec(),
        );
        self.last_wb14_parent_replay_bytes = envelope
            .hydrology()
            .surface_ingress()
            .wb14_parent_replay_bytes()
            .map(ToOwned::to_owned);
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &candidate,
            input,
            &envelope,
            ending_snow_owner_bytes,
            self.day_index,
            &self.soil_top_boundary_credits(&accepted_snow_soil_receipts)?,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(final_boundary_receipts);
        self.last_lane_boundary_receipts = Some(final_lane_boundary_receipts);
        self.last_component_carrier_receipts = Some(final_component_carrier_receipts);
        self.last_snow_soil_heat_receipts = Some(accepted_snow_soil_receipts);
        self.last_precipitation_parcel_sets = Some(installed_precipitation_sets);
        self.last_physical_outcome_ledgers = Some(physical_outcome_ledgers);
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

    #[test]
    fn coherently_resealed_lane_aggregate_cannot_replace_destination_fold() {
        let reconstructed = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(
            validate_destination_reconstruction_against_lane_aggregate(
                reconstructed,
                reconstructed,
            )
            .is_ok()
        );

        for index in 0..reconstructed.len() {
            let mut substituted = reconstructed;
            substituted[index] += 2.0e-6;
            assert!(
                validate_destination_reconstruction_against_lane_aggregate(
                    reconstructed,
                    substituted,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn coherent_interlayer_pair_substitution_fails_owner_state_reconstruction() {
        let accepted = reconstruct_interlayer_from_owner_states(40.0, 42.0, 2.0, -2.0)
            .expect("owner-state reconstructed interlayer transfer");
        assert_eq!(accepted, (2.0, -2.0));

        assert!(reconstruct_interlayer_from_owner_states(40.0, 42.0, 3.0, -3.0,).is_err());
    }

    #[test]
    fn lower_layer_refreeze_after_conduction_does_not_change_internal_transfer() {
        // A later lower-layer refreeze/repartition can change the substep's
        // final cold-content partition. The bound snapshots deliberately
        // bracket conduction itself, so the accepted transfer remains the
        // material delta across that operation only.
        assert_eq!(
            reconstruct_interlayer_from_owner_states(40.0, 42.0, 2.0, -2.0)
                .expect("pre/post-conduction lower material snapshots"),
            (2.0, -2.0),
        );
    }
}
