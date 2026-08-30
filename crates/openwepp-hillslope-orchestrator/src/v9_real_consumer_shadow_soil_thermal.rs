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
pub(crate) struct SoilThermalTopBoundaryCreditV1 {
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
