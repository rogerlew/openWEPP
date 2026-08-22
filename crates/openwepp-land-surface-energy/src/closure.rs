//! Independent reconstruction of LSE component and owner joins.
//!
//! Validators consume primitive operands only.  No producer-supplied residual
//! or declaration that closure passed is accepted by these APIs.

// Cross-owner joins deliberately require exact identity arithmetic before a
// dimensional closure envelope is applied; every Result is a typed validator.
#![allow(clippy::float_cmp, clippy::missing_errors_doc)]

use crate::physics::{energy_tolerance, liquid_enthalpy_j_kg, vapor_export_w_m2};
use crate::{LandSurfaceEnergyError, canonical_tile_fraction_sum_closes};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceEnergyOperands {
    pub absorbed_shortwave_w_m2: f64,
    pub net_longwave_w_m2: f64,
    /// Positive away from the surface.
    pub sensible_w_m2: f64,
    /// Positive evaporation, negative condensation.
    pub signed_vapor_kg_m2_s: f64,
    pub surface_temperature_k: f64,
    /// Positive downward from the surface to the soil owner.
    pub ground_heat_w_m2: f64,
    /// Positive increase of surface enthalpy.
    pub storage_w_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClosureValue {
    pub reconstructed_residual: f64,
    pub tolerance: f64,
}

fn require_finite(values: &[(f64, &'static str)]) -> Result<(), LandSurfaceEnergyError> {
    for (value, field) in values {
        if !value.is_finite() {
            return Err(LandSurfaceEnergyError::NonFinite(field));
        }
    }
    Ok(())
}

pub fn validate_surface_energy(
    operands: SurfaceEnergyOperands,
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    require_finite(&[
        (operands.absorbed_shortwave_w_m2, "absorbed_shortwave_w_m2"),
        (operands.net_longwave_w_m2, "net_longwave_w_m2"),
        (operands.sensible_w_m2, "sensible_w_m2"),
        (operands.signed_vapor_kg_m2_s, "signed_vapor_kg_m2_s"),
        (operands.surface_temperature_k, "surface_temperature_k"),
        (operands.ground_heat_w_m2, "ground_heat_w_m2"),
        (operands.storage_w_m2, "storage_w_m2"),
    ])?;
    let vapor = vapor_export_w_m2(
        operands.signed_vapor_kg_m2_s,
        operands.surface_temperature_k,
    )?;
    let components = [
        operands.absorbed_shortwave_w_m2,
        operands.net_longwave_w_m2,
        -operands.sensible_w_m2,
        -vapor,
        -operands.ground_heat_w_m2,
        -operands.storage_w_m2,
    ];
    let residual = components.iter().sum::<f64>();
    let tolerance = energy_tolerance(
        components
            .iter()
            .map(|component| component.abs())
            .sum::<f64>(),
    );
    if residual.abs() > tolerance {
        return Err(LandSurfaceEnergyError::ControlVolumeClosure(
            "ground_surface_energy",
        ));
    }
    Ok(ClosureValue {
        reconstructed_residual: residual,
        tolerance,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShortwaveOwnershipOperands {
    pub incident_w_m2: f64,
    pub absorbed_w_m2: f64,
    pub reflected_w_m2: f64,
    pub transmitted_w_m2: f64,
}

pub fn validate_shortwave_ownership(
    operands: ShortwaveOwnershipOperands,
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    require_finite(&[
        (operands.incident_w_m2, "incident_shortwave_w_m2"),
        (operands.absorbed_w_m2, "absorbed_shortwave_w_m2"),
        (operands.reflected_w_m2, "reflected_shortwave_w_m2"),
        (operands.transmitted_w_m2, "transmitted_shortwave_w_m2"),
    ])?;
    if operands.incident_w_m2 < 0.0
        || operands.absorbed_w_m2 < 0.0
        || operands.reflected_w_m2 < 0.0
        || operands.transmitted_w_m2 < 0.0
    {
        return Err(LandSurfaceEnergyError::ComponentClosure(
            "negative_shortwave_component",
        ));
    }
    let residual = operands.incident_w_m2
        - operands.absorbed_w_m2
        - operands.reflected_w_m2
        - operands.transmitted_w_m2;
    let tolerance = energy_tolerance(
        operands.incident_w_m2.abs()
            + operands.absorbed_w_m2.abs()
            + operands.reflected_w_m2.abs()
            + operands.transmitted_w_m2.abs(),
    );
    if residual.abs() > tolerance {
        return Err(LandSurfaceEnergyError::ComponentClosure(
            "shortwave_ownership",
        ));
    }
    Ok(ClosureValue {
        reconstructed_residual: residual,
        tolerance,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterUseOperands {
    pub request_kg_m2: f64,
    pub authorization_kg_m2: f64,
    pub finalized_use_kg_m2: f64,
    pub beginning_store_kg_m2: f64,
    pub condensation_credit_kg_m2: f64,
    pub ending_pre_ingress_store_kg_m2: f64,
}

pub fn validate_water_use(operands: WaterUseOperands) -> Result<(), LandSurfaceEnergyError> {
    require_finite(&[
        (operands.request_kg_m2, "water_request_kg_m2"),
        (operands.authorization_kg_m2, "water_authorization_kg_m2"),
        (operands.finalized_use_kg_m2, "water_finalized_use_kg_m2"),
        (
            operands.beginning_store_kg_m2,
            "water_beginning_store_kg_m2",
        ),
        (
            operands.condensation_credit_kg_m2,
            "water_condensation_credit_kg_m2",
        ),
        (
            operands.ending_pre_ingress_store_kg_m2,
            "water_ending_pre_ingress_store_kg_m2",
        ),
    ])?;
    if operands.request_kg_m2 < 0.0
        || operands.authorization_kg_m2 < 0.0
        || operands.finalized_use_kg_m2 < 0.0
        || operands.beginning_store_kg_m2 < 0.0
        || operands.condensation_credit_kg_m2 < 0.0
        || operands.ending_pre_ingress_store_kg_m2 < 0.0
        || operands.authorization_kg_m2 > operands.request_kg_m2
        || operands.finalized_use_kg_m2 > operands.authorization_kg_m2
    {
        return Err(LandSurfaceEnergyError::water_bound("D/A/F"));
    }
    let reconstructed = operands.beginning_store_kg_m2 - operands.finalized_use_kg_m2
        + operands.condensation_credit_kg_m2;
    if reconstructed != operands.ending_pre_ingress_store_kg_m2 {
        return Err(LandSurfaceEnergyError::water_closure(
            "pre_ingress_source_mass_closure",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LatentJoinOperands {
    pub signed_vapor_kg_m2_s: f64,
    pub interval_s: f64,
    pub surface_temperature_k: f64,
    pub signed_water_amount_kg_m2: f64,
    pub vapor_energy_j_m2: f64,
}

pub fn validate_latent_join(
    operands: LatentJoinOperands,
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    require_finite(&[
        (operands.signed_vapor_kg_m2_s, "signed_vapor_kg_m2_s"),
        (operands.interval_s, "interval_s"),
        (operands.surface_temperature_k, "surface_temperature_k"),
        (
            operands.signed_water_amount_kg_m2,
            "signed_water_amount_kg_m2",
        ),
        (operands.vapor_energy_j_m2, "vapor_energy_j_m2"),
    ])?;
    if operands.interval_s <= 0.0 {
        return Err(LandSurfaceEnergyError::LatentJoin("interval_domain"));
    }
    let expected_mass = operands.signed_vapor_kg_m2_s * operands.interval_s;
    if expected_mass != operands.signed_water_amount_kg_m2 {
        return Err(LandSurfaceEnergyError::LatentJoin("mass_rate_interval"));
    }
    let expected_energy = vapor_export_w_m2(
        operands.signed_vapor_kg_m2_s,
        operands.surface_temperature_k,
    )? * operands.interval_s;
    let residual = operands.vapor_energy_j_m2 - expected_energy;
    let tolerance = 1.0e-7 + 64.0 * f64::EPSILON * expected_energy.abs().max(1.0);
    if residual.abs() > tolerance {
        return Err(LandSurfaceEnergyError::LatentJoin(
            "signed_mass_energy_identity",
        ));
    }
    Ok(ClosureValue {
        reconstructed_residual: residual,
        tolerance,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundHeatJoinOperands {
    /// Surface control-volume outward/downward amount.
    pub surface_debit_j_m2: f64,
    /// Soil thermal control-volume incoming amount.
    pub soil_credit_j_m2: f64,
}

pub fn validate_ground_heat_join(
    operands: GroundHeatJoinOperands,
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    require_finite(&[
        (
            operands.surface_debit_j_m2,
            "surface_ground_heat_debit_j_m2",
        ),
        (operands.soil_credit_j_m2, "soil_ground_heat_credit_j_m2"),
    ])?;
    let residual = operands.surface_debit_j_m2 - operands.soil_credit_j_m2;
    let scale = operands
        .surface_debit_j_m2
        .abs()
        .max(operands.soil_credit_j_m2.abs())
        .max(1.0);
    let tolerance = 1.0e-7 + 64.0 * f64::EPSILON * scale;
    if residual.abs() > tolerance {
        return Err(LandSurfaceEnergyError::GroundHeatJoin(
            "surface_soil_equal_opposite",
        ));
    }
    Ok(ClosureValue {
        reconstructed_residual: residual,
        tolerance,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiquidParcelOperands {
    pub parcel_id: String,
    pub mass_kg_m2: f64,
    pub temperature_k: Option<f64>,
    pub enthalpy_j_m2: f64,
}

pub fn validate_liquid_parcel(
    parcel: &LiquidParcelOperands,
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    if parcel.parcel_id.is_empty() {
        return Err(LandSurfaceEnergyError::LatentJoin("empty_parcel_identity"));
    }
    require_finite(&[
        (parcel.mass_kg_m2, "parcel_mass_kg_m2"),
        (parcel.enthalpy_j_m2, "parcel_enthalpy_j_m2"),
    ])?;
    if parcel.mass_kg_m2 < 0.0 {
        return Err(LandSurfaceEnergyError::LatentJoin("negative_parcel_mass"));
    }
    if parcel.mass_kg_m2 == 0.0 {
        if parcel.temperature_k.is_some() || parcel.enthalpy_j_m2 != 0.0 {
            return Err(LandSurfaceEnergyError::LatentJoin(
                "zero_mass_parcel_has_thermal_payload",
            ));
        }
        return Ok(ClosureValue {
            reconstructed_residual: 0.0,
            tolerance: 0.0,
        });
    }
    let temperature = parcel
        .temperature_k
        .ok_or(LandSurfaceEnergyError::LatentJoin(
            "missing_parcel_temperature",
        ))?;
    if !temperature.is_finite() || temperature <= 0.0 {
        return Err(LandSurfaceEnergyError::LatentJoin(
            "parcel_temperature_domain",
        ));
    }
    let expected = parcel.mass_kg_m2 * liquid_enthalpy_j_kg(temperature);
    let residual = parcel.enthalpy_j_m2 - expected;
    let tolerance = 1.0e-7 + 64.0 * f64::EPSILON * expected.abs().max(1.0);
    if residual.abs() > tolerance {
        return Err(LandSurfaceEnergyError::LatentJoin("liquid_parcel_enthalpy"));
    }
    Ok(ClosureValue {
        reconstructed_residual: residual,
        tolerance,
    })
}

pub fn conservatively_mix_liquid_parcels(
    parcels: &[LiquidParcelOperands],
) -> Result<Option<(f64, f64, f64)>, LandSurfaceEnergyError> {
    let mut mass = 0.0;
    let mut enthalpy = 0.0;
    for parcel in parcels {
        validate_liquid_parcel(parcel)?;
        mass += parcel.mass_kg_m2;
        enthalpy += parcel.enthalpy_j_m2;
    }
    if mass == 0.0 {
        return Ok(None);
    }
    let temperature = crate::physics::REFERENCE_TEMPERATURE_K
        + enthalpy / (crate::physics::WATER_HEAT_CAPACITY_J_KG_K * mass);
    if !temperature.is_finite() || temperature <= 0.0 {
        return Err(LandSurfaceEnergyError::LatentJoin(
            "mixed_liquid_temperature",
        ));
    }
    Ok(Some((mass, enthalpy, temperature)))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedTileEnergyOperands {
    pub tile_fraction: f64,
    pub local_input_j_m2_tile: f64,
    pub local_output_j_m2_tile: f64,
    pub local_storage_change_j_m2_tile: f64,
    /// Sum of absolute interval-integrated primitive boundary/storage terms,
    /// before signed aggregation or cross-tile weighting.
    pub local_sum_abs_integrated_components_j_m2_tile: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedOfeEnergyDecomposition {
    pub interval_s: f64,
    pub weighted_input_j_m2: f64,
    pub weighted_output_j_m2: f64,
    pub weighted_storage_change_j_m2: f64,
    pub weighted_residual_j_m2: f64,
    pub weighted_sum_abs_integrated_components_j_m2: f64,
    pub tolerance_j_m2: f64,
}

pub fn decompose_weighted_ofe_energy(
    interval_s: f64,
    tiles: &[WeightedTileEnergyOperands],
) -> Result<WeightedOfeEnergyDecomposition, LandSurfaceEnergyError> {
    if !interval_s.is_finite() || interval_s <= 0.0 {
        return Err(LandSurfaceEnergyError::ControlVolumeClosure(
            "weighted_ofe_interval",
        ));
    }
    if tiles.is_empty() {
        return Err(LandSurfaceEnergyError::ControlVolumeClosure(
            "empty_tile_set",
        ));
    }
    let mut fraction_sum = 0.0;
    let mut inputs = 0.0;
    let mut outputs = 0.0;
    let mut storage = 0.0;
    let mut sum_abs_integrated_components = 0.0;
    for tile in tiles {
        require_finite(&[
            (tile.tile_fraction, "tile_fraction"),
            (tile.local_input_j_m2_tile, "tile_input_j_m2"),
            (tile.local_output_j_m2_tile, "tile_output_j_m2"),
            (
                tile.local_storage_change_j_m2_tile,
                "tile_storage_change_j_m2",
            ),
            (
                tile.local_sum_abs_integrated_components_j_m2_tile,
                "tile_sum_abs_integrated_components_j_m2",
            ),
        ])?;
        if tile.tile_fraction <= 0.0 || tile.tile_fraction > 1.0 {
            return Err(LandSurfaceEnergyError::ControlVolumeClosure(
                "tile_fraction_domain",
            ));
        }
        if tile.local_sum_abs_integrated_components_j_m2_tile < 0.0 {
            return Err(LandSurfaceEnergyError::ControlVolumeClosure(
                "tile_sum_abs_integrated_components_domain",
            ));
        }
        fraction_sum += tile.tile_fraction;
        inputs += tile.tile_fraction * tile.local_input_j_m2_tile;
        outputs += tile.tile_fraction * tile.local_output_j_m2_tile;
        storage += tile.tile_fraction * tile.local_storage_change_j_m2_tile;
        sum_abs_integrated_components +=
            tile.tile_fraction * tile.local_sum_abs_integrated_components_j_m2_tile;
    }
    if !canonical_tile_fraction_sum_closes(fraction_sum) {
        return Err(LandSurfaceEnergyError::ControlVolumeClosure(
            "tile_fraction_sum",
        ));
    }
    require_finite(&[
        (inputs, "weighted_input_j_m2"),
        (outputs, "weighted_output_j_m2"),
        (storage, "weighted_storage_j_m2"),
        (
            sum_abs_integrated_components,
            "weighted_sum_abs_integrated_components_j_m2",
        ),
    ])?;
    let residual = inputs - outputs - storage;
    // The authorized relative scale precedes signed aggregation so inward
    // fluxes, condensation, and heterogeneous-tile cancellation cannot shrink it.
    let tolerance = 1.0e-6 * interval_s + 1.0e-10 * interval_s.max(sum_abs_integrated_components);
    Ok(WeightedOfeEnergyDecomposition {
        interval_s,
        weighted_input_j_m2: inputs,
        weighted_output_j_m2: outputs,
        weighted_storage_change_j_m2: storage,
        weighted_residual_j_m2: residual,
        weighted_sum_abs_integrated_components_j_m2: sum_abs_integrated_components,
        tolerance_j_m2: tolerance,
    })
}

pub fn validate_weighted_ofe_energy(
    interval_s: f64,
    tiles: &[WeightedTileEnergyOperands],
) -> Result<ClosureValue, LandSurfaceEnergyError> {
    let decomposition = decompose_weighted_ofe_energy(interval_s, tiles)?;
    if decomposition.weighted_residual_j_m2.abs() > decomposition.tolerance_j_m2 {
        return Err(LandSurfaceEnergyError::ControlVolumeClosure(
            "weighted_ofe_energy",
        ));
    }
    Ok(ClosureValue {
        reconstructed_residual: decomposition.weighted_residual_j_m2,
        tolerance: decomposition.tolerance_j_m2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condensation_retains_signed_mass_and_energy() {
        let temperature = 290.0;
        let flux = -2.5e-5;
        let interval = 1_800.0;
        let energy = vapor_export_w_m2(flux, temperature).expect("valid") * interval;
        let result = validate_latent_join(LatentJoinOperands {
            signed_vapor_kg_m2_s: flux,
            interval_s: interval,
            surface_temperature_k: temperature,
            signed_water_amount_kg_m2: flux * interval,
            vapor_energy_j_m2: energy,
        });
        assert!(result.is_ok());
        assert!(energy < 0.0);
    }

    #[test]
    fn authorization_is_not_accepted_as_use_when_demand_is_lower() {
        let error = validate_water_use(WaterUseOperands {
            request_kg_m2: 1.0,
            authorization_kg_m2: 0.8,
            finalized_use_kg_m2: 0.8,
            beginning_store_kg_m2: 2.0,
            condensation_credit_kg_m2: 0.0,
            ending_pre_ingress_store_kg_m2: 1.6,
        })
        .expect_err("ending store exposes substituted authorization");
        assert_eq!(
            error,
            LandSurfaceEnergyError::water_closure("pre_ingress_source_mass_closure")
        );
    }

    #[test]
    fn ground_heat_must_be_equal_and_opposite() {
        assert!(
            validate_ground_heat_join(GroundHeatJoinOperands {
                surface_debit_j_m2: -12.0,
                soil_credit_j_m2: -12.0,
            })
            .is_ok()
        );
        assert!(
            validate_ground_heat_join(GroundHeatJoinOperands {
                surface_debit_j_m2: -12.0,
                soil_credit_j_m2: 12.0,
            })
            .is_err()
        );
    }

    #[test]
    fn zero_mass_parcel_cannot_carry_temperature_or_energy() {
        let result = validate_liquid_parcel(&LiquidParcelOperands {
            parcel_id: "zero".into(),
            mass_kg_m2: 0.0,
            temperature_k: Some(273.15),
            enthalpy_j_m2: 0.0,
        });
        assert!(matches!(result, Err(LandSurfaceEnergyError::LatentJoin(_))));
    }

    #[test]
    fn tile_fraction_is_applied_once_after_local_closure() {
        let result = validate_weighted_ofe_energy(
            1.0,
            &[
                WeightedTileEnergyOperands {
                    tile_fraction: 0.4,
                    local_input_j_m2_tile: 10.0,
                    local_output_j_m2_tile: 8.0,
                    local_storage_change_j_m2_tile: 2.0,
                    local_sum_abs_integrated_components_j_m2_tile: 20.0,
                },
                WeightedTileEnergyOperands {
                    tile_fraction: 0.6,
                    local_input_j_m2_tile: 20.0,
                    local_output_j_m2_tile: 17.0,
                    local_storage_change_j_m2_tile: 3.0,
                    local_sum_abs_integrated_components_j_m2_tile: 40.0,
                },
            ],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn weighted_energy_integrates_absolute_rate_tolerance_over_interval() {
        let interval_s = 1_800.0;
        let within = WeightedTileEnergyOperands {
            tile_fraction: 1.0,
            local_input_j_m2_tile: 100.0,
            local_output_j_m2_tile: 100.0 - 0.999e-6 * interval_s,
            local_storage_change_j_m2_tile: 0.0,
            local_sum_abs_integrated_components_j_m2_tile: 200.0,
        };
        let accepted =
            validate_weighted_ofe_energy(interval_s, &[within]).expect("within rate tolerance");
        assert!(accepted.tolerance >= 1.0e-6 * interval_s);

        let outside = WeightedTileEnergyOperands {
            local_output_j_m2_tile: 100.0 - 1.001e-6 * interval_s,
            ..within
        };
        assert!(validate_weighted_ofe_energy(interval_s, &[outside]).is_err());
    }

    #[test]
    fn weighted_energy_rejects_invalid_interval() {
        let tile = WeightedTileEnergyOperands {
            tile_fraction: 1.0,
            local_input_j_m2_tile: 1.0,
            local_output_j_m2_tile: 1.0,
            local_storage_change_j_m2_tile: 0.0,
            local_sum_abs_integrated_components_j_m2_tile: 2.0,
        };
        for interval in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(validate_weighted_ofe_energy(interval, &[tile]).is_err());
        }
    }

    #[test]
    fn weighted_energy_uses_canonical_tile_fraction_boundary() {
        let tile = |tile_fraction| WeightedTileEnergyOperands {
            tile_fraction,
            local_input_j_m2_tile: 0.0,
            local_output_j_m2_tile: 0.0,
            local_storage_change_j_m2_tile: 0.0,
            local_sum_abs_integrated_components_j_m2_tile: 0.0,
        };
        let admitted = [tile(0.5), tile(0.5 + 32.0 * f64::EPSILON)];
        assert!(canonical_tile_fraction_sum_closes(
            admitted.iter().map(|value| value.tile_fraction).sum()
        ));
        validate_weighted_ofe_energy(1.0, &admitted).expect("canonical admitted fraction sum");

        let rejected = [tile(0.5), tile(0.5 + 128.0 * f64::EPSILON)];
        assert!(!canonical_tile_fraction_sum_closes(
            rejected.iter().map(|value| value.tile_fraction).sum()
        ));
        assert!(validate_weighted_ofe_energy(1.0, &rejected).is_err());
    }

    #[test]
    fn weighted_energy_scale_precedes_signed_and_cross_tile_cancellation() {
        let interval_s = 10.0;
        let tiles = [
            WeightedTileEnergyOperands {
                tile_fraction: 0.5,
                local_input_j_m2_tile: 1.0e12,
                local_output_j_m2_tile: -1.0e12,
                local_storage_change_j_m2_tile: 2.0e12,
                local_sum_abs_integrated_components_j_m2_tile: 4.0e12,
            },
            WeightedTileEnergyOperands {
                tile_fraction: 0.5,
                local_input_j_m2_tile: -1.0e12,
                local_output_j_m2_tile: 1.0e12,
                local_storage_change_j_m2_tile: -2.0e12,
                local_sum_abs_integrated_components_j_m2_tile: 4.0e12,
            },
        ];
        let closure = validate_weighted_ofe_energy(interval_s, &tiles).expect("exact cancellation");
        assert_eq!(closure.reconstructed_residual, 0.0);
        assert_eq!(closure.tolerance, 1.0e-6 * interval_s + 1.0e-10 * 4.0e12);
    }

    #[test]
    fn weighted_energy_decomposition_exposes_residual_and_primitive_scale() {
        let decomposition = decompose_weighted_ofe_energy(
            10.0,
            &[WeightedTileEnergyOperands {
                tile_fraction: 1.0,
                local_input_j_m2_tile: 12.0,
                local_output_j_m2_tile: 8.0,
                local_storage_change_j_m2_tile: 3.0,
                local_sum_abs_integrated_components_j_m2_tile: 40.0,
            }],
        )
        .expect("valid weighted decomposition");
        assert_eq!(decomposition.weighted_residual_j_m2, 1.0);
        assert_eq!(
            decomposition.weighted_sum_abs_integrated_components_j_m2,
            40.0
        );
        assert!((decomposition.tolerance_j_m2 - (1.0e-6 * 10.0 + 1.0e-10 * 40.0)).abs() < 1.0e-20);
    }

    #[test]
    fn weighted_energy_relative_tolerance_has_exact_authorized_boundary() {
        let interval_s = 10.0;
        let scale = 1.0e8;
        let tolerance = 1.0e-6 * interval_s + 1.0e-10 * scale;
        let at_boundary = WeightedTileEnergyOperands {
            tile_fraction: 1.0,
            local_input_j_m2_tile: tolerance,
            local_output_j_m2_tile: 0.0,
            local_storage_change_j_m2_tile: 0.0,
            // Represents large opposing inward/condensation component terms.
            local_sum_abs_integrated_components_j_m2_tile: scale,
        };
        let accepted = validate_weighted_ofe_energy(interval_s, &[at_boundary])
            .expect("exact authorized boundary");
        assert_eq!(accepted.tolerance, tolerance);
        let outside = WeightedTileEnergyOperands {
            local_input_j_m2_tile: f64::from_bits(tolerance.to_bits() + 1),
            ..at_boundary
        };
        assert!(validate_weighted_ofe_energy(interval_s, &[outside]).is_err());
    }
}
