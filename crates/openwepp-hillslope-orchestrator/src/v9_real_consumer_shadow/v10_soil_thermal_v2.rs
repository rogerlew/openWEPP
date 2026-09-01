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

#[derive(Clone, Copy, Debug, PartialEq)]
enum DirectSoilThermalUnpublishedFixedPointPostureV2<'a> {
    BasePhysical(&'a openwepp_land_surface_energy::SoilThermalTrialStateV2),
    NumericalCoordinateProjection(
        &'a openwepp_land_surface_energy::SoilThermalTrialStateV2,
    ),
}

impl<'a> DirectSoilThermalUnpublishedFixedPointPostureV2<'a> {
    fn try_from_candidate(
        candidate: &'a DirectSoilThermalCandidate,
    ) -> Result<Self, DirectV9RealConsumerError> {
        let trial = candidate.v2()?;
        match (
            trial.numerical_coordinate_authority_sha256(),
            trial.numerical_coordinate_set_sha256(),
        ) {
            (None, None) => Ok(Self::BasePhysical(trial)),
            (Some(_), Some(_)) => Ok(Self::NumericalCoordinateProjection(trial)),
            _ => Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 projected fixed-point custody posture",
            )),
        }
    }

    const fn trial(self) -> &'a openwepp_land_surface_energy::SoilThermalTrialStateV2 {
        match self {
            Self::BasePhysical(trial) | Self::NumericalCoordinateProjection(trial) => trial,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectSoilThermalAtomicCompleteOwnerTransactionKindV2 {
    SameSourceAndSoilTarget,
    AuthenticatedSoilSuccessor,
    AuthenticatedPreparedBeginning,
}

#[derive(Clone, Copy, Debug)]
enum DirectSoilThermalAtomicInstallAuthorityV2<'a> {
    Physical(
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
    ),
    AuthenticatedPreparedBeginning(
        &'a DirectSoilThermalPreparedBeginningInstallAuthorityV2,
    ),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2 {
    source_transaction_id: TransactionId,
    soil_target_transaction_id: TransactionId,
    soil_expected_predecessor_transaction_id: Option<TransactionId>,
    kind: DirectSoilThermalAtomicCompleteOwnerTransactionKindV2,
}

fn direct_soil_thermal_complete_source_transaction_v2(
    candidate: &DirectV10RealConsumerShadow,
) -> Result<TransactionId, DirectV9RealConsumerError> {
    let source_transaction_id = TransactionId(candidate.vegetation_state.0.last_transaction_id);
    if source_transaction_id.0 == 0
        || candidate.lse_state.0.last_accepted_transaction_id != Some(source_transaction_id)
        || candidate.inner.biogeochemistry.last_transaction_id != source_transaction_id.0
    {
        return Err(DirectV9RealConsumerError::Identity(
            "V2 soil atomic complete-owner source transaction join",
        ));
    }
    Ok(source_transaction_id)
}

fn direct_soil_thermal_non_soil_ending_matches_v2(
    candidate: &DirectV10RealConsumerShadow,
    expected: &DirectV10RealConsumerShadow,
) -> bool {
    let mut expected = expected.clone();
    expected.inner.soil_thermal = candidate.inner.soil_thermal.clone();
    expected.accepted_publication_history = candidate.accepted_publication_history.clone();
    candidate == &expected
}

fn direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
    candidate: &DirectV10RealConsumerShadow,
    accepted_resident: &DirectV10SoilThermalResidentV2,
    split_authority: Option<DirectSoilThermalAtomicInstallAuthorityV2<'_>>,
) -> Result<DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2, DirectV9RealConsumerError> {
    accepted_resident.validate()?;
    let source_transaction_id = direct_soil_thermal_complete_source_transaction_v2(candidate)?;
    let soil_target_transaction_id = accepted_resident.owner.transaction_id;
    let soil_expected_predecessor_transaction_id =
        accepted_resident.owner.expected_predecessor_transaction_id;
    if accepted_resident.owner.state.last_accepted_transaction_id
        != Some(soil_target_transaction_id)
        || accepted_resident.owner.state.ofes.iter().any(|ofe| {
            ofe.ordered_layers.iter().any(|layer| {
                layer.last_accepted_transaction_id != Some(soil_target_transaction_id)
            })
        })
    {
        return Err(DirectV9RealConsumerError::Identity(
            "V2 soil atomic target-state transaction join",
        ));
    }

    match split_authority {
        None if source_transaction_id == soil_target_transaction_id => {
            Ok(DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2 {
                source_transaction_id,
                soil_target_transaction_id,
                soil_expected_predecessor_transaction_id,
                kind: DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::
                    SameSourceAndSoilTarget,
            })
        }
        Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(authority)) => {
            let native_authority = crate::land_surface_energy_shadow::
                PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                    authority.source_transaction_id,
                    authority.soil_thermal_transaction_id,
                )
                .map_err(|_| {
                    DirectV9RealConsumerError::Identity(
                        "V2 soil atomic native transaction authority",
                    )
                })?;
            if authority != native_authority
                || authority.source_transaction_id != source_transaction_id
                || authority.soil_thermal_transaction_id != soil_target_transaction_id
            {
                return Err(DirectV9RealConsumerError::Identity(
                    "V2 soil atomic explicit transaction authority join",
                ));
            }
            if source_transaction_id == soil_target_transaction_id {
                return Ok(DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2 {
                    source_transaction_id,
                    soil_target_transaction_id,
                    soil_expected_predecessor_transaction_id,
                    kind: DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::
                        SameSourceAndSoilTarget,
                });
            }
            if soil_expected_predecessor_transaction_id != Some(source_transaction_id) {
                return Err(DirectV9RealConsumerError::Identity(
                    "V2 soil atomic authenticated predecessor transaction join",
                ));
            }
            Ok(DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2 {
                source_transaction_id,
                soil_target_transaction_id,
                soil_expected_predecessor_transaction_id,
                kind: DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::
                    AuthenticatedSoilSuccessor,
            })
        }
        Some(DirectSoilThermalAtomicInstallAuthorityV2::AuthenticatedPreparedBeginning(
            authority,
        )) => {
            authority.authoritative_resident.validate()?;
            authority
                .authoritative_resident
                .validate_prepared_beginning(&authority.prepared_beginning)?;
            let physical = authority.physical_transaction_authority;
            let reconstructed_physical = crate::land_surface_energy_shadow::
                PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                    source_transaction_id,
                    soil_target_transaction_id,
                )
                .map_err(|_| {
                    DirectV9RealConsumerError::Identity(
                        "V2 soil atomic prepared native transaction authority",
                    )
                })?;
            let source_authority_matches = match &authority.source_authority {
                DirectSoilThermalPreparedBeginningSourceAuthorityV2::AuthenticatedBeginning(
                    authenticated_source_transaction_id,
                ) => *authenticated_source_transaction_id == source_transaction_id,
                DirectSoilThermalPreparedBeginningSourceAuthorityV2::ValidatedOuterTransition(
                    transition,
                ) => transition.validated_source_and_expected().is_ok_and(
                    |(authenticated_source, expected)| {
                        authenticated_source == source_transaction_id
                            && direct_soil_thermal_non_soil_ending_matches_v2(candidate, expected)
                    },
                ),
            };
            let refusal = if physical != reconstructed_physical
                || physical.source_transaction_id != source_transaction_id
                || physical.soil_thermal_transaction_id != soil_target_transaction_id
            {
                Some("V2 soil atomic authenticated prepared-beginning physical authority join")
            } else if !source_authority_matches {
                Some("V2 soil atomic authenticated prepared-beginning source authority join")
            } else if authority.prepared_beginning.transaction_id != soil_target_transaction_id {
                Some("V2 soil atomic authenticated prepared-beginning target join")
            } else if soil_expected_predecessor_transaction_id
                != authority
                    .prepared_beginning
                    .expected_predecessor_transaction_id
            {
                Some("V2 soil atomic authenticated accepted predecessor join")
            } else if accepted_resident
                .latest_accepted
                .as_ref()
                .is_none_or(|custody| custody.predecessor != authority.prepared_beginning)
            {
                Some("V2 soil atomic authenticated latest-accepted custody join")
            } else {
                None
            };
            if let Some(reason) = refusal {
                return Err(DirectV9RealConsumerError::Identity(
                    reason,
                ));
            }
            Ok(DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2 {
                source_transaction_id,
                soil_target_transaction_id,
                soil_expected_predecessor_transaction_id,
                kind: DirectSoilThermalAtomicCompleteOwnerTransactionKindV2::
                    AuthenticatedPreparedBeginning,
            })
        }
        None => Err(DirectV9RealConsumerError::Identity(
            "V2 soil atomic split transaction authority required",
        )),
    }
}

/// Authenticated continuation of an unpublished native V2 trial across one
/// contiguous physical child support.
///
/// This is a validation and recomposition context, not an owner envelope. It
/// never publishes the retained ending or treats it as an accepted resident.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSoilThermalUnpublishedContinuationV2 {
    original_prepared: openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    retained_trial: openwepp_land_surface_energy::SoilThermalTrialStateV2,
    retained_accumulated_operands:
        Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
    retained_layer_credit_chain:
        Vec<Vec<openwepp_land_surface_energy::SoilThermalLayerEnergyCreditV2>>,
    child_support_start_ns: u128,
    child_support_end_ns: u128,
}

/// Opaque unpublished child result retained by the carrier.
///
/// `physical_trial` is sequential and may be used as the next constitutive
/// beginning. `accumulated_operands` is a separate final-replay sidecar and is
/// never applied to that sequential child a second time.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSoilThermalUnpublishedContinuationResultV2 {
    original_prepared: openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
    physical_trial: openwepp_land_surface_energy::SoilThermalTrialStateV2,
    accumulated_operands: Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
    layer_credit_chain: Vec<Vec<openwepp_land_surface_energy::SoilThermalLayerEnergyCreditV2>>,
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

/// Opaque authority for one accepted native-V2 soil successor whose exact
/// authenticated predecessor may advance independently of the fixed outer V11
/// source transaction. It is constructed and reconstructed only through the
/// complete resident/prepared-beginning validator.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSoilThermalPreparedBeginningInstallAuthorityV2 {
    physical_transaction_authority:
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
    source_authority: DirectSoilThermalPreparedBeginningSourceAuthorityV2,
    authoritative_resident: DirectV10SoilThermalResidentV2,
    prepared_beginning: openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
}

#[derive(Clone, Debug, PartialEq)]
enum DirectSoilThermalPreparedBeginningSourceAuthorityV2 {
    AuthenticatedBeginning(TransactionId),
    ValidatedOuterTransition(DirectSoilThermalOuterOwnerTransitionAuthorityV2),
}

/// Opaque proof that a complete non-soil ending was reconstructed from the
/// authenticated constitutive beginning and one validated covered-V8 owner
/// envelope. Soil and publication history are deliberately outside this
/// transition because they retain independent custody.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
    authenticated_transition: DirectSoilThermalOuterOwnerTransitionSourceV2,
}

#[derive(Clone, Debug, PartialEq)]
enum DirectSoilThermalOuterOwnerTransitionSourceV2 {
    ValidatedEnvelope(v11_covered::AuthenticatedCoveredV8OuterOwnerTransitionV1),
    #[cfg(test)]
    TestOnly {
        source_transaction_id: TransactionId,
        expected_non_soil_ending: Box<DirectV10RealConsumerShadow>,
    },
}

impl DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
    fn validated_source_and_expected(
        &self,
    ) -> Result<(TransactionId, &DirectV10RealConsumerShadow), DirectV9RealConsumerError> {
        match &self.authenticated_transition {
            DirectSoilThermalOuterOwnerTransitionSourceV2::ValidatedEnvelope(transition) => {
                transition.envelope().validate()?;
                Ok((
                    transition.envelope().transaction_id(),
                    transition.expected_non_soil_ending(),
                ))
            }
            #[cfg(test)]
            DirectSoilThermalOuterOwnerTransitionSourceV2::TestOnly {
                source_transaction_id,
                expected_non_soil_ending,
            } => Ok((*source_transaction_id, expected_non_soil_ending)),
        }
    }
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

    /// Compare only constitutive soil state. Transaction, receipt-chain, and
    /// accepted-publication custody remain independently validated by their
    /// typed envelopes and are deliberately not projected into this view.
    pub fn physically_equals(self, other: Self) -> bool {
        match (self, other) {
            (Self::V1(left), Self::V1(right)) => left == right,
            (Self::V2(left), Self::V2(right)) => {
                soil_thermal_v2_physical_ending_matches(left, right)
            }
            _ => false,
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

    /// Prepare the next native V2 support from the resident's authenticated
    /// custody chain.
    ///
    /// A receipt-free resident already names the transaction reserved for its
    /// first accepted child. After acceptance, the installed owner is the
    /// predecessor authority and the next child receives the checked numeric
    /// successor. The support join is independently enforced by
    /// `validate_prepared_beginning`; callers cannot reuse a parent-envelope
    /// transaction across accepted children.
    pub fn prepare_next_v2_support(
        &self,
        support_start_ns: u128,
        support_end_ns: u128,
    ) -> Result<openwepp_land_surface_energy::PreparedSoilThermalSupportV2, DirectV9RealConsumerError>
    {
        let resident = self.v2()?;
        resident.validate()?;
        let transaction_id = if resident.receipt_free_seals.is_some() {
            resident.owner.transaction_id
        } else {
            resident
                .owner
                .transaction_id
                .0
                .checked_add(1)
                .map(TransactionId)
                .ok_or(DirectV9RealConsumerError::OwnerClosure(
                    "accepted V2 support transaction overflow",
                ))?
        };
        let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            &resident.owner,
            transaction_id,
            support_start_ns,
            support_end_ns,
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("prepare next V2 soil support"))?;
        resident.validate_prepared_beginning(prepared.beginning_owner())?;
        Ok(prepared)
    }

    pub fn prepare_unpublished_physical_beginning_v2(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
        candidate: &DirectSoilThermalCandidate,
        prior: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<
        openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2,
        DirectV9RealConsumerError,
    > {
        let retained_trial = candidate.v2()?;
        let resident = self.v2()?;
        let same_support_prior = prior.is_some_and(|prior| {
            let trial = prior.physical_trial();
            trial.support_start_ns() == child_support_start_ns
                && trial.support_end_ns() == child_support_end_ns
        });
        let original_prepared = if same_support_prior {
            // A cached base result for this child is authentication evidence,
            // not the unpublished predecessor. Re-authenticate it against the
            // installed resident's fresh transaction before rebuilding the
            // physical beginning from the retained prior-support trial.
            let prior = prior.ok_or(DirectV9RealConsumerError::OwnerClosure(
                "V2 same-support continuation custody",
            ))?;
            self.authenticate_same_support_prior_v2(
                configuration,
                prior,
                child_support_start_ns,
                child_support_end_ns,
            )?;
            if retained_trial.support_end_ns() != child_support_start_ns {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 same-support predecessor support",
                ));
            }
            openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
                &resident.owner,
                prior.original_prepared.beginning_owner().transaction_id,
                retained_trial.support_start_ns(),
                child_support_end_ns,
            )
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 same-support physical beginning authority",
                )
            })?
        } else {
            let original_support_start_ns = prior
                .map_or(retained_trial.support_start_ns(), |prior| {
                    prior.original_prepared.beginning_owner().support_start_ns
                });
            self.prepare_next_v2_support(original_support_start_ns, child_support_end_ns)?
        };
        let soil_thermal_transaction_id = original_prepared.beginning_owner().transaction_id;
        let continuation = match prior {
            Some(prior) if !same_support_prior => {
                DirectSoilThermalUnpublishedContinuationV2::try_from_result(
                    resident,
                    configuration,
                    &original_prepared,
                    prior,
                    candidate.state_sha256(),
                    child_support_start_ns,
                    child_support_end_ns,
                )?
            }
            Some(_) => DirectSoilThermalUnpublishedContinuationV2 {
                original_prepared: original_prepared.clone(),
                retained_trial: retained_trial.clone(),
                retained_accumulated_operands: Vec::new(),
                retained_layer_credit_chain: vec![retained_trial.layer_credits().to_vec()],
                child_support_start_ns,
                child_support_end_ns,
            },
            None => {
                resident.validate_prepared_beginning(original_prepared.beginning_owner())?;
                let prior_prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
                    original_prepared.beginning_owner(),
                    soil_thermal_transaction_id,
                    retained_trial.support_start_ns(),
                    retained_trial.support_end_ns(),
                )
                .map_err(|_| {
                    DirectV9RealConsumerError::OwnerClosure(
                        "V2 base unpublished physical prior support",
                    )
                })?;
                DirectSoilThermalUnpublishedContinuationV2::try_new(
                    resident,
                    configuration,
                    &original_prepared,
                    &prior_prepared,
                    retained_trial,
                    candidate.state_sha256(),
                    child_support_start_ns,
                    child_support_end_ns,
                )?
            }
        };
        openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2::try_new(
            continuation.original_prepared(),
            continuation.retained_trial(),
            soil_thermal_transaction_id,
            child_support_start_ns,
            child_support_end_ns,
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 unpublished physical beginning"))
    }

    fn authenticate_same_support_prior_v2(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
        prior: &DirectSoilThermalUnpublishedContinuationResultV2,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<(), DirectV9RealConsumerError> {
        let resident = self.v2()?;
        let fresh = self.prepare_next_v2_support(child_support_start_ns, child_support_end_ns)?;
        let original = prior.original_prepared.beginning_owner();
        let expected = fresh.beginning_owner();
        if original.owner_tag != expected.owner_tag
            || original.schema_sha256 != expected.schema_sha256
            || original.exact_carry_definition_sha256 != expected.exact_carry_definition_sha256
            || original.parent_v1_state_sha256 != expected.parent_v1_state_sha256
            || original.contract_version != expected.contract_version
            || original.model_version != expected.model_version
            || original.model_definition_sha256 != expected.model_definition_sha256
            || original.run_id != expected.run_id
            || original.transaction_id != expected.transaction_id
            || original.expected_predecessor_transaction_id
                != expected.expected_predecessor_transaction_id
            || original.receipt_chain_sha256 != expected.receipt_chain_sha256
            || original.state != expected.state
            || prior.physical_trial().support_start_ns() != child_support_start_ns
            || prior.physical_trial().support_end_ns() != child_support_end_ns
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 same-support continuation identity or support",
            ));
        }
        let authenticated = if original.support_start_ns == child_support_start_ns
            && original.support_end_ns == child_support_end_ns
        {
            DirectSoilThermalUnpublishedContinuationResultV2::try_from_base_unpublished_trial(
                resident,
                configuration,
                &fresh,
                prior.physical_trial(),
                prior.accumulated_operands(),
            )?
        } else {
            openwepp_land_surface_energy::validate_soil_thermal_unpublished_trial_v2(
                prior.physical_trial(),
            )
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure("V2 same-support continuation trial seal")
            })?;
            prior.compose_accepted_outer_candidate(configuration)?;
            prior.clone()
        };
        if authenticated != *prior {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 same-support continuation substitution",
            ));
        }
        Ok(())
    }

    pub fn validate_unpublished_fixed_point_v2(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
        candidate: &DirectSoilThermalCandidate,
        prior: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<bool, DirectV9RealConsumerError> {
        let posture =
            DirectSoilThermalUnpublishedFixedPointPostureV2::try_from_candidate(candidate)?;
        let candidate = posture.trial();
        if candidate.support_start_ns() != child_support_start_ns
            || candidate.support_end_ns() != child_support_end_ns
        {
            return Ok(false);
        }
        let Some(prior) = prior else {
            let fresh = self.prepare_next_v2_support(child_support_start_ns, child_support_end_ns)?;
            if fresh.beginning_owner().transaction_id != candidate.transaction_id() {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 same-support fixed-point transaction join",
                ));
            }
            if matches!(
                posture,
                DirectSoilThermalUnpublishedFixedPointPostureV2::NumericalCoordinateProjection(_)
            ) {
                self.validate_projected_fixed_point_v2(
                    &fresh,
                    candidate,
                    child_support_start_ns,
                    child_support_end_ns,
                )?;
                return Ok(true);
            }
            let authenticated_operands = candidate
                .layer_credits()
                .iter()
                .flat_map(|credit| credit.accepted_operands.iter().cloned())
                .collect::<Vec<_>>();
            DirectSoilThermalUnpublishedContinuationResultV2::try_from_base_unpublished_trial(
                self.v2()?,
                configuration,
                &fresh,
                candidate,
                &authenticated_operands,
            )?;
            return Ok(true);
        };
        if prior.physical_trial() != candidate {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 same-support fixed-point trial substitution",
            ));
        }
        self.authenticate_same_support_prior_v2(
            configuration,
            prior,
            child_support_start_ns,
            child_support_end_ns,
        )?;
        Ok(true)
    }

    fn validate_projected_fixed_point_v2(
        &self,
        prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<(), DirectV9RealConsumerError> {
        let resident = self.v2()?;
        resident.validate()?;
        resident.validate_prepared_beginning(prepared.beginning_owner())?;
        openwepp_land_surface_energy::validate_soil_thermal_unpublished_trial_v2(trial)
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 projected fixed-point unpublished trial seal",
                )
            })?;
        let beginning = prepared.beginning_owner();
        if trial.transaction_id() != beginning.transaction_id
            || trial.predecessor_transaction_id()
                != beginning.expected_predecessor_transaction_id
            || trial.support_start_ns() != child_support_start_ns
            || trial.support_end_ns() != child_support_end_ns
            || beginning.support_start_ns != child_support_start_ns
            || beginning.support_end_ns != child_support_end_ns
            || trial.beginning_state_sha256() != &beginning.state.state_sha256
            || trial.accepted_predecessor_receipt_chain_sha256()
                != Some(&beginning.receipt_chain_sha256)
            || trial.unpublished_predecessor_trial_sha256().is_some()
            || trial.numerical_coordinate_authority_sha256().is_none()
            || trial.numerical_coordinate_set_sha256().is_none()
            || !trial.layer_credits().is_empty()
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 projected fixed-point identity or custody",
            ));
        }
        let ending = trial.ending_state();
        if ending.owner_id != beginning.state.owner_id
            || ending.configuration_sha256 != beginning.state.configuration_sha256
            || ending.last_accepted_transaction_id
                != beginning.state.last_accepted_transaction_id
            || ending.ofes.len() != beginning.state.ofes.len()
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 projected fixed-point owner or topology",
            ));
        }
        for (beginning_ofe, ending_ofe) in beginning.state.ofes.iter().zip(&ending.ofes) {
            if beginning_ofe.ofe_id != ending_ofe.ofe_id
                || beginning_ofe.ordered_layers.len() != ending_ofe.ordered_layers.len()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 projected fixed-point OFE or layer order",
                ));
            }
            let beginning_top = beginning_ofe.ordered_layers.first().ok_or(
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 projected fixed-point missing beginning top layer",
                ),
            )?;
            let ending_top = ending_ofe.ordered_layers.first().ok_or(
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 projected fixed-point missing ending top layer",
                ),
            )?;
            if beginning_top.layer_id != ending_top.layer_id
                || beginning_top.last_accepted_transaction_id
                    != ending_top.last_accepted_transaction_id
                || !ending_top.enthalpy_hi_j_m2_ofe_ground.is_finite()
                || (ending_top.enthalpy_hi_j_m2_ofe_ground == 0.0
                    && ending_top.enthalpy_hi_j_m2_ofe_ground.to_bits()
                        != 0.0_f64.to_bits())
                || ending_top.enthalpy_carry
                    != openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
                || !ending_top.temperature_k.is_finite()
                || !(200.0..=350.0).contains(&ending_top.temperature_k)
                || beginning_ofe.ordered_layers[1..] != ending_ofe.ordered_layers[1..]
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 projected fixed-point substitution or carry",
                ));
            }
        }
        Ok(())
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

impl DirectSoilThermalUnpublishedContinuationV2 {
    fn try_new(
        resident: &DirectV10SoilThermalResidentV2,
        configuration: &LandSurfaceEnergyConfiguration,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        prior_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        retained_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        expected_retained_ending_state_sha256: &Sha256Digest,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<Self, DirectV9RealConsumerError> {
        resident.validate()?;
        resident.validate_prepared_beginning(original_prepared.beginning_owner())?;
        resident.validate_prepared_beginning(prior_prepared.beginning_owner())?;
        let original = original_prepared.beginning_owner();
        let prior = prior_prepared.beginning_owner();
        validate_unpublished_continuation_lineage(
            original,
            prior,
            child_support_start_ns,
            child_support_end_ns,
        )?;
        if retained_trial.transaction_id() != prior.transaction_id
            || retained_trial.predecessor_transaction_id()
                != prior.expected_predecessor_transaction_id
            || retained_trial.support_start_ns() != prior.support_start_ns
            || retained_trial.support_end_ns() != prior.support_end_ns
            || retained_trial.beginning_state_sha256() != &prior.state.state_sha256
            || retained_trial.accepted_predecessor_receipt_chain_sha256()
                != Some(&prior.receipt_chain_sha256)
            || &retained_trial.ending_state().state_sha256 != expected_retained_ending_state_sha256
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 retained trial identity or support",
            ));
        }
        retained_trial
            .ending_state()
            .validate()
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 retained trial ending"))?;
        let retained_operands = retained_trial
            .layer_credits()
            .iter()
            .flat_map(|credit| credit.accepted_operands.iter().cloned())
            .collect::<Vec<_>>();
        let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
            original,
            configuration,
            retained_operands,
        )?;
        let reconstructed = openwepp_land_surface_energy::advance_soil_thermal_composed_trial_v2(
            original_prepared,
            prior.support_start_ns,
            prior.support_end_ns,
            expected.accepted_operands(),
            expected.temperature_projections(),
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 retained trial reconstruction"))?;
        if &reconstructed != retained_trial {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 retained trial substitution or carry",
            ));
        }
        Ok(Self {
            original_prepared: original_prepared.clone(),
            retained_trial: retained_trial.clone(),
            retained_accumulated_operands: expected.accepted_operands().to_vec(),
            retained_layer_credit_chain: vec![retained_trial.layer_credits().to_vec()],
            child_support_start_ns,
            child_support_end_ns,
        })
    }

    fn try_from_result(
        resident: &DirectV10SoilThermalResidentV2,
        configuration: &LandSurfaceEnergyConfiguration,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        prior: &DirectSoilThermalUnpublishedContinuationResultV2,
        expected_retained_ending_state_sha256: &Sha256Digest,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<Self, DirectV9RealConsumerError> {
        resident.validate()?;
        resident.validate_prepared_beginning(original_prepared.beginning_owner())?;
        let original = original_prepared.beginning_owner();
        let prior_original = prior.original_prepared.beginning_owner();
        let trial = &prior.physical_trial;
        if original.owner_tag != prior_original.owner_tag
            || original.schema_sha256 != prior_original.schema_sha256
            || original.exact_carry_definition_sha256
                != prior_original.exact_carry_definition_sha256
            || original.parent_v1_state_sha256 != prior_original.parent_v1_state_sha256
            || original.contract_version != prior_original.contract_version
            || original.model_version != prior_original.model_version
            || original.model_definition_sha256 != prior_original.model_definition_sha256
            || original.run_id != prior_original.run_id
            || original.transaction_id != prior_original.transaction_id
            || original.expected_predecessor_transaction_id
                != prior_original.expected_predecessor_transaction_id
            || original.receipt_chain_sha256 != prior_original.receipt_chain_sha256
            || original.state != prior_original.state
            || original.support_start_ns != prior_original.support_start_ns
            || prior_original.support_end_ns != child_support_start_ns
            || child_support_end_ns != original.support_end_ns
            || &trial.ending_state().state_sha256 != expected_retained_ending_state_sha256
            || trial.support_end_ns() != prior_original.support_end_ns
            || child_support_start_ns >= child_support_end_ns
            || child_support_end_ns - child_support_start_ns
                < openwepp_land_surface_energy::MINIMUM_SUPPORT_NS
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 chained continuation identity or support",
            ));
        }
        let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
            original,
            configuration,
            prior.accumulated_operands.clone(),
        )?;
        if expected.accepted_operands() != prior.accumulated_operands {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 chained continuation accumulated custody",
            ));
        }
        openwepp_land_surface_energy::validate_soil_thermal_unpublished_trial_v2(trial).map_err(
            |_| {
                DirectV9RealConsumerError::OwnerClosure("V2 chained continuation unpublished trial")
            },
        )?;
        openwepp_land_surface_energy::compose_soil_thermal_accepted_from_unpublished_v2(
            &prior.original_prepared,
            trial,
            &prior.accumulated_operands,
            &prior.layer_credit_chain,
        )
        .map_err(|_| {
            DirectV9RealConsumerError::OwnerClosure("V2 chained continuation prior replay")
        })?;
        Ok(Self {
            original_prepared: original_prepared.clone(),
            retained_trial: trial.clone(),
            retained_accumulated_operands: prior.accumulated_operands.clone(),
            retained_layer_credit_chain: prior.layer_credit_chain.clone(),
            child_support_start_ns,
            child_support_end_ns,
        })
    }

    pub fn child_beginning_state(&self) -> &openwepp_land_surface_energy::SoilThermalOwnedStateV2 {
        self.retained_trial.ending_state()
    }

    pub const fn child_support_start_ns(&self) -> u128 {
        self.child_support_start_ns
    }

    pub const fn child_support_end_ns(&self) -> u128 {
        self.child_support_end_ns
    }

    pub fn original_prepared(&self) -> &openwepp_land_surface_energy::PreparedSoilThermalSupportV2 {
        &self.original_prepared
    }

    pub fn retained_trial(&self) -> &openwepp_land_surface_energy::SoilThermalTrialStateV2 {
        &self.retained_trial
    }

    pub fn child_top_boundary_operands_v2(
        &self,
        credits: &[SoilThermalTopBoundaryCreditV1],
        source_owner_id: &ResourceOwnerId,
    ) -> Result<
        Vec<openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2>,
        DirectV9RealConsumerError,
    > {
        let beginning = self.child_beginning_state();
        let mut identities = BTreeSet::new();
        let mut operands = Vec::with_capacity(credits.len());
        for credit in credits {
            let ofe = beginning
                .ofes
                .iter()
                .find(|ofe| ofe.ofe_id == credit.ofe_id)
                .ok_or(DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation top-boundary OFE",
                ))?;
            let layer =
                ofe.ordered_layers
                    .first()
                    .ok_or(DirectV9RealConsumerError::OwnerClosure(
                        "V2 continuation top-boundary layer",
                    ))?;
            if credit.beginning_owner_id != beginning.owner_id
                || credit.beginning_configuration_sha256 != beginning.configuration_sha256
                || credit.beginning_state_sha256 != beginning.state_sha256
                || credit.first_layer_id != layer.layer_id
                || u128::try_from(credit.support_start_ns).ok() != Some(self.child_support_start_ns)
                || u128::try_from(credit.support_end_ns).ok() != Some(self.child_support_end_ns)
                || !credit
                    .accepted_positive_downward_j_m2_ofe_ground
                    .is_finite()
                || credit.soil_thermal_credit_j_m2_ofe_ground.to_bits()
                    != credit.accepted_positive_downward_j_m2_ofe_ground.to_bits()
                || !identities.insert((credit.ofe_id.clone(), credit.lane_id))
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation top-boundary identity or support",
                ));
            }
            operands.push(
                openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
                    ofe_id: credit.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    source_kind:
                        openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::TopBoundary,
                    source_owner_id: source_owner_id.clone(),
                    debit_credit_identity_sha256: credit.snow_soil_heat_receipt_sha256.clone(),
                    ordinal: credit.lane_id,
                    units: "J m^-2 OFE-ground".to_owned(),
                    basis: "ofe_ground".to_owned(),
                    energy_j_m2_ofe_ground: credit.soil_thermal_credit_j_m2_ofe_ground,
                },
            );
        }
        canonicalize_v2_operand_order(self.original_prepared.beginning_owner(), &mut operands)?;
        Ok(operands)
    }

    fn advance_sequential(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
        child_operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
    ) -> Result<DirectSoilThermalUnpublishedContinuationResultV2, DirectV9RealConsumerError> {
        let mut physical_operands = child_operands.to_vec();
        reordinal_and_canonicalize_v2_operands(
            self.original_prepared.beginning_owner(),
            &mut physical_operands,
        )?;
        let projections = v2_temperature_projections_for_unpublished_state(
            self.retained_trial.ending_state(),
            configuration,
            &physical_operands,
        )?;
        let physical_trial =
            openwepp_land_surface_energy::advance_soil_thermal_sequential_unpublished_trial_v2(
                &self.retained_trial,
                self.child_support_start_ns,
                self.child_support_end_ns,
                &physical_operands,
                &projections,
            )
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure("V2 sequential continuation trial")
            })?;
        let mut accumulated_operands = self
            .retained_accumulated_operands
            .iter()
            .cloned()
            .chain(physical_operands)
            .collect::<Vec<_>>();
        reordinal_and_canonicalize_v2_operands(
            self.original_prepared.beginning_owner(),
            &mut accumulated_operands,
        )?;
        Ok(DirectSoilThermalUnpublishedContinuationResultV2 {
            original_prepared: self.original_prepared.clone(),
            layer_credit_chain: self
                .retained_layer_credit_chain
                .iter()
                .cloned()
                .chain(std::iter::once(physical_trial.layer_credits().to_vec()))
                .collect(),
            physical_trial,
            accumulated_operands,
        })
    }
}

const V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY: &str =
    "V2 unpublished continuation immutable owner/schema/model/run/state identity";
const V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE: &str =
    "V2 unpublished continuation transaction/predecessor/receipt lineage";
const V2_UNPUBLISHED_CONTINUATION_SUPPORT_START: &str =
    "V2 unpublished continuation original/prior support start";
const V2_UNPUBLISHED_CONTINUATION_PRIOR_END: &str =
    "V2 unpublished continuation prior end/child start";
const V2_UNPUBLISHED_CONTINUATION_OUTER_END: &str =
    "V2 unpublished continuation child/original support end";
const V2_UNPUBLISHED_CONTINUATION_WIDTH: &str =
    "V2 unpublished continuation child support width/floor";

fn validate_unpublished_continuation_lineage(
    original: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    prior: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    child_support_start_ns: u128,
    child_support_end_ns: u128,
) -> Result<(), DirectV9RealConsumerError> {
    if original.owner_tag != prior.owner_tag {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.schema_sha256 != prior.schema_sha256 {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.exact_carry_definition_sha256 != prior.exact_carry_definition_sha256 {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.parent_v1_state_sha256 != prior.parent_v1_state_sha256 {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.contract_version != prior.contract_version {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.model_version != prior.model_version {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.model_definition_sha256 != prior.model_definition_sha256 {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.run_id != prior.run_id {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if original.transaction_id != prior.transaction_id {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE,
        ));
    }
    if original.expected_predecessor_transaction_id != prior.expected_predecessor_transaction_id {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE,
        ));
    }
    if original.receipt_chain_sha256 != prior.receipt_chain_sha256 {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_TRANSACTION_LINEAGE,
        ));
    }
    if original.state != prior.state {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_IMMUTABLE_IDENTITY,
        ));
    }
    if prior.support_start_ns != original.support_start_ns {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_SUPPORT_START,
        ));
    }
    if prior.support_end_ns != child_support_start_ns {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_PRIOR_END,
        ));
    }
    if child_support_end_ns != original.support_end_ns {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_OUTER_END,
        ));
    }
    if child_support_start_ns >= child_support_end_ns {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_WIDTH,
        ));
    }
    if child_support_end_ns - child_support_start_ns
        < openwepp_land_surface_energy::MINIMUM_SUPPORT_NS
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            V2_UNPUBLISHED_CONTINUATION_WIDTH,
        ));
    }
    Ok(())
}

impl DirectSoilThermalUnpublishedContinuationResultV2 {
    fn try_from_base_unpublished_trial(
        resident: &DirectV10SoilThermalResidentV2,
        configuration: &LandSurfaceEnergyConfiguration,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        retained_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        authenticated_operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
    ) -> Result<Self, DirectV9RealConsumerError> {
        resident.validate()?;
        resident.validate_prepared_beginning(original_prepared.beginning_owner())?;
        openwepp_land_surface_energy::validate_soil_thermal_unpublished_trial_v2(retained_trial)
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure("V2 base unpublished result trial seal")
            })?;
        let original = original_prepared.beginning_owner();
        if retained_trial.transaction_id() != original.transaction_id
            || retained_trial.predecessor_transaction_id()
                != original.expected_predecessor_transaction_id
            || retained_trial.support_start_ns() != original.support_start_ns
            || retained_trial.support_end_ns() > original.support_end_ns
            || retained_trial.beginning_state_sha256() != &original.state.state_sha256
            || retained_trial.accepted_predecessor_receipt_chain_sha256()
                != Some(&original.receipt_chain_sha256)
            || retained_trial
                .unpublished_predecessor_trial_sha256()
                .is_some()
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 base unpublished result identity or support",
            ));
        }
        let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
            original,
            configuration,
            authenticated_operands.to_vec(),
        )?;
        if expected.accepted_operands() != authenticated_operands {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 base unpublished result operand order",
            ));
        }
        let reconstructed = openwepp_land_surface_energy::advance_soil_thermal_composed_trial_v2(
            original_prepared,
            retained_trial.support_start_ns(),
            retained_trial.support_end_ns(),
            expected.accepted_operands(),
            expected.temperature_projections(),
        )
        .map_err(|_| {
            DirectV9RealConsumerError::OwnerClosure("V2 base unpublished result reconstruction")
        })?;
        if &reconstructed != retained_trial {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 base unpublished result substitution or carry",
            ));
        }
        let base_prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            original,
            original.transaction_id,
            retained_trial.support_start_ns(),
            retained_trial.support_end_ns(),
        )
        .map_err(|_| {
            DirectV9RealConsumerError::OwnerClosure("V2 base unpublished result support")
        })?;
        resident.validate_prepared_beginning(base_prepared.beginning_owner())?;
        Ok(Self {
            original_prepared: base_prepared,
            physical_trial: retained_trial.clone(),
            accumulated_operands: expected.accepted_operands().to_vec(),
            layer_credit_chain: vec![retained_trial.layer_credits().to_vec()],
        })
    }

    pub fn original_prepared(&self) -> &openwepp_land_surface_energy::PreparedSoilThermalSupportV2 {
        &self.original_prepared
    }

    pub fn physical_trial(&self) -> &openwepp_land_surface_energy::SoilThermalTrialStateV2 {
        &self.physical_trial
    }

    pub fn accumulated_operands(
        &self,
    ) -> &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2] {
        &self.accumulated_operands
    }

    pub fn compose_accepted_outer_candidate(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<SoilThermalAcceptedCandidateV2, DirectV9RealConsumerError> {
        let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
            self.original_prepared.beginning_owner(),
            configuration,
            self.accumulated_operands.clone(),
        )?;
        let candidate =
            openwepp_land_surface_energy::compose_soil_thermal_accepted_from_unpublished_v2(
                &self.original_prepared,
                &self.physical_trial,
                expected.accepted_operands(),
                &self.layer_credit_chain,
            )
            .map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation accepted outer composition",
                )
            })?;
        Ok(SoilThermalAcceptedCandidateV2 {
            ending_owner: candidate.ending_owner,
            credit_receipt: candidate.credit_receipt,
            expected_sources: expected,
        })
    }

    /// Validate the independently reconstructed terminal operand image as the
    /// exact per-layer/source suffix of the authenticated parent accumulation.
    /// Local child ordinals restart at zero; parent ordinals retain the
    /// checked number of earlier operands in that same canonical group.
    pub fn validate_terminal_operand_suffix(
        &self,
        reconstructed: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
    ) -> Result<(), DirectV9RealConsumerError> {
        let mut canonical = reconstructed.to_vec();
        canonicalize_v2_operand_order(self.original_prepared.beginning_owner(), &mut canonical)?;
        if canonical != reconstructed || reconstructed.is_empty() {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 continuation terminal operand canonical order",
            ));
        }
        let retained_groups = v2_operand_groups(&self.accumulated_operands);
        let reconstructed_groups = v2_operand_groups(reconstructed);
        let mut retained_index = 0;
        for reconstructed in reconstructed_groups {
            let reconstructed_key = reconstructed.first().map(v2_operand_group_key);
            let Some(relative_index) =
                retained_groups[retained_index..]
                    .iter()
                    .position(|retained| {
                        retained.first().map(v2_operand_group_key) == reconstructed_key
                    })
            else {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation terminal operand suffix",
                ));
            };
            retained_index += relative_index;
            let retained = retained_groups[retained_index];
            retained_index += 1;
            if reconstructed.len() > retained.len() {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation terminal operand suffix",
                ));
            }
            let prefix_len = retained.len() - reconstructed.len();
            let prefix_ordinal = u32::try_from(prefix_len).map_err(|_| {
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation terminal operand prefix overflow",
                )
            })?;
            for (index, (retained, reconstructed)) in retained[prefix_len..]
                .iter()
                .zip(reconstructed.iter())
                .enumerate()
            {
                let local_ordinal = u32::try_from(index).map_err(|_| {
                    DirectV9RealConsumerError::OwnerClosure(
                        "V2 continuation terminal operand ordinal overflow",
                    )
                })?;
                let parent_ordinal = prefix_ordinal.checked_add(local_ordinal).ok_or(
                    DirectV9RealConsumerError::OwnerClosure(
                        "V2 continuation terminal operand ordinal overflow",
                    ),
                )?;
                if reconstructed.ordinal != local_ordinal
                    || retained.ordinal != parent_ordinal
                    || !v2_operand_matches_except_ordinal(retained, reconstructed)
                {
                    return Err(DirectV9RealConsumerError::OwnerClosure(
                        "V2 continuation terminal operand suffix",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn into_physical_candidate(
        self,
    ) -> Result<DirectSoilThermalCandidate, DirectV9RealConsumerError> {
        DirectSoilThermalCandidate::from_v2(self.physical_trial)
    }

    fn validate_selected_accepted_child(
        &self,
        candidate_resident: &DirectV10SoilThermalResidentV2,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: &SoilThermalAcceptedCandidateV2,
    ) -> Result<(), DirectV9RealConsumerError> {
        candidate_resident.validate()?;
        candidate_resident.validate_prepared_beginning(self.original_prepared.beginning_owner())?;
        self.validate_selected_accepted_child_without_resident(prepared_beginning, accepted)
    }

    fn validate_selected_accepted_child_without_resident(
        &self,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: &SoilThermalAcceptedCandidateV2,
    ) -> Result<(), DirectV9RealConsumerError> {
        openwepp_land_surface_energy::validate_soil_thermal_unpublished_trial_v2(
            &self.physical_trial,
        )
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 selected unpublished trial"))?;
        let original = self.original_prepared.beginning_owner();
        let receipt = &accepted.credit_receipt;
        let physical_ending = self.physical_trial.ending_state();
        let accepted_ending = &accepted.ending_owner.state;
        let predicates = [
            prepared_beginning == original,
            !self
                .physical_trial
                .accepted_predecessor_receipt_chain_sha256()
                .is_some(),
            self.physical_trial
                .unpublished_predecessor_trial_sha256()
                .is_some(),
            self.physical_trial.support_end_ns() == original.support_end_ns,
            receipt.transaction_id == original.transaction_id,
            receipt.predecessor_transaction_id == original.expected_predecessor_transaction_id,
            receipt.support_start_ns == original.support_start_ns,
            receipt.support_end_ns == original.support_end_ns,
            receipt.beginning_owner_state_sha256 == original.state.state_sha256,
            receipt.predecessor_receipt_chain_sha256 == original.receipt_chain_sha256,
            accepted.expected_sources.accepted_operands() == self.accumulated_operands,
            soil_thermal_v2_physical_ending_matches(physical_ending, accepted_ending),
        ];
        if predicates.into_iter().any(|predicate| !predicate) {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 selected continuation final replay join",
            ));
        }
        Ok(())
    }
}

fn v2_operand_groups(
    operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Vec<&[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2]> {
    let mut groups = Vec::new();
    let mut start = 0;
    while start < operands.len() {
        let mut end = start + 1;
        while end < operands.len()
            && v2_operand_group_key(&operands[start]) == v2_operand_group_key(&operands[end])
        {
            end += 1;
        }
        groups.push(&operands[start..end]);
        start = end;
    }
    groups
}

fn v2_operand_group_key(
    operand: &openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2,
) -> (
    &OfeId,
    &SoilLayerId,
    openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2,
) {
    (&operand.ofe_id, &operand.layer_id, operand.source_kind)
}

fn v2_operand_matches_except_ordinal(
    left: &openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2,
    right: &openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2,
) -> bool {
    left.ofe_id == right.ofe_id
        && left.layer_id == right.layer_id
        && left.source_kind == right.source_kind
        && left.source_owner_id == right.source_owner_id
        && left.debit_credit_identity_sha256 == right.debit_credit_identity_sha256
        && left.units == right.units
        && left.basis == right.basis
        && left.energy_j_m2_ofe_ground.to_bits() == right.energy_j_m2_ofe_ground.to_bits()
}

fn soil_thermal_v2_physical_ending_matches(
    left: &openwepp_land_surface_energy::SoilThermalOwnedStateV2,
    right: &openwepp_land_surface_energy::SoilThermalOwnedStateV2,
) -> bool {
    left.owner_id == right.owner_id
        && left.configuration_sha256 == right.configuration_sha256
        && left.ofes.len() == right.ofes.len()
        && left.ofes.iter().zip(&right.ofes).all(|(left, right)| {
            left.ofe_id == right.ofe_id
                && left.ordered_layers.len() == right.ordered_layers.len()
                && left
                    .ordered_layers
                    .iter()
                    .zip(&right.ordered_layers)
                    .all(|(left, right)| {
                        left.layer_id == right.layer_id
                            && left.temperature_k.to_bits() == right.temperature_k.to_bits()
                            && left.enthalpy_hi_j_m2_ofe_ground.to_bits()
                                == right.enthalpy_hi_j_m2_ofe_ground.to_bits()
                            && left.enthalpy_carry == right.enthalpy_carry
                    })
        })
}

fn reordinal_and_canonicalize_v2_operands(
    topology: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    operands: &mut [openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Result<(), DirectV9RealConsumerError> {
    let mut ordinals = BTreeMap::new();
    for operand in operands.iter_mut() {
        let ordinal = ordinals
            .entry((
                operand.ofe_id.clone(),
                operand.layer_id.clone(),
                operand.source_kind,
            ))
            .or_insert(0_u32);
        operand.ordinal = *ordinal;
        *ordinal = ordinal
            .checked_add(1)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "V2 continuation operand ordinal overflow",
            ))?;
    }
    canonicalize_v2_operand_order(topology, operands)
}

fn v2_temperature_projections_for_unpublished_state(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnedStateV2,
    configuration: &LandSurfaceEnergyConfiguration,
    operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
) -> Result<
    Vec<openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2>,
    DirectV9RealConsumerError,
> {
    beginning
        .validate()
        .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 continuation beginning state"))?;
    if beginning.configuration_sha256
        != configuration
            .soil_thermal_configuration
            .configuration_sha256
        || beginning.ofes.len() != configuration.ofes.len()
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "V2 continuation configuration",
        ));
    }
    let mut projections = Vec::new();
    for (owner_ofe, configured_ofe) in beginning.ofes.iter().zip(&configuration.ofes) {
        if owner_ofe.ofe_id != configured_ofe.ofe_id
            || owner_ofe.ordered_layers.len() != configured_ofe.soil_interface_layers.len()
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "V2 continuation OFE topology",
            ));
        }
        for (layer, configured_layer) in owner_ofe
            .ordered_layers
            .iter()
            .zip(&configured_ofe.soil_interface_layers)
        {
            if layer.layer_id != configured_layer.layer_id {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 continuation layer topology",
                ));
            }
            let values = operands
                .iter()
                .filter(|operand| {
                    operand.ofe_id == owner_ofe.ofe_id && operand.layer_id == layer.layer_id
                })
                .map(|operand| operand.energy_j_m2_ofe_ground)
                .collect::<Vec<_>>();
            let total = openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum_binary64(
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                &values,
            )
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 continuation exact sum"))?;
            let (ending_high, ending_carry) = if values.is_empty() {
                (
                    layer.enthalpy_hi_j_m2_ofe_ground,
                    layer.enthalpy_carry.clone(),
                )
            } else {
                total.rounded_high_and_remainder().map_err(|_| {
                    DirectV9RealConsumerError::OwnerClosure("V2 continuation rounding")
                })?
            };
            let ending_temperature_k = openwepp_land_surface_energy::project_soil_temperature_k(
                layer.temperature_k,
                configured_layer.areal_heat_capacity_j_m2_k,
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                ending_high,
                &ending_carry,
            )
            .map_err(|_| DirectV9RealConsumerError::OwnerClosure("V2 continuation temperature"))?;
            projections.push(
                openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2 {
                    ofe_id: owner_ofe.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    heat_capacity_j_m2_k: configured_layer.areal_heat_capacity_j_m2_k,
                    ending_temperature_k,
                },
            );
        }
    }
    Ok(projections)
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
            || beginning.model_version != self.owner.model_version
            || beginning.model_definition_sha256 != self.owner.model_definition_sha256
            || beginning.run_id != self.owner.run_id
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
        if self.receipt_free_seals.is_some() {
            if beginning.transaction_id != self.owner.transaction_id
                || beginning.support_start_ns < self.owner.support_start_ns
                || beginning.support_end_ns > self.owner.support_end_ns
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "receipt-free V2 prepared support join",
                ));
            }
        } else {
            let next_transaction = self
                .owner
                .transaction_id
                .0
                .checked_add(1)
                .map(TransactionId)
                .ok_or(DirectV9RealConsumerError::OwnerClosure(
                    "accepted V2 prepared transaction overflow",
                ))?;
            if beginning.transaction_id != next_transaction
                || beginning.support_start_ns != self.owner.support_end_ns
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "accepted V2 prepared successor join",
                ));
            }
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

    fn is_exact_accepted_candidate(
        &self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        candidate: &SoilThermalAcceptedCandidateV2,
        seals: &SoilThermalOrchestratorSealsV2,
    ) -> Result<bool, DirectV9RealConsumerError> {
        Ok(
            self.is_exact_accepted_candidate_without_external_seals(beginning, candidate)?
                && self
                    .latest_accepted
                    .as_ref()
                    .is_some_and(|custody| custody.seals == *seals),
        )
    }

    fn is_exact_accepted_candidate_without_external_seals(
        &self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        candidate: &SoilThermalAcceptedCandidateV2,
    ) -> Result<bool, DirectV9RealConsumerError> {
        self.validate()?;
        Ok(self.receipt_free_seals.is_none()
            && self.owner == candidate.ending_owner
            && self.latest_accepted.as_ref().is_some_and(|custody| {
                custody.predecessor == *beginning
                    && custody.credit_receipt == candidate.credit_receipt
                    && custody.expected_sources == candidate.expected_sources
            }))
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

    pub fn prepare_next_soil_thermal_support_v2(
        &self,
        support_start_ns: u128,
        support_end_ns: u128,
    ) -> Result<
        openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        DirectV10RealConsumerError,
    > {
        self.inner
            .soil_thermal
            .prepare_next_v2_support(support_start_ns, support_end_ns)
            .map_err(Into::into)
    }

    /// Authenticate an unpublished sequential ending as the constitutive
    /// beginning for the next child support. The returned value is read-only
    /// physical custody: it cannot seal, accept, install, or publish an owner.
    pub fn prepare_soil_thermal_unpublished_physical_beginning_v2(
        &self,
        candidate: &DirectSoilThermalCandidate,
        prior: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<
        openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2,
        DirectV10RealConsumerError,
    > {
        let retained_trial = candidate.v2()?;
        let active_owner = self.soil_thermal_v2()?.owner();
        let original_prepared = self.prepare_next_soil_thermal_support_v2(
            active_owner.support_start_ns,
            child_support_end_ns,
        )?;
        let soil_thermal_transaction_id = original_prepared.beginning_owner().transaction_id;
        let continuation = match prior {
            Some(prior) => self.prepare_next_soil_thermal_unpublished_continuation_v2(
                &original_prepared,
                prior,
                candidate.state_sha256(),
                child_support_start_ns,
                child_support_end_ns,
            )?,
            None => self.prepare_soil_thermal_base_unpublished_continuation_v2(
                &original_prepared,
                retained_trial,
                candidate.state_sha256(),
                child_support_start_ns,
                child_support_end_ns,
            )?,
        };
        openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2::try_new(
            continuation.original_prepared(),
            continuation.retained_trial(),
            soil_thermal_transaction_id,
            child_support_start_ns,
            child_support_end_ns,
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::OwnerClosure(
                "V2 unpublished physical beginning",
            ))
        })
    }

    pub fn prepare_soil_thermal_unpublished_continuation_v2(
        &self,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        prior_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        retained_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        expected_retained_ending_state_sha256: &Sha256Digest,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<DirectSoilThermalUnpublishedContinuationV2, DirectV10RealConsumerError> {
        DirectSoilThermalUnpublishedContinuationV2::try_new(
            self.inner.soil_thermal.v2()?,
            &self.inner.lse_configuration,
            original_prepared,
            prior_prepared,
            retained_trial,
            expected_retained_ending_state_sha256,
            child_support_start_ns,
            child_support_end_ns,
        )
        .map_err(Into::into)
    }

    /// Prepare the first sequential continuation from an authenticated
    /// resident and its exact prepared successor support. The retained trial
    /// remains unpublished and is never installed as an accepted resident.
    pub fn prepare_soil_thermal_base_unpublished_continuation_v2(
        &self,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        retained_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        expected_retained_ending_state_sha256: &Sha256Digest,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<DirectSoilThermalUnpublishedContinuationV2, DirectV10RealConsumerError> {
        let resident = self.inner.soil_thermal.v2()?;
        resident.validate()?;
        resident.validate_prepared_beginning(original_prepared.beginning_owner())?;
        let prior_prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            original_prepared.beginning_owner(),
            original_prepared.beginning_owner().transaction_id,
            retained_trial.support_start_ns(),
            retained_trial.support_end_ns(),
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::OwnerClosure(
                "V2 base unpublished prior support",
            ))
        })?;
        DirectSoilThermalUnpublishedContinuationV2::try_new(
            resident,
            &self.inner.lse_configuration,
            original_prepared,
            &prior_prepared,
            retained_trial,
            expected_retained_ending_state_sha256,
            child_support_start_ns,
            child_support_end_ns,
        )
        .map_err(Into::into)
    }

    /// Authenticate the first unpublished native-V2 child as opaque transient
    /// custody without advancing another child or creating an accepted owner.
    ///
    /// `authenticated_operands` must come from the enclosing physical
    /// envelope reconstruction. Receipt-contained operands are not an
    /// independent authority for this constructor.
    pub fn authenticate_soil_thermal_base_unpublished_result_v2(
        &self,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        retained_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        authenticated_operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
    ) -> Result<DirectSoilThermalUnpublishedContinuationResultV2, DirectV10RealConsumerError> {
        DirectSoilThermalUnpublishedContinuationResultV2::try_from_base_unpublished_trial(
            self.inner.soil_thermal.v2()?,
            &self.inner.lse_configuration,
            original_prepared,
            retained_trial,
            authenticated_operands,
        )
        .map_err(Into::into)
    }

    pub fn prepare_next_soil_thermal_unpublished_continuation_v2(
        &self,
        original_prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        prior: &DirectSoilThermalUnpublishedContinuationResultV2,
        expected_retained_ending_state_sha256: &Sha256Digest,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
    ) -> Result<DirectSoilThermalUnpublishedContinuationV2, DirectV10RealConsumerError> {
        DirectSoilThermalUnpublishedContinuationV2::try_from_result(
            self.inner.soil_thermal.v2()?,
            &self.inner.lse_configuration,
            original_prepared,
            prior,
            expected_retained_ending_state_sha256,
            child_support_start_ns,
            child_support_end_ns,
        )
        .map_err(Into::into)
    }

    pub fn advance_soil_thermal_unpublished_continuation_v2(
        &self,
        continuation: &DirectSoilThermalUnpublishedContinuationV2,
        child_operands: &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
    ) -> Result<DirectSoilThermalUnpublishedContinuationResultV2, DirectV10RealConsumerError> {
        let resident = self.inner.soil_thermal.v2()?;
        resident.validate_prepared_beginning(continuation.original_prepared.beginning_owner())?;
        continuation
            .advance_sequential(&self.inner.lse_configuration, child_operands)
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

    /// Build one private sibling numerical image from the exact accepted V2
    /// beginning. The returned candidate is read-only carrier input; its
    /// dedicated custody tag cannot be advanced, accepted, or installed.
    pub fn project_soil_thermal_unpublished_coordinates_v2(
        &self,
        prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        coordinates: &[openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2],
    ) -> Result<DirectSoilThermalCandidate, DirectV10RealConsumerError> {
        self.soil_thermal_v2()?
            .validate_prepared_beginning(prepared.beginning_owner())?;
        let projection =
            openwepp_land_surface_energy::project_soil_thermal_unpublished_coordinates_v2(
                prepared,
                coordinates,
            )
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::OwnerClosure(
                    "V2 numerical coordinate projection",
                ))
            })?;
        DirectSoilThermalCandidate::from_v2(projection.into_trial()).map_err(Into::into)
    }

    /// Project one private top-layer coordinate per OFE while retaining every
    /// lower native-V2 layer bit-exact from the authenticated beginning.
    pub fn project_soil_thermal_unpublished_top_layer_coordinates_v2(
        &self,
        prepared: &openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        coordinates: &[openwepp_land_surface_energy::SoilThermalUnpublishedCoordinateV2],
    ) -> Result<DirectSoilThermalCandidate, DirectV10RealConsumerError> {
        self.soil_thermal_v2()?
            .validate_prepared_beginning(prepared.beginning_owner())?;
        let projection =
            openwepp_land_surface_energy::project_soil_thermal_unpublished_top_layer_coordinates_v2(
                prepared,
                coordinates,
            )
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::OwnerClosure(
                    "V2 top-layer numerical coordinate projection",
                ))
            })?;
        DirectSoilThermalCandidate::from_v2(projection.into_trial()).map_err(Into::into)
    }

    pub fn install_soil_thermal_accepted_v2(
        &mut self,
        beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let accepted_resident = self
            .inner
            .soil_thermal
            .v2()?
            .accepted(beginning, accepted, seals)?;
        self.install_validated_soil_thermal_resident_v2(accepted_resident, None, false)
    }

    /// Install one accepted V2 soil owner constructed exclusively from the
    /// authoritative beginning host. A precomputed candidate may already
    /// retain that exact accepted resident; that case is independently
    /// validated and remains a byte-exact no-op rather than a second install.
    pub fn install_soil_thermal_accepted_v2_from_beginning(
        &mut self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        let accepted_resident =
            authoritative_resident.accepted(prepared_beginning, accepted, seals)?;
        let candidate_resident = self.inner.soil_thermal.v2()?;
        candidate_resident.validate()?;
        if candidate_resident != authoritative_resident && candidate_resident != &accepted_resident
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 candidate resident beginning-or-ending join",
                ),
            ));
        }
        self.install_validated_soil_thermal_resident_v2(accepted_resident, None, true)
    }

    /// Construct the explicit native-V2 source/target authority for one
    /// accepted result derived from an authenticated prepared beginning.
    /// Generic/public install remains same-ID-only and cannot call this seam.
    pub fn authenticate_soil_thermal_prepared_beginning_install_authority_v2(
        &self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    ) -> Result<
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
        DirectV10RealConsumerError,
    > {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        authoritative_resident.validate_prepared_beginning(prepared_beginning)?;
        let source_transaction_id =
            direct_soil_thermal_complete_source_transaction_v2(self).map_err(|error| {
                DirectV10RealConsumerError::Runtime(error)
            })?;
        if source_transaction_id != prepared_beginning.transaction_id
            && prepared_beginning.expected_predecessor_transaction_id
                != Some(source_transaction_id)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 prepared-beginning install predecessor transaction authority",
                ),
            ));
        }
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            source_transaction_id,
            prepared_beginning.transaction_id,
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "V2 prepared-beginning install transaction authority",
            ))
        })
    }

    /// Install one accepted V2 ending derived from an exact authenticated
    /// prepared beginning while retaining distinct outer-source and soil-target
    /// transaction custody. No unpublished result is published by this method.
    #[allow(clippy::too_many_arguments)]
    pub fn install_soil_thermal_accepted_v2_from_authenticated_beginning(
        &mut self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        transaction_authority:
            crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let (accepted_resident, exact_accepted_noop) = self
            .validated_authenticated_prepared_accepted_resident_v2(
                authoritative_beginning,
                prepared_beginning,
                accepted,
                seals,
                "V2 authenticated prepared candidate beginning-or-ending join",
            )?;
        let expected_transaction_authority = self
            .authenticate_soil_thermal_prepared_beginning_install_authority_v2(
                authoritative_beginning,
                prepared_beginning,
            )?;
        if transaction_authority != expected_transaction_authority {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 prepared-beginning install explicit transaction authority",
                ),
            ));
        }
        self.install_validated_soil_thermal_resident_v2(
            accepted_resident,
            Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                transaction_authority,
            )),
            exact_accepted_noop,
        )
    }

    /// Construct the opaque three-domain authority for a prepared native-V2
    /// soil successor. The complete outer source, exact authenticated resident
    /// predecessor, and exact prepared target remain distinct identities; no
    /// numeric adjacency supplies authority.
    pub fn authenticate_soil_thermal_prepared_beginning_install_authority_v3(
        &self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    ) -> Result<DirectSoilThermalPreparedBeginningInstallAuthorityV2, DirectV10RealConsumerError>
    {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        authoritative_resident.validate_prepared_beginning(prepared_beginning)?;
        let source_transaction_id =
            direct_soil_thermal_complete_source_transaction_v2(self).map_err(|error| {
                DirectV10RealConsumerError::Runtime(error)
            })?;
        let authoritative_source_transaction_id =
            direct_soil_thermal_complete_source_transaction_v2(authoritative_beginning)
                .map_err(DirectV10RealConsumerError::Runtime)?;
        if source_transaction_id != authoritative_source_transaction_id {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 multi-child prepared-beginning authoritative source join",
                ),
            ));
        }
        let physical_transaction_authority = crate::land_surface_energy_shadow::
            PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                source_transaction_id,
                prepared_beginning.transaction_id,
            )
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "V2 multi-child prepared-beginning transaction authority",
                ))
            })?;
        Ok(DirectSoilThermalPreparedBeginningInstallAuthorityV2 {
            physical_transaction_authority,
            source_authority:
                DirectSoilThermalPreparedBeginningSourceAuthorityV2::AuthenticatedBeginning(
                    authoritative_source_transaction_id,
                ),
            authoritative_resident: authoritative_resident.clone(),
            prepared_beginning: prepared_beginning.clone(),
        })
    }

    /// Install one accepted native-V2 ending after independently
    /// reconstructing its exact outer-source/resident-predecessor/target
    /// authority. Generic and V48 installation retain their stricter postures.
    #[allow(clippy::too_many_arguments)]
    pub fn install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
        &mut self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        authority: DirectSoilThermalPreparedBeginningInstallAuthorityV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let (accepted_resident, exact_accepted_noop) = self
            .validated_authenticated_prepared_accepted_resident_v2(
                authoritative_beginning,
                prepared_beginning,
                accepted,
                seals,
                "V2 multi-child prepared candidate beginning-or-ending join",
            )?;
        let expected_authority = self
            .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
                authoritative_beginning,
                prepared_beginning,
            )?;
        if authority != expected_authority {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 multi-child prepared-beginning explicit authority",
                ),
            ));
        }
        self.install_validated_soil_thermal_resident_v2(
            accepted_resident,
            Some(
                DirectSoilThermalAtomicInstallAuthorityV2::AuthenticatedPreparedBeginning(
                    &authority,
                ),
            ),
            exact_accepted_noop,
        )
    }

    /// Construct the V50 outer-transition authority for a native-V2 prepared
    /// successor. The constitutive beginning may carry heterogeneous source
    /// owner transactions; only the separately reconstructed covered-V8
    /// ending proves the candidate's complete non-soil source.
    pub fn authenticate_soil_thermal_prepared_beginning_install_authority_v4(
        &self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        outer_owner_transition_authority: &DirectSoilThermalOuterOwnerTransitionAuthorityV2,
    ) -> Result<DirectSoilThermalPreparedBeginningInstallAuthorityV2, DirectV10RealConsumerError>
    {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        authoritative_resident.validate_prepared_beginning(prepared_beginning)?;
        let source_transaction_id =
            direct_soil_thermal_complete_source_transaction_v2(self)
                .map_err(DirectV10RealConsumerError::Runtime)?;
        let (authenticated_source_transaction_id, expected_non_soil_ending) =
            outer_owner_transition_authority
                .validated_source_and_expected()
                .map_err(DirectV10RealConsumerError::Runtime)?;
        if source_transaction_id != authenticated_source_transaction_id
            || !direct_soil_thermal_non_soil_ending_matches_v2(self, expected_non_soil_ending)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 multi-child prepared-beginning envelope source join",
                ),
            ));
        }
        let physical_transaction_authority = crate::land_surface_energy_shadow::
            PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                source_transaction_id,
                prepared_beginning.transaction_id,
            )
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "V2 multi-child prepared-beginning transaction authority",
                ))
            })?;
        Ok(DirectSoilThermalPreparedBeginningInstallAuthorityV2 {
            physical_transaction_authority,
            source_authority:
                DirectSoilThermalPreparedBeginningSourceAuthorityV2::ValidatedOuterTransition(
                    outer_owner_transition_authority.clone(),
                ),
            authoritative_resident: authoritative_resident.clone(),
            prepared_beginning: prepared_beginning.clone(),
        })
    }

    /// Install a V50 native-V2 ending using the opaque validated covered-V8
    /// outer-owner transition and the exact authenticated soil predecessor.
    #[allow(clippy::too_many_arguments)]
    pub fn install_soil_thermal_accepted_v2_from_authenticated_beginning_v4(
        &mut self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        outer_owner_transition_authority: DirectSoilThermalOuterOwnerTransitionAuthorityV2,
        authority: DirectSoilThermalPreparedBeginningInstallAuthorityV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let (accepted_resident, exact_accepted_noop) = self
            .validated_authenticated_prepared_accepted_resident_v2(
                authoritative_beginning,
                prepared_beginning,
                accepted,
                seals,
                "V2 envelope-authorized prepared candidate beginning-or-ending join",
            )?;
        let expected_authority = self
            .authenticate_soil_thermal_prepared_beginning_install_authority_v4(
                authoritative_beginning,
                prepared_beginning,
                &outer_owner_transition_authority,
            )?;
        if authority != expected_authority {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 envelope-authorized prepared-beginning explicit authority",
                ),
            ));
        }
        self.install_validated_soil_thermal_resident_v2(
            accepted_resident,
            Some(
                DirectSoilThermalAtomicInstallAuthorityV2::AuthenticatedPreparedBeginning(
                    &authority,
                ),
            ),
            exact_accepted_noop,
        )
    }

    pub(crate) fn authenticate_soil_thermal_outer_owner_transition_v2(
        &self,
        authenticated_transition: &v11_covered::AuthenticatedCoveredV8OuterOwnerTransitionV1,
    ) -> Result<DirectSoilThermalOuterOwnerTransitionAuthorityV2, DirectV10RealConsumerError> {
        authenticated_transition.envelope().validate().map_err(|error| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::from(error))
        })?;
        let authenticated_complete_owner_source_transaction_id =
            authenticated_transition.envelope().transaction_id();
        let expected_non_soil_ending = authenticated_transition.expected_non_soil_ending();
        let candidate_source = direct_soil_thermal_complete_source_transaction_v2(self)
            .map_err(DirectV10RealConsumerError::Runtime)?;
        let expected_source =
            direct_soil_thermal_complete_source_transaction_v2(expected_non_soil_ending)
                .map_err(DirectV10RealConsumerError::Runtime)?;
        if candidate_source != authenticated_complete_owner_source_transaction_id
            || expected_source != authenticated_complete_owner_source_transaction_id
            || !direct_soil_thermal_non_soil_ending_matches_v2(self, expected_non_soil_ending)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 authenticated complete-owner source transition",
                ),
            ));
        }
        Ok(DirectSoilThermalOuterOwnerTransitionAuthorityV2 {
            authenticated_transition:
                DirectSoilThermalOuterOwnerTransitionSourceV2::ValidatedEnvelope(
                    authenticated_transition.clone(),
                ),
        })
    }

    fn validated_authenticated_prepared_accepted_resident_v2(
        &self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
        candidate_join_error: &'static str,
    ) -> Result<(DirectV10SoilThermalResidentV2, bool), DirectV10RealConsumerError> {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        let accepted_resident =
            authoritative_resident.accepted(prepared_beginning, accepted, seals)?;
        let candidate_resident = self.inner.soil_thermal.v2()?;
        candidate_resident.validate()?;
        if candidate_resident != authoritative_resident && candidate_resident != &accepted_resident
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::OwnerClosure(candidate_join_error),
            ));
        }
        let exact_accepted_noop = candidate_resident == &accepted_resident;
        Ok((accepted_resident, exact_accepted_noop))
    }

    /// Construct the explicit native-V2 source/target authority required by
    /// an authenticated unpublished-continuation install. This cannot make a
    /// generic/public accepted install eligible for split transaction custody.
    pub fn authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
        &self,
        continuation: &DirectSoilThermalUnpublishedContinuationResultV2,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    ) -> Result<
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
        DirectV10RealConsumerError,
    > {
        if continuation.original_prepared().beginning_owner() != prepared_beginning {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 continuation install authority prepared owner",
                ),
            ));
        }
        let source_transaction_id =
            direct_soil_thermal_complete_source_transaction_v2(self).map_err(|error| {
                DirectV10RealConsumerError::Runtime(error)
            })?;
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            source_transaction_id,
            prepared_beginning.transaction_id,
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "V2 continuation install transaction authority",
            ))
        })
    }

    /// Install one independently accepted sequential child selected from an
    /// authenticated unpublished continuation.
    ///
    /// The candidate must still carry the continuation's original resident;
    /// the selected physical trial must exactly equal the independently
    /// accepted ending and credits. The private trial custody is never
    /// interpreted as an accepted receipt chain.
    pub fn install_soil_thermal_accepted_v2_from_unpublished_continuation(
        &mut self,
        authoritative_beginning: &DirectV10RealConsumerShadow,
        continuation: &DirectSoilThermalUnpublishedContinuationResultV2,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        transaction_authority:
            crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
        accepted: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        let authoritative_resident = authoritative_beginning.inner.soil_thermal.v2()?;
        authoritative_resident.validate()?;
        let candidate_resident = self.inner.soil_thermal.v2()?;
        self.validate_soil_thermal_accepted_v2_from_unpublished_continuation(
            continuation.physical_trial(),
            continuation,
            prepared_beginning,
            &accepted,
        )?;
        let expected_transaction_authority = self
            .authenticate_soil_thermal_unpublished_continuation_install_authority_v2(
                continuation,
                prepared_beginning,
            )?;
        if transaction_authority != expected_transaction_authority {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "V2 continuation install explicit transaction authority",
                ),
            ));
        }
        if candidate_resident.is_exact_accepted_candidate(prepared_beginning, &accepted, &seals)? {
            if authoritative_resident != candidate_resident {
                return Err(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::OwnerClosure(
                        "V2 selected continuation exact no-op authority",
                    ),
                ));
            }
            return self.install_validated_soil_thermal_resident_v2(
                candidate_resident.clone(),
                Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                    transaction_authority,
                )),
                true,
            );
        }
        let accepted_resident =
            authoritative_resident.accepted(prepared_beginning, accepted, seals)?;
        self.install_validated_soil_thermal_resident_v2(
            accepted_resident,
            Some(DirectSoilThermalAtomicInstallAuthorityV2::Physical(
                transaction_authority,
            )),
            false,
        )
    }

    /// Validate one selected unpublished sequential ending against its
    /// retained continuation and proposed outer acceptance without installing
    /// or publishing any owner.
    pub fn validate_soil_thermal_accepted_v2_from_unpublished_continuation(
        &self,
        selected_trial: &openwepp_land_surface_energy::SoilThermalTrialStateV2,
        continuation: &DirectSoilThermalUnpublishedContinuationResultV2,
        prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
        accepted: &SoilThermalAcceptedCandidateV2,
    ) -> Result<(), DirectV10RealConsumerError> {
        if continuation.physical_trial() != selected_trial {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::OwnerClosure(
                    "V2 selected unpublished physical trial substitution",
                ),
            ));
        }
        let candidate_resident = self.inner.soil_thermal.v2()?;
        if candidate_resident
            .is_exact_accepted_candidate_without_external_seals(prepared_beginning, accepted)?
        {
            continuation
                .validate_selected_accepted_child_without_resident(prepared_beginning, accepted)
                .map_err(Into::into)
        } else {
            continuation
                .validate_selected_accepted_child(candidate_resident, prepared_beginning, accepted)
                .map_err(Into::into)
        }
    }

    fn install_validated_soil_thermal_resident_v2(
        &mut self,
        accepted_resident: DirectV10SoilThermalResidentV2,
        split_authority: Option<DirectSoilThermalAtomicInstallAuthorityV2<'_>>,
        exact_accepted_noop: bool,
    ) -> Result<(), DirectV10RealConsumerError> {
        direct_soil_thermal_atomic_complete_owner_transaction_posture_v2(
            self,
            &accepted_resident,
            split_authority,
        )?;
        if exact_accepted_noop
            && self
                .inner
                .soil_thermal
                .v2()
                .is_ok_and(|resident| resident == &accepted_resident)
        {
            return Ok(());
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
fn install_v2_soil_from_authenticated_prepared_beginning_v1(
    candidate: &mut DirectV10RealConsumerShadow,
    authoritative_beginning: &DirectV10RealConsumerShadow,
    prepared_beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    accepted: SoilThermalAcceptedCandidateV2,
    seals: SoilThermalOrchestratorSealsV2,
) -> Result<(), DirectV10RealConsumerError> {
    let authority = candidate
        .authenticate_soil_thermal_prepared_beginning_install_authority_v3(
            authoritative_beginning,
            prepared_beginning,
        )?;
    candidate.install_soil_thermal_accepted_v2_from_authenticated_beginning_v3(
        authoritative_beginning,
        prepared_beginning,
        authority,
        accepted,
        seals,
    )
}

#[cfg(test)]
mod direct_v10_soil_thermal_v2_tests {
    include!("v10_soil_thermal_v2_tests.rs");
}
#[cfg(test)]
mod direct_v10_soil_thermal_v2_v49_tests {
    include!("v10_soil_thermal_v2_v49_tests.rs");
}
#[cfg(test)]
pub(crate) use direct_v10_soil_thermal_v2_v49_tests::migrate_shadow_to_native_v2_for_parent_test;
