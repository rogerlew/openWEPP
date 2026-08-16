//! Exact aggregation of potential and accepted occupancy carbon operands.
//!
//! The pass identity is an explicit input to the private implementation so
//! potential E19 demand cannot silently consume capped carbon, and accepted
//! allocation cannot silently consume potential carbon. This module performs
//! no persistent-state mutation and publishes no owner candidate.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, StratumId};

use crate::VegetationError;
use crate::carbon_nitrogen::{gpp_kg_c, leaf_rd_carbon_debit};
use crate::column::{OccupancyCarbonOperands, TileColumnsResult};
use crate::diagnostics::CoupledSolvePass;

/// Exact stand-ground interval carbon operands aggregated once per stratum.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StratumCarbonOperands {
    pub gross_primary_production_kg_c_m2: f64,
    pub accepted_leaf_respiration_kg_c_m2: f64,
    pub advanced_t10_k: f64,
}

/// Exact class-resolved physical operands required to integrate gross carbon
/// gain and accepted leaf dark respiration. Acclimation state is deliberately
/// absent because it is persistent-owner state, not an integration operand.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ClassCarbonOperands {
    pub sun_leaf_area_m2_m2_tile_ground: f64,
    pub shade_leaf_area_m2_m2_tile_ground: f64,
    pub sun_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub shade_dark_respiration_umol_co2_m2_leaf_s: f64,
}

/// Aggregate the accepted capped pass without re-running `FvCB`.
pub(crate) fn aggregate_stratum_carbon(
    columns: &TileColumnsResult,
    interval_s: f64,
) -> Result<BTreeMap<StratumId, StratumCarbonOperands>, VegetationError> {
    aggregate_stratum_carbon_for_pass(columns, interval_s, CoupledSolvePass::Capped)
}

/// Aggregate the owner-uncapped potential pass without re-running `FvCB`.
pub(crate) fn aggregate_potential_stratum_carbon(
    columns: &TileColumnsResult,
    interval_s: f64,
) -> Result<BTreeMap<StratumId, StratumCarbonOperands>, VegetationError> {
    aggregate_stratum_carbon_for_pass(columns, interval_s, CoupledSolvePass::Potential)
}

fn aggregate_stratum_carbon_for_pass(
    columns: &TileColumnsResult,
    interval_s: f64,
    required_pass: CoupledSolvePass,
) -> Result<BTreeMap<StratumId, StratumCarbonOperands>, VegetationError> {
    if !interval_s.is_finite() || interval_s <= 0.0 {
        return Err(VegetationError::Domain("E16 aggregation interval"));
    }
    if columns.columns.is_empty() {
        return Err(VegetationError::Receipt(
            "accepted E16 capped columns absent".into(),
        ));
    }
    let mut seen = BTreeSet::<OccupancyId>::new();
    let mut aggregate = BTreeMap::<StratumId, StratumCarbonOperands>::new();
    for column in &columns.columns {
        let tile_fraction = column.ledger.tile_fraction;
        if !tile_fraction.is_finite() || tile_fraction <= 0.0 || tile_fraction > 1.0 {
            return Err(VegetationError::Domain("E16 tile fraction"));
        }
        for occupancy in &column.occupancy_results {
            if occupancy.diagnostics.pass != required_pass {
                return Err(VegetationError::Receipt(
                    "E16 carbon operands do not match the required solve pass".into(),
                ));
            }
            if occupancy.occupancy_id.tile_id != column.tile_id
                || !seen.insert(occupancy.occupancy_id.clone())
            {
                return Err(VegetationError::Receipt("E16 occupancy identity".into()));
            }
            let carbon = occupancy.carbon_operands.ok_or(VegetationError::Receipt(
                "accepted E16 occupancy carbon operands absent".into(),
            ))?;
            let t10 = carbon.advanced_t10_k;
            let (gpp, rd) = integrate_occupancy(carbon, interval_s, tile_fraction)?;
            let entry = aggregate
                .entry(occupancy.occupancy_id.stratum_id.clone())
                .or_insert(StratumCarbonOperands {
                    advanced_t10_k: t10,
                    ..StratumCarbonOperands::default()
                });
            if entry.advanced_t10_k.to_bits() != t10.to_bits() {
                return Err(VegetationError::Receipt(
                    "shared E17 acclimation identity".into(),
                ));
            }
            entry.gross_primary_production_kg_c_m2 += gpp;
            entry.accepted_leaf_respiration_kg_c_m2 += rd;
        }
    }
    if aggregate.values().any(|value| {
        !value.gross_primary_production_kg_c_m2.is_finite()
            || !value.accepted_leaf_respiration_kg_c_m2.is_finite()
            || !value.advanced_t10_k.is_finite()
    }) {
        return Err(VegetationError::Domain("E16/E17 aggregate"));
    }
    Ok(aggregate)
}

pub(crate) fn integrate_occupancy(
    operands: OccupancyCarbonOperands,
    interval_s: f64,
    tile_fraction: f64,
) -> Result<(f64, f64), VegetationError> {
    integrate_class_carbon(
        ClassCarbonOperands {
            sun_leaf_area_m2_m2_tile_ground: operands.sun_leaf_area_m2_m2_tile_ground,
            shade_leaf_area_m2_m2_tile_ground: operands.shade_leaf_area_m2_m2_tile_ground,
            sun_gross_assimilation_umol_co2_m2_leaf_s: operands
                .sun_gross_assimilation_umol_co2_m2_leaf_s,
            shade_gross_assimilation_umol_co2_m2_leaf_s: operands
                .shade_gross_assimilation_umol_co2_m2_leaf_s,
            sun_dark_respiration_umol_co2_m2_leaf_s: operands
                .sun_dark_respiration_umol_co2_m2_leaf_s,
            shade_dark_respiration_umol_co2_m2_leaf_s: operands
                .shade_dark_respiration_umol_co2_m2_leaf_s,
        },
        interval_s,
        tile_fraction,
    )
}

pub(crate) fn integrate_class_carbon(
    operands: ClassCarbonOperands,
    interval_s: f64,
    tile_fraction: f64,
) -> Result<(f64, f64), VegetationError> {
    if operands.sun_gross_assimilation_umol_co2_m2_leaf_s < 0.0
        || operands.shade_gross_assimilation_umol_co2_m2_leaf_s < 0.0
    {
        return Err(VegetationError::Domain("E16 gross assimilation"));
    }
    let gpp = gpp_kg_c(
        interval_s,
        tile_fraction,
        operands.sun_gross_assimilation_umol_co2_m2_leaf_s,
        operands.sun_leaf_area_m2_m2_tile_ground,
        operands.shade_gross_assimilation_umol_co2_m2_leaf_s,
        operands.shade_leaf_area_m2_m2_tile_ground,
    )?;
    let sun_rd = leaf_rd_carbon_debit(
        operands.sun_dark_respiration_umol_co2_m2_leaf_s,
        operands.sun_leaf_area_m2_m2_tile_ground,
        interval_s,
        tile_fraction,
    )?;
    let shade_rd = leaf_rd_carbon_debit(
        operands.shade_dark_respiration_umol_co2_m2_leaf_s,
        operands.shade_leaf_area_m2_m2_tile_ground,
        interval_s,
        tile_fraction,
    )?;
    Ok((gpp, sun_rd + shade_rd))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operands() -> OccupancyCarbonOperands {
        OccupancyCarbonOperands {
            advanced_t10_k: 295.0,
            sun_leaf_area_m2_m2_tile_ground: 1.25,
            shade_leaf_area_m2_m2_tile_ground: 2.75,
            sun_gross_assimilation_umol_co2_m2_leaf_s: 12.0,
            shade_gross_assimilation_umol_co2_m2_leaf_s: 4.0,
            sun_dark_respiration_umol_co2_m2_leaf_s: 0.8,
            shade_dark_respiration_umol_co2_m2_leaf_s: 0.6,
        }
    }

    #[test]
    fn accepted_class_operands_integrate_gross_and_rd_once() {
        let actual = integrate_occupancy(operands(), 1_800.0, 0.35).expect("valid operands");
        let expected_gpp: f64 = 0.012_011e-6 * 1_800.0 * 0.35 * (12.0 * 1.25 + 4.0 * 2.75);
        // Frozen binary64 result for two separately integrated class debits;
        // summing classes before conversion is the one-bit poison here.
        let expected_rd = f64::from_bits(4_536_539_638_510_786_796);
        assert_eq!(actual.0.to_bits(), expected_gpp.to_bits());
        assert_eq!(actual.1.to_bits(), expected_rd.to_bits());
    }

    #[test]
    fn net_assimilation_and_missing_tile_weight_are_distinct_poisons() {
        let accepted = integrate_occupancy(operands(), 1_800.0, 0.35).expect("accepted");
        let mut net = operands();
        net.sun_gross_assimilation_umol_co2_m2_leaf_s -=
            net.sun_dark_respiration_umol_co2_m2_leaf_s;
        net.shade_gross_assimilation_umol_co2_m2_leaf_s -=
            net.shade_dark_respiration_umol_co2_m2_leaf_s;
        let net_result = integrate_occupancy(net, 1_800.0, 0.35).expect("finite poison");
        let unweighted = integrate_occupancy(operands(), 1_800.0, 1.0).expect("finite poison");
        assert_ne!(accepted.0.to_bits(), net_result.0.to_bits());
        assert_ne!(accepted.0.to_bits(), unweighted.0.to_bits());
        assert_eq!(accepted.1.to_bits(), net_result.1.to_bits());
    }
}
