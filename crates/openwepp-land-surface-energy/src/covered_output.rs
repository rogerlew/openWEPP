//! Accepted component outputs from a covered-column residual evaluation.

use std::collections::BTreeSet;

use crate::physics::{BandDirectionalFluxes, CanopyLongwaveResult};
use crate::{
    CoveredOccupancyLiquidLedger, CoveredOccupancyShortwaveInputs, GroundWaterFlux,
    LandSurfaceEnergyError, Sha256Digest, SourceWaterFlux, V10LeafGasBranch, WaterBranch,
    energy_tolerance, water_tolerance,
};

/// Primitive energy terms for one named canopy surface. No producer residual
/// is retained; the receiving owner reconstructs it from these terms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoveredSurfaceEnergyOperands {
    pub absorbed_shortwave_w_m2_tile: f64,
    pub net_longwave_w_m2_tile: f64,
    pub sensible_to_canopy_air_w_m2_tile: f64,
    pub signed_vapor_to_canopy_air_kg_m2_tile_s: f64,
    pub surface_temperature_k: f64,
    pub latent_heat_j_kg: f64,
}

impl CoveredSurfaceEnergyOperands {
    /// # Errors
    /// Returns a typed domain error when the operand is non-finite or invalid.
    pub fn validate(self) -> Result<(), LandSurfaceEnergyError> {
        let values = [
            self.absorbed_shortwave_w_m2_tile,
            self.net_longwave_w_m2_tile,
            self.sensible_to_canopy_air_w_m2_tile,
            self.signed_vapor_to_canopy_air_kg_m2_tile_s,
            self.surface_temperature_k,
            self.latent_heat_j_kg,
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err(LandSurfaceEnergyError::NonFinite(
                "covered surface energy operand",
            ));
        }
        if self.absorbed_shortwave_w_m2_tile < 0.0
            || self.surface_temperature_k <= 0.0
            || self.latent_heat_j_kg <= 0.0
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered surface energy domain",
            ));
        }
        let latent = self.latent_heat_j_kg * self.signed_vapor_to_canopy_air_kg_m2_tile_s;
        let residual = self.absorbed_shortwave_w_m2_tile + self.net_longwave_w_m2_tile
            - self.sensible_to_canopy_air_w_m2_tile
            - latent;
        let scale = self.absorbed_shortwave_w_m2_tile.abs()
            + self.net_longwave_w_m2_tile.abs()
            + self.sensible_to_canopy_air_w_m2_tile.abs()
            + latent.abs();
        if residual.abs() > energy_tolerance(scale) {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered surface energy",
            ));
        }
        Ok(())
    }
}

/// Four independently owned energy surfaces for one exact occupancy.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyEnergyOperands {
    pub occupancy_id: String,
    pub sun_leaf: CoveredSurfaceEnergyOperands,
    pub shade_leaf: CoveredSurfaceEnergyOperands,
    pub wet_surface: CoveredSurfaceEnergyOperands,
    pub dry_stem: CoveredSurfaceEnergyOperands,
}

impl CoveredOccupancyEnergyOperands {
    /// # Errors
    /// Returns a typed domain error when an occupancy operand is invalid.
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.occupancy_id.is_empty() || self.occupancy_id.trim() != self.occupancy_id {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered occupancy energy identity",
            ));
        }
        self.sun_leaf.validate()?;
        self.shade_leaf.validate()?;
        self.wet_surface.validate()?;
        self.dry_stem.validate()?;
        if self
            .dry_stem
            .signed_vapor_to_canopy_air_kg_m2_tile_s
            .to_bits()
            != 0.0_f64.to_bits()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "dry stem vapor ownership",
            ));
        }
        Ok(())
    }

    fn surfaces(&self) -> [CoveredSurfaceEnergyOperands; 4] {
        [
            self.sun_leaf,
            self.shade_leaf,
            self.wet_surface,
            self.dry_stem,
        ]
    }
}

/// Exact shortwave handoff retained by band and direction. Canopy absorption
/// remains component owned while the ground terminal handoff is partitioned
/// independently into absorbed and reflected terms.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnShortwaveOperands {
    pub incident_w_m2_tile: BandDirectionalFluxes,
    pub top_reflected_w_m2_tile: BandDirectionalFluxes,
    pub ground_absorbed_by_incident_w_m2_tile: BandDirectionalFluxes,
    pub ground_terminal_w_m2_tile: BandDirectionalFluxes,
    pub ground_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub ground_reflected_w_m2_tile: BandDirectionalFluxes,
    pub occupancies: Vec<CoveredOccupancyShortwaveInputs>,
}

impl CoveredColumnShortwaveOperands {
    /// # Errors
    /// Returns a typed domain error when the column operands do not close.
    pub fn validate(
        &self,
        occupancies: &[CoveredOccupancyEnergyOperands],
    ) -> Result<(), LandSurfaceEnergyError> {
        self.ground_terminal_w_m2_tile.validate_nonnegative()?;
        self.incident_w_m2_tile.validate_nonnegative()?;
        self.top_reflected_w_m2_tile.validate_nonnegative()?;
        self.ground_absorbed_by_incident_w_m2_tile
            .validate_nonnegative()?;
        self.ground_absorbed_w_m2_tile.validate_nonnegative()?;
        self.ground_reflected_w_m2_tile.validate_nonnegative()?;
        for (terminal, absorbed, reflected) in band_values(
            self.ground_terminal_w_m2_tile,
            self.ground_absorbed_w_m2_tile,
            self.ground_reflected_w_m2_tile,
        ) {
            if (terminal - absorbed - reflected).abs()
                > energy_tolerance(terminal.abs() + absorbed.abs() + reflected.abs())
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "covered band/direction shortwave ownership",
                ));
            }
        }
        if self.occupancies.len() != occupancies.len() {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered occupancy shortwave cardinality",
            ));
        }
        for (radiation, occupancy) in self.occupancies.iter().zip(occupancies) {
            radiation
                .sun_leaf_absorbed_w_m2_tile
                .validate_nonnegative()?;
            radiation
                .shade_leaf_absorbed_w_m2_tile
                .validate_nonnegative()?;
            radiation.stem_absorbed_w_m2_tile.validate_nonnegative()?;
            let sun_total = radiation.sun_leaf_absorbed_w_m2_tile.total();
            let shade_total = radiation.shade_leaf_absorbed_w_m2_tile.total();
            let stem_total = radiation.stem_absorbed_w_m2_tile.total();
            let expected_wet = sun_total - occupancy.sun_leaf.absorbed_shortwave_w_m2_tile
                + shade_total
                - occupancy.shade_leaf.absorbed_shortwave_w_m2_tile
                + stem_total
                - occupancy.dry_stem.absorbed_shortwave_w_m2_tile;
            if radiation.occupancy_id != occupancy.occupancy_id
                || occupancy.sun_leaf.absorbed_shortwave_w_m2_tile > sun_total
                || occupancy.shade_leaf.absorbed_shortwave_w_m2_tile > shade_total
                || occupancy.dry_stem.absorbed_shortwave_w_m2_tile > stem_total
                || (expected_wet - occupancy.wet_surface.absorbed_shortwave_w_m2_tile).abs()
                    > energy_tolerance(
                        expected_wet.abs()
                            + occupancy.wet_surface.absorbed_shortwave_w_m2_tile.abs(),
                    )
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "covered leaf/stem shortwave ownership",
                ));
            }
        }
        let incident = directional(self.incident_w_m2_tile);
        let top_reflected = directional(self.top_reflected_w_m2_tile);
        let ground_absorbed = directional(self.ground_absorbed_by_incident_w_m2_tile);
        for index in 0..4 {
            let canopy_absorbed: f64 = self
                .occupancies
                .iter()
                .map(|occupancy| {
                    directional(occupancy.sun_leaf_absorbed_w_m2_tile)[index]
                        + directional(occupancy.shade_leaf_absorbed_w_m2_tile)[index]
                        + directional(occupancy.stem_absorbed_w_m2_tile)[index]
                })
                .sum();
            let residual =
                incident[index] - top_reflected[index] - canopy_absorbed - ground_absorbed[index];
            if residual.abs()
                > energy_tolerance(
                    incident[index].abs()
                        + top_reflected[index].abs()
                        + canopy_absorbed.abs()
                        + ground_absorbed[index].abs(),
                )
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "covered whole-column band/direction shortwave ownership",
                ));
            }
        }
        let physical_ground_absorbed = self.ground_absorbed_w_m2_tile.total();
        let attributed_ground_absorbed = self.ground_absorbed_by_incident_w_m2_tile.total();
        if (physical_ground_absorbed - attributed_ground_absorbed).abs()
            > energy_tolerance(physical_ground_absorbed.abs() + attributed_ground_absorbed.abs())
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered ground shortwave attribution",
            ));
        }
        Ok(())
    }
}

/// Primitive reciprocal-longwave boundaries and component ownership.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnLongwaveOperands {
    pub atmospheric_downward_w_m2_tile: f64,
    pub transmissivities: Vec<f64>,
    pub downward_boundaries_w_m2_tile: Vec<f64>,
    pub upward_boundaries_w_m2_tile: Vec<f64>,
    pub top_upward_w_m2_tile: f64,
    pub ground_net_w_m2_tile: f64,
    pub occupancy_component_net_w_m2_tile: Vec<(String, [f64; 4])>,
}

impl CoveredColumnLongwaveOperands {
    fn validate(
        &self,
        occupancies: &[CoveredOccupancyEnergyOperands],
    ) -> Result<(), LandSurfaceEnergyError> {
        let count = occupancies.len();
        if !self.atmospheric_downward_w_m2_tile.is_finite()
            || !self.top_upward_w_m2_tile.is_finite()
            || !self.ground_net_w_m2_tile.is_finite()
            || self.transmissivities.len() != count
            || self.downward_boundaries_w_m2_tile.len() != count + 1
            || self.upward_boundaries_w_m2_tile.len() != count + 1
            || self.occupancy_component_net_w_m2_tile.len() != count
            || self
                .transmissivities
                .iter()
                .any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))
            || self
                .downward_boundaries_w_m2_tile
                .iter()
                .chain(&self.upward_boundaries_w_m2_tile)
                .any(|value| !value.is_finite())
            || self.downward_boundaries_w_m2_tile[0].to_bits()
                != self.atmospheric_downward_w_m2_tile.to_bits()
            || self.upward_boundaries_w_m2_tile[0].to_bits() != self.top_upward_w_m2_tile.to_bits()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered longwave topology",
            ));
        }
        let mut component_total = 0.0;
        for (index, ((identity, values), occupancy)) in self
            .occupancy_component_net_w_m2_tile
            .iter()
            .zip(occupancies)
            .enumerate()
        {
            if identity != &occupancy.occupancy_id
                || values.iter().any(|value| !value.is_finite())
                || !values
                    .iter()
                    .zip(occupancy.surfaces())
                    .all(|(value, surface)| {
                        value.to_bits() == surface.net_longwave_w_m2_tile.to_bits()
                    })
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "covered longwave component ownership",
                ));
            }
            let layer_total = values.iter().sum::<f64>();
            let boundary_reconstruction = self.downward_boundaries_w_m2_tile[index]
                - self.downward_boundaries_w_m2_tile[index + 1]
                + self.upward_boundaries_w_m2_tile[index + 1]
                - self.upward_boundaries_w_m2_tile[index];
            if (layer_total - boundary_reconstruction).abs()
                > energy_tolerance(layer_total.abs() + boundary_reconstruction.abs())
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "covered reciprocal longwave layer",
                ));
            }
            component_total += layer_total;
        }
        let terminal_ground_net =
            self.downward_boundaries_w_m2_tile[count] - self.upward_boundaries_w_m2_tile[count];
        if (terminal_ground_net - self.ground_net_w_m2_tile).abs()
            > energy_tolerance(terminal_ground_net.abs() + self.ground_net_w_m2_tile.abs())
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered reciprocal longwave ground",
            ));
        }
        let residual = self.atmospheric_downward_w_m2_tile
            - self.top_upward_w_m2_tile
            - self.ground_net_w_m2_tile
            - component_total;
        if residual.abs()
            > energy_tolerance(
                self.atmospheric_downward_w_m2_tile.abs()
                    + self.top_upward_w_m2_tile.abs()
                    + self.ground_net_w_m2_tile.abs()
                    + component_total.abs(),
            )
        {
            return Err(LandSurfaceEnergyError::ControlVolumeClosure(
                "covered whole-column longwave",
            ));
        }
        Ok(())
    }
}

/// Shared zero-storage canopy-air node. Component and ground transfers are
/// reconstructed independently against the reference-atmosphere exchange.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoveredCanopyAirEnergyOperands {
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub ground_sensible_to_canopy_air_w_m2_tile: f64,
    pub ground_vapor_to_canopy_air_kg_m2_tile_s: f64,
    pub sensible_to_reference_air_w_m2_tile: f64,
    pub vapor_to_reference_air_kg_m2_tile_s: f64,
}

/// Complete covered-column energy receipt supplementing the ground surface,
/// latent-water join, and equal/opposite ground-heat receipt.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnEnergyOperands {
    pub occupancies: Vec<CoveredOccupancyEnergyOperands>,
    pub canopy_air: CoveredCanopyAirEnergyOperands,
    pub shortwave: CoveredColumnShortwaveOperands,
    pub longwave: CoveredColumnLongwaveOperands,
    /// Energy transferred from the canopy-only control volume into the
    /// Stage-3 snow owner. Historical covered columns keep this at zero.
    pub stage3_lower_boundary_energy_w_m2_tile: f64,
    pub optical_receipt_sha256: Option<Sha256Digest>,
    pub reciprocal_longwave_receipt_sha256: Option<Sha256Digest>,
    pub final_canopy_boundary_receipt_sha256: Option<Sha256Digest>,
}

impl CoveredColumnEnergyOperands {
    /// # Errors
    /// Returns a typed domain error when any published operand is invalid.
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if !self.stage3_lower_boundary_energy_w_m2_tile.is_finite() {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered Stage-3 lower-boundary energy",
            ));
        }
        if self.occupancies.is_empty() {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "empty covered occupancy energy set",
            ));
        }
        let mut identities = BTreeSet::new();
        for occupancy in &self.occupancies {
            occupancy.validate()?;
            if !identities.insert(&occupancy.occupancy_id) {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "duplicate covered occupancy energy identity",
                ));
            }
        }
        self.shortwave.validate(&self.occupancies)?;
        self.longwave.validate(&self.occupancies)?;
        let air = self.canopy_air;
        if [
            air.canopy_air_temperature_k,
            air.canopy_air_specific_humidity_kg_kg,
            air.ground_sensible_to_canopy_air_w_m2_tile,
            air.ground_vapor_to_canopy_air_kg_m2_tile_s,
            air.sensible_to_reference_air_w_m2_tile,
            air.vapor_to_reference_air_kg_m2_tile_s,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || air.canopy_air_temperature_k <= 0.0
            || air.canopy_air_specific_humidity_kg_kg < 0.0
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered canopy-air operand domain",
            ));
        }
        let canopy_sensible: f64 = self
            .occupancies
            .iter()
            .flat_map(CoveredOccupancyEnergyOperands::surfaces)
            .map(|surface| surface.sensible_to_canopy_air_w_m2_tile)
            .sum();
        let canopy_vapor: f64 = self
            .occupancies
            .iter()
            .flat_map(CoveredOccupancyEnergyOperands::surfaces)
            .map(|surface| surface.signed_vapor_to_canopy_air_kg_m2_tile_s)
            .sum();
        let heat_residual = canopy_sensible + air.ground_sensible_to_canopy_air_w_m2_tile
            - air.sensible_to_reference_air_w_m2_tile;
        let vapor_residual = canopy_vapor + air.ground_vapor_to_canopy_air_kg_m2_tile_s
            - air.vapor_to_reference_air_kg_m2_tile_s;
        if heat_residual.abs()
            > energy_tolerance(
                canopy_sensible.abs()
                    + air.ground_sensible_to_canopy_air_w_m2_tile.abs()
                    + air.sensible_to_reference_air_w_m2_tile.abs(),
            )
            || vapor_residual.abs()
                > water_tolerance(
                    canopy_vapor
                        .abs()
                        .max(air.ground_vapor_to_canopy_air_kg_m2_tile_s.abs())
                        .max(air.vapor_to_reference_air_kg_m2_tile_s.abs()),
                )
        {
            return Err(LandSurfaceEnergyError::ControlVolumeClosure(
                "covered canopy-air heat/vapor",
            ));
        }
        Ok(())
    }
}

fn directional(value: BandDirectionalFluxes) -> [f64; 4] {
    [
        value.direct_vis,
        value.diffuse_vis,
        value.direct_nir,
        value.diffuse_nir,
    ]
}

fn band_values(
    terminal: BandDirectionalFluxes,
    absorbed: BandDirectionalFluxes,
    reflected: BandDirectionalFluxes,
) -> [(f64, f64, f64); 4] {
    [
        (
            terminal.direct_vis,
            absorbed.direct_vis,
            reflected.direct_vis,
        ),
        (
            terminal.diffuse_vis,
            absorbed.diffuse_vis,
            reflected.diffuse_vis,
        ),
        (
            terminal.direct_nir,
            absorbed.direct_nir,
            reflected.direct_nir,
        ),
        (
            terminal.diffuse_nir,
            absorbed.diffuse_nir,
            reflected.diffuse_nir,
        ),
    ]
}

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
    pub gas_branches: [V10LeafGasBranch; 2],
    /// Accepted class-resolved `[sun, shade]` `FvCB` carbon operands.
    pub gross_assimilation_umol_co2_m2_leaf_s: [f64; 2],
    pub net_assimilation_umol_co2_m2_leaf_s: [f64; 2],
    pub dark_respiration_umol_co2_m2_leaf_s: [f64; 2],
    /// Internal beta=1 maximum-demand evaluation for `[sun, shade]`.
    pub emax_kg_m2_s: [f64; 2],
    pub liquid: CoveredOccupancyLiquidLedger,
    /// Ordered sun-leaf, shade-leaf, wet-surface, dry-stem primitive terms.
    pub absorbed_shortwave_w_m2: [f64; 4],
    pub net_longwave_w_m2: [f64; 4],
    pub sensible_to_canopy_air_w_m2: [f64; 4],
    pub signed_vapor_to_canopy_air_kg_m2_s: [f64; 4],
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
    pub ground_sensible_to_canopy_air_w_m2: f64,
    pub lower_boundary_vapor_to_canopy_air_kg_m2_s: f64,
    pub sensible_to_reference_air_w_m2: f64,
    pub vapor_to_reference_air_kg_m2_s: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CoveredLowerBoundaryEnergyOperands, CoveredTileEnergyOperandSet, GroundHeatJoinOperands,
        LatentJoinOperands, OfeId, Sha256Digest, Stage3LowerBoundaryEnergyOperands,
        Stage3SnowOpticalBoundaryReceiptInputs, Stage3SnowOpticalBoundaryReceiptV1,
        SurfaceEnergyOperands, TileEnergyOperandSet, vapor_export_w_m2,
    };
    use openwepp_kernel_contract::TileId;

    const LATENT: f64 = 2_500_000.0;

    fn surface(
        shortwave: f64,
        longwave: f64,
        vapor: f64,
        temperature: f64,
    ) -> CoveredSurfaceEnergyOperands {
        CoveredSurfaceEnergyOperands {
            absorbed_shortwave_w_m2_tile: shortwave,
            net_longwave_w_m2_tile: longwave,
            sensible_to_canopy_air_w_m2_tile: shortwave + longwave - LATENT * vapor,
            signed_vapor_to_canopy_air_kg_m2_tile_s: vapor,
            surface_temperature_k: temperature,
            latent_heat_j_kg: LATENT,
        }
    }

    fn valid_column() -> CoveredColumnEnergyOperands {
        let occupancy = CoveredOccupancyEnergyOperands {
            occupancy_id: "canopy-rank-0".into(),
            sun_leaf: surface(10.0, -2.0, 2.0e-6, 296.0),
            shade_leaf: surface(20.0, -3.0, 3.0e-6, 295.0),
            wet_surface: surface(7.0, -4.0, 4.0e-6, 294.0),
            dry_stem: surface(30.0, -5.0, 0.0, 293.0),
        };
        let canopy_sensible: f64 = occupancy
            .surfaces()
            .iter()
            .map(|surface| surface.sensible_to_canopy_air_w_m2_tile)
            .sum();
        let ground_terminal = BandDirectionalFluxes {
            direct_vis: 1.0,
            diffuse_vis: 2.0,
            direct_nir: 3.0,
            diffuse_nir: 4.0,
        };
        let ground_absorbed = BandDirectionalFluxes {
            direct_vis: 0.5,
            diffuse_vis: 1.0,
            direct_nir: 1.5,
            diffuse_nir: 2.0,
        };
        let ground_reflected = BandDirectionalFluxes {
            direct_vis: 0.5,
            diffuse_vis: 1.0,
            direct_nir: 1.5,
            diffuse_nir: 2.0,
        };
        let radiation = CoveredOccupancyShortwaveInputs {
            occupancy_id: occupancy.occupancy_id.clone(),
            sun_leaf_absorbed_w_m2_tile: BandDirectionalFluxes {
                direct_vis: 12.0,
                ..BandDirectionalFluxes::default()
            },
            shade_leaf_absorbed_w_m2_tile: BandDirectionalFluxes {
                diffuse_vis: 22.0,
                ..BandDirectionalFluxes::default()
            },
            stem_absorbed_w_m2_tile: BandDirectionalFluxes {
                direct_nir: 33.0,
                ..BandDirectionalFluxes::default()
            },
        };
        CoveredColumnEnergyOperands {
            occupancies: vec![occupancy],
            canopy_air: CoveredCanopyAirEnergyOperands {
                canopy_air_temperature_k: 295.0,
                canopy_air_specific_humidity_kg_kg: 0.01,
                ground_sensible_to_canopy_air_w_m2_tile: 2.0,
                ground_vapor_to_canopy_air_kg_m2_tile_s: 1.0e-6,
                sensible_to_reference_air_w_m2_tile: canopy_sensible + 2.0,
                vapor_to_reference_air_kg_m2_tile_s: 10.0e-6,
            },
            shortwave: CoveredColumnShortwaveOperands {
                incident_w_m2_tile: BandDirectionalFluxes {
                    direct_vis: 17.5,
                    diffuse_vis: 29.0,
                    direct_nir: 41.5,
                    diffuse_nir: 10.0,
                },
                top_reflected_w_m2_tile: BandDirectionalFluxes {
                    direct_vis: 5.0,
                    diffuse_vis: 6.0,
                    direct_nir: 7.0,
                    diffuse_nir: 8.0,
                },
                ground_absorbed_by_incident_w_m2_tile: ground_absorbed,
                ground_terminal_w_m2_tile: ground_terminal,
                ground_absorbed_w_m2_tile: ground_absorbed,
                ground_reflected_w_m2_tile: ground_reflected,
                occupancies: vec![radiation],
            },
            longwave: CoveredColumnLongwaveOperands {
                atmospheric_downward_w_m2_tile: 100.0,
                transmissivities: vec![0.5],
                downward_boundaries_w_m2_tile: vec![100.0, 80.0],
                upward_boundaries_w_m2_tile: vec![84.0, 50.0],
                top_upward_w_m2_tile: 84.0,
                ground_net_w_m2_tile: 30.0,
                occupancy_component_net_w_m2_tile: vec![(
                    "canopy-rank-0".into(),
                    [-2.0, -3.0, -4.0, -5.0],
                )],
            },
            stage3_lower_boundary_energy_w_m2_tile: 0.0,
            optical_receipt_sha256: None,
            reciprocal_longwave_receipt_sha256: None,
            final_canopy_boundary_receipt_sha256: None,
        }
    }

    fn valid_tile() -> CoveredTileEnergyOperandSet {
        let column = valid_column();
        let temperature = 292.0;
        let vapor = column.canopy_air.ground_vapor_to_canopy_air_kg_m2_tile_s;
        let vapor_energy = vapor_export_w_m2(vapor, temperature).expect("ground latent");
        let surface = SurfaceEnergyOperands {
            absorbed_shortwave_w_m2: column.shortwave.ground_absorbed_w_m2_tile.total(),
            net_longwave_w_m2: column.longwave.ground_net_w_m2_tile,
            sensible_w_m2: column.canopy_air.ground_sensible_to_canopy_air_w_m2_tile,
            signed_vapor_kg_m2_s: vapor,
            surface_temperature_k: temperature,
            ground_heat_w_m2: 1.0,
            storage_w_m2: column.shortwave.ground_absorbed_w_m2_tile.total()
                + column.longwave.ground_net_w_m2_tile
                - column.canopy_air.ground_sensible_to_canopy_air_w_m2_tile
                - vapor_energy
                - 1.0,
        };
        CoveredTileEnergyOperandSet {
            authority: crate::CoveredColumnAuthority::HistoricalV8,
            lower_boundary: CoveredLowerBoundaryEnergyOperands::SnowFree(TileEnergyOperandSet {
                surface,
                latent: LatentJoinOperands {
                    signed_vapor_kg_m2_s: vapor,
                    interval_s: 1_800.0,
                    surface_temperature_k: temperature,
                    signed_water_amount_kg_m2: vapor * 1_800.0,
                    vapor_energy_j_m2: vapor_energy * 1_800.0,
                },
                ground_heat: vec![GroundHeatJoinOperands {
                    surface_debit_j_m2: 1_800.0,
                    soil_credit_j_m2: 1_800.0,
                }],
            }),
            column,
        }
    }

    fn valid_stage3_tile() -> CoveredTileEnergyOperandSet {
        let mut column = valid_column();
        let digest = Sha256Digest::try_new("a".repeat(64)).expect("digest");
        let optical =
            Stage3SnowOpticalBoundaryReceiptV1::try_new(Stage3SnowOpticalBoundaryReceiptInputs {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                tile_id: TileId::try_new("tile-1").expect("tile"),
                terminal_w_m2_tile: column.shortwave.ground_terminal_w_m2_tile,
                absorbed_w_m2_tile: column.shortwave.ground_absorbed_w_m2_tile,
                reflected_w_m2_tile: column.shortwave.ground_reflected_w_m2_tile,
                snow_vis_albedo: 0.5,
                snow_nir_albedo: 0.5,
                stage3_albedo_state_sha256: digest.clone(),
                forcing_receipt_sha256: digest.clone(),
            })
            .expect("optical receipt");
        column.stage3_lower_boundary_energy_w_m2_tile = 30.5;
        column.optical_receipt_sha256 = Some(digest.clone());
        column.reciprocal_longwave_receipt_sha256 = Some(digest.clone());
        column.final_canopy_boundary_receipt_sha256 = Some(digest.clone());
        CoveredTileEnergyOperandSet {
            authority: crate::CoveredColumnAuthority::V11SnowCovered,
            lower_boundary: CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(
                Stage3LowerBoundaryEnergyOperands {
                    optical,
                    snow_temperature_k: 280.0,
                    vapor_to_canopy_air_kg_m2_tile_s: 1.0e-6,
                    interval_s: 1_800.0,
                    latent_heat_j_kg: 2_500_000.0,
                    latent_energy_to_canopy_air_j_m2_tile: 4_500.0,
                    sensible_to_canopy_air_w_m2_tile: 2.0,
                    net_longwave_w_m2_tile: 30.0,
                    precipitation_advection_w_m2_tile: 0.0,
                    boundary_energy_w_m2_tile: 30.5,
                    carrier_receipt_id: digest.clone(),
                    optical_receipt_sha256: Some(digest.clone()),
                    reciprocal_longwave_receipt_sha256: Some(digest.clone()),
                    final_canopy_boundary_receipt_sha256: Some(digest),
                },
            ),
            column,
        }
    }

    #[test]
    fn complete_covered_energy_operands_reconstruct_without_producer_residual() {
        valid_tile().validate().expect("complete covered energy");
    }

    #[test]
    fn covered_component_shared_air_and_radiation_poisons_fail_closed() {
        let mut omitted = valid_tile();
        omitted.column.occupancies.pop();
        assert!(omitted.validate().is_err());

        let mut swapped_component = valid_tile();
        swapped_component.column.occupancies[0]
            .sun_leaf
            .net_longwave_w_m2_tile = -3.0;
        assert!(swapped_component.validate().is_err());

        let mut omitted_ground_air = valid_tile();
        omitted_ground_air
            .column
            .canopy_air
            .ground_sensible_to_canopy_air_w_m2_tile = 0.0;
        assert!(omitted_ground_air.validate().is_err());

        let mut wrong_band = valid_tile();
        wrong_band.column.shortwave.incident_w_m2_tile.direct_vis += 1.0;
        assert!(wrong_band.validate().is_err());

        let mut leaf_stem_swap = valid_tile();
        leaf_stem_swap.column.shortwave.occupancies[0].sun_leaf_absorbed_w_m2_tile =
            leaf_stem_swap.column.shortwave.occupancies[0].stem_absorbed_w_m2_tile;
        assert!(leaf_stem_swap.validate().is_err());

        let mut wrong_longwave_boundary = valid_tile();
        wrong_longwave_boundary
            .column
            .longwave
            .downward_boundaries_w_m2_tile[1] += 1.0;
        assert!(wrong_longwave_boundary.validate().is_err());
    }

    #[test]
    fn stage3_lower_boundary_and_column_joins_reject_one_bit_poison() {
        let valid = valid_stage3_tile();
        valid.validate().expect("Stage-3 cross-join");

        let mut poisoned_optical = valid.clone();
        if let CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) =
            &mut poisoned_optical.lower_boundary
        {
            stage3.optical_receipt_sha256 =
                Some(Sha256Digest::try_new("b".repeat(64)).expect("digest"));
        }
        assert!(poisoned_optical.validate().is_err());

        let mut poisoned_longwave = valid;
        if let CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) =
            &mut poisoned_longwave.lower_boundary
        {
            stage3.net_longwave_w_m2_tile =
                f64::from_bits(stage3.net_longwave_w_m2_tile.to_bits() + 1);
        }
        assert!(poisoned_longwave.validate().is_err());
    }

    #[test]
    fn authorization_cannot_replace_finalized_water_in_covered_latent_join() {
        let mut poisoned = valid_tile();
        if let CoveredLowerBoundaryEnergyOperands::SnowFree(ground) = &mut poisoned.lower_boundary {
            ground.latent.signed_water_amount_kg_m2 += 1.0e-6;
        }
        assert!(poisoned.validate().is_err());
    }
}
