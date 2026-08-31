use openwepp_hillslope_orchestrator::STAGE3_V11_PARENT_SUPPORT_NS;
use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, PreparedSoilThermalSupportV2, Sha256Digest,
    SoilThermalReceiptFreeOwnerSealsV2, SoilThermalSnapshot, SoilThermalV2MigrationIdentity,
    migrate_soil_thermal_v1_to_v2, prepare_soil_thermal_support_v2,
    seal_soil_thermal_receipt_free_owner_v2,
};

use super::{HillslopeCliError, failure, nested};

pub(super) fn bootstrap_soil_thermal_v2(
    v1_owner: &SoilThermalSnapshot,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    live_lane_count: usize,
    transaction_id: TransactionId,
    run_id: &str,
    receipt_chain_sha256: Sha256Digest,
) -> Result<
    (
        PreparedSoilThermalSupportV2,
        SoilThermalReceiptFreeOwnerSealsV2,
    ),
    HillslopeCliError,
> {
    let soil_configuration = &lse_configuration.soil_thermal_configuration;
    if v1_owner.owner_id != soil_configuration.owner_id
        || v1_owner.configuration_sha256 != soil_configuration.configuration_sha256
        || v1_owner.ofes.len() != lse_configuration.ofes.len()
        || v1_owner.ofes.len() != live_lane_count
        || v1_owner
            .ofes
            .iter()
            .zip(&lse_configuration.ofes)
            .any(|(owner_ofe, configured_ofe)| {
                owner_ofe.ofe_id != configured_ofe.ofe_id
                    || owner_ofe.ordered_layers.len() != configured_ofe.soil_interface_layers.len()
                    || owner_ofe
                        .ordered_layers
                        .iter()
                        .zip(&configured_ofe.soil_interface_layers)
                        .any(|(owner_layer, configured_layer)| {
                            owner_layer.layer_id != configured_layer.layer_id
                        })
            })
    {
        return Err(failure(
            "V1-to-V2 soil bootstrap owner/configuration/topology join",
        ));
    }
    let expected_transaction_id = v1_owner
        .last_accepted_transaction_id
        .map_or(Some(1), |predecessor| predecessor.0.checked_add(1))
        .ok_or_else(|| failure("V1-to-V2 soil bootstrap transaction overflow"))?;
    if transaction_id.0 != expected_transaction_id {
        return Err(failure(format!(
            "V1-to-V2 soil bootstrap requires the exact successor transaction {expected_transaction_id}, observed {}",
            transaction_id.0
        )));
    }
    let migrated = migrate_soil_thermal_v1_to_v2(
        v1_owner,
        SoilThermalV2MigrationIdentity {
            model_version: soil_configuration.model_version.clone(),
            model_definition_sha256: soil_configuration.model_definition_sha256.clone(),
            run_id: run_id.to_owned(),
            transaction_id,
            support_start_ns: 0,
            support_end_ns: STAGE3_V11_PARENT_SUPPORT_NS,
            receipt_chain_sha256,
        },
    )
    .map_err(nested)?;
    let prepared =
        prepare_soil_thermal_support_v2(&migrated, transaction_id, 0, STAGE3_V11_PARENT_SUPPORT_NS)
            .map_err(nested)?;
    let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).map_err(nested)?;
    Ok((prepared, seals))
}
