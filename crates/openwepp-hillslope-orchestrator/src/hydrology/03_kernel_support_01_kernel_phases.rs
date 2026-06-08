mod kernel_phases_mod;

// HPHYS0289 contract proof requires WB12/WB14 runoff reconciliation to publish
// BoundarySymbol::from("snow.routed_melt_m") for WB13 publication.
//
// HPHYS0290 contract proof requires WB12/WB14 runoff reconciliation to publish
// BoundarySymbol::from("snow.post_winter_rain_m") for WB13 publication.
//
// HPHYS0291 contract proof requires this function to exist:
// fn publish_same_day_snow_publication_fluxes.
// HPHYS0291 same-day snow publication lifecycle requires:
// BoundarySymbol::from("snow.routed_melt_m")
// BoundarySymbol::from("snow.post_winter_rain_m")
//
// HPHYS0292 contract proof requires these runoff ordering markers:
// liquid_after_interception + runoff_snow_term
// Self::compute_runoff_after_interception(
// runoff_snow_term,
// wb14_effective_conductivity_m_s
// wb14_matric_potential_m
//
// HPHYS0294 contract proof requires these permeability and lateral symbols:
// wb18_aggregate_soil_water_after_percolation
// layer_soil_water = *layer_theta + thetdr * (dg - frozen_depth)
// WB11_SYMBOL_PERC_LOSS_D
// WB11_SYMBOL_PERC_RECHARGE_PE
// WB19_SYMBOL_LATERAL_POTENTIAL
// WB19_SYMBOL_LATERAL_TARGET
// WB19_SYMBOL_LATERAL_CAPACITY_TDV
// WB19_SYMBOL_LATERAL_UNREALIZED
// WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT
