/// Sole resident soil-thermal owner for the direct V9/V10 consumers.
///
/// V9 construction admits only `V1`; the successor V10 constructor admits
/// only `V2`. The variants are never projected into one another after
/// construction.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectSoilThermalResident {
    V1(SoilThermalSnapshot),
    V2(DirectV10SoilThermalResidentV2),
}

/// One unpublished soil-thermal candidate without a compatibility projection.
///
/// The V2 arm retains the native trial, including its exact-carry ending and
/// transaction lineage. It must be sealed through the V2 acceptance path;
/// V1 receipt helpers deliberately cannot consume it.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectSoilThermalCandidate {
    V1(SoilThermalSnapshot),
    V2(openwepp_land_surface_energy::SoilThermalTrialStateV2),
}

/// Receipt and source custody retained for the latest accepted V2 support.
/// The predecessor is historical validation evidence, not a second resident
/// owner and is never used as constitutive state.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DirectV10SoilThermalAcceptedCustodyV2 {
    predecessor: openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    credit_receipt: openwepp_land_surface_energy::SoilThermalEnergyCreditReceiptV2,
    expected_sources: SoilThermalExpectedAcceptedOperandSetV2,
    seals: SoilThermalOrchestratorSealsV2,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DirectV10SoilThermalResidentV2 {
    owner: openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    receipt_free_seals: Option<openwepp_land_surface_energy::SoilThermalReceiptFreeOwnerSealsV2>,
    latest_accepted: Option<DirectV10SoilThermalAcceptedCustodyV2>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectSoilThermalReadView<'a> {
    V1(&'a SoilThermalSnapshot),
    V2(&'a openwepp_land_surface_energy::SoilThermalOwnedStateV2),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectSoilThermalOfeReadView<'a> {
    V1(&'a openwepp_land_surface_energy::SoilThermalOfeSnapshot),
    V2(&'a openwepp_land_surface_energy::SoilThermalOfeStateV2),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectSoilThermalLayerReadView<'a> {
    V1(&'a openwepp_land_surface_energy::SoilThermalLayerSnapshot),
    V2(&'a openwepp_land_surface_energy::SoilThermalLayerStateV2),
}

impl Serialize for DirectSoilThermalResident {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::V1(owner) => owner.serialize(serializer),
            Self::V2(resident) => resident.serialize(serializer),
        }
    }
}

impl<'a> DirectSoilThermalReadView<'a> {
    pub fn validate(self) -> Result<(), DirectV9RealConsumerError> {
        match self {
            Self::V1(owner) => owner.validate().map_err(Into::into),
            Self::V2(owner) => owner
                .validate()
                .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 soil read view")),
        }
    }

    pub fn ordered_ofes(self) -> Vec<DirectSoilThermalOfeReadView<'a>> {
        match self {
            Self::V1(owner) => owner
                .ofes
                .iter()
                .map(DirectSoilThermalOfeReadView::V1)
                .collect(),
            Self::V2(owner) => owner
                .ofes
                .iter()
                .map(DirectSoilThermalOfeReadView::V2)
                .collect(),
        }
    }
}

impl<'a> DirectSoilThermalOfeReadView<'a> {
    pub fn ofe_id(self) -> &'a OfeId {
        match self {
            Self::V1(ofe) => &ofe.ofe_id,
            Self::V2(ofe) => &ofe.ofe_id,
        }
    }

    pub fn ordered_layers(self) -> Vec<DirectSoilThermalLayerReadView<'a>> {
        match self {
            Self::V1(ofe) => ofe
                .ordered_layers
                .iter()
                .map(DirectSoilThermalLayerReadView::V1)
                .collect(),
            Self::V2(ofe) => ofe
                .ordered_layers
                .iter()
                .map(DirectSoilThermalLayerReadView::V2)
                .collect(),
        }
    }
}

impl<'a> DirectSoilThermalLayerReadView<'a> {
    pub fn layer_id(self) -> &'a SoilLayerId {
        match self {
            Self::V1(layer) => &layer.layer_id,
            Self::V2(layer) => &layer.layer_id,
        }
    }

    pub fn temperature_k(self) -> f64 {
        match self {
            Self::V1(layer) => layer.temperature_k,
            Self::V2(layer) => layer.temperature_k,
        }
    }

    pub fn enthalpy_high_j_m2_ofe_ground(self) -> f64 {
        match self {
            Self::V1(layer) => layer.enthalpy_j_m2_ofe_ground,
            Self::V2(layer) => layer.enthalpy_hi_j_m2_ofe_ground,
        }
    }

    pub fn exact_carry(self) -> Option<&'a openwepp_land_surface_energy::ExactDyadicEnthalpy> {
        match self {
            Self::V1(_) => None,
            Self::V2(layer) => Some(&layer.enthalpy_carry),
        }
    }
}

impl DirectSoilThermalResident {
    pub fn try_new_v1(owner: SoilThermalSnapshot) -> Result<Self, DirectV9RealConsumerError> {
        owner.validate()?;
        Ok(Self::V1(owner))
    }

    pub fn try_new_v2(
        prepared: openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        seals: openwepp_land_surface_energy::SoilThermalReceiptFreeOwnerSealsV2,
    ) -> Result<Self, DirectV9RealConsumerError> {
        openwepp_land_surface_energy::validate_soil_thermal_receipt_free_owner_v2(
            &prepared, &seals,
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("receipt-free V2 soil owner"))?;
        Ok(Self::V2(DirectV10SoilThermalResidentV2 {
            owner: prepared.beginning_owner().clone(),
            receipt_free_seals: Some(seals),
            latest_accepted: None,
        }))
    }

    pub fn validate(&self) -> Result<(), DirectV9RealConsumerError> {
        match self {
            Self::V1(owner) => owner.validate().map_err(Into::into),
            Self::V2(resident) => resident.validate(),
        }
    }

    pub fn v1(&self) -> Result<&SoilThermalSnapshot, DirectV9RealConsumerError> {
        match self {
            Self::V1(owner) => Ok(owner),
            Self::V2(_) => Err(DirectV9RealConsumerError::Unsupported(
                "V1 soil accessor on V2 resident",
            )),
        }
    }

    #[cfg(any(test, feature = "restart-authority-evidence"))]
    fn v1_mut(&mut self) -> Result<&mut SoilThermalSnapshot, DirectV9RealConsumerError> {
        match self {
            Self::V1(owner) => Ok(owner),
            Self::V2(_) => Err(DirectV9RealConsumerError::Unsupported(
                "V1 soil mutation on V2 resident",
            )),
        }
    }

    pub fn v2(&self) -> Result<&DirectV10SoilThermalResidentV2, DirectV9RealConsumerError> {
        match self {
            Self::V1(_) => Err(DirectV9RealConsumerError::Unsupported(
                "V2 soil accessor on V1 resident",
            )),
            Self::V2(owner) => Ok(owner),
        }
    }

    pub fn read_view(&self) -> DirectSoilThermalReadView<'_> {
        match self {
            Self::V1(owner) => DirectSoilThermalReadView::V1(owner),
            Self::V2(resident) => DirectSoilThermalReadView::V2(&resident.owner.state),
        }
    }

    pub fn owner_id(&self) -> &ResourceOwnerId {
        match self {
            Self::V1(owner) => &owner.owner_id,
            Self::V2(resident) => &resident.owner.state.owner_id,
        }
    }

    pub fn configuration_sha256(&self) -> &Sha256Digest {
        match self {
            Self::V1(owner) => &owner.configuration_sha256,
            Self::V2(resident) => &resident.owner.state.configuration_sha256,
        }
    }

    pub fn state_sha256(&self) -> &Sha256Digest {
        match self {
            Self::V1(owner) => &owner.state_sha256,
            Self::V2(resident) => &resident.owner.state.state_sha256,
        }
    }

    pub fn last_accepted_transaction_id(&self) -> Option<TransactionId> {
        match self {
            Self::V1(owner) => owner.last_accepted_transaction_id,
            Self::V2(resident) => resident.owner.state.last_accepted_transaction_id,
        }
    }

    pub fn canonical_active_owner_bytes(&self) -> Result<Vec<u8>, DirectV9RealConsumerError> {
        match self {
            Self::V1(owner) => serde_json::to_vec(owner)
                .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string())),
            Self::V2(resident) => resident.canonical_active_owner_bytes(),
        }
    }

    pub fn prepare_v2_support(
        &self,
        transaction_id: TransactionId,
        support_start_ns: u128,
        support_end_ns: u128,
    ) -> Result<openwepp_land_surface_energy::PreparedSoilThermalSupportV2, DirectV9RealConsumerError>
    {
        let current = self.v2()?.owner();
        openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            current,
            transaction_id,
            support_start_ns,
            support_end_ns,
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("prepare V2 soil support"))
    }
}

impl DirectSoilThermalCandidate {
    pub fn from_v1(candidate: SoilThermalSnapshot) -> Result<Self, DirectV9RealConsumerError> {
        candidate.validate()?;
        Ok(Self::V1(candidate))
    }

    pub fn from_v2(
        candidate: openwepp_land_surface_energy::SoilThermalTrialStateV2,
    ) -> Result<Self, DirectV9RealConsumerError> {
        candidate
            .ending_state()
            .validate()
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 soil trial ending"))?;
        Ok(Self::V2(candidate))
    }

    pub fn v1(&self) -> Result<&SoilThermalSnapshot, DirectV9RealConsumerError> {
        match self {
            Self::V1(candidate) => Ok(candidate),
            Self::V2(_) => Err(DirectV9RealConsumerError::Unsupported(
                "V1 soil candidate accessor on V2 trial",
            )),
        }
    }

    pub fn v2(
        &self,
    ) -> Result<&openwepp_land_surface_energy::SoilThermalTrialStateV2, DirectV9RealConsumerError>
    {
        match self {
            Self::V1(_) => Err(DirectV9RealConsumerError::Unsupported(
                "V2 soil candidate accessor on V1 candidate",
            )),
            Self::V2(candidate) => Ok(candidate),
        }
    }

    pub fn read_view(&self) -> DirectSoilThermalReadView<'_> {
        match self {
            Self::V1(candidate) => DirectSoilThermalReadView::V1(candidate),
            Self::V2(candidate) => DirectSoilThermalReadView::V2(candidate.ending_state()),
        }
    }

    pub fn transaction_id(&self) -> Option<TransactionId> {
        match self {
            Self::V1(candidate) => candidate.last_accepted_transaction_id,
            Self::V2(candidate) => Some(candidate.transaction_id()),
        }
    }

    pub fn owner_id(&self) -> &ResourceOwnerId {
        match self {
            Self::V1(candidate) => &candidate.owner_id,
            Self::V2(candidate) => &candidate.ending_state().owner_id,
        }
    }

    pub fn state_sha256(&self) -> &Sha256Digest {
        match self {
            Self::V1(candidate) => &candidate.state_sha256,
            Self::V2(candidate) => &candidate.ending_state().state_sha256,
        }
    }
}

impl DirectV10SoilThermalResidentV2 {
    pub fn owner(&self) -> &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2 {
        &self.owner
    }

    pub fn latest_accepted(&self) -> Option<&DirectV10SoilThermalAcceptedCustodyV2> {
        self.latest_accepted.as_ref()
    }

    pub fn receipt_free_seals(
        &self,
    ) -> Option<&openwepp_land_surface_energy::SoilThermalReceiptFreeOwnerSealsV2> {
        self.receipt_free_seals.as_ref()
    }

    fn validate_prepared_beginning(
        &self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    ) -> Result<(), DirectV9RealConsumerError> {
        beginning
            .validate()
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("prepared V2 soil beginning"))?;
        if beginning.owner_tag != self.owner.owner_tag
            || beginning.schema_sha256 != self.owner.schema_sha256
            || beginning.exact_carry_definition_sha256 != self.owner.exact_carry_definition_sha256
            || beginning.parent_v1_state_sha256 != self.owner.parent_v1_state_sha256
            || beginning.state != self.owner.state
            || beginning.receipt_chain_sha256 != self.owner.receipt_chain_sha256
            || beginning.expected_predecessor_transaction_id
                != self.owner.state.last_accepted_transaction_id
            || beginning.state.last_accepted_transaction_id == Some(beginning.transaction_id)
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 resident prepared-beginning join",
            ));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), DirectV9RealConsumerError> {
        self.owner
            .validate()
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("resident V2 soil owner"))?;
        if self.receipt_free_seals.is_some() == self.latest_accepted.is_some() {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 resident custody cardinality",
            ));
        }
        if let Some(seals) = &self.receipt_free_seals {
            let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
                &self.owner,
                self.owner.transaction_id,
                self.owner.support_start_ns,
                self.owner.support_end_ns,
            )
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("receipt-free V2 support"))?;
            if prepared.beginning_owner() != &self.owner {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "receipt-free V2 support reconstruction",
                ));
            }
            openwepp_land_surface_energy::validate_soil_thermal_receipt_free_owner_v2(
                &prepared, seals,
            )
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("receipt-free V2 seals"))?;
        }
        if let Some(custody) = &self.latest_accepted {
            let candidate = SoilThermalAcceptedCandidateV2 {
                ending_owner: self.owner.clone(),
                credit_receipt: custody.credit_receipt.clone(),
                expected_sources: custody.expected_sources.clone(),
            };
            validate_soil_thermal_orchestrator_seals_v2(
                &custody.predecessor,
                &candidate,
                &custody.seals,
            )?;
        }
        Ok(())
    }

    fn accepted(
        &self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        candidate: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<Self, DirectV9RealConsumerError> {
        self.validate_prepared_beginning(beginning)?;
        validate_soil_thermal_orchestrator_seals_v2(beginning, &candidate, &seals)?;
        canonical_soil_thermal_v2_bundle_bytes(beginning, &candidate, &seals)?;
        let accepted = Self {
            owner: candidate.ending_owner,
            receipt_free_seals: None,
            latest_accepted: Some(DirectV10SoilThermalAcceptedCustodyV2 {
                predecessor: beginning.clone(),
                credit_receipt: candidate.credit_receipt,
                expected_sources: candidate.expected_sources,
                seals,
            }),
        };
        accepted.validate()?;
        Ok(accepted)
    }

    fn canonical_active_owner_bytes(&self) -> Result<Vec<u8>, DirectV9RealConsumerError> {
        #[derive(Serialize)]
        struct CanonicalResident<'a> {
            schema: &'static str,
            owner: &'a openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
            latest_credit_receipt_sha256: Option<&'a Sha256Digest>,
            expected_operand_set_sha256: Option<&'a Sha256Digest>,
            orchestrator_seal_sha256: Option<&'a Sha256Digest>,
            receipt_free_seal_sha256: Option<&'a Sha256Digest>,
        }
        self.validate()?;
        serde_json::to_vec(&CanonicalResident {
            schema: "OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V2",
            owner: &self.owner,
            latest_credit_receipt_sha256: self
                .latest_accepted
                .as_ref()
                .map(|custody| &custody.credit_receipt.receipt_sha256),
            expected_operand_set_sha256: self
                .latest_accepted
                .as_ref()
                .map(|custody| custody.expected_sources.expected_set_sha256()),
            orchestrator_seal_sha256: self
                .latest_accepted
                .as_ref()
                .map(|custody| &custody.seals.orchestrator_seal_sha256),
            receipt_free_seal_sha256: self
                .receipt_free_seals
                .as_ref()
                .map(|seals| &seals.receipt_free_seal_sha256),
        })
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))
    }
}

impl DirectV10SoilThermalAcceptedCustodyV2 {
    pub fn predecessor(&self) -> &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2 {
        &self.predecessor
    }

    pub fn credit_receipt(
        &self,
    ) -> &openwepp_land_surface_energy::SoilThermalEnergyCreditReceiptV2 {
        &self.credit_receipt
    }

    pub fn expected_sources(&self) -> &SoilThermalExpectedAcceptedOperandSetV2 {
        &self.expected_sources
    }

    pub fn seals(&self) -> &SoilThermalOrchestratorSealsV2 {
        &self.seals
    }
}

impl DirectV10RealConsumerShadow {
    pub fn soil_thermal_resident(&self) -> &DirectSoilThermalResident {
        &self.inner.soil_thermal
    }

    pub fn soil_thermal_v2(
        &self,
    ) -> Result<&DirectV10SoilThermalResidentV2, DirectV10RealConsumerError> {
        self.inner.soil_thermal.v2().map_err(Into::into)
    }

    pub fn prepare_soil_thermal_support_v2(
        &self,
        transaction_id: TransactionId,
        support_start_ns: u128,
        support_end_ns: u128,
    ) -> Result<
        openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        DirectV10RealConsumerError,
    > {
        self.inner
            .soil_thermal
            .prepare_v2_support(transaction_id, support_start_ns, support_end_ns)
            .map_err(Into::into)
    }

    pub fn advance_soil_thermal_trial_v2(
        &self,
        prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        physical_operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
        temperature_projections: &[openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2],
    ) -> Result<DirectSoilThermalCandidate, DirectV10RealConsumerError> {
        let resident = self.inner.soil_thermal.v2()?;
        resident.validate_prepared_beginning(prepared.beginning_owner())?;
        let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
            prepared,
            physical_operands,
            temperature_projections,
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::OwnerClosure(
                "advance V2 soil trial",
            ))
        })?;
        DirectSoilThermalCandidate::from_v2(trial).map_err(Into::into)
    }

    pub fn install_soil_thermal_accepted_v2(
        &mut self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let resident = self.inner.soil_thermal.v2()?;
        let accepted_resident = resident.accepted(beginning, accepted, seals)?;
        let accepted_transaction = accepted_resident.owner.state.last_accepted_transaction_id;
        let expected_transaction = TransactionId(self.vegetation_state.0.last_transaction_id);
        if accepted_transaction != Some(expected_transaction)
            || self.lse_state.0.last_accepted_transaction_id != Some(expected_transaction)
            || self.inner.biogeochemistry.last_transaction_id != expected_transaction.0
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 soil atomic complete-owner transaction join",
                ),
            ));
        }
        let mut candidate = self.clone();
        candidate.inner.soil_thermal = DirectSoilThermalResident::V2(accepted_resident);
        candidate.inner.soil_thermal.validate()?;
        candidate
            .inner
            .soil_thermal
            .canonical_active_owner_bytes()?;
        *self = candidate;
        Ok(())
    }
}

#[cfg(test)]
mod direct_v10_soil_thermal_v2_tests {
    include!("v10_soil_thermal_v2_tests.rs");
}
