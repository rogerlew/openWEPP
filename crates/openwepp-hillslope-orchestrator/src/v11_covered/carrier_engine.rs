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

impl DirectV11SnowCoveredRealConsumerStack<'_> {
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
