#![allow(clippy::float_cmp, clippy::missing_errors_doc)]
// Exact equality is required for digest-bound numerical configuration identity.

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId};
use serde::{Deserialize, Serialize};

use crate::{
    LandSurfaceEnergyError, MODEL_DEFINITION_SHA256, MODEL_VERSION, OfeId, Sha256Digest,
    VEGETATION_MODEL_DEFINITION_SHA256, VEGETATION_MODEL_VERSION, canonical_digest,
    require_finite_nonnegative, require_finite_positive,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerConfigurationRef {
    pub owner_id: ResourceOwnerId,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumericalConfiguration {
    pub iteration_limit: u32,
    pub backtracking_exponents: Vec<u32>,
    pub finite_difference: String,
    pub pivot_threshold: String,
    pub equal_pivot_rule: String,
    pub temperature_bounds_k: [u16; 2],
    pub humidity_bounds_kg_kg: [f64; 2],
    pub temperature_step_tolerance_k: f64,
    pub humidity_step_tolerance_kg_kg: f64,
    pub hydraulic_step_tolerance_mm: f64,
    pub beta_step_tolerance: f64,
}

impl NumericalConfiguration {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        let expected_backtracking: Vec<u32> = (0..=20).collect();
        let exact = self.iteration_limit == 50
            && self.backtracking_exponents == expected_backtracking
            && self.finite_difference == "centered_sqrt_binary64_epsilon_minus_then_plus"
            && self.pivot_threshold == "64_times_binary64_epsilon_times_matrix_infinity_norm"
            && self.equal_pivot_rule == "lowest_row"
            && self.temperature_bounds_k == [200, 350]
            && self.humidity_bounds_kg_kg == [0.0, 0.1]
            && self.temperature_step_tolerance_k == 1.0e-8
            && self.humidity_step_tolerance_kg_kg == 1.0e-12
            && self.hydraulic_step_tolerance_mm == 1.0e-7
            && self.beta_step_tolerance == 1.0e-10;
        if exact {
            Ok(())
        } else {
            Err(LandSurfaceEnergyError::Identity {
                field: "numerics",
                expected: "OPENWEPP_SNOW_FREE_LSE_V1 frozen numerical constants".into(),
                found: "noncanonical numerical configuration".into(),
            })
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilInterfaceLayer {
    pub layer_id: SoilLayerId,
    pub thickness_m: f64,
    pub thermal_conductivity_w_m_k: f64,
    pub areal_heat_capacity_j_m2_k: f64,
}

impl SoilInterfaceLayer {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        require_finite_positive(self.thickness_m, "soil_interface_layer.thickness_m")?;
        require_finite_positive(
            self.thermal_conductivity_w_m_k,
            "soil_interface_layer.thermal_conductivity_w_m_k",
        )?;
        require_finite_positive(
            self.areal_heat_capacity_j_m2_k,
            "soil_interface_layer.areal_heat_capacity_j_m2_k",
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceHeatStorageMode {
    FiniteCapacity,
    EquilibriumZero,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum TurbulenceConfiguration {
    OpenNeutral {
        reference_height_m: f64,
        roughness_momentum_m: f64,
        roughness_heat_m: f64,
        roughness_vapor_m: f64,
    },
    CoveredNeutral {
        canopy_height_m: f64,
        ground_exchange_roughness_m: f64,
        leaf_area_index_m2_m2_tile_ground: f64,
        canopy_to_reference: CanopyReferenceGeometry,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanopyReferenceGeometry {
    pub reference_height_m: f64,
    pub displacement_m: f64,
    pub roughness_momentum_m: f64,
    pub roughness_heat_m: f64,
    pub roughness_vapor_m: f64,
}

impl TurbulenceConfiguration {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        match self {
            Self::OpenNeutral {
                reference_height_m,
                roughness_momentum_m,
                roughness_heat_m,
                roughness_vapor_m,
            } => {
                for (value, name) in [
                    (*reference_height_m, "open.reference_height_m"),
                    (*roughness_momentum_m, "open.roughness_momentum_m"),
                    (*roughness_heat_m, "open.roughness_heat_m"),
                    (*roughness_vapor_m, "open.roughness_vapor_m"),
                ] {
                    require_finite_positive(value, name)?;
                }
                if *reference_height_m
                    <= roughness_momentum_m
                        .max(*roughness_heat_m)
                        .max(*roughness_vapor_m)
                {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "open reference height and roughness ordering",
                    ));
                }
            }
            Self::CoveredNeutral {
                canopy_height_m,
                ground_exchange_roughness_m,
                leaf_area_index_m2_m2_tile_ground,
                canopy_to_reference,
            } => {
                for (value, name) in [
                    (*canopy_height_m, "covered.canopy_height_m"),
                    (
                        *ground_exchange_roughness_m,
                        "covered.ground_exchange_roughness_m",
                    ),
                    (
                        *leaf_area_index_m2_m2_tile_ground,
                        "covered.leaf_area_index",
                    ),
                    (
                        canopy_to_reference.reference_height_m,
                        "covered.reference_height_m",
                    ),
                    (
                        canopy_to_reference.roughness_momentum_m,
                        "covered.roughness_momentum_m",
                    ),
                    (
                        canopy_to_reference.roughness_heat_m,
                        "covered.roughness_heat_m",
                    ),
                    (
                        canopy_to_reference.roughness_vapor_m,
                        "covered.roughness_vapor_m",
                    ),
                ] {
                    require_finite_positive(value, name)?;
                }
                require_finite_nonnegative(
                    canopy_to_reference.displacement_m,
                    "covered.displacement_m",
                )?;
                let largest_roughness = canopy_to_reference
                    .roughness_momentum_m
                    .max(canopy_to_reference.roughness_heat_m)
                    .max(canopy_to_reference.roughness_vapor_m);
                if canopy_to_reference.reference_height_m
                    <= canopy_to_reference.displacement_m + largest_roughness
                {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "covered reference height and roughness ordering",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "surface_class", rename_all = "snake_case", deny_unknown_fields)]
pub enum SurfaceConfiguration {
    BareMineralSoil {
        dry_areal_heat_capacity_j_m2_k: f64,
        mineral_skin_thickness_m: f64,
        mineral_skin_thermal_conductivity_w_m_k: f64,
        top_layer_saturated_water_content_m3_m3: f64,
        top_layer_porosity_m3_m3: f64,
        top_layer_saturated_matric_potential_mm: f64,
        top_layer_clapp_hornberger_b: f64,
        top_layer_initial_water_content_m3_m3: f64,
    },
    ForestLitter {
        liquid_capacity_kg_m2_tile_ground: f64,
        thickness_m: f64,
        dry_density_kg_m3: f64,
        dry_specific_heat_j_kg_k: f64,
    },
}

impl SurfaceConfiguration {
    fn validate(&self, mode: SurfaceHeatStorageMode) -> Result<(), LandSurfaceEnergyError> {
        match self {
            Self::BareMineralSoil {
                dry_areal_heat_capacity_j_m2_k,
                mineral_skin_thickness_m,
                mineral_skin_thermal_conductivity_w_m_k,
                top_layer_saturated_water_content_m3_m3,
                top_layer_porosity_m3_m3,
                top_layer_saturated_matric_potential_mm,
                top_layer_clapp_hornberger_b,
                top_layer_initial_water_content_m3_m3,
            } => {
                require_finite_nonnegative(
                    *dry_areal_heat_capacity_j_m2_k,
                    "bare.dry_areal_heat_capacity_j_m2_k",
                )?;
                for (value, name) in [
                    (*mineral_skin_thickness_m, "bare.mineral_skin_thickness_m"),
                    (
                        *mineral_skin_thermal_conductivity_w_m_k,
                        "bare.skin_conductivity",
                    ),
                    (*top_layer_clapp_hornberger_b, "bare.clapp_hornberger_b"),
                ] {
                    require_finite_positive(value, name)?;
                }
                for (value, name) in [
                    (*top_layer_saturated_water_content_m3_m3, "bare.theta_sat"),
                    (*top_layer_porosity_m3_m3, "bare.porosity"),
                ] {
                    require_finite_positive(value, name)?;
                    if value > 1.0 {
                        return Err(LandSurfaceEnergyError::ConstitutiveDomain(name));
                    }
                }
                require_finite_nonnegative(
                    *top_layer_initial_water_content_m3_m3,
                    "bare.theta_init",
                )?;
                if *top_layer_initial_water_content_m3_m3 > 1.0 {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "bare.theta_init",
                    ));
                }
                if !top_layer_saturated_matric_potential_mm.is_finite()
                    || *top_layer_saturated_matric_potential_mm >= 0.0
                {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain("bare.psi_sat"));
                }
                if mode == SurfaceHeatStorageMode::EquilibriumZero
                    && *dry_areal_heat_capacity_j_m2_k != 0.0
                {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "equilibrium-zero dry heat capacity",
                    ));
                }
            }
            Self::ForestLitter {
                liquid_capacity_kg_m2_tile_ground,
                thickness_m,
                dry_density_kg_m3,
                dry_specific_heat_j_kg_k,
            } => {
                if mode == SurfaceHeatStorageMode::EquilibriumZero {
                    return Err(LandSurfaceEnergyError::UnsupportedDomain(
                        "equilibrium-zero forest litter",
                    ));
                }
                for (value, name) in [
                    (*liquid_capacity_kg_m2_tile_ground, "litter.liquid_capacity"),
                    (*thickness_m, "litter.thickness_m"),
                    (*dry_density_kg_m3, "litter.dry_density"),
                    (*dry_specific_heat_j_kg_k, "litter.dry_specific_heat"),
                ] {
                    require_finite_positive(value, name)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TileConfiguration {
    pub tile_id: TileId,
    pub fraction_ofe_ground: f64,
    pub vegetation_tile_id: TileId,
    pub surface_vis_albedo: f64,
    pub surface_nir_albedo: f64,
    pub surface_heat_storage_mode: SurfaceHeatStorageMode,
    pub turbulence: TurbulenceConfiguration,
    pub surface: SurfaceConfiguration,
}

impl TileConfiguration {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        require_finite_positive(self.fraction_ofe_ground, "tile.fraction_ofe_ground")?;
        if self.fraction_ofe_ground > 1.0 {
            return Err(LandSurfaceEnergyError::Topology("tile fraction above one"));
        }
        for (value, name) in [
            (self.surface_vis_albedo, "tile.surface_vis_albedo"),
            (self.surface_nir_albedo, "tile.surface_nir_albedo"),
        ] {
            require_finite_nonnegative(value, name)?;
            if value > 1.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(name));
            }
        }
        self.turbulence.validate()?;
        self.surface.validate(self.surface_heat_storage_mode)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OfeConfiguration {
    pub ofe_id: OfeId,
    pub area_m2: f64,
    pub soil_interface_layers: Vec<SoilInterfaceLayer>,
    pub tiles: Vec<TileConfiguration>,
}

impl OfeConfiguration {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        require_finite_positive(self.area_m2, "ofe.area_m2")?;
        if self.soil_interface_layers.is_empty() || self.tiles.is_empty() {
            return Err(LandSurfaceEnergyError::Topology(
                "empty OFE layer or tile set",
            ));
        }
        let mut layers = BTreeSet::new();
        for layer in &self.soil_interface_layers {
            if !layers.insert(layer.layer_id.clone()) {
                return Err(LandSurfaceEnergyError::Topology("duplicate soil layer"));
            }
            layer.validate()?;
        }
        let mut tiles = BTreeSet::new();
        let mut fraction_sum = 0.0;
        for tile in &self.tiles {
            if !tiles.insert(tile.tile_id.clone()) {
                return Err(LandSurfaceEnergyError::Topology("duplicate tile"));
            }
            tile.validate()?;
            fraction_sum += tile.fraction_ofe_ground;
        }
        let tolerance = 64.0 * f64::EPSILON * fraction_sum.abs().max(1.0);
        if (fraction_sum - 1.0).abs() > tolerance {
            return Err(LandSurfaceEnergyError::Topology(
                "tile fractions do not sum to one",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LandSurfaceEnergyConfiguration {
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub owner_id: ResourceOwnerId,
    pub vegetation_configuration: OwnerConfigurationRef,
    pub hydrology_configuration: OwnerConfigurationRef,
    pub soil_thermal_configuration: OwnerConfigurationRef,
    pub numerics: NumericalConfiguration,
    pub ofes: Vec<OfeConfiguration>,
}

impl LandSurfaceEnergyConfiguration {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.model_version != MODEL_VERSION {
            return Err(LandSurfaceEnergyError::Identity {
                field: "model_version",
                expected: MODEL_VERSION.into(),
                found: self.model_version.clone(),
            });
        }
        if self.model_definition_sha256.as_str() != MODEL_DEFINITION_SHA256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "model_definition_sha256",
                expected: MODEL_DEFINITION_SHA256.into(),
                found: self.model_definition_sha256.to_string(),
            });
        }
        if self.vegetation_configuration.model_version != VEGETATION_MODEL_VERSION
            || self
                .vegetation_configuration
                .model_definition_sha256
                .as_str()
                != VEGETATION_MODEL_DEFINITION_SHA256
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "vegetation_configuration",
                expected: format!(
                    "{VEGETATION_MODEL_VERSION}/{VEGETATION_MODEL_DEFINITION_SHA256}"
                ),
                found: format!(
                    "{}/{}",
                    self.vegetation_configuration.model_version,
                    self.vegetation_configuration.model_definition_sha256
                ),
            });
        }
        self.numerics.validate()?;
        if self.ofes.is_empty() {
            return Err(LandSurfaceEnergyError::Topology("empty OFE set"));
        }
        let mut ofes = BTreeSet::new();
        for ofe in &self.ofes {
            if !ofes.insert(ofe.ofe_id.clone()) {
                return Err(LandSurfaceEnergyError::Topology("duplicate OFE"));
            }
            ofe.validate()?;
        }
        let computed = self.canonical_sha256()?;
        if self.configuration_sha256 != computed {
            return Err(LandSurfaceEnergyError::Identity {
                field: "configuration_sha256",
                expected: computed.to_string(),
                found: self.configuration_sha256.to_string(),
            });
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, LandSurfaceEnergyError> {
        let mut value = serde_json::to_value(self)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        let digest = value.get_mut("configuration_sha256").ok_or(
            LandSurfaceEnergyError::MalformedSerialization(
                "configuration_sha256 absent from serialized configuration".into(),
            ),
        )?;
        *digest = serde_json::Value::String(String::new());
        if let Some(lower) = value
            .get_mut("numerics")
            .and_then(|numerics| numerics.get_mut("humidity_bounds_kg_kg"))
            .and_then(|bounds| bounds.get_mut(0))
        {
            *lower = serde_json::Value::from(0);
        }
        canonical_digest(&value)
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, LandSurfaceEnergyError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        value.validate()?;
        Ok(value)
    }
}
