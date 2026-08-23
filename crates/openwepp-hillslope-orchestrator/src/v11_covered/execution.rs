/// One explicit default-off invocation of the actual `DirectV10` owner stack.
#[derive(Clone)]
pub struct DirectV11RealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV9ShadowIntervalInput,
    pub day_index: usize,
    pub interval_index: usize,
    pub(super) ending: Option<DirectV10RealConsumerShadow>,
    pub(super) last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    pub(super) ending_snow_owner_bytes: Option<Vec<u8>>,
}

/// Explicit covered lower-boundary adopter for the V11 imported transaction.
///
/// This type is intentionally separate from [`DirectV11RealConsumerStack`].
/// It evaluates the Child-2C carrier and the actual persistent Stage-3
/// transition from the same beginning states and support before it constructs
/// the V11 canopy/soil owner candidate.
#[derive(Clone)]
pub struct DirectV11SnowCoveredRealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub snow_surface_forcing_by_destination:
        &'a BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub day_index: usize,
    pub interval_index: usize,
    ending: Option<DirectV10RealConsumerShadow>,
    ending_stage3_by_lane: Option<BTreeMap<u32, DirectSnowStage3PersistentState>>,
    last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    last_final_boundary_receipts:
        Option<BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>>,
    last_lane_boundary_receipts: Option<BTreeMap<u32, LaneStage3BoundaryReceiptV1>>,
    last_component_carrier_receipts:
        Option<BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>>,
}

pub struct DirectV11SnowCoveredStackInputs<'a> {
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub snow_surface_forcing_by_destination:
        &'a BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub day_index: usize,
    pub interval_index: usize,
}

impl<'a> DirectV11SnowCoveredRealConsumerStack<'a> {
    #[must_use]
    pub fn new(
        beginning: &DirectV10RealConsumerShadow,
        inputs: DirectV11SnowCoveredStackInputs<'a>,
    ) -> Self {
        Self {
            beginning: beginning.clone(),
            interval: inputs.interval,
            stage3_inputs_by_lane: inputs.stage3_inputs_by_lane,
            stage3_forcing_by_lane: inputs.stage3_forcing_by_lane,
            snow_surface_forcing_by_destination: inputs.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: inputs.stage3_beginning_by_lane,
            day_index: inputs.day_index,
            interval_index: inputs.interval_index,
            ending: None,
            ending_stage3_by_lane: None,
            last_support_receipt: None,
            last_final_boundary_receipts: None,
            last_lane_boundary_receipts: None,
            last_component_carrier_receipts: None,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn derive_live_carrier_input(
        &self,
        lane_id: u32,
        stage3_state: &DirectSnowStage3PersistentState,
        vegetation_state: &V8CoupledOwnedState,
        _stage3_forcing: DirectSnowStage3SupportInput,
        sealed: &SealedCoveredCarrierForcing,
        tile_override: Option<&TileId>,
        _interval_s: f64,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let lane_index = self
            .stage3_beginning_by_lane
            .keys()
            .position(|value| *value == lane_id)
            .ok_or(DirectV11RealConsumerError::Identity("covered lane order"))?;
        let tile = match tile_override {
            Some(tile_id) => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .iter()
                .find(|tile| tile.tile_id == *tile_id)
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
            None => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .get(lane_index % self.beginning.vegetation_configuration.topology_tiles.len())
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
        };
        let tile_air = vegetation_state.tile_canopy_air.get(&tile.tile_id).ok_or(
            DirectV11RealConsumerError::Identity("committed canopy-air owner"),
        )?;
        let occupancies = self
            .beginning
            .vegetation_configuration
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.iter().any(|id| id == &tile.tile_id))
            .filter_map(|stratum| {
                let identity = openwepp_kernel_contract::OccupancyId {
                    stratum_id: stratum.stratum_id.clone(),
                    tile_id: tile.tile_id.clone(),
                };
                vegetation_state
                    .occupancies
                    .get(&identity)
                    .map(|state| (stratum, state))
            })
            .collect::<Vec<_>>();
        if occupancies.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered canopy owner topology",
            ));
        }
        let count = f64::from(
            u32::try_from(occupancies.len())
                .map_err(|_| DirectV11RealConsumerError::Identity("covered occupancy count"))?,
        );
        let leaf_temperature_k = occupancies
            .iter()
            .map(|(_, state)| {
                state
                    .sun_leaf_temperature_k
                    .midpoint(state.shade_leaf_temperature_k)
            })
            .sum::<f64>()
            / count;
        let stem_temperature_k = occupancies
            .iter()
            .map(|(_, state)| state.dry_stem_temperature_k)
            .sum::<f64>()
            / count;
        let canopy_wind = sealed.exposure.wind_m_s;
        let (canopy_heat, canopy_vapor) = occupancies.iter().try_fold(
            (0.0, 0.0),
            |(heat, vapor), (stratum, _)| -> Result<(f64, f64), DirectV11RealConsumerError> {
                let u_star = canopy_surface_friction_velocity(
                    canopy_wind,
                    self.interval.vegetation_forcing.reference_height_m,
                    stratum.displacement_m,
                    stratum.z0m_m,
                )
                .map_err(|_| DirectV11RealConsumerError::Identity("canopy wind exposure"))?;
                let leaf = leaf_boundary_conductance(u_star, stratum.leaf_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("leaf conductance"))?;
                let wet = leaf_boundary_conductance(u_star, stratum.wet_surface_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("wet conductance"))?;
                let stem = leaf_boundary_conductance(u_star, stratum.stem_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("stem conductance"))?;
                Ok((heat + (leaf + wet + stem) / 3.0, vapor + leaf.midpoint(wet)))
            },
        )?;
        let canopy_heat = canopy_heat / count;
        let canopy_vapor = canopy_vapor / count;
        let reference_resistance = neutral_resistance(
            sealed.exposure.transfer_height_m,
            0.0,
            sealed.exposure.roughness_m,
            sealed.exposure.roughness_m,
            sealed.exposure.wind_m_s,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("reference exposure"))?;
        let snow_resistance = reference_resistance;
        let snow_conductance = 1.0 / snow_resistance;
        let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(stage3_state)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow active-volume surface"))?;
        let snow_temperature_k = surface.surface_temperature_k;
        let snow_temperature = TemperatureCelsius::try_new(snow_temperature_k - 273.15)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow temperature"))?;
        let saturation_pressure_pa = kilopascals_to_pascals(
            saturation_vapor_pressure_ice_kpa(snow_temperature)
                .map_err(|_| DirectV11RealConsumerError::Identity("snow saturation pressure"))?
                .as_kilopascals(),
        );
        let air_pressure_pa = self.interval.lse_forcing.air_pressure_pa;
        if !air_pressure_pa.is_finite() || air_pressure_pa <= 0.378 * saturation_pressure_pa {
            return Err(DirectV11RealConsumerError::Identity(
                "snow surface humidity pressure",
            ));
        }
        let snow_humidity = (0.622 * saturation_pressure_pa
            / (air_pressure_pa - 0.378 * saturation_pressure_pa))
            .min(1.0);
        let reference_heat = 1.0 / reference_resistance;
        let reference = CarrierSurface {
            temperature_k: sealed.reference_temperature_k,
            specific_humidity: sealed.reference_specific_humidity,
            heat_conductance_m_s: reference_heat,
            vapor_conductance_m_s: reference_heat,
        };
        let canopy = CarrierSurface {
            temperature_k: tile_air.canopy_air_temperature_k,
            specific_humidity: tile_air.canopy_air_specific_humidity_kg_kg,
            heat_conductance_m_s: canopy_heat,
            vapor_conductance_m_s: canopy_vapor,
        };
        let snow = CarrierSurface {
            temperature_k: snow_temperature_k,
            specific_humidity: snow_humidity,
            heat_conductance_m_s: snow_conductance,
            vapor_conductance_m_s: snow_conductance,
        };
        let weight_sum = leaf_temperature_k + stem_temperature_k;
        let components = vec![
            CanopyLongwaveComponent {
                temperature_k: leaf_temperature_k,
                emissive_area_weight: leaf_temperature_k / weight_sum,
            },
            CanopyLongwaveComponent {
                temperature_k: stem_temperature_k,
                emissive_area_weight: stem_temperature_k / weight_sum,
            },
        ];
        let heat_total = reference.heat_conductance_m_s
            + canopy.heat_conductance_m_s
            + snow.heat_conductance_m_s;
        let vapor_total = reference.vapor_conductance_m_s
            + canopy.vapor_conductance_m_s
            + snow.vapor_conductance_m_s;
        let shared_temperature = (reference.heat_conductance_m_s * reference.temperature_k
            + canopy.heat_conductance_m_s * canopy.temperature_k
            + snow.heat_conductance_m_s * snow.temperature_k)
            / heat_total;
        let shared_humidity = (reference.vapor_conductance_m_s * reference.specific_humidity
            + canopy.vapor_conductance_m_s * canopy.specific_humidity
            + snow.vapor_conductance_m_s * snow.specific_humidity)
            / vapor_total;
        let snow_sensible = -sealed.rho_air_kg_m3
            * sealed.cp_air_j_kg_k
            * snow.heat_conductance_m_s
            * (snow.temperature_k - shared_temperature);
        let snow_vapor = -sealed.rho_air_kg_m3
            * snow.vapor_conductance_m_s
            * (snow.specific_humidity - shared_humidity);
        let sky_view = (1.0 - sealed.effective_canopy_cover).powf(1.6);
        let canopy_longwave = components
            .iter()
            .map(|component| {
                component.emissive_area_weight * 5.670_374_419e-8 * component.temperature_k.powi(4)
            })
            .sum::<f64>();
        let snow_emission = 5.670_374_419e-8 * snow.temperature_k.powi(4);
        let snow_longwave_net_w_m2 = sky_view * sealed.atmospheric_longwave_w_m2
            + (1.0 - sky_view) * canopy_longwave
            - snow_emission;
        let scalar_bytes = [
            shared_temperature,
            shared_humidity,
            snow_temperature_k,
            snow_sensible,
            snow_vapor,
            snow_longwave_net_w_m2,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        let diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-v1",
            &[openwepp_coupled_time::FramedField {
                tag: "numerical_guess",
                value: &scalar_bytes,
            }],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier initial guess"))?;
        Ok(CoveredCarrierInitialGuessV1 {
            snow_temperature_k,
            snow_sensible_into_surface_w_m2: -snow_sensible,
            snow_vapor_into_surface_kg_m2_s: -snow_vapor,
            snow_longwave_net_w_m2,
            diagnostic_sha256,
        })
    }

    fn lane_stage3_terms_from_boundaries(
        &self,
        destination_receipts: &BTreeMap<(OfeId, TileId), Digest32>,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        interval_s: f64,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryTerms>, DirectV11RealConsumerError> {
        let mut lanes = BTreeMap::<u32, LaneStage3BoundaryTerms>::new();
        for (destination, carrier) in destination_receipts {
            let boundary =
                boundaries
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane boundary destination",
                    ))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane boundary OFE",
                ))?;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = lanes.entry(binding.production_lane_id).or_insert_with(|| {
                LaneStage3BoundaryTerms {
                    fractions: 0.0,
                    provisional_carrier_bytes: Vec::new(),
                    provisional_carrier_receipt_sha256: Digest32::zero(),
                    sensible_to_canopy_air_w_m2: 0.0,
                    vapor_to_canopy_air_kg_m2_s: 0.0,
                    latent_energy_to_canopy_air_j_m2: 0.0,
                    snow_absorbed_shortwave_w_m2: 0.0,
                    snow_net_longwave_w_m2: 0.0,
                    snow_temperature_k: 0.0,
                    latent_heat_j_kg: 0.0,
                    common_snow_temperature_k: None,
                    common_latent_heat_j_kg: None,
                }
            });
            if entry
                .common_snow_temperature_k
                .is_some_and(|value| value.to_bits() != boundary.snow_temperature_k.to_bits())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane common snow temperature",
                ));
            }
            if entry
                .common_latent_heat_j_kg
                .is_some_and(|value| value.to_bits() != boundary.latent_heat_j_kg.to_bits())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane common latent heat",
                ));
            }
            entry.common_snow_temperature_k = Some(boundary.snow_temperature_k);
            entry.common_latent_heat_j_kg = Some(boundary.latent_heat_j_kg);
            entry.fractions += fraction;
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.0.as_str().as_bytes());
            entry.provisional_carrier_bytes.push(0);
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.1.as_str().as_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(&fraction.to_bits().to_le_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(carrier.as_bytes());
            entry.sensible_to_canopy_air_w_m2 += fraction * boundary.sensible_to_canopy_air_w_m2;
            entry.vapor_to_canopy_air_kg_m2_s += fraction * boundary.vapor_to_canopy_air_kg_m2_s;
            entry.latent_energy_to_canopy_air_j_m2 += fraction
                * boundary.vapor_to_canopy_air_kg_m2_s
                * boundary.latent_heat_j_kg
                * interval_s;
            entry.snow_absorbed_shortwave_w_m2 += fraction * boundary.shortwave_absorbed_w_m2;
            entry.snow_net_longwave_w_m2 += fraction * boundary.net_longwave_w_m2;
            entry.snow_temperature_k += fraction * boundary.snow_temperature_k;
            entry.latent_heat_j_kg += fraction * boundary.latent_heat_j_kg;
        }
        for terms in lanes.values_mut() {
            if !terms.fractions.is_finite()
                || (terms.fractions - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane is missing a snow-surface contribution",
                ));
            }
            terms.provisional_carrier_receipt_sha256 =
                digest_bytes(&terms.provisional_carrier_bytes);
            terms.snow_temperature_k =
                terms
                    .common_snow_temperature_k
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane snow temperature",
                    ))?;
            terms.latent_heat_j_kg =
                terms
                    .common_latent_heat_j_kg
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane latent heat",
                    ))?;
            terms.latent_energy_to_canopy_air_j_m2 =
                (terms.vapor_to_canopy_air_kg_m2_s * interval_s) * terms.latent_heat_j_kg;
        }
        Ok(lanes)
    }

    fn final_lane_boundary_receipts(
        &self,
        input: &V11ImportedV10SegmentInput,
        final_receipts: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryReceiptV1>, DirectV11RealConsumerError> {
        let topology_configuration_sha256 = self.covered_topology_digest();
        let mut grouped =
            BTreeMap::<u32, Vec<((OfeId, TileId), f64, &FinalStage3TileBoundaryReceiptV1)>>::new();
        for (destination, receipt) in final_receipts {
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final lane boundary OFE",
                ))?
                .production_lane_id;
            grouped.entry(lane_id).or_default().push((
                destination.clone(),
                self.covered_destination_fraction(&destination.0, &destination.1)?,
                receipt,
            ));
        }
        grouped
            .into_iter()
            .map(|(lane_id, mut values)| {
                values.sort_by(|left, right| left.0.cmp(&right.0));
                let ofe_id = values.first().map(|value| value.0.0.clone()).ok_or(
                    DirectV11RealConsumerError::Identity("empty covered final lane boundary"),
                )?;
                let fraction_sum = values.iter().map(|value| value.1).sum::<f64>();
                if !fraction_sum.is_finite()
                    || (fraction_sum - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered final lane boundary is missing a snow-surface contribution",
                    ));
                }
                let mut contributions = Vec::with_capacity(values.len());
                let mut expected_topology = Vec::with_capacity(values.len());
                let mut aggregate = [0.0; 7];
                for (destination, fraction, receipt) in values {
                    if !fraction.is_finite() || fraction <= 0.0 {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered final lane boundary fraction",
                        ));
                    }
                    receipt.validate()?;
                    let (boundary_class, model_definition) = match receipt {
                        FinalStage3TileBoundaryReceiptV1::V11Canopy(_) => (
                            Stage3TileBoundaryClassV1::V11CanopyCovered,
                            digest_bytes(b"OPENWEPP_FINAL_STAGE3_CANOPY_BOUNDARY_V1"),
                        ),
                        FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => (
                            Stage3TileBoundaryClassV1::OpenSnow,
                            digest_bytes(b"OPENWEPP_FINAL_STAGE3_OPEN_SNOW_BOUNDARY_V1"),
                        ),
                    };
                    expected_topology.push(LaneBoundaryTopologyExpectationV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction_bits: fraction.to_bits(),
                        boundary_class,
                        boundary_model_definition_sha256: model_definition,
                    });
                    let sources = receipt.source_digests();
                    let physical = receipt.physical_operands();
                    let contribution = LaneBoundaryContributionV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction: fraction,
                        boundary_class,
                        boundary_model_definition_sha256: model_definition,
                        beginning_stage3_state_sha256: receipt.beginning_stage3_state_sha256(),
                        provisional_carrier_receipt_sha256: sources.0,
                        optical_receipt_sha256: sources.1,
                        reciprocal_longwave_receipt_sha256: sources.2,
                        final_boundary_receipt_sha256: sources.3,
                        sensible_to_canopy_air_w_m2: physical[0],
                        vapor_to_canopy_air_kg_m2_s: physical[1],
                        latent_energy_to_canopy_air_j_m2: physical[2],
                        snow_absorbed_shortwave_w_m2: physical[3],
                        snow_net_longwave_w_m2: physical[4],
                        snow_temperature_k: physical[5],
                        latent_heat_j_kg: physical[6],
                    };
                    for (index, value) in [
                        contribution.sensible_to_canopy_air_w_m2,
                        contribution.vapor_to_canopy_air_kg_m2_s,
                        contribution.latent_energy_to_canopy_air_j_m2,
                        contribution.snow_absorbed_shortwave_w_m2,
                        contribution.snow_net_longwave_w_m2,
                        contribution.snow_temperature_k,
                        contribution.latent_heat_j_kg,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        aggregate[index] += fraction * value;
                    }
                    contributions.push(contribution);
                }
                let common_snow_temperature_k = contributions[0].snow_temperature_k;
                let common_latent_heat_j_kg = contributions[0].latent_heat_j_kg;
                aggregate[2] = (aggregate[1] * f64::from_bits(input.support.duration_s_bits()))
                    * common_latent_heat_j_kg;
                let lane_receipt = LaneStage3BoundaryReceiptV1::try_new(
                    LaneStage3BoundaryReceiptV1 {
                        lane_id,
                        ofe_id,
                        support: input.support,
                        area_basis: Stage3LaneAreaBasisV1::OfeGround,
                        topology_configuration_sha256,
                        provisional_carrier_receipt_sha256: Digest32::zero(),
                        optical_receipt_sha256: Digest32::zero(),
                        reciprocal_longwave_receipt_sha256: Digest32::zero(),
                        final_destination_receipt_sha256: Digest32::zero(),
                        ordered_destinations: contributions,
                        aggregate_sensible_to_canopy_air_w_m2: aggregate[0],
                        aggregate_vapor_to_canopy_air_kg_m2_s: aggregate[1],
                        aggregate_latent_energy_to_canopy_air_j_m2: aggregate[2],
                        aggregate_snow_absorbed_shortwave_w_m2: aggregate[3],
                        aggregate_snow_net_longwave_w_m2: aggregate[4],
                        aggregate_snow_temperature_k: common_snow_temperature_k,
                        aggregate_latent_heat_j_kg: common_latent_heat_j_kg,
                        receipt_sha256: Digest32::zero(),
                    },
                    &expected_topology,
                )?;
                Ok((lane_id, lane_receipt))
            })
            .collect()
    }

    fn covered_topology_digest(&self) -> Digest32 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_COVERED_TILE_TOPOLOGY_V1\0");
        for record in &self.beginning.inner.surface_configuration.records {
            bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
            bytes.extend_from_slice(&record.tile_fraction.to_bits().to_le_bytes());
        }
        digest_bytes(&bytes)
    }

    fn carrier_receipts_by_destination(
        &self,
        interval_s: f64,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>, DirectV11RealConsumerError>
    {
        let surface = &self.beginning.inner.surface_configuration;
        let lane_to_ofe = self.covered_lane_to_ofe(stage3_state_by_lane)?;
        let expected_destinations = self.covered_expected_destinations();
        let configured_destinations = surface
            .records
            .iter()
            .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
            .filter(|destination| expected_destinations.contains(destination))
            .collect::<BTreeSet<_>>();
        if expected_destinations != configured_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered surface/LSE destination set",
            ));
        }

        let mut receipts = BTreeMap::new();
        for (ofe_id, tile_id) in expected_destinations {
            let binding = surface
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination OFE binding",
                ))?;
            if lane_to_ofe.get(&binding.production_lane_id) != Some(&ofe_id) {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered destination lane/OFE binding",
                ));
            }
            let carrier = self.carrier_for_destination(
                interval_s,
                binding.production_lane_id,
                &ofe_id,
                &tile_id,
                vegetation_state,
                stage3_state_by_lane,
                stage3_forcing_by_lane,
            )?;
            if receipts.insert((ofe_id, tile_id), carrier).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination carrier receipt",
                ));
            }
        }
        Ok(receipts)
    }

    fn open_snow_boundaries_by_destination(
        &self,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Digest32>,
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
        ),
        DirectV11RealConsumerError,
    > {
        let mut diagnostics = BTreeMap::new();
        let mut boundaries = BTreeMap::new();
        let mut candidates = BTreeMap::new();
        for (destination, forcing) in self.snow_surface_forcing_by_destination {
            let SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) = forcing else {
                continue;
            };
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "open-snow destination OFE binding",
                ))?
                .production_lane_id;
            let current =
                stage3_states
                    .get(&lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "open-snow current Stage-3 lane",
                    ))?;
            let beginning = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("open-snow beginning Stage-3 lane"),
            )?;
            let beginning_digest = Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning)
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "open-snow beginning active-volume surface",
                    )
                })?
                .beginning_stage3_state_sha256;
            let stage3_inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("open-snow Stage-3 inputs"),
            )?;
            let candidate = evaluate_open_snow_tile_boundary(
                current,
                beginning_digest,
                stage3_inputs,
                forcing,
            )?;
            let carrier_receipt_id = Sha256Digest::try_new(digest32_hex(
                candidate.exposure_receipt_sha256,
            ))
            .map_err(|_| DirectV11RealConsumerError::Identity("open-snow exposure receipt ID"))?;
            let albedo = stage3_inputs
                .snow_albedo_state
                .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
            let boundary = Stage3SnowCoveredLowerBoundary {
                snow_temperature_k: candidate.snow_temperature_k,
                latent_heat_j_kg: candidate.latent_heat_j_kg,
                sensible_to_canopy_air_w_m2: candidate.sensible_outward_w_m2,
                vapor_to_canopy_air_kg_m2_s: candidate.vapor_outward_kg_m2_s,
                net_longwave_w_m2: candidate.snow_net_longwave_w_m2,
                shortwave_absorbed_w_m2: candidate.snow_absorbed_shortwave_w_m2,
                precipitation_advection_w_m2: 0.0,
                carrier_receipt_id,
                snow_vis_albedo: albedo,
                snow_nir_albedo: albedo,
                stage3_albedo_state_sha256: stage3_albedo_state_digest(stage3_inputs)?,
                forcing_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                    candidate.forcing_receipt_sha256,
                ))
                .map_err(|_| DirectV11RealConsumerError::Identity("open-snow forcing receipt"))?,
                optical_receipt_sha256: Some(
                    Sha256Digest::try_new(digest32_hex(candidate.optical_receipt_sha256)).map_err(
                        |_| DirectV11RealConsumerError::Identity("open-snow optical receipt"),
                    )?,
                ),
                reciprocal_longwave_receipt_sha256: Some(
                    Sha256Digest::try_new(digest32_hex(candidate.longwave_receipt_sha256))
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity("open-snow longwave receipt")
                        })?,
                ),
                final_canopy_boundary_receipt_sha256: None,
            };
            boundary.validate().map_err(|_| {
                DirectV11RealConsumerError::Identity("open-snow lower boundary operands")
            })?;
            diagnostics.insert(destination.clone(), candidate.exposure_receipt_sha256);
            boundaries.insert(destination.clone(), boundary);
            candidates.insert(destination.clone(), candidate);
        }
        Ok((diagnostics, boundaries, candidates))
    }

    fn seal_final_open_snow_boundaries(
        &self,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), FinalStage3OpenSnowBoundaryReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        let (_, mut boundaries, candidates) =
            self.open_snow_boundaries_by_destination(stage3_states)?;
        let receipts = candidates
            .into_iter()
            .map(|(destination, candidate)| {
                let receipt = FinalStage3OpenSnowBoundaryReceiptV1::try_new(
                    candidate,
                    ending_stage3_state_sha256,
                )?;
                let boundary = boundaries.get_mut(&destination).ok_or(
                    DirectV11RealConsumerError::Identity("final open-snow lower boundary"),
                )?;
                boundary.final_canopy_boundary_receipt_sha256 = Some(
                    Sha256Digest::try_new(digest32_hex(receipt.receipt_sha256)).map_err(|_| {
                        DirectV11RealConsumerError::Identity("final open-snow boundary receipt")
                    })?,
                );
                boundary.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("sealed final open-snow lower boundary")
                })?;
                Ok((destination, receipt))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        Ok((boundaries, receipts))
    }

    fn complete_final_boundary_receipts(
        &self,
        covered: BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
        open: BTreeMap<(OfeId, TileId), FinalStage3OpenSnowBoundaryReceiptV1>,
    ) -> Result<
        BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        DirectV11RealConsumerError,
    > {
        let mut complete = covered
            .into_iter()
            .map(|(destination, receipt)| {
                (
                    destination,
                    FinalStage3TileBoundaryReceiptV1::V11Canopy(receipt),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (destination, receipt) in open {
            if complete
                .insert(
                    destination,
                    FinalStage3TileBoundaryReceiptV1::OpenSnow(receipt),
                )
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered/open final boundary intersection",
                ));
            }
        }
        Ok(complete)
    }

    fn covered_destination_fraction(
        &self,
        ofe_id: &OfeId,
        tile_id: &TileId,
    ) -> Result<f64, DirectV11RealConsumerError> {
        let record = self
            .beginning
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == *ofe_id && record.key.tile_id == *tile_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction",
            ))?;
        if !record.tile_fraction.is_finite() || record.tile_fraction <= 0.0 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction domain",
            ));
        }
        Ok(record.tile_fraction)
    }

    fn corrected_covered_boundaries_from_envelope(
        &self,
        base: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<u32, f64>,
            BTreeMap<u32, f64>,
        ),
        DirectV11RealConsumerError,
    > {
        if base.is_empty() {
            return Ok((BTreeMap::new(), BTreeMap::new(), BTreeMap::new()));
        }
        let shortwave = envelope
            .covered_snow_shortwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered optical shortwave set"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered reciprocal longwave set"))?;
        if shortwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
            || longwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered corrected boundary destination set",
            ));
        }
        let mut corrected = base.clone();
        let mut shortwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        let mut longwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        for (destination, value) in shortwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected shortwave destination",
                    ))?;
            boundary.shortwave_absorbed_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected shortwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = shortwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        for (destination, value) in longwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected longwave destination",
                    ))?;
            boundary.net_longwave_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected longwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = longwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        Ok((
            corrected,
            shortwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
            longwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
        ))
    }

    fn apply_lse_iteration_exchange(
        &self,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        states: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        if boundaries.keys().collect::<BTreeSet<_>>() != states.keys().collect::<BTreeSet<_>>() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered LSE iteration exchange destination set",
            ));
        }
        let mut next = boundaries.clone();
        for (destination, state) in states {
            let boundary =
                next.get_mut(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered LSE iteration exchange destination",
                    ))?;
            boundary.sensible_to_canopy_air_w_m2 = state.snow_sensible_w_m2;
            boundary.vapor_to_canopy_air_kg_m2_s = state.snow_vapor_kg_m2_s;
        }
        Ok(next)
    }

    fn seal_final_covered_boundaries(
        &self,
        input: &V11ImportedV10SegmentInput,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        destination_receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        ending_v8_physical_candidate_sha256: Digest32,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        if boundaries.is_empty() {
            if !destination_receipts.is_empty() {
                return Err(DirectV11RealConsumerError::Identity(
                    "open-only carrier receipt set",
                ));
            }
            return Ok((BTreeMap::new(), BTreeMap::new()));
        }
        let optical = envelope
            .covered_snow_optical_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final optical receipts"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final longwave receipts"))?;
        let beginning_v11_state_sha256 = digest32_from_lower_hex(&input.beginning.0.state_sha256)?;
        let mut final_boundaries = boundaries.clone();
        let mut final_receipts = BTreeMap::new();
        for (destination, boundary) in boundaries {
            let carrier = destination_receipts.get(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final carrier receipt join"),
            )?;
            let optical = optical
                .get(destination)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final optical receipt join",
                ))?;
            let final_longwave =
                longwave
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered final longwave receipt join",
                    ))?;
            if optical.absorbed_w_m2_tile.total().to_bits()
                != boundary.shortwave_absorbed_w_m2.to_bits()
                || final_longwave.to_bits() != boundary.net_longwave_w_m2.to_bits()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered final boundary self-reconstruction",
                ));
            }
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final boundary lane join",
                ))?
                .production_lane_id;
            let beginning_stage3 = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered final beginning Stage-3 state"),
            )?;
            let beginning_stage3_state_sha256 =
                Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning_stage3)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered final beginning active-volume surface",
                        )
                    })?
                    .beginning_stage3_state_sha256;
            let optical_receipt_sha256 = digest32_from_lower_hex(optical.receipt_sha256.as_str())?;
            let reciprocal_longwave_receipt_sha256 =
                reciprocal_longwave_receipt_digest(destination, input.support, *final_longwave);
            let final_receipt = FinalStage3CanopyBoundaryReceiptV1::try_new(
                FinalStage3CanopyBoundaryReceiptInputs {
                    support: input.support,
                    destination: destination.clone(),
                    beginning_v11_state_sha256,
                    beginning_stage3_state_sha256,
                    ending_v8_physical_candidate_sha256,
                    ending_stage3_state_sha256,
                    provisional_carrier_receipt_sha256: carrier.diagnostic_sha256,
                    optical_receipt_sha256,
                    reciprocal_longwave_receipt_sha256,
                    sensible_to_canopy_air_w_m2: boundary.sensible_to_canopy_air_w_m2,
                    vapor_to_canopy_air_kg_m2_s: boundary.vapor_to_canopy_air_kg_m2_s,
                    latent_energy_to_canopy_air_j_m2: boundary.vapor_to_canopy_air_kg_m2_s
                        * boundary.latent_heat_j_kg
                        * f64::from_bits(input.support.duration_s_bits()),
                    snow_temperature_k: boundary.snow_temperature_k,
                    latent_heat_j_kg: boundary.latent_heat_j_kg,
                    snow_absorbed_shortwave_w_m2: optical.absorbed_w_m2_tile.total(),
                    snow_net_longwave_w_m2: *final_longwave,
                },
            )?;
            let final_boundary = final_boundaries.get_mut(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final boundary storage"),
            )?;
            final_boundary.optical_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(optical_receipt_sha256)).map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered optical receipt digest")
                })?,
            );
            final_boundary.reciprocal_longwave_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(reciprocal_longwave_receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered reciprocal longwave receipt digest",
                        )
                    },
                )?,
            );
            final_boundary.final_canopy_boundary_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(final_receipt.receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered final boundary receipt digest",
                        )
                    },
                )?,
            );
            final_receipts.insert(destination.clone(), final_receipt);
        }
        Ok((final_boundaries, final_receipts))
    }

    fn covered_lane_to_ofe(
        &self,
        stage3_beginning_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<u32, OfeId>, DirectV11RealConsumerError> {
        let mut lane_to_ofe = BTreeMap::new();
        for binding in &self.beginning.inner.surface_configuration.ofe_bindings {
            if lane_to_ofe
                .insert(binding.production_lane_id, binding.ofe_id.clone())
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered lane/OFE binding",
                ));
            }
        }
        if stage3_beginning_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != lane_to_ofe.keys().copied().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier lane/OFE set",
            ));
        }
        Ok(lane_to_ofe)
    }

    fn covered_expected_destinations(&self) -> BTreeSet<(OfeId, TileId)> {
        self.snow_surface_forcing_by_destination
            .iter()
            .filter_map(|(destination, forcing)| {
                matches!(
                    forcing,
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(_)
                )
                .then(|| destination.clone())
            })
            .collect()
    }

    fn carrier_for_destination(
        &self,
        interval_s: f64,
        lane_id: u32,
        ofe_id: &OfeId,
        tile_id: &TileId,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let stage3_state =
            stage3_state_by_lane
                .get(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination Stage-3 state",
                ))?;
        let stage3_forcing = stage3_forcing_by_lane.get(&lane_id).copied().ok_or(
            DirectV11RealConsumerError::Identity("covered destination Stage-3 forcing"),
        )?;
        let sealed = self
            .snow_surface_forcing_by_destination
            .get(&(ofe_id.clone(), tile_id.clone()))
            .and_then(|forcing| match forcing {
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => Some(forcing),
                SealedStage3TileBoundaryForcingV1::OpenSnow(_) => None,
            })
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination carrier forcing",
            ))?;
        let vegetation_tile_id = self
            .beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == *ofe_id)
            .and_then(|ofe| ofe.tiles.iter().find(|tile| tile.tile_id == *tile_id))
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination vegetation tile",
            ))?
            .vegetation_tile_id
            .clone();
        let mut guess = self.derive_live_carrier_input(
            lane_id,
            stage3_state,
            vegetation_state,
            stage3_forcing,
            sealed,
            Some(&vegetation_tile_id),
            interval_s,
        )?;
        let stage3_beginning_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([
                (lane_id, stage3_state.clone()),
            ]))?);
        let forcing_sha256 = stage3_support_forcing_digest(stage3_forcing)?;
        let numerical_seed_context_sha256 = sealed.diagnostic_seed_context_digest();
        let duration_bits = interval_s.to_bits().to_be_bytes();
        guess.diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-diagnostic-v1",
            &[
                openwepp_coupled_time::FramedField {
                    tag: "ofe_id",
                    value: ofe_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "tile_id",
                    value: tile_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "duration_bits",
                    value: &duration_bits,
                },
                openwepp_coupled_time::FramedField {
                    tag: "diagnostic_numerical_seed_context",
                    value: numerical_seed_context_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "forcing_receipt",
                    value: forcing_sha256.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_v11_state",
                    value: self.beginning.vegetation_state.0.state_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_stage3_state",
                    value: stage3_beginning_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "guess_values",
                    value: guess.diagnostic_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered initial guess diagnostic"))?;
        Ok(guess)
    }

    fn stage3_lower_boundaries_by_destination(
        &self,
        receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        stage3_inputs_by_lane: &BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        let expected_destinations = self.covered_expected_destinations();
        if receipts.keys().cloned().collect::<BTreeSet<_>>() != expected_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination carrier receipt set",
            ));
        }
        let mut boundaries = BTreeMap::new();
        for (destination, receipt) in receipts {
            let carrier_receipt_id = Sha256Digest::try_new(digest32_hex(receipt.diagnostic_sha256))
                .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier receipt ID"))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary OFE binding",
                ))?;
            let stage3_input = stage3_inputs_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 inputs",
                ))?;
            let stage3_forcing = stage3_forcing_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 forcing",
                ))?;
            let snow_albedo = stage3_input
                .snow_albedo_state
                .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
            let boundary = Stage3SnowCoveredLowerBoundary {
                snow_temperature_k: receipt.snow_temperature_k,
                latent_heat_j_kg:
                    openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                        TemperatureCelsius::try_new(receipt.snow_temperature_k - 273.15).map_err(
                            |_| DirectV11RealConsumerError::Identity("covered temperature"),
                        )?,
                    )
                    .map_err(|_| DirectV11RealConsumerError::Identity("covered latent heat"))?
                    .as_joules_per_kilogram(),
                sensible_to_canopy_air_w_m2: -receipt.snow_sensible_into_surface_w_m2,
                vapor_to_canopy_air_kg_m2_s: -receipt.snow_vapor_into_surface_kg_m2_s,
                net_longwave_w_m2: receipt.snow_longwave_net_w_m2,
                // The current released carrier receipt does not yet expose a
                // canonical shortwave or precipitation-advection term. Keep
                // those owners explicit and zero only at this default-off
                // seam; the physical covered cutover remains blocked on their
                // Stage-3 projections and ledger reconstruction.
                shortwave_absorbed_w_m2: 0.0,
                precipitation_advection_w_m2: 0.0,
                carrier_receipt_id,
                snow_vis_albedo: snow_albedo,
                snow_nir_albedo: snow_albedo,
                stage3_albedo_state_sha256: stage3_albedo_state_digest(stage3_input)?,
                forcing_receipt_sha256: stage3_support_forcing_digest(*stage3_forcing)?,
                optical_receipt_sha256: None,
                reciprocal_longwave_receipt_sha256: None,
                final_canopy_boundary_receipt_sha256: None,
            };
            boundary
                .validate()
                .map_err(|_| DirectV11RealConsumerError::Identity("covered boundary operands"))?;
            if boundaries.insert(destination.clone(), boundary).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination lower boundary",
                ));
            }
        }
        Ok(boundaries)
    }

    /// Merge the latest persistent Stage-3 state operands into the boundary
    /// whose radiative and turbulent terms came from the preceding LSE solve.
    /// Stage-3 owns snow temperature (and therefore latent heat); LSE owns the
    /// exchanged fluxes.  Neither side may replace the other's operands.
    fn merge_latest_stage3_state_operands(
        &self,
        flux_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        flux_boundaries
            .iter()
            .map(|(destination, boundary)| {
                let lane_id = self
                    .beginning
                    .inner
                    .surface_configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.ofe_id == destination.0)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 state boundary OFE",
                    ))?
                    .production_lane_id;
                let state =
                    stage3_states
                        .get(&lane_id)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary lane",
                        ))?;
                let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary active-volume surface",
                        )
                    })?;
                let snow_temperature_k = surface.surface_temperature_k;
                let latent_heat_j_kg = surface.latent_heat_j_kg;
                let mut merged = boundary.clone();
                merged.snow_temperature_k = snow_temperature_k;
                merged.latent_heat_j_kg = latent_heat_j_kg;
                merged.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered merged Stage-3/LSE boundary")
                })?;
                Ok((destination.clone(), merged))
            })
            .collect()
    }

    pub fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        self.ending.take()
    }

    pub fn take_staged_stage3(&mut self) -> Option<BTreeMap<u32, DirectSnowStage3PersistentState>> {
        self.ending_stage3_by_lane.take()
    }

    #[must_use]
    pub fn last_final_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>> {
        self.last_final_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_lane_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, LaneStage3BoundaryReceiptV1>> {
        self.last_lane_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_component_carrier_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>> {
        self.last_component_carrier_receipts.as_ref()
    }
}
