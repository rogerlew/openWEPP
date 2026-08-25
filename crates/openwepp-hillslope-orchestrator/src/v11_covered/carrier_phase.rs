// Pure, unpublished covered-carrier trial construction.
//
// This phase deliberately stops at the sealed Stage-3 boundary.  It does not
// evaluate Stage 3, adopt an owner envelope, accept a coupled-time slab, or
// publish any receipt into the owning stack.

use crate::hydrology::{
    CoveredProbeChildIdentityV1, CoveredTerminalJointTrialStateV1,
    CoveredTerminalTrialRequestV1, CoveredTerminalTrialTransitionV1,
};

/// Snow operand presented to the shared covered carrier engine.
///
/// Persistent execution uses the canonical Stage-3 lane map. Terminal trials
/// retain that map only as lineage and replace the target lane's physical
/// bottom/surface operands with the aggregate one-volume trial state.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CoveredSnowBoundaryStateV1 {
    Persistent,
    TerminalTrial {
        lane_id: u32,
        ice_kg_m2: f64,
        liquid_kg_m2: f64,
        cold_content_j_m2: f64,
        surface_temperature_k: f64,
        depth_m: f64,
        density_kg_m3: f64,
    },
}

/// Whether a shared carrier result is an unpublished probe or the candidate
/// used by the accepted covered execution path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CoveredCarrierExecutionIdentityV1 {
    Probe(CoveredProbeChildIdentityV1),
    Accepted,
}

impl CoveredSnowBoundaryStateV1 {
    fn apply_to_boundaries(
        self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    ) -> Result<(), DirectV11RealConsumerError> {
        let Self::TerminalTrial {
            lane_id,
            ice_kg_m2,
            liquid_kg_m2,
            cold_content_j_m2,
            surface_temperature_k,
            depth_m,
            density_kg_m3,
        } = self
        else {
            return Ok(());
        };
        if !surface_temperature_k.is_finite()
            || surface_temperature_k <= 0.0
            || !ice_kg_m2.is_finite()
            || ice_kg_m2 < 0.0
            || !liquid_kg_m2.is_finite()
            || liquid_kg_m2 < 0.0
            || !cold_content_j_m2.is_finite()
            || cold_content_j_m2 < 0.0
            || !depth_m.is_finite()
            || depth_m < 0.0
            || !density_kg_m3.is_finite()
            || density_kg_m3 < 0.0
            || (ice_kg_m2 - density_kg_m3 * depth_m).abs() > 1.0e-9
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal trial snow boundary state",
            ));
        }
        let mut matched = false;
        for (destination, boundary) in boundaries {
            if bindings.iter().any(|binding| {
                binding.ofe_id == destination.0 && binding.production_lane_id == lane_id
            }) {
                matched = true;
                boundary.snow_temperature_k = surface_temperature_k;
            }
        }
        if !matched {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal trial lane topology",
            ));
        }
        Ok(())
    }
}

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
    terminal_snow_soil_trial_receipt:
        Option<physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
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
            terminal_snow_soil_trial_receipt: None,
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
        self.execute_shared_covered_carrier_engine_v1(
            beginning,
            request,
            CoveredSnowBoundaryStateV1::TerminalTrial {
                lane_id: request.lane_id,
                ice_kg_m2: request.ice_kg_m2,
                liquid_kg_m2: request.liquid_kg_m2,
                cold_content_j_m2: request.cold_content_j_m2,
                surface_temperature_k: request.surface_temperature_c + 273.15,
                depth_m: request.snow_depth_m,
                density_kg_m3: request.snow_density_kg_m3,
            },
            CoveredCarrierExecutionIdentityV1::Probe(child),
        )
    }

    /// Execute the value-returning covered carrier engine without adopting a
    /// slab or publishing any receipt. Both persistent and terminal callers
    /// use this mapping; execution identity controls only lineage, never
    /// physical operands.
    fn execute_shared_covered_carrier_engine_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
        execution_identity: CoveredCarrierExecutionIdentityV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let CoveredCarrierExecutionIdentityV1::Probe(child) = execution_identity else {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted shared carrier engine not yet joined",
            ));
        };
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
        let seed = self.stage3_lower_boundaries_by_destination(
            &carrier_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let mut seed =
            self.merge_latest_stage3_state_operands(&seed, &beginning.stage3_by_lane)?;
        snow_boundary_state.apply_to_boundaries(
            &beginning.shadow.inner.surface_configuration.ofe_bindings,
            &mut seed,
        )?;
        // Stage-3 lower-boundary construction, rather than the broader
        // carrier-diagnostic receipt topology, is the authority for which
        // destinations enter the covered LSE branch. Ordinary canopy/open
        // destinations remain in the complete envelope below, but must not
        // be reclassified as snow-covered merely because they have a carrier
        // diagnostic receipt.
        let covered_destinations = seed.keys().cloned().collect::<BTreeSet<_>>();
        if covered_destinations != self.covered_expected_destinations() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier Stage-3 lower-boundary membership",
            ));
        }
        let (open_diagnostics, open_boundaries, _) =
            self.open_snow_boundaries_by_destination(&beginning.stage3_by_lane)?;
        if covered_destinations
            .iter()
            .any(|destination| open_boundaries.contains_key(destination))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier Stage-3/open destination membership",
            ));
        }
        // One provider call is one joint carrier mapping. The hydrology
        // terminal solver owns the outer fixed-point replay and returns the
        // preceding snow estimate through `ending_snow_hint`; iterating only
        // the carrier here would omit snow and soil from convergence.
        let envelope = self.build_covered_carrier_envelope_value_v1(
            CoveredCarrierEnvelopeBuildV1 {
                candidate: &beginning.shadow,
                interval_s,
                duration_s_bits: request.support.duration_s_bits(),
                covered_destinations: &covered_destinations,
                covered_boundaries: &seed,
                open_boundaries: &open_boundaries,
                // Every provider replay starts from an unsealed carrier
                // operand. Coupling iteration is joint-solver chronology, not
                // authority to reinterpret that operand as a final optical
                // boundary.
                provisional: true,
                finalize_wb14_parent_interval: false,
            },
        )?;
        let (corrected, _lse_states) = self.rebuild_covered_lse_carrier_value_v1(
            &seed,
            &envelope,
            &beginning.stage3_by_lane,
            snow_boundary_state,
        )?;
        let precipitation_sets = self.precipitation_parcel_sets(request.support, &envelope)?;
        let lane_id = request.lane_id;
        let ofe_id = self
            .covered_lane_to_ofe(&beginning.stage3_by_lane)?
            .remove(&lane_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier terminal snow-soil OFE",
            ))?;
        let configured_ofe = beginning
            .shadow
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|value| value.ofe_id == ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier terminal configured OFE",
            ))?;
        let configured_top = configured_ofe.soil_interface_layers.first().ok_or(
            DirectV11RealConsumerError::Identity("covered carrier terminal configured soil top"),
        )?;
        let beginning_soil_ofe = beginning
            .shadow
            .inner
            .soil_thermal
            .ofes
            .iter()
            .find(|value| value.ofe_id == ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier terminal beginning soil OFE",
            ))?;
        let beginning_soil_top = beginning_soil_ofe.ordered_layers.first().ok_or(
            DirectV11RealConsumerError::Identity("covered carrier terminal beginning soil top"),
        )?;
        let stage3_inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier terminal Stage-3 inputs"),
        )?;
        let terminal_soil_trial = physical_outcome_ledger::evaluate_terminal_snow_bottom_soil_trial_v1(
            &physical_outcome_ledger::TerminalSnowBottomSoilTrialInputsV1 {
                support: request.support,
                lane_id,
                ofe_id: &ofe_id,
                canonical_source_sha256: child.receipt_sha256,
                ice_kg_m2: request.ice_kg_m2,
                liquid_kg_m2: request.liquid_kg_m2,
                cold_content_j_m2: request.cold_content_j_m2,
                depth_m: request.snow_depth_m,
                density_kg_m3: request.snow_density_kg_m3,
                temperature_k: request.surface_temperature_c + 273.15,
                atmospheric_pressure_pa: stage3_inputs
                    .surface_energy_options
                    .atmospheric_pressure_pa,
                first_soil_configuration: configured_top,
                beginning_first_soil: beginning_soil_top,
            },
        )
        .map_err(|_| {
            DirectV11RealConsumerError::Identity("covered carrier terminal snow-soil trial")
        })?;
        let terminal_soil_credit = SoilThermalTopBoundaryCreditV1 {
            lane_id,
            ofe_id: ofe_id.clone(),
            first_layer_id: configured_top.layer_id.clone(),
            beginning_owner_id: beginning.shadow.inner.soil_thermal.owner_id.clone(),
            beginning_configuration_sha256: beginning
                .shadow
                .inner
                .soil_thermal
                .configuration_sha256
                .clone(),
            beginning_state_sha256: beginning
                .shadow
                .inner
                .soil_thermal
                .state_sha256
                .clone(),
            support_start_ns: i64::try_from(request.support.start_ns().get()).map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal soil credit support start")
            })?,
            support_end_ns: i64::try_from(request.support.end_ns().get()).map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal soil credit support end")
            })?,
            accepted_positive_downward_j_m2_ofe_ground: terminal_soil_trial.soil_heat_j_m2,
            soil_thermal_credit_j_m2_ofe_ground: terminal_soil_trial.soil_heat_j_m2,
            snow_soil_heat_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                terminal_soil_trial.receipt.receipt_sha256,
            ))
            .map_err(|_| DirectV11RealConsumerError::Identity("terminal soil credit digest"))?,
        };
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
                snow_soil_heat_j_m2: terminal_soil_trial.snow_heat_j_m2,
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
            .accept_envelope_with_soil_top_boundary_credits(
                envelope.transaction_id(),
                &envelope,
                &[terminal_soil_credit],
            )
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
        let mut ending_candidates = CoveredCarrierEphemeralCandidatesV1::try_new(
            ending_joint,
            candidate,
            beginning.stage3_by_lane.clone(),
        )?;
        ending_candidates.terminal_snow_soil_trial_receipt =
            Some(terminal_soil_trial.receipt.clone());
        let transition = CoveredTerminalTrialTransitionV1 {
            boundary,
            beginning_joint: beginning.joint.clone(),
            ending_joint: ending_candidates.joint.clone(),
            probe_child_identity: child,
            trial_snow_soil_receipt: Some(terminal_soil_trial.receipt),
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
        let implementation = source
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        assert!(source.contains("execute_covered_carrier_phase_v1(\n        &self,"));
        assert!(source.contains("child.trial_support != request.support"));
        assert!(source.contains("child.beginning_joint_sha256 != beginning.joint.receipt_sha256()"));
        assert!(source.contains("forcing.duration_seconds.to_bits() != interval_s.to_bits()"));
        assert!(source.contains("boundary.snow_temperature_k = trial_temperature_k"));
        assert!(source.contains("One provider call is one joint carrier mapping"));
        assert!(source.contains("provisional: true"));
        assert!(source.contains("accept_envelope(envelope.transaction_id(), &envelope)"));
        assert!(!implementation.contains("let ending_candidates = beginning.clone()"));
    }
}
