// Pure, unpublished covered-carrier trial construction.
//
// This phase deliberately stops at the sealed Stage-3 boundary.  It does not
// evaluate Stage 3, adopt an owner envelope, accept a coupled-time slab, or
// publish any receipt into the owning stack.

use crate::hydrology::{
    CoveredProbeChildIdentityV1, CoveredTerminalJointTrialStateV1,
    CoveredTerminalTrialRequestV1, CoveredTerminalTrialTransitionV1,
};

/// Typed companions for opaque canonical joint-owner bytes.
///
/// Canonical owner bytes are intentionally not a deserialization protocol.
/// The probe therefore retains the typed, unpublished candidates beside their
/// canonical joint identity and validates the pair before every trial.
#[derive(Clone)]
pub(crate) struct CoveredCarrierEphemeralCandidatesV1 {
    joint: CoveredTerminalJointTrialStateV1,
    shadow: DirectV10RealConsumerShadow,
    stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
}

impl CoveredCarrierEphemeralCandidatesV1 {
    pub(crate) fn try_new(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let actual = shadow.canonical_owner_state_bytes()?;
        if actual.iter().any(|(owner_id, bytes)| {
            owner_id != "snow"
                && joint
                    .owner_bytes()
                    .get(owner_id)
                    .is_none_or(|joint_bytes| joint_bytes != bytes)
        }) {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier typed/joint beginning",
            ));
        }
        Ok(Self {
            joint,
            shadow,
            stage3_by_lane,
        })
    }

    pub(crate) const fn joint(&self) -> &CoveredTerminalJointTrialStateV1 {
        &self.joint
    }

    pub(crate) const fn shadow(&self) -> &DirectV10RealConsumerShadow {
        &self.shadow
    }

    pub(crate) const fn stage3_by_lane(
        &self,
    ) -> &BTreeMap<u32, DirectSnowStage3PersistentState> {
        &self.stage3_by_lane
    }
}

/// Result of one genuine carrier-only mapping at an exact trial support.
#[derive(Clone)]
pub(crate) struct CoveredCarrierPhaseResultV1 {
    pub transition: CoveredTerminalTrialTransitionV1,
    pub ending_candidates: CoveredCarrierEphemeralCandidatesV1,
    pub precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    /// Construct the actual V11/LSE/precipitation/snow--soil carrier for one
    /// immutable terminal trial and stop before Stage-3 evaluation.
    pub(crate) fn execute_covered_carrier_phase_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        child: CoveredProbeChildIdentityV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        if child.trial_support != request.support
            || child.role != request.role
            || child.attempt_ordinal != request.attempt_ordinal
            || child.beginning_joint_sha256 != beginning.joint.receipt_sha256()
            || request.beginning_joint != beginning.joint
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier probe-child join",
            ));
        }
        let interval_s = f64::from_bits(request.support.duration_s_bits());
        if interval_s <= 0.0 || !interval_s.is_finite() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier positive trial support",
            ));
        }
        for forcing in self.stage3_forcing_by_lane.values() {
            if forcing.duration_seconds.to_bits() != interval_s.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered carrier exact projected forcing duration",
                ));
            }
        }

        let (_, vegetation) = project_v9_runtime_to_v8(
            &beginning.shadow.inner.vegetation_configuration,
            &beginning.shadow.inner.vegetation_state,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::V9(error),
            ))
        })?;
        let carrier_receipts = self.carrier_receipts_by_destination(
            interval_s,
            &vegetation,
            &beginning.stage3_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let covered_destinations = carrier_receipts.keys().cloned().collect::<BTreeSet<_>>();
        let seed = self.stage3_lower_boundaries_by_destination(
            &carrier_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let mut seed =
            self.merge_latest_stage3_state_operands(&seed, &beginning.stage3_by_lane)?;
        let trial_temperature_k = request.surface_temperature_c + 273.15;
        if !trial_temperature_k.is_finite() || trial_temperature_k <= 0.0 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier trial snow temperature",
            ));
        }
        // The hydrology root solver owns the evolving aggregate snow state.
        // Every LSE carrier evaluation must therefore use this trial's exact
        // surface temperature, never the persistent support's beginning value.
        for (destination, boundary) in &mut seed {
            if beginning
                .shadow
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .any(|binding| {
                    binding.ofe_id == destination.0
                        && binding.production_lane_id == request.lane_id
                })
            {
                boundary.snow_temperature_k = trial_temperature_k;
            }
        }
        let (open_diagnostics, open_boundaries, _) =
            self.open_snow_boundaries_by_destination(&beginning.stage3_by_lane)?;
        let (envelope, corrected) = 'fixed_point: {
            let mut iterate = seed;
            for _ in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
                let mut execution_iterate = iterate.clone();
                for (destination, boundary) in &open_boundaries {
                    if execution_iterate
                        .insert(destination.clone(), boundary.clone())
                        .is_some()
                    {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered carrier covered/open destination intersection",
                        ));
                    }
                }
                let envelope = beginning
                    .shadow
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        request.support.duration_s_bits(),
                        &covered_destinations,
                        &execution_iterate,
                        true,
                        false,
                        self.wb14_coupled_child_binding,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(
                            DirectV10RealConsumerError::Runtime(error),
                        )
                    })?;
                let (next, _, _) =
                    self.corrected_covered_boundaries_from_envelope(&iterate, &envelope)?;
                let lse_states = envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("covered carrier LSE state")
                    })?;
                let next = self.apply_lse_iteration_exchange(&next, &lse_states)?;
                let mut next =
                    self.merge_latest_stage3_state_operands(&next, &beginning.stage3_by_lane)?;
                for (destination, boundary) in &mut next {
                    if beginning
                        .shadow
                        .inner
                        .surface_configuration
                        .ofe_bindings
                        .iter()
                        .any(|binding| {
                            binding.ofe_id == destination.0
                                && binding.production_lane_id == request.lane_id
                        })
                    {
                        boundary.snow_temperature_k = trial_temperature_k;
                    }
                }
                if covered_fixed_point_boundaries_equal(&iterate, &next) {
                    // Rebuild once from the converged operand so the retained
                    // typed endpoints and boundary share one exact mapping.
                    let final_envelope = beginning
                        .shadow
                        .inner
                        .construct_covered_interval_envelope_with_duration(
                            self.day_index,
                            self.interval_index,
                            self.interval,
                            interval_s,
                            request.support.duration_s_bits(),
                            &covered_destinations,
                            &{
                                let mut execution = next.clone();
                                for (destination, boundary) in &open_boundaries {
                                    execution.insert(destination.clone(), boundary.clone());
                                }
                                execution
                            },
                            false,
                            false,
                            self.wb14_coupled_child_binding,
                        )
                        .map_err(|error| {
                            DirectV11RealConsumerError::Runtime(
                                DirectV10RealConsumerError::Runtime(error),
                            )
                        })?;
                    let (rebuilt, _, _) = self
                        .corrected_covered_boundaries_from_envelope(&next, &final_envelope)?;
                    let final_lse = final_envelope
                        .covered_lse_iteration_state_by_destination()
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered carrier final LSE state",
                            )
                        })?;
                    let rebuilt = self.apply_lse_iteration_exchange(&rebuilt, &final_lse)?;
                    let mut rebuilt = self.merge_latest_stage3_state_operands(
                        &rebuilt,
                        &beginning.stage3_by_lane,
                    )?;
                    for (destination, boundary) in &mut rebuilt {
                        if beginning
                            .shadow
                            .inner
                            .surface_configuration
                            .ofe_bindings
                            .iter()
                            .any(|binding| {
                                binding.ofe_id == destination.0
                                    && binding.production_lane_id == request.lane_id
                            })
                        {
                            boundary.snow_temperature_k = trial_temperature_k;
                        }
                    }
                    if !covered_fixed_point_boundaries_equal(&next, &rebuilt) {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered carrier rebuilt fixed-point mismatch",
                        ));
                    }
                    break 'fixed_point (final_envelope, rebuilt);
                }
                iterate = next;
            }
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier fixed-point iteration limit",
            ));
        };
        let precipitation_sets = self.precipitation_parcel_sets(request.support, &envelope)?;
        let snow_soil = self.snow_soil_heat_receipts(
            request.support,
            &beginning.stage3_by_lane,
            &beginning.shadow.inner.soil_thermal,
        )?;
        let destination_receipts = carrier_receipts
            .iter()
            .map(|(key, value)| (key.clone(), value.diagnostic_sha256))
            .chain(open_diagnostics)
            .collect::<BTreeMap<_, _>>();
        let mut corrected = corrected;
        for (destination, boundary) in open_boundaries {
            corrected.insert(destination, boundary);
        }
        let terms = self.lane_stage3_terms_from_boundaries(
            &destination_receipts,
            &corrected,
            interval_s,
        )?;
        let lane_id = request.lane_id;
        let lane_terms = terms.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier active lane"),
        )?;
        let precipitation = precipitation_sets.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier precipitation lane"),
        )?;
        let (_, advection) = reconstruct_precipitation_mass_and_advected_heat(precipitation)
            .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
        let snow = beginning.stage3_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier snow lane"),
        )?;
        let snow_digest = if crate::hydrology::stage3_is_terminal_event_domain(snow) {
            Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(snow)
        } else {
            Wb11HydrologyKernel::project_stage3_surface_state_v1(snow)
        }
        .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier snow projection"))?
        .beginning_stage3_state_sha256;
        let (sensible, vapor, latent) = outward_snow_fluxes_to_stage3(
            lane_terms.sensible_to_canopy_air_w_m2,
            lane_terms.vapor_to_canopy_air_kg_m2_s,
            lane_terms.latent_energy_to_canopy_air_j_m2,
            interval_s,
        );
        let boundary = Stage3SnowSurfaceBoundaryReceiptV1::try_new(
            Stage3SnowSurfaceBoundaryReceiptInputs {
                support: request.support,
                sensible_energy_j_m2: sensible,
                vapor_mass_kg_m2: vapor,
                latent_energy_j_m2: latent,
                shortwave_energy_j_m2: lane_terms.snow_absorbed_shortwave_w_m2 * interval_s,
                net_longwave_energy_j_m2: lane_terms.snow_net_longwave_w_m2 * interval_s,
                precipitation_advection_j_m2: advection,
                snow_soil_heat_j_m2: snow_soil
            .get(&lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered carrier snow-soil lane",
                    ))?
                    .snow_candidate_heat_j_m2_ofe_ground,
                latent_heat_j_kg: lane_terms.latent_heat_j_kg,
                beginning_stage3_state_sha256: snow_digest,
                identity: Stage3BoundaryIdentity::Provisional {
                    carrier_receipt_sha256: lane_terms.provisional_carrier_receipt_sha256,
                },
            },
        )?;

        // Adopt only into the unpublished clone. This evolves the six
        // carrier-owned typed candidates without accepting a slab, publishing
        // a receipt, or mutating the owning stack. Hydrology seals the seventh
        // (snow) candidate after applying this boundary.
        let mut candidate = beginning.shadow.clone();
        candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
        candidate
            .inner
            .accept_envelope(envelope.transaction_id(), &envelope)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        let mut ending_owner_bytes = candidate.canonical_owner_state_bytes()?;
        let trial_snow = request
            .beginning_joint
            .owner_bytes()
            .get("snow")
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier trial snow owner",
            ))?
            .clone();
        ending_owner_bytes.insert("snow".to_owned(), trial_snow);
        let ending_joint = CoveredTerminalJointTrialStateV1::try_new(ending_owner_bytes)
            .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier ending joint"))?;
        let ending_candidates = CoveredCarrierEphemeralCandidatesV1::try_new(
            ending_joint,
            candidate,
            beginning.stage3_by_lane.clone(),
        )?;
        let transition = CoveredTerminalTrialTransitionV1 {
            boundary,
            beginning_joint: beginning.joint.clone(),
            ending_joint: ending_candidates.joint.clone(),
            probe_child_identity: child,
            terminal_snow_soil_receipt: None,
        };
        Ok(CoveredCarrierPhaseResultV1 {
            transition,
            ending_candidates,
            precipitation_sets,
        })
    }
}

#[cfg(test)]
mod covered_carrier_phase_tests {
    #[test]
    fn phase_has_no_stage3_evaluation_or_publication_surface() {
        let source = include_str!("carrier_phase.rs");
        let implementation = source
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        for forbidden in [
            "evaluate_stage3_persistent_support",
            "evaluate_stage3_terminal_support",
            "accept_slab(",
            "finalize_v11_imported_segment",
            "last_support_receipt =",
            "last_final_boundary_receipts =",
            "last_wb14_",
            "self.ending =",
        ] {
            assert!(
                !implementation.contains(forbidden),
                "carrier phase reached forbidden publication/evaluation surface: {forbidden}"
            );
        }
    }

    #[test]
    fn phase_receiver_is_immutable_and_trial_identity_is_exact() {
        let source = include_str!("carrier_phase.rs");
        assert!(source.contains("execute_covered_carrier_phase_v1(\n        &self,"));
        assert!(source.contains("child.trial_support != request.support"));
        assert!(source.contains("child.beginning_joint_sha256 != beginning.joint.receipt_sha256()"));
        assert!(source.contains("forcing.duration_seconds.to_bits() != interval_s.to_bits()"));
        assert!(source.contains("boundary.snow_temperature_k = trial_temperature_k"));
        assert!(source.contains("'fixed_point:"));
        assert!(source.contains("covered_fixed_point_boundaries_equal"));
        assert!(source.contains("accept_envelope(envelope.transaction_id(), &envelope)"));
        assert!(!source.contains("let ending_candidates = beginning.clone()"));
    }
}
