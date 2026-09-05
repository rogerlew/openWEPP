#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn project_live_vegetation_forcing(
    provider: &SnowFreeForcing,
    hydrology: &RealHydrologyShadowAdapter,
    soil_thermal: DirectSoilThermalReadView<'_>,
    root_zone: Option<&DirectRootZoneHydraulicConfiguration>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_state: &V9CoupledOwnedState,
    receipt_vegetation_configuration_sha256: String,
    hydrology_snapshot_sha256: Sha256Digest,
    transaction_id: TransactionId,
    day_index: usize,
    interval_index: u8,
) -> Result<(SnowFreeForcing, Option<V10RootZoneReceiptSet>), DirectV9RealConsumerError> {
    let mut forcing = provider.clone();
    // V10 does not consume the legacy global ground scalars. Shortwave optics
    // come from each LSE tile, while reciprocal longwave uses the current
    // coupled ground trial. Canonical zeros prevent caller control and permit
    // heterogeneous tile configurations.
    forcing.ground_albedo_vis = 0.0;
    forcing.ground_albedo_nir = 0.0;
    forcing.longwave_up_w_m2 = 0.0;
    for layer in &mut forcing.soil_layers {
        let water_values = hydrology
            .layer_facts()
            .iter()
            .filter(|(source, _)| source.layer_id == layer.layer_id)
            .map(|(_, fact)| fact.liquid_supply_kg_m2)
            .collect::<Vec<_>>();
        let temperature_values = soil_thermal
            .ordered_ofes()
            .into_iter()
            .filter_map(|ofe| {
                ofe.ordered_layers()
                    .into_iter()
                    .find(|candidate| candidate.layer_id() == &layer.layer_id)
                    .map(DirectSoilThermalLayerReadView::temperature_k)
            })
            .collect::<Vec<_>>();
        let water = if root_zone.is_some() {
            water_values
                .first()
                .copied()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation soil-water projection",
                ))?
        } else {
            common_provider_value(&water_values, "vegetation soil-water projection")?
        };
        let temperature = if root_zone.is_some() {
            temperature_values
                .first()
                .copied()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation soil-temperature projection",
                ))?
        } else {
            common_provider_value(
                &temperature_values,
                "vegetation soil-temperature projection",
            )?
        };
        layer.water_beginning_kg_m2 = water;
        layer.temperature_k = temperature;
    }
    if let Some(root_zone) = root_zone {
        let mut receipts = Vec::new();
        for occupancy_id in vegetation_state.0.occupancies.keys() {
            let stratum = vegetation_configuration
                .strata
                .iter()
                .find(|value| value.stratum_id == occupancy_id.stratum_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "root-zone occupancy/stratum join",
                ))?;
            let geometry = root_zone
                .ordered_strata
                .iter()
                .find(|value| value.stratum_id == stratum.stratum_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "root-zone stratum geometry",
                ))?;
            for root in &stratum.root_layers {
                if root.root_fraction == 0.0 {
                    continue;
                }
                for configured in root_zone
                    .ordered_layers
                    .iter()
                    .filter(|value| value.layer_id == root.layer_id)
                {
                    let source = crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey {
                        ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                            lane_index: configured.production_lane_index,
                            lane_id: configured.production_lane_id,
                        },
                        layer_id: configured.layer_id.clone(),
                    };
                    let fact = hydrology.layer_facts().get(&source).ok_or(
                        DirectV9RealConsumerError::Identity("root-zone live hydrology layer"),
                    )?;
                    let mut top_m = 0.0;
                    for value in root_zone.ordered_layers.iter().take_while(|value| {
                        (
                            value.production_lane_index,
                            value.production_lane_id,
                            &value.layer_id,
                        ) != (
                            configured.production_lane_index,
                            configured.production_lane_id,
                            &configured.layer_id,
                        )
                    }) {
                        if value.production_lane_index == configured.production_lane_index
                            && value.production_lane_id == configured.production_lane_id
                        {
                            let prior = crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey {
                                ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                                    lane_index: value.production_lane_index,
                                    lane_id: value.production_lane_id,
                                },
                                layer_id: value.layer_id.clone(),
                            };
                            top_m += hydrology
                                .layer_facts()
                                .get(&prior)
                                .ok_or(DirectV9RealConsumerError::Identity(
                                    "root-zone predecessor hydrology layer",
                                ))?
                                .layer_thickness_m;
                        }
                    }
                    let source_values = root_zone_hydraulic_values(
                        fact,
                        configured,
                        top_m,
                        geometry.root_tissue_lateral_path_m,
                    )?;
                    let matching_ofes = surface_configuration
                        .ofe_bindings
                        .iter()
                        .filter(|binding| {
                            binding.production_lane_index == configured.production_lane_index
                                && binding.production_lane_id == configured.production_lane_id
                                && lse_configuration.ofes.iter().any(|ofe| {
                                    ofe.ofe_id == binding.ofe_id
                                        && ofe.tiles.iter().any(|tile| {
                                            tile.vegetation_tile_id == occupancy_id.tile_id
                                        })
                                })
                        })
                        .collect::<Vec<_>>();
                    // Root-zone configuration is complete over production
                    // lanes, while a vegetation occupancy exists only on OFEs
                    // containing that configured topology tile. A snow-free
                    // or open-only lane therefore contributes no receipt for
                    // this occupancy; it is not an identity failure.
                    for ofe in matching_ofes {
                        receipts.push(root_zone_hydraulic_receipt(
                            V10RootZoneReceiptKey {
                                ofe_id: ofe.ofe_id.clone(),
                                production_lane_index: configured.production_lane_index,
                                production_lane_id: configured.production_lane_id,
                                occupancy_id: occupancy_id.clone(),
                                stratum_id: stratum.stratum_id.clone(),
                                layer_id: root.layer_id.clone(),
                            },
                            source_values,
                            root.lateral_root_length_m,
                        )?);
                    }
                }
            }
        }
        return Ok((
            forcing,
            Some(V10RootZoneReceiptSet::try_new(
                root_zone.restart_identity_sha256().map_err(|_| {
                    DirectV9RealConsumerError::Identity("root-zone configuration identity")
                })?,
                lse_configuration
                    .hydrology_configuration
                    .configuration_sha256
                    .clone(),
                receipt_vegetation_configuration_sha256,
                lse_configuration.configuration_sha256.clone(),
                hydrology_snapshot_sha256,
                transaction_id,
                day_index,
                interval_index,
                receipts,
            )?),
        ));
    }
    Ok((forcing, None))
}
