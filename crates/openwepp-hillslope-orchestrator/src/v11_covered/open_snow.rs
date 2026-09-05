// Canonical open-snow production entrypoint.
//
// ADR-0044 requires this entrypoint to delegate exactly once to the canonical
// regime controller. The physical map closures remain named here so the
// production call edge is source-auditable; all state admission, evaluation
// charging, continuous convergence, and final installation belong to the
// controller.

impl<C, V> CanonicalStage3OpenSnowExecutionV1<&V11ImportedV10SegmentInput, C, V>
    for DirectV11SnowCoveredRealConsumerStack<'_>
{
    type Output = V11ImportedV10SegmentOutput;
    type Error = DirectV11RealConsumerError;

    fn execute_canonical_stage3_open_snow(
        &mut self,
        input: &V11ImportedV10SegmentInput,
        _evaluate_covered: C,
        _execute_thin_pack_v22: V,
    ) -> Result<Self::Output, Self::Error> {
        if input.configuration != self.beginning.vegetation_configuration
            || input.beginning != self.beginning.vegetation_state
            || input.duration_s_bits != input.support.duration_s_bits()
            || self.interval.lse_forcing.interval_s.to_bits() != input.duration_s_bits
            || self.snow_surface_forcing_by_destination.is_empty()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered support / DirectV10 beginning join",
            ));
        }

        // These endpoints are authenticated results from this same canonical
        // algorithm on the same physical support. Consuming one transfers its
        // immutable result; it does not evaluate or replay a physical map.
        if self.terminal_physical_reuse_seed.is_some() {
            return self.execute_terminal_physical_reuse(input);
        }
        if self.ordinary_physical_reuse_seed.is_some() {
            return self.execute_ordinary_physical_reuse(input);
        }
        if let Some(endpoint) = self.precomputed_terminal_accepted.take() {
            return self.execute_precomputed_terminal_accepted_endpoint(input, endpoint);
        }

        self.execute_canonical_covered_production_v1(input)
    }
}

impl crate::v11_vegetation_consumer::DirectV11ImportedStack
    for DirectV11SnowCoveredRealConsumerStack<'_>
{
    type Error = DirectV11RealConsumerError;

    fn execute_imported_v10_stack(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error> {
        let evaluate_stage3 = || ();
        let open_snow_boundaries_by_destination = || ();
        let result = canonical_stage3_open_snow_execute_v1(
            self,
            input,
            evaluate_stage3,
            open_snow_boundaries_by_destination,
        );
        return result;
    }
}

#[cfg(test)]
include!("open_snow_tail_tests.rs");
