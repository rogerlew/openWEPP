// Shared value-only construction primitives for the covered carrier engine.

struct CoveredCarrierEnvelopeBuildV1<'a> {
    candidate: &'a DirectV10RealConsumerShadow,
    interval_s: f64,
    duration_s_bits: u64,
    covered_destinations: &'a BTreeSet<(OfeId, TileId)>,
    covered_boundaries: &'a BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    open_boundaries: &'a BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    provisional: bool,
    finalize_wb14_parent_interval: bool,
}

#[derive(Clone)]
pub(crate) struct AcceptedCoveredCarrierEvidenceV1 {
    pub final_boundaries: BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    pub final_lanes: BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    pub component_receipts:
        BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
    pub wb14_child_receipt_set_sha256: String,
    pub wb14_parent_receipt_set_sha256: Option<String>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    /// Seal accepted carrier evidence solely from the winning exact carrier
    /// values. This function never evaluates Stage 3 or LSE and never writes
    /// publication fields on the owning stack.
    pub(crate) fn seal_accepted_carrier_evidence_v1(
        &self,
        phase: &CoveredCarrierPhaseResultV1,
        input: &V11ImportedV10SegmentInput,
        ending_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<AcceptedCoveredCarrierEvidenceV1, DirectV11RealConsumerError> {
        if input.support != phase.transition.boundary.support
            || input.accepted_slab_receipt.slab_ordinal()
                != phase.transition.probe_child_identity.physical_child_ordinal
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted carrier slab/probe identity",
            ));
        }
        let ending_v8_sha256 = digest32_from_lower_hex(
            &phase
                .carrier_envelope
                .vegetation()
                .ending_state()
                .state_sha256,
        )?;
        let ending_snow_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(ending_stage3)?);
        let (_covered_boundaries, covered_receipts) = self.seal_final_covered_boundaries(
            input,
            &phase.complete_lower_boundaries,
            &phase.carrier_source_receipts,
            &phase.carrier_envelope,
            ending_v8_sha256,
            ending_snow_sha256,
        )?;
        let (_open_boundaries, open_receipts) =
            self.seal_final_open_snow_boundaries(ending_stage3, ending_snow_sha256)?;
        let final_boundaries =
            self.complete_final_boundary_receipts(covered_receipts, open_receipts)?;
        let final_lanes =
            self.final_lane_boundary_receipts(input, &final_boundaries, &phase.precipitation_sets)?;
        let component_receipts = phase
            .covered_lse_states
            .iter()
            .map(|(destination, state)| {
                let boundary = final_boundaries
                    .get(destination)
                    .and_then(|value| match value {
                        FinalStage3TileBoundaryReceiptV1::V11Canopy(value) => Some(value),
                        FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => None,
                    })
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "accepted component carrier destination",
                    ))?;
                Ok((
                    destination.clone(),
                    ComponentResolvedCarrierReceiptV1::try_new(
                        destination.clone(),
                        state,
                        boundary,
                    )?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        Ok(AcceptedCoveredCarrierEvidenceV1 {
            final_boundaries,
            final_lanes,
            component_receipts,
            wb14_child_receipt_set_sha256: phase.wb14_child_receipt_set_sha256.clone(),
            wb14_parent_receipt_set_sha256: phase.wb14_parent_receipt_set_sha256.clone(),
            wb14_child_replay_bytes: phase.wb14_child_replay_bytes.clone(),
            wb14_parent_replay_bytes: phase.wb14_parent_replay_bytes.clone(),
        })
    }

    /// Build one unpublished carrier envelope from a complete, disjoint
    /// covered/open destination set. The returned value is not adopted.
    fn build_covered_carrier_envelope_value_v1(
        &self,
        inputs: CoveredCarrierEnvelopeBuildV1<'_>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV11RealConsumerError> {
        let mut complete = inputs.covered_boundaries.clone();
        if inputs.provisional {
            for boundary in complete.values_mut() {
                boundary.optical_receipt_sha256 = None;
                boundary.reciprocal_longwave_receipt_sha256 = None;
                boundary.final_canopy_boundary_receipt_sha256 = None;
            }
        }
        for (destination, boundary) in inputs.open_boundaries {
            let mut boundary = boundary.clone();
            if inputs.provisional {
                boundary.optical_receipt_sha256 = None;
                boundary.reciprocal_longwave_receipt_sha256 = None;
                boundary.final_canopy_boundary_receipt_sha256 = None;
            }
            if complete
                .insert(destination.clone(), boundary)
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered/open destination forcing intersection",
                ));
            }
        }
        if inputs.provisional {
            for boundary in complete.values_mut() {
                boundary.optical_receipt_sha256 = None;
                boundary.reciprocal_longwave_receipt_sha256 = None;
                boundary.final_canopy_boundary_receipt_sha256 = None;
            }
        }
        inputs
            .candidate
            .inner
            .construct_covered_interval_envelope_with_duration(
                self.day_index,
                self.interval_index,
                self.interval,
                inputs.interval_s,
                inputs.duration_s_bits,
                inputs.covered_destinations,
                &complete,
                inputs.provisional,
                inputs.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })
    }

    /// Rebuild the component-resolved LSE carrier from one immutable envelope.
    fn rebuild_covered_lse_carrier_value_v1(
        &self,
        source: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
        ),
        DirectV11RealConsumerError,
    > {
        let (corrected, _, _) =
            self.corrected_covered_boundaries_from_envelope(source, envelope)?;
        let lse = envelope
            .covered_lse_iteration_state_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier LSE state"))?;
        let rebuilt = self.apply_lse_iteration_exchange(&corrected, &lse)?;
        let mut rebuilt = self.merge_latest_stage3_state_operands(&rebuilt, stage3_states)?;
        snow_boundary_state.apply_to_boundaries(
            &self.beginning.inner.surface_configuration.ofe_bindings,
            &mut rebuilt,
        )?;
        Ok((rebuilt, lse))
    }
}

#[cfg(test)]
mod accepted_carrier_evidence_tests {
    #[test]
    fn accepted_sealer_is_value_only_and_does_not_publish() {
        let source = include_str!("carrier_engine.rs");
        let body = source
            .split("pub(crate) fn seal_accepted_carrier_evidence_v1")
            .nth(1)
            .expect("accepted sealer")
            .split("fn build_covered_carrier_envelope_value_v1")
            .next()
            .expect("accepted sealer body");
        for forbidden in [
            "evaluate_stage3_persistent_support",
            "evaluate_stage3_terminal_support",
            "construct_covered_interval_envelope_with_duration(\n                self.day_index",
            "last_final_boundary_receipts =",
            "last_wb14_",
            "self.ending =",
        ] {
            assert!(!body.contains(forbidden), "accepted sealer used {forbidden}");
        }
    }

    #[test]
    fn provisional_envelope_clears_all_accepted_boundary_identities() {
        let source = include_str!("carrier_engine.rs");
        assert!(source.contains("boundary.optical_receipt_sha256 = None"));
        assert!(source.contains("boundary.reciprocal_longwave_receipt_sha256 = None"));
        assert!(source.contains("boundary.final_canopy_boundary_receipt_sha256 = None"));
    }
}
