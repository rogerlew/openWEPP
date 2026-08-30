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

fn wb14_rebind_target_is_slab_only(
    prior: DirectWb14CoupledChildBindingV1,
    target: DirectWb14CoupledChildBindingV1,
) -> bool {
    DirectWb14CoupledChildBindingV1 {
        accepted_slab_sha256: target.accepted_slab_sha256,
        ..prior
    } == target
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
        let actual = authority
            .coupled_child_binding_v1()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay validation"))?;
        if actual != binding {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay/coupled binding fields",
            ));
        }
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
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 replay child receipt",
            ))?;
        if previous_queue_after
            .is_some_and(|digest| digest != child.pending_routed_parcels_before_sha256)
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay routed-queue adjacency",
            ));
        }
        previous_queue_after = Some(child.pending_routed_parcels_after_sha256);
    }
    Ok(())
}

pub(crate) fn wb14_child_replay_binding(
    bytes: &[u8],
) -> Result<DirectWb14CoupledChildBindingV1, DirectSurfaceLiquidError> {
    let rows: Vec<(OfeId, DirectWb14ParentIntervalV1)> = serde_json::from_slice(bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay decoding"))?;
    let mut binding = None;
    for (_, authority) in rows {
        let actual = authority
            .coupled_child_binding_v1()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay validation"))?;
        if binding.is_some_and(|prior| prior != actual) {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay binding disagreement",
            ));
        }
        binding = Some(actual);
    }
    binding.ok_or(DirectSurfaceLiquidError::Identity("empty WB14 replay set"))
}

pub(crate) fn rebind_wb14_replay_to_accepted_slab(
    child_bytes: &[u8],
    parent_bytes_present: bool,
    binding: DirectWb14CoupledChildBindingV1,
) -> Result<(Vec<u8>, Option<Vec<u8>>), DirectSurfaceLiquidError> {
    let rows: Vec<(OfeId, DirectWb14ParentIntervalV1)> = serde_json::from_slice(child_bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay decoding"))?;
    if rows.is_empty() {
        return Err(DirectSurfaceLiquidError::Identity("empty WB14 replay set"));
    }
    let mut rebuilt_rows = Vec::with_capacity(rows.len());
    let mut finalizations = Vec::with_capacity(rows.len());
    for (ofe_id, authority) in rows {
        let prior_binding = authority
            .coupled_child_binding_v1()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay validation"))?;
        if !wb14_rebind_target_is_slab_only(prior_binding, binding) {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay physical authority substitution",
            ));
        }
        let rebuilt = authority
            .rebind_final_accepted_slab(binding.accepted_slab_sha256)
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay child reseal"))?;
        if parent_bytes_present {
            finalizations.push((
                ofe_id.clone(),
                rebuilt
                    .finalize()
                    .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay parent reseal"))?,
            ));
        }
        rebuilt_rows.push((ofe_id, rebuilt));
    }
    let child = serde_json::to_vec(&rebuilt_rows)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay serialization"))?;
    let parent = parent_bytes_present
        .then(|| serde_json::to_vec(&finalizations))
        .transpose()
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent replay serialization"))?;
    Ok((child, parent))
}

#[cfg(test)]
mod wb14_rebind_binding_tests {
    use super::*;

    fn binding_fixture() -> DirectWb14CoupledChildBindingV1 {
        DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 60.0_f64.to_bits(),
            coupled_parent_transaction_sha256: [1; 32],
            accepted_slab_sha256: [2; 32],
            parent_beginning_complete_owner_set_sha256: [3; 32],
            parent_support_start_ns: 10,
            parent_support_end_ns: 1_800_000_000_010,
            child_support_start_ns: 10,
            child_support_end_ns: 60_000_000_010,
        }
    }

    #[test]
    fn final_slab_rebind_rejects_every_non_slab_binding_substitution() {
        let prior = binding_fixture();
        let slab_only = DirectWb14CoupledChildBindingV1 {
            accepted_slab_sha256: [9; 32],
            ..prior
        };
        assert!(wb14_rebind_target_is_slab_only(prior, slab_only));
        let poisons = [
            DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: 61.0_f64.to_bits(),
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                coupled_parent_transaction_sha256: [4; 32],
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                parent_beginning_complete_owner_set_sha256: [5; 32],
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                parent_support_start_ns: 11,
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                parent_support_end_ns: slab_only.parent_support_end_ns + 1,
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                child_support_start_ns: 11,
                ..slab_only
            },
            DirectWb14CoupledChildBindingV1 {
                child_support_end_ns: slab_only.child_support_end_ns + 1,
                ..slab_only
            },
        ];
        for poison in poisons {
            assert!(!wb14_rebind_target_is_slab_only(prior, poison));
        }
    }
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
    accepted_until_ns: u128,
    parameters: Vec<DirectOfeWb14Parameters>,
    persistent_beginning_state: DirectSurfaceLiquidOwnedState,
    candidate_state: DirectSurfaceLiquidOwnedState,
    per_ofe_authorities: BTreeMap<OfeId, DirectWb14ParentIntervalV1>,
    parent_finalizations: Option<BTreeMap<OfeId, DirectWb14ParentFinalizationV1>>,
}

const WB14_PARENT_WORKING_SCHEMA: &str = "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V2";
const WB14_MODEL_DEFINITION: &[u8] =
    b"OPENWEPP_WB14_GREEN_AMPT_MODEL_DEFINITION_V1:advance_wb14_continuation_interval";

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
    let nanoseconds = accepted_duration_s * 1.0e9;
    if !accepted_duration_s.is_finite()
        || accepted_duration_s < 60.0
        || !nanoseconds.is_finite()
        || nanoseconds.fract() != 0.0
        || (nanoseconds as u128) % 60_000_000_000 != 0
    {
        return Err(DirectSurfaceLiquidError::Domain(
            "accepted WB14 child is outside the adaptive temporal grid",
        ));
    }
    Ok(accepted_duration_s)
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

    pub(crate) fn with_zero_duration_receiver_candidate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        candidate_state: DirectSurfaceLiquidOwnedState,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let mut candidate = self.clone();
        candidate_state.validate(configuration)?;
        if candidate_state.continuations != self.candidate_state.continuations
            || candidate_state
                .records
                .iter()
                .zip(&self.candidate_state.records)
                .any(|(ending, beginning)| {
                    ending.key != beginning.key
                        || ending.last_accepted_transaction_id
                            != beginning.last_accepted_transaction_id
                })
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration receiver changed WB14 parent-local lineage",
            ));
        }
        candidate.candidate_state = candidate_state;
        candidate.validate_nested(configuration)?;
        Ok(candidate)
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
            || self.production_lane_ids.len() != configuration.ofe_topology.len()
            || self.parent_finalizations.is_some()
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
                .get(index)
                .filter(|value| &value.ofe_id == ofe_id)
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
                    self.accepted_until_ns,
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

#[cfg(test)]
mod cadence_tests {
    use super::*;

    #[test]
    fn exact_sixty_second_proposal_grid_rejects_one_tick_below_and_admits_larger_support() {
        let below = 59_999_999_999_f64 / 1_000_000_000.0;
        assert!(matches!(
            proposed_upper_bound_s(below),
            Err(DirectSurfaceLiquidError::Domain(
                "accepted WB14 child is outside the adaptive temporal grid"
            ))
        ));
        assert_eq!(
            proposed_upper_bound_s(60.0)
                .expect("exact floor")
                .to_bits(),
            60.0_f64.to_bits()
        );
        assert_eq!(
            proposed_upper_bound_s(120.0)
                .expect("ordinary larger proposal")
                .to_bits(),
            120.0_f64.to_bits()
        );
    }
}
