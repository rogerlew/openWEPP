//! V4 whole-column radiation preparation for occupancy-local solves.
//!
//! The constitutive E01--E03 implementation remains in [`crate::radiation`].
//! This module binds its ordered layer results to exact topology occupancy
//! identities and carries the conditional leaf/stem areas into the occupancy
//! solver boundary. It does not average strata or solve layers independently.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{OccupancyId, TileId};

use crate::VegetationError;
use crate::config::VegetationConfiguration;
use crate::radiation::{
    ColumnRadiationResult, IncidentComponent, MixedLayer, OwnedLayerAbsorption, RadiationBand,
    SurfaceOptics, solve_mixed_column,
};
use crate::transaction::{CoupledOwnedState, SnowFreeForcing};

/// One band/component owner result with its identity retained at the adapter
/// boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OccupancyRadiationComponent {
    pub band: RadiationBand,
    pub component: IncidentComponent,
    pub absorption: OwnedLayerAbsorption,
}

/// Complete V4 radiation operands for one exact `(stratum, tile)` occupancy.
#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyRadiation {
    pub occupancy_id: OccupancyId,
    pub conditional_lai_m2_m2_tile_ground: f64,
    pub conditional_wai_m2_m2_tile_ground: f64,
    pub visible_direct: OccupancyRadiationComponent,
    pub visible_diffuse: OccupancyRadiationComponent,
    pub near_infrared_direct: OccupancyRadiationComponent,
    pub near_infrared_diffuse: OccupancyRadiationComponent,
}

/// Whole-column boundary results retained for independent energy closure.
#[derive(Clone, Debug, PartialEq)]
pub struct TileColumnRadiation {
    pub tile_id: TileId,
    pub visible_direct: ColumnRadiationResult,
    pub visible_diffuse: ColumnRadiationResult,
    pub near_infrared_direct: ColumnRadiationResult,
    pub near_infrared_diffuse: ColumnRadiationResult,
}

/// Radiation preparation for all configured tile columns.
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedRadiation {
    pub occupancies: BTreeMap<OccupancyId, OccupancyRadiation>,
    pub columns: BTreeMap<TileId, TileColumnRadiation>,
}

impl OccupancyRadiationComponent {
    fn from_absorption(
        band: RadiationBand,
        component: IncidentComponent,
        absorption: OwnedLayerAbsorption,
    ) -> Result<Self, VegetationError> {
        if absorption.band != band || absorption.component != component {
            return Err(VegetationError::Receipt(
                "V4 radiation band/component identity".into(),
            ));
        }
        Ok(Self {
            band,
            component,
            absorption,
        })
    }
}

/// Solves every complete configured tile column once for each exact
/// VIS/NIR-by-direct/diffuse identity and binds ordered owner results to exact
/// occupancy IDs.
#[allow(clippy::too_many_lines)]
pub fn prepare_whole_column_radiation(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<PreparedRadiation, VegetationError> {
    configuration.validate()?;
    beginning.validate(configuration)?;

    let mut occupancies = BTreeMap::new();
    let mut columns = BTreeMap::new();

    for tile in &configuration.topology_tiles {
        let mut strata = configuration
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.contains(&tile.tile_id))
            .collect::<Vec<_>>();
        strata.sort_by_key(|stratum| stratum.vertical_rank);

        let mut occupancy_ids = Vec::with_capacity(strata.len());
        let mut conditional_areas = Vec::with_capacity(strata.len());
        let mut layers = Vec::with_capacity(strata.len());
        for stratum in strata {
            let occupancy_id = OccupancyId {
                stratum_id: stratum.stratum_id.clone(),
                tile_id: tile.tile_id.clone(),
            };
            let shared =
                beginning
                    .strata
                    .get(&stratum.stratum_id)
                    .ok_or(VegetationError::Domain(
                        "V4 radiation shared stratum identity",
                    ))?;
            let coverage = configuration.stratum_coverage(&stratum.stratum_id)?;
            let leaf_area_tile = shared.leaf_area / coverage;
            let stem_area_tile = shared.stem_area / coverage;
            if !leaf_area_tile.is_finite()
                || !stem_area_tile.is_finite()
                || leaf_area_tile < 0.0
                || stem_area_tile < 0.0
            {
                return Err(VegetationError::Domain(
                    "V4 radiation conditional plant area",
                ));
            }
            occupancy_ids.push(occupancy_id);
            conditional_areas.push((leaf_area_tile, stem_area_tile));
            layers.push(MixedLayer {
                leaf_area: leaf_area_tile,
                stem_area: stem_area_tile,
                clumping_index: stratum.clumping_index,
                leaf_angle_chi: stratum.leaf_angle_chi,
                leaf_optics: SurfaceOptics {
                    reflectance: stratum.leaf_rho_vis,
                    transmittance: stratum.leaf_tau_vis,
                },
                stem_optics: SurfaceOptics {
                    reflectance: stratum.stem_rho_vis,
                    transmittance: stratum.stem_tau_vis,
                },
            });
        }

        let visible_direct = solve_mixed_column(
            &layers,
            RadiationBand::Visible,
            IncidentComponent::Direct,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_vis,
            forcing.direct_par_w_m2,
        )?;
        let visible_diffuse = solve_mixed_column(
            &layers,
            RadiationBand::Visible,
            IncidentComponent::Diffuse,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_vis,
            forcing.diffuse_par_w_m2,
        )?;

        let nir_layers = strata_to_nir_layers(configuration, &occupancy_ids, &conditional_areas)?;
        let near_infrared_direct = solve_mixed_column(
            &nir_layers,
            RadiationBand::NearInfrared,
            IncidentComponent::Direct,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_nir,
            forcing.direct_nir_w_m2,
        )?;
        let near_infrared_diffuse = solve_mixed_column(
            &nir_layers,
            RadiationBand::NearInfrared,
            IncidentComponent::Diffuse,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_nir,
            forcing.diffuse_nir_w_m2,
        )?;

        require_layer_cardinality(&occupancy_ids, &visible_direct)?;
        require_layer_cardinality(&occupancy_ids, &visible_diffuse)?;
        require_layer_cardinality(&occupancy_ids, &near_infrared_direct)?;
        require_layer_cardinality(&occupancy_ids, &near_infrared_diffuse)?;

        for (index, occupancy_id) in occupancy_ids.iter().enumerate() {
            let (leaf_area_tile, stem_area_tile) = conditional_areas[index];
            let value = OccupancyRadiation {
                occupancy_id: occupancy_id.clone(),
                conditional_lai_m2_m2_tile_ground: leaf_area_tile,
                conditional_wai_m2_m2_tile_ground: stem_area_tile,
                visible_direct: component_at(
                    &visible_direct,
                    index,
                    RadiationBand::Visible,
                    IncidentComponent::Direct,
                )?,
                visible_diffuse: component_at(
                    &visible_diffuse,
                    index,
                    RadiationBand::Visible,
                    IncidentComponent::Diffuse,
                )?,
                near_infrared_direct: component_at(
                    &near_infrared_direct,
                    index,
                    RadiationBand::NearInfrared,
                    IncidentComponent::Direct,
                )?,
                near_infrared_diffuse: component_at(
                    &near_infrared_diffuse,
                    index,
                    RadiationBand::NearInfrared,
                    IncidentComponent::Diffuse,
                )?,
            };
            if occupancies.insert(occupancy_id.clone(), value).is_some() {
                return Err(VegetationError::Domain(
                    "duplicate V4 radiation occupancy identity",
                ));
            }
        }

        let column = TileColumnRadiation {
            tile_id: tile.tile_id.clone(),
            visible_direct,
            visible_diffuse,
            near_infrared_direct,
            near_infrared_diffuse,
        };
        if columns.insert(tile.tile_id.clone(), column).is_some() {
            return Err(VegetationError::Domain(
                "duplicate V4 radiation tile identity",
            ));
        }
    }

    if occupancies
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        != configuration.expected_occupancies()
    {
        return Err(VegetationError::Domain(
            "V4 radiation occupancy set identity",
        ));
    }
    Ok(PreparedRadiation {
        occupancies,
        columns,
    })
}

fn strata_to_nir_layers(
    configuration: &VegetationConfiguration,
    occupancy_ids: &[OccupancyId],
    conditional_areas: &[(f64, f64)],
) -> Result<Vec<MixedLayer>, VegetationError> {
    occupancy_ids
        .iter()
        .zip(conditional_areas)
        .map(|(occupancy_id, &(leaf_area, stem_area))| {
            let stratum = configuration
                .strata
                .iter()
                .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
                .ok_or(VegetationError::Domain("V4 NIR radiation stratum identity"))?;
            Ok(MixedLayer {
                leaf_area,
                stem_area,
                clumping_index: stratum.clumping_index,
                leaf_angle_chi: stratum.leaf_angle_chi,
                leaf_optics: SurfaceOptics {
                    reflectance: stratum.leaf_rho_nir,
                    transmittance: stratum.leaf_tau_nir,
                },
                stem_optics: SurfaceOptics {
                    reflectance: stratum.stem_rho_nir,
                    transmittance: stratum.stem_tau_nir,
                },
            })
        })
        .collect()
}

fn require_layer_cardinality(
    occupancy_ids: &[OccupancyId],
    result: &ColumnRadiationResult,
) -> Result<(), VegetationError> {
    if result.layers.len() != occupancy_ids.len() {
        return Err(VegetationError::Receipt(
            "V4 radiation layer cardinality".into(),
        ));
    }
    Ok(())
}

fn component_at(
    result: &ColumnRadiationResult,
    index: usize,
    band: RadiationBand,
    component: IncidentComponent,
) -> Result<OccupancyRadiationComponent, VegetationError> {
    let absorption = result
        .layers
        .get(index)
        .copied()
        .ok_or_else(|| VegetationError::Receipt("V4 radiation layer identity".into()))?;
    OccupancyRadiationComponent::from_absorption(band, component, absorption)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StratumConfiguration;
    use crate::occupancy_state::OccupancyState;
    use crate::transaction::{SoilLayerForcing, StratumSharedState};
    use openwepp_kernel_contract::{SoilLayerId, StratumId};

    fn layer_id(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer identity")
    }

    fn stratum_id(value: &str) -> StratumId {
        StratumId::try_new(value).expect("stratum identity")
    }

    fn close(left: f64, right: f64) {
        assert!((left - right).abs() <= 1.0e-13 * left.abs().max(right.abs()).max(1.0));
    }

    fn exact_positive_zero(value: f64) {
        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
    }

    fn forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.14,
            ground_albedo_nir: 0.31,
            longwave_down_w_m2: 350.0,
            longwave_up_w_m2: 390.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![SoilLayerForcing {
                layer_id: layer_id("soil-1"),
                water_beginning_kg_m2: 20.0,
                matric_potential_mm: -1_000.0,
                hydraulic_conductivity_mm_s: 1.0e-5,
                root_path_length_mm: 100.0,
                gravity_root_mm: 500.0,
                temperature_k: 295.0,
                accessible: true,
                frozen: false,
            }],
            root_zone_hydraulics: None,
            gsi: 1.0,
        }
    }

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let (mut configuration, mut state) = crate::transaction::v7_identity_rebound_fixture();

        let upper = configuration.strata.get_mut(0).expect("upper config");
        upper.stratum_id = stratum_id("upper");
        upper.height_m = 12.5;
        upper.leaf_rho_vis = 0.09;
        upper.leaf_tau_vis = 0.06;
        upper.stem_rho_vis = 0.18;
        upper.stem_tau_vis = 0.03;
        upper.leaf_rho_nir = 0.41;
        upper.leaf_tau_nir = 0.31;
        upper.stem_rho_nir = 0.29;
        upper.stem_tau_nir = 0.12;
        upper.clumping_index = 0.74;
        upper.leaf_angle_chi = 0.12;
        upper.sai_relation = 0.7 / 2.6;

        let mut lower: StratumConfiguration = upper.clone();
        lower.stratum_id = stratum_id("lower");
        lower.vertical_rank = 1;
        lower.height_m = 6.0;
        lower.crown_base_m = 1.0;
        lower.leaf_rho_vis = 0.12;
        lower.leaf_tau_vis = 0.04;
        lower.stem_rho_vis = 0.22;
        lower.stem_tau_vis = 0.02;
        lower.leaf_rho_nir = 0.37;
        lower.leaf_tau_nir = 0.27;
        lower.stem_rho_nir = 0.25;
        lower.stem_tau_nir = 0.10;
        lower.clumping_index = 0.86;
        lower.leaf_angle_chi = -0.08;
        lower.sai_relation = 0.45 / 1.35;
        let upper_sla = upper.sla_m2_per_kg_c;
        let upper_root_to_leaf_area = upper.root_to_leaf_area;
        let lower_sla = lower.sla_m2_per_kg_c;
        let lower_root_to_leaf_area = lower.root_to_leaf_area;
        configuration.strata.push(lower);

        let original_shared: StratumSharedState = state
            .strata
            .remove(&stratum_id("tree-1"))
            .expect("original shared");
        let mut upper_shared = original_shared.clone();
        let upper_leaf = upper_shared
            .tissues
            .get_mut(&crate::carbon_nitrogen::Tissue::Leaf)
            .expect("upper leaf");
        upper_leaf.display.carbon = 2.6 / upper_sla;
        upper_shared.leaf_area = upper_leaf.display.carbon * upper_sla;
        upper_shared.stem_area = upper_shared.leaf_area * (0.7 / 2.6);
        upper_shared.root_area =
            (upper_shared.leaf_area + upper_shared.stem_area) * upper_root_to_leaf_area;
        let mut lower_shared = original_shared;
        let lower_leaf = lower_shared
            .tissues
            .get_mut(&crate::carbon_nitrogen::Tissue::Leaf)
            .expect("lower leaf");
        lower_leaf.display.carbon = 1.35 / lower_sla;
        lower_shared.leaf_area = lower_leaf.display.carbon * lower_sla;
        lower_shared.stem_area = lower_shared.leaf_area * (0.45 / 1.35);
        lower_shared.root_area =
            (lower_shared.leaf_area + lower_shared.stem_area) * lower_root_to_leaf_area;
        state.strata.insert(stratum_id("upper"), upper_shared);
        state.strata.insert(stratum_id("lower"), lower_shared);

        let original_id = OccupancyId {
            stratum_id: stratum_id("tree-1"),
            tile_id: configuration.topology_tiles[0].tile_id.clone(),
        };
        let lane: OccupancyState = state
            .occupancies
            .remove(&original_id)
            .expect("original occupancy");
        for stratum_id in [stratum_id("upper"), stratum_id("lower")] {
            state.occupancies.insert(
                OccupancyId {
                    stratum_id,
                    tile_id: configuration.topology_tiles[0].tile_id.clone(),
                },
                lane.clone(),
            );
        }

        configuration.configuration_sha256.clear();
        configuration.initial_state_sha256.clear();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");
        state.configuration_sha256 = configuration.configuration_sha256.clone();
        state.state_sha256.clear();
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        configuration.initial_state_sha256 = state.state_sha256.clone();
        configuration.validate().expect("configuration");
        state.validate(&configuration).expect("state");
        (configuration, state)
    }

    #[test]
    fn adapter_preserves_exact_occupancy_band_component_and_owner_identity() {
        let (configuration, state) = fixture();
        let prepared = prepare_whole_column_radiation(&configuration, &state, &forcing())
            .expect("whole-column preparation");
        assert_eq!(prepared.occupancies.len(), 2);
        let upper_id = OccupancyId {
            stratum_id: stratum_id("upper"),
            tile_id: configuration.topology_tiles[0].tile_id.clone(),
        };
        let lower_id = OccupancyId {
            stratum_id: stratum_id("lower"),
            tile_id: configuration.topology_tiles[0].tile_id.clone(),
        };
        let upper = prepared.occupancies.get(&upper_id).expect("upper result");
        let lower = prepared.occupancies.get(&lower_id).expect("lower result");
        assert_eq!(upper.occupancy_id, upper_id);
        assert_eq!(lower.occupancy_id, lower_id);
        assert_eq!(upper.visible_direct.band, RadiationBand::Visible);
        assert_eq!(upper.visible_direct.component, IncidentComponent::Direct);
        assert_eq!(
            upper.near_infrared_diffuse.band,
            RadiationBand::NearInfrared
        );
        assert_eq!(
            upper.near_infrared_diffuse.component,
            IncidentComponent::Diffuse
        );
        exact_positive_zero(upper.visible_diffuse.absorption.leaf_sun_area);
        assert!(upper.visible_direct.absorption.absorbed_leaf_sun > 0.0);
        assert!(lower.visible_direct.absorption.absorbed_leaf_sun > 0.0);
        assert!(
            (upper.visible_direct.absorption.absorbed_leaf_sun
                - lower.visible_direct.absorption.absorbed_leaf_sun)
                .abs()
                > 1.0e-3
        );
    }

    #[test]
    fn adapter_uses_conditional_areas_and_never_averages_stratum_optics() {
        let (configuration, state) = fixture();
        let prepared = prepare_whole_column_radiation(&configuration, &state, &forcing())
            .expect("whole-column preparation");
        let tile_id = configuration.topology_tiles[0].tile_id.clone();
        let upper = prepared
            .occupancies
            .get(&OccupancyId {
                stratum_id: stratum_id("upper"),
                tile_id: tile_id.clone(),
            })
            .expect("upper");
        let lower = prepared
            .occupancies
            .get(&OccupancyId {
                stratum_id: stratum_id("lower"),
                tile_id,
            })
            .expect("lower");
        close(upper.conditional_lai_m2_m2_tile_ground, 2.6);
        close(upper.conditional_wai_m2_m2_tile_ground, 0.7);
        close(lower.conditional_lai_m2_m2_tile_ground, 1.35);
        close(lower.conditional_wai_m2_m2_tile_ground, 0.45);
        let expected_upper_rho = (2.6 * 0.09 + 0.7 * 0.18) / 3.3;
        let expected_lower_rho = (1.35 * 0.12 + 0.45 * 0.22) / 1.8;
        close(
            upper.visible_direct.absorption.effective_reflectance,
            expected_upper_rho,
        );
        close(
            lower.visible_direct.absorption.effective_reflectance,
            expected_lower_rho,
        );
        assert!((expected_upper_rho - expected_lower_rho).abs() > 1.0e-3);
        assert!(
            (upper.visible_direct.absorption.effective_reflectance
                - expected_upper_rho.midpoint(expected_lower_rho))
            .abs()
                > 1.0e-3
        );
    }

    #[test]
    fn topology_vector_order_cannot_swap_occupancy_results() {
        let (mut configuration, mut state) = fixture();
        let baseline =
            prepare_whole_column_radiation(&configuration, &state, &forcing()).expect("baseline");
        configuration.strata.reverse();
        configuration.configuration_sha256.clear();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");
        state.configuration_sha256 = configuration.configuration_sha256.clone();
        state.state_sha256.clear();
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        configuration.initial_state_sha256 = state.state_sha256.clone();
        let reordered = prepare_whole_column_radiation(&configuration, &state, &forcing())
            .expect("reordered configuration");
        assert_eq!(baseline.occupancies, reordered.occupancies);
        assert_eq!(baseline.columns, reordered.columns);
    }

    #[test]
    fn invalid_state_identity_fails_before_any_radiation_is_returned() {
        let (configuration, mut state) = fixture();
        let id = state.occupancies.keys().next().expect("occupancy").clone();
        state.occupancies.remove(&id);
        state.state_sha256.clear();
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        let error = prepare_whole_column_radiation(&configuration, &state, &forcing())
            .expect_err("missing occupancy must fail");
        assert!(matches!(
            error,
            VegetationError::Domain(_) | VegetationError::Receipt(_)
        ));
    }

    #[test]
    fn direct_and_diffuse_inputs_cannot_alias() {
        let (configuration, state) = fixture();
        let mut changed = forcing();
        changed.direct_par_w_m2 = 0.0;
        let prepared = prepare_whole_column_radiation(&configuration, &state, &changed)
            .expect("diffuse-only VIS preparation");
        for value in prepared.occupancies.values() {
            exact_positive_zero(value.visible_direct.absorption.absorbed_plant);
            exact_positive_zero(value.visible_direct.absorption.leaf_sun_area);
            assert!(value.visible_diffuse.absorption.absorbed_plant > 0.0);
        }
    }

    #[test]
    fn transaction_identity_is_not_fabricated_by_radiation_preparation() {
        let (configuration, state) = fixture();
        assert_eq!(state.last_transaction_id, 0);
        let prepared = prepare_whole_column_radiation(&configuration, &state, &forcing())
            .expect("preparation");
        assert_eq!(prepared.occupancies.len(), 2);
        assert!(
            state
                .occupancies
                .values()
                .all(|lane| lane.last_accepted_transaction_id.is_none())
        );
    }
}
