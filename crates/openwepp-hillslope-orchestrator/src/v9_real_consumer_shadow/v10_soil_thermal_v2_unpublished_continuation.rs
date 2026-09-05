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
        let original = original_prepared.beginning_owner();
        let aggregate_base = resident.receipt_free_seals.is_none()
            && original.support_start_ns != resident.owner.support_end_ns;
        if !aggregate_base {
            resident.validate_prepared_beginning(original)?;
        } else {
            resident.validate_unpublished_aggregate_base_prepared_beginning(
                original,
                retained_trial,
                child_support_start_ns,
                child_support_end_ns,
            )?;
        }
        resident.validate_prepared_beginning(prior_prepared.beginning_owner())?;
        let prior = prior_prepared.beginning_owner();
        if aggregate_base {
            resident.validate_unpublished_aggregate_base_lineage(
                original,
                prior,
                retained_trial,
                child_support_start_ns,
                child_support_end_ns,
            )?;
        } else {
            validate_unpublished_continuation_lineage(
                original,
                prior,
                child_support_start_ns,
                child_support_end_ns,
            )?;
        }
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
        let original = original_prepared.beginning_owner();
        if resident.receipt_free_seals.is_some()
            || original.support_start_ns == resident.owner.support_end_ns
        {
            resident.validate_prepared_beginning(original)?;
        } else {
            resident.validate_unpublished_aggregate_prepared_extension(
                prior,
                original,
                child_support_start_ns,
                child_support_end_ns,
            )?;
        }
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
        prior.compose_accepted_outer_candidate(configuration).map_err(|_| {
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
        if self
            .physical_trial
            .unpublished_predecessor_trial_sha256()
            .is_none()
            && self
                .physical_trial
                .accepted_predecessor_receipt_chain_sha256()
                == Some(&self.original_prepared.beginning_owner().receipt_chain_sha256)
            && self.layer_credit_chain.len() == 1
            && self.layer_credit_chain.first().map(Vec::as_slice)
                == Some(self.physical_trial.layer_credits())
        {
            let candidate = aggregate_soil_thermal_ending_v2(
                self.original_prepared.beginning_owner(),
                configuration,
                &expected,
            )?;
            if candidate.ending_owner.state != *self.physical_trial.ending_state()
                || candidate.credit_receipt.layer_credits != self.physical_trial.layer_credits()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "V2 base unpublished accepted outer composition",
                ));
            }
            return Ok(SoilThermalAcceptedCandidateV2 {
                ending_owner: candidate.ending_owner,
                credit_receipt: candidate.credit_receipt,
                expected_sources: expected,
            });
        }
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
        for reconstructed in &reconstructed_groups {
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
        candidate_resident.validate_unpublished_continuation_prepared_beginning(
            self,
            self.original_prepared.beginning_owner(),
        )?;
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
        let base_custody = self
            .physical_trial
            .accepted_predecessor_receipt_chain_sha256()
            == Some(&original.receipt_chain_sha256)
            && self
                .physical_trial
                .unpublished_predecessor_trial_sha256()
                .is_none()
            && self.layer_credit_chain.len() == 1;
        let continued_custody = self
            .physical_trial
            .accepted_predecessor_receipt_chain_sha256()
            .is_none()
            && self
                .physical_trial
                .unpublished_predecessor_trial_sha256()
                .is_some()
            && self.layer_credit_chain.len() >= 2;
        let predicates = [
            prepared_beginning == original,
            base_custody || continued_custody,
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
