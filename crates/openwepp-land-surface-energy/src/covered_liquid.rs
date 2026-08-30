//! Exact V8 E04 canopy-liquid preparation and finalization.

use serde::Serialize;

use crate::physics::{REFERENCE_TEMPERATURE_K, WATER_HEAT_CAPACITY_J_KG_K};
use crate::{CoveredOccupancyInputs, LandSurfaceEnergyError};

const LIQUID_REFERENCE_NEXT_UP_BITS: u64 = 0x4071_1266_6666_6667;

fn canonical_liquid_reference_temperature_k(temperature_k: f64) -> f64 {
    if temperature_k.to_bits() == LIQUID_REFERENCE_NEXT_UP_BITS {
        REFERENCE_TEMPERATURE_K
    } else {
        temperature_k
    }
}

/// Numerical pass that produced an E04 liquid candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CoveredLiquidPass {
    Potential,
    FixedAuthorizationFinal,
}

/// Accepted E04 operands and releases for one occupancy, all on tile-ground
/// interval basis. No residual supplied by the producer is accepted as
/// closure evidence; [`Self::validate`] reconstructs it from operands.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct CoveredOccupancyLiquidLedger {
    pub pass: CoveredLiquidPass,
    pub beginning_store_kg_m2_tile: f64,
    pub incident_rain_kg_m2_tile: f64,
    pub ending_store_kg_m2_tile: f64,
    pub evaporation_kg_m2_tile: f64,
    pub condensation_kg_m2_tile: f64,
    pub throughfall_kg_m2_tile: f64,
    pub stemflow_kg_m2_tile: f64,
    pub initial_drainage_kg_m2_tile: f64,
    pub second_drainage_kg_m2_tile: f64,
    pub wet_fraction: f64,
    pub wet_surface_temperature_k: f64,
    pub wet_surface_specific_enthalpy_j_kg: f64,
}

impl CoveredOccupancyLiquidLedger {
    /// Independently reconstruct E04 mass closure and liquid enthalpy.
    ///
    /// # Errors
    ///
    /// Returns a typed error for a nonfinite or invalid operand, inconsistent
    /// enthalpy, or a liquid mass-closure failure.
    pub fn validate(self) -> Result<(), LandSurfaceEnergyError> {
        let amounts = [
            self.beginning_store_kg_m2_tile,
            self.incident_rain_kg_m2_tile,
            self.ending_store_kg_m2_tile,
            self.evaporation_kg_m2_tile,
            self.condensation_kg_m2_tile,
            self.throughfall_kg_m2_tile,
            self.stemflow_kg_m2_tile,
            self.initial_drainage_kg_m2_tile,
            self.second_drainage_kg_m2_tile,
        ];
        if amounts.iter().any(|value| !value.is_finite())
            || !self.wet_fraction.is_finite()
            || !self.wet_surface_temperature_k.is_finite()
            || !self.wet_surface_specific_enthalpy_j_kg.is_finite()
        {
            return Err(LandSurfaceEnergyError::NonFinite(
                "covered occupancy liquid ledger",
            ));
        }
        if amounts.iter().any(|value| *value < 0.0)
            || !(0.0..=1.0).contains(&self.wet_fraction)
            || self.wet_surface_temperature_k < REFERENCE_TEMPERATURE_K
            || self.wet_surface_specific_enthalpy_j_kg.to_bits()
                != (WATER_HEAT_CAPACITY_J_KG_K
                    * (self.wet_surface_temperature_k - REFERENCE_TEMPERATURE_K))
                    .to_bits()
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered occupancy liquid ledger",
            ));
        }
        let residual = self.beginning_store_kg_m2_tile
            + self.incident_rain_kg_m2_tile
            + self.condensation_kg_m2_tile
            - self.ending_store_kg_m2_tile
            - self.evaporation_kg_m2_tile
            - self.throughfall_kg_m2_tile
            - self.stemflow_kg_m2_tile
            - self.initial_drainage_kg_m2_tile
            - self.second_drainage_kg_m2_tile;
        let scale = amounts.iter().map(|value| value.abs()).sum::<f64>();
        if residual.abs() > 1.0e-14 + 64.0 * f64::EPSILON * scale {
            return Err(LandSurfaceEnergyError::water_closure(
                "covered occupancy liquid ledger",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CoveredLiquidPreparation {
    pub(crate) beginning_store: f64,
    pub(crate) incident_rain: f64,
    pub(crate) preliminary_store: f64,
    pub(crate) throughfall: f64,
    pub(crate) stemflow: f64,
    pub(crate) initial_drainage: f64,
    pub(crate) capacity: f64,
    pub(crate) wet_fraction: f64,
}

fn drain_to_capacity(store_before_drainage: f64, capacity: f64) -> (f64, f64) {
    if store_before_drainage > capacity {
        (store_before_drainage - capacity, capacity)
    } else {
        (0.0, store_before_drainage)
    }
}

pub(crate) fn prepare_covered_liquid(
    occupancy: &CoveredOccupancyInputs,
    incident_rain: f64,
) -> Result<CoveredLiquidPreparation, LandSurfaceEnergyError> {
    let values = [
        occupancy.beginning_canopy_liquid_kg_m2_tile,
        occupancy.liquid_interception_fraction,
        occupancy.liquid_capacity_kg_m2_plant,
        occupancy.stemflow_fraction,
        occupancy.lai,
        occupancy.sai,
        incident_rain,
    ];
    if values.iter().any(|value| !value.is_finite()) {
        return Err(LandSurfaceEnergyError::NonFinite("covered E04 preparation"));
    }
    if occupancy.beginning_canopy_liquid_kg_m2_tile < 0.0
        || !(0.0..=1.0).contains(&occupancy.liquid_interception_fraction)
        || occupancy.liquid_capacity_kg_m2_plant < 0.0
        || !(0.0..=1.0).contains(&occupancy.stemflow_fraction)
        || occupancy.lai < 0.0
        || occupancy.sai < 0.0
        || incident_rain < 0.0
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered E04 preparation",
        ));
    }
    let plant_area = occupancy.lai + occupancy.sai;
    let intercepted = occupancy.liquid_interception_fraction * plant_area.tanh() * incident_rain;
    let free = incident_rain - intercepted;
    let stemflow = occupancy.stemflow_fraction * free;
    let throughfall = (1.0 - occupancy.stemflow_fraction) * free;
    let capacity = occupancy.liquid_capacity_kg_m2_plant * plant_area;
    let store_before_drainage = occupancy.beginning_canopy_liquid_kg_m2_tile + intercepted;
    let (initial_drainage, preliminary_store) = drain_to_capacity(store_before_drainage, capacity);
    let wet_fraction = if capacity > 0.0 {
        (preliminary_store / capacity).powf(2.0 / 3.0)
    } else {
        0.0
    };
    Ok(CoveredLiquidPreparation {
        beginning_store: occupancy.beginning_canopy_liquid_kg_m2_tile,
        incident_rain,
        preliminary_store,
        throughfall,
        stemflow,
        initial_drainage,
        capacity,
        wet_fraction,
    })
}

pub(crate) fn finalize_covered_liquid(
    preparation: CoveredLiquidPreparation,
    signed_vapor_amount: f64,
    wet_surface_temperature_k: f64,
    pass: CoveredLiquidPass,
) -> Result<CoveredOccupancyLiquidLedger, LandSurfaceEnergyError> {
    if !signed_vapor_amount.is_finite() || !wet_surface_temperature_k.is_finite() {
        return Err(LandSurfaceEnergyError::NonFinite(
            "covered E04 finalization",
        ));
    }
    let wet_surface_temperature_k =
        canonical_liquid_reference_temperature_k(wet_surface_temperature_k);
    if wet_surface_temperature_k < REFERENCE_TEMPERATURE_K {
        return Err(LandSurfaceEnergyError::UnsupportedDomain(
            "covered_canopy_snow",
        ));
    }
    let (ending_store, evaporation, condensation, second_drainage) = if signed_vapor_amount >= 0.0 {
        let evaporation = signed_vapor_amount.min(preparation.preliminary_store);
        (
            preparation.preliminary_store - evaporation,
            evaporation,
            0.0,
            0.0,
        )
    } else {
        let condensation = -signed_vapor_amount;
        let store_with_condensation = preparation.preliminary_store + condensation;
        let (second_drainage, ending_store) =
            drain_to_capacity(store_with_condensation, preparation.capacity);
        (ending_store, 0.0, condensation, second_drainage)
    };
    let result = CoveredOccupancyLiquidLedger {
        pass,
        beginning_store_kg_m2_tile: preparation.beginning_store,
        incident_rain_kg_m2_tile: preparation.incident_rain,
        ending_store_kg_m2_tile: ending_store,
        evaporation_kg_m2_tile: evaporation,
        condensation_kg_m2_tile: condensation,
        throughfall_kg_m2_tile: preparation.throughfall,
        stemflow_kg_m2_tile: preparation.stemflow,
        initial_drainage_kg_m2_tile: preparation.initial_drainage,
        second_drainage_kg_m2_tile: second_drainage,
        wet_fraction: preparation.wet_fraction,
        wet_surface_temperature_k,
        wet_surface_specific_enthalpy_j_kg: WATER_HEAT_CAPACITY_J_KG_K
            * (wet_surface_temperature_k - REFERENCE_TEMPERATURE_K),
    };
    result.validate()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_preparation() -> CoveredLiquidPreparation {
        CoveredLiquidPreparation {
            beginning_store: 0.0,
            incident_rain: 0.0,
            preliminary_store: 0.0,
            throughfall: 0.0,
            stemflow: 0.0,
            initial_drainage: 0.0,
            capacity: 0.0,
            wet_fraction: 0.0,
        }
    }

    #[test]
    fn exact_first_upward_reference_neighbor_is_canonicalized() {
        let ledger = finalize_covered_liquid(
            empty_preparation(),
            0.0,
            f64::from_bits(LIQUID_REFERENCE_NEXT_UP_BITS),
            CoveredLiquidPass::FixedAuthorizationFinal,
        )
        .expect("one-ULP reference representation is admitted");

        assert_eq!(
            ledger.wet_surface_temperature_k.to_bits(),
            REFERENCE_TEMPERATURE_K.to_bits()
        );
        assert_eq!(ledger.wet_surface_specific_enthalpy_j_kg.to_bits(), 0);
    }

    #[test]
    fn liquid_reference_canonicalization_has_an_exact_bit_boundary() {
        let at_reference = finalize_covered_liquid(
            empty_preparation(),
            0.0,
            REFERENCE_TEMPERATURE_K,
            CoveredLiquidPass::FixedAuthorizationFinal,
        )
        .expect("exact reference is admitted");
        assert_eq!(
            at_reference.wet_surface_temperature_k.to_bits(),
            REFERENCE_TEMPERATURE_K.to_bits()
        );

        let second_neighbor = f64::from_bits(LIQUID_REFERENCE_NEXT_UP_BITS + 1);
        let above = finalize_covered_liquid(
            empty_preparation(),
            0.0,
            second_neighbor,
            CoveredLiquidPass::FixedAuthorizationFinal,
        )
        .expect("second neighbor remains a physical temperature");
        assert_eq!(
            above.wet_surface_temperature_k.to_bits(),
            second_neighbor.to_bits()
        );
        assert!(above.wet_surface_specific_enthalpy_j_kg > 0.0);

        let below = f64::from_bits(REFERENCE_TEMPERATURE_K.to_bits() - 1);
        assert!(matches!(
            finalize_covered_liquid(
                empty_preparation(),
                0.0,
                below,
                CoveredLiquidPass::FixedAuthorizationFinal,
            ),
            Err(LandSurfaceEnergyError::UnsupportedDomain(
                "covered_canopy_snow"
            ))
        ));
    }

    #[test]
    fn saturated_fixture_drainage_retains_exact_capacity() {
        let capacity = f64::from_bits(0x3fd3_020c_49ba_5e37);
        let store_before_drainage = f64::from_bits(0x3ff9_74cb_d1f7_a2af);

        let subtractive_drainage = store_before_drainage - capacity;
        let subtractive_store = store_before_drainage - subtractive_drainage;
        assert_eq!(subtractive_store.to_bits(), capacity.to_bits() + 1);
        assert!((subtractive_store / capacity).powf(2.0 / 3.0) > 1.0);

        let (drainage, retained_store) = drain_to_capacity(store_before_drainage, capacity);
        assert_eq!(drainage.to_bits(), subtractive_drainage.to_bits());
        assert_eq!(retained_store.to_bits(), capacity.to_bits());
        assert_eq!(
            (retained_store / capacity).powf(2.0 / 3.0).to_bits(),
            1.0_f64.to_bits()
        );
    }

    #[test]
    fn wet_fraction_above_one_remains_a_typed_poison() {
        let poison = CoveredOccupancyLiquidLedger {
            pass: CoveredLiquidPass::FixedAuthorizationFinal,
            beginning_store_kg_m2_tile: 0.0,
            incident_rain_kg_m2_tile: 0.0,
            ending_store_kg_m2_tile: 0.0,
            evaporation_kg_m2_tile: 0.0,
            condensation_kg_m2_tile: 0.0,
            throughfall_kg_m2_tile: 0.0,
            stemflow_kg_m2_tile: 0.0,
            initial_drainage_kg_m2_tile: 0.0,
            second_drainage_kg_m2_tile: 0.0,
            wet_fraction: f64::from_bits(1.0_f64.to_bits() + 1),
            wet_surface_temperature_k: REFERENCE_TEMPERATURE_K,
            wet_surface_specific_enthalpy_j_kg: 0.0,
        };

        assert!(matches!(
            poison.validate(),
            Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered occupancy liquid ledger"
            ))
        ));
    }
}
