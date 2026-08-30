#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3TerminalLiquidCustodyV2 {
    pub schema: String,
    pub terminal_group_receipt_sha256: Digest32,
    pub parent_transaction_sha256: Digest32,
    pub support: TimeSupport,
    pub output_set_sha256: Digest32,
    pub receiver_event: AcceptedEventReceiptV1,
    pub surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
    pub surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
    pub lse_beginning_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    pub lse_ending_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    pub receiver_receipt_set_sha256: Digest32,
    pub receiver_receipts: Vec<crate::DirectZeroDurationSnowLiquidReceiptV1>,
    pub custody_sha256: Digest32,
}

impl Stage3TerminalLiquidCustodyV2 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.custody_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 serialization",
            )
        })?;
        Ok(digest_bytes(&bytes))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn seal(
        group: &Stage3V11TerminalEventGroupV1,
        receiver_event: AcceptedEventReceiptV1,
        output_set_sha256: Digest32,
        surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
        surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
        lse_beginning_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
        lse_ending_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
        receiver_receipt_set_sha256: Digest32,
        receiver_receipts: Vec<crate::DirectZeroDurationSnowLiquidReceiptV1>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let predecessor = group.accepted_event_receipt.as_ref().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 predecessor event",
            ),
        )?;
        let support = group
            .produced_unconsumed_parcels
            .first()
            .map(|parcel| parcel.support)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 parcel set",
            ))?;
        let mut value = Self {
            schema: "openwepp.stage3-terminal-liquid-custody.v2".to_owned(),
            terminal_group_receipt_sha256: group.accepted_group_receipt_sha256.ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal-liquid custody V2 accepted group",
                ),
            )?,
            parent_transaction_sha256: predecessor.parent_transaction_id().digest(),
            support,
            output_set_sha256,
            receiver_event,
            surface_beginning_state,
            surface_ending_state,
            lse_beginning_state,
            lse_ending_state,
            receiver_receipt_set_sha256,
            receiver_receipts,
            custody_sha256: Digest32::zero(),
        };
        value.custody_sha256 = value.reconstructed_digest()?;
        value.validate(group)?;
        Ok(value)
    }

    pub fn validate(
        &self,
        group: &Stage3V11TerminalEventGroupV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let predecessor = group.accepted_event_receipt.as_ref().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 predecessor event",
            ),
        )?;
        predecessor.validate().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 predecessor seal",
            )
        })?;
        self.receiver_event.validate().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 receiver event seal",
            )
        })?;
        let reconstructed_receipt_set =
            crate::zero_duration_snow_liquid_receipt_set_sha256(&self.receiver_receipts)
                .map(Digest32::from_bytes)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "terminal-liquid custody V2 typed receipt set",
                    )
                })?;
        let receiver_ledger = LedgerEntryV1::new(
            "terminal-liquid-receiver".to_owned(),
            "kg-m-2-and-j-m-2-ofe-ground".to_owned(),
            self.output_set_sha256,
            self.output_set_sha256,
            reconstructed_receipt_set,
        )?;
        self.receiver_event
            .validate_ledger_entries(&[receiver_ledger])
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal-liquid custody V2 event/receipt-set ledger join",
                )
            })?;
        let parcel_fields = group
            .produced_unconsumed_parcel_digests
            .iter()
            .map(|digest| FramedField {
                tag: "parcel",
                value: digest.as_bytes(),
            })
            .collect::<Vec<_>>();
        let reconstructed_output_set =
            framed_sha256("stage3-v11-terminal-receiver-parcel-set", &parcel_fields)?;
        if self.schema != "openwepp.stage3-terminal-liquid-custody.v2"
            || self.custody_sha256 == Digest32::zero()
            || self.custody_sha256 != self.reconstructed_digest()?
            || Some(self.terminal_group_receipt_sha256) != group.accepted_group_receipt_sha256
            || self.parent_transaction_sha256 != predecessor.parent_transaction_id().digest()
            || self.output_set_sha256 != reconstructed_output_set
            || self.receiver_receipt_set_sha256 != reconstructed_receipt_set
            || self.receiver_event.parent_transaction_id() != predecessor.parent_transaction_id()
            || self.receiver_event.tick() != group.tick
            || self.receiver_event.beginning_owner_set_digest()
                != predecessor.ending_owner_set_digest()
            || self.receiver_event.ordinal() != predecessor.ordinal().checked_add(1).ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal-liquid custody V2 ordinal overflow",
                ),
            )?
            || group
                .produced_unconsumed_parcels
                .iter()
                .any(|parcel| parcel.support != self.support)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 identity join",
            ));
        }
        let event_context = self.receiver_event.event_context_digest();
        let event_ordinal = self.receiver_event.ordinal();
        if self.receiver_receipts.iter().any(|receipt| {
            receipt.output_set_sha256 != *self.output_set_sha256.as_bytes()
                || receipt.predecessor_owner_set_sha256
                    != *predecessor.ending_owner_set_digest().as_bytes()
                || receipt.receiver_context_sha256 != *event_context.as_bytes()
                || receipt.support_start_ns != self.support.start_ns().get()
                || receipt.support_end_ns != self.support.end_ns().get()
                || receipt.receiver_ordinal != event_ordinal
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 receipt/event join",
            ));
        }
        let mut expected_output_custody = BTreeMap::new();
        for parcel in &group.produced_unconsumed_parcels {
            if expected_output_custody
                .insert(
                    *parcel.parcel_digest.as_bytes(),
                    terminal_destination_output_custody_v1(
                        parcel.receiver_destinations.iter().map(|destination| {
                            (
                                destination.destination_ofe_id.as_str(),
                                destination.destination_tile_id.as_str(),
                                destination.destination_fraction,
                            )
                        }),
                        parcel.mass_kg_m2_tile_ground,
                        parcel.specific_liquid_enthalpy_j_kg,
                    )?,
                )
                .is_some()
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal-liquid custody V2 duplicate parcel output",
                ));
            }
        }
        let first_hop_output_custody =
            reconstruct_first_hop_output_custody_v1(&self.receiver_receipts)?;
        if !exact_first_hop_output_custody_matches_v1(
            &first_hop_output_custody,
            &expected_output_custody,
        ) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal-liquid custody V2 per-output first-hop mass/enthalpy closure",
            ));
        }
        validate_support_liquid_surface_delta_v1(
            &self.surface_beginning_state,
            &self.surface_ending_state,
            &self.receiver_receipts,
        )?;
        validate_support_liquid_lse_delta_v1(
            &self.lse_beginning_state,
            &self.lse_ending_state,
            &self.receiver_receipts,
        )?;
        Ok(())
    }
}

impl Stage3V11TerminalEventGroupV1 {
    #[must_use]
    pub fn terminal_receiver_custody_v2(&self) -> Option<&Stage3TerminalLiquidCustodyV2> {
        self.terminal_receiver_custody_v2.as_ref()
    }

    pub fn install_terminal_receiver_custody_v2(
        &mut self,
        custody: Stage3TerminalLiquidCustodyV2,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.terminal_receiver_custody_v2.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate terminal-liquid custody V2 install",
            ));
        }
        custody.validate(self)?;
        self.terminal_receiver_custody_v2 = Some(custody);
        Ok(())
    }

    pub fn validate_terminal_receiver_custody_v2(
        &self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if let Some(custody) = &self.terminal_receiver_custody_v2 {
            custody.validate(self)?;
        }
        Ok(())
    }
}
