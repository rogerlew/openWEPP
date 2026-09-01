#[derive(Clone, Copy, Debug, PartialEq)]
struct RootZoneHydraulicSourceValues {
    liquid_water_depth_m: f64,
    layer_thickness_m: f64,
    porosity: f64,
    saturated_conductivity_m_s: f64,
    saturated_matric_potential_mm: f64,
    clapp_hornberger_b: f64,
    layer_top_m: f64,
    root_tissue_lateral_path_m: f64,
    frozen: bool,
}

fn root_zone_hydraulic_receipt(
    key: V10RootZoneReceiptKey,
    source: RootZoneHydraulicSourceValues,
    lateral_root_length_m: f64,
) -> Result<V10RootZoneLayerReceipt, DirectV9RealConsumerError> {
    if !lateral_root_length_m.is_finite() || lateral_root_length_m <= 0.0 {
        return Err(V8InputProjectionError::RootDomain.into());
    }
    let capacity = source.porosity * source.layer_thickness_m;
    let capacity_limit = f64::from_bits(capacity.to_bits() + 1);
    if source.liquid_water_depth_m > capacity_limit {
        return Err(V8InputProjectionError::WaterAbovePoreCapacity.into());
    }
    let relative_saturation_raw = source.liquid_water_depth_m / capacity;
    let mut relative_saturation = relative_saturation_raw.clamp(0.0, 1.0);
    if relative_saturation == 0.0 {
        relative_saturation = 0.0;
    }
    let matric_potential_mm = (source.saturated_matric_potential_mm
        * libm::pow(relative_saturation.max(0.01), -source.clapp_hornberger_b))
    .max(-1.0e8);
    let hydraulic_conductivity_mm_s = 1000.0
        * source.saturated_conductivity_m_s.min(
            source.saturated_conductivity_m_s
                * libm::pow(relative_saturation, 2.0 * source.clapp_hornberger_b + 3.0),
        );
    let layer_node_depth_m = source.layer_top_m + 0.5 * source.layer_thickness_m;
    Ok(V10RootZoneLayerReceipt {
        key,
        matric_potential_mm,
        hydraulic_conductivity_mm_s,
        root_path_length_mm: 1000.0 * (layer_node_depth_m + source.root_tissue_lateral_path_m),
        gravity_root_mm: -1000.0 * layer_node_depth_m,
        lateral_root_length_m,
    })
}

fn root_zone_hydraulic_values(
    fact: &crate::vegetation_real_hydrology_shadow::RealHydrologyLayerFact,
    configuration: &DirectRootZoneLayerConfiguration,
    top_m: f64,
    root_tissue_lateral_path_m: f64,
) -> Result<RootZoneHydraulicSourceValues, DirectV9RealConsumerError> {
    let values = [
        fact.liquid_water_depth_m,
        fact.layer_thickness_m,
        fact.porosity,
        fact.saturated_conductivity_m_s,
        configuration.saturated_matric_potential_mm,
        configuration.clapp_hornberger_b,
        top_m,
        root_tissue_lateral_path_m,
    ];
    if values.iter().any(|value| !value.is_finite())
        || fact.liquid_water_depth_m < 0.0
        || fact.layer_thickness_m <= 0.0
        || !(0.0 < fact.porosity && fact.porosity <= 1.0)
        || fact.saturated_conductivity_m_s <= 0.0
        || configuration.saturated_matric_potential_mm >= 0.0
        || configuration.clapp_hornberger_b <= 0.0
        || top_m < 0.0
        || root_tissue_lateral_path_m < 0.0
    {
        return Err(V8InputProjectionError::RootDomain.into());
    }
    if fact.frozen {
        return Err(V8InputProjectionError::FrozenRootedLayerUnsupported.into());
    }
    Ok(RootZoneHydraulicSourceValues {
        liquid_water_depth_m: fact.liquid_water_depth_m,
        layer_thickness_m: fact.layer_thickness_m,
        porosity: fact.porosity,
        saturated_conductivity_m_s: fact.saturated_conductivity_m_s,
        saturated_matric_potential_mm: configuration.saturated_matric_potential_mm,
        clapp_hornberger_b: configuration.clapp_hornberger_b,
        layer_top_m: top_m,
        root_tissue_lateral_path_m,
        frozen: fact.frozen,
    })
}

fn common_provider_value(
    values: &[f64],
    detail: &'static str,
) -> Result<f64, DirectV9RealConsumerError> {
    let first = values
        .first()
        .copied()
        .ok_or(DirectV9RealConsumerError::Identity(detail))?;
    if values
        .iter()
        .any(|value| value.to_bits() != first.to_bits())
    {
        return Err(DirectV9RealConsumerError::Unsupported(detail));
    }
    Ok(first)
}

struct BiogeochemistryNitrogenArbiter {
    available: BTreeMap<MineralNitrogenKey, f64>,
}

impl BiogeochemistryNitrogenArbiter {
    fn try_new(state: &BiogeochemistryState) -> Result<Self, DirectV9RealConsumerError> {
        Ok(Self {
            available: available_by_key(state)?,
        })
    }
}

impl NitrogenArbiter for BiogeochemistryNitrogenArbiter {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError> {
        self.available
            .get(key)
            .copied()
            .ok_or(VegetationError::Domain("unknown nitrogen inventory"))
    }

    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        authorize_proportionally(
            requests,
            &self.available,
            ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
        )
        .map_err(VegetationError::from)
    }
}

fn aggregate_soil_thermal_ending(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    candidates: &[SoilThermalTileCandidate],
) -> Result<SoilThermalSnapshot, DirectV9RealConsumerError> {
    aggregate_soil_thermal_ending_with_top_boundary_credits(
        beginning,
        configuration,
        transaction_id,
        candidates,
        &[],
    )
    .map(|value| value.ending)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SoilThermalTopBoundaryCreditV1 {
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub first_layer_id: SoilLayerId,
    pub beginning_owner_id: ResourceOwnerId,
    pub beginning_configuration_sha256: Sha256Digest,
    pub beginning_state_sha256: Sha256Digest,
    pub support_start_ns: i64,
    pub support_end_ns: i64,
    pub accepted_positive_downward_j_m2_ofe_ground: f64,
    pub soil_thermal_credit_j_m2_ofe_ground: f64,
    pub snow_soil_heat_receipt_sha256: Sha256Digest,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SoilThermalTopBoundaryCreditSetV1 {
    pub ending: SoilThermalSnapshot,
    pub accepted_credit_set_sha256: Sha256Digest,
}

pub(crate) fn aggregate_soil_thermal_ending_with_top_boundary_credits(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    candidates: &[SoilThermalTileCandidate],
    top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
) -> Result<SoilThermalTopBoundaryCreditSetV1, DirectV9RealConsumerError> {
    validate_soil_thermal_candidate_set(beginning, configuration, candidates)?;
    let credits = validate_top_boundary_credit_set(beginning, top_boundary_credits)?;
    let mut ofes = Vec::with_capacity(beginning.ofes.len());
    for beginning_ofe in &beginning.ofes {
        ofes.push(aggregate_soil_thermal_ofe(
            beginning,
            beginning_ofe,
            configuration,
            candidates,
            credits.get(&beginning_ofe.ofe_id).copied(),
        )?);
    }
    let state_sha256 = digest_soil_state(&beginning.owner_id, transaction_id, &ofes)?;
    let snapshot_sha256 = digest_soil_snapshot(
        &beginning.owner_id,
        &beginning.configuration_sha256,
        &state_sha256,
        transaction_id,
        &ofes,
    )?;
    let ending = SoilThermalSnapshot {
        owner_id: beginning.owner_id.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256,
        snapshot_sha256,
        last_accepted_transaction_id: Some(transaction_id),
        ofes,
    };
    ending.validate()?;
    let ordered_credits = credits.values().copied().collect::<Vec<_>>();
    let accepted_credit_set_sha256 = digest_serialized(&(
        "OPENWEPP_SOIL_TOP_BOUNDARY_CREDIT_SET_V1",
        transaction_id,
        ordered_credits,
    ))?;
    Ok(SoilThermalTopBoundaryCreditSetV1 {
        ending,
        accepted_credit_set_sha256,
    })
}

fn validate_top_boundary_credit_set<'a>(
    beginning: &SoilThermalSnapshot,
    credits: &'a [SoilThermalTopBoundaryCreditV1],
) -> Result<BTreeMap<OfeId, &'a SoilThermalTopBoundaryCreditV1>, DirectV9RealConsumerError> {
    let mut by_ofe = BTreeMap::new();
    let mut lanes = BTreeSet::new();
    for credit in credits {
        let beginning_ofe = beginning
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == credit.ofe_id)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "soil top-boundary OFE",
            ))?;
        let first_layer =
            beginning_ofe
                .ordered_layers
                .first()
                .ok_or(DirectV9RealConsumerError::OwnerClosure(
                    "soil top-boundary first layer",
                ))?;
        if credit.beginning_owner_id != beginning.owner_id
            || credit.beginning_configuration_sha256 != beginning.configuration_sha256
            || credit.beginning_state_sha256 != beginning.state_sha256
            || credit.first_layer_id != first_layer.layer_id
            || credit.support_end_ns <= credit.support_start_ns
            || !credit
                .accepted_positive_downward_j_m2_ofe_ground
                .is_finite()
            || credit.soil_thermal_credit_j_m2_ofe_ground.to_bits()
                != credit.accepted_positive_downward_j_m2_ofe_ground.to_bits()
            || !lanes.insert(credit.lane_id)
            || by_ofe.insert(credit.ofe_id.clone(), credit).is_some()
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "soil top-boundary credit identity or sign",
            ));
        }
    }
    Ok(by_ofe)
}

fn validate_soil_thermal_candidate_set(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
) -> Result<(), DirectV9RealConsumerError> {
    let configured_tiles = configuration
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.tiles.iter().map(move |tile| {
                (
                    (ofe.ofe_id.clone(), tile.tile_id.clone()),
                    tile.fraction_ofe_ground,
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    let actual_tiles = candidates
        .iter()
        .map(|candidate| (candidate.ofe_id.clone(), candidate.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    let configured_ofes = configuration
        .ofes
        .iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<BTreeSet<_>>();
    let beginning_ofes = beginning
        .ofes
        .iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<BTreeSet<_>>();
    if actual_tiles.len() != candidates.len()
        || actual_tiles != configured_tiles.keys().cloned().collect()
        || beginning_ofes.len() != beginning.ofes.len()
        || beginning_ofes != configured_ofes
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal tile candidate set",
        ));
    }
    Ok(())
}

fn aggregate_soil_thermal_ofe(
    beginning: &SoilThermalSnapshot,
    beginning_ofe: &SoilThermalOfeSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
    top_boundary_credit: Option<&SoilThermalTopBoundaryCreditV1>,
) -> Result<SoilThermalOfeSnapshot, DirectV9RealConsumerError> {
    let configured_ofe = configuration
        .ofes
        .iter()
        .find(|ofe| ofe.ofe_id == beginning_ofe.ofe_id)
        .ok_or(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE configuration",
        ))?;
    let mut tile_candidates = candidates
        .iter()
        .filter(|candidate| candidate.ofe_id == beginning_ofe.ofe_id)
        .collect::<Vec<_>>();
    tile_candidates.sort_unstable_by(|left, right| left.tile_id.cmp(&right.tile_id));
    if tile_candidates.len() != configured_ofe.tiles.len() {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE tile cardinality",
        ));
    }
    if beginning_ofe.ordered_layers.len() != configured_ofe.soil_interface_layers.len() {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal beginning/configured layer cardinality",
        ));
    }
    let mut ordered_layers = Vec::with_capacity(beginning_ofe.ordered_layers.len());
    for (layer_index, beginning_layer) in beginning_ofe.ordered_layers.iter().enumerate() {
        let configured_layer = configured_ofe
            .soil_interface_layers
            .get(layer_index)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal configured layer order",
            ))?;
        if configured_layer.layer_id != beginning_layer.layer_id
            || !configured_layer.areal_heat_capacity_j_m2_k.is_finite()
            || configured_layer.areal_heat_capacity_j_m2_k <= 0.0
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal layer identity or capacity",
            ));
        }
        let mut ending_enthalpy = beginning_layer.enthalpy_j_m2_ofe_ground;
        for candidate in &tile_candidates {
            if candidate.owner_id != beginning.owner_id
                || candidate.beginning_state_sha256 != beginning.state_sha256
                || candidate.layers.len() != beginning_ofe.ordered_layers.len()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate owner lineage or layer cardinality",
                ));
            }
            let layer = candidate.layers.get(layer_index).ok_or(
                DirectV9RealConsumerError::OwnerClosure("soil-thermal candidate layer cardinality"),
            )?;
            if layer.layer_id != beginning_layer.layer_id
                || layer.beginning_enthalpy_j_m2_ofe_ground.to_bits()
                    != beginning_layer.enthalpy_j_m2_ofe_ground.to_bits()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate beginning layer",
                ));
            }
            ending_enthalpy +=
                layer.ending_enthalpy_j_m2_ofe_ground - layer.beginning_enthalpy_j_m2_ofe_ground;
        }
        if layer_index == 0 {
            if let Some(credit) = top_boundary_credit {
                ending_enthalpy += credit.soil_thermal_credit_j_m2_ofe_ground;
            }
        }
        let ending_temperature_k = beginning_layer.temperature_k
            + (ending_enthalpy - beginning_layer.enthalpy_j_m2_ofe_ground)
                / configured_layer.areal_heat_capacity_j_m2_k;
        if !ending_enthalpy.is_finite() || !(200.0..=350.0).contains(&ending_temperature_k) {
            return Err(DirectV9RealConsumerError::Unsupported(
                "aggregated soil-thermal ending domain",
            ));
        }
        ordered_layers.push(SoilThermalLayerSnapshot {
            layer_id: beginning_layer.layer_id.clone(),
            temperature_k: ending_temperature_k,
            enthalpy_j_m2_ofe_ground: ending_enthalpy,
        });
    }
    Ok(SoilThermalOfeSnapshot {
        ofe_id: beginning_ofe.ofe_id.clone(),
        ordered_layers,
    })
}

fn digest_soil_state(
    owner_id: &ResourceOwnerId,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(owner_id, transaction_id, ofes))
}

fn digest_soil_snapshot(
    owner_id: &ResourceOwnerId,
    configuration_sha256: &Sha256Digest,
    state_sha256: &Sha256Digest,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(
        owner_id,
        configuration_sha256,
        state_sha256,
        transaction_id,
        ofes,
    ))
}

/// Independently sourced, canonically ordered V2 soil-energy receiver inputs.
///
/// This set is constructed from physical owner receipts before the LSE credit
/// receipt exists. Keeping it separate prevents a resealed receipt from
/// supplying its own expected operands during replay or restart validation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalExpectedAcceptedOperandSetV2 {
    accepted_operands: Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
    temperature_projections: Vec<openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2>,
    expected_set_sha256: Sha256Digest,
}

impl SoilThermalExpectedAcceptedOperandSetV2 {
    pub fn try_new(
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        configuration: &LandSurfaceEnergyConfiguration,
        accepted_operands: Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
    ) -> Result<Self, DirectV9RealConsumerError> {
        validate_v2_beginning_configuration(beginning, configuration)?;
        validate_v2_operand_order_and_topology(beginning, &accepted_operands)?;
        let temperature_projections =
            v2_temperature_projections(beginning, configuration, &accepted_operands)?;
        let expected_set_sha256 = digest_serialized(&(
            "OPENWEPP_SOIL_THERMAL_EXPECTED_ACCEPTED_OPERAND_SET_V2",
            &accepted_operands,
            &temperature_projections,
        ))?;
        Ok(Self {
            accepted_operands,
            temperature_projections,
            expected_set_sha256,
        })
    }

    pub fn validate(
        &self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<(), DirectV9RealConsumerError> {
        let rebuilt = Self::try_new(beginning, configuration, self.accepted_operands.clone())?;
        if rebuilt != *self {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 expected soil-energy operand set",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn accepted_operands(
        &self,
    ) -> &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2] {
        &self.accepted_operands
    }

    #[must_use]
    pub fn temperature_projections(
        &self,
    ) -> &[openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2] {
        &self.temperature_projections
    }

    #[must_use]
    pub const fn expected_set_sha256(&self) -> &Sha256Digest {
        &self.expected_set_sha256
    }
}

/// Clone-only orchestrator candidate. Installation remains the enclosing
/// complete-owner transaction's responsibility.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalAcceptedCandidateV2 {
    pub ending_owner: openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    pub credit_receipt: openwepp_land_surface_energy::SoilThermalEnergyCreditReceiptV2,
    pub expected_sources: SoilThermalExpectedAcceptedOperandSetV2,
}

pub fn aggregate_soil_thermal_ending_v2(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    configuration: &LandSurfaceEnergyConfiguration,
    expected_sources: &SoilThermalExpectedAcceptedOperandSetV2,
) -> Result<SoilThermalAcceptedCandidateV2, DirectV9RealConsumerError> {
    expected_sources.validate(beginning, configuration)?;
    let candidate = openwepp_land_surface_energy::apply_soil_thermal_energy_credit_v2(
        beginning,
        expected_sources.accepted_operands(),
        expected_sources.temperature_projections(),
    )
    .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 exact soil-energy credit"))?;
    candidate
        .credit_receipt
        .validate_independent(
            beginning,
            &candidate.ending_owner,
            expected_sources.accepted_operands(),
            expected_sources.temperature_projections(),
        )
        .map_err(|_| {
            DirectV9RealConsumerError::OwnerClosure("V2 exact soil-energy reconstruction")
        })?;
    Ok(SoilThermalAcceptedCandidateV2 {
        ending_owner: candidate.ending_owner,
        credit_receipt: candidate.credit_receipt,
        expected_sources: expected_sources.clone(),
    })
}

pub fn soil_thermal_top_boundary_operands_v2(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    credits: &[SoilThermalTopBoundaryCreditV1],
    source_owner_id: &ResourceOwnerId,
) -> Result<
    Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
    DirectV9RealConsumerError,
> {
    let mut operands = Vec::with_capacity(credits.len());
    let mut identities = BTreeSet::new();
    for credit in credits {
        let ofe = beginning
            .state
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == credit.ofe_id)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "V2 top-boundary OFE",
            ))?;
        let layer = ofe
            .ordered_layers
            .first()
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "V2 top-boundary first layer",
            ))?;
        let support_start_ns = u128::try_from(credit.support_start_ns).map_err(|_| {
            DirectV9RealConsumerError::OwnerClosure("V2 top-boundary support start")
        })?;
        let support_end_ns = u128::try_from(credit.support_end_ns)
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 top-boundary support end"))?;
        if credit.beginning_owner_id != beginning.state.owner_id
            || credit.beginning_configuration_sha256 != beginning.state.configuration_sha256
            || credit.beginning_state_sha256 != beginning.state.state_sha256
            || credit.first_layer_id != layer.layer_id
            || support_start_ns != beginning.support_start_ns
            || support_end_ns != beginning.support_end_ns
            || !credit
                .accepted_positive_downward_j_m2_ofe_ground
                .is_finite()
            || credit.soil_thermal_credit_j_m2_ofe_ground.to_bits()
                != credit.accepted_positive_downward_j_m2_ofe_ground.to_bits()
            || !identities.insert((credit.ofe_id.clone(), credit.lane_id))
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 top-boundary credit identity, support, or sign",
            ));
        }
        operands.push(
            openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
                ofe_id: credit.ofe_id.clone(),
                layer_id: layer.layer_id.clone(),
                source_kind:
                    openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::TopBoundary,
                source_owner_id: source_owner_id.clone(),
                debit_credit_identity_sha256: credit.snow_soil_heat_receipt_sha256.clone(),
                ordinal: credit.lane_id,
                units: "J m^-2 OFE-ground".to_owned(),
                basis: "ofe_ground".to_owned(),
                energy_j_m2_ofe_ground: credit.soil_thermal_credit_j_m2_ofe_ground,
            },
        );
    }
    canonicalize_v2_operand_order(beginning, &mut operands)?;
    Ok(operands)
}

/// Successor physical receiver path. It reconstructs soil-internal and
/// infiltration operands from the frozen LSE/surface receipts, accepts only
/// separately sealed top-boundary operands, and returns an unpublished clone.
#[allow(clippy::too_many_arguments)]
pub fn aggregate_soil_thermal_physical_ending_v2(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    lse_owner_id: &ResourceOwnerId,
    surface_owner_id: &ResourceOwnerId,
    pre_ingress_candidates: &[SoilThermalTileCandidate],
    ingress: &crate::DirectSurfaceLiquidIngressCandidate,
    top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
    top_boundary_source_owner_id: &ResourceOwnerId,
) -> Result<SoilThermalAcceptedCandidateV2, DirectV9RealConsumerError> {
    let transaction_authority =
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            transaction_id,
            beginning.transaction_id,
        )
        .map_err(DirectV9RealConsumerError::LandSurfaceShadow)?;
    let mut operands = crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
        transaction_authority,
        beginning.support_start_ns,
        beginning.support_end_ns,
        lse_owner_id,
        surface_owner_id,
        pre_ingress_candidates,
        ingress,
    )
    .map_err(DirectV9RealConsumerError::LandSurfaceShadow)?;
    crate::land_surface_energy_shadow::validate_soil_thermal_v2_surface_cancellation(
        transaction_authority,
        beginning.support_start_ns,
        beginning.support_end_ns,
        surface_owner_id,
        &operands,
        ingress,
    )
    .map_err(DirectV9RealConsumerError::LandSurfaceShadow)?;
    let mut top_boundary_operands = soil_thermal_top_boundary_operands_v2(
        beginning,
        top_boundary_credits,
        top_boundary_source_owner_id,
    )?;
    operands.append(&mut top_boundary_operands);
    canonicalize_v2_operand_order(beginning, &mut operands)?;
    let expected =
        SoilThermalExpectedAcceptedOperandSetV2::try_new(beginning, configuration, operands)?;
    aggregate_soil_thermal_ending_v2(beginning, configuration, &expected)
}

fn validate_v2_beginning_configuration(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    configuration: &LandSurfaceEnergyConfiguration,
) -> Result<(), DirectV9RealConsumerError> {
    beginning
        .validate()
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 soil beginning owner"))?;
    match configuration.model_version.as_str() {
        openwepp_land_surface_energy::MODEL_VERSION => configuration.validate()?,
        openwepp_land_surface_energy::V2_MODEL_VERSION => configuration.validate_v2()?,
        _ => {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 soil supported LSE configuration identity",
            ));
        }
    }
    if beginning.state.owner_id != configuration.soil_thermal_configuration.owner_id
        || beginning.state.configuration_sha256
            != configuration
                .soil_thermal_configuration
                .configuration_sha256
        || beginning.state.ofes.len() != configuration.ofes.len()
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "V2 soil configuration identity",
        ));
    }
    for (owner_ofe, configured_ofe) in beginning.state.ofes.iter().zip(&configuration.ofes) {
        if owner_ofe.ofe_id != configured_ofe.ofe_id
            || owner_ofe.ordered_layers.len() != configured_ofe.soil_interface_layers.len()
            || owner_ofe
                .ordered_layers
                .iter()
                .zip(&configured_ofe.soil_interface_layers)
                .any(|(owner, configured)| owner.layer_id != configured.layer_id)
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 soil OFE/layer configuration topology",
            ));
        }
    }
    Ok(())
}

fn validate_v2_operand_order_and_topology(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Result<(), DirectV9RealConsumerError> {
    let layers = beginning
        .state
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.ordered_layers
                .iter()
                .map(move |layer| (&ofe.ofe_id, &layer.layer_id))
        })
        .collect::<BTreeSet<_>>();
    let mut prior_by_layer = BTreeMap::new();
    let mut debit_credit_identities = BTreeSet::new();
    for operand in operands {
        if !layers.contains(&(&operand.ofe_id, &operand.layer_id))
            || operand.units != "J m^-2 OFE-ground"
            || operand.basis != "ofe_ground"
            || !operand.energy_j_m2_ofe_ground.is_finite()
            || !debit_credit_identities.insert(operand.debit_credit_identity_sha256.clone())
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 soil operand identity, units, domain, or uniqueness",
            ));
        }
        let layer_key = (operand.ofe_id.clone(), operand.layer_id.clone());
        let order_key = (operand.source_kind, operand.ordinal);
        if prior_by_layer
            .insert(layer_key, order_key)
            .is_some_and(|prior| prior >= order_key)
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 soil operand canonical order",
            ));
        }
    }
    let mut canonical = operands.to_vec();
    canonicalize_v2_operand_order(beginning, &mut canonical)?;
    if canonical != operands {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "V2 soil operand global canonical order",
        ));
    }
    Ok(())
}

fn canonicalize_v2_operand_order(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    operands: &mut [openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Result<(), DirectV9RealConsumerError> {
    let ranks = beginning
        .state
        .ofes
        .iter()
        .enumerate()
        .flat_map(|(ofe_rank, ofe)| {
            ofe.ordered_layers
                .iter()
                .enumerate()
                .map(move |(layer_rank, layer)| {
                    (
                        (ofe.ofe_id.clone(), layer.layer_id.clone()),
                        (ofe_rank, layer_rank),
                    )
                })
        })
        .collect::<BTreeMap<_, _>>();
    if operands
        .iter()
        .any(|operand| !ranks.contains_key(&(operand.ofe_id.clone(), operand.layer_id.clone())))
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "V2 soil operand layer topology",
        ));
    }
    operands.sort_unstable_by_key(|operand| {
        (
            ranks[&(operand.ofe_id.clone(), operand.layer_id.clone())],
            operand.source_kind,
            operand.ordinal,
        )
    });
    Ok(())
}

fn v2_temperature_projections(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    configuration: &LandSurfaceEnergyConfiguration,
    operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Result<
    Vec<openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2>,
    DirectV9RealConsumerError,
> {
    let mut projections = Vec::new();
    for (owner_ofe, configured_ofe) in beginning.state.ofes.iter().zip(&configuration.ofes) {
        for (layer, configured_layer) in owner_ofe
            .ordered_layers
            .iter()
            .zip(&configured_ofe.soil_interface_layers)
        {
            let values = operands
                .iter()
                .filter(|operand| {
                    operand.ofe_id == owner_ofe.ofe_id && operand.layer_id == layer.layer_id
                })
                .map(|operand| operand.energy_j_m2_ofe_ground)
                .collect::<Vec<_>>();
            let total = openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum_binary64(
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                &values,
            )
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 exact soil-energy sum"))?;
            let (ending_high, ending_carry) = if values.is_empty() {
                (
                    layer.enthalpy_hi_j_m2_ofe_ground,
                    layer.enthalpy_carry.clone(),
                )
            } else {
                total.rounded_high_and_remainder().map_err(|_| {
                    DirectV9RealConsumerError::OwnerClosure("V2 soil-energy finite rounding")
                })?
            };
            let ending_temperature_k = openwepp_land_surface_energy::project_soil_temperature_k(
                layer.temperature_k,
                configured_layer.areal_heat_capacity_j_m2_k,
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                ending_high,
                &ending_carry,
            )
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure("V2 soil temperature projection")
            })?;
            projections.push(
                openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2 {
                    ofe_id: owner_ofe.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    heat_capacity_j_m2_k: configured_layer.areal_heat_capacity_j_m2_k,
                    ending_temperature_k,
                },
            );
        }
    }
    Ok(projections)
}

/// Canonical restart/checkpoint custody for one accepted V2 owner.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOrchestratorSealsV2 {
    pub restart: openwepp_land_surface_energy::SoilThermalOwnerRestartV2,
    pub checkpoint: openwepp_land_surface_energy::SoilThermalOwnerCheckpointV2,
    pub latest_credit_receipt_sha256: Sha256Digest,
    pub expected_operand_set_sha256: Sha256Digest,
    pub orchestrator_seal_sha256: Sha256Digest,
}

pub fn seal_soil_thermal_accepted_candidate_v2(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    candidate: &SoilThermalAcceptedCandidateV2,
) -> Result<SoilThermalOrchestratorSealsV2, DirectV9RealConsumerError> {
    candidate
        .credit_receipt
        .validate_independent(
            beginning,
            &candidate.ending_owner,
            candidate.expected_sources.accepted_operands(),
            candidate.expected_sources.temperature_projections(),
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 accepted candidate seal"))?;
    let zero = Sha256Digest::try_new("0".repeat(64))?;
    let ending = &candidate.ending_owner;
    let mut restart = openwepp_land_surface_energy::SoilThermalOwnerRestartV2 {
        owner_tag: ending.owner_tag.clone(),
        schema_sha256: ending.schema_sha256.clone(),
        exact_carry_definition_sha256: ending.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: ending.parent_v1_state_sha256.clone(),
        owner_state_sha256: ending.state.state_sha256.clone(),
        last_accepted_transaction_id: ending.state.last_accepted_transaction_id,
        receipt_chain_sha256: ending.receipt_chain_sha256.clone(),
        restart_sha256: zero.clone(),
    };
    restart.restart_sha256 = soil_thermal_restart_v2_sha256(&restart)?;
    let mut checkpoint = openwepp_land_surface_energy::SoilThermalOwnerCheckpointV2 {
        owner_tag: ending.owner_tag.clone(),
        schema_sha256: ending.schema_sha256.clone(),
        exact_carry_definition_sha256: ending.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: ending.parent_v1_state_sha256.clone(),
        owner_state_sha256: ending.state.state_sha256.clone(),
        last_accepted_transaction_id: ending.state.last_accepted_transaction_id,
        receipt_chain_sha256: ending.receipt_chain_sha256.clone(),
        checkpoint_sha256: zero,
    };
    checkpoint.checkpoint_sha256 = soil_thermal_checkpoint_v2_sha256(&checkpoint)?;
    let latest_credit_receipt_sha256 = candidate.credit_receipt.receipt_sha256.clone();
    let expected_operand_set_sha256 = candidate.expected_sources.expected_set_sha256().clone();
    let orchestrator_seal_sha256 = digest_serialized(&(
        "OPENWEPP_SOIL_THERMAL_ORCHESTRATOR_SEALS_V2",
        &restart,
        &checkpoint,
        &latest_credit_receipt_sha256,
        &expected_operand_set_sha256,
    ))?;
    let seals = SoilThermalOrchestratorSealsV2 {
        restart,
        checkpoint,
        latest_credit_receipt_sha256,
        expected_operand_set_sha256,
        orchestrator_seal_sha256,
    };
    validate_soil_thermal_orchestrator_seals_v2(beginning, candidate, &seals)?;
    Ok(seals)
}

pub fn validate_soil_thermal_orchestrator_seals_v2(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    candidate: &SoilThermalAcceptedCandidateV2,
    seals: &SoilThermalOrchestratorSealsV2,
) -> Result<(), DirectV9RealConsumerError> {
    candidate
        .credit_receipt
        .validate_independent(
            beginning,
            &candidate.ending_owner,
            candidate.expected_sources.accepted_operands(),
            candidate.expected_sources.temperature_projections(),
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 accepted receipt replay"))?;
    let ending = &candidate.ending_owner;
    if seals.restart.owner_tag != ending.owner_tag
        || seals.restart.schema_sha256 != ending.schema_sha256
        || seals.restart.exact_carry_definition_sha256 != ending.exact_carry_definition_sha256
        || seals.restart.parent_v1_state_sha256 != ending.parent_v1_state_sha256
        || seals.restart.owner_state_sha256 != ending.state.state_sha256
        || seals.restart.last_accepted_transaction_id != ending.state.last_accepted_transaction_id
        || seals.restart.receipt_chain_sha256 != ending.receipt_chain_sha256
        || seals.restart.restart_sha256 != soil_thermal_restart_v2_sha256(&seals.restart)?
        || seals.checkpoint.owner_tag != ending.owner_tag
        || seals.checkpoint.schema_sha256 != ending.schema_sha256
        || seals.checkpoint.exact_carry_definition_sha256 != ending.exact_carry_definition_sha256
        || seals.checkpoint.parent_v1_state_sha256 != ending.parent_v1_state_sha256
        || seals.checkpoint.owner_state_sha256 != ending.state.state_sha256
        || seals.checkpoint.last_accepted_transaction_id
            != ending.state.last_accepted_transaction_id
        || seals.checkpoint.receipt_chain_sha256 != ending.receipt_chain_sha256
        || seals.checkpoint.checkpoint_sha256
            != soil_thermal_checkpoint_v2_sha256(&seals.checkpoint)?
        || seals.latest_credit_receipt_sha256 != candidate.credit_receipt.receipt_sha256
        || seals.expected_operand_set_sha256 != *candidate.expected_sources.expected_set_sha256()
        || seals.orchestrator_seal_sha256
            != digest_serialized(&(
                "OPENWEPP_SOIL_THERMAL_ORCHESTRATOR_SEALS_V2",
                &seals.restart,
                &seals.checkpoint,
                &seals.latest_credit_receipt_sha256,
                &seals.expected_operand_set_sha256,
            ))?
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "V2 restart/checkpoint/receipt seal join",
        ));
    }
    Ok(())
}

fn soil_thermal_restart_v2_sha256(
    restart: &openwepp_land_surface_energy::SoilThermalOwnerRestartV2,
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(
        "OPENWEPP_SOIL_THERMAL_OWNER_RESTART_V2",
        &restart.owner_tag,
        &restart.schema_sha256,
        &restart.exact_carry_definition_sha256,
        &restart.parent_v1_state_sha256,
        &restart.owner_state_sha256,
        restart.last_accepted_transaction_id,
        &restart.receipt_chain_sha256,
    ))
}

fn soil_thermal_checkpoint_v2_sha256(
    checkpoint: &openwepp_land_surface_energy::SoilThermalOwnerCheckpointV2,
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(
        "OPENWEPP_SOIL_THERMAL_OWNER_CHECKPOINT_V2",
        &checkpoint.owner_tag,
        &checkpoint.schema_sha256,
        &checkpoint.exact_carry_definition_sha256,
        &checkpoint.parent_v1_state_sha256,
        &checkpoint.owner_state_sha256,
        checkpoint.last_accepted_transaction_id,
        &checkpoint.receipt_chain_sha256,
    ))
}

/// Verify both nested soil-owner digests using the real consumer's exact
/// digest recipes. This is exposed only to the package-local authority
/// evidence feature.
#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_validate_soil_thermal_digests(
    snapshot: &SoilThermalSnapshot,
) -> Result<(), DirectV9RealConsumerError> {
    let transaction_id =
        snapshot
            .last_accepted_transaction_id
            .ok_or(DirectV9RealConsumerError::Identity(
                "soil-thermal transaction lineage",
            ))?;
    let state = digest_soil_state(&snapshot.owner_id, transaction_id, &snapshot.ofes)?;
    let outer = digest_soil_snapshot(
        &snapshot.owner_id,
        &snapshot.configuration_sha256,
        &state,
        transaction_id,
        &snapshot.ofes,
    )?;
    if state != snapshot.state_sha256 || outer != snapshot.snapshot_sha256 {
        return Err(DirectV9RealConsumerError::Identity(
            "soil-thermal nested digest",
        ));
    }
    Ok(())
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_seal_soil_thermal_digests(
    snapshot: &mut SoilThermalSnapshot,
) -> Result<(), DirectV9RealConsumerError> {
    let transaction_id =
        snapshot
            .last_accepted_transaction_id
            .ok_or(DirectV9RealConsumerError::Identity(
                "soil-thermal transaction lineage",
            ))?;
    snapshot.state_sha256 = digest_soil_state(&snapshot.owner_id, transaction_id, &snapshot.ofes)?;
    snapshot.snapshot_sha256 = digest_soil_snapshot(
        &snapshot.owner_id,
        &snapshot.configuration_sha256,
        &snapshot.state_sha256,
        transaction_id,
        &snapshot.ofes,
    )?;
    Ok(())
}

#[cfg(test)]
mod soil_thermal_exact_carry_v2_tests {
    include!("v9_real_consumer_shadow/soil_thermal_exact_carry_v2_tests.rs");
}
