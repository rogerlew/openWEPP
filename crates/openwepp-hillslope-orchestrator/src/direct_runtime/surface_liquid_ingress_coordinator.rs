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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3CoveredNativeInactiveChildCustodyV1 {
    schema: String,
    proposed_upper_bound_s_bits: u64,
    coupled_parent_transaction_sha256: [u8; 32],
    accepted_slab_sha256: [u8; 32],
    parent_beginning_complete_owner_set_sha256: [u8; 32],
    parent_support_start_ns: u128,
    parent_support_end_ns: u128,
    child_support_start_ns: u128,
    child_support_end_ns: u128,
    ofe_topology: Vec<OfeId>,
}

impl DirectSurfaceLiquidIngressCandidate {
    #[cfg(test)]
    pub(crate) fn canonical_private_projection_v1(
        &self,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        #[derive(serde::Serialize)]
        struct Wire<'a> {
            schema: &'static str,
            transaction_id: TransactionId,
            beginning_state: &'a DirectSurfaceLiquidOwnedState,
            ending_state: &'a DirectSurfaceLiquidOwnedState,
            receipts: &'a [DirectSurfaceLiquidParcelReceipt],
            ledgers: &'a [DirectSurfaceLiquidIngressLedger],
            wb14_calls_by_ofe: &'a BTreeMap<OfeId, u8>,
            closure_operands: Vec<u8>,
            open_ingress_parcels: &'a [DirectOpenLiquidIngressParcel],
            parent_child_mode: bool,
            finalize_parent_interval: bool,
            input_parent_working_state: &'a Option<DirectWb14ParentWorkingState>,
            parent_working_state: &'a Option<DirectWb14ParentWorkingState>,
            wb14_child_receipt_set_sha256: &'a Sha256Digest,
            wb14_parent_receipt_set_sha256: &'a Option<Sha256Digest>,
            wb14_child_replay_bytes: &'a [u8],
            wb14_parent_replay_bytes: &'a Option<Vec<u8>>,
            stage3_covered_native_inactive: bool,
        }

        serde_json::to_vec(&Wire {
            schema: "OPENWEPP_SURFACE_LIQUID_INGRESS_PRIVATE_PROJECTION_V1",
            transaction_id: self.transaction_id,
            beginning_state: &self.beginning_state,
            ending_state: &self.ending_state,
            receipts: &self.receipts,
            ledgers: &self.ledgers,
            wb14_calls_by_ofe: &self.wb14_calls_by_ofe,
            closure_operands: self.closure_operands.canonical_private_projection_v1()?,
            open_ingress_parcels: &self.open_ingress_parcels,
            parent_child_mode: self.parent_child_mode,
            finalize_parent_interval: self.finalize_parent_interval,
            input_parent_working_state: &self.input_parent_working_state,
            parent_working_state: &self.parent_working_state,
            wb14_child_receipt_set_sha256: &self.wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256: &self.wb14_parent_receipt_set_sha256,
            wb14_child_replay_bytes: &self.wb14_child_replay_bytes,
            wb14_parent_replay_bytes: &self.wb14_parent_replay_bytes,
            stage3_covered_native_inactive: self.stage3_covered_native_inactive,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("ingress private projection"))
    }

    fn validate_stage3_covered_native_inactive(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        resource: &DirectSurfaceLiquidResourceCandidate,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        preflight_surface_liquid_ingress_public_identities(configuration, resource, input)?;
        resource.validate(configuration)?;
        self.validate_stage3_covered_native_inactive_structure(resource, input)
    }

    fn validate_stage3_covered_native_inactive_with_validated_resource(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        validated_resource: &ValidatedStage3CoveredNativeInactiveResourceV1,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let resource = validated_resource.resource();
        preflight_surface_liquid_ingress_public_identities(configuration, resource, input)?;
        self.validate_stage3_covered_native_inactive_structure(resource, input)
    }

    fn validate_stage3_covered_native_inactive_structure(
        &self,
        resource: &DirectSurfaceLiquidResourceCandidate,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.transaction_id != input.transaction_id
            || self.transaction_id != resource.transaction_id()
            || self.beginning_state != *resource.beginning_state()
            || self.ending_state != *resource.working_state()
            || self.beginning_state != self.ending_state
            || !self.receipts.is_empty()
            || !self.ledgers.is_empty()
            || !self.wb14_calls_by_ofe.is_empty()
            || !self.open_ingress_parcels.is_empty()
            || self.parent_working_state != self.input_parent_working_state
            || self.wb14_parent_receipt_set_sha256.is_some()
            || self.wb14_parent_replay_bytes.is_some()
            || self.closure_operands.transaction_id() != self.transaction_id
        {
            return Err(DirectSurfaceLiquidError::Closure(
                "Stage3CoveredNative inactive ingress custody",
            ));
        }
        Ok(())
    }

    /// Construct a typed inactive-surface continuation for represented snow.
    /// This validates identity and custody without entering surface or WB14 physics.
    pub(crate) fn try_new_stage3_covered_native_inactive(
        configuration: &DirectSurfaceLiquidConfiguration,
        validated_resource: &ValidatedStage3CoveredNativeInactiveResourceV1,
        input: &DirectSurfaceLiquidIngressInput,
        parent_working_state: Option<&DirectWb14ParentWorkingState>,
        coupled_binding: DirectWb14CoupledChildBindingV1,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let resource = validated_resource.resource();
        preflight_surface_liquid_ingress_public_identities(configuration, resource, input)?;
        let has_nonzero_ingress = input.tile_ingress.iter().any(|row| match row {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => raw_precipitation.mass_kg_m2_tile_ground.to_bits() != 0.0_f64.to_bits(),
            DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => parcels.iter().any(
                |parcel| parcel.amount.mass_kg_m2_tile_ground.to_bits() != 0.0_f64.to_bits(),
            ),
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. }
            | DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { release, .. } => {
                [
                    &release.throughfall,
                    &release.initial_drainage,
                    &release.second_drainage,
                    &release.stemflow,
                ]
                .into_iter()
                .any(|amount| amount.mass_kg_m2_tile_ground.to_bits() != 0.0_f64.to_bits())
                    || matches!(
                        row,
                        DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
                            runon_parcels,
                            ..
                        } if runon_parcels.iter().any(|parcel| parcel.amount.mass_kg_m2_tile_ground.to_bits() != 0.0_f64.to_bits())
                    )
            }
        });
        if has_nonzero_ingress {
            return Err(DirectSurfaceLiquidError::Closure(
                "Stage3CoveredNative inactive surface received liquid ingress",
            ));
        }
        let closure_operands =
            DirectSurfaceLiquidClosureOperands::try_new_stage3_covered_native_inactive(
                input.transaction_id,
                configuration,
                resource,
            )?;
        let wb14_child_replay_bytes = stage3_covered_native_inactive_child_custody_bytes(
            coupled_binding,
            &configuration.ofe_topology,
        )?;
        let wb14_child_receipt_set_sha256 =
            Sha256Digest::try_new(format!("{:x}", Sha256::digest(&wb14_child_replay_bytes)))
                .map_err(|_| DirectSurfaceLiquidError::Schema("inactive WB14 custody digest"))?;
        let candidate = Self {
            transaction_id: input.transaction_id,
            beginning_state: resource.beginning_state().clone(),
            ending_state: resource.working_state().clone(),
            receipts: Vec::new(),
            ledgers: Vec::new(),
            wb14_calls_by_ofe: BTreeMap::new(),
            closure_operands,
            open_ingress_parcels: Vec::new(),
            parent_child_mode: true,
            finalize_parent_interval: false,
            input_parent_working_state: parent_working_state.cloned(),
            parent_working_state: parent_working_state.cloned(),
            wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256: None,
            wb14_child_replay_bytes,
            wb14_parent_replay_bytes: None,
            stage3_covered_native_inactive: true,
        };
        candidate.validate_stage3_covered_native_inactive_with_validated_resource(
            configuration,
            validated_resource,
            input,
        )?;
        Ok(candidate)
    }
}

pub(crate) fn stage3_covered_native_inactive_child_custody_bytes(
    binding: DirectWb14CoupledChildBindingV1,
    ofe_topology: &[OfeId],
) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
    if ofe_topology.is_empty()
        || binding.child_support_start_ns >= binding.child_support_end_ns
        || binding.child_support_start_ns < binding.parent_support_start_ns
        || binding.child_support_end_ns > binding.parent_support_end_ns
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "Stage3CoveredNative inactive child custody bounds",
        ));
    }
    serde_json::to_vec(&Stage3CoveredNativeInactiveChildCustodyV1 {
        schema: "openwepp.stage3-covered-native-inactive-child-custody.v1".to_owned(),
        proposed_upper_bound_s_bits: binding.proposed_upper_bound_s_bits,
        coupled_parent_transaction_sha256: binding.coupled_parent_transaction_sha256,
        accepted_slab_sha256: binding.accepted_slab_sha256,
        parent_beginning_complete_owner_set_sha256: binding
            .parent_beginning_complete_owner_set_sha256,
        parent_support_start_ns: binding.parent_support_start_ns,
        parent_support_end_ns: binding.parent_support_end_ns,
        child_support_start_ns: binding.child_support_start_ns,
        child_support_end_ns: binding.child_support_end_ns,
        ofe_topology: ofe_topology.to_vec(),
    })
    .map_err(|_| DirectSurfaceLiquidError::Schema("inactive child custody serialization"))
}

pub(crate) fn stage3_covered_native_inactive_child_custody_binding(
    bytes: &[u8],
    expected_ofe_topology: &[OfeId],
) -> Result<Option<DirectWb14CoupledChildBindingV1>, DirectSurfaceLiquidError> {
    if bytes.first() != Some(&b'{') {
        return Ok(None);
    }
    let wire: Stage3CoveredNativeInactiveChildCustodyV1 = serde_json::from_slice(bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("inactive child custody decoding"))?;
    let canonical = serde_json::to_vec(&wire)
        .map_err(|_| DirectSurfaceLiquidError::Schema("inactive child custody serialization"))?;
    if canonical != bytes
        || wire.schema != "openwepp.stage3-covered-native-inactive-child-custody.v1"
        || wire.ofe_topology != expected_ofe_topology
        || wire.child_support_start_ns >= wire.child_support_end_ns
        || wire.child_support_start_ns < wire.parent_support_start_ns
        || wire.child_support_end_ns > wire.parent_support_end_ns
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "Stage3CoveredNative inactive child custody identity",
        ));
    }
    Ok(Some(DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: wire.proposed_upper_bound_s_bits,
        coupled_parent_transaction_sha256: wire.coupled_parent_transaction_sha256,
        accepted_slab_sha256: wire.accepted_slab_sha256,
        parent_beginning_complete_owner_set_sha256: wire.parent_beginning_complete_owner_set_sha256,
        parent_support_start_ns: wire.parent_support_start_ns,
        parent_support_end_ns: wire.parent_support_end_ns,
        child_support_start_ns: wire.child_support_start_ns,
        child_support_end_ns: wire.child_support_end_ns,
    }))
}

pub(crate) fn rebind_stage3_covered_native_inactive_child_custody(
    bytes: &[u8],
    target: DirectWb14CoupledChildBindingV1,
    expected_ofe_topology: &[OfeId],
) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
    let prior = stage3_covered_native_inactive_child_custody_binding(bytes, expected_ofe_topology)?
        .ok_or(DirectSurfaceLiquidError::Identity(
            "Stage3CoveredNative inactive child custody posture",
        ))?;
    if !wb14_rebind_target_is_slab_only(prior, target) {
        return Err(DirectSurfaceLiquidError::Identity(
            "Stage3CoveredNative inactive child custody rebind",
        ));
    }
    stage3_covered_native_inactive_child_custody_bytes(target, expected_ofe_topology)
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
    expected_ofe_topology: &[OfeId],
) -> Result<(), DirectSurfaceLiquidError> {
    let rows: Vec<(OfeId, DirectWb14ParentIntervalV1)> = serde_json::from_slice(bytes)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay decoding"))?;
    if rows.is_empty() || rows.len() != expected_ofe_topology.len() {
        return Err(DirectSurfaceLiquidError::Identity(
            "WB14 replay topology cardinality",
        ));
    }
    let mut previous_queue_after = None;
    for ((ofe_id, authority), expected_ofe_id) in rows.into_iter().zip(expected_ofe_topology) {
        if &ofe_id != expected_ofe_id {
            return Err(DirectSurfaceLiquidError::Identity("WB14 replay OFE order"));
        }
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
    super::surface_liquid_wb14::note_wb14_reseal_input_validation();
    for (_, authority) in &rows {
        authority
            .validate()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay validation"))?;
        let prior_binding = authority
            .coupled_child_binding_from_validated_reseal_source()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay validation"))?;
        if !wb14_rebind_target_is_slab_only(prior_binding, binding) {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 replay physical authority substitution",
            ));
        }
    }
    let mut rebuilt_rows = Vec::with_capacity(rows.len());
    for (ofe_id, authority) in rows {
        let rebuilt = authority
            .rebind_final_accepted_slab_from_validated_reseal_source(binding.accepted_slab_sha256)
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay child reseal"))?;
        rebuilt_rows.push((ofe_id, rebuilt));
    }
    super::surface_liquid_wb14::note_wb14_reseal_final_validation();
    for (_, authority) in &rebuilt_rows {
        authority
            .validate()
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 replay child reseal"))?;
    }
    let finalizations = parent_bytes_present
        .then(|| {
            rebuilt_rows
                .iter()
                .map(|(ofe_id, authority)| {
                    authority
                        .finalize_from_validated_reseal_source()
                        .map(|finalization| (ofe_id.clone(), finalization))
                        .map_err(|_| {
                            DirectSurfaceLiquidError::Identity("WB14 replay parent reseal")
                        })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;
    let child = serde_json::to_vec(&rebuilt_rows)
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay serialization"))?;
    let parent = finalizations
        .as_ref()
        .map(serde_json::to_vec)
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
    pub(crate) const fn persistent_beginning_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.persistent_beginning_state
    }

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

    /// Validate a trusted in-process parent without materializing durable
    /// restart bytes. External and restart consumers retain `restart_bytes`
    /// and `from_restart_bytes` as their canonical serialization boundary.
    pub(crate) fn validate_in_process(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.validate_nested(configuration)
    }

    /// Reseal only the accepted-slab authorization on the final child of an
    /// already validated parent. All support, owner, queue, transition, and
    /// physical arithmetic remains byte-identical.
    pub(crate) fn rebind_final_accepted_slab(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        target: DirectWb14CoupledChildBindingV1,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        super::surface_liquid_wb14::note_wb14_reseal_input_validation();
        self.validate_nested(configuration)?;
        let mut rebuilt = self.clone();
        let authorities = std::mem::take(&mut rebuilt.per_ofe_authorities);
        for (ofe_id, authority) in authorities {
            let observed = authority
                .coupled_child_binding_from_validated_reseal_source()
                .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 parent rebind source"))?;
            if !wb14_rebind_target_is_slab_only(observed, target) {
                return Err(DirectSurfaceLiquidError::Identity(
                    "WB14 parent rebind physical authority substitution",
                ));
            }
            let rebound = authority
                .rebind_final_accepted_slab_from_validated_reseal_source(
                    target.accepted_slab_sha256,
                )
                .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 parent child reseal"))?;
            rebuilt.per_ofe_authorities.insert(ofe_id, rebound);
        }
        super::surface_liquid_wb14::note_wb14_reseal_final_validation();
        rebuilt.validate_nested(configuration)?;
        Ok(rebuilt)
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
    transaction_id: TransactionId,
    wb14_parameters: &[DirectOfeWb14Parameters],
    persistent_beginning: &DirectSurfaceLiquidOwnedState,
    parent_support_start_ns: i128,
    parent_support_end_ns: i128,
    wb14_configuration_sha256: &str,
    wb14_model_definition_sha256: &str,
    coupled_binding: Option<DirectWb14CoupledChildBindingV1>,
    inactive_prefix: Option<super::surface_liquid_wb14::ValidatedNativeInactiveWb14PrefixV1>,
) -> Result<BTreeMap<OfeId, DirectWb14ParentIntervalV1>, DirectSurfaceLiquidError> {
    let support_start_ns = u128::try_from(parent_support_start_ns)
        .map_err(|_| DirectSurfaceLiquidError::Identity("negative WB14 parent support"))?;
    let support_end_ns = u128::try_from(parent_support_end_ns)
        .map_err(|_| DirectSurfaceLiquidError::Identity("negative WB14 parent support"))?;
    let coupled_parent_transaction_sha256 = coupled_binding.map_or_else(
        || Sha256::digest(transaction_id.0.to_be_bytes()).into(),
        |binding| binding.coupled_parent_transaction_sha256,
    );
    let schema_sha256 = Sha256::digest(WB14_PARENT_WORKING_SCHEMA.as_bytes()).into();
    let surface_configuration_sha256 = digest32(&configuration.configuration_sha256)?;
    let wb14_configuration_sha256 = digest32(wb14_configuration_sha256)?;
    let wb14_model_definition_sha256 = digest32(wb14_model_definition_sha256)?;
    let ordinary_parent_beginning = coupled_binding.map_or_else(
        || digest32(&persistent_beginning.state_sha256),
        |binding| Ok(binding.parent_beginning_complete_owner_set_sha256),
    )?;
    let parent_beginning_owner_sha256 = inactive_prefix.map_or(
        ordinary_parent_beginning,
        super::surface_liquid_wb14::ValidatedNativeInactiveWb14PrefixV1::parent_beginning_owner_sha256,
    );
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
        let parameter = wb14_parameters
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
        .map_err(|error| {
            wb14_parent_interval_failure(
                transaction_id,
                Some(configuration.owner_id.clone()),
                Some(ofe_id.clone()),
                &error,
                format!("WB14 scalar parent authority: {error}"),
            )
        })?;
        let parent = inactive_prefix
            .map_or_else(
                || DirectWb14ParentIntervalV1::begin(authority, cursor),
                |prefix| {
                    DirectWb14ParentIntervalV1::begin_after_native_inactive_prefix(
                        authority, cursor, prefix,
                    )
                },
            )
            .map_err(|error| {
                wb14_parent_interval_failure(
                    transaction_id,
                    Some(configuration.owner_id.clone()),
                    Some(ofe_id.clone()),
                    &error,
                    format!("WB14 scalar parent begin: {error}"),
                )
            })?;
        authorities.insert(ofe_id.clone(), parent);
    }
    Ok(authorities)
}

impl DirectWb14ParentWorkingState {
    pub(crate) fn begin_after_native_inactive_prefix(
        configuration: &DirectSurfaceLiquidConfiguration,
        transaction_id: TransactionId,
        day_index: usize,
        interval_index: usize,
        wb14_parameters: &[DirectOfeWb14Parameters],
        persistent_beginning: &DirectSurfaceLiquidOwnedState,
        coupled_binding: DirectWb14CoupledChildBindingV1,
        prefix: super::surface_liquid_wb14::ValidatedNativeInactiveWb14PrefixV1,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        configuration.validate()?;
        persistent_beginning.validate(configuration)?;
        prefix
            .validate_successor_binding(coupled_binding)
            .map_err(|error| {
                wb14_parent_interval_failure(
                    transaction_id,
                    Some(configuration.owner_id.clone()),
                    None,
                    &error,
                    format!("native inactive-prefix successor: {error}"),
                )
            })?;
        let interval_ordinal = day_index
            .checked_mul(48)
            .and_then(|value| value.checked_add(interval_index))
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 parent support overflow",
            ))?;
        let parent_support_start_ns = i128::try_from(interval_ordinal)
            .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 parent support overflow"))?
            .checked_mul(1_800_000_000_000)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 parent support overflow",
            ))?;
        let parent_support_end_ns = parent_support_start_ns
            .checked_add(1_800_000_000_000)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 parent support overflow",
            ))?;
        if u128::try_from(parent_support_start_ns).ok()
            != Some(coupled_binding.parent_support_start_ns)
            || u128::try_from(parent_support_end_ns).ok()
                != Some(coupled_binding.parent_support_end_ns)
        {
            return Err(production_binding_failure(
                transaction_id,
                None,
                "native inactive-prefix WB14 parent support",
            ));
        }
        let parameter_bytes = serde_json::to_vec(wb14_parameters)
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 configuration serialization"))?;
        let wb14_configuration_sha256 = format!("{:x}", Sha256::digest(parameter_bytes));
        let wb14_model_definition_sha256 = format!("{:x}", Sha256::digest(WB14_MODEL_DEFINITION));
        let per_ofe_authorities = begin_scalar_wb14_authorities(
            configuration,
            transaction_id,
            wb14_parameters,
            persistent_beginning,
            parent_support_start_ns,
            parent_support_end_ns,
            &wb14_configuration_sha256,
            &wb14_model_definition_sha256,
            Some(coupled_binding),
            Some(prefix),
        )?;
        let value = Self {
            schema: WB14_PARENT_WORKING_SCHEMA.to_owned(),
            parent_day_index: day_index,
            parent_interval_index: u8::try_from(interval_index)
                .map_err(|_| DirectSurfaceLiquidError::Identity("WB14 interval index overflow"))?,
            parent_support_start_ns,
            parent_support_end_ns,
            surface_liquid_configuration_sha256: configuration.configuration_sha256.clone(),
            wb14_configuration_sha256,
            wb14_model_definition_sha256,
            production_lane_ids: configuration
                .ofe_bindings
                .iter()
                .map(|binding| binding.production_lane_id)
                .collect(),
            accepted_until_ns: coupled_binding.child_support_start_ns,
            parameters: wb14_parameters.to_vec(),
            persistent_beginning_state: persistent_beginning.clone(),
            candidate_state: persistent_beginning.clone(),
            per_ofe_authorities,
            parent_finalizations: None,
        };
        value.validate_nested(configuration)?;
        Ok(value)
    }
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
            proposed_upper_bound_s(60.0).expect("exact floor").to_bits(),
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
fn wb14_parent_interval_failure(
    transaction_id: TransactionId,
    owner_id: Option<ResourceOwnerId>,
    ofe_id: Option<OfeId>,
    error: &super::surface_liquid_wb14::DirectWb14ParentIntervalErrorV1,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        error.canonical_surface_liquid_error_code(),
        DirectSurfaceLiquidPhase::Restart,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id,
            ofe_id,
            ..DirectSurfaceLiquidErrorContext::default()
        },
        super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}
