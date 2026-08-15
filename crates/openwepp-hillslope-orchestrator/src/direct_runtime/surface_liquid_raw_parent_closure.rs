//! Frozen-parent mass custody reconstruction for independent surface-liquid closure.

use std::collections::BTreeMap;

use openwepp_land_surface_energy::OfeId;

use super::{
    DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError,
    DirectSurfaceLiquidErrorCode, checked_surface_liquid_add, contextual_comparison_failure,
};

type RawParentMassProjection = (BTreeMap<OfeId, f64>, BTreeMap<(OfeId, String), f64>);

pub(super) fn reconstruct_raw_parent_mass(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
) -> Result<RawParentMassProjection, DirectSurfaceLiquidError> {
    let mut totals = BTreeMap::<OfeId, f64>::new();
    let mut source_mass = BTreeMap::<(OfeId, String), f64>::new();
    for source in &operands.source_parcels {
        let total = totals.entry(source.basis_ofe_id.clone()).or_default();
        *total = checked_surface_liquid_add(*total, source.mass_kg_m2_basis_ofe_ground)
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &source.origin_store_key,
                    Some(source.source_parcel_id.clone()),
                    "raw parent OFE mass arithmetic",
                )
            })?;
        let accumulated = source_mass
            .entry((source.basis_ofe_id.clone(), source.source_parcel_id.clone()))
            .or_default();
        *accumulated = checked_surface_liquid_add(*accumulated, source.mass_kg_m2_basis_ofe_ground)
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &source.origin_store_key,
                    Some(source.source_parcel_id.clone()),
                    "raw parent source mass arithmetic",
                )
            })?;
    }
    Ok((totals, source_mass))
}
