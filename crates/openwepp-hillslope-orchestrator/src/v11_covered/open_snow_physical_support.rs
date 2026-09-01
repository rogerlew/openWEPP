#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CoveredOrdinaryPhysicalAuthorityV1 {
    physical_input_sha256: Digest32,
    beginning_authority_sha256: Digest32,
    accepted_authorization_sha256: Digest32,
}

fn covered_ordinary_physical_authority_v1(
    input: &V11ImportedV10SegmentInput,
) -> Result<CoveredOrdinaryPhysicalAuthorityV1, DirectV11RealConsumerError> {
    fn frame<T: Serialize>(value: &T) -> Result<Digest32, DirectV11RealConsumerError> {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            DirectV11RealConsumerError::Identity("covered ordinary physical authority framing")
        })?;
        Ok(digest_bytes(&bytes))
    }
    Ok(CoveredOrdinaryPhysicalAuthorityV1 {
        physical_input_sha256: frame(&(
            input.parent_transaction_id,
            input.support,
            input.duration_s_bits,
            &input.configuration,
        ))?,
        beginning_authority_sha256: frame(&(&input.beginning, &input.staged_resource_owners))?,
        accepted_authorization_sha256: frame(&(
            input.accepted_slab_receipt.parent_transaction_id(),
            input.accepted_slab_receipt.segment_id(),
            input.accepted_slab_receipt.slab_ordinal(),
            input.accepted_slab_receipt.support(),
            input.accepted_slab_receipt.duration_s_bits(),
        ))?,
    })
}

fn validate_covered_ordinary_physical_reuse_gate_v1(
    seed: CoveredOrdinaryPhysicalAuthorityV1,
    candidate: CoveredOrdinaryPhysicalAuthorityV1,
    terminal_endpoint_mode: bool,
) -> Result<(), DirectV11RealConsumerError> {
    if terminal_endpoint_mode || seed != candidate {
        return Err(DirectV11RealConsumerError::Identity(
            "covered ordinary physical reuse identity",
        ));
    }
    Ok(())
}

fn validate_covered_terminal_reuse_trial_binding_v1(
    observed: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    accepted: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    trial_support: TimeSupport,
    replay_trial_sha256: Digest32,
    replay_beginning_owner_set_sha256: Digest32,
) -> Result<(), DirectV11RealConsumerError> {
    let expected = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        accepted_slab_sha256: *replay_trial_sha256.as_bytes(),
        parent_beginning_complete_owner_set_sha256: *replay_beginning_owner_set_sha256.as_bytes(),
        child_support_start_ns: trial_support.start_ns().get(),
        child_support_end_ns: trial_support.end_ns().get(),
        ..accepted
    };
    if observed != expected {
        return Err(DirectV11RealConsumerError::Identity(
            "covered terminal reuse WB14 trial authorization",
        ));
    }
    Ok(())
}

#[cfg(test)]
include!("open_snow_reuse_gate_tests.rs");

fn reconstruct_stage3_phase_forcing_v1(
    liquid_mass_kg_m2: f64,
    solid_mass_kg_m2: f64,
) -> Result<(f64, f64, f64), DirectV11RealConsumerError> {
    if !liquid_mass_kg_m2.is_finite()
        || !solid_mass_kg_m2.is_finite()
        || liquid_mass_kg_m2 < 0.0
        || solid_mass_kg_m2 < 0.0
    {
        return Err(DirectV11RealConsumerError::Identity(
            "precipitation phase/mass same-set join",
        ));
    }
    let rain_m = liquid_mass_kg_m2 / 1_000.0;
    let snowfall_m = solid_mass_kg_m2 / 100.0;
    let active_precipitation_m = (liquid_mass_kg_m2 + solid_mass_kg_m2) / 1_000.0;
    if (active_precipitation_m - (rain_m + snowfall_m * 0.1)).abs() > 1.0e-12 {
        return Err(DirectV11RealConsumerError::Identity(
            "precipitation phase active-volume reconstruction",
        ));
    }
    Ok((rain_m, snowfall_m, active_precipitation_m))
}

#[cfg(test)]
mod stage3_phase_forcing_reconstruction_tests {
    use super::*;

    #[test]
    fn solid_geometric_depth_reconstructs_water_equivalent_and_rejects_raw_depth_alias() {
        let liquid_mass = 2.854_810_069_486_077;
        let solid_mass = 7.145_189_930_513_923;
        let (rain_m, snowfall_m, active_m) =
            reconstruct_stage3_phase_forcing_v1(liquid_mass, solid_mass)
                .expect("phase reconstruction");
        assert_eq!(rain_m.to_bits(), (liquid_mass / 1_000.0).to_bits());
        assert_eq!(snowfall_m.to_bits(), (solid_mass / 100.0).to_bits());
        assert_eq!(
            active_m.to_bits(),
            ((liquid_mass + solid_mass) / 1_000.0).to_bits(),
        );
        assert!((active_m - 0.01).abs() <= f64::EPSILON);
        assert_ne!(active_m.to_bits(), (rain_m + snowfall_m).to_bits());
        assert!(reconstruct_stage3_phase_forcing_v1(-f64::MIN_POSITIVE, solid_mass).is_err());
    }
}

#[derive(Clone, Copy, Debug)]
struct TerminalEndpointExternalLedgerV1 {
    energy_j_m2: [f64; 6],
    vapor_kg_m2: f64,
}

impl TerminalEndpointExternalLedgerV1 {
    fn ordered_add(self, ending: Self) -> Self {
        Self {
            energy_j_m2: std::array::from_fn(|index| {
                self.energy_j_m2[index] + ending.energy_j_m2[index]
            }),
            vapor_kg_m2: self.vapor_kg_m2 + ending.vapor_kg_m2,
        }
    }

    fn matches(self, other: Self) -> bool {
        self.energy_j_m2
            .iter()
            .zip(other.energy_j_m2)
            .all(|(actual, expected)| (actual - expected).abs() <= 1.0e-6)
            && (self.vapor_kg_m2 - other.vapor_kg_m2).abs() <= 1.0e-9
    }
}

fn accepted_terminal_endpoint_timing_v1(
    entry_seconds: f64,
    terminal_seconds: f64,
    unevaluated_seconds: f64,
    hour_offset_seconds: f64,
    complete_evaluated_seconds: f64,
    support_seconds: f64,
) -> bool {
    unevaluated_seconds.abs() <= 1.0e-6
        && (entry_seconds + terminal_seconds - support_seconds).abs() <= 1.0e-6
        && (hour_offset_seconds - support_seconds).abs() <= 1.0e-6
        && (complete_evaluated_seconds - support_seconds).abs() <= 1.0e-6
}

fn validate_accepted_terminal_endpoint_composition_v1(
    evaluation: &crate::hydrology::DirectSnowStage3EvaluationDiagnostics,
    reconciliation: &crate::hydrology::DirectSnowStage3OperatorReconciliation,
    event: &DirectSnowTerminalEventResult,
    support_seconds: f64,
) -> Result<bool, DirectV11RealConsumerError> {
    if !event.event_occurred {
        return Ok(false);
    }
    if !accepted_terminal_endpoint_timing_v1(
        event.terminal_entry_offset_seconds,
        event.evaluated_seconds,
        event.unevaluated_seconds,
        event.hour_offset_seconds,
        evaluation.evaluated_seconds,
        support_seconds,
    ) {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix/terminal chronology",
        ));
    }
    let mut prefix_seconds = 0.0;
    let mut prefix = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [0.0; 6],
        vapor_kg_m2: 0.0,
    };
    for tuple in reconciliation.tuples.iter().filter(|tuple| {
        tuple.applicable
            && tuple.elapsed_start_seconds + tuple.duration_seconds
                <= event.terminal_entry_offset_seconds + 1.0e-9
    }) {
        prefix_seconds += tuple.duration_seconds;
        prefix = prefix.ordered_add(TerminalEndpointExternalLedgerV1 {
            energy_j_m2: [
                tuple.sensible_flux_w_m2 * tuple.duration_seconds,
                tuple.net_shortwave_w_m2 * tuple.duration_seconds,
                tuple.latent_flux_w_m2 * tuple.duration_seconds,
                tuple.net_longwave_w_m2 * tuple.duration_seconds,
                tuple.precipitation_advected_flux_w_m2 * tuple.duration_seconds,
                tuple.snow_soil_heat_flux_w_m2 * tuple.duration_seconds,
            ],
            vapor_kg_m2: tuple.vapor_mass_exchange_kg_m2,
        });
    }
    if (prefix_seconds - event.terminal_entry_offset_seconds).abs() > 1.0e-6 {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix chronology",
        ));
    }
    let terminal = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [
            event.sensible_energy_j_m2,
            event.shortwave_energy_j_m2,
            event.latent_energy_j_m2,
            event.longwave_energy_j_m2,
            event.advected_energy_j_m2,
            event.snow_soil_heat_energy_j_m2,
        ],
        vapor_kg_m2: event.deposition_kg_m2 - event.sublimation_kg_m2,
    };
    let complete = TerminalEndpointExternalLedgerV1 {
        energy_j_m2: [
            evaluation.complete_arm_sensible_j_m2,
            evaluation.complete_arm_shortwave_j_m2,
            evaluation.complete_arm_latent_j_m2,
            evaluation.complete_arm_longwave_j_m2,
            evaluation.complete_arm_advected_j_m2,
            evaluation.complete_arm_snow_soil_heat_j_m2,
        ],
        vapor_kg_m2: evaluation.complete_arm_vapor_mass_exchange_kg_m2,
    };
    if !prefix.ordered_add(terminal).matches(complete) {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal endpoint accepted prefix/terminal ledger composition",
        ));
    }
    Ok(true)
}

include!("open_snow_physical_reconstruction_helpers.rs");

struct SnowSoilReceiptInputs<'a> {
    support: openwepp_coupled_time::TimeSupport,
    duration_s: f64,
    lane_id: u32,
    ofe_id: &'a OfeId,
    topology_identity_sha256: Digest32,
    configuration_identity_sha256: Digest32,
    beginning_soil_owner_identity_sha256: Digest32,
    beginning_stage: &'a DirectSnowStage3PersistentState,
    ending_stage: &'a DirectSnowStage3PersistentState,
    ending_ofe: &'a openwepp_land_surface_energy::SoilThermalOfeSnapshot,
    configured_top: &'a openwepp_land_surface_energy::SoilInterfaceLayer,
    beginning_top: &'a openwepp_land_surface_energy::SoilThermalLayerSnapshot,
    ending_top: &'a openwepp_land_surface_energy::SoilThermalLayerSnapshot,
    beginning_bottom_temperature_k: f64,
    beginning_bottom_thickness_m: f64,
    beginning_bottom_conductivity_w_m_k: f64,
    beginning_snow_owner_sha256: Digest32,
    ending_bottom_temperature_k: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentCnTrialOperandV1 {
    support: openwepp_coupled_time::TimeSupport,
    lane_id: u32,
    ofe_id: OfeId,
    topology_identity_sha256: Digest32,
    configuration_identity_sha256: Digest32,
    beginning_snow_owner_identity_sha256: Digest32,
    beginning_soil_owner_identity_sha256: Digest32,
    snow_candidate_heat_j_m2_ofe_ground: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentPhysicalArtifactsV1 {
    stage3_candidate: BTreeMap<u32, DirectSnowStage3PersistentState>,
    stage3_support_images: BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    corrected_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    transaction_id: TransactionId,
    soil_candidates: Vec<openwepp_land_surface_energy::SoilThermalTileCandidate>,
    soil_candidate: DirectSoilThermalCandidate,
    cn_trial_operands: BTreeMap<u32, CoveredPhaseConsistentCnTrialOperandV1>,
    snow_enthalpy_material_owner: Option<
        crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
}

fn covered_soil_layer_enthalpy_coordinate_v1(
    layer: DirectSoilThermalLayerReadView<'_>,
) -> Result<f64, DirectV11RealConsumerError> {
    match layer.exact_carry() {
        None => Ok(layer.enthalpy_high_j_m2_ofe_ground()),
        Some(carry) => openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum_binary64(
            layer.enthalpy_high_j_m2_ofe_ground(),
            carry,
            &[],
        )
        .and_then(|value| value.round_to_f64())
        .map_err(|_| DirectV11RealConsumerError::Identity("V2 soil exact enthalpy coordinate")),
    }
}

impl CoveredPhaseConsistentCnTrialOperandV1 {
    fn from_sealed_receipt(
        receipt: &SnowSoilHeatReceiptV1,
        snow_candidate_heat_j_m2_ofe_ground: f64,
    ) -> Result<Self, DirectV11RealConsumerError> {
        crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
            .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
        if !snow_candidate_heat_j_m2_ofe_ground.is_finite() {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "phase-consistent CN trial energy",
            ));
        }
        Ok(Self {
            support: receipt.support,
            lane_id: receipt.lane_id,
            ofe_id: receipt.ofe_id.clone(),
            topology_identity_sha256: receipt.topology_identity_sha256,
            configuration_identity_sha256: receipt.configuration_identity_sha256,
            beginning_snow_owner_identity_sha256: receipt.beginning_snow_owner_identity_sha256,
            beginning_soil_owner_identity_sha256: receipt.beginning_soil_owner_identity_sha256,
            snow_candidate_heat_j_m2_ofe_ground,
        })
    }

    fn validate_against(
        &self,
        receipt: &SnowSoilHeatReceiptV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        if self.support != receipt.support
            || self.lane_id != receipt.lane_id
            || self.ofe_id != receipt.ofe_id
            || self.topology_identity_sha256 != receipt.topology_identity_sha256
            || self.configuration_identity_sha256 != receipt.configuration_identity_sha256
            || self.beginning_snow_owner_identity_sha256
                != receipt.beginning_snow_owner_identity_sha256
            || self.beginning_soil_owner_identity_sha256
                != receipt.beginning_soil_owner_identity_sha256
            || !self.snow_candidate_heat_j_m2_ofe_ground.is_finite()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "phase-consistent CN trial immutable join",
            ));
        }
        Ok(())
    }
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    fn initial_unpublished_soil_iteration_candidate_v1(
        &self,
        support: TimeSupport,
    ) -> Result<DirectSoilThermalCandidate, DirectV11RealConsumerError> {
        match &self.beginning.inner.soil_thermal {
            DirectSoilThermalResident::V1(beginning) => {
                DirectSoilThermalCandidate::from_v1(beginning.clone()).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })
            }
            DirectSoilThermalResident::V2(_) => {
                let prepared = self
                    .beginning
                    .prepare_next_soil_thermal_support_v2(
                        support.start_ns().get(),
                        support.end_ns().get(),
                    )
                    .map_err(DirectV11RealConsumerError::Runtime)?;
                let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
                    prepared.beginning_owner(),
                    &self.beginning.inner.lse_configuration,
                    Vec::new(),
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
                let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
                    &prepared,
                    expected.accepted_operands(),
                    expected.temperature_projections(),
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "initial unpublished V2 covered soil trial",
                    )
                })?;
                DirectSoilThermalCandidate::from_v2(trial).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })
            }
        }
    }

    fn unpublished_soil_candidate_for_covered_iteration_v1(
        &self,
        support: TimeSupport,
        transaction_id: TransactionId,
        soil_candidates_v1: &[openwepp_land_surface_energy::SoilThermalTileCandidate],
        soil_energy_operands_v2:
            &[openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2],
        top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
    ) -> Result<DirectSoilThermalCandidate, DirectV11RealConsumerError> {
        match &self.beginning.inner.soil_thermal {
            DirectSoilThermalResident::V1(beginning) => {
                let ending = aggregate_soil_thermal_ending_with_top_boundary_credits(
                    beginning,
                    &self.beginning.inner.lse_configuration,
                    transaction_id,
                    soil_candidates_v1,
                    top_boundary_credits,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?
                .ending;
                DirectSoilThermalCandidate::from_v1(ending).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })
            }
            DirectSoilThermalResident::V2(_) => {
                let prepared = self
                    .beginning
                    .prepare_next_soil_thermal_support_v2(
                        support.start_ns().get(),
                        support.end_ns().get(),
                    )
                    .map_err(DirectV11RealConsumerError::Runtime)?;
                let source_owner_id = ResourceOwnerId::try_new("snow").map_err(|_| {
                    DirectV11RealConsumerError::Identity("unpublished V2 covered soil source owner")
                })?;
                let mut operands = soil_energy_operands_v2.to_vec();
                operands.extend(
                    soil_thermal_top_boundary_operands_v2(
                        prepared.beginning_owner(),
                        top_boundary_credits,
                        &source_owner_id,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?,
                );
                canonicalize_v2_operand_order(prepared.beginning_owner(), &mut operands).map_err(
                    |error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    },
                )?;
                let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
                    prepared.beginning_owner(),
                    &self.beginning.inner.lse_configuration,
                    operands,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
                let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
                    &prepared,
                    expected.accepted_operands(),
                    expected.temperature_projections(),
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("unpublished V2 covered soil trial")
                })?;
                DirectSoilThermalCandidate::from_v2(trial).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })
            }
        }
    }

    fn unpublished_soil_candidate_for_covered_envelope_v1(
        &self,
        support: TimeSupport,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
    ) -> Result<DirectSoilThermalCandidate, DirectV11RealConsumerError> {
        let operands = crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
            crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                envelope.transaction_id(),
                support.start_ns().get(),
                support.end_ns().get(),
                envelope.hydrology().pre_ingress_soil_thermal_candidates(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::LandSurfaceShadow(error),
                ))
            })?,
            support.start_ns().get(),
            support.end_ns().get(),
            &self.beginning.inner.lse_configuration.owner_id,
            &self.beginning.inner.surface_configuration.owner_id,
            envelope.hydrology().pre_ingress_soil_thermal_candidates(),
            envelope.hydrology().surface_ingress(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::LandSurfaceShadow(error),
            ))
        })?;
        self.unpublished_soil_candidate_for_covered_iteration_v1(
            support,
            envelope.transaction_id(),
            envelope.hydrology().soil_thermal_candidates(),
            &operands,
            top_boundary_credits,
        )
    }
}

fn seal_snow_soil_heat_receipt(
    inputs: &SnowSoilReceiptInputs<'_>,
) -> Result<SnowSoilHeatReceiptV1, DirectV11RealConsumerError> {
    let (beginning_heat, ending_heat, accepted_heat) =
        crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
            0.5 * inputs.beginning_bottom_thickness_m,
            inputs.beginning_bottom_conductivity_w_m_k,
            0.5 * inputs.configured_top.thickness_m,
            inputs.configured_top.thermal_conductivity_w_m_k,
            inputs.beginning_bottom_temperature_k,
            inputs.beginning_top.temperature_k,
            inputs.ending_bottom_temperature_k,
            inputs.ending_top.temperature_k,
        )
        .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
    let accepted_j = accepted_heat * inputs.duration_s;
    let ending_snow_sha256 = digest_bytes(
        &serde_json::to_vec(inputs.ending_stage)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow-soil trial snow seal"))?,
    );
    let ending_soil_sha256 = digest_bytes(
        &serde_json::to_vec(inputs.ending_ofe)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow-soil trial soil seal"))?,
    );
    SnowSoilHeatReceiptV1 {
        schema_version: 1,
        model_identity_sha256: digest_bytes(b"SC-SNOWENERGY-001@18-SNOW-SOIL-CN-V1"),
        support: inputs.support,
        support_duration_ns: inputs.support.duration_ns(),
        lane_id: inputs.lane_id,
        ofe_id: inputs.ofe_id.clone(),
        ofe_ground_basis: true,
        topology_identity_sha256: inputs.topology_identity_sha256,
        configuration_identity_sha256: inputs.configuration_identity_sha256,
        beginning_snow_owner_identity_sha256: inputs.beginning_snow_owner_sha256,
        beginning_soil_owner_identity_sha256: inputs.beginning_soil_owner_identity_sha256,
        bottom_snow_layer_id: u32::try_from(inputs.beginning_stage.layers.len().saturating_sub(1))
            .map_err(|_| DirectV11RealConsumerError::Identity("snow bottom layer ordinal"))?,
        first_soil_layer_id: inputs.configured_top.layer_id.clone(),
        bottom_snow_half_thickness_m: 0.5 * inputs.beginning_bottom_thickness_m,
        bottom_snow_conductivity_w_m_k: inputs.beginning_bottom_conductivity_w_m_k,
        top_soil_half_thickness_m: 0.5 * inputs.configured_top.thickness_m,
        top_soil_conductivity_w_m_k: inputs.configured_top.thermal_conductivity_w_m_k,
        beginning_bottom_snow_temperature_k: inputs.beginning_bottom_temperature_k,
        beginning_top_soil_temperature_k: inputs.beginning_top.temperature_k,
        ending_bottom_snow_temperature_k: inputs.ending_bottom_temperature_k,
        ending_top_soil_temperature_k: inputs.ending_top.temperature_k,
        beginning_heat_flux_w_m2_ofe_ground: beginning_heat,
        ending_heat_flux_w_m2_ofe_ground: ending_heat,
        accepted_heat_flux_w_m2_ofe_ground: accepted_heat,
        accepted_heat_j_m2_ofe_ground: accepted_j,
        snow_candidate_heat_j_m2_ofe_ground: -accepted_j,
        soil_candidate_heat_j_m2_ofe_ground: accepted_j,
        snow_candidate_ending_identity_sha256: ending_snow_sha256,
        soil_candidate_ending_identity_sha256: ending_soil_sha256,
        receipt_sha256: Digest32::zero(),
    }
    .seal()
    .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))
}

struct PhysicalOutcomeLedgerInputs<'a> {
    support: openwepp_coupled_time::TimeSupport,
    ending: &'a BTreeMap<u32, DirectSnowStage3PersistentState>,
    lanes: &'a BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    destinations: &'a BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    precipitation: &'a BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    soil: &'a BTreeMap<u32, SnowSoilHeatReceiptV1>,
    terminal_soil: &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>,
    adaptive_trial_soil: &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
    terminal_events: &'a BTreeMap<u32, DirectSnowTerminalEventResult>,
    /// Vapor material enthalpy, active/lower interlayer custody, and the
    /// independently reported Stage-3 snow--soil energy, respectively.
    diagnostics: &'a BTreeMap<u32, (f64, f64, f64, f64)>,
}

include!("open_snow_terminal_accepted_endpoint.rs");

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    #[allow(clippy::too_many_lines)]
    fn physical_outcome_ledgers(
        &self,
        inputs: &PhysicalOutcomeLedgerInputs<'_>,
    ) -> Result<
        BTreeMap<u32, physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>,
        DirectV11RealConsumerError,
    > {
        use physical_outcome_ledger::{
            Stage3LanePhysicalOutcomeExpectationV1, Stage3LanePhysicalOutcomeLedgerV1,
        };
        let support = inputs.support;
        let duration_s = f64::from_bits(support.duration_s_bits());
        let mut result = BTreeMap::new();
        for (lane_id, beginning) in &self.stage3_beginning_by_lane {
            let ending = inputs
                .ending
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger ending lane",
                ))?;
            let terminal_ending = self.terminal_endpoint_mode
                && ending.layers.is_empty()
                && !crate::hydrology::stage3_has_represented_ice(ending)
                && ending.detached_retained_liquid_kg_m2.to_bits() == 0.0_f64.to_bits();
            let reappearance = beginning.layers.is_empty()
                && crate::hydrology::stage3_has_represented_ice(ending)
                && inputs.precipitation.get(lane_id).is_some_and(|set| {
                    set.parcels.iter().any(|parcel| {
                        parcel.phase == Stage3PrecipitationPhaseV1::Solid
                            && parcel.mass_kg_m2_tile_ground > 0.0
                    })
                });
            if (!crate::hydrology::stage3_is_resolved_thermal_domain(beginning)
                && !crate::hydrology::stage3_is_terminal_event_domain(beginning)
                && !reappearance)
                || (!crate::hydrology::stage3_is_resolved_thermal_domain(ending)
                    && !crate::hydrology::stage3_is_terminal_event_domain(ending)
                    && !terminal_ending)
            {
                continue;
            }
            let lane = inputs
                .lanes
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger boundary lane",
                ))?;
            let parcels =
                inputs
                    .precipitation
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "physical ledger precipitation lane",
                    ))?;
            let soil = inputs.soil.get(lane_id);
            let adaptive_trial_soil = inputs.adaptive_trial_soil.get(lane_id);
            if soil.is_none() && adaptive_trial_soil.is_none() && !terminal_ending && !reappearance
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "physical ledger soil lane",
                ));
            }
            let beginning_digest = digest_bytes(
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
            );
            let ending_digest = digest_bytes(
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending)?,
            );
            let mut solid = 0.0;
            let mut liquid = 0.0;
            let mut advection = 0.0;
            for parcel in &parcels.parcels {
                let destination = parcels
                    .destinations
                    .iter()
                    .find(|value| value.topology_index == parcel.destination_topology_index)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "physical ledger precipitation destination",
                    ))?;
                let mass = parcel.mass_kg_m2_tile_ground * destination.fraction_of_ofe;
                let specific_enthalpy = match parcel.enthalpy_provider {
                    Stage3PrecipitationEnthalpyProviderV1::Temperature {
                        temperature_k,
                        reference_temperature_k,
                        specific_heat_j_kg_k,
                        ..
                    } => specific_heat_j_kg_k * (temperature_k - reference_temperature_k),
                    Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                        specific_enthalpy_j_kg,
                        ..
                    } => specific_enthalpy_j_kg,
                };
                advection += mass * specific_enthalpy;
                match parcel.phase {
                    Stage3PrecipitationPhaseV1::Solid => solid += mass,
                    Stage3PrecipitationPhaseV1::Liquid => liquid += mass,
                }
            }
            let beginning_ice = beginning
                .layers
                .iter()
                .map(|v| v.mass_swe_m * 1_000.0)
                .sum::<f64>();
            let ending_ice = ending
                .layers
                .iter()
                .map(|v| v.mass_swe_m * 1_000.0)
                .sum::<f64>();
            let beginning_liquid = beginning
                .layers
                .iter()
                .map(|v| v.liquid_water_m * 1_000.0)
                .sum::<f64>()
                + beginning.detached_retained_liquid_kg_m2;
            let ending_liquid = ending
                .layers
                .iter()
                .map(|v| v.liquid_water_m * 1_000.0)
                .sum::<f64>()
                + ending.detached_retained_liquid_kg_m2;
            let deposition =
                ending.cumulative_deposition_kg_m2 - beginning.cumulative_deposition_kg_m2;
            let sublimation =
                ending.cumulative_sublimation_kg_m2 - beginning.cumulative_sublimation_kg_m2;
            let melt = ending.cumulative_melt_kg_m2 - beginning.cumulative_melt_kg_m2;
            // Reconstruct phase conversion from the independent ice owner.
            // The per-layer refrozen counter is diagnostic and does not
            // include every rain-on-snow conversion represented by the
            // accepted ending ice mass.
            let reconstructed_refreeze =
                ending_ice - (beginning_ice + solid + deposition - sublimation - melt);
            // Refreeze is a nonnegative process amount. Preserve the original
            // independently reconstructed ice equation: if a negative value
            // is material, replacing only the process amount with zero leaves
            // that discrepancy in `ice_residual_kg_m2` and closure fails at
            // the unchanged ledger tolerance.
            let refreeze = reconstructed_refreeze.max(0.0);
            let terminal = ending.cumulative_unresolved_liquid_kg_m2
                - beginning.cumulative_unresolved_liquid_kg_m2;
            let terminal_liquid_sensible_enthalpy = ending
                .cumulative_terminal_unallocated_energy_j_m2
                - beginning.cumulative_terminal_unallocated_energy_j_m2;
            // Independently reconstruct the lane exchange from the immutable
            // destination records in their sealed semantic topology order.
            // The lane aggregate is an audited result, never ledger authority.
            let mut destination_aggregate = [0.0_f64; 5];
            for (destination, receipt) in inputs
                .destinations
                .iter()
                .filter(|(destination, _)| destination.0 == lane.ofe_id)
            {
                let physical = receipt.physical_operands();
                let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
                destination_aggregate[0] += fraction * physical[0];
                destination_aggregate[1] += fraction * physical[2];
                destination_aggregate[2] += fraction * physical[3];
                destination_aggregate[3] += fraction * physical[4];
                destination_aggregate[4] += fraction * physical[1];
            }
            let (receipt_vapor, receipt_latent) =
                lane.terminal_bounded_vapor_receipt.as_ref().map_or(
                    (
                        lane.aggregate_vapor_to_canopy_air_kg_m2_s,
                        lane.aggregate_latent_energy_to_canopy_air_j_m2,
                    ),
                    |receipt| {
                        (
                            receipt.raw_vapor_to_canopy_air_kg_m2_s,
                            receipt.raw_latent_energy_to_canopy_air_j_m2,
                        )
                    },
                );
            let aggregate_receipt = [
                lane.aggregate_sensible_to_canopy_air_w_m2,
                receipt_latent,
                lane.aggregate_snow_absorbed_shortwave_w_m2,
                lane.aggregate_snow_net_longwave_w_m2,
                receipt_vapor,
            ];
            validate_destination_reconstruction_against_lane_aggregate(
                destination_aggregate,
                aggregate_receipt,
            )?;
            let terminal_event = inputs.terminal_events.get(lane_id);
            let sensible = terminal_event.map_or(-destination_aggregate[0] * duration_s, |event| {
                event.sensible_energy_j_m2
            });
            let latent = terminal_event
                .map_or(-lane.aggregate_latent_energy_to_canopy_air_j_m2, |event| {
                    event.latent_energy_j_m2
                });
            let shortwave = terminal_event.map_or(destination_aggregate[2] * duration_s, |event| {
                event.shortwave_energy_j_m2
            });
            let longwave = terminal_event.map_or(destination_aggregate[3] * duration_s, |event| {
                event.longwave_energy_j_m2
            });
            // Reconstruct material enthalpy from immutable owner state, never
            // from the accepted external-energy sum being audited.
            let beginning_cold = beginning
                .layers
                .iter()
                .map(|v| v.cold_content_j_m2)
                .sum::<f64>();
            let ending_cold = ending
                .layers
                .iter()
                .map(|v| v.cold_content_j_m2)
                .sum::<f64>();
            let beginning_enthalpy =
                -beginning_cold + OUTCOME_LATENT_HEAT_FUSION_J_KG * beginning_liquid;
            let ending_enthalpy = -ending_cold + OUTCOME_LATENT_HEAT_FUSION_J_KG * ending_liquid;
            let (vapor_material_enthalpy, interlayer_active, interlayer_lower, soil_heat) = *inputs
                .diagnostics
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "physical ledger diagnostic lane",
                ))?;
            let soil_receipt_sha256 = if let Some(receipt) = adaptive_trial_soil {
                receipt.receipt_sha256
            } else if terminal_ending {
                inputs
                    .terminal_soil
                    .get(lane_id)
                    .map_or(Digest32::zero(), |receipt| receipt.receipt_sha256)
            } else if reappearance {
                digest_bytes(b"SC-SNOWENERGY-001@22-REAPPEARANCE-NO-SNOW-SOIL-EXCHANGE-V1")
            } else {
                soil.map_or(Digest32::zero(), |receipt| receipt.receipt_sha256)
            };
            let expected = Stage3LanePhysicalOutcomeExpectationV1 {
                support,
                lane_id: *lane_id,
                ofe_id: lane.ofe_id.clone(),
                topology_sha256: lane.topology_configuration_sha256,
                beginning_snow_owner_sha256: beginning_digest,
                ending_snow_owner_sha256: ending_digest,
                precipitation_set_sha256: parcels.receipt_sha256,
                source_receipts_sha256: [
                    lane.optical_receipt_sha256,
                    lane.reciprocal_longwave_receipt_sha256,
                    lane.provisional_carrier_receipt_sha256,
                    lane.final_destination_receipt_sha256,
                    soil_receipt_sha256,
                    lane.receipt_sha256,
                ],
            };
            let (destination_liquid_outcomes, terminal, terminal_liquid_sensible_enthalpy) =
                super::physical_outcome_ledger::seal_destination_liquid_outcomes_v1(
                    &lane.ofe_id,
                    lane.ordered_destinations
                        .iter()
                        .map(|destination| (&destination.tile_id, destination.tile_fraction)),
                    terminal,
                    terminal_liquid_sensible_enthalpy,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "physical outcome destination liquid authority",
                    )
                })?;
            let value = Stage3LanePhysicalOutcomeLedgerV1 {
                support,
                lane_id: *lane_id,
                ofe_id: lane.ofe_id.clone(),
                area_basis: Stage3LaneAreaBasisV1::OfeGround,
                topology_sha256: lane.topology_configuration_sha256,
                beginning_snow_owner_sha256: beginning_digest,
                ending_snow_owner_sha256: ending_digest,
                precipitation_set_sha256: parcels.receipt_sha256,
                source_receipts_sha256: expected.source_receipts_sha256,
                beginning_ice_kg_m2: beginning_ice,
                beginning_liquid_kg_m2: beginning_liquid,
                beginning_cold_content_j_m2: beginning_cold,
                beginning_enthalpy_j_m2: beginning_enthalpy,
                ending_ice_kg_m2: ending_ice,
                ending_liquid_kg_m2: ending_liquid,
                ending_cold_content_j_m2: ending_cold,
                ending_enthalpy_j_m2: ending_enthalpy,
                solid_precipitation_kg_m2: solid,
                liquid_precipitation_kg_m2: liquid,
                precipitation_advection_j_m2: terminal_event
                    .map_or(advection, |event| event.advected_energy_j_m2),
                deposition_kg_m2: deposition,
                sublimation_kg_m2: sublimation,
                vapor_transfer_kg_m2: deposition - sublimation,
                latent_heat_j_kg: lane.aggregate_latent_heat_j_kg,
                snow_surface_temperature_k: lane.aggregate_snow_temperature_k,
                vapor_material_enthalpy_j_m2: vapor_material_enthalpy,
                melt_kg_m2: melt,
                refreeze_kg_m2: refreeze,
                terminal_liquid_kg_m2: terminal,
                terminal_liquid_sensible_enthalpy_j_m2: terminal_liquid_sensible_enthalpy,
                destination_liquid_outcomes,
                retained_liquid_kg_m2: ending_liquid,
                shortwave_j_m2: shortwave,
                longwave_j_m2: longwave,
                sensible_j_m2: sensible,
                latent_j_m2: latent,
                soil_heat_j_m2: soil_heat,
                interlayer_active_conduction_j_m2: interlayer_active,
                interlayer_lower_conduction_j_m2: interlayer_lower,
                interlayer_conduction_j_m2: interlayer_active + interlayer_lower,
                refreeze_fusion_j_m2: 0.0,
                mass_residual_kg_m2: 0.0,
                ice_residual_kg_m2: 0.0,
                liquid_residual_kg_m2: 0.0,
                vapor_residual_kg_m2: 0.0,
                energy_residual_j_m2: 0.0,
                ending_liquid_residual_kg_m2: 0.0,
                receipt_sha256: Digest32::zero(),
            };
            let ledger = Stage3LanePhysicalOutcomeLedgerV1::try_new(value.clone(), &expected)
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("physical outcome ledger closure")
                })?;
            result.insert(*lane_id, ledger);
        }
        Ok(result)
    }

    fn snow_soil_heat_receipts(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &SoilThermalSnapshot,
    ) -> Result<BTreeMap<u32, SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        let lane_to_ofe = self.covered_lane_to_ofe(&self.stage3_beginning_by_lane)?;
        let mut topology_bytes = Vec::new();
        for binding in &self.beginning.inner.surface_configuration.ofe_bindings {
            topology_bytes.extend_from_slice(&binding.production_lane_id.to_be_bytes());
            topology_bytes.extend_from_slice(binding.ofe_id.as_str().as_bytes());
            topology_bytes.push(0);
        }
        let topology_identity_sha256 = digest_bytes(&topology_bytes);
        let configuration_identity_sha256 = digest32_from_lower_hex(
            self.beginning
                .inner
                .lse_configuration
                .configuration_sha256
                .as_str(),
        )?;
        let beginning_soil_owner_identity_sha256 =
            digest32_from_lower_hex(self.beginning.inner.soil_thermal.state_sha256().as_str())?;
        let mut receipts = BTreeMap::new();
        for (lane_id, ofe_id) in lane_to_ofe {
            if let Some(receipt) = self.snow_soil_heat_receipt_for_lane(
                support,
                lane_id,
                &ofe_id,
                trial_stage3,
                trial_soil,
                topology_identity_sha256,
                configuration_identity_sha256,
                beginning_soil_owner_identity_sha256,
            )? {
                receipts.insert(lane_id, receipt);
            }
        }
        Ok(receipts)
    }

    fn snow_soil_heat_receipts_for_candidate_v1(
        &self,
        support: TimeSupport,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &DirectSoilThermalCandidate,
    ) -> Result<BTreeMap<u32, SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        match trial_soil {
            DirectSoilThermalCandidate::V1(candidate) => {
                self.snow_soil_heat_receipts(support, trial_stage3, candidate)
            }
            DirectSoilThermalCandidate::V2(_) => {
                self.snow_soil_heat_receipts_v2(support, trial_stage3, trial_soil)
            }
        }
    }

    fn retain_terminal_limiting_snow_soil_receipts(
        &self,
        mut next: BTreeMap<u32, SnowSoilHeatReceiptV1>,
        previous: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
        if self.terminal_endpoint_mode {
            for (lane_id, state) in trial_stage3 {
                if state.layers.is_empty()
                    && !crate::hydrology::stage3_has_represented_ice(state)
                    && state.detached_retained_liquid_kg_m2.to_bits() == 0.0_f64.to_bits()
                {
                    if let Some(limiting) = previous.get(lane_id) {
                        next.insert(*lane_id, limiting.clone());
                    }
                }
            }
        }
        next
    }

    fn terminal_snow_soil_heat_receipts(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        ending: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        installed_soil: &DirectSoilThermalCandidate,
        limiting: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        diagnostics: &BTreeMap<u32, (f64, f64, f64, f64)>,
    ) -> Result<
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>,
        DirectV11RealConsumerError,
    > {
        let mut result = BTreeMap::new();
        for (lane_id, ending_state) in ending {
            if !self.terminal_endpoint_mode
                || !ending_state.layers.is_empty()
                || crate::hydrology::stage3_has_represented_ice(ending_state)
                || ending_state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            {
                continue;
            }
            let beginning = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("terminal snow-soil beginning lane"),
            )?;
            let limiting_receipt =
                limiting
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "terminal snow-soil limiting receipt",
                    ))?;
            let ending_soil_ofe = installed_soil
                .read_view()
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &limiting_receipt.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "terminal snow-soil ending OFE",
                ))?;
            let snow_heat = diagnostics
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "terminal snow-soil event-integrated operand",
                ))?
                .3;
            let receipt = physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1 {
                support,
                lane_id: *lane_id,
                ofe_id: limiting_receipt.ofe_id.clone(),
                beginning_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning)?,
                ),
                ending_dormant_snow_owner_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending_state)?,
                ),
                ending_soil_owner_sha256: digest_bytes(
                    &match ending_soil_ofe {
                        DirectSoilThermalOfeReadView::V1(value) => serde_json::to_vec(value),
                        DirectSoilThermalOfeReadView::V2(value) => serde_json::to_vec(value),
                    }
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "terminal snow-soil ending soil identity",
                        )
                    })?,
                ),
                limiting_boundary_receipt_sha256: limiting_receipt.receipt_sha256,
                snow_heat_j_m2: snow_heat,
                soil_heat_j_m2: -snow_heat,
                receipt_sha256: Digest32::zero(),
            }
            .seal()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal snow-soil receipt closure")
            })?;
            result.insert(*lane_id, receipt);
        }
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn snow_soil_heat_receipt_for_lane(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        lane_id: u32,
        ofe_id: &OfeId,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: &SoilThermalSnapshot,
        topology_identity_sha256: Digest32,
        configuration_identity_sha256: Digest32,
        beginning_soil_owner_identity_sha256: Digest32,
    ) -> Result<Option<SnowSoilHeatReceiptV1>, DirectV11RealConsumerError> {
        let inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("snow-soil Stage-3 inputs"),
        )?;
        let beginning_stage = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("snow-soil beginning snow owner"),
        )?;
        let ending_stage =
            trial_stage3
                .get(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil trial snow owner",
                ))?;
        // An unresolved lane has no admissible snow thermal boundary.
        // Preserve its carried owner unchanged and emit no heat receipt or
        // soil credit; fabricating a bottom temperature/conductivity would
        // violate the canonical Stage-3 lifecycle boundary.
        let beginning_terminal = crate::hydrology::stage3_is_terminal_event_domain(beginning_stage);
        if !crate::hydrology::stage3_is_resolved_thermal_domain(beginning_stage)
            && !beginning_terminal
        {
            return Ok(None);
        }
        let beginning_bottom = if beginning_terminal {
            Wb11HydrologyKernel::project_stage3_terminal_bottom_volume_v1(
                beginning_stage,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        } else {
            Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
                beginning_stage,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        };
        let ending_bottom_temperature_k =
            if crate::hydrology::stage3_is_resolved_thermal_domain(ending_stage) {
                Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
                    ending_stage,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?
                .temperature_k
            } else if crate::hydrology::stage3_is_terminal_event_domain(ending_stage) {
                Wb11HydrologyKernel::project_stage3_terminal_bottom_volume_v1(
                    ending_stage,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?
                .temperature_k
            } else if self.terminal_endpoint_mode {
                // Dormancy has no projectable snow node.  A terminal-specific
                // event-integrated receipt must carry this custody; the
                // persistent Crank--Nicolson receipt cannot be completed by
                // fabricating an endpoint temperature.
                return Ok(None);
            } else {
                return Ok(None);
            };
        let configured_ofe = self
            .beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-soil configured OFE",
            ))?;
        let configured_top = configured_ofe.soil_interface_layers.first().ok_or(
            DirectV11RealConsumerError::Identity("snow-soil configured top layer"),
        )?;
        let beginning_soil = self.beginning.inner.soil_thermal.v1().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
        let beginning_ofe = beginning_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-soil beginning OFE",
            ))?;
        let ending_ofe = trial_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == *ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity("snow-soil trial OFE"))?;
        let beginning_top =
            beginning_ofe
                .ordered_layers
                .first()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil beginning top layer",
                ))?;
        let ending_top =
            ending_ofe
                .ordered_layers
                .first()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil trial top layer",
                ))?;
        if beginning_top.layer_id != configured_top.layer_id
            || ending_top.layer_id != configured_top.layer_id
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-soil ordered top-layer identity",
            ));
        }
        seal_snow_soil_heat_receipt(&SnowSoilReceiptInputs {
            support,
            duration_s: f64::from_bits(self.interval.lse_forcing.interval_s.to_bits()),
            lane_id,
            ofe_id,
            topology_identity_sha256,
            configuration_identity_sha256,
            beginning_soil_owner_identity_sha256,
            beginning_stage,
            ending_stage,
            ending_ofe,
            configured_top,
            beginning_top,
            ending_top,
            beginning_bottom_temperature_k: beginning_bottom.temperature_k,
            beginning_bottom_thickness_m: beginning_bottom.thickness_m,
            beginning_bottom_conductivity_w_m_k: beginning_bottom.thermal_conductivity_w_m_k,
            beginning_snow_owner_sha256: beginning_bottom.beginning_stage3_state_sha256,
            ending_bottom_temperature_k,
        })
        .map(Some)
    }

    fn soil_top_boundary_credits(
        &self,
        receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    ) -> Result<Vec<SoilThermalTopBoundaryCreditV1>, DirectV11RealConsumerError> {
        receipts
            .values()
            .map(|receipt| {
                crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
                    .map_err(|error| {
                        DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                    })?;
                let beginning_soil = self.beginning.inner.soil_thermal.read_view();
                let beginning_ofe = beginning_soil
                    .ordered_ofes()
                    .into_iter()
                    .find(|value| value.ofe_id() == &receipt.ofe_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "soil credit beginning OFE",
                    ))?;
                let first_layer = beginning_ofe.ordered_layers().into_iter().next().ok_or(
                    DirectV11RealConsumerError::Identity("soil credit first layer"),
                )?;
                crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt_installed_join(
                    receipt,
                    first_layer.layer_id(),
                    receipt.snow_candidate_ending_identity_sha256,
                    receipt.soil_candidate_ending_identity_sha256,
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                })?;
                Ok(SoilThermalTopBoundaryCreditV1 {
                    lane_id: receipt.lane_id,
                    ofe_id: receipt.ofe_id.clone(),
                    first_layer_id: first_layer.layer_id().clone(),
                    beginning_owner_id: self.beginning.inner.soil_thermal.owner_id().clone(),
                    beginning_configuration_sha256: self
                        .beginning
                        .inner
                        .soil_thermal
                        .configuration_sha256()
                        .clone(),
                    beginning_state_sha256: self
                        .beginning
                        .inner
                        .soil_thermal
                        .state_sha256()
                        .clone(),
                    support_start_ns: i64::try_from(receipt.support.start_ns().get()).map_err(
                        |_| DirectV11RealConsumerError::Identity("soil heat support start"),
                    )?,
                    support_end_ns: i64::try_from(receipt.support.end_ns().get()).map_err(
                        |_| DirectV11RealConsumerError::Identity("soil heat support end"),
                    )?,
                    accepted_positive_downward_j_m2_ofe_ground: receipt
                        .accepted_heat_j_m2_ofe_ground,
                    soil_thermal_credit_j_m2_ofe_ground: receipt
                        .soil_candidate_heat_j_m2_ofe_ground,
                    snow_soil_heat_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                        receipt.receipt_sha256,
                    ))
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("soil heat receipt digest")
                    })?,
                })
            })
            .collect()
    }

    fn validate_snow_soil_heat_receipt_iterate_joins(
        &self,
        receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        trial_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        trial_soil: DirectSoilThermalReadView<'_>,
    ) -> Result<(), DirectV11RealConsumerError> {
        for (lane_id, receipt) in receipts {
            let stage3 = trial_stage3
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil iterate lane join",
                ))?;
            if self.terminal_endpoint_mode
                && !crate::hydrology::stage3_has_represented_ice(stage3)
                && stage3.layers.is_empty()
            {
                // Terminal dormancy has no snow node. Its last limiting
                // receipt is joined separately by the terminal receipt path.
                continue;
            }
            let soil_ofe = trial_soil
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &receipt.ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "snow-soil iterate OFE join",
                ))?;
            let first_layer = soil_ofe.ordered_layers().into_iter().next().ok_or(
                DirectV11RealConsumerError::Identity("snow-soil iterate first-layer join"),
            )?;
            let snow_sha256 = digest_bytes(&serde_json::to_vec(stage3).map_err(|_| {
                DirectV11RealConsumerError::Identity("snow-soil iterate snow serialization")
            })?);
            let soil_sha256 = digest_bytes(
                &match soil_ofe {
                    DirectSoilThermalOfeReadView::V1(value) => serde_json::to_vec(value),
                    DirectSoilThermalOfeReadView::V2(value) => serde_json::to_vec(value),
                }
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("snow-soil iterate soil serialization")
                })?,
            );
            crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt_installed_join(
                receipt,
                first_layer.layer_id(),
                snow_sha256,
                soil_sha256,
            )
            .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
        }
        Ok(())
    }
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    fn execute_terminal_physical_reuse(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let endpoint = self.precomputed_terminal_accepted.take().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse endpoint"),
        )?;
        let seed = self.terminal_physical_reuse_seed.take().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse physical seed"),
        )?;
        let beginning_owner_states = input
            .staged_resource_owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()?;
        let beginning_owner_set_sha256 =
            openwepp_coupled_time::complete_owner_set_digest(&beginning_owner_states).map_err(
                |_| DirectV11RealConsumerError::Identity("covered terminal reuse beginning owner"),
            )?;
        if endpoint.accepted_slab_sha256 != input.accepted_slab_receipt.slab_id().digest() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse accepted slab",
            ));
        }
        if endpoint.accepted_envelope_support != input.support {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse support",
            ));
        }
        if endpoint.beginning_owner_set_sha256 != beginning_owner_set_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse beginning owner",
            ));
        }
        if endpoint.beginning_pending_terminal_parcels != self.pending_terminal_parcels {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse pending parcels",
            ));
        }
        if endpoint.pre_event_authority_sha256
            != precomputed_terminal_pre_event_authority_sha256_v1(&endpoint)?
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse pre-event seal",
            ));
        }
        if seed.physical_authority_sha256
            != precomputed_terminal_physical_authority_sha256_v1(&endpoint)?
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse physical authority",
            ));
        }
        if seed.soil_candidate != endpoint.carrier_phase.soil_candidate
            || seed.soil_continuation.as_ref()
                != endpoint.carrier_phase.ending_candidates.soil_continuation()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal physical reuse soil continuation",
            ));
        }
        let child_replay = self.last_wb14_child_replay_bytes.as_deref().ok_or(
            DirectV11RealConsumerError::Identity("covered terminal reuse WB14 child replay"),
        )?;
        let trial_binding = crate::direct_runtime::wb14_child_replay_binding(child_replay)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered terminal reuse WB14 trial binding")
            })?;
        validate_covered_terminal_reuse_trial_binding_v1(
            trial_binding,
            self.wb14_coupled_child_binding,
            endpoint.carrier_phase.transition.boundary.support,
            endpoint.wb14_replay_trial_sha256,
            endpoint.wb14_replay_beginning_owner_set_sha256,
        )?;
        crate::direct_runtime::validate_wb14_child_replay_binding(child_replay, trial_binding)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered terminal reuse WB14 child validation")
            })?;
        if let Some(parent) = self.last_wb14_parent_replay_bytes.as_deref() {
            crate::direct_runtime::validate_wb14_parent_replay(child_replay, parent).map_err(
                |_| DirectV11RealConsumerError::Identity("covered terminal reuse WB14 parent"),
            )?;
        }
        let child_sha256 = digest_bytes(child_replay);
        let parent_sha256 = self
            .last_wb14_parent_replay_bytes
            .as_deref()
            .map(digest_bytes);
        if endpoint.wb14_child_receipt_set_sha256 != child_sha256
            || endpoint.wb14_parent_receipt_set_sha256 != parent_sha256
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered terminal reuse WB14 authorization",
            ));
        }
        let publication_posture = match self.terminal_publication_posture {
            TerminalPublicationPostureV1::RetainFinalWithDeferredNativeV2Soil {
                pre_event_authority_sha256,
            } if pre_event_authority_sha256 == endpoint.pre_event_authority_sha256 => {
                AcceptedPublicationFinalizationPostureV1::RetainFinalWithDeferredNativeV2Soil {
                    pre_event_authority_sha256,
                }
            }
            TerminalPublicationPostureV1::RetainFinal => {
                AcceptedPublicationFinalizationPostureV1::RetainFinal
            }
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal reuse publication posture",
                ));
            }
        };
        let (output, candidate, support_receipt) =
            finalize_v11_imported_segment_with_soil_continuation(
                &self.beginning,
                input,
                &seed.envelope,
                Some(&seed.compositional_envelopes),
                Some(
                    self.ending
                        .as_ref()
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "covered terminal reuse precomputed physical ending",
                        ))?,
                ),
                Some(&seed.soil_candidate),
                seed.soil_continuation.as_ref(),
                seed.ending_snow_owner_bytes,
                self.day_index,
                self.interval_index,
                self.interval,
                &seed.soil_top_boundary_credits,
                self.last_physical_outcome_ledgers.as_ref().ok_or(
                    DirectV11RealConsumerError::Identity(
                        "covered terminal reuse physical outcome ledgers",
                    ),
                )?,
                publication_posture,
            )?;
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        Ok(output)
    }

    fn execute_ordinary_physical_reuse(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let seed = self.ordinary_physical_reuse_seed.take().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary physical reuse seed"),
        )?;
        validate_covered_ordinary_physical_reuse_gate_v1(
            seed.physical_authority,
            covered_ordinary_physical_authority_v1(input)?,
            self.terminal_endpoint_mode,
        )?;
        let child_replay = self.last_wb14_child_replay_bytes.as_deref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child replay"),
        )?;
        let (rebound_child, rebound_parent) =
            crate::direct_runtime::rebind_wb14_replay_to_accepted_slab(
                child_replay,
                self.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
            )
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child reseal")
            })?;
        crate::direct_runtime::validate_wb14_child_replay_binding(
            &rebound_child,
            self.wb14_coupled_child_binding,
        )
        .map_err(|_| {
            DirectV11RealConsumerError::Identity("covered ordinary reuse WB14 child validation")
        })?;
        if let Some(parent) = rebound_parent.as_deref() {
            crate::direct_runtime::validate_wb14_parent_replay(&rebound_child, parent).map_err(
                |_| {
                    DirectV11RealConsumerError::Identity(
                        "covered ordinary reuse WB14 parent validation",
                    )
                },
            )?;
        }
        self.last_wb14_child_receipt_set_sha256 = Some(digest32_hex(digest_bytes(&rebound_child)));
        self.last_wb14_parent_receipt_set_sha256 = rebound_parent
            .as_ref()
            .map(|bytes| digest32_hex(digest_bytes(bytes)));
        self.last_wb14_child_replay_bytes = Some(rebound_child);
        self.last_wb14_parent_replay_bytes = rebound_parent;
        let ending_stage3 =
            self.ending_stage3_by_lane
                .as_ref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered ordinary reuse Stage-3 ending",
                ))?;
        let lanes = self.last_lane_boundary_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse lane receipts"),
        )?;
        let destinations = self.last_final_boundary_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse boundary receipts"),
        )?;
        let soil = self.last_snow_soil_heat_receipts.as_ref().ok_or(
            DirectV11RealConsumerError::Identity("covered ordinary reuse snow-soil receipts"),
        )?;
        let ending_snow_owner_bytes =
            canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
                ending_stage3,
                &self.pending_terminal_parcels,
                lanes,
                destinations,
            )?;
        let soil_credits = self.soil_top_boundary_credits(soil)?;
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &self.beginning,
            input,
            &seed.envelope,
            None,
            None,
            None,
            ending_snow_owner_bytes,
            self.day_index,
            self.interval_index,
            self.interval,
            &soil_credits,
            self.last_physical_outcome_ledgers.as_ref().ok_or(
                DirectV11RealConsumerError::Identity(
                    "covered ordinary reuse physical outcome ledgers",
                ),
            )?,
            AcceptedPublicationFinalizationPostureV1::RetainFinal,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        Ok(output)
    }
}
