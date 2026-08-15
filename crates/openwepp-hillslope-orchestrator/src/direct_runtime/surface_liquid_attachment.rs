//! Canonical error completion for attaching the optional surface-liquid owner.

use openwepp_land_surface_energy::{
    CondensationCredit, GroundWaterKey, OfeId, RequestingComponent, StandGroundWaterAmountBasis,
    SurfaceClass, WaterAmount, WaterAuthorization, WaterAuthorizationReason, WaterSourceType,
};
use sha2::{Digest, Sha256};

use super::{
    DirectGroundIngressMode, DirectLaneFrame, DirectSurfaceLiquidArbitration,
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidRollbackHashes,
    DirectSurfaceLiquidStoreKey,
};

struct RawAttemptHash(Sha256);

impl RawAttemptHash {
    fn new(domain: &'static str) -> Self {
        let mut value = Self(Sha256::new());
        value.bytes("domain", domain.as_bytes());
        value
    }

    fn bytes(&mut self, tag: &'static str, value: &[u8]) {
        self.0.update((tag.len() as u64).to_be_bytes());
        self.0.update(tag.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn string(&mut self, tag: &'static str, value: &str) {
        self.bytes(tag, value.as_bytes());
    }

    fn u64(&mut self, tag: &'static str, value: u64) {
        self.bytes(tag, &value.to_be_bytes());
    }

    fn option_string(&mut self, tag: &'static str, value: Option<&str>) {
        self.u64("option-present", u64::from(value.is_some()));
        if let Some(value) = value {
            self.string(tag, value);
        }
    }

    fn option_transaction(&mut self, value: Option<u128>) {
        self.u64("transaction-present", u64::from(value.is_some()));
        if let Some(value) = value {
            self.bytes("transaction", &value.to_be_bytes());
        }
    }

    fn store_key(&mut self, key: &DirectSurfaceLiquidStoreKey) {
        self.u64("key-run", key.run_id);
        self.string("key-ofe", key.ofe_id.as_str());
        self.string("key-tile", key.tile_id.as_str());
        self.string("key-surface", key.surface_id.as_str());
        self.u64(
            "key-surface-class",
            match key.surface_class {
                SurfaceClass::BareMineralSoil => 0,
                SurfaceClass::ForestLitter => 1,
            },
        );
        self.u64(
            "key-source-type",
            match key.source_type {
                WaterSourceType::SurfaceLiquid => 0,
                WaterSourceType::LitterLiquid => 1,
                WaterSourceType::SoilLayerLiquid => 2,
            },
        );
        self.string("key-source", key.source_id.as_str());
    }

    fn water_key(&mut self, key: &GroundWaterKey) {
        self.bytes("water-transaction", &key.transaction_id.0.to_be_bytes());
        self.string("water-owner", key.requesting_owner_id.as_str());
        self.u64(
            "water-component",
            match key.requesting_component {
                RequestingComponent::VegetationRoot => 0,
                RequestingComponent::GroundSurface => 1,
            },
        );
        self.string("water-ofe", key.ofe_id.as_str());
        self.string("water-requesting-tile", key.requesting_tile_id.as_str());
        self.option_string(
            "water-occupancy",
            key.occupancy_id
                .as_ref()
                .map(openwepp_land_surface_energy::ComponentId::as_str),
        );
        self.option_string(
            "water-surface",
            key.surface_id
                .as_ref()
                .map(openwepp_land_surface_energy::SurfaceId::as_str),
        );
        self.u64(
            "water-surface-class-present",
            u64::from(key.surface_class.is_some()),
        );
        if let Some(surface_class) = key.surface_class {
            self.u64(
                "water-surface-class",
                match surface_class {
                    SurfaceClass::BareMineralSoil => 0,
                    SurfaceClass::ForestLitter => 1,
                },
            );
        }
        self.u64(
            "water-source-type",
            match key.source_type {
                WaterSourceType::SurfaceLiquid => 0,
                WaterSourceType::LitterLiquid => 1,
                WaterSourceType::SoilLayerLiquid => 2,
            },
        );
        self.string("water-source", key.source_id.as_str());
        self.option_string(
            "water-source-tile",
            key.source_tile_id
                .as_ref()
                .map(openwepp_kernel_contract::TileId::as_str),
        );
        self.option_string(
            "water-soil-layer",
            key.soil_layer_id
                .as_ref()
                .map(openwepp_kernel_contract::SoilLayerId::as_str),
        );
        self.u64(
            "water-basis",
            match key.amount_basis {
                StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval => 0,
            },
        );
    }

    fn water_amount(&mut self, tag: &'static str, value: &WaterAmount) {
        self.string("water-row-kind", tag);
        self.water_key(&value.key);
        self.u64("water-amount", value.amount_kg_m2_stand_ground.to_bits());
    }

    fn water_authorization(&mut self, value: &WaterAuthorization) {
        self.water_key(&value.key);
        self.u64(
            "authorization-amount",
            value.amount_kg_m2_stand_ground.to_bits(),
        );
        self.u64(
            "authorization-reason",
            match value.reason {
                WaterAuthorizationReason::FullSupply => 0,
                WaterAuthorizationReason::ProportionalSupply => 1,
                WaterAuthorizationReason::ZeroSupply => 2,
                WaterAuthorizationReason::DrySource => 3,
                WaterAuthorizationReason::FrozenSource => 4,
                WaterAuthorizationReason::InaccessibleSource => 5,
            },
        );
    }

    fn condensation_credit(&mut self, value: &CondensationCredit) {
        self.bytes(
            "condensation-transaction",
            &value.transaction_id.0.to_be_bytes(),
        );
        self.string("condensation-owner", value.hydrology_owner_id.as_str());
        self.string("condensation-ofe", value.ofe_id.as_str());
        self.string("condensation-tile", value.tile_id.as_str());
        self.string("condensation-surface", value.surface_id.as_str());
        self.u64(
            "condensation-amount",
            value.amount_kg_m2_stand_ground.to_bits(),
        );
        self.u64(
            "condensation-basis",
            match value.amount_basis {
                StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval => 0,
            },
        );
        self.u64("condensation-temperature", value.temperature_k.to_bits());
        self.u64(
            "condensation-enthalpy",
            value.specific_liquid_enthalpy_j_kg.to_bits(),
        );
    }

    fn finish(self) -> String {
        format!("{:x}", self.0.finalize())
    }
}

pub(crate) fn surface_liquid_raw_bytes_sha256(domain: &'static str, bytes: &[u8]) -> String {
    let mut hash = RawAttemptHash::new(domain);
    hash.bytes("attempted-bytes", bytes);
    hash.finish()
}

fn hash_raw_configuration(
    hash: &mut RawAttemptHash,
    configuration: &DirectSurfaceLiquidConfiguration,
) {
    hash.string("configuration-owner", configuration.owner_id.as_str());
    hash.u64("configuration-run", configuration.run_id);
    hash.string(
        "configuration-declared-digest",
        &configuration.configuration_sha256,
    );
    hash.u64("topology-count", configuration.ofe_topology.len() as u64);
    for ofe_id in &configuration.ofe_topology {
        hash.string("topology-ofe", ofe_id.as_str());
    }
    hash.u64("binding-count", configuration.ofe_bindings.len() as u64);
    for binding in &configuration.ofe_bindings {
        hash.string("binding-ofe", binding.ofe_id.as_str());
        hash.u64("binding-lane-index", binding.production_lane_index as u64);
        hash.u64("binding-lane-id", u64::from(binding.production_lane_id));
        hash.u64(
            "binding-layer-count",
            binding.ordered_soil_layer_ids.len() as u64,
        );
        for layer_id in &binding.ordered_soil_layer_ids {
            hash.string("binding-layer", layer_id.as_str());
        }
        hash.string(
            "binding-thermal-layer",
            binding.infiltration_soil_thermal_layer_id.as_str(),
        );
    }
    hash.u64(
        "configuration-record-count",
        configuration.records.len() as u64,
    );
    for record in &configuration.records {
        hash.store_key(&record.key);
        hash.u64("record-tile-fraction", record.tile_fraction.to_bits());
        hash.u64("record-capacity", record.capacity_kg_m2_tile.to_bits());
        hash.u64("record-ofe-area", record.ofe_area_m2.to_bits());
        hash.u64(
            "record-ingress-mode",
            match record.ground_ingress_mode {
                DirectGroundIngressMode::OpenRawPrecipitation => 0,
                DirectGroundIngressMode::CoveredCanopyRelease => 1,
            },
        );
        hash.option_string(
            "record-destination-ofe",
            record.runon_destination_ofe_id.as_ref().map(OfeId::as_str),
        );
        hash.option_string(
            "record-destination-tile",
            record
                .runon_destination_tile_id
                .as_ref()
                .map(openwepp_kernel_contract::TileId::as_str),
        );
    }
}

fn hash_raw_state(hash: &mut RawAttemptHash, state: &DirectSurfaceLiquidOwnedState) {
    hash.string("state-owner", state.owner_id.as_str());
    hash.string("state-configuration-digest", &state.configuration_sha256);
    hash.string("state-declared-digest", &state.state_sha256);
    hash.u64("state-record-count", state.records.len() as u64);
    for record in &state.records {
        hash.store_key(&record.key);
        hash.u64("state-liquid", record.liquid_kg_m2_tile.to_bits());
        hash.option_transaction(record.last_accepted_transaction_id.map(|value| value.0));
    }
    hash.u64("continuation-count", state.continuations.len() as u64);
    for continuation in &state.continuations {
        hash.string("continuation-ofe", continuation.ofe_id.as_str());
        hash.u64("continuation-day", continuation.day_index as u64);
        hash.u64(
            "continuation-interval",
            u64::from(continuation.next_interval_index),
        );
        hash.u64(
            "continuation-supply",
            continuation.cumulative_supply_m.to_bits(),
        );
        hash.u64(
            "continuation-infiltration",
            continuation.cumulative_infiltration_m.to_bits(),
        );
        hash.option_transaction(
            continuation
                .last_accepted_transaction_id
                .map(|value| value.0),
        );
    }
}

pub(crate) fn surface_liquid_raw_state_sha256(state: &DirectSurfaceLiquidOwnedState) -> String {
    let mut hash = RawAttemptHash::new("openwepp-surface-liquid-raw-state-v1");
    hash_raw_state(&mut hash, state);
    hash.finish()
}

pub(crate) fn surface_liquid_raw_authorization_attempt_sha256(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning: &DirectSurfaceLiquidOwnedState,
    transaction_id: openwepp_kernel_contract::TransactionId,
    expected_predecessor: Option<openwepp_kernel_contract::TransactionId>,
    requests: &[WaterAmount],
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-surface-liquid-authorization-attempt-v1");
    hash_raw_configuration(&mut hash, configuration);
    hash_raw_state(&mut hash, beginning);
    hash.bytes("candidate-transaction", &transaction_id.0.to_be_bytes());
    hash.option_transaction(expected_predecessor.map(|value| value.0));
    hash.u64("request-count", requests.len() as u64);
    for request in requests {
        hash.water_amount("request", request);
    }
    hash.finish()
}

pub(crate) fn surface_liquid_raw_resource_attempt_sha256(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-surface-liquid-resource-attempt-v1");
    hash_raw_configuration(&mut hash, configuration);
    hash_raw_state(&mut hash, arbitration.beginning_state());
    hash.bytes(
        "candidate-transaction",
        &arbitration.transaction_id().0.to_be_bytes(),
    );
    hash.option_transaction(arbitration.expected_predecessor().map(|value| value.0));
    hash.u64("request-count", arbitration.requests().len() as u64);
    for request in arbitration.requests() {
        hash.water_amount("request", request);
    }
    hash.u64(
        "authorization-count",
        arbitration.authorizations().len() as u64,
    );
    for authorization in arbitration.authorizations() {
        hash.water_authorization(authorization);
    }
    hash.u64(
        "request-store-key-count",
        arbitration.request_store_keys().len() as u64,
    );
    for store_key in arbitration.request_store_keys() {
        hash.store_key(store_key);
    }
    hash.u64("finalized-use-count", finalized_uses.len() as u64);
    for finalized in finalized_uses {
        hash.water_amount("finalized-use", finalized);
    }
    hash.u64("condensation-count", condensation_credits.len() as u64);
    for credit in condensation_credits {
        hash.condensation_credit(credit);
    }
    hash.finish()
}

pub(crate) fn surface_liquid_raw_candidate_attempt_sha256(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &DirectSurfaceLiquidResourceCandidate,
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-surface-liquid-candidate-attempt-v1");
    hash_raw_configuration(&mut hash, configuration);
    hash_raw_state(&mut hash, candidate.beginning_state());
    hash_raw_state(&mut hash, candidate.working_state());
    hash.bytes(
        "candidate-transaction",
        &candidate.transaction_id().0.to_be_bytes(),
    );
    hash.option_transaction(candidate.expected_predecessor().map(|value| value.0));
    hash.u64("request-count", candidate.requests().len() as u64);
    for request in candidate.requests() {
        hash.water_amount("request", request);
    }
    hash.u64(
        "authorization-count",
        candidate.authorizations().len() as u64,
    );
    for authorization in candidate.authorizations() {
        hash.water_authorization(authorization);
    }
    hash.u64(
        "request-store-key-count",
        candidate.request_store_keys().len() as u64,
    );
    for store_key in candidate.request_store_keys() {
        hash.store_key(store_key);
    }
    hash.u64(
        "finalized-use-count",
        candidate.finalized_uses().len() as u64,
    );
    for finalized in candidate.finalized_uses() {
        hash.water_amount("finalized-use", finalized);
    }
    hash.u64(
        "condensation-count",
        candidate.condensation_credits().len() as u64,
    );
    for credit in candidate.condensation_credits() {
        hash.condensation_credit(credit);
    }
    hash.u64(
        "overflow-count",
        candidate.condensation_overflow().len() as u64,
    );
    for overflow in candidate.condensation_overflow() {
        hash.store_key(&overflow.store_key);
        hash.u64(
            "overflow-amount",
            overflow.amount_kg_m2_ofe_ground.to_bits(),
        );
        hash.u64("overflow-temperature", overflow.temperature_k.to_bits());
        hash.u64(
            "overflow-enthalpy",
            overflow.specific_liquid_enthalpy_j_kg.to_bits(),
        );
    }
    hash.finish()
}

pub(crate) fn surface_liquid_raw_attempt_sha256(
    configuration: &DirectSurfaceLiquidConfiguration,
    state: Option<&DirectSurfaceLiquidOwnedState>,
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-surface-liquid-raw-attempt-v1");
    hash_raw_configuration(&mut hash, configuration);
    hash.u64("state-present", u64::from(state.is_some()));
    if let Some(state) = state {
        hash_raw_state(&mut hash, state);
    }
    hash.finish()
}

pub(crate) fn surface_liquid_raw_snapshot_sha256(
    production_snapshot: &[u8],
    state: Option<&DirectSurfaceLiquidOwnedState>,
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-unified-hydrology-raw-snapshot-v1");
    hash.bytes("production-snapshot", production_snapshot);
    hash.u64("surface-state-present", u64::from(state.is_some()));
    if let Some(state) = state {
        hash_raw_state(&mut hash, state);
    }
    hash.finish()
}

pub(crate) fn surface_liquid_raw_snapshot_attempt_sha256(
    production_snapshot: &[u8],
    configuration: &DirectSurfaceLiquidConfiguration,
    state: Option<&DirectSurfaceLiquidOwnedState>,
) -> String {
    let mut hash = RawAttemptHash::new("openwepp-unified-hydrology-raw-attempt-v1");
    hash.bytes("production-snapshot", production_snapshot);
    hash_raw_configuration(&mut hash, configuration);
    hash.u64("surface-state-present", u64::from(state.is_some()));
    if let Some(state) = state {
        hash_raw_state(&mut hash, state);
    }
    hash.finish()
}

pub(crate) fn surface_liquid_attachment_hashes(
    configuration: &DirectSurfaceLiquidConfiguration,
    state: &DirectSurfaceLiquidOwnedState,
    beginning: Option<&DirectSurfaceLiquidOwnedState>,
) -> (Option<String>, Option<String>) {
    (
        beginning.map(surface_liquid_raw_state_sha256),
        Some(surface_liquid_raw_attempt_sha256(
            configuration,
            Some(state),
        )),
    )
}

pub(crate) fn surface_liquid_configuration_context(
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
) -> DirectSurfaceLiquidErrorContext {
    let record = ofe_id.and_then(|ofe_id| {
        configuration
            .records
            .iter()
            .find(|record| &record.key.ofe_id == ofe_id)
    });
    record.map_or_else(
        || DirectSurfaceLiquidErrorContext {
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        |record| DirectSurfaceLiquidErrorContext {
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: Some(record.key.ofe_id.clone()),
            tile_id: Some(record.key.tile_id.clone()),
            surface_id: Some(record.key.surface_id.clone()),
            source_id: Some(record.key.source_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
    )
}

pub(crate) fn surface_liquid_state_context(
    state: &DirectSurfaceLiquidOwnedState,
) -> DirectSurfaceLiquidErrorContext {
    state.records.first().map_or_else(
        || DirectSurfaceLiquidErrorContext {
            owner_id: Some(state.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        |record| DirectSurfaceLiquidErrorContext {
            transaction_id: record.last_accepted_transaction_id,
            owner_id: Some(state.owner_id.clone()),
            ofe_id: Some(record.key.ofe_id.clone()),
            tile_id: Some(record.key.tile_id.clone()),
            surface_id: Some(record.key.surface_id.clone()),
            source_id: Some(record.key.source_id.clone()),
            parcel_id: None,
        },
    )
}

pub(crate) fn surface_liquid_attachment_error(
    error: DirectSurfaceLiquidError,
    phase: DirectSurfaceLiquidPhase,
    fallback_context: DirectSurfaceLiquidErrorContext,
    beginning_owner_sha256: Option<String>,
    attempted_owner_sha256: Option<String>,
) -> DirectSurfaceLiquidError {
    let code = error.code();
    let mut completed = error.complete_context(
        code,
        phase,
        fallback_context,
        beginning_owner_sha256.clone(),
        attempted_owner_sha256.clone(),
    );
    if let DirectSurfaceLiquidError::Failure(failure) = &mut completed {
        failure.rollback.beginning_owner_sha256 = beginning_owner_sha256;
        failure.rollback.attempted_owner_sha256 = attempted_owner_sha256;
    }
    completed
}

pub(crate) fn surface_liquid_frame_identity_error(
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
    beginning_owner_sha256: Option<String>,
    attempted_owner_sha256: Option<String>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E002,
        DirectSurfaceLiquidPhase::Configuration,
        surface_liquid_configuration_context(configuration, ofe_id),
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256,
            attempted_owner_sha256,
        },
        detail,
    )
}

pub(crate) fn validate_surface_liquid_frame_identities(
    run_id: u64,
    lanes: &[DirectLaneFrame],
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_owner_sha256: Option<String>,
    attempted_owner_sha256: Option<String>,
) -> Result<(), DirectSurfaceLiquidError> {
    if configuration.run_id != run_id {
        return Err(surface_liquid_frame_identity_error(
            configuration,
            None,
            beginning_owner_sha256,
            attempted_owner_sha256,
            "surface-liquid run identity does not match the direct frame",
        ));
    }
    if configuration.ofe_bindings.len() != lanes.len()
        || configuration.ofe_topology.len() != lanes.len()
    {
        let excess_configured_ofe = (configuration.ofe_topology.len() > lanes.len())
            .then(|| configuration.ofe_topology.get(lanes.len()))
            .flatten();
        return Err(surface_liquid_frame_identity_error(
            configuration,
            excess_configured_ofe,
            beginning_owner_sha256,
            attempted_owner_sha256,
            "surface-liquid production lane cardinality does not match the direct frame",
        ));
    }
    for (topology_index, (ofe_id, binding)) in configuration
        .ofe_topology
        .iter()
        .zip(&configuration.ofe_bindings)
        .enumerate()
    {
        let lane = &lanes[topology_index];
        if &binding.ofe_id != ofe_id
            || binding.production_lane_index != topology_index
            || binding.production_lane_id != lane.lane_id
        {
            return Err(surface_liquid_frame_identity_error(
                configuration,
                Some(ofe_id),
                beginning_owner_sha256,
                attempted_owner_sha256,
                "surface-liquid production lane identity does not match the direct frame",
            ));
        }
        if binding.ordered_soil_layer_ids.len() != lane.subsurface_layers.len() {
            return Err(surface_liquid_frame_identity_error(
                configuration,
                Some(ofe_id),
                beginning_owner_sha256,
                attempted_owner_sha256,
                "surface-liquid production soil-layer cardinality does not match the direct frame",
            ));
        }
        if let Some(record) = configuration.records.iter().find(|record| {
            record.key.ofe_id == *ofe_id
                && record.ofe_area_m2.is_finite()
                && lane.area_m2.is_finite()
                && record.ofe_area_m2.to_bits() != lane.area_m2.to_bits()
        }) {
            return Err(surface_liquid_frame_identity_error(
                configuration,
                Some(&record.key.ofe_id),
                beginning_owner_sha256,
                attempted_owner_sha256,
                "surface-liquid configured OFE area does not match the direct production lane",
            ));
        }
    }
    Ok(())
}
