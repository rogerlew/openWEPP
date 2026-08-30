#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DirectZeroDurationSnowLiquidInputV1 {
    pub output_receipt_sha256: [u8; 32],
    pub output_set_sha256: [u8; 32],
    pub predecessor_owner_set_sha256: [u8; 32],
    pub receiver_context_sha256: [u8; 32],
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub receiver_ordinal: u32,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub mass_kg_m2_tile_ground: f64,
    pub sensible_enthalpy_j_m2_tile_ground: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectZeroDurationSnowLiquidDispositionV1 {
    RetainedSurface,
    RoutedRunoff,
    OutletRunoff,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectZeroDurationSnowLiquidReceiptV1 {
    pub output_receipt_sha256: [u8; 32],
    pub output_set_sha256: [u8; 32],
    pub predecessor_owner_set_sha256: [u8; 32],
    pub receiver_context_sha256: [u8; 32],
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub receiver_ordinal: u32,
    pub transaction_id: TransactionId,
    pub origin_ofe_id: OfeId,
    pub origin_tile_id: TileId,
    pub basis_ofe_id: OfeId,
    pub recipient_ofe_id: OfeId,
    pub recipient_tile_id: Option<TileId>,
    pub recipient_tile_fraction: Option<f64>,
    pub disposition: DirectZeroDurationSnowLiquidDispositionV1,
    pub mass_kg_m2_basis_ofe_ground: f64,
    pub sensible_enthalpy_j_m2_basis_ofe_ground: f64,
    pub credited_mass_kg_m2_recipient_tile_ground: Option<f64>,
    pub credited_enthalpy_j_m2_recipient_tile_ground: Option<f64>,
    pub predecessor_receipt_sha256: [u8; 32],
    pub receipt_sha256: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DirectZeroDurationSnowLiquidEnthalpyCreditV1 {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub enthalpy_j_m2_tile_ground: f64,
    pub receipt_sha256: [u8; 32],
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectZeroDurationSnowLiquidOutcomeV1 {
    pub ending_state: DirectSurfaceLiquidOwnedState,
    pub receipts: Vec<DirectZeroDurationSnowLiquidReceiptV1>,
    pub retained_enthalpy_credits: Vec<DirectZeroDurationSnowLiquidEnthalpyCreditV1>,
    pub receipt_set_sha256: [u8; 32],
}

#[derive(Clone)]
struct PendingZeroDurationSnowLiquidV1 {
    input: DirectZeroDurationSnowLiquidInputV1,
    basis_ofe_id: OfeId,
    destination_tile_id: TileId,
    mass_kg_m2_basis_ofe_ground: f64,
    enthalpy_j_m2_basis_ofe_ground: f64,
}

fn zero_duration_snow_digest(parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Sha256::new();
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.finalize().into()
}

fn zero_duration_snow_receipt_digest(
    receipt: &DirectZeroDurationSnowLiquidReceiptV1,
) -> [u8; 32] {
    let mut canonical = receipt.clone();
    canonical.receipt_sha256 = [0; 32];
    serde_json::to_vec(&canonical)
        .map_or([0; 32], |bytes| zero_duration_snow_digest(&[&bytes]))
}

impl DirectZeroDurationSnowLiquidReceiptV1 {
    pub fn validate_seal(&self) -> Result<(), DirectSurfaceLiquidError> {
        let recipient_is_valid = match self.disposition {
            DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            => {
                self.recipient_tile_id.is_some()
                    && self.recipient_tile_fraction.is_some_and(|fraction| {
                        fraction.is_finite() && fraction > 0.0 && fraction <= 1.0
                    })
                    && self
                        .credited_mass_kg_m2_recipient_tile_ground
                        .is_some_and(|mass| mass.is_finite() && mass > 0.0)
                    && self
                        .credited_enthalpy_j_m2_recipient_tile_ground
                        .is_some_and(|enthalpy| enthalpy.is_finite() && enthalpy >= 0.0)
            }
            DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff => {
                self.recipient_tile_id.is_some()
                    && self.recipient_tile_fraction.is_some_and(|fraction| {
                        fraction.is_finite() && fraction > 0.0 && fraction <= 1.0
                    })
                    && self.credited_mass_kg_m2_recipient_tile_ground.is_none()
                    && self.credited_enthalpy_j_m2_recipient_tile_ground.is_none()
            }
            DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff => {
                self.recipient_tile_id.is_none()
                    && self.recipient_tile_fraction.is_none()
                    && self.credited_mass_kg_m2_recipient_tile_ground.is_none()
                    && self.credited_enthalpy_j_m2_recipient_tile_ground.is_none()
            }
        };
        if self.output_receipt_sha256 == [0; 32]
            || self.output_set_sha256 == [0; 32]
            || self.predecessor_owner_set_sha256 == [0; 32]
            || self.receiver_context_sha256 == [0; 32]
            || self.support_end_ns <= self.support_start_ns
            || self.mass_kg_m2_basis_ofe_ground <= 0.0
            || !self.mass_kg_m2_basis_ofe_ground.is_finite()
            || self.sensible_enthalpy_j_m2_basis_ofe_ground < 0.0
            || !self.sensible_enthalpy_j_m2_basis_ofe_ground.is_finite()
            || self.receipt_sha256 == [0; 32]
            || self.receipt_sha256 != zero_duration_snow_receipt_digest(self)
            || !recipient_is_valid
        {
            return Err(DirectSurfaceLiquidError::Closure(
                "zero-duration snow-liquid receipt seal",
            ));
        }
        Ok(())
    }
}

pub fn zero_duration_snow_liquid_receipt_set_sha256(
    receipts: &[DirectZeroDurationSnowLiquidReceiptV1],
) -> Result<[u8; 32], DirectSurfaceLiquidError> {
    if receipts.is_empty() {
        return Err(DirectSurfaceLiquidError::Identity(
            "zero-duration snow-liquid receipt set",
        ));
    }
    let mut predecessor = [0; 32];
    for receipt in receipts {
        receipt.validate_seal()?;
        if receipt.predecessor_receipt_sha256 != predecessor {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid receipt predecessor chain",
            ));
        }
        predecessor = receipt.receipt_sha256;
    }
    Ok(zero_duration_snow_digest(
        &receipts
            .iter()
            .map(|receipt| receipt.receipt_sha256.as_slice())
            .collect::<Vec<_>>(),
    ))
}

pub(crate) fn validate_zero_duration_snow_liquid_surface_delta_v1(
    beginning: &DirectSurfaceLiquidOwnedState,
    ending: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectZeroDurationSnowLiquidReceiptV1],
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction = receipts
        .first()
        .map(|receipt| receipt.transaction_id)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "empty zero-duration snow-liquid surface delta receipt set",
        ))?;
    if receipts
        .iter()
        .any(|receipt| receipt.transaction_id != transaction)
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "zero-duration snow-liquid surface delta transaction substitution",
        ));
    }
    let mut expected = beginning.clone();
    for receipt in receipts {
        receipt.validate_seal()?;
        if receipt.disposition != DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface {
            continue;
        }
        let tile_id = receipt
            .recipient_tile_id
            .as_ref()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid surface delta retained tile",
            ))?;
        let credit = receipt
            .credited_mass_kg_m2_recipient_tile_ground
            .ok_or(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid surface delta retained credit",
            ))?;
        let mut matches = expected.records.iter_mut().filter(|record| {
            record.key.ofe_id == receipt.recipient_ofe_id && &record.key.tile_id == tile_id
        });
        let record = matches.next().ok_or(DirectSurfaceLiquidError::Identity(
            "zero-duration snow-liquid surface delta retained topology",
        ))?;
        if matches.next().is_some() {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid surface delta duplicate topology",
            ));
        }
        record.liquid_kg_m2_tile = checked_surface_liquid_add(
            record.liquid_kg_m2_tile,
            credit,
        )
        .ok_or(DirectSurfaceLiquidError::Domain(
            "zero-duration snow-liquid surface delta retained addition",
        ))?;
    }

    let lineage_advanced = ending
        .records
        .iter()
        .zip(&beginning.records)
        .any(|(after, before)| {
            after.last_accepted_transaction_id != before.last_accepted_transaction_id
        })
        || ending
            .continuations
            .iter()
            .zip(&beginning.continuations)
            .any(|(after, before)| {
                after.last_accepted_transaction_id != before.last_accepted_transaction_id
            });
    if lineage_advanced {
        for record in &mut expected.records {
            record.last_accepted_transaction_id = Some(transaction);
        }
        for continuation in &mut expected.continuations {
            continuation.last_accepted_transaction_id = Some(transaction);
        }
    }
    expected.state_sha256 = expected.recomputed_sha256()?;
    if &expected != ending {
        return Err(DirectSurfaceLiquidError::Identity(
            "zero-duration snow-liquid surface delta storage/lineage substitution",
        ));
    }
    Ok(())
}

fn zero_duration_snow_failure(detail: &'static str) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::Closure(detail)
}

impl DirectSurfaceLiquidOwnedState {
    /// Consume an already-accepted positive-support snow-liquid output at a
    /// zero-duration ownership boundary. This performs no infiltration and
    /// does not advance WB14; it applies canonical capacity retention and
    /// topology routing atomically from the immutable beginning owner.
    pub(crate) fn accept_zero_duration_snow_liquid_outputs_v1(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        inputs: &[DirectZeroDurationSnowLiquidInputV1],
        advance_persistent_lineage: bool,
    ) -> Result<DirectZeroDurationSnowLiquidOutcomeV1, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        if inputs.is_empty() {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid input set",
            ));
        }
        let predecessor = self.accepted_transaction()?;
        let transaction_id = TransactionId(
            predecessor
                .map_or(0, |value| value.0)
                .checked_add(1)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid transaction overflow",
                ))?,
        );
        let first = &inputs[0];
        if first.output_set_sha256 == [0; 32]
            || first.predecessor_owner_set_sha256 == [0; 32]
            || first.receiver_context_sha256 == [0; 32]
            || first.support_end_ns <= first.support_start_ns
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid receiver binding",
            ));
        }
        let mut seen = BTreeSet::new();
        let mut output_authority = BTreeMap::<
            [u8; 32],
            (OfeId, f64, f64, BTreeSet<TileId>, f64),
        >::new();
        let mut pending = BTreeMap::<OfeId, Vec<PendingZeroDurationSnowLiquidV1>>::new();
        for input in inputs {
            if input.output_receipt_sha256 == [0; 32]
                || input.output_set_sha256 != first.output_set_sha256
                || input.predecessor_owner_set_sha256 != first.predecessor_owner_set_sha256
                || input.receiver_context_sha256 != first.receiver_context_sha256
                || input.support_start_ns != first.support_start_ns
                || input.support_end_ns != first.support_end_ns
                || input.receiver_ordinal != first.receiver_ordinal
                || !seen.insert((
                    input.output_receipt_sha256,
                    input.ofe_id.clone(),
                    input.tile_id.clone(),
                ))
                || !input.tile_fraction.is_finite()
                || input.tile_fraction <= 0.0
                || !input.mass_kg_m2_tile_ground.is_finite()
                || input.mass_kg_m2_tile_ground <= 0.0
                || !input.sensible_enthalpy_j_m2_tile_ground.is_finite()
                || input.sensible_enthalpy_j_m2_tile_ground < 0.0
            {
                return Err(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid input identity/domain",
                ));
            }
            let configured = configuration
                .records
                .iter()
                .find(|record| {
                    record.key.ofe_id == input.ofe_id && record.key.tile_id == input.tile_id
                })
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid destination topology",
                ))?;
            if configured.tile_fraction.to_bits() != input.tile_fraction.to_bits() {
                return Err(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid destination fraction",
                ));
            }
            let authority = output_authority
                .entry(input.output_receipt_sha256)
                .or_insert_with(|| {
                    (
                        input.ofe_id.clone(),
                        input.mass_kg_m2_tile_ground,
                        input.sensible_enthalpy_j_m2_tile_ground,
                        BTreeSet::new(),
                        0.0,
                    )
                });
            if authority.0 != input.ofe_id
                || authority.1.to_bits() != input.mass_kg_m2_tile_ground.to_bits()
                || authority.2.to_bits()
                    != input.sensible_enthalpy_j_m2_tile_ground.to_bits()
                || !authority.3.insert(input.tile_id.clone())
            {
                return Err(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid sealed destination authority",
                ));
            }
            authority.4 = checked_surface_liquid_add(authority.4, input.tile_fraction).ok_or(
                DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid destination fraction sum",
                ),
            )?;
            let mass = checked_surface_liquid_mul(
                input.tile_fraction,
                input.mass_kg_m2_tile_ground,
            )
            .ok_or(DirectSurfaceLiquidError::Domain(
                "zero-duration snow-liquid OFE mass conversion",
            ))?;
            let enthalpy = checked_surface_liquid_mul(
                input.tile_fraction,
                input.sensible_enthalpy_j_m2_tile_ground,
            )
            .ok_or(DirectSurfaceLiquidError::Domain(
                "zero-duration snow-liquid OFE enthalpy conversion",
            ))?;
            pending
                .entry(input.ofe_id.clone())
                .or_default()
                .push(PendingZeroDurationSnowLiquidV1 {
                    input: input.clone(),
                    basis_ofe_id: input.ofe_id.clone(),
                    destination_tile_id: input.tile_id.clone(),
                    mass_kg_m2_basis_ofe_ground: mass,
                    enthalpy_j_m2_basis_ofe_ground: enthalpy,
                });
        }
        for (_, (ofe_id, _, _, tiles, fraction_sum)) in output_authority {
            let configured_tiles = configuration
                .records
                .iter()
                .filter(|record| record.key.ofe_id == ofe_id)
                .map(|record| record.key.tile_id.clone())
                .collect::<BTreeSet<_>>();
            if tiles != configured_tiles || (fraction_sum - 1.0).abs() > 1.0e-12 {
                return Err(DirectSurfaceLiquidError::Identity(
                    "zero-duration snow-liquid complete destination partition",
                ));
            }
        }
        let mut ending = self.clone();
        let mut receipts = Vec::new();
        let mut credits = BTreeMap::<(OfeId, TileId), (f64, Vec<[u8; 32]>)>::new();
        let mut chain = [0; 32];
        for ofe_id in &configuration.ofe_topology {
            let mut ofe_pending = pending.remove(ofe_id).unwrap_or_default();
            ofe_pending.sort_by(|left, right| {
                (&left.destination_tile_id, left.input.output_receipt_sha256)
                    .cmp(&(&right.destination_tile_id, right.input.output_receipt_sha256))
            });
            let mut grouped = BTreeMap::<TileId, Vec<PendingZeroDurationSnowLiquidV1>>::new();
            for parcel in ofe_pending {
                grouped
                    .entry(parcel.destination_tile_id.clone())
                    .or_default()
                    .push(parcel);
            }
            for (tile_id, parcels) in grouped {
                let config_index = configuration
                    .records
                    .iter()
                    .position(|record| record.key.ofe_id == *ofe_id && record.key.tile_id == tile_id)
                    .ok_or(DirectSurfaceLiquidError::Identity(
                        "zero-duration snow-liquid routed destination",
                    ))?;
                let configured = &configuration.records[config_index];
                let state = &mut ending.records[config_index];
                let available_tile = checked_surface_liquid_sub(
                    configured.capacity_kg_m2_tile,
                    state.liquid_kg_m2_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid capacity difference",
                ))?;
                let available = checked_surface_liquid_mul(
                    configured.tile_fraction,
                    available_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid capacity conversion",
                ))?;
                if available < 0.0 {
                    return Err(DirectSurfaceLiquidError::Bound(
                        "zero-duration snow-liquid negative capacity",
                    ));
                }
                let total = checked_surface_liquid_sum(
                    parcels.iter().map(|parcel| parcel.mass_kg_m2_basis_ofe_ground),
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid mass sum",
                ))?;
                let capacity_ofe = checked_surface_liquid_mul(
                    configured.tile_fraction,
                    configured.capacity_kg_m2_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid capacity envelope",
                ))?;
                let stored_ofe = checked_surface_liquid_mul(
                    configured.tile_fraction,
                    state.liquid_kg_m2_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid storage envelope",
                ))?;
                let retained_total = super::surface_liquid_ingress::effective_retained_mass(
                    total.min(available),
                    capacity_ofe,
                    stored_ofe,
                    total,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid representational envelope",
                ))?;
                let mut allocated = 0.0;
                let mut allocated_tile = 0.0;
                for (index, parcel) in parcels.iter().enumerate() {
                    let retained = super::surface_liquid_ingress::allocate_retained_mass(
                        retained_total,
                        total,
                        allocated,
                        parcel.mass_kg_m2_basis_ofe_ground,
                        index + 1 == parcels.len(),
                    )
                    .ok_or(zero_duration_snow_failure(
                        "zero-duration snow-liquid retained allocation",
                    ))?;
                    allocated = checked_surface_liquid_add(allocated, retained).ok_or(
                        DirectSurfaceLiquidError::Domain(
                            "zero-duration snow-liquid retained accumulation",
                        ),
                    )?;
                    let runoff = checked_surface_liquid_sub(
                        parcel.mass_kg_m2_basis_ofe_ground,
                        retained,
                    )
                    .ok_or(zero_duration_snow_failure(
                        "zero-duration snow-liquid runoff remainder",
                    ))?;
                    let retained_enthalpy = if runoff.to_bits() == 0.0_f64.to_bits() {
                        parcel.enthalpy_j_m2_basis_ofe_ground
                    } else if retained.to_bits() == 0.0_f64.to_bits() {
                        0.0
                    } else {
                        checked_surface_liquid_mul(
                            parcel.enthalpy_j_m2_basis_ofe_ground,
                            retained,
                        )
                        .and_then(|value| {
                            checked_surface_liquid_div(
                                value,
                                parcel.mass_kg_m2_basis_ofe_ground,
                            )
                        })
                        .ok_or(DirectSurfaceLiquidError::Domain(
                            "zero-duration snow-liquid retained enthalpy",
                        ))?
                    };
                    let runoff_enthalpy = checked_surface_liquid_sub(
                        parcel.enthalpy_j_m2_basis_ofe_ground,
                        retained_enthalpy,
                    )
                    .ok_or(zero_duration_snow_failure(
                        "zero-duration snow-liquid runoff enthalpy",
                    ))?;
                    if retained > 0.0 {
                        let credit_mass_tile = if retained_total.to_bits() == available.to_bits()
                            && allocated.to_bits() == retained_total.to_bits()
                        {
                            checked_surface_liquid_sub(available_tile, allocated_tile).ok_or(
                                zero_duration_snow_failure(
                                    "zero-duration snow-liquid retained tile remainder",
                                ),
                            )?
                        } else {
                            checked_surface_liquid_div(retained, configured.tile_fraction).ok_or(
                                DirectSurfaceLiquidError::Domain(
                                    "zero-duration snow-liquid retained mass tile conversion",
                                ),
                            )?
                        };
                        allocated_tile = checked_surface_liquid_add(
                            allocated_tile,
                            credit_mass_tile,
                        )
                        .ok_or(DirectSurfaceLiquidError::Domain(
                            "zero-duration snow-liquid retained tile accumulation",
                        ))?;
                        let credit_tile = checked_surface_liquid_div(
                            retained_enthalpy,
                            configured.tile_fraction,
                        )
                        .ok_or(DirectSurfaceLiquidError::Domain(
                            "zero-duration snow-liquid retained enthalpy tile conversion",
                        ))?;
                        let credit = credits
                            .entry((ofe_id.clone(), tile_id.clone()))
                            .or_insert_with(|| (0.0, Vec::new()));
                        credit.0 = checked_surface_liquid_add(credit.0, credit_tile).ok_or(
                            DirectSurfaceLiquidError::Domain(
                                "zero-duration snow-liquid tile enthalpy accumulation",
                            ),
                        )?;
                        let mut receipt = DirectZeroDurationSnowLiquidReceiptV1 {
                            output_receipt_sha256: parcel.input.output_receipt_sha256,
                            output_set_sha256: first.output_set_sha256,
                            predecessor_owner_set_sha256: first.predecessor_owner_set_sha256,
                            receiver_context_sha256: first.receiver_context_sha256,
                            support_start_ns: first.support_start_ns,
                            support_end_ns: first.support_end_ns,
                            receiver_ordinal: first.receiver_ordinal,
                            transaction_id,
                            origin_ofe_id: parcel.input.ofe_id.clone(),
                            origin_tile_id: parcel.input.tile_id.clone(),
                            basis_ofe_id: parcel.basis_ofe_id.clone(),
                            recipient_ofe_id: ofe_id.clone(),
                            recipient_tile_id: Some(tile_id.clone()),
                            recipient_tile_fraction: Some(configured.tile_fraction),
                            disposition: DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface,
                            mass_kg_m2_basis_ofe_ground: retained,
                            sensible_enthalpy_j_m2_basis_ofe_ground: retained_enthalpy,
                            credited_mass_kg_m2_recipient_tile_ground: Some(credit_mass_tile),
                            credited_enthalpy_j_m2_recipient_tile_ground: Some(credit_tile),
                            predecessor_receipt_sha256: chain,
                            receipt_sha256: [0; 32],
                        };
                        receipt.receipt_sha256 = zero_duration_snow_receipt_digest(&receipt);
                        if receipt.receipt_sha256 == [0; 32] {
                            return Err(DirectSurfaceLiquidError::Schema(
                                "zero-duration snow-liquid receipt serialization",
                            ));
                        }
                        chain = receipt.receipt_sha256;
                        credit.1.push(chain);
                        receipts.push(receipt);
                    }
                    if runoff > 0.0 {
                        let route = configured;
                        let (
                            disposition,
                            recipient_ofe_id,
                            recipient_tile_id,
                            recipient_tile_fraction,
                        ) = match (
                            &route.runon_destination_ofe_id,
                            &route.runon_destination_tile_id,
                        ) {
                            (Some(destination_ofe), Some(destination_tile)) => {
                                let source_rank = configuration
                                    .ofe_topology
                                    .iter()
                                    .position(|value| value == ofe_id);
                                let destination_rank = configuration
                                    .ofe_topology
                                    .iter()
                                    .position(|value| value == destination_ofe);
                                if source_rank.zip(destination_rank).is_none_or(|(source, destination)| destination <= source) {
                                    return Err(DirectSurfaceLiquidError::Identity(
                                        "zero-duration snow-liquid non-forward route",
                                    ));
                                }
                                let destination = configuration.records.iter().find(|record| {
                                    record.key.ofe_id == *destination_ofe
                                        && record.key.tile_id == *destination_tile
                                }).ok_or(DirectSurfaceLiquidError::Identity(
                                    "zero-duration snow-liquid route destination",
                                ))?;
                                let ratio = checked_surface_liquid_div(
                                    route.ofe_area_m2,
                                    destination.ofe_area_m2,
                                ).ok_or(DirectSurfaceLiquidError::Domain(
                                    "zero-duration snow-liquid route area ratio",
                                ))?;
                                let routed_mass = checked_surface_liquid_mul(runoff, ratio)
                                    .ok_or(DirectSurfaceLiquidError::Domain(
                                        "zero-duration snow-liquid routed mass",
                                    ))?;
                                let routed_enthalpy = checked_surface_liquid_mul(runoff_enthalpy, ratio)
                                    .ok_or(DirectSurfaceLiquidError::Domain(
                                        "zero-duration snow-liquid routed enthalpy",
                                    ))?;
                                pending.entry(destination_ofe.clone()).or_default().push(
                                    PendingZeroDurationSnowLiquidV1 {
                                        input: parcel.input.clone(),
                                        basis_ofe_id: destination_ofe.clone(),
                                        destination_tile_id: destination_tile.clone(),
                                        mass_kg_m2_basis_ofe_ground: routed_mass,
                                        enthalpy_j_m2_basis_ofe_ground: routed_enthalpy,
                                    },
                                );
                                (
                                    DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff,
                                    destination_ofe.clone(),
                                    Some(destination_tile.clone()),
                                    Some(destination.tile_fraction),
                                )
                            }
                            (None, None) => (
                                DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff,
                                ofe_id.clone(),
                                None,
                                None,
                            ),
                            _ => return Err(DirectSurfaceLiquidError::Identity(
                                "zero-duration snow-liquid incomplete route",
                            )),
                        };
                        let mut receipt = DirectZeroDurationSnowLiquidReceiptV1 {
                            output_receipt_sha256: parcel.input.output_receipt_sha256,
                            output_set_sha256: first.output_set_sha256,
                            predecessor_owner_set_sha256: first.predecessor_owner_set_sha256,
                            receiver_context_sha256: first.receiver_context_sha256,
                            support_start_ns: first.support_start_ns,
                            support_end_ns: first.support_end_ns,
                            receiver_ordinal: first.receiver_ordinal,
                            transaction_id,
                            origin_ofe_id: parcel.input.ofe_id.clone(),
                            origin_tile_id: parcel.input.tile_id.clone(),
                            basis_ofe_id: parcel.basis_ofe_id.clone(),
                            recipient_ofe_id,
                            recipient_tile_id,
                            recipient_tile_fraction,
                            disposition,
                            mass_kg_m2_basis_ofe_ground: runoff,
                            sensible_enthalpy_j_m2_basis_ofe_ground: runoff_enthalpy,
                            credited_mass_kg_m2_recipient_tile_ground: None,
                            credited_enthalpy_j_m2_recipient_tile_ground: None,
                            predecessor_receipt_sha256: chain,
                            receipt_sha256: [0; 32],
                        };
                        receipt.receipt_sha256 = zero_duration_snow_receipt_digest(&receipt);
                        if receipt.receipt_sha256 == [0; 32] {
                            return Err(DirectSurfaceLiquidError::Schema(
                                "zero-duration snow-liquid receipt serialization",
                            ));
                        }
                        chain = receipt.receipt_sha256;
                        receipts.push(receipt);
                    }
                }
                state.liquid_kg_m2_tile = checked_surface_liquid_add(
                    state.liquid_kg_m2_tile,
                    allocated_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "zero-duration snow-liquid retained storage",
                ))?;
                if state.liquid_kg_m2_tile > configured.capacity_kg_m2_tile {
                    return Err(zero_duration_snow_failure(
                        "zero-duration snow-liquid capacity closure",
                    ));
                }
            }
        }
        if pending.values().any(|values| !values.is_empty()) {
            return Err(DirectSurfaceLiquidError::Identity(
                "zero-duration snow-liquid unresolved route",
            ));
        }
        if advance_persistent_lineage {
            for record in &mut ending.records {
                record.last_accepted_transaction_id = Some(transaction_id);
            }
            for continuation in &mut ending.continuations {
                continuation.last_accepted_transaction_id = Some(transaction_id);
            }
        }
        ending.state_sha256 = ending.recomputed_sha256()?;
        ending.validate(configuration)?;
        let retained_enthalpy_credits = credits
            .into_iter()
            .map(|((ofe_id, tile_id), (enthalpy, receipt_ids))| {
                let receipt_sha256 = zero_duration_snow_digest(
                    &receipt_ids.iter().map(<[u8; 32]>::as_slice).collect::<Vec<_>>(),
                );
                DirectZeroDurationSnowLiquidEnthalpyCreditV1 {
                    ofe_id,
                    tile_id,
                    enthalpy_j_m2_tile_ground: enthalpy,
                    receipt_sha256,
                }
            })
            .collect::<Vec<_>>();
        let receipt_set_sha256 = zero_duration_snow_liquid_receipt_set_sha256(&receipts)?;
        Ok(DirectZeroDurationSnowLiquidOutcomeV1 {
            ending_state: ending,
            receipts,
            retained_enthalpy_credits,
            receipt_set_sha256,
        })
    }
}

#[cfg(test)]
mod zero_duration_snow_liquid_tests {
    use super::*;

    fn capacity_configuration() -> DirectSurfaceLiquidConfiguration {
        let base = super::tests::configuration();
        let mut records = base.records.clone();
        for record in &mut records {
            if record.key.tile_id.as_str() == "open" {
                record.tile_fraction = 0.38;
                record.capacity_kg_m2_tile = 3.0;
            } else {
                record.tile_fraction = 0.62;
                record.capacity_kg_m2_tile = 6.0;
            }
        }
        DirectSurfaceLiquidConfiguration::new(
            base.owner_id,
            base.run_id,
            base.ofe_topology,
            base.ofe_bindings,
            records,
        )
        .expect("capacity configuration")
    }

    fn beginning_state(
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> DirectSurfaceLiquidOwnedState {
        let liquid = configuration
            .records
            .iter()
            .map(|record| {
                (
                    record.key.clone(),
                    if record.key.tile_id.as_str() == "covered" {
                        4.0
                    } else {
                        0.0
                    },
                )
            })
            .collect();
        DirectSurfaceLiquidOwnedState::new_initial(configuration, &liquid, 0)
            .expect("beginning state")
    }

    fn inputs(
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Vec<DirectZeroDurationSnowLiquidInputV1> {
        configuration
            .records
            .iter()
            .map(|record| DirectZeroDurationSnowLiquidInputV1 {
                output_receipt_sha256: [7; 32],
                output_set_sha256: [8; 32],
                predecessor_owner_set_sha256: [9; 32],
                receiver_context_sha256: [10; 32],
                support_start_ns: 1,
                support_end_ns: 2,
                receiver_ordinal: 3,
                ofe_id: record.key.ofe_id.clone(),
                tile_id: record.key.tile_id.clone(),
                tile_fraction: record.tile_fraction,
                mass_kg_m2_tile_ground: 4.0,
                sensible_enthalpy_j_m2_tile_ground: 40.0,
            })
            .collect()
    }

    fn routed_configuration() -> DirectSurfaceLiquidConfiguration {
        let make_binding = |name: &str, lane_index: usize| {
            let top = SoilLayerId::try_new(format!("{name}-top")).expect("soil layer");
            DirectSurfaceLiquidOfeBinding {
                ofe_id: OfeId::try_new(name).expect("OFE"),
                production_lane_index: lane_index,
                production_lane_id: u32::try_from(lane_index + 1).expect("lane"),
                ordered_soil_layer_ids: vec![
                    top.clone(),
                    SoilLayerId::try_new(format!("{name}-bottom")).expect("soil layer"),
                ],
                infiltration_soil_thermal_layer_id: top,
            }
        };
        let make_record = |name: &str,
                           area: f64,
                           capacity: f64,
                           route: Option<(&str, &str)>| {
            DirectSurfaceLiquidConfigurationRecord {
                key: DirectSurfaceLiquidStoreKey {
                    run_id: 72,
                    ofe_id: OfeId::try_new(name).expect("OFE"),
                    tile_id: TileId::try_new(format!("{name}-tile")).expect("tile"),
                    surface_id: SurfaceId::try_new(format!("{name}-surface")).expect("surface"),
                    surface_class: SurfaceClass::BareMineralSoil,
                    source_type: WaterSourceType::SurfaceLiquid,
                    source_id: SourceId::try_new(format!("{name}-source")).expect("source"),
                },
                tile_fraction: 1.0,
                capacity_kg_m2_tile: capacity,
                ofe_area_m2: area,
                ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
                runon_destination_ofe_id: route
                    .map(|(ofe, _)| OfeId::try_new(ofe).expect("route OFE")),
                runon_destination_tile_id: route
                    .map(|(_, tile)| TileId::try_new(tile).expect("route tile")),
            }
        };
        DirectSurfaceLiquidConfiguration::new(
            ResourceOwnerId::try_new("hydrology").expect("owner"),
            72,
            vec![
                OfeId::try_new("upper").expect("OFE"),
                OfeId::try_new("lower").expect("OFE"),
            ],
            vec![make_binding("upper", 0), make_binding("lower", 1)],
            vec![
                make_record("upper", 100.0, 1.0, Some(("lower", "lower-tile"))),
                make_record("lower", 200.0, 10.0, None),
            ],
        )
        .expect("routed configuration")
    }

    #[test]
    fn insufficient_capacity_retains_exactly_and_routes_mass_and_enthalpy_to_outlet() {
        let configuration = capacity_configuration();
        let beginning = beginning_state(&configuration);
        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(
                &configuration,
                &inputs(&configuration),
                false,
            )
            .expect("capacity/routing transaction");
        let open = outcome
            .ending_state
            .records
            .iter()
            .find(|record| record.key.tile_id.as_str() == "open")
            .expect("open state");
        let covered = outcome
            .ending_state
            .records
            .iter()
            .find(|record| record.key.tile_id.as_str() == "covered")
            .expect("covered state");
        assert_eq!(open.liquid_kg_m2_tile.to_bits(), 3.0_f64.to_bits());
        assert_eq!(covered.liquid_kg_m2_tile.to_bits(), 6.0_f64.to_bits());
        assert_eq!(outcome.retained_enthalpy_credits.len(), 2);
        let open_credit = outcome
            .retained_enthalpy_credits
            .iter()
            .find(|credit| credit.tile_id.as_str() == "open")
            .expect("open credit");
        let covered_credit = outcome
            .retained_enthalpy_credits
            .iter()
            .find(|credit| credit.tile_id.as_str() == "covered")
            .expect("covered credit");
        assert_eq!(open_credit.enthalpy_j_m2_tile_ground.to_bits(), 30.0_f64.to_bits());
        assert_eq!(covered_credit.enthalpy_j_m2_tile_ground.to_bits(), 20.0_f64.to_bits());
        let retained_mass = outcome
            .receipts
            .iter()
            .filter(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            })
            .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground)
            .sum::<f64>();
        let outlet_mass = outcome
            .receipts
            .iter()
            .filter(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff
            })
            .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground)
            .sum::<f64>();
        let retained_enthalpy = outcome
            .receipts
            .iter()
            .filter(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            })
            .map(|receipt| receipt.sensible_enthalpy_j_m2_basis_ofe_ground)
            .sum::<f64>();
        let outlet_enthalpy = outcome
            .receipts
            .iter()
            .filter(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff
            })
            .map(|receipt| receipt.sensible_enthalpy_j_m2_basis_ofe_ground)
            .sum::<f64>();
        assert!((retained_mass + outlet_mass - 4.0).abs() < 1.0e-12);
        assert!((retained_enthalpy + outlet_enthalpy - 40.0).abs() < 1.0e-12);
        assert_eq!(
            outcome.receipt_set_sha256,
            zero_duration_snow_liquid_receipt_set_sha256(&outcome.receipts)
                .expect("receipt set"),
        );
    }

    #[test]
    fn destination_fraction_partition_redistribution_replay_and_tamper_fail_closed() {
        let configuration = capacity_configuration();
        let beginning = beginning_state(&configuration);
        let canonical = inputs(&configuration);

        let mut omitted = canonical.clone();
        omitted.pop();
        assert!(beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &omitted, false)
            .is_err());

        let mut wrong_fraction = canonical.clone();
        wrong_fraction[0].tile_fraction = 0.39;
        assert!(beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &wrong_fraction, false)
            .is_err());

        let mut redistributed = canonical.clone();
        redistributed[0].mass_kg_m2_tile_ground = 5.0;
        redistributed[1].mass_kg_m2_tile_ground =
            (4.0 - 0.38 * 5.0) / 0.62;
        assert!(beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &redistributed, false)
            .is_err());

        let mut replay = canonical.clone();
        replay.extend(canonical.clone());
        assert!(beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &replay, false)
            .is_err());
        assert_eq!(beginning, beginning_state(&configuration));

        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &canonical, false)
            .expect("canonical outcome");
        let mut wrong_basis = outcome.receipts[0].clone();
        wrong_basis.recipient_tile_fraction = Some(0.5);
        assert!(wrong_basis.validate_seal().is_err());
        let mut wrong_enthalpy = outcome.receipts[0].clone();
        wrong_enthalpy.sensible_enthalpy_j_m2_basis_ofe_ground += 1.0;
        assert!(wrong_enthalpy.validate_seal().is_err());
        let mut duplicate = outcome.receipts.clone();
        duplicate.push(outcome.receipts[0].clone());
        assert!(zero_duration_snow_liquid_receipt_set_sha256(&duplicate).is_err());
    }

    #[test]
    fn overflow_routes_with_exact_area_mass_and_enthalpy_custody() {
        let configuration = routed_configuration();
        let liquid = configuration
            .records
            .iter()
            .map(|record| (record.key.clone(), 0.0))
            .collect();
        let beginning = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &liquid, 0)
            .expect("beginning state");
        let upper = &configuration.records[0];
        let input = DirectZeroDurationSnowLiquidInputV1 {
            output_receipt_sha256: [17; 32],
            output_set_sha256: [18; 32],
            predecessor_owner_set_sha256: [19; 32],
            receiver_context_sha256: [20; 32],
            support_start_ns: 1,
            support_end_ns: 2,
            receiver_ordinal: 0,
            ofe_id: upper.key.ofe_id.clone(),
            tile_id: upper.key.tile_id.clone(),
            tile_fraction: 1.0,
            mass_kg_m2_tile_ground: 3.0,
            sensible_enthalpy_j_m2_tile_ground: 30.0,
        };
        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &[input], false)
            .expect("routed transaction");
        assert_eq!(outcome.ending_state.records[0].liquid_kg_m2_tile, 1.0);
        assert_eq!(outcome.ending_state.records[1].liquid_kg_m2_tile, 1.0);
        let routed = outcome
            .receipts
            .iter()
            .find(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
            })
            .expect("routed receipt");
        assert_eq!(routed.mass_kg_m2_basis_ofe_ground.to_bits(), 2.0_f64.to_bits());
        assert_eq!(
            routed.sensible_enthalpy_j_m2_basis_ofe_ground.to_bits(),
            20.0_f64.to_bits(),
        );
        let lower_retained = outcome
            .receipts
            .iter()
            .find(|receipt| {
                receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
                    && receipt.recipient_ofe_id.as_str() == "lower"
            })
            .expect("lower retained receipt");
        assert_eq!(
            lower_retained.mass_kg_m2_basis_ofe_ground.to_bits(),
            1.0_f64.to_bits(),
        );
        assert_eq!(
            lower_retained
                .sensible_enthalpy_j_m2_basis_ofe_ground
                .to_bits(),
            10.0_f64.to_bits(),
        );
        assert_eq!(
            100.0 * 3.0,
            100.0 * outcome.ending_state.records[0].liquid_kg_m2_tile
                + 200.0 * outcome.ending_state.records[1].liquid_kg_m2_tile,
        );
    }

    #[test]
    fn all_runoff_surface_delta_is_exact_lineage_only_and_poisons_reject() {
        let configuration = routed_configuration();
        let liquid = configuration
            .records
            .iter()
            .map(|record| (record.key.clone(), record.capacity_kg_m2_tile))
            .collect();
        let mut beginning = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &liquid, 0)
            .expect("saturated beginning state");
        for record in &mut beginning.records {
            record.last_accepted_transaction_id = Some(TransactionId(7));
        }
        for continuation in &mut beginning.continuations {
            continuation.next_interval_index = 1;
            continuation.last_accepted_transaction_id = Some(TransactionId(7));
        }
        beginning.state_sha256 = beginning.recomputed_sha256().expect("accepted beginning seal");
        beginning
            .validate(&configuration)
            .expect("accepted beginning state");
        let upper = &configuration.records[0];
        let input = DirectZeroDurationSnowLiquidInputV1 {
            output_receipt_sha256: [31; 32],
            output_set_sha256: [32; 32],
            predecessor_owner_set_sha256: [33; 32],
            receiver_context_sha256: [34; 32],
            support_start_ns: 1,
            support_end_ns: 2,
            receiver_ordinal: 0,
            ofe_id: upper.key.ofe_id.clone(),
            tile_id: upper.key.tile_id.clone(),
            tile_fraction: 1.0,
            mass_kg_m2_tile_ground: f64::from_bits(0x3ac0_0000_0000_0000),
            sensible_enthalpy_j_m2_tile_ground: 0.0,
        };
        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &[input], true)
            .expect("all-runoff lineage transaction");
        assert_eq!(outcome.receipts.len(), 2);
        assert!(outcome.receipts.iter().all(|receipt| {
            receipt.disposition != DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
        }));
        assert!(validate_zero_duration_snow_liquid_surface_delta_v1(
            &beginning,
            &outcome.ending_state,
            &outcome.receipts,
        )
        .is_ok());
        assert!(beginning
            .records
            .iter()
            .zip(&outcome.ending_state.records)
            .all(|(before, after)| {
                before.liquid_kg_m2_tile.to_bits() == after.liquid_kg_m2_tile.to_bits()
                    && before.last_accepted_transaction_id
                        != after.last_accepted_transaction_id
            }));

        let mut partial_lineage = outcome.ending_state.clone();
        partial_lineage.records[0].last_accepted_transaction_id =
            beginning.records[0].last_accepted_transaction_id;
        partial_lineage.state_sha256 = partial_lineage.recomputed_sha256().expect("poison seal");
        assert!(validate_zero_duration_snow_liquid_surface_delta_v1(
            &beginning,
            &partial_lineage,
            &outcome.receipts,
        )
        .is_err());

        let mut false_storage = outcome.ending_state.clone();
        false_storage.records[0].liquid_kg_m2_tile = f64::from_bits(
            false_storage.records[0]
                .liquid_kg_m2_tile
                .to_bits()
                .checked_add(1)
                .expect("one ULP"),
        );
        false_storage.state_sha256 = false_storage.recomputed_sha256().expect("poison seal");
        assert!(validate_zero_duration_snow_liquid_surface_delta_v1(
            &beginning,
            &false_storage,
            &outcome.receipts,
        )
        .is_err());

        let mut substituted_receipts = outcome.receipts.clone();
        substituted_receipts[1].transaction_id = TransactionId(99);
        substituted_receipts[1].receipt_sha256 =
            zero_duration_snow_receipt_digest(&substituted_receipts[1]);
        assert!(validate_zero_duration_snow_liquid_surface_delta_v1(
            &beginning,
            &outcome.ending_state,
            &substituted_receipts,
        )
        .is_err());
        assert!(validate_zero_duration_snow_liquid_surface_delta_v1(
            &beginning,
            &outcome.ending_state,
            &[],
        )
        .is_err());
    }
}
