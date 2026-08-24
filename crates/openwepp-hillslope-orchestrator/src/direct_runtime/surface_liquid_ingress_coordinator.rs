#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectWb14CoupledChildBindingV1 {
    pub proposed_upper_bound_s_bits: u64,
    pub coupled_parent_transaction_sha256: [u8; 32],
    pub accepted_slab_sha256: [u8; 32],
    pub parent_beginning_complete_owner_set_sha256: [u8; 32],
    pub parent_support_start_ns: u128,
    pub parent_support_end_ns: u128,
    pub child_support_start_ns: u128,
    pub child_support_end_ns: u128,
}

pub(crate) fn validate_wb14_child_replay_binding(
    bytes: &[u8],
    binding: DirectWb14CoupledChildBindingV1,
) -> Result<(), DirectSurfaceLiquidError> {
    let rows: Vec<(OfeId, DirectWb14ParentIntervalV1)> = serde_json::from_slice(bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay decoding"))?;
    if rows.is_empty() {
        return Err(DirectSurfaceLiquidError::Identity("empty WB14 replay set"));
    }
    let mut predecessor: Option<OfeId> = None;
    let mut previous_queue_after = None;
    for (ofe_id, authority) in rows {
        if predecessor.as_ref().is_some_and(|value| value >= &ofe_id) {
            return Err(DirectSurfaceLiquidError::Identity("WB14 replay OFE order"));
        }
        predecessor = Some(ofe_id.clone());
        authority
            .validate_coupled_child_binding(
                binding.coupled_parent_transaction_sha256,
                binding.parent_beginning_complete_owner_set_sha256,
                binding.parent_support_start_ns,
                binding.parent_support_end_ns,
                binding.child_support_start_ns,
                binding.child_support_end_ns,
                binding.proposed_upper_bound_s_bits,
                binding.accepted_slab_sha256,
                Sha256::digest(ofe_id.as_str().as_bytes()).into(),
            )
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay/coupled join"))?;
        let child = authority
            .receipts()
            .last()
            .ok_or(DirectSurfaceLiquidError::Identity("WB14 replay child receipt"))?;
        if previous_queue_after.is_some_and(|digest| {
            digest != child.pending_routed_parcels_before_sha256
        }) {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay routed-queue adjacency",
            ));
        }
        previous_queue_after = Some(child.pending_routed_parcels_after_sha256);
    }
    Ok(())
}

pub(crate) fn validate_wb14_parent_replay(
    child_bytes: &[u8],
    parent_bytes: &[u8],
) -> Result<(), DirectSurfaceLiquidError> {
    let child_rows: Vec<(OfeId, DirectWb14ParentIntervalV1)> = serde_json::from_slice(child_bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay decoding"))?;
    let parent_rows: Vec<(OfeId, DirectWb14ParentFinalizationV1)> =
        serde_json::from_slice(parent_bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent replay decoding"))?;
    if child_rows.len() != parent_rows.len() || child_rows.is_empty() {
        return Err(DirectSurfaceLiquidError::Identity(
            "WB14 parent replay cardinality",
        ));
    }
    for ((child_ofe, authority), (parent_ofe, finalization)) in
        child_rows.into_iter().zip(parent_rows)
    {
        if child_ofe != parent_ofe
            || authority.validated_finalization().map_err(|_| {
                DirectSurfaceLiquidError::Identity("WB14 parent finalization replay")
            })? != finalization
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 child/parent replay join",
            ));
        }
    }
    Ok(())
}

/// Parent-local Green--Ampt state. This is candidate state, never a
/// persistent surface-liquid continuation: covered children carry it until
/// the child whose accepted support closes the coupled parent.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DirectWb14ParentWorkingState {
    schema: String,
    parent_day_index: usize,
    parent_interval_index: u8,
    parent_support_start_ns: i128,
    parent_support_end_ns: i128,
    surface_liquid_configuration_sha256: String,
    wb14_configuration_sha256: String,
    wb14_model_definition_sha256: String,
    production_lane_ids: Vec<u32>,
    accepted_duration_s: f64,
    parameters: Vec<DirectOfeWb14Parameters>,
    persistent_beginning_state: DirectSurfaceLiquidOwnedState,
    candidate_state: DirectSurfaceLiquidOwnedState,
    per_ofe_authorities: BTreeMap<OfeId, DirectWb14ParentIntervalV1>,
    parent_finalizations: Option<BTreeMap<OfeId, DirectWb14ParentFinalizationV1>>,
}

const WB14_PARENT_WORKING_SCHEMA: &str = "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V2";
const WB14_MODEL_DEFINITION: &[u8] =
    b"OPENWEPP_WB14_GREEN_AMPT_MODEL_DEFINITION_V1:advance_wb14_continuation_interval";

fn hex_digest(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn digest32(value: &str) -> Result<[u8; 32], DirectSurfaceLiquidError> {
    if value.len() != 64 {
        return Err(DirectSurfaceLiquidError::Identity(
            "invalid SHA-256 identity",
        ));
    }
    let mut result = [0_u8; 32];
    for (index, byte) in result.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .map_err(|_| DirectSurfaceLiquidError::Identity("invalid SHA-256 identity"))?;
    }
    Ok(result)
}

fn proposed_upper_bound_s(accepted_duration_s: f64) -> Result<f64, DirectSurfaceLiquidError> {
    [60.0_f64, 900.0, 1_800.0]
        .into_iter()
        .find(|proposal| accepted_duration_s <= *proposal)
        .ok_or(DirectSurfaceLiquidError::Domain(
            "accepted WB14 child exceeds every proposed upper bound",
        ))
}

fn ordered_receipt_set_sha256<'a>(
    domain: &[u8],
    rows: impl IntoIterator<Item = (&'a OfeId, [u8; 32])>,
) -> Result<Sha256Digest, DirectSurfaceLiquidError> {
    let mut digest = Sha256::new();
    digest.update((domain.len() as u64).to_be_bytes());
    digest.update(domain);
    for (ofe_id, receipt) in rows {
        digest.update((ofe_id.as_str().len() as u64).to_be_bytes());
        digest.update(ofe_id.as_str().as_bytes());
        digest.update(receipt);
    }
    Sha256Digest::try_new(format!("{:x}", digest.finalize()))
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 receipt-set digest"))
}

fn wb14_parent_binding(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(i128, i128, String, Vec<u32>), DirectSurfaceLiquidError> {
    let interval_ordinal = input
        .day_index
        .checked_mul(48)
        .and_then(|value| value.checked_add(usize::from(input.interval_index)))
        .ok_or(DirectSurfaceLiquidError::Identity(
            "WB14 parent support overflow",
        ))?;
    let start_ns = i128::try_from(interval_ordinal)
        .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 parent support overflow"))?
        .checked_mul(1_800_000_000_000)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "WB14 parent support overflow",
        ))?;
    let end_ns =
        start_ns
            .checked_add(1_800_000_000_000)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 parent support overflow",
            ))?;
    let parameter_bytes = serde_json::to_vec(&input.wb14_parameters)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 configuration serialization"))?;
    Ok((
        start_ns,
        end_ns,
        format!("{:x}", Sha256::digest(parameter_bytes)),
        configuration
            .ofe_bindings
            .iter()
            .map(|binding| binding.production_lane_id)
            .collect(),
    ))
}

impl DirectWb14ParentWorkingState {
    #[must_use]
    pub(crate) const fn candidate_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.candidate_state
    }

    pub(crate) fn validate_receiving_owner(
        &self,
        current: &DirectSurfaceLiquidOwnedState,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if &self.persistent_beginning_state != current {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 restart receiving surface owner",
            ));
        }
        Ok(())
    }
    pub(crate) fn restart_bytes(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate_nested(configuration)?;
        serde_json::to_vec(self)
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent restart serialization"))
    }

    pub(crate) fn from_restart_bytes(
        configuration: &DirectSurfaceLiquidConfiguration,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent restart decoding"))?;
        value.validate_nested(configuration)?;
        Ok(value)
    }

    fn validate_nested(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.schema != WB14_PARENT_WORKING_SCHEMA {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 parent restart schema",
            ));
        }
        configuration.validate()?;
        self.persistent_beginning_state.validate(configuration)?;
        self.candidate_state.validate(configuration)?;
        if self.surface_liquid_configuration_sha256 != configuration.configuration_sha256
            || self.per_ofe_authorities.len() != configuration.ofe_topology.len()
            || self.parameters.len() != configuration.ofe_topology.len()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 restart configuration join",
            ));
        }
        let support_start = u128::try_from(self.parent_support_start_ns)
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 restart support"))?;
        let support_end = u128::try_from(self.parent_support_end_ns)
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 restart support"))?;
        for (index, ofe_id) in configuration.ofe_topology.iter().enumerate() {
            let authority = self
                .per_ofe_authorities
                .get(ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity("WB14 restart OFE map"))?;
            let parameter = self
                .parameters
                .iter()
                .find(|value| &value.ofe_id == ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "WB14 restart parameter map",
                ))?;
            let continuation = self
                .persistent_beginning_state
                .continuations
                .iter()
                .find(|value| &value.ofe_id == ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "WB14 restart persistent cursor",
                ))?;
            authority
                .validate_coordinator_binding(
                    Sha256::digest(ofe_id.as_str().as_bytes()).into(),
                    self.production_lane_ids[index],
                    digest32(&self.surface_liquid_configuration_sha256)?,
                    digest32(&self.wb14_configuration_sha256)?,
                    digest32(&self.wb14_model_definition_sha256)?,
                    (
                        parameter.effective_conductivity_m_s.to_bits(),
                        parameter.matric_potential_m.to_bits(),
                        parameter.infiltration_storage_capacity_m.to_bits(),
                    ),
                    support_start,
                    support_end,
                    self.accepted_duration_s.to_bits(),
                    DirectWb14PersistentCursorV1 {
                        day_index: continuation.day_index,
                        next_interval_index: continuation.next_interval_index,
                        cumulative_supply_m: continuation.cumulative_supply_m,
                        cumulative_infiltration_m: continuation.cumulative_infiltration_m,
                    },
                )
                .map_err(|_| {
                    DirectSurfaceLiquidError::Identity(
                        "WB14 parent restart receipt/coordinator join",
                    )
                })?;
        }
        Ok(())
    }
    pub(crate) fn effective_surface_state(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidError> {
        self.candidate_state.validate(configuration)?;
        if self.candidate_state.recomputed_sha256()? != self.candidate_state.state_sha256 {
            return Err(DirectSurfaceLiquidError::Identity(
                "stale nested WB14 parent surface candidate seal",
            ));
        }
        Ok(self.candidate_state.clone())
    }

    pub(crate) fn canonical_sha256(&self) -> Result<Sha256Digest, DirectSurfaceLiquidError> {
        #[derive(Serialize)]
        struct Projection<'a> {
            schema: &'a str,
            parent_day_index: usize,
            parent_interval_index: u8,
            parent_support_start_ns: i128,
            parent_support_end_ns: i128,
            surface_liquid_configuration_sha256: &'a str,
            wb14_configuration_sha256: &'a str,
            wb14_model_definition_sha256: &'a str,
            production_lane_ids: &'a [u32],
            accepted_duration_s_bits: u64,
            parameters: &'a [DirectOfeWb14Parameters],
            persistent_beginning_state_sha256: &'a str,
            candidate_state_sha256: &'a str,
            per_ofe_authority_sha256: Vec<(&'a str, String)>,
            parent_receipt_sha256: Vec<(&'a str, String)>,
        }
        self.candidate_state
            .preflight_declared_digest()
            .map_err(|_| {
                DirectSurfaceLiquidError::Identity(
                    "stale nested WB14 parent surface candidate seal",
                )
            })?;
        self.persistent_beginning_state
            .preflight_declared_digest()
            .map_err(|_| {
                DirectSurfaceLiquidError::Identity("stale nested WB14 persistent beginning seal")
            })?;
        let per_ofe_authority_sha256 = self
            .per_ofe_authorities
            .iter()
            .map(|(ofe_id, authority)| {
                authority
                    .canonical_sha256()
                    .map(|digest| (ofe_id.as_str(), hex_digest(digest)))
                    .map_err(|_| {
                        DirectSurfaceLiquidError::Identity("invalid sealed WB14 parent authority")
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let parent_receipt_sha256 = self
            .parent_finalizations
            .as_ref()
            .map(|rows| {
                rows.iter()
                    .map(|(ofe_id, finalization)| {
                        (
                            ofe_id.as_str(),
                            hex_digest(finalization.receipt.receipt_sha256),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let bytes = serde_json::to_vec(&Projection {
            schema: &self.schema,
            parent_day_index: self.parent_day_index,
            parent_interval_index: self.parent_interval_index,
            parent_support_start_ns: self.parent_support_start_ns,
            parent_support_end_ns: self.parent_support_end_ns,
            surface_liquid_configuration_sha256: &self.surface_liquid_configuration_sha256,
            wb14_configuration_sha256: &self.wb14_configuration_sha256,
            wb14_model_definition_sha256: &self.wb14_model_definition_sha256,
            production_lane_ids: &self.production_lane_ids,
            accepted_duration_s_bits: self.accepted_duration_s.to_bits(),
            parameters: &self.parameters,
            persistent_beginning_state_sha256: &self.persistent_beginning_state.state_sha256,
            candidate_state_sha256: &self.candidate_state.state_sha256,
            per_ofe_authority_sha256,
            parent_receipt_sha256,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent-state serialization"))?;
        Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent-state digest"))
    }
}

fn begin_scalar_wb14_authorities(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
    persistent_beginning: &DirectSurfaceLiquidOwnedState,
    parent_support_start_ns: i128,
    parent_support_end_ns: i128,
    wb14_configuration_sha256: &str,
    wb14_model_definition_sha256: &str,
    coupled_binding: Option<DirectWb14CoupledChildBindingV1>,
) -> Result<BTreeMap<OfeId, DirectWb14ParentIntervalV1>, DirectSurfaceLiquidError> {
    let support_start_ns = u128::try_from(parent_support_start_ns)
        .map_err(|_| DirectSurfaceLiquidError::Identity("negative WB14 parent support"))?;
    let support_end_ns = u128::try_from(parent_support_end_ns)
        .map_err(|_| DirectSurfaceLiquidError::Identity("negative WB14 parent support"))?;
    let coupled_parent_transaction_sha256 = coupled_binding.map_or_else(
        || Sha256::digest(input.transaction_id.0.to_be_bytes()).into(),
        |binding| binding.coupled_parent_transaction_sha256,
    );
    let schema_sha256 = Sha256::digest(WB14_PARENT_WORKING_SCHEMA.as_bytes()).into();
    let surface_configuration_sha256 = digest32(&configuration.configuration_sha256)?;
    let wb14_configuration_sha256 = digest32(wb14_configuration_sha256)?;
    let wb14_model_definition_sha256 = digest32(wb14_model_definition_sha256)?;
    let parent_beginning_owner_sha256 = coupled_binding
        .map_or(digest32(&persistent_beginning.state_sha256)?, |binding| {
            binding.parent_beginning_complete_owner_set_sha256
        });
    let mut authorities = BTreeMap::new();
    for (topology_index, ofe_id) in configuration.ofe_topology.iter().enumerate() {
        let continuation = persistent_beginning
            .continuations
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity("missing WB14 cursor"))?;
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "missing WB14 lane binding",
            ))?;
        if binding.production_lane_index != topology_index {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 topology/lane order",
            ));
        }
        let parameter = input
            .wb14_parameters
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "missing WB14 parameters",
            ))?;
        let cursor = DirectWb14PersistentCursorV1 {
            day_index: continuation.day_index,
            next_interval_index: continuation.next_interval_index,
            cumulative_supply_m: continuation.cumulative_supply_m,
            cumulative_infiltration_m: continuation.cumulative_infiltration_m,
        };
        let identity = DirectWb14ImmutableIdentityV1 {
            schema_sha256,
            ofe_id_sha256: Sha256::digest(ofe_id.as_str().as_bytes()).into(),
            production_lane_id: binding.production_lane_id,
            surface_liquid_configuration_sha256: surface_configuration_sha256,
            wb14_configuration_sha256,
            wb14_model_definition_sha256,
            effective_conductivity_m_s_bits: parameter.effective_conductivity_m_s.to_bits(),
            matric_potential_m_bits: parameter.matric_potential_m.to_bits(),
            storage_capacity_m_bits: parameter.infiltration_storage_capacity_m.to_bits(),
        };
        let authority = wb14_parent_authority_v1(
            coupled_parent_transaction_sha256,
            support_start_ns,
            support_end_ns,
            parent_beginning_owner_sha256,
            cursor,
            identity,
        )
        .map_err(|_| {
            production_binding_failure(
                input.transaction_id,
                Some(ofe_id.clone()),
                "WB14 scalar parent authority",
            )
        })?;
        let parent = DirectWb14ParentIntervalV1::begin(authority, cursor).map_err(|_| {
            production_binding_failure(
                input.transaction_id,
                Some(ofe_id.clone()),
                "WB14 scalar parent begin",
            )
        })?;
        authorities.insert(ofe_id.clone(), parent);
    }
    Ok(authorities)
}
