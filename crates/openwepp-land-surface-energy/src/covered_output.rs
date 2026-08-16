//! Accepted component outputs from a covered-column residual evaluation.

use crate::physics::CanopyLongwaveResult;
use crate::{CoveredOccupancyLiquidLedger, GroundWaterFlux, SourceWaterFlux, WaterBranch};

/// Per-occupancy hydraulic, energy, carbon, and E04 operands.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyEvaluation {
    pub residuals: Vec<f64>,
    pub tolerances: Vec<f64>,
    pub source_water: Vec<SourceWaterFlux>,
    pub canopy_sensible_w_m2: f64,
    pub canopy_vapor_kg_m2_s: f64,
    pub wet_vapor_kg_m2_s: f64,
    pub wet_branch: WaterBranch,
    pub component_temperatures_k: [f64; 4],
    pub ci_pa: [f64; 2],
    /// Accepted class-resolved `[sun, shade]` `FvCB` carbon operands.
    pub gross_assimilation_umol_co2_m2_leaf_s: [f64; 2],
    pub net_assimilation_umol_co2_m2_leaf_s: [f64; 2],
    pub dark_respiration_umol_co2_m2_leaf_s: [f64; 2],
    pub liquid: CoveredOccupancyLiquidLedger,
}

/// Complete whole-column residual evaluation and accepted component operands.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnEvaluation {
    pub raw_residuals: Vec<f64>,
    pub normalized_residuals: Vec<f64>,
    pub tolerances: Vec<f64>,
    pub occupancies: Vec<CoveredOccupancyEvaluation>,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub ground_temperature_k: f64,
    pub soil_temperature_k: Vec<f64>,
    pub ground_water: GroundWaterFlux,
    pub ground_heat_cn_w_m2_tile: Vec<f64>,
    pub ground_storage_w_m2_tile: f64,
    pub ending_surface_enthalpy_j_m2_tile: f64,
    pub whole_column_longwave: CanopyLongwaveResult,
    pub ground_canopy_release_kg_m2_tile: f64,
    pub ground_stemflow_kg_m2_tile: f64,
}
