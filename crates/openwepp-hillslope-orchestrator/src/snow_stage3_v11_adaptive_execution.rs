include!("snow_stage3_v11_adaptive_frontend.rs");

pub(crate) fn adaptive_comparison_audit_includes_discrete_surface_kind_v1(
    kind: crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1,
) -> bool {
    kind != crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1::ReceiptLineage
}

fn adaptive_complete_owner_error_v1(
    support: TimeSupport,
    direct_consumer: &DirectV10RealConsumerShadow,
    direct_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    composed_consumer: &DirectV10RealConsumerShadow,
    composed_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    direct_pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    composed_pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<
    (
        crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
        crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
        f64,
        bool,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let direct = direct_consumer
        .adaptive_complete_owner_comparison_v1(canonical_stage3_snow_owner_bytes(direct_stage3)?)?;
    let composed = composed_consumer.adaptive_complete_owner_comparison_v1(
        canonical_stage3_snow_owner_bytes(composed_stage3)?,
    )?;
    let (mut maximum_scaled_error, mut discrete_mismatch) = direct.scaled_error(&composed)?;
    let mut comparison_audit = adaptive_comparison_test_audit_enabled_v1().then(|| {
        let maximum = direct
            .scalars
            .iter()
            .zip(&composed.scalars)
            .filter_map(|(left, right)| {
                let denominator = left.absolute_tolerance
                    + left.relative_tolerance * left.value.abs().max(right.value.abs());
                (denominator.is_finite() && denominator > 0.0).then_some((
                    left.owner_id.clone(),
                    left.path.clone(),
                    left.tolerance_authority.clone(),
                    (right.value - left.value).abs() / denominator,
                    left.value,
                    right.value,
                    denominator,
                ))
            })
            .max_by(|left, right| left.3.total_cmp(&right.3));
        let direct_cross_path_surfaces = direct
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| {
                adaptive_comparison_audit_includes_discrete_surface_kind_v1(surface.kind)
            })
            .collect::<Vec<_>>();
        let composed_cross_path_surfaces = composed
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| {
                adaptive_comparison_audit_includes_discrete_surface_kind_v1(surface.kind)
            })
            .collect::<Vec<_>>();
        let first_discrete_surface_pair = direct_cross_path_surfaces
            .iter()
            .zip(&composed_cross_path_surfaces)
            .find(|(left, right)| left != right);
        let first_discrete_surface_kind =
            first_discrete_surface_pair.map(|(left, _)| format!("{:?}", left.kind));
        let first_discrete_surface_delta = first_discrete_surface_pair
            .map(|(left, right)| {
                (
                    left.owner_id.clone(),
                    left.path.clone(),
                    left.exact_value.clone(),
                    right.exact_value.clone(),
                )
            })
            .or_else(|| {
                (direct_cross_path_surfaces.len() != composed_cross_path_surfaces.len()).then(
                    || {
                        (
                            "complete_owner".to_owned(),
                            "exact_discrete_surfaces.len".to_owned(),
                            direct_cross_path_surfaces.len().to_string(),
                            composed_cross_path_surfaces.len().to_string(),
                        )
                    },
                )
            });
        AdaptiveComparisonAuditV1 {
            support,
            maximum_owner_id: maximum.as_ref().map(|value| value.0.clone()),
            maximum_path: maximum.as_ref().map(|value| value.1.clone()),
            maximum_tolerance_authority: maximum.as_ref().map(|value| {
                format!(
                    "{}:{}:{:?}",
                    value.2.contract_id, value.2.tolerance_id, value.2.dimension
                )
            }),
            maximum_direct_value: maximum.as_ref().map(|value| value.4),
            maximum_composed_value: maximum.as_ref().map(|value| value.5),
            maximum_tolerance_denominator: maximum.as_ref().map(|value| value.6),
            maximum_scaled_error,
            first_discrete_surface_kind,
            first_discrete_surface_delta,
        }
    });

    // Trial-local parcel digests and predecessor receipts differ between a
    // direct transaction and its two-child composition. Compare the parcel
    // owner's prognostic payload and exact posture instead of attempting to
    // parse the framed V4 publication envelope as the snow-state JSON DTO.
    let mut direct_parcels = direct_pending_terminal_parcels.values().collect::<Vec<_>>();
    let mut composed_parcels = composed_pending_terminal_parcels
        .values()
        .collect::<Vec<_>>();
    let parcel_order = |left: &&DirectSnowStage3V11TerminalParcel,
                        right: &&DirectSnowStage3V11TerminalParcel| {
        (
            left.source_lane_id,
            left.destination_ofe_id.as_str(),
            left.event_ordinal,
        )
            .cmp(&(
                right.source_lane_id,
                right.destination_ofe_id.as_str(),
                right.event_ordinal,
            ))
    };
    direct_parcels.sort_by(parcel_order);
    composed_parcels.sort_by(parcel_order);
    if direct_parcels.len() != composed_parcels.len() {
        if let Some(comparison_audit) = comparison_audit.as_mut() {
            comparison_audit.maximum_owner_id = Some("snow".to_owned());
            comparison_audit.maximum_path = Some("pending_terminal_parcels.len".to_owned());
            comparison_audit.maximum_tolerance_authority = None;
            comparison_audit.maximum_direct_value = None;
            comparison_audit.maximum_composed_value = None;
            comparison_audit.maximum_tolerance_denominator = None;
            comparison_audit.maximum_scaled_error = f64::INFINITY;
            audit_adaptive_comparison_v1(comparison_audit.clone());
        }
        // Parcel-set cardinality is an exact discrete mismatch. Preserve the
        // finite physical scaled error already reconstructed above; the
        // discrete flag alone authorizes rejection, while comparison receipts
        // intentionally prohibit non-finite diagnostic values.
        return Ok((direct, composed, maximum_scaled_error, true));
    }
    for (left, right) in direct_parcels.iter().zip(composed_parcels) {
        discrete_mismatch |= left.source_lane_id != right.source_lane_id
            || left.destination_ofe_id != right.destination_ofe_id
            || left.receiver_destinations != right.receiver_destinations
            || left.event_ordinal != right.event_ordinal
            || left.posture != right.posture;
        let left_expected_specific_enthalpy =
            openwepp_land_surface_energy::liquid_enthalpy_j_kg(left.temperature_k);
        let right_expected_specific_enthalpy =
            openwepp_land_surface_energy::liquid_enthalpy_j_kg(right.temperature_k);
        if left.specific_liquid_enthalpy_j_kg.to_bits() != left_expected_specific_enthalpy.to_bits()
            || right.specific_liquid_enthalpy_j_kg.to_bits()
                != right_expected_specific_enthalpy.to_bits()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive terminal-parcel liquid thermodynamic identity",
            ));
        }
        let mass_policy = crate::v9_real_consumer_shadow::adaptive_scalar_policy(
            "snow",
            "pending_terminal_parcels[].mass_kg_m2_tile_ground",
        )
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive terminal-parcel mass tolerance authority",
        ))?;
        let temperature_policy = crate::v9_real_consumer_shadow::adaptive_scalar_policy(
            "snow",
            "pending_terminal_parcels[].temperature_k",
        )
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive terminal-parcel temperature tolerance authority",
        ))?;
        for (
            _field,
            left_value,
            right_value,
            _tolerance_authority,
            absolute_tolerance,
            relative_tolerance,
        ) in [
            (
                "mass_kg_m2_tile_ground",
                left.mass_kg_m2_tile_ground,
                right.mass_kg_m2_tile_ground,
                mass_policy.0,
                mass_policy.1,
                mass_policy.2,
            ),
            (
                "temperature_k",
                left.temperature_k,
                right.temperature_k,
                temperature_policy.0,
                temperature_policy.1,
                temperature_policy.2,
            ),
        ] {
            let denominator =
                absolute_tolerance + relative_tolerance * left_value.abs().max(right_value.abs());
            let scaled = (right_value - left_value).abs() / denominator;
            if !scaled.is_finite() {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive terminal-parcel comparison domain",
                ));
            }
            maximum_scaled_error = maximum_scaled_error.max(scaled);
            if let Some(comparison_audit) = comparison_audit
                .as_mut()
                .filter(|audit| scaled > audit.maximum_scaled_error)
            {
                comparison_audit.maximum_owner_id = Some("snow".to_owned());
                comparison_audit.maximum_path = Some(format!(
                    "pending_terminal_parcels[{}].{_field}",
                    left.source_lane_id
                ));
                comparison_audit.maximum_tolerance_authority = Some(format!(
                    "{}:{}:{:?}",
                    _tolerance_authority.contract_id,
                    _tolerance_authority.tolerance_id,
                    _tolerance_authority.dimension
                ));
                comparison_audit.maximum_direct_value = Some(left_value);
                comparison_audit.maximum_composed_value = Some(right_value);
                comparison_audit.maximum_tolerance_denominator = Some(denominator);
                comparison_audit.maximum_scaled_error = scaled;
            }
        }
    }
    if let Some(mut comparison_audit) = comparison_audit {
        comparison_audit.maximum_scaled_error = maximum_scaled_error;
        audit_adaptive_comparison_v1(comparison_audit);
    }
    Ok((direct, composed, maximum_scaled_error, discrete_mismatch))
}

fn adaptive_discrete_surface_receipts_v1(
    comparison: &crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<
    (Digest32, Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>),
    DirectSnowStage3V11AttachmentError,
> {
    use crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1;
    let mut surfaces = comparison
        .exact_discrete_surfaces
        .iter()
        .map(|surface| Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: surface.owner_id.clone(),
            path: surface.path.clone(),
            kind: match surface.kind {
                AdaptiveDiscreteSurfaceKindV1::ActiveSet => "active_set",
                AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache => "derived_integrity_cache",
                AdaptiveDiscreteSurfaceKindV1::ExactBinary64 => "exact_binary64",
                AdaptiveDiscreteSurfaceKindV1::Identity => "identity",
                AdaptiveDiscreteSurfaceKindV1::Membership => "membership",
                AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart => "numerical_warm_start",
                AdaptiveDiscreteSurfaceKindV1::Ordering => "ordering",
                AdaptiveDiscreteSurfaceKindV1::Posture => "posture",
                AdaptiveDiscreteSurfaceKindV1::ReceiptLineage => "receipt_lineage",
                AdaptiveDiscreteSurfaceKindV1::ReceiptOrdering => "receipt_ordering",
                AdaptiveDiscreteSurfaceKindV1::Schema => "schema",
                AdaptiveDiscreteSurfaceKindV1::Topology => "topology",
            }
            .to_owned(),
            exact_value: surface.exact_value.clone(),
        })
        .collect::<Vec<_>>();
    surfaces.extend(adaptive_scalar_and_pending_identity_surfaces_v1(
        comparison,
        pending_parcels,
    )?);
    for (ordinal, parcel) in pending_parcels.values().enumerate() {
        surfaces.push(Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: format!("pending_terminal_parcels[{ordinal}].posture"),
            kind: "posture".to_owned(),
            exact_value: match parcel.posture {
                DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed => {
                    "produced_unconsumed"
                }
                DirectSnowStage3V11TerminalParcelPosture::Consumed => "consumed",
            }
            .to_owned(),
        });
    }
    surfaces.sort_by(|left, right| {
        (
            left.owner_id.as_str(),
            left.path.as_str(),
            left.kind.as_str(),
            left.exact_value.as_str(),
        )
            .cmp(&(
                right.owner_id.as_str(),
                right.path.as_str(),
                right.kind.as_str(),
                right.exact_value.as_str(),
            ))
    });
    if surfaces.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive exact-discrete duplicate surface",
        ));
    }
    let parcel_posture_sha256 = if pending_parcels.is_empty() {
        digest_bytes(b"OPENWEPP_ADAPTIVE_EMPTY_PENDING_PARCEL_POSTURE_V1")
    } else {
        let fields = pending_parcels
            .values()
            .map(|parcel| FramedField {
                tag: "parcel",
                value: parcel.parcel_digest.as_bytes(),
            })
            .collect::<Vec<_>>();
        framed_sha256("stage3-adaptive-pending-parcel-posture-v1", &fields)?
    };
    let digest = framed_sha256(
        "stage3-adaptive-complete-exact-discrete-v1",
        &[
            FramedField {
                tag: "canonical_owner_exact_discrete_sha256",
                value: comparison.exact_discrete_sha256.as_bytes(),
            },
            FramedField {
                tag: "pending_parcel_posture_sha256",
                value: parcel_posture_sha256.as_bytes(),
            },
        ],
    )?;
    Ok((digest, surfaces))
}

fn adaptive_trial_ledger_set_sha256_v1(
    receipts: &[Stage3CoupledSubslabReceiptV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if receipts.is_empty() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive trial physical-ledger cardinality",
        ));
    }
    let fields = receipts
        .iter()
        .map(|receipt| FramedField {
            tag: "physical_ledger",
            value: receipt.physical_outcome_ledger_set_sha256.as_bytes(),
        })
        .collect::<Vec<_>>();
    Ok(framed_sha256(
        "stage3-adaptive-trial-physical-ledger-set-v1",
        &fields,
    )?)
}

fn adaptive_trial_phase_result_sha256_v1(
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    Ok(framed_sha256(
        "stage3-adaptive-phase-result-v1",
        &[FramedField {
            tag: "snow_owner_bytes_sha256",
            value: digest_bytes(&canonical_stage3_snow_owner_bytes(stage3)?).as_bytes(),
        }],
    )?)
}

fn adaptive_child_phase_result_sha256_v1(
    receipts: &[Stage3CoupledSubslabReceiptV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let ending = receipts
        .last()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive child phase receipt cardinality",
        ))?
        .effective_ending_complete_owner_set_sha256();
    let ledger = adaptive_trial_ledger_set_sha256_v1(receipts)?;
    Ok(framed_sha256(
        "stage3-adaptive-child-phase-result-v1",
        &[
            FramedField {
                tag: "ending_complete_owner_set_sha256",
                value: ending.as_bytes(),
            },
            FramedField {
                tag: "physical_ledger_set_sha256",
                value: ledger.as_bytes(),
            },
        ],
    )?)
}

fn adaptive_event_posture_v1(
    has_terminal_event: bool,
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Stage3AdaptiveEventPostureV1 {
    if has_terminal_event {
        Stage3AdaptiveEventPostureV1::TerminalEvent
    } else if pending_parcels.is_empty() {
        Stage3AdaptiveEventPostureV1::NoEvent
    } else {
        Stage3AdaptiveEventPostureV1::PendingParcel
    }
}

struct AdaptiveTrialReceiptEvidenceV1 {
    receipt: Stage3AdaptiveDirectTrialReceiptV1,
    exact_discrete_sha256: Digest32,
    exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
}

#[allow(clippy::too_many_arguments)]
fn adaptive_direct_trial_receipt_v1(
    request: &Stage3AdaptiveParentRequestReceiptV1,
    consumer: &DirectV10RealConsumerShadow,
    clock: &CoupledClockStateV1,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipts: &[Stage3CoupledSubslabReceiptV1],
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    has_terminal_event: bool,
) -> Result<AdaptiveTrialReceiptEvidenceV1, DirectSnowStage3V11AttachmentError> {
    let comparison = consumer
        .adaptive_complete_owner_comparison_v1(canonical_stage3_snow_owner_bytes(stage3)?)?;
    let (exact_discrete_sha256, exact_discrete_surfaces) =
        adaptive_discrete_surface_receipts_v1(&comparison, pending_parcels)?;
    let receipt = Stage3AdaptiveDirectTrialReceiptV1::try_new(
        request,
        adaptive_trial_ledger_set_sha256_v1(receipts)?,
        complete_owner_set_digest(clock.owners())?,
        adaptive_trial_phase_result_sha256_v1(stage3)?,
        adaptive_event_posture_v1(has_terminal_event, pending_parcels),
        Stage3AdaptiveTrialDispositionV1::Closed,
    )?;
    Ok(AdaptiveTrialReceiptEvidenceV1 {
        receipt,
        exact_discrete_sha256,
        exact_discrete_surfaces,
    })
}

fn adaptive_refinable_failure_digest_v1(
    request: &Stage3AdaptiveParentRequestReceiptV1,
    position: &'static str,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    Ok(framed_sha256(
        "stage3-adaptive-refinable-trial-failure-v1",
        &[
            FramedField {
                tag: "parent_request_sha256",
                value: request.receipt_sha256.as_bytes(),
            },
            FramedField {
                tag: "failure_class",
                value: b"terminal_step_domain",
            },
            FramedField {
                tag: "trial_position",
                value: position.as_bytes(),
            },
        ],
    )?)
}

#[allow(clippy::too_many_arguments)]
fn adaptive_record_refinable_trial_failure_v1(
    accumulator: &mut AdaptiveReceiptAccumulatorV1,
    request: &Stage3AdaptiveParentRequestReceiptV1,
    consumer: &DirectV10RealConsumerShadow,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    first_support: TimeSupport,
    second_support: TimeSupport,
    direct_evidence: Option<AdaptiveTrialReceiptEvidenceV1>,
    composed_receipts: Option<&[Stage3CoupledSubslabReceiptV1]>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let comparison = consumer
        .adaptive_complete_owner_comparison_v1(canonical_stage3_snow_owner_bytes(stage3)?)?;
    let (exact_discrete_sha256, exact_discrete_surfaces) =
        adaptive_discrete_surface_receipts_v1(&comparison, pending_parcels)?;
    let direct = if let Some(evidence) = direct_evidence {
        evidence.receipt
    } else {
        Stage3AdaptiveDirectTrialReceiptV1::try_new(
            request,
            adaptive_refinable_failure_digest_v1(request, "direct")?,
            request.context.beginning_complete_owner_set_sha256,
            adaptive_trial_phase_result_sha256_v1(stage3)?,
            adaptive_event_posture_v1(false, pending_parcels),
            Stage3AdaptiveTrialDispositionV1::TypedRejected,
        )?
    };
    let child_receipts = |support: TimeSupport| {
        composed_receipts.map(|receipts| {
            receipts
                .iter()
                .filter(|receipt| {
                    receipt.support.start_ns() >= support.start_ns()
                        && receipt.support.end_ns() <= support.end_ns()
                })
                .cloned()
                .collect::<Vec<_>>()
        })
    };
    let first_receipts = child_receipts(first_support);
    let second_receipts = child_receipts(second_support);
    let child_values = |position: &'static str,
                        receipts: Option<&Vec<Stage3CoupledSubslabReceiptV1>>|
     -> Result<
        (
            Digest32,
            Digest32,
            Digest32,
            Stage3AdaptiveTrialDispositionV1,
        ),
        DirectSnowStage3V11AttachmentError,
    > {
        if let Some(receipts) = receipts.filter(|receipts| !receipts.is_empty()) {
            Ok((
                adaptive_trial_ledger_set_sha256_v1(receipts)?,
                receipts
                    .last()
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive refinable child receipt",
                    ))?
                    .effective_ending_complete_owner_set_sha256(),
                adaptive_child_phase_result_sha256_v1(receipts)?,
                Stage3AdaptiveTrialDispositionV1::Closed,
            ))
        } else {
            Ok((
                adaptive_refinable_failure_digest_v1(request, position)?,
                request.context.beginning_complete_owner_set_sha256,
                adaptive_trial_phase_result_sha256_v1(stage3)?,
                Stage3AdaptiveTrialDispositionV1::TypedRejected,
            ))
        }
    };
    let (first_ledger, first_ending, first_phase, first_disposition) =
        child_values("child_1", first_receipts.as_ref())?;
    let child_1 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_1(
        request,
        &direct,
        first_support,
        first_ledger,
        first_ending,
        first_phase,
        Stage3AdaptiveEventPostureV1::NoEvent,
        first_disposition,
    )?;
    let (second_ledger, second_ending, second_phase, second_disposition) =
        child_values("child_2", second_receipts.as_ref())?;
    let child_2 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_2(
        request,
        &child_1,
        second_support,
        second_ledger,
        second_ending,
        second_phase,
        Stage3AdaptiveEventPostureV1::NoEvent,
        second_disposition,
    )?;
    let selected_ledger = composed_receipts.map_or_else(
        || adaptive_refinable_failure_digest_v1(request, "composed"),
        adaptive_trial_ledger_set_sha256_v1,
    )?;
    let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_composed(
        request,
        &direct,
        &child_1,
        &child_2,
        selected_ledger,
        exact_discrete_sha256,
        exact_discrete_sha256,
        exact_discrete_surfaces.clone(),
        exact_discrete_surfaces,
        0.0,
        false,
        false,
    )?;
    accumulator.parent_requests.push(request.clone());
    accumulator.direct_trials.push(direct);
    accumulator.split_child_trials.push(child_1);
    accumulator.split_child_trials.push(child_2);
    accumulator.comparisons.push(comparison);
    accumulator.reject(true, false)
}

fn adaptive_forcing_projection_sha256_v1(
    forcing_receipt: Digest32,
    support: TimeSupport,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let start = support.start_ns().get().to_be_bytes();
    let end = support.end_ns().get().to_be_bytes();
    Ok(framed_sha256(
        "stage3-adaptive-forcing-projection-v1",
        &[
            FramedField {
                tag: "parent_forcing_receipt_sha256",
                value: forcing_receipt.as_bytes(),
            },
            FramedField {
                tag: "support_start_ns",
                value: &start,
            },
            FramedField {
                tag: "support_end_ns",
                value: &end,
            },
        ],
    )?)
}

fn adaptive_configuration_sha256_v1(
    context: &DirectSnowStage3V11StaticContext,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    Ok(framed_sha256(
        "stage3-adaptive-configuration-v1",
        &[
            FramedField {
                tag: "controller_policy_sha256",
                value: context.controller_policy.as_bytes(),
            },
            FramedField {
                tag: "topology_sha256",
                value: context.topology_identity.as_bytes(),
            },
        ],
    )?)
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveReceiptAccumulatorV1 {
    parent_requests: Vec<Stage3AdaptiveParentRequestReceiptV1>,
    direct_trials: Vec<Stage3AdaptiveDirectTrialReceiptV1>,
    split_child_trials: Vec<Stage3AdaptiveSplitChildTrialReceiptV1>,
    comparisons: Vec<Stage3AdaptiveStepComparisonReceiptV1>,
    accepted_microsteps: Vec<Stage3AdaptiveAcceptedMicrostepReceiptV1>,
    attempt_ordinal: u32,
}

#[cfg(test)]
mod adaptive_receipt_accumulator_serialization_tests {
    use super::*;

    #[test]
    fn restart_accumulator_omits_diagnostic_counters_and_rejects_unknown_counter() {
        let accumulator = AdaptiveReceiptAccumulatorV1::default();
        let mut value = serde_json::to_value(&accumulator).expect("adaptive accumulator JSON");
        let object = value.as_object_mut().expect("adaptive accumulator object");
        for key in [
            "rejected_candidates",
            "phase_refinements",
            "event_refinements",
            "minimum_accepted_step_ns",
            "maximum_accepted_step_ns",
        ] {
            assert!(!object.contains_key(key), "persisted diagnostic key: {key}");
        }

        object.insert("rejected_candidates".to_owned(), serde_json::json!(1));
        assert!(serde_json::from_value::<AdaptiveReceiptAccumulatorV1>(value).is_err());
    }
}

impl AdaptiveReceiptAccumulatorV1 {
    fn request(
        &self,
        static_context: &DirectSnowStage3V11StaticContext,
        clock: &CoupledClockStateV1,
        parent_support: TimeSupport,
        step_support: TimeSupport,
        forcing_receipt: Digest32,
        proposed_step_quanta: u128,
    ) -> Result<Stage3AdaptiveParentRequestReceiptV1, DirectSnowStage3V11AttachmentError> {
        Stage3AdaptiveParentRequestReceiptV1::try_new(
            Stage3AdaptiveReceiptContextV1 {
                parent_transaction_id: clock.parent_transaction_id(),
                parent_support,
                step_support,
                step_ordinal: u32::try_from(self.accepted_microsteps.len()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive accepted microstep ordinal width",
                    )
                })?,
                attempt_ordinal: self.attempt_ordinal,
                beginning_complete_owner_set_sha256: complete_owner_set_digest(clock.owners())?,
                forcing_projection_sha256: adaptive_forcing_projection_sha256_v1(
                    forcing_receipt,
                    step_support,
                )?,
                topology_sha256: static_context.topology_identity,
                configuration_sha256: adaptive_configuration_sha256_v1(static_context)?,
            },
            proposed_step_quanta,
        )
    }

    fn accept(&mut self, accepted: Stage3AdaptiveAcceptedMicrostepReceiptV1) {
        self.accepted_microsteps.push(accepted);
        self.attempt_ordinal = 0;
    }

    fn reject(
        &mut self,
        phase: bool,
        event: bool,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        crate::snow_stage3_v11_attachment::record_adaptive_parent_rejection_v1(phase, event);
        self.attempt_ordinal = self.attempt_ordinal.checked_add(1).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("adaptive attempt ordinal overflow"),
        )?;
        Ok(())
    }

    fn finalize(
        self,
        clock: &CoupledClockStateV1,
        parent_support: TimeSupport,
    ) -> Result<Stage3AdaptiveSupportReceiptV1, DirectSnowStage3V11AttachmentError> {
        let parent_request_set_sha256 =
            stage3_adaptive_parent_request_set_sha256_v1(&self.parent_requests)?;
        let accepted_microstep_set_sha256 =
            stage3_adaptive_accepted_microstep_set_sha256_v1(&self.accepted_microsteps)?;
        let receipt = Stage3AdaptiveSupportReceiptV1 {
            parent_transaction_id: clock.parent_transaction_id(),
            parent_support,
            parent_requests: self.parent_requests,
            direct_trials: self.direct_trials,
            split_child_trials: self.split_child_trials,
            comparisons: self.comparisons,
            accepted_microsteps: self.accepted_microsteps,
            parent_request_set_sha256,
            accepted_microstep_set_sha256,
        };
        receipt.validate()?;
        Ok(receipt)
    }
}

fn adaptive_terminal_child_end_v1(
    adaptive_receipts: &AdaptiveReceiptAccumulatorV1,
    parent_support: TimeSupport,
) -> Result<ModelTimeNs, DirectSnowStage3V11AttachmentError> {
    let accepted = adaptive_receipts
        .accepted_microsteps
        .last()
        .ok_or_else(|| adaptive_receipt_identity_error("adaptive terminal child authority"))?;
    if accepted.receipt_sha256 != accepted.reconstructed_digest()? {
        return Err(adaptive_receipt_identity_error(
            "adaptive terminal child seal",
        ));
    }
    if accepted.context.parent_support != parent_support {
        return Err(adaptive_receipt_identity_error(
            "adaptive terminal child parent",
        ));
    }
    if accepted.event_posture != Stage3AdaptiveEventPostureV1::TerminalEvent {
        return Err(adaptive_receipt_identity_error(
            "adaptive terminal child authority",
        ));
    }
    Ok(accepted.context.step_support.end_ns())
}

struct AdaptiveCoveredTrialV1 {
    parent: Box<V11ParentTransaction>,
    consumer: Box<DirectV10RealConsumerShadow>,
    clock: Box<CoupledClockStateV1>,
    stage3: Box<BTreeMap<u32, DirectSnowStage3PersistentState>>,
    receipts: Vec<Stage3CoupledSubslabReceiptV1>,
    composed_children: Vec<AdaptiveCoveredTrialMemoEntryV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AdaptiveCoveredTrialMemoKeyV1 {
    parent_transaction_id: ParentTransactionId,
    support: TimeSupport,
    child_ordinal: u32,
    beginning_complete_owner_set_sha256: Digest32,
    forcing_projection_sha256: Digest32,
    topology_sha256: Digest32,
    configuration_sha256: Digest32,
    pending_terminal_parcel_set_sha256: Digest32,
}

struct AdaptiveCoveredTrialMemoEntryV1 {
    key: AdaptiveCoveredTrialMemoKeyV1,
    trial: Box<AdaptiveCoveredTrialV1>,
}

fn adaptive_pending_terminal_parcel_set_sha256_v1(
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let canonical = pending_terminal_parcels
        .iter()
        .map(|(key, parcel)| (key, parcel))
        .collect::<Vec<_>>();
    let bytes = serde_json::to_vec(&canonical).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "adaptive covered memo pending terminal parcel set",
        )
    })?;
    Ok(digest_bytes(&bytes))
}

#[allow(clippy::too_many_arguments)]
fn adaptive_covered_trial_memo_key_v1(
    context: &DirectSnowStage3V11StaticContext,
    clock: &CoupledClockStateV1,
    forcing_receipt: Digest32,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    support: TimeSupport,
    child_ordinal: u32,
) -> Result<AdaptiveCoveredTrialMemoKeyV1, DirectSnowStage3V11AttachmentError> {
    Ok(AdaptiveCoveredTrialMemoKeyV1 {
        parent_transaction_id: clock.parent_transaction_id(),
        support,
        child_ordinal,
        beginning_complete_owner_set_sha256: complete_owner_set_digest(clock.owners())?,
        forcing_projection_sha256: adaptive_forcing_projection_sha256_v1(forcing_receipt, support)?,
        topology_sha256: context.topology_identity,
        configuration_sha256: adaptive_configuration_sha256_v1(context)?,
        pending_terminal_parcel_set_sha256: adaptive_pending_terminal_parcel_set_sha256_v1(
            pending_terminal_parcels,
        )?,
    })
}

fn validate_adaptive_covered_trial_memo_key_v1(
    retained: AdaptiveCoveredTrialMemoKeyV1,
    requested: AdaptiveCoveredTrialMemoKeyV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if retained != requested {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive covered physical trial memo authority",
        ));
    }
    Ok(())
}

fn take_adaptive_covered_trial_memo_v1(
    memo: &mut Vec<AdaptiveCoveredTrialMemoEntryV1>,
    requested: AdaptiveCoveredTrialMemoKeyV1,
) -> Result<Option<Box<AdaptiveCoveredTrialV1>>, DirectSnowStage3V11AttachmentError> {
    let Some(index) = memo.iter().position(|entry| entry.key == requested) else {
        return Ok(None);
    };
    let entry = memo.remove(index);
    validate_adaptive_covered_trial_memo_key_v1(entry.key, requested)?;
    Ok(Some(entry.trial))
}

struct AdaptiveTerminalPathV1 {
    actual: Box<ActualTerminalSubslabV1>,
    ending_pending_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
}

#[allow(clippy::too_many_arguments)]
fn execute_adaptive_terminal_path_v1<M>(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    supports: &[TimeSupport],
    first_child_ordinal: u32,
    event_ordinal: u64,
    evidence: &mut M::State,
) -> Result<AdaptiveTerminalPathV1, DirectSnowStage3V11AttachmentError>
where
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
{
    let mut parent = beginning_parent.clone();
    let mut consumer = beginning_consumer.clone();
    let mut clock = beginning_clock.clone();
    let mut stage3 = beginning_stage3.clone();
    let mut pending = beginning_pending_terminal_parcels.clone();
    let mut receipts = Vec::new();
    let mut parcels = Vec::new();
    let mut accepted_group = None;
    for (offset, support) in supports.iter().copied().enumerate() {
        if support.start_ns() != clock.accepted_until() || accepted_group.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "adaptive terminal composed regime boundary",
            ));
        }
        let child_ordinal = first_child_ordinal
            .checked_add(u32::try_from(receipts.len()).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("adaptive terminal path ordinal width")
            })?)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive terminal path ordinal overflow",
            ))?;
        let active_lanes = stage3
            .iter()
            .filter_map(|(lane, state)| {
                (stage3_is_resolved_thermal_domain(state)
                    || crate::hydrology::stage3_is_terminal_event_domain(state)
                    || (state.layers.is_empty()
                        && prepared
                            .support_forcing_by_lane
                            .get(lane)
                            .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)))
                .then_some(*lane)
            })
            .collect::<BTreeSet<_>>();
        let projected = prepared
            .coupled_subslab(support, child_ordinal)?
            .retain_active_snow_lanes(&active_lanes)?;
        let actual = if stage3
            .values()
            .any(crate::hydrology::stage3_is_terminal_event_domain)
        {
            let terminal = try_actual_terminal_subslab_with_evidence::<M>(
                context,
                &parent,
                &consumer,
                &clock,
                &projected,
                day_index,
                interval_index,
                forcing_receipt,
                &stage3,
                &pending,
                f64::from_bits(support.duration_s_bits()),
                child_ordinal,
                event_ordinal,
                evidence,
            )?;
            if let Some(actual) = terminal {
                actual
            } else {
                let solid_reappearance = stage3.iter().any(|(lane, state)| {
                    crate::hydrology::stage3_is_terminal_event_domain(state)
                        && projected
                            .support_forcing_by_lane
                            .get(lane)
                            .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)
                });
                if !solid_reappearance {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive terminal path lost terminal domain",
                    ));
                }
                let ordinary = execute_adaptive_covered_trial_v1(
                    context,
                    &parent,
                    &consumer,
                    &clock,
                    prepared,
                    day_index,
                    interval_index,
                    forcing_receipt,
                    &stage3,
                    &pending,
                    None,
                    &[support],
                    child_ordinal,
                )?;
                ActualTerminalSubslabV1 {
                    parent: *ordinary.parent,
                    consumer: *ordinary.consumer,
                    clock: *ordinary.clock,
                    stage3: *ordinary.stage3,
                    receipts: ordinary.receipts,
                    group: None,
                    parcels: Vec::new(),
                }
            }
        } else {
            // A terminal-domain lane may leave the terminal control volume at
            // an accepted composed endpoint (for example when snowfall makes
            // a reappeared pack thermally resolved).  Continue the remaining
            // composed support through the ordinary covered owner path from
            // that exact accepted state; restarting terminal integration
            // would apply the wrong control-volume contract.
            if receipts.is_empty() {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "adaptive terminal path missing entry terminal domain",
                ));
            }
            let ordinary = execute_adaptive_covered_trial_v1(
                context,
                &parent,
                &consumer,
                &clock,
                prepared,
                day_index,
                interval_index,
                forcing_receipt,
                &stage3,
                &pending,
                None,
                &[support],
                child_ordinal,
            )?;
            ActualTerminalSubslabV1 {
                parent: *ordinary.parent,
                consumer: *ordinary.consumer,
                clock: *ordinary.clock,
                stage3: *ordinary.stage3,
                receipts: ordinary.receipts,
                group: None,
                parcels: Vec::new(),
            }
        };
        parent = actual.parent;
        consumer = actual.consumer;
        clock = actual.clock;
        stage3 = actual.stage3;
        receipts.extend(actual.receipts);
        for parcel in actual.parcels {
            if pending
                .insert(parcel.parcel_digest, parcel.clone())
                .is_some()
            {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "adaptive terminal path duplicate parcel",
                ));
            }
            parcels.push(parcel);
        }
        accepted_group = actual.group;
        if accepted_group.is_some() && offset + 1 != supports.len() {
            return Err(DirectSnowStage3V11AttachmentError::AdaptiveRefinement(
                "adaptive terminal event precedes composed endpoint",
            ));
        }
    }
    Ok(AdaptiveTerminalPathV1 {
        actual: Box::new(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts,
            group: accepted_group,
            parcels,
        }),
        ending_pending_terminal_parcels: pending,
    })
}

#[allow(clippy::too_many_arguments)]
fn execute_adaptive_covered_trial_v1(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    mut trial_memo: Option<&mut Vec<AdaptiveCoveredTrialMemoEntryV1>>,
    supports: &[TimeSupport],
    first_child_ordinal: u32,
) -> Result<Box<AdaptiveCoveredTrialV1>, DirectSnowStage3V11AttachmentError> {
    #[cfg(test)]
    let trial_started = std::time::Instant::now();
    let mut parent = Box::new(beginning_parent.clone());
    let mut consumer = Box::new(beginning_consumer.clone());
    let mut clock = Box::new(beginning_clock.clone());
    let mut stage3 = Box::new(beginning_stage3.clone());
    let mut receipts = Vec::with_capacity(supports.len());
    let mut composed_children = Vec::with_capacity(supports.len());
    for (offset, support) in supports.iter().copied().enumerate() {
        if support.start_ns() != clock.accepted_until() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive trial child predecessor support",
            ));
        }
        let active_lanes = stage3
            .iter()
            .filter_map(|(lane, state)| {
                (stage3_is_resolved_thermal_domain(state)
                    || crate::hydrology::stage3_is_terminal_event_domain(state)
                    || (state.layers.is_empty()
                        && prepared
                            .support_forcing_by_lane
                            .get(lane)
                            .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)))
                .then_some(*lane)
            })
            .collect::<BTreeSet<_>>();
        if active_lanes.is_empty() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive covered trial lost every snow lane",
            ));
        }
        let ordinal = first_child_ordinal
            .checked_add(u32::try_from(offset).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("adaptive trial child ordinal width")
            })?)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive trial child ordinal overflow",
            ))?;
        let memo_key = adaptive_covered_trial_memo_key_v1(
            context,
            &clock,
            forcing_receipt,
            pending_terminal_parcels,
            support,
            ordinal,
        )?;
        let memoized = if adaptive_covered_child_memo_disabled_v1() {
            None
        } else if let Some(memo) = trial_memo.as_deref_mut() {
            take_adaptive_covered_trial_memo_v1(memo, memo_key)?
        } else {
            None
        };
        crate::snow_stage3_v11_attachment::record_adaptive_parent_covered_child_memo_v1(
            memoized.is_some(),
            supports.len() > 1,
        );
        let (next_parent, next_consumer, next_clock, next_stage3, receipt) =
            if let Some(memoized) = memoized {
                let AdaptiveCoveredTrialV1 {
                    parent,
                    consumer,
                    clock,
                    stage3,
                    mut receipts,
                    composed_children,
                } = *memoized;
                if receipts.len() != 1
                    || receipts[0].support != support
                    || !composed_children.is_empty()
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive covered physical trial memo result",
                    ));
                }
                let receipt = receipts.remove(0);
                (*parent, *consumer, *clock, *stage3, receipt)
            } else {
                let subslab = prepared
                    .coupled_subslab(support, ordinal)?
                    .retain_active_snow_lanes(&active_lanes)?;
                let selected_upper_bound_s = f64::from_bits(support.duration_s_bits());
                #[cfg(test)]
                let subslab_started = std::time::Instant::now();
                let result = execute_covered_real_v11_subslab(
                    context,
                    &parent,
                    &consumer,
                    &clock,
                    &subslab,
                    day_index,
                    interval_index,
                    forcing_receipt,
                    std::mem::take(&mut *stage3),
                    pending_terminal_parcels,
                    selected_upper_bound_s,
                    None,
                )?;
                #[cfg(test)]
                record_adaptive_performance_span_v1(
                    "covered_complete_owner_subslab",
                    support.duration_ns(),
                    subslab_started,
                );
                result
            };
        if !receipt.terminal_events.is_empty() {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "adaptive ordinary trial crossed terminal event",
            ));
        }
        parent = Box::new(next_parent);
        consumer = Box::new(next_consumer);
        clock = Box::new(next_clock);
        stage3 = Box::new(next_stage3);
        receipts.push(receipt);
        if supports.len() > 1 {
            composed_children.push(AdaptiveCoveredTrialMemoEntryV1 {
                key: memo_key,
                trial: Box::new(AdaptiveCoveredTrialV1 {
                    parent: parent.clone(),
                    consumer: consumer.clone(),
                    clock: clock.clone(),
                    stage3: stage3.clone(),
                    receipts: vec![receipts.last().cloned().ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "adaptive covered memo child receipt",
                        ),
                    )?],
                    composed_children: Vec::new(),
                }),
            });
        }
    }
    let result = Box::new(AdaptiveCoveredTrialV1 {
        parent,
        consumer,
        clock,
        stage3,
        receipts,
        composed_children,
    });
    #[cfg(test)]
    record_adaptive_performance_span_v1(
        if supports.len() == 1 {
            "complete_owner_direct_trial"
        } else {
            "complete_owner_composed_trial"
        },
        supports.iter().map(|support| support.duration_ns()).sum(),
        trial_started,
    );
    Ok(result)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn execute_covered_real_v11_parent_capture(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    failure_injection: Option<Stage3V11FailureInjection>,
) -> (
    Result<
        (
            V11ParentTransaction,
            DirectV10RealConsumerShadow,
            CoupledClockStateV1,
            V11ParentCandidate,
            BTreeMap<u32, DirectSnowStage3PersistentState>,
            Vec<Stage3CoupledSubslabReceiptV1>,
            Vec<Stage3V11TerminalEventGroupV1>,
            Vec<DirectSnowStage3V11TerminalParcel>,
        ),
        DirectSnowStage3V11AttachmentError,
    >,
    crate::hydrology::CaptureState,
) {
    let mut evidence =
        <crate::hydrology::CaptureEvidence as crate::hydrology::TerminalEvidenceMode<
            Option<CoveredTerminalJointTrialStateV1>,
        >>::new_state();
    let result =
        execute_covered_real_v11_parent_with_evidence::<crate::hydrology::CaptureEvidence>(
            context,
            beginning_parent,
            beginning_consumer,
            beginning_clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_stage3,
            beginning_terminal_parcels,
            failure_injection,
            &mut evidence,
            None,
            None,
        )
        .and_then(|outcome| match outcome {
            AdaptiveSupportExecutionOutcomeV2::Complete((
                parent,
                consumer,
                clock,
                finalized,
                stage3,
                receipts,
                groups,
                parcels,
                _,
                _,
            )) => Ok((
                parent, consumer, clock, finalized, stage3, receipts, groups, parcels,
            )),
            AdaptiveSupportExecutionOutcomeV2::Paused(_) => {
                Err(DirectSnowStage3V11AttachmentError::Identity(
                    "capture execution cannot be restart-interrupted",
                ))
            }
        });
    (result, evidence)
}

include!("snow_stage3_v11_adaptive_outcomes.rs");
include!("snow_stage3_v11_reappearance_transition.rs");

#[derive(Clone, Copy, Debug, PartialEq)]
struct SolidReappearanceThermodynamicsV1 {
    mass_kg_m2: f64,
    advected_energy_j_m2: f64,
    temperature_c: f64,
}

fn reconstruct_solid_reappearance_thermodynamics_v1(
    weighted_parcels: &[(f64, f64, f64, f64)],
) -> Result<SolidReappearanceThermodynamicsV1, DirectSnowStage3V11AttachmentError> {
    const SOLID_SPECIFIC_HEAT_J_KG_K: f64 = 2_100.0;
    const MELTING_TEMPERATURE_K: f64 = 273.15;
    let mut mass_kg_m2 = 0.0;
    let mut advected_energy_j_m2 = 0.0;
    for &(destination_fraction, parcel_mass_kg_m2, parcel_temperature_k, parcel_enthalpy_j_m2) in
        weighted_parcels
    {
        let expected_enthalpy_j_m2 = parcel_mass_kg_m2
            * SOLID_SPECIFIC_HEAT_J_KG_K
            * (parcel_temperature_k - MELTING_TEMPERATURE_K);
        if !destination_fraction.is_finite()
            || destination_fraction <= 0.0
            || !parcel_mass_kg_m2.is_finite()
            || parcel_mass_kg_m2 < 0.0
            || !parcel_temperature_k.is_finite()
            || parcel_temperature_k <= 0.0
            || parcel_temperature_k > MELTING_TEMPERATURE_K
            || !parcel_enthalpy_j_m2.is_finite()
            || parcel_enthalpy_j_m2.to_bits() != expected_enthalpy_j_m2.to_bits()
        {
            return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "solid reappearance parcel thermodynamics",
            ));
        }
        mass_kg_m2 += destination_fraction * parcel_mass_kg_m2;
        advected_energy_j_m2 += destination_fraction * parcel_enthalpy_j_m2;
    }
    if !mass_kg_m2.is_finite()
        || mass_kg_m2 <= 0.0
        || !advected_energy_j_m2.is_finite()
        || advected_energy_j_m2 > 0.0
    {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "solid reappearance parcel aggregate",
        ));
    }
    let temperature_c = if advected_energy_j_m2.to_bits() == 0.0_f64.to_bits() {
        0.0
    } else {
        advected_energy_j_m2 / (mass_kg_m2 * SOLID_SPECIFIC_HEAT_J_KG_K)
    };
    if !temperature_c.is_finite() || temperature_c > 0.0 {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "solid reappearance parcel temperature",
        ));
    }
    Ok(SolidReappearanceThermodynamicsV1 {
        mass_kg_m2,
        advected_energy_j_m2,
        temperature_c,
    })
}

fn validate_solid_reappearance_beginning_snow_owner_v1(
    installed: &[u8],
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let canonical_v1 = canonical_stage3_snow_owner_bytes(beginning_stage3)?;
    if installed == canonical_v1
        || crate::snow_owner_v4::validate_snow_owner_v4_reappearance_projection(
            installed,
            beginning_stage3,
        )
        .is_ok()
    {
        return Ok(());
    }
    Err(DirectSnowStage3V11AttachmentError::Identity(
        "solid reappearance beginning snow owner join",
    ))
}

#[cfg(test)]
mod solid_reappearance_thermodynamics_tests {
    use super::*;

    #[test]
    fn warm_mixed_bulk_temperature_cannot_override_sealed_solid_parcel() {
        const BULK_MIXED_TEMPERATURE_C: f64 = 1.25;
        let parcels = [(1.0, 4.0, 273.15, 0.0)];
        let result = reconstruct_solid_reappearance_thermodynamics_v1(&parcels)
            .expect("bounded solid parcel");
        assert_eq!(result.temperature_c.to_bits(), 0.0_f64.to_bits());
        assert_ne!(
            result.temperature_c.to_bits(),
            BULK_MIXED_TEMPERATURE_C.to_bits(),
            "bulk mixed-phase temperature is not solid material authority",
        );
        let mut warm_solid_poison = parcels;
        warm_solid_poison[0].2 = 273.16;
        warm_solid_poison[0].3 = 4.0 * 2_100.0 * 0.01;
        assert!(reconstruct_solid_reappearance_thermodynamics_v1(&warm_solid_poison).is_err());
        let mut enthalpy_poison = parcels;
        enthalpy_poison[0].3 = -1.0;
        assert!(reconstruct_solid_reappearance_thermodynamics_v1(&enthalpy_poison).is_err());
    }
}

#[allow(clippy::too_many_arguments)]
fn canonical_solid_reappearance_transition_v1(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Option<SolidReappearanceTransitionV1>, DirectSnowStage3V11AttachmentError> {
    let lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            (state.layers.is_empty()
                && !stage3_has_represented_ice(state)
                && prepared
                    .support_forcing_by_lane
                    .get(lane)
                    .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0))
            .then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    if lanes.is_empty() {
        return Ok(None);
    }
    if beginning_clock.accepted_until() != prepared.support.start_ns()
        || !beginning_terminal_parcels.is_empty()
    {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "solid reappearance beginning chronology or parcel posture",
        ));
    }
    let mut stage3 = beginning_stage3.clone();
    let mut lane_receipts = Vec::with_capacity(lanes.len());
    let topology_digest = digest_bytes(
        &context
            .surface_liquid_configuration
            .canonical_bytes()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance destination topology",
                )
            })?,
    );
    for lane in &lanes {
        let beginning =
            beginning_stage3
                .get(lane)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance beginning lane",
                ))?;
        if beginning.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits() {
            return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "solid reappearance retained liquid",
            ));
        }
        let forcing = prepared.support_forcing_by_lane.get(lane).copied().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("solid reappearance forcing lane"),
        )?;
        let source_identities = prepared.support_identity_by_lane.get(lane).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("solid reappearance source-custody lane"),
        )?;
        if source_identities.windows(2).any(|pair| {
            (&pair[0].destination_ofe_id, &pair[0].destination_tile_id)
                >= (&pair[1].destination_ofe_id, &pair[1].destination_tile_id)
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "solid reappearance canonical destination order",
            ));
        }
        if !source_identities
            .iter()
            .any(|identity| !identity.solid_precipitation_parcels.is_empty())
        {
            return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "solid reappearance source-custody parcel",
            ));
        }
        let source_custody_digest =
            digest_bytes(&serde_json::to_vec(source_identities).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance source-custody serialization",
                )
            })?);
        let inputs = prepared.snow_inputs_by_lane.get(lane).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("solid reappearance input lane"),
        )?;
        let ofe_id = context
            .surface_liquid_configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.production_lane_id == *lane)
            .map(|binding| &binding.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "solid reappearance lane/OFE topology",
            ))?;
        let mut weighted_solid_parcels = Vec::new();
        for identity in source_identities {
            if identity.destination_ofe_id != ofe_id.as_str() {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance source/OFE topology",
                ));
            }
            let fraction = context
                .surface_liquid_configuration
                .records
                .iter()
                .find(|record| {
                    record.key.ofe_id == *ofe_id
                        && record.key.tile_id.as_str() == identity.destination_tile_id
                })
                .map(|record| record.tile_fraction)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance source/destination topology",
                ))?;
            for parcel in &identity.solid_precipitation_parcels {
                if parcel.destination_ofe_id != identity.destination_ofe_id
                    || parcel.destination_tile_id != identity.destination_tile_id
                {
                    return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                        "solid reappearance parcel destination",
                    ));
                }
                weighted_solid_parcels.push((
                    fraction,
                    parcel.mass_kg_m2,
                    parcel.temperature_k,
                    parcel.enthalpy_j_m2,
                ));
            }
        }
        const SOLID_SPECIFIC_HEAT_J_KG_K: f64 = 2_100.0;
        let thermodynamics =
            reconstruct_solid_reappearance_thermodynamics_v1(&weighted_solid_parcels)?;
        let solid_mass_kg_m2 = thermodynamics.mass_kg_m2;
        let advected_energy_j_m2 = thermodynamics.advected_energy_j_m2;
        let material_temperature_c = thermodynamics.temperature_c;
        let forcing_mass_kg_m2 = forcing.forcing.snowfall_m * 100.0;
        let mass_swe_m = solid_mass_kg_m2 / 1_000.0;
        let density_kg_m3 = inputs.newsnw_kg_m3;
        if !solid_mass_kg_m2.is_finite()
            || solid_mass_kg_m2 <= 0.0
            || (solid_mass_kg_m2 - forcing_mass_kg_m2).abs() > 1.0e-9
            || !density_kg_m3.is_finite()
            || density_kg_m3 <= 0.0
            || !advected_energy_j_m2.is_finite()
            || advected_energy_j_m2 > 0.0
        {
            return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "solid reappearance material domain",
            ));
        }
        let cold_content_j_m2 = -advected_energy_j_m2;
        let mut ending = beginning.clone();
        ending.layers = vec![DirectSnowLayerState {
            mass_swe_m,
            thickness_m: mass_swe_m * 1_000.0 / density_kg_m3,
            density_kg_m3,
            settle_day_count: 0.0,
            temperature_c: material_temperature_c,
            liquid_water_m: 0.0,
            cold_content_j_m2,
            refrozen_liquid_m: 0.0,
        }];
        ending.cumulative_snowfall_kg_m2 += solid_mass_kg_m2;
        ending.cumulative_complete_energy_j_m2 += advected_energy_j_m2;
        ending.cumulative_cold_energy_change_j_m2 += advected_energy_j_m2;
        ending.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&ending);
        Wb11HydrologyKernel::validate_stage3_persistent_state(&ending)?;
        let forcing_digest = parse_lower_hex_digest(
            crate::v9_real_consumer_shadow::stage3_support_forcing_digest(forcing)?.as_str(),
        )?;
        let beginning_digest = digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
        );
        let ending_digest = digest_bytes(&Wb11HydrologyKernel::serialize_stage3_persistent_state(
            &ending,
        )?);
        let scalars = [
            solid_mass_kg_m2,
            mass_swe_m,
            density_kg_m3,
            material_temperature_c,
            SOLID_SPECIFIC_HEAT_J_KG_K,
            advected_energy_j_m2,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        let lane_bytes = lane.to_be_bytes();
        lane_receipts.push(framed_sha256(
            "stage3-v11-solid-reappearance-lane-v1",
            &[
                FramedField {
                    tag: "lane",
                    value: &lane_bytes,
                },
                FramedField {
                    tag: "forcing",
                    value: forcing_digest.as_bytes(),
                },
                FramedField {
                    tag: "topology",
                    value: topology_digest.as_bytes(),
                },
                FramedField {
                    tag: "source_custody",
                    value: source_custody_digest.as_bytes(),
                },
                FramedField {
                    tag: "beginning",
                    value: beginning_digest.as_bytes(),
                },
                FramedField {
                    tag: "ending",
                    value: ending_digest.as_bytes(),
                },
                FramedField {
                    tag: "material",
                    value: &scalars,
                },
            ],
        )?);
        stage3.insert(*lane, ending);
    }
    let ending_snow_bytes = canonical_stage3_snow_owner_bytes(&stage3)?;
    let installed_beginning_snow = beginning_clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "solid reappearance beginning snow owner",
        ))?;
    validate_solid_reappearance_beginning_snow_owner_v1(
        installed_beginning_snow.state_bytes(),
        beginning_stage3,
    )?;
    let ending_owners = beginning_clock
        .owners()
        .iter()
        .map(|owner| {
            if owner.owner_id() == "snow" {
                OwnerState::new("snow".to_owned(), ending_snow_bytes.clone())
            } else {
                Ok(owner.clone())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let lane_fields = lane_receipts
        .iter()
        .map(|receipt| FramedField {
            tag: "lane",
            value: receipt.as_bytes(),
        })
        .collect::<Vec<_>>();
    let lane_set = framed_sha256("stage3-v11-solid-reappearance-lane-set-v1", &lane_fields)?;
    let beginning_owner_set = complete_owner_set_digest(beginning_clock.owners())?;
    let ending_owner_set = complete_owner_set_digest(&ending_owners)?;
    let context_digest = framed_sha256(
        "stage3-v11-solid-reappearance-transition-v1",
        &[
            FramedField {
                tag: "parent_transaction",
                value: beginning_clock.parent_transaction_id().digest().as_bytes(),
            },
            FramedField {
                tag: "tick",
                value: &prepared.support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "lanes",
                value: lane_set.as_bytes(),
            },
            FramedField {
                tag: "beginning_owner_set",
                value: beginning_owner_set.as_bytes(),
            },
            FramedField {
                tag: "ending_owner_set",
                value: ending_owner_set.as_bytes(),
            },
        ],
    )?;
    let ledger = LedgerEntryV1::new(
        "solid-reappearance-precipitation".to_owned(),
        "kg-m-2-ofe-ground-and-j-m-2".to_owned(),
        lane_set,
        lane_set,
        context_digest,
    )?;
    // Reappearance mutates the canonical aggregate snow owner.  Lane IDs are
    // sealed in `lane_set`, but they are not independently installed coupled-
    // time owners and therefore cannot be introduced as active participants.
    let participants = beginning_clock.active_participants().to_vec();
    let event = EventProposalV1::new(
        EventClass::RegimeTransition,
        "snow".to_owned(),
        context_digest,
        ending_owners.clone(),
        vec!["snow".to_owned()],
        "snow-stage3-v11-mixed".to_owned(),
        participants,
        vec![ledger],
    )?;
    let mut parent = beginning_parent.clone();
    let consumer = beginning_consumer.clone();
    let mut clock = beginning_clock.clone();
    let mut queue = EventQueueV1::new(prepared.support.start_ns(), vec![event])?;
    let accepted_event =
        queue
            .apply_next(&mut clock)?
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "solid reappearance event application",
            ))?;
    if queue.apply_next(&mut clock)?.is_some()
        || accepted_event.beginning_owner_set_digest() != beginning_owner_set
        || accepted_event.ending_owner_set_digest() != ending_owner_set
        || accepted_event.event_context_digest() != context_digest
        || clock.owners() != ending_owners
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "solid reappearance event owner join",
        ));
    }
    parent.accept_zero_duration_owner_transition(
        &context.vegetation_configuration,
        prepared.support.start_ns(),
        owner_envelopes_from_states(&ending_owners)?,
        &["snow".to_owned()],
    )?;
    Ok(Some(SolidReappearanceTransitionV1 {
        lanes,
        parent,
        consumer,
        clock,
        stage3,
        accepted_event,
    }))
}

include!("snow_stage3_v11_adaptive_execution_stack_helpers.rs");

fn execute_covered_real_v11_parent_with_evidence<M>(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    failure_injection: Option<Stage3V11FailureInjection>,
    evidence: &mut M::State,
    restart: Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
    interrupt_at: Option<DirectSnowStage3V11InterruptionPostureV2>,
) -> Result<AdaptiveSupportExecutionOutcomeV2, DirectSnowStage3V11AttachmentError>
where
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
{
    let parent_telemetry_started = std::time::Instant::now();
    let mut execution = match initialize_adaptive_parent_execution_v1(
        context,
        beginning_parent,
        beginning_consumer,
        beginning_clock,
        prepared,
        beginning_stage3,
        beginning_terminal_parcels,
        failure_injection,
        restart,
        interrupt_at,
    )? {
        AdaptiveParentInitializationOutcomeV1::Paused(outcome) => return Ok(*outcome),
        AdaptiveParentInitializationOutcomeV1::Ready(execution) => execution,
    };
    execution = match execute_adaptive_parent_loop_closure_v1(execution, |execution| {
        let AdaptiveParentExecutionStateV1 {
            prepared,
            mut restart,
            mut parent,
            mut consumer,
            mut clock,
            mut stage3,
            mut owner_joins,
            mut event_groups,
            mut terminal_parcels,
            mut pending_terminal_parcels,
            mut expected_child_beginning,
            mut adaptive_receipts,
            mut snow_free_successor_receipts,
            mut adaptive_trial_quanta,
            mut covered_trial_memo,
        } = *execution;
        const ADAPTIVE_MIN_STEP_NS: u128 = STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
        macro_rules! interrupt {
            ($posture:expr, $request:expr) => {
                if let Some(outcome) = adaptive_interruption_outcome_v2(
                    interrupt_at,
                    $posture,
                    &mut restart,
                    $request,
                    &parent,
                    &consumer,
                    &clock,
                    &stage3,
                    &pending_terminal_parcels,
                    &owner_joins,
                    &event_groups,
                    &terminal_parcels,
                    expected_child_beginning,
                    &adaptive_receipts,
                    &snow_free_successor_receipts,
                    adaptive_trial_quanta,
                )? {
                    return Ok(AdaptiveParentLoopOutcomeV1::Paused(outcome));
                }
            };
        }
        while clock.accepted_until() < prepared.support.end_ns() {
            let active_lanes = stage3
                .iter()
                .filter_map(|(lane, state)| {
                    (stage3_is_resolved_thermal_domain(state)
                        || crate::hydrology::stage3_is_terminal_event_domain(state)
                        || (state.layers.is_empty()
                            && prepared
                                .support_forcing_by_lane
                                .get(lane)
                                .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)))
                    .then_some(*lane)
                })
                .collect::<BTreeSet<_>>();
            if active_lanes.is_empty() {
                let receiver_pending = !pending_terminal_parcels.is_empty();
                if receiver_pending {
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                }
                let successor_end = if receiver_pending {
                    ModelTimeNs::new(
                        clock
                            .accepted_until()
                            .get()
                            .checked_add(ADAPTIVE_MIN_STEP_NS)
                            .ok_or_else(|| {
                                adaptive_receipt_identity_error(
                                    "terminal receiver support overflow",
                                )
                            })?
                            .min(prepared.support.end_ns().get()),
                    )
                } else if !snow_free_successor_receipts.is_empty() {
                    let terminal_child_end =
                        adaptive_terminal_child_end_v1(&adaptive_receipts, prepared.support)?;
                    if clock.accepted_until() >= terminal_child_end {
                        prepared.support.end_ns()
                    } else {
                        let cadence_ns = adaptive_trial_quanta
                            .checked_mul(ADAPTIVE_MIN_STEP_NS)
                            .filter(|duration| *duration > 0)
                            .ok_or_else(|| {
                                adaptive_receipt_identity_error("successor cadence overflow")
                            })?;
                        ModelTimeNs::new(
                            clock
                                .accepted_until()
                                .get()
                                .checked_add(cadence_ns)
                                .ok_or_else(|| {
                                    adaptive_receipt_identity_error("successor support overflow")
                                })?
                                .min(terminal_child_end.get()),
                        )
                    }
                } else {
                    prepared.support.end_ns()
                };
                let remainder_support = TimeSupport::new(clock.accepted_until(), successor_end)?;
                if receiver_pending && remainder_support.duration_ns() != ADAPTIVE_MIN_STEP_NS {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "terminal receiver cannot obtain one minimum quantum",
                    ));
                }
                let mut successor = prepared
                    .coupled_subslab(
                        remainder_support,
                        u32::try_from(owner_joins.len()).map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Identity(
                                "successor subslab ordinal",
                            )
                        })?,
                    )?
                    .snow_free_successor()?;
                if receiver_pending {
                    successor =
                        successor.with_terminal_receiver_parcels(&pending_terminal_parcels)?;
                }
                if remainder_support.end_ns() == prepared.support.end_ns() {
                    for state in stage3.values_mut() {
                        let beginning = state.clone();
                        Wb11HydrologyKernel::project_stage3_parent_cadence_state(
                            &beginning, state, true,
                        )?;
                    }
                }
                let ending_snow_owner_bytes = if receiver_pending {
                    canonical_stage3_snow_owner_bytes(&stage3)?
                } else if pending_terminal_parcels.is_empty() {
                    canonical_stage3_snow_owner_bytes(&stage3)?
                } else {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "unreachable terminal receiver posture",
                    ));
                };
                let beginning_pending_terminal_parcels = pending_terminal_parcels.clone();
                let next = execute_adaptive_snow_free_successor_v1(
                    context,
                    &parent,
                    &consumer,
                    &clock,
                    &successor,
                    day_index,
                    interval_index,
                    forcing_receipt,
                    ending_snow_owner_bytes,
                )?;
                if receiver_pending {
                    for parcel in pending_terminal_parcels.values() {
                        if let Some(produced) = terminal_parcels
                            .iter_mut()
                            .find(|value| value.parcel_digest == parcel.parcel_digest)
                        {
                            produced.posture = DirectSnowStage3V11TerminalParcelPosture::Consumed;
                        } else {
                            let mut consumed = parcel.clone();
                            consumed.posture = DirectSnowStage3V11TerminalParcelPosture::Consumed;
                            terminal_parcels.push(consumed);
                        }
                    }
                    pending_terminal_parcels.clear();
                }
                snow_free_successor_receipts.push(Stage3SnowFreeSuccessorReceiptV1::seal(
                    &successor,
                    day_index,
                    interval_index,
                    beginning_parent.parent_transaction_id(),
                    u32::try_from(snow_free_successor_receipts.len()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "snow-free successor receipt ordinal",
                        )
                    })?,
                    forcing_receipt,
                    &beginning_pending_terminal_parcels,
                    &pending_terminal_parcels,
                    next.accepted_support,
                )?);
                parent = next.parent;
                consumer = next.consumer;
                clock = next.clock;
                if receiver_pending {
                    expected_child_beginning = complete_owner_set_digest(clock.owners())?;
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                }
                if clock.accepted_until() == prepared.support.end_ns() {
                    break;
                }
                continue;
            }
            let candidate_ceiling = prepared
                .hard_boundaries
                .iter()
                .copied()
                .find(|boundary| {
                    *boundary > clock.accepted_until() && *boundary < prepared.support.end_ns()
                })
                .unwrap_or(prepared.support.end_ns());
            let available_ns = candidate_ceiling.get() - clock.accepted_until().get();
            if available_ns == 0 || available_ns % ADAPTIVE_MIN_STEP_NS != 0 {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "adaptive hard boundary is outside the 60-second grid",
                ));
            }
            let available_quanta = available_ns / ADAPTIVE_MIN_STEP_NS;
            let candidate_quanta = adaptive_trial_quanta.min(available_quanta).max(1);
            let candidate_ns = candidate_quanta * ADAPTIVE_MIN_STEP_NS;
            let end_ns = ModelTimeNs::new(clock.accepted_until().get() + candidate_ns);
            let support = TimeSupport::new(clock.accepted_until(), end_ns)?;
            let child_ordinal = u32::try_from(owner_joins.len()).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("coupled subslab ordinal overflow")
            })?;
            let adaptive_request = adaptive_receipts.request(
                context,
                &clock,
                prepared.support,
                support,
                forcing_receipt,
                candidate_quanta,
            )?;
            if let Some(checkpoint) = restart.as_mut() {
                if let Some(expected) = checkpoint.pending_adaptive_request.take() {
                    if expected != adaptive_request {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "restart pending adaptive request join",
                        ));
                    }
                }
            }
            let snow_reappearance = stage3.iter().any(|(lane, state)| {
                state.layers.is_empty()
                    && prepared
                        .support_forcing_by_lane
                        .get(lane)
                        .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)
            });
            if stage3
                .values()
                .any(crate::hydrology::stage3_is_terminal_event_domain)
            {
                interrupt!(
                    DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalEvent,
                    Some(&adaptive_request)
                );
                let event_ordinal = u64::try_from(clock.event_ordinal()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width")
                })?;
                let (accepted, maximum_scaled_error) =
                    match select_adaptive_terminal_candidate_v1::<M>(
                        context,
                        &parent,
                        &consumer,
                        &clock,
                        &prepared,
                        day_index,
                        interval_index,
                        forcing_receipt,
                        &stage3,
                        &pending_terminal_parcels,
                        &mut adaptive_receipts,
                        &adaptive_request,
                        support,
                        candidate_quanta,
                        child_ordinal,
                        event_ordinal,
                        evidence,
                    )? {
                        AdaptiveCandidateSelectionV1::Accepted {
                            trial,
                            maximum_scaled_error,
                        } => (*trial, maximum_scaled_error),
                        AdaptiveCandidateSelectionV1::Refine { next_trial_quanta } => {
                            adaptive_trial_quanta = next_trial_quanta;
                            continue;
                        }
                    };
                let AdaptiveTerminalPathV1 {
                    mut actual,
                    ending_pending_terminal_parcels,
                } = accepted;
                let mut terminal_predecessor = expected_child_beginning;
                for receipt in &actual.receipts {
                    if receipt.owner_join.beginning_complete_owner_set_sha256
                        != terminal_predecessor
                    {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "terminal child complete-owner predecessor join",
                        ));
                    }
                    terminal_predecessor = receipt.effective_ending_complete_owner_set_sha256();
                }
                parent = actual.parent;
                consumer = actual.consumer;
                clock = actual.clock;
                stage3 = actual.stage3;
                owner_joins.append(&mut actual.receipts);
                if let Some(group) = actual.group.take() {
                    event_groups.push(group);
                }
                pending_terminal_parcels = ending_pending_terminal_parcels;
                terminal_parcels.append(&mut actual.parcels);
                if clock.accepted_until() == prepared.support.end_ns()
                    && !pending_terminal_parcels.is_empty()
                {
                    expected_child_beginning = complete_owner_set_digest(clock.owners())?;
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                    let endpoint =
                        owner_joins
                            .last()
                            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                                "parent-end terminal receiver endpoint support",
                            ))?;
                    let terminal_group_index = event_groups.len().checked_sub(1).ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "parent-end terminal receiver event group",
                        ),
                    )?;
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                    let terminal_group = event_groups.get_mut(terminal_group_index).ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "parent-end terminal receiver event group",
                        ),
                    )?;
                    consume_parent_end_terminal_parcels_v1(
                        context,
                        &mut parent,
                        &mut consumer,
                        &mut clock,
                        &stage3,
                        &mut terminal_parcels,
                        &mut pending_terminal_parcels,
                        endpoint,
                        terminal_group,
                    )?;
                    expected_child_beginning = complete_owner_set_digest(clock.owners())?;
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                    if failure_injection
                        == Some(Stage3V11FailureInjection::ParentEndTerminalReceiverCompleted)
                    {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "injected parent-end terminal receiver rollback",
                        ));
                    }
                }
                expected_child_beginning = complete_owner_set_digest(clock.owners())?;
                let remaining_quanta =
                    (candidate_ceiling.get() - clock.accepted_until().get()) / ADAPTIVE_MIN_STEP_NS;
                adaptive_trial_quanta = if maximum_scaled_error < 0.125 {
                    adaptive_test_growth_quanta(candidate_quanta, remaining_quanta)
                } else {
                    candidate_quanta.min(remaining_quanta)
                };
                if !pending_terminal_parcels.is_empty() {
                    interrupt!(
                        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent,
                        None::<&Stage3AdaptiveParentRequestReceiptV1>
                    );
                }
                interrupt!(
                    DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                    None::<&Stage3AdaptiveParentRequestReceiptV1>
                );
                continue;
            }
            if snow_reappearance {
                interrupt!(
                    DirectSnowStage3V11InterruptionPostureV2::BeforeSnowReappearance,
                    Some(&adaptive_request)
                );
            }
            let (accepted, maximum_scaled_error) = match select_adaptive_covered_candidate_v1(
                context,
                &parent,
                &consumer,
                &clock,
                &prepared,
                day_index,
                interval_index,
                forcing_receipt,
                &stage3,
                &pending_terminal_parcels,
                &mut covered_trial_memo,
                &mut adaptive_receipts,
                &adaptive_request,
                support,
                candidate_quanta,
                child_ordinal,
            )? {
                AdaptiveCandidateSelectionV1::Accepted {
                    trial,
                    maximum_scaled_error,
                } => (trial, maximum_scaled_error),
                AdaptiveCandidateSelectionV1::Refine { next_trial_quanta } => {
                    adaptive_trial_quanta = next_trial_quanta;
                    continue;
                }
            };
            for (offset, receipt) in accepted.receipts.iter().enumerate() {
                let accepted_ordinal = owner_joins.len() + offset + 1;
                if failure_injection
                    == Some(Stage3V11FailureInjection::OutcomeLedgerBuilt(
                        accepted_ordinal,
                    ))
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "injected post-outcome-ledger rollback",
                    ));
                }
                if failure_injection
                    == Some(Stage3V11FailureInjection::PrecipitationReceiptRejected(
                        accepted_ordinal,
                    ))
                {
                    return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                        "injected live precipitation-receipt rejection",
                    ));
                }
                if failure_injection
                    == Some(Stage3V11FailureInjection::SnowSoilHeatReceiptRejected(
                        accepted_ordinal,
                    ))
                {
                    return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
                        "injected live snow-soil-receipt rejection",
                    ));
                }
                let expected = if offset == 0 {
                    expected_child_beginning
                } else {
                    accepted.receipts[offset - 1].effective_ending_complete_owner_set_sha256()
                };
                if receipt.owner_join.beginning_complete_owner_set_sha256 != expected {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive accepted child predecessor join",
                    ));
                }
            }
            expected_child_beginning = accepted
                .receipts
                .last()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive accepted receipt cardinality",
                ))?
                .effective_ending_complete_owner_set_sha256();
            if complete_owner_set_digest(accepted.clock.owners())? != expected_child_beginning {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive accepted ending complete-owner clock join",
                ));
            }
            parent = *accepted.parent;
            consumer = *accepted.consumer;
            clock = *accepted.clock;
            stage3 = *accepted.stage3;
            owner_joins.extend(accepted.receipts);
            for accepted_ordinal in 1..=owner_joins.len() {
                if failure_injection
                    == Some(Stage3V11FailureInjection::SubslabAccepted(accepted_ordinal))
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "injected coupled subslab rollback",
                    ));
                }
            }
            let remaining_quanta =
                (candidate_ceiling.get() - clock.accepted_until().get()) / ADAPTIVE_MIN_STEP_NS;
            adaptive_trial_quanta = if maximum_scaled_error < 0.125 {
                adaptive_test_growth_quanta(candidate_quanta, remaining_quanta)
            } else {
                candidate_quanta.min(remaining_quanta)
            };
            if snow_reappearance {
                interrupt!(
                    DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance,
                    None::<&Stage3AdaptiveParentRequestReceiptV1>
                );
            }
            interrupt!(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                None::<&Stage3AdaptiveParentRequestReceiptV1>
            );
        }
        Ok(AdaptiveParentLoopOutcomeV1::Complete(Box::new(
            AdaptiveParentExecutionStateV1 {
                prepared,
                restart,
                parent,
                consumer,
                clock,
                stage3,
                owner_joins,
                event_groups,
                terminal_parcels,
                pending_terminal_parcels,
                expected_child_beginning,
                adaptive_receipts,
                snow_free_successor_receipts,
                adaptive_trial_quanta,
                covered_trial_memo,
            },
        )))
    })? {
        AdaptiveParentLoopOutcomeV1::Paused(outcome) => return Ok(*outcome),
        AdaptiveParentLoopOutcomeV1::Complete(execution) => execution,
    };
    finalize_adaptive_parent_execution_state_v1(
        context,
        failure_injection,
        day_index,
        interval_index,
        parent_telemetry_started,
        execution,
    )
    .map(|outcome| *outcome)
}

#[cfg(test)]
include!("snow_stage3_v11_adaptive_execution_tests.rs");
