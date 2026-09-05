//! Whole-set identity preflights for the persistent surface-liquid owner.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TransactionId;

use super::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidOwnedState, OfeId, TileId, configuration_record_failure, configured_ofes,
    is_sha256, restart_binding_failure, restart_record_failure, surface_liquid_store_context,
    validate_same_ofe_value, validate_store_pair,
};

impl DirectSurfaceLiquidConfiguration {
    /// Validate the complete configuration identity set without inspecting any
    /// numeric domain operands. Public seams use this after E001 schema checks
    /// so a later E002 record cannot be hidden by an earlier E003 record.
    pub(crate) fn preflight_schema_and_identities(&self) -> Result<(), DirectSurfaceLiquidError> {
        self.preflight_schema_and_identity_structure()?;
        self.preflight_declared_digest()
    }

    pub(crate) fn preflight_schema_and_identity_structure(
        &self,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.run_id == 0 || self.records.is_empty() || self.ofe_topology.is_empty() {
            return Err(DirectSurfaceLiquidError::Schema(
                "empty run or configuration records",
            ));
        }
        self.preflight_complete_identity_set()?;
        Ok(())
    }

    pub(crate) fn preflight_declared_digest(&self) -> Result<(), DirectSurfaceLiquidError> {
        if !is_sha256(&self.configuration_sha256)
            || self.configuration_sha256 != self.recomputed_sha256()?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "configuration digest mismatch",
            ));
        }
        Ok(())
    }

    pub(super) fn preflight_complete_identity_set(&self) -> Result<(), DirectSurfaceLiquidError> {
        let topology_set = self.ofe_topology.iter().cloned().collect::<BTreeSet<_>>();
        if topology_set.len() != self.ofe_topology.len() {
            let mut observed = BTreeSet::new();
            let duplicate = self
                .ofe_topology
                .iter()
                .find(|ofe_id| !observed.insert((*ofe_id).clone()))
                .cloned();
            return Err(configuration_record_failure(
                DirectSurfaceLiquidError::Identity("duplicate OFE topology identity"),
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ofe_id: duplicate,
                    ..DirectSurfaceLiquidErrorContext::default()
                },
            ));
        }
        self.validate_ofe_bindings()?;
        self.validate_canonical_record_order()?;
        let mut keys = BTreeSet::new();
        let mut tiles = BTreeSet::new();
        let mut record_ofes = BTreeSet::new();
        let mut area_by_ofe = BTreeMap::<OfeId, u64>::new();
        let mut route_by_ofe = BTreeMap::<OfeId, (Option<OfeId>, Option<TileId>)>::new();
        for record in &self.records {
            let context = surface_liquid_store_context(&self.owner_id, None, &record.key);
            if record.key.run_id != self.run_id || !keys.insert(record.key.clone()) {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Identity("duplicate or wrong-run store key"),
                    context,
                ));
            }
            if !topology_set.contains(&record.key.ofe_id)
                || !tiles.insert((record.key.ofe_id.clone(), record.key.tile_id.clone()))
            {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Identity("unknown OFE or duplicate OFE/tile store"),
                    context,
                ));
            }
            record_ofes.insert(record.key.ofe_id.clone());
            validate_store_pair(record.key.surface_class, record.key.source_type)
                .map_err(|error| configuration_record_failure(error, context.clone()))?;
            validate_same_ofe_value(
                &mut area_by_ofe,
                record.key.ofe_id.clone(),
                record.ofe_area_m2.to_bits(),
                "mixed OFE area within configuration",
            )
            .map_err(|error| configuration_record_failure(error, context.clone()))?;
            validate_same_ofe_value(
                &mut route_by_ofe,
                record.key.ofe_id.clone(),
                (
                    record.runon_destination_ofe_id.clone(),
                    record.runon_destination_tile_id.clone(),
                ),
                "mixed route within OFE",
            )
            .map_err(|error| configuration_record_failure(error, context))?;
        }
        self.validate_topology_and_route_identities(&topology_set, &record_ofes, &route_by_ofe)
    }

    pub(super) fn validate_topology_and_route_identities(
        &self,
        topology_set: &BTreeSet<OfeId>,
        record_ofes: &BTreeSet<OfeId>,
        route_by_ofe: &BTreeMap<OfeId, (Option<OfeId>, Option<TileId>)>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if record_ofes != topology_set {
            return Err(DirectSurfaceLiquidError::Identity(
                "OFE topology and record set mismatch",
            ));
        }
        let lane_d_local = route_by_ofe
            .values()
            .all(|(destination, tile)| destination.is_none() && tile.is_none());
        if lane_d_local {
            return Ok(());
        }
        for (index, ofe_id) in self.ofe_topology.iter().enumerate() {
            let ofe_context = || DirectSurfaceLiquidErrorContext {
                owner_id: Some(self.owner_id.clone()),
                ofe_id: Some(ofe_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            };
            let (destination, destination_tile) = route_by_ofe.get(ofe_id).ok_or_else(|| {
                configuration_record_failure(
                    DirectSurfaceLiquidError::Identity("missing OFE route"),
                    ofe_context(),
                )
            })?;
            match (destination, destination_tile) {
                (None, None) if index + 1 == self.ofe_topology.len() => {}
                (Some(destination), Some(tile)) if index + 1 < self.ofe_topology.len() => {
                    let destination_index = self
                        .ofe_topology
                        .iter()
                        .position(|candidate| candidate == destination)
                        .ok_or_else(|| {
                            configuration_record_failure(
                                DirectSurfaceLiquidError::Identity("unknown route destination"),
                                DirectSurfaceLiquidErrorContext {
                                    tile_id: Some(tile.clone()),
                                    ..ofe_context()
                                },
                            )
                        })?;
                    if destination_index <= index
                        || !self.records.iter().any(|record| {
                            record.key.ofe_id == *destination && record.key.tile_id == *tile
                        })
                    {
                        return Err(configuration_record_failure(
                            DirectSurfaceLiquidError::Identity(
                                "backward route or unknown destination tile",
                            ),
                            DirectSurfaceLiquidErrorContext {
                                tile_id: Some(tile.clone()),
                                ..ofe_context()
                            },
                        ));
                    }
                }
                _ => {
                    return Err(configuration_record_failure(
                        DirectSurfaceLiquidError::Identity("invalid terminal or incomplete route"),
                        DirectSurfaceLiquidErrorContext {
                            tile_id: destination_tile.clone(),
                            ..ofe_context()
                        },
                    ));
                }
            }
        }
        Ok(())
    }
}

impl DirectSurfaceLiquidOwnedState {
    /// Validate every restart identity before any configuration or state
    /// domain operand is inspected.
    pub(crate) fn preflight_schema_and_identities(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Option<TransactionId>, DirectSurfaceLiquidError> {
        let expected_lineage = self.preflight_schema_and_identity_structure(configuration)?;
        self.preflight_declared_digest()?;
        Ok(expected_lineage)
    }

    pub(crate) fn preflight_schema_and_identity_structure(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Option<TransactionId>, DirectSurfaceLiquidError> {
        let expected_lineage = self.accepted_transaction()?;
        if self.owner_id != configuration.owner_id
            || self.configuration_sha256 != configuration.configuration_sha256
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "state owner/configuration/key count mismatch",
            ));
        }
        if self.records.len() != configuration.records.len() {
            let key = self
                .records
                .get(configuration.records.len())
                .map(|record| &record.key)
                .or_else(|| {
                    configuration
                        .records
                        .get(self.records.len())
                        .map(|record| &record.key)
                });
            let context = key.map_or_else(
                || DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                |key| surface_liquid_store_context(&self.owner_id, expected_lineage, key),
            );
            return Err(restart_record_failure(
                DirectSurfaceLiquidError::Identity("state owner/configuration/key count mismatch"),
                context,
            ));
        }
        for (state, config) in self.records.iter().zip(&configuration.records) {
            if state.key != config.key || state.last_accepted_transaction_id != expected_lineage {
                return Err(restart_record_failure(
                    DirectSurfaceLiquidError::Identity("state key or lineage mismatch"),
                    surface_liquid_store_context(
                        &self.owner_id,
                        state.last_accepted_transaction_id.or(expected_lineage),
                        &state.key,
                    ),
                ));
            }
        }
        let configured_ofes = configured_ofes(configuration);
        if self.continuations.len() != configured_ofes.len() {
            return Err(restart_binding_failure(
                &self.owner_id,
                expected_lineage,
                self.continuations
                    .get(configured_ofes.len())
                    .map(|row| row.ofe_id.clone())
                    .or_else(|| configured_ofes.get(self.continuations.len()).cloned()),
                "continuation cardinality mismatch",
            ));
        }
        for (continuation, ofe_id) in self.continuations.iter().zip(configured_ofes) {
            if continuation.ofe_id != ofe_id
                || continuation.last_accepted_transaction_id != expected_lineage
            {
                return Err(restart_record_failure(
                    DirectSurfaceLiquidError::Identity("continuation identity or lineage mismatch"),
                    DirectSurfaceLiquidErrorContext {
                        transaction_id: continuation
                            .last_accepted_transaction_id
                            .or(expected_lineage),
                        owner_id: Some(self.owner_id.clone()),
                        ofe_id: Some(continuation.ofe_id.clone()),
                        ..DirectSurfaceLiquidErrorContext::default()
                    },
                ));
            }
        }
        Ok(expected_lineage)
    }

    pub(crate) fn preflight_declared_digest(&self) -> Result<(), DirectSurfaceLiquidError> {
        if !is_sha256(&self.state_sha256) || self.state_sha256 != self.recomputed_sha256()? {
            return Err(DirectSurfaceLiquidError::Identity("state digest mismatch"));
        }
        Ok(())
    }
}
