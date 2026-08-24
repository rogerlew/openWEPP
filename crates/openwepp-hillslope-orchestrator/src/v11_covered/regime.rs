impl<'a> DirectV11RealConsumerStack<'a> {
    #[must_use]
    pub fn new(
        beginning: &DirectV10RealConsumerShadow,
        interval: &'a DirectV9ShadowIntervalInput,
        day_index: usize,
        interval_index: usize,
    ) -> Self {
        Self {
            beginning: beginning.clone(),
            interval,
            day_index,
            interval_index,
            ending: None,
            last_support_receipt: None,
            #[cfg(test)]
            last_hydrology_candidate: None,
            ending_snow_owner_bytes: None,
        }
    }

    /// Bind the Stage-3 state that the shared parent transaction has already
    /// staged as the sole ending snow owner. This constructor remains the
    /// snow-free lower-boundary executor; it does not admit snow forcing.
    #[must_use]
    pub fn new_with_ending_snow_owner(
        beginning: &DirectV10RealConsumerShadow,
        interval: &'a DirectV9ShadowIntervalInput,
        day_index: usize,
        interval_index: usize,
        ending_snow_owner_bytes: Vec<u8>,
    ) -> Self {
        let mut value = Self::new(beginning, interval, day_index, interval_index);
        value.ending_snow_owner_bytes = Some(ending_snow_owner_bytes);
        value
    }

    /// Consume the isolated staged ending only after the V11 parent accepts
    /// the corresponding segment candidate.
    pub fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        self.ending.take()
    }

    #[must_use]
    pub fn last_support_receipt(&self) -> Option<&LseSupportAdmissibilityReceiptV1> {
        self.last_support_receipt.as_ref()
    }

    #[must_use]
    #[cfg(test)]
    pub fn last_hydrology_candidate(
        &self,
    ) -> Option<&crate::land_surface_energy_shadow::UnifiedRealHydrologyCandidate> {
        self.last_hydrology_candidate.as_ref()
    }
}
