#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3ParentIntegratedBoundaryLedgerV1 {
    pub by_lane: BTreeMap<u32, Stage3IntegratedBoundaryLedgerV1>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3IntegratedBoundaryLedgerV1 {
    pub sensible_energy_into_snow_j_m2: f64,
    pub vapor_mass_into_snow_kg_m2: f64,
    pub latent_energy_into_snow_j_m2: f64,
    pub shortwave_energy_into_snow_j_m2: f64,
    pub net_longwave_energy_into_snow_j_m2: f64,
}

fn reconstruct_integrated_boundary_ledger(
    subslabs: &[Stage3CoupledSubslabReceiptV1],
) -> Stage3ParentIntegratedBoundaryLedgerV1 {
    let mut ledger = Stage3ParentIntegratedBoundaryLedgerV1::default();
    for subslab in subslabs {
        let duration_s = f64::from_bits(subslab.support.duration_s_bits());
        for (lane_id, receipt) in &subslab.lane_receipts {
            let lane = ledger.by_lane.entry(*lane_id).or_default();
            lane.sensible_energy_into_snow_j_m2 +=
                -receipt.aggregate_sensible_to_canopy_air_w_m2 * duration_s;
            lane.vapor_mass_into_snow_kg_m2 +=
                -receipt.aggregate_vapor_to_canopy_air_kg_m2_s * duration_s;
            lane.latent_energy_into_snow_j_m2 +=
                -receipt.aggregate_latent_energy_to_canopy_air_j_m2;
            lane.shortwave_energy_into_snow_j_m2 +=
                receipt.aggregate_snow_absorbed_shortwave_w_m2 * duration_s;
            lane.net_longwave_energy_into_snow_j_m2 +=
                receipt.aggregate_snow_net_longwave_w_m2 * duration_s;
        }
    }
    ledger
}
