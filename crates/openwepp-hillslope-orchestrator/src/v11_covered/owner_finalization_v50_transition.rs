#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct V50OuterOwnerTransitionEvidenceV1 {
    pub beginning_vegetation_transaction_id: u128,
    pub beginning_lse_transaction_id: Option<TransactionId>,
    pub beginning_bgc_transaction_id: u128,
    pub beginning_soil_transaction_id: Option<TransactionId>,
    pub envelope_transaction_id: TransactionId,
    pub reconstructed_vegetation_transaction_id: u128,
    pub reconstructed_lse_transaction_id: Option<TransactionId>,
    pub reconstructed_bgc_transaction_id: u128,
}

#[cfg(test)]
std::thread_local! {
    static V50_OUTER_OWNER_TRANSITION_EVIDENCE: std::cell::RefCell<Option<Vec<V50OuterOwnerTransitionEvidenceV1>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_v50_outer_owner_transition_evidence_v1() {
    V50_OUTER_OWNER_TRANSITION_EVIDENCE.with(|rows| *rows.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
pub(crate) fn take_v50_outer_owner_transition_evidence_v1() -> Vec<V50OuterOwnerTransitionEvidenceV1>
{
    V50_OUTER_OWNER_TRANSITION_EVIDENCE.with(|rows| rows.borrow_mut().take().unwrap_or_default())
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AuthenticatedCoveredV8OuterOwnerTransitionV1 {
    envelope: UncommittedCoveredV8OwnerEnvelope,
    expected_non_soil_ending: DirectV10RealConsumerShadow,
}

impl AuthenticatedCoveredV8OuterOwnerTransitionV1 {
    pub(crate) const fn envelope(&self) -> &UncommittedCoveredV8OwnerEnvelope {
        &self.envelope
    }

    pub(crate) const fn expected_non_soil_ending(&self) -> &DirectV10RealConsumerShadow {
        &self.expected_non_soil_ending
    }
}

fn authenticate_v50_covered_v8_outer_owner_transition_v1(
    beginning: &DirectV10RealConsumerShadow,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    charged_candidate: &DirectV10RealConsumerShadow,
) -> Result<AuthenticatedCoveredV8OuterOwnerTransitionV1, DirectV11RealConsumerError> {
    envelope.validate().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let mut expected = beginning.clone();
    match (
        charged_candidate.frozen_litter_v3_resident(),
        charged_candidate.frozen_litter_v4_resident(),
    ) {
        (Some(physical), Some(exact)) => expected
            .install_validated_frozen_litter_v4_residents(physical.clone(), exact.clone())
            .map_err(DirectV11RealConsumerError::Runtime)?,
        (None, None)
            if expected.frozen_litter_v3_resident().is_none()
                && expected.frozen_litter_v4_resident().is_none() => {}
        _ => {
            return Err(DirectV11RealConsumerError::Identity(
                "covered V50 native frozen-litter pair posture",
            ));
        }
    }
    super::super::stage_unpublished_v2_carrier_owners(&mut expected, envelope)?;
    expected.vegetation_state = project_v9_runtime_to_v10(
        expected.inner.vegetation_state(),
        &expected.vegetation_configuration,
    )
    .map_err(|error| DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error)))?;
    expected.lse_state = project_validated_v1_runtime_to_v2(
        &expected.inner.lse_configuration,
        expected.inner.lse_state(),
        &expected.lse_configuration,
        &openwepp_land_surface_energy::Sha256Digest::try_new(
            expected
                .vegetation_configuration
                .configuration_sha256
                .clone(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
    })?;
    match (
        charged_candidate.frozen_litter_v3_resident(),
        charged_candidate.frozen_litter_v4_resident(),
    ) {
        (Some(physical), Some(exact)) => {
            let charged_lse_owner_bytes = charged_candidate
                .canonical_v11_lse_owner_bytes()
                .map_err(DirectV11RealConsumerError::Runtime)?;
            let reconstructed_lse_owner_bytes = expected
                .canonical_v11_lse_owner_bytes()
                .map_err(DirectV11RealConsumerError::Runtime)?;
            if expected.frozen_litter_v3_resident() != Some(physical)
                || expected.frozen_litter_v4_resident() != Some(exact)
                || reconstructed_lse_owner_bytes != charged_lse_owner_bytes
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered V50 authenticated native frozen-litter pair",
                ));
            }
        }
        (None, None)
            if expected.frozen_litter_v3_resident().is_none()
                && expected.frozen_litter_v4_resident().is_none() => {}
        _ => {
            return Err(DirectV11RealConsumerError::Identity(
                "covered V50 native frozen-litter pair posture",
            ));
        }
    }
    #[cfg(test)]
    V50_OUTER_OWNER_TRANSITION_EVIDENCE.with(|rows| {
        if let Some(rows) = rows.borrow_mut().as_mut() {
            rows.push(V50OuterOwnerTransitionEvidenceV1 {
                beginning_vegetation_transaction_id: beginning
                    .vegetation_state
                    .0
                    .last_transaction_id,
                beginning_lse_transaction_id: beginning.lse_state.0.last_accepted_transaction_id,
                beginning_bgc_transaction_id: beginning.inner.biogeochemistry.last_transaction_id,
                beginning_soil_transaction_id: beginning
                    .soil_thermal_v2()
                    .ok()
                    .map(|resident| resident.owner.transaction_id),
                envelope_transaction_id: envelope.transaction_id(),
                reconstructed_vegetation_transaction_id: expected
                    .vegetation_state
                    .0
                    .last_transaction_id,
                reconstructed_lse_transaction_id: expected.lse_state.0.last_accepted_transaction_id,
                reconstructed_bgc_transaction_id: expected
                    .inner
                    .biogeochemistry
                    .last_transaction_id,
            });
        }
    });
    Ok(AuthenticatedCoveredV8OuterOwnerTransitionV1 {
        envelope: envelope.clone(),
        expected_non_soil_ending: expected,
    })
}

#[cfg(test)]
mod v50_native_pair_binding_source_guards {
    #[test]
    fn authenticated_transition_binds_only_the_charged_exact_pair() {
        let source = include_str!("owner_finalization_v50_transition.rs");
        let body = source
            .split("fn authenticate_v50_covered_v8_outer_owner_transition_v1")
            .nth(1)
            .expect("V50 authentication body");
        assert!(body.contains("charged_candidate: &DirectV10RealConsumerShadow"));
        assert_eq!(
            body.matches("install_validated_frozen_litter_v4_residents")
                .count(),
            1
        );
        assert!(body.contains("expected.frozen_litter_v3_resident() != Some(physical)"));
        assert!(body.contains("expected.frozen_litter_v4_resident() != Some(exact)"));
        assert!(body.contains("reconstructed_lse_owner_bytes != charged_lse_owner_bytes"));
        assert!(!body.contains("install_frozen_litter_v4_resident("));
        assert!(!body.contains("install_restored_frozen_litter_v4_residents"));
    }
}
