// Small V10-stack identity and provider-owner accessors used by both the
// persistent day executor and the covered terminal probe. Keeping these
// lifecycle helpers separate prevents the real-consumer implementation from
// becoming the owner of terminal chronology details.

impl DirectV10RealConsumerShadow {
    pub(crate) fn next_lse_transaction_id(
        &self,
    ) -> Result<TransactionId, DirectV10RealConsumerError> {
        Ok(TransactionId(
            self.inner
                .vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or_else(|| {
                    DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                        "next LSE transaction overflow",
                    ))
                })?,
        ))
    }

    #[must_use]
    pub const fn v11_next_day_index(&self) -> usize {
        self.inner.next_day_index()
    }

    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        self.inner.hydrology_frame()
    }

    /// Install the provider/GSI owner transition only on a cloned candidate
    /// after all coupled Stage-3/V11 supports have accepted. This keeps the
    /// runner cursor out of the live state on any failed support.
    pub(crate) fn commit_prepared_provider_day(
        &mut self,
        prepared: PreparedSnowFreeGsiDayV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        let accepted_receipt = prepared.gsi_receipt().receipt_sha256.clone();
        prepared
            .commit(&mut self.gsi_state, &mut self.provider_cursor)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::ForcingProvider(
                    error,
                ))
            })?;
        self.inner.provider_gsi_receipt_sha256 = accepted_receipt;
        Ok(())
    }
}
