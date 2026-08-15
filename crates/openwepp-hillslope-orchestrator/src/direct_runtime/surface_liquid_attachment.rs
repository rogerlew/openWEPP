//! Canonical error completion for attaching the optional surface-liquid owner.

use openwepp_land_surface_energy::{OfeId, SurfaceClass, WaterSourceType};
use sha2::{Digest, Sha256};

use super::{
    DirectGroundIngressMode, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError,
    DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes, DirectSurfaceLiquidStoreKey,
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
    let record = ofe_id.map_or_else(
        || configuration.records.first(),
        |ofe_id| {
            configuration
                .records
                .iter()
                .find(|record| &record.key.ofe_id == ofe_id)
        },
    );
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
