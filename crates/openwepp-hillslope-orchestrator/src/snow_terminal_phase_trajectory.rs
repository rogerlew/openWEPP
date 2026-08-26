//! Test-only trajectory models for terminal snow phase chronology.
//!
//! The models retain ordered forcing segments and event histories. They are
//! characterization evidence only and are not imported by production code.

const LF_J_KG: f64 = 333_600.0;
const MASS_TOL_KG_M2: f64 = 1.0e-9;
const ENERGY_TOL_J_M2: f64 = 1.0e-6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrajectoryModel {
    ReleasedOrderedTrajectory,
    EventDrivenFrostHybrid,
    TimeResolvedComplementarity,
    ExistingSnowFrostSubtype,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrajectoryEventKind {
    Meltout,
    SublimationExhaustion,
    Reappearance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TrajectoryEvent {
    pub kind: TrajectoryEventKind,
    pub tick_ns: u128,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TrajectoryState {
    pub pack_ice_kg_m2: f64,
    pub surface_frost_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ForcingSegment {
    pub duration_ns: u128,
    pub vapor_mass_into_snow_kg_m2: f64,
    pub latent_heat_j_kg: f64,
    pub latent_energy_into_snow_j_m2: f64,
    pub external_liquid_kg_m2: f64,
    pub complete_energy_j_m2: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct TrajectoryLedger {
    pub deposition_kg_m2: f64,
    pub sublimation_kg_m2: f64,
    pub melt_kg_m2: f64,
    pub refrozen_kg_m2: f64,
    pub external_liquid_kg_m2: f64,
    pub complete_energy_j_m2: f64,
    pub cold_content_change_j_m2: f64,
    pub unallocated_energy_j_m2: f64,
    pub vapor_energy_residual_j_m2: f64,
    pub solid_residual_kg_m2: f64,
    pub liquid_residual_kg_m2: f64,
    pub energy_residual_j_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TrajectoryOutcome {
    pub model: TrajectoryModel,
    pub ending: TrajectoryState,
    pub events: Vec<TrajectoryEvent>,
    pub ledger: TrajectoryLedger,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrajectoryFailure {
    DomainOrNonFinite,
    VaporLatentMismatch,
    SublimationOverdraw,
    Closure,
    UnsupportedFrostInput,
    TickOverflow,
    RestartSchema,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Qualification {
    RequiresPostEventEnergyRecipient,
    UnresolvedSolidEnergyCoexistence,
    EventDrivenModelNotInstantiated,
    MissingSimultaneousEquilibriumAuthority,
    IncompleteSnowOwnerSchema,
}

fn finite_nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0 && !(value == 0.0 && value.is_sign_negative())
}

fn validate_state(state: TrajectoryState) -> Result<(), TrajectoryFailure> {
    if [
        state.pack_ice_kg_m2,
        state.surface_frost_kg_m2,
        state.liquid_kg_m2,
        state.cold_content_j_m2,
    ]
    .into_iter()
    .all(finite_nonnegative)
    {
        if state.pack_ice_kg_m2 + state.surface_frost_kg_m2 <= MASS_TOL_KG_M2
            && state.cold_content_j_m2 > ENERGY_TOL_J_M2
        {
            Err(TrajectoryFailure::DomainOrNonFinite)
        } else {
            Ok(())
        }
    } else {
        Err(TrajectoryFailure::DomainOrNonFinite)
    }
}

fn validate_segment(segment: ForcingSegment) -> Result<(), TrajectoryFailure> {
    if segment.duration_ns == 0
        || !segment.vapor_mass_into_snow_kg_m2.is_finite()
        || !finite_nonnegative(segment.latent_heat_j_kg)
        || (segment.vapor_mass_into_snow_kg_m2 != 0.0 && segment.latent_heat_j_kg == 0.0)
        || !segment.latent_energy_into_snow_j_m2.is_finite()
        || !finite_nonnegative(segment.external_liquid_kg_m2)
        || !segment.complete_energy_j_m2.is_finite()
    {
        return Err(TrajectoryFailure::DomainOrNonFinite);
    }
    let reconstructed = segment.vapor_mass_into_snow_kg_m2 * segment.latent_heat_j_kg;
    let scale = 1.0_f64.max(reconstructed.abs() + segment.latent_energy_into_snow_j_m2.abs());
    if !reconstructed.is_finite()
        || (reconstructed - segment.latent_energy_into_snow_j_m2).abs()
            > ENERGY_TOL_J_M2.max(1.0e-12 * scale)
    {
        return Err(TrajectoryFailure::VaporLatentMismatch);
    }
    Ok(())
}

fn event_tick(
    start_ns: u128,
    duration_ns: u128,
    numerator: f64,
    denominator: f64,
) -> Result<u128, TrajectoryFailure> {
    if denominator <= 0.0 || numerator <= 0.0 {
        return Ok(start_ns);
    }
    let fraction = (numerator / denominator).clamp(0.0, 1.0);
    start_ns
        .checked_add(((duration_ns as f64) * fraction).ceil() as u128)
        .ok_or(TrajectoryFailure::TickOverflow)
}

fn remove_solid(
    state: &mut TrajectoryState,
    requested_kg_m2: f64,
) -> Result<(), TrajectoryFailure> {
    let available = state.pack_ice_kg_m2 + state.surface_frost_kg_m2;
    if requested_kg_m2 > available + MASS_TOL_KG_M2 {
        return Err(TrajectoryFailure::SublimationOverdraw);
    }
    let frost = requested_kg_m2.min(state.surface_frost_kg_m2);
    state.surface_frost_kg_m2 -= frost;
    let pack = requested_kg_m2 - frost;
    state.pack_ice_kg_m2 -= pack;
    Ok(())
}

fn apply_energy(state: &mut TrajectoryState, energy_j_m2: f64) -> (f64, f64, f64, f64) {
    if energy_j_m2 < state.cold_content_j_m2 {
        let deficit = state.cold_content_j_m2 - energy_j_m2;
        let refrozen = (deficit / LF_J_KG).min(state.liquid_kg_m2);
        state.liquid_kg_m2 -= refrozen;
        state.pack_ice_kg_m2 += refrozen;
        state.cold_content_j_m2 = deficit - LF_J_KG * refrozen;
        return (0.0, refrozen, 0.0, energy_j_m2);
    }
    let cold_change = state.cold_content_j_m2;
    let phase_energy = energy_j_m2 - cold_change;
    state.cold_content_j_m2 = 0.0;
    let available = state.pack_ice_kg_m2 + state.surface_frost_kg_m2;
    let melt = (phase_energy / LF_J_KG).min(available);
    let frost_melt = melt.min(state.surface_frost_kg_m2);
    state.surface_frost_kg_m2 -= frost_melt;
    state.pack_ice_kg_m2 -= melt - frost_melt;
    state.liquid_kg_m2 += melt;
    let unallocated = (phase_energy - LF_J_KG * melt).max(0.0);
    (melt, 0.0, unallocated, cold_change)
}

fn finalize(
    model: TrajectoryModel,
    beginning: TrajectoryState,
    ending: TrajectoryState,
    events: Vec<TrajectoryEvent>,
    mut ledger: TrajectoryLedger,
    cold_change_j_m2: f64,
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    validate_state(ending)?;
    ledger.cold_content_change_j_m2 = cold_change_j_m2;
    ledger.solid_residual_kg_m2 = beginning.pack_ice_kg_m2
        + beginning.surface_frost_kg_m2
        + ledger.deposition_kg_m2
        + ledger.refrozen_kg_m2
        - ledger.sublimation_kg_m2
        - ledger.melt_kg_m2
        - ending.pack_ice_kg_m2
        - ending.surface_frost_kg_m2;
    ledger.liquid_residual_kg_m2 =
        beginning.liquid_kg_m2 + ledger.external_liquid_kg_m2 + ledger.melt_kg_m2
            - ledger.refrozen_kg_m2
            - ending.liquid_kg_m2;
    ledger.energy_residual_j_m2 =
        ledger.complete_energy_j_m2 - ledger.cold_content_change_j_m2 - LF_J_KG * ledger.melt_kg_m2
            + LF_J_KG * ledger.refrozen_kg_m2
            - ledger.unallocated_energy_j_m2;
    if [
        ledger.solid_residual_kg_m2,
        ledger.liquid_residual_kg_m2,
        ledger.energy_residual_j_m2,
        ledger.vapor_energy_residual_j_m2,
    ]
    .into_iter()
    .any(|value| !value.is_finite())
        || ledger.solid_residual_kg_m2.abs() > MASS_TOL_KG_M2
        || ledger.liquid_residual_kg_m2.abs() > MASS_TOL_KG_M2
        || ledger.energy_residual_j_m2.abs() > ENERGY_TOL_J_M2
        || ledger.vapor_energy_residual_j_m2.abs() > ENERGY_TOL_J_M2
    {
        return Err(TrajectoryFailure::Closure);
    }
    Ok(TrajectoryOutcome {
        model,
        ending,
        events,
        ledger,
    })
}

fn run_ordered(
    model: TrajectoryModel,
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    validate_state(beginning)?;
    let mut state = beginning;
    let mut events = Vec::new();
    let mut ledger = TrajectoryLedger::default();
    let mut cursor_ns = 0_u128;
    let mut cold_change = 0.0;
    for segment in segments {
        validate_segment(*segment)?;
        let segment_end = cursor_ns
            .checked_add(segment.duration_ns)
            .ok_or(TrajectoryFailure::TickOverflow)?;
        let solid_at_start = state.pack_ice_kg_m2 + state.surface_frost_kg_m2;
        let sublimation = (-segment.vapor_mass_into_snow_kg_m2).max(0.0);
        remove_solid(&mut state, sublimation)?;
        let solid_before_energy = state.pack_ice_kg_m2 + state.surface_frost_kg_m2;
        if solid_at_start > MASS_TOL_KG_M2 && solid_before_energy <= MASS_TOL_KG_M2 {
            events.push(TrajectoryEvent {
                kind: TrajectoryEventKind::SublimationExhaustion,
                tick_ns: event_tick(cursor_ns, segment.duration_ns, solid_at_start, sublimation)?,
            });
        }
        state.liquid_kg_m2 += segment.external_liquid_kg_m2;
        let cold_before = state.cold_content_j_m2;
        let energy_needed =
            cold_before + LF_J_KG * (state.pack_ice_kg_m2 + state.surface_frost_kg_m2);
        let (melt, refrozen, unallocated, _) =
            apply_energy(&mut state, segment.complete_energy_j_m2);
        cold_change += cold_before - state.cold_content_j_m2;
        let solid_after_energy = state.pack_ice_kg_m2 + state.surface_frost_kg_m2;
        if solid_before_energy > MASS_TOL_KG_M2 && solid_after_energy <= MASS_TOL_KG_M2 {
            events.push(TrajectoryEvent {
                kind: TrajectoryEventKind::Meltout,
                tick_ns: event_tick(
                    cursor_ns,
                    segment.duration_ns,
                    energy_needed,
                    segment.complete_energy_j_m2,
                )?,
            });
        }
        let deposition = segment.vapor_mass_into_snow_kg_m2.max(0.0);
        if deposition > 0.0 {
            state.pack_ice_kg_m2 += deposition;
            if solid_after_energy <= MASS_TOL_KG_M2 {
                events.push(TrajectoryEvent {
                    kind: TrajectoryEventKind::Reappearance,
                    tick_ns: segment_end,
                });
            }
        }
        ledger.deposition_kg_m2 += deposition;
        ledger.sublimation_kg_m2 += sublimation;
        ledger.melt_kg_m2 += melt;
        ledger.refrozen_kg_m2 += refrozen;
        ledger.external_liquid_kg_m2 += segment.external_liquid_kg_m2;
        ledger.complete_energy_j_m2 += segment.complete_energy_j_m2;
        ledger.unallocated_energy_j_m2 += unallocated;
        ledger.vapor_energy_residual_j_m2 += segment.latent_energy_into_snow_j_m2
            - segment.vapor_mass_into_snow_kg_m2 * segment.latent_heat_j_kg;
        cursor_ns = segment_end;
    }
    finalize(model, beginning, state, events, ledger, cold_change)
}

pub(crate) fn released_ordered_trajectory(
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    if beginning.surface_frost_kg_m2 > 0.0 {
        return Err(TrajectoryFailure::UnsupportedFrostInput);
    }
    run_ordered(
        TrajectoryModel::ReleasedOrderedTrajectory,
        beginning,
        segments,
    )
}

pub(crate) fn event_driven_frost_hybrid(
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    run_hybrid(TrajectoryModel::EventDrivenFrostHybrid, beginning, segments)
}

fn run_hybrid(
    model: TrajectoryModel,
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    validate_state(beginning)?;
    let mut state = beginning;
    let mut events = Vec::new();
    let mut ledger = TrajectoryLedger::default();
    let mut cursor_ns = 0_u128;
    let mut cold_change = 0.0;
    for segment in segments {
        validate_segment(*segment)?;
        let segment_end = cursor_ns
            .checked_add(segment.duration_ns)
            .ok_or(TrajectoryFailure::TickOverflow)?;
        if state.pack_ice_kg_m2 > MASS_TOL_KG_M2 && state.surface_frost_kg_m2 > MASS_TOL_KG_M2 {
            return Err(TrajectoryFailure::UnsupportedFrostInput);
        }
        if (-segment.vapor_mass_into_snow_kg_m2).max(0.0)
            > state.pack_ice_kg_m2 + state.surface_frost_kg_m2 + MASS_TOL_KG_M2
        {
            return Err(TrajectoryFailure::SublimationOverdraw);
        }
        let began_as_frost =
            state.pack_ice_kg_m2 <= MASS_TOL_KG_M2 && state.surface_frost_kg_m2 > MASS_TOL_KG_M2;
        let input = crate::snow_terminal_phase_competition::TerminalPhaseInputs {
            beginning_pack_ice_kg_m2: state.pack_ice_kg_m2 + state.surface_frost_kg_m2,
            beginning_surface_frost_kg_m2: 0.0,
            beginning_liquid_kg_m2: state.liquid_kg_m2,
            beginning_cold_content_j_m2: state.cold_content_j_m2,
            deposition_kg_m2: segment.vapor_mass_into_snow_kg_m2.max(0.0),
            sublimation_kg_m2: (-segment.vapor_mass_into_snow_kg_m2).max(0.0),
            external_liquid_kg_m2: segment.external_liquid_kg_m2,
            non_vapor_energy_j_m2: segment.complete_energy_j_m2
                - segment.latent_energy_into_snow_j_m2,
            vapor_latent_energy_j_m2: segment.latent_energy_into_snow_j_m2,
            complete_energy_j_m2: segment.complete_energy_j_m2,
            support_seconds: segment.duration_ns as f64 / 1.0e9,
        };
        let candidate = crate::snow_terminal_phase_competition::simultaneous_complementarity(input)
            .map_err(|_| TrajectoryFailure::Closure)?;
        let reappeared = matches!(
            candidate.event,
            crate::snow_terminal_phase_competition::TerminalEventChronology::Reappeared
        );
        if matches!(
            candidate.event,
            crate::snow_terminal_phase_competition::TerminalEventChronology::Interior
                | crate::snow_terminal_phase_competition::TerminalEventChronology::AtEnd
        ) {
            events.push(TrajectoryEvent {
                kind: TrajectoryEventKind::Meltout,
                tick_ns: segment_end,
            });
        } else if reappeared {
            events.push(TrajectoryEvent {
                kind: TrajectoryEventKind::Reappearance,
                tick_ns: segment_end,
            });
        }
        cold_change += state.cold_content_j_m2 - candidate.ending_cold_content_j_m2;
        let ending_solid = candidate.ending_pack_ice_kg_m2;
        state = TrajectoryState {
            pack_ice_kg_m2: if began_as_frost || reappeared {
                0.0
            } else {
                ending_solid
            },
            surface_frost_kg_m2: if began_as_frost || reappeared {
                ending_solid
            } else {
                0.0
            },
            liquid_kg_m2: candidate.ending_liquid_kg_m2,
            cold_content_j_m2: candidate.ending_cold_content_j_m2,
        };
        ledger.deposition_kg_m2 += input.deposition_kg_m2;
        ledger.sublimation_kg_m2 += input.sublimation_kg_m2;
        ledger.melt_kg_m2 += candidate.melt_kg_m2;
        ledger.refrozen_kg_m2 += candidate.refrozen_kg_m2;
        ledger.external_liquid_kg_m2 += segment.external_liquid_kg_m2;
        ledger.complete_energy_j_m2 += segment.complete_energy_j_m2;
        ledger.unallocated_energy_j_m2 += candidate.unallocated_energy_j_m2;
        ledger.vapor_energy_residual_j_m2 += segment.latent_energy_into_snow_j_m2
            - segment.vapor_mass_into_snow_kg_m2 * segment.latent_heat_j_kg;
        cursor_ns = segment_end;
    }
    finalize(model, beginning, state, events, ledger, cold_change)
}

pub(crate) fn time_resolved_complementarity(
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<TrajectoryOutcome, TrajectoryFailure> {
    validate_state(beginning)?;
    if beginning.surface_frost_kg_m2 > 0.0 {
        return Err(TrajectoryFailure::UnsupportedFrostInput);
    }
    let mut state = beginning;
    let mut events = Vec::new();
    let mut ledger = TrajectoryLedger::default();
    let mut cursor_ns = 0_u128;
    let mut cold_change = 0.0;
    for segment in segments {
        validate_segment(*segment)?;
        let segment_end = cursor_ns
            .checked_add(segment.duration_ns)
            .ok_or(TrajectoryFailure::TickOverflow)?;
        let input = crate::snow_terminal_phase_competition::TerminalPhaseInputs {
            beginning_pack_ice_kg_m2: state.pack_ice_kg_m2,
            beginning_surface_frost_kg_m2: 0.0,
            beginning_liquid_kg_m2: state.liquid_kg_m2,
            beginning_cold_content_j_m2: state.cold_content_j_m2,
            deposition_kg_m2: segment.vapor_mass_into_snow_kg_m2.max(0.0),
            sublimation_kg_m2: (-segment.vapor_mass_into_snow_kg_m2).max(0.0),
            external_liquid_kg_m2: segment.external_liquid_kg_m2,
            non_vapor_energy_j_m2: segment.complete_energy_j_m2
                - segment.latent_energy_into_snow_j_m2,
            vapor_latent_energy_j_m2: segment.latent_energy_into_snow_j_m2,
            complete_energy_j_m2: segment.complete_energy_j_m2,
            support_seconds: segment.duration_ns as f64 / 1.0e9,
        };
        let candidate = crate::snow_terminal_phase_competition::simultaneous_complementarity(input)
            .map_err(|_| TrajectoryFailure::Closure)?;
        match candidate.event {
            crate::snow_terminal_phase_competition::TerminalEventChronology::Interior
            | crate::snow_terminal_phase_competition::TerminalEventChronology::AtEnd => {
                events.push(TrajectoryEvent {
                    kind: TrajectoryEventKind::Meltout,
                    tick_ns: segment_end,
                });
            }
            crate::snow_terminal_phase_competition::TerminalEventChronology::Reappeared => {
                events.push(TrajectoryEvent {
                    kind: TrajectoryEventKind::Reappearance,
                    tick_ns: segment_end,
                });
            }
            _ => {}
        }
        cold_change += state.cold_content_j_m2 - candidate.ending_cold_content_j_m2;
        state = TrajectoryState {
            pack_ice_kg_m2: candidate.ending_pack_ice_kg_m2,
            surface_frost_kg_m2: 0.0,
            liquid_kg_m2: candidate.ending_liquid_kg_m2,
            cold_content_j_m2: candidate.ending_cold_content_j_m2,
        };
        ledger.deposition_kg_m2 += input.deposition_kg_m2;
        ledger.sublimation_kg_m2 += input.sublimation_kg_m2;
        ledger.melt_kg_m2 += candidate.melt_kg_m2;
        ledger.refrozen_kg_m2 += candidate.refrozen_kg_m2;
        ledger.external_liquid_kg_m2 += segment.external_liquid_kg_m2;
        ledger.complete_energy_j_m2 += segment.complete_energy_j_m2;
        ledger.unallocated_energy_j_m2 += candidate.unallocated_energy_j_m2;
        ledger.vapor_energy_residual_j_m2 += segment.latent_energy_into_snow_j_m2
            - segment.vapor_mass_into_snow_kg_m2 * segment.latent_heat_j_kg;
        cursor_ns = segment_end;
    }
    finalize(
        TrajectoryModel::TimeResolvedComplementarity,
        beginning,
        state,
        events,
        ledger,
        cold_change,
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TaggedSnowOwnerEnvelope {
    pub state: TrajectoryState,
}

impl TaggedSnowOwnerEnvelope {
    pub(crate) fn canonical_bytes(self) -> Vec<u8> {
        let mut bytes = b"OPENWEPP_TEST_TAGGED_SNOW_FROST_V1".to_vec();
        for value in [
            self.state.pack_ice_kg_m2,
            self.state.surface_frost_kg_m2,
            self.state.liquid_kg_m2,
            self.state.cold_content_j_m2,
        ] {
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        bytes
    }

    pub(crate) fn restore(bytes: &[u8]) -> Result<Self, TrajectoryFailure> {
        const PREFIX: &[u8] = b"OPENWEPP_TEST_TAGGED_SNOW_FROST_V1";
        if bytes.len() != PREFIX.len() + 32 || !bytes.starts_with(PREFIX) {
            return Err(TrajectoryFailure::RestartSchema);
        }
        let mut values = [0.0; 4];
        for (index, value) in values.iter_mut().enumerate() {
            let start = PREFIX.len() + index * 8;
            let raw: [u8; 8] = bytes[start..start + 8]
                .try_into()
                .map_err(|_| TrajectoryFailure::RestartSchema)?;
            *value = f64::from_bits(u64::from_be_bytes(raw));
        }
        let envelope = Self {
            state: TrajectoryState {
                pack_ice_kg_m2: values[0],
                surface_frost_kg_m2: values[1],
                liquid_kg_m2: values[2],
                cold_content_j_m2: values[3],
            },
        };
        validate_state(envelope.state)?;
        Ok(envelope)
    }
}

pub(crate) fn existing_snow_frost_subtype(
    beginning: TrajectoryState,
    segments: &[ForcingSegment],
) -> Result<(TrajectoryOutcome, TaggedSnowOwnerEnvelope), TrajectoryFailure> {
    let mut outcome = event_driven_frost_hybrid(beginning, segments)?;
    outcome.model = TrajectoryModel::ExistingSnowFrostSubtype;
    let envelope = TaggedSnowOwnerEnvelope {
        state: outcome.ending,
    };
    let replay = TaggedSnowOwnerEnvelope::restore(&envelope.canonical_bytes())?;
    if replay != envelope {
        return Err(TrajectoryFailure::RestartSchema);
    }
    Ok((outcome, envelope))
}

pub(crate) fn qualification(outcome: &TrajectoryOutcome) -> Qualification {
    if outcome.ending.pack_ice_kg_m2 + outcome.ending.surface_frost_kg_m2 > MASS_TOL_KG_M2
        && outcome.ledger.unallocated_energy_j_m2 > ENERGY_TOL_J_M2
    {
        return Qualification::UnresolvedSolidEnergyCoexistence;
    }
    match outcome.model {
        TrajectoryModel::ReleasedOrderedTrajectory => {
            Qualification::RequiresPostEventEnergyRecipient
        }
        TrajectoryModel::EventDrivenFrostHybrid => Qualification::EventDrivenModelNotInstantiated,
        TrajectoryModel::TimeResolvedComplementarity => {
            Qualification::MissingSimultaneousEquilibriumAuthority
        }
        TrajectoryModel::ExistingSnowFrostSubtype => Qualification::IncompleteSnowOwnerSchema,
    }
}

pub(crate) fn segment_from_real_endpoint(
    endpoint: &crate::snow_stage3_v11_attachment::RealDiscreteCompleteEndpointEvidenceV1,
) -> ForcingSegment {
    let vapor_mass =
        f64::from_bits(endpoint.deposition_bits) - f64::from_bits(endpoint.sublimation_bits);
    let latent = f64::from_bits(endpoint.latent_energy_bits);
    let latent_heat = if vapor_mass == 0.0 {
        0.0
    } else {
        latent / vapor_mass
    };
    ForcingSegment {
        duration_ns: endpoint.support.duration_ns(),
        vapor_mass_into_snow_kg_m2: vapor_mass,
        latent_heat_j_kg: latent_heat,
        latent_energy_into_snow_j_m2: latent,
        external_liquid_kg_m2: f64::from_bits(endpoint.external_liquid_bits),
        complete_energy_j_m2: f64::from_bits(endpoint.complete_energy_bits),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LS: f64 = 2_834_000.0;

    fn state(ice: f64, liquid: f64, cold: f64) -> TrajectoryState {
        TrajectoryState {
            pack_ice_kg_m2: ice,
            surface_frost_kg_m2: 0.0,
            liquid_kg_m2: liquid,
            cold_content_j_m2: cold,
        }
    }

    fn segment(duration_ns: u128, vapor: f64, liquid: f64, energy: f64) -> ForcingSegment {
        ForcingSegment {
            duration_ns,
            vapor_mass_into_snow_kg_m2: vapor,
            latent_heat_j_kg: LS,
            latent_energy_into_snow_j_m2: vapor * LS,
            external_liquid_kg_m2: liquid,
            complete_energy_j_m2: energy,
        }
    }

    fn assert_closed(outcome: &TrajectoryOutcome) {
        assert!(outcome.ledger.solid_residual_kg_m2.abs() <= MASS_TOL_KG_M2);
        assert!(outcome.ledger.liquid_residual_kg_m2.abs() <= MASS_TOL_KG_M2);
        assert!(outcome.ledger.energy_residual_j_m2.abs() <= ENERGY_TOL_J_M2);
        assert!(outcome.ledger.vapor_energy_residual_j_m2.abs() <= ENERGY_TOL_J_M2);
    }

    #[test]
    fn result_blind_matrix_closes_for_all_four_models() {
        let vectors = [
            (
                "zero",
                state(0.6, 0.0, 0.0),
                segment(10, 0.0, 0.0, 10_000.0),
            ),
            (
                "sublimation",
                state(0.6, 0.0, 0.0),
                segment(10, -0.01, 0.0, 10_000.0),
            ),
            (
                "deposition",
                state(0.6, 0.0, 0.0),
                segment(10, 0.01, 0.0, 10_000.0),
            ),
            (
                "refreeze",
                state(0.6, 0.02, 10_000.0),
                segment(10, 0.0, 0.0, 0.0),
            ),
            (
                "rain",
                state(0.6, 0.0, 1_000.0),
                segment(10, 0.0, 0.03, 20_000.0),
            ),
        ];
        for (name, beginning, forcing) in vectors {
            for outcome in [
                released_ordered_trajectory(beginning, &[forcing])
                    .unwrap_or_else(|error| panic!("{name} released: {error:?}")),
                event_driven_frost_hybrid(beginning, &[forcing])
                    .unwrap_or_else(|error| panic!("{name} hybrid: {error:?}")),
                time_resolved_complementarity(beginning, &[forcing])
                    .unwrap_or_else(|error| panic!("{name} complementarity: {error:?}")),
                existing_snow_frost_subtype(beginning, &[forcing])
                    .unwrap_or_else(|error| panic!("{name} tagged: {error:?}"))
                    .0,
            ] {
                assert_closed(&outcome);
            }
        }
    }

    #[test]
    fn released_ordering_retains_meltout_then_reappearance() {
        let forcing = segment(10, 0.002, 0.0, 0.602 * LF_J_KG);
        let outcome = released_ordered_trajectory(state(0.6, 0.0, 0.0), &[forcing])
            .expect("released chronology");
        assert_eq!(outcome.events.len(), 2);
        assert_eq!(outcome.events[0].kind, TrajectoryEventKind::Meltout);
        assert_eq!(outcome.events[1].kind, TrajectoryEventKind::Reappearance);
        assert_eq!(outcome.ending.pack_ice_kg_m2.to_bits(), 0.002_f64.to_bits());
        assert!(outcome.ledger.unallocated_energy_j_m2 > ENERGY_TOL_J_M2);
        assert_eq!(
            qualification(&outcome),
            Qualification::UnresolvedSolidEnergyCoexistence
        );
    }

    #[test]
    fn fixed_path_refinement_converges_but_reordering_changes_chronology() {
        let beginning = state(0.6, 0.0, 0.0);
        let coarse = [segment(10, 0.002, 0.0, 0.5 * LF_J_KG)];
        let refined = [
            segment(5, 0.001, 0.0, 0.25 * LF_J_KG),
            segment(5, 0.001, 0.0, 0.25 * LF_J_KG),
        ];
        let coarse_out = time_resolved_complementarity(beginning, &coarse).expect("coarse");
        let refined_out = time_resolved_complementarity(beginning, &refined).expect("refined");
        assert!(
            (coarse_out.ending.pack_ice_kg_m2 - refined_out.ending.pack_ice_kg_m2).abs()
                <= MASS_TOL_KG_M2
        );
        assert!(
            (coarse_out.ending.liquid_kg_m2 - refined_out.ending.liquid_kg_m2).abs()
                <= MASS_TOL_KG_M2
        );

        let energy_then_deposition = [
            segment(5, 0.0, 0.0, 0.602 * LF_J_KG),
            segment(5, 0.002, 0.0, 0.0),
        ];
        let deposition_then_energy = [
            segment(5, 0.002, 0.0, 0.0),
            segment(5, 0.0, 0.0, 0.602 * LF_J_KG),
        ];
        let first = time_resolved_complementarity(beginning, &energy_then_deposition)
            .expect("energy then deposition");
        let second = time_resolved_complementarity(beginning, &deposition_then_energy)
            .expect("deposition then energy");
        assert_ne!(first.events, second.events);
        assert_ne!(
            first.ending.pack_ice_kg_m2.to_bits(),
            second.ending.pack_ice_kg_m2.to_bits()
        );
    }

    #[test]
    fn released_ordering_fails_fixed_path_refinement_at_meltout() {
        let beginning = state(0.6, 0.0, 0.0);
        let coarse = [segment(10, 0.002, 0.0, 0.602 * LF_J_KG)];
        let refined = [
            segment(5, 0.001, 0.0, 0.301 * LF_J_KG),
            segment(5, 0.001, 0.0, 0.301 * LF_J_KG),
        ];
        let released_coarse = released_ordered_trajectory(beginning, &coarse).expect("coarse");
        let released_refined = released_ordered_trajectory(beginning, &refined).expect("refined");
        assert!(
            (released_coarse.ending.pack_ice_kg_m2 - released_refined.ending.pack_ice_kg_m2).abs()
                > MASS_TOL_KG_M2
        );

        let hybrid_coarse = event_driven_frost_hybrid(beginning, &coarse).expect("hybrid coarse");
        let hybrid_refined =
            event_driven_frost_hybrid(beginning, &refined).expect("hybrid refined");
        assert!(
            (hybrid_coarse.ending.pack_ice_kg_m2 - hybrid_refined.ending.pack_ice_kg_m2).abs()
                <= MASS_TOL_KG_M2
        );
        assert!(
            (hybrid_coarse.ending.liquid_kg_m2 - hybrid_refined.ending.liquid_kg_m2).abs()
                <= MASS_TOL_KG_M2
        );
    }

    #[test]
    fn hybrid_and_tagged_subtype_retain_frost_without_alias_or_deletion() {
        let forcing = [
            segment(5, 0.0, 0.0, 0.602 * LF_J_KG),
            segment(5, 0.002, 0.0, 0.0),
        ];
        let hybrid = event_driven_frost_hybrid(state(0.6, 0.0, 0.0), &forcing).expect("hybrid");
        assert_eq!(hybrid.ending.pack_ice_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            hybrid.ending.surface_frost_kg_m2.to_bits(),
            0.002_f64.to_bits()
        );
        assert_eq!(
            qualification(&hybrid),
            Qualification::UnresolvedSolidEnergyCoexistence
        );
        let (tagged, envelope) =
            existing_snow_frost_subtype(state(0.6, 0.0, 0.0), &forcing).expect("tagged subtype");
        assert_eq!(tagged.ending, hybrid.ending);
        assert_eq!(
            TaggedSnowOwnerEnvelope::restore(&envelope.canonical_bytes()),
            Ok(envelope)
        );
        assert_eq!(
            qualification(&tagged),
            Qualification::UnresolvedSolidEnergyCoexistence
        );
        let mut poison = envelope.canonical_bytes();
        poison[0] ^= 1;
        assert_eq!(
            TaggedSnowOwnerEnvelope::restore(&poison),
            Err(TrajectoryFailure::RestartSchema)
        );
    }

    #[test]
    fn exact_latent_custody_and_failure_rollback_are_fail_closed() {
        let beginning = state(0.6, 0.0, 0.0);
        let valid = segment(10, 0.001, 0.0, 10_000.0);
        released_ordered_trajectory(beginning, &[valid]).expect("valid latent custody");
        let poisoned = ForcingSegment {
            latent_energy_into_snow_j_m2: valid.latent_energy_into_snow_j_m2 + 1.0,
            ..valid
        };
        assert_eq!(
            released_ordered_trajectory(beginning, &[poisoned]),
            Err(TrajectoryFailure::VaporLatentMismatch)
        );
        assert_eq!(beginning, state(0.6, 0.0, 0.0));
        let zero_latent_coefficient = ForcingSegment {
            latent_heat_j_kg: 0.0,
            latent_energy_into_snow_j_m2: 0.0,
            ..valid
        };
        assert_eq!(
            released_ordered_trajectory(beginning, &[zero_latent_coefficient]),
            Err(TrajectoryFailure::DomainOrNonFinite)
        );
        let overdraw = segment(10, -0.7, 0.0, 0.0);
        assert_eq!(
            event_driven_frost_hybrid(beginning, &[overdraw]),
            Err(TrajectoryFailure::SublimationOverdraw)
        );
        assert_eq!(beginning, state(0.6, 0.0, 0.0));
    }

    #[test]
    fn chronology_domain_and_tick_failures_are_typed() {
        let exhaustion =
            released_ordered_trajectory(state(0.6, 0.0, 0.0), &[segment(10, -0.6, 0.0, 0.0)])
                .expect("exact sublimation exhaustion");
        assert_eq!(
            exhaustion.events,
            vec![TrajectoryEvent {
                kind: TrajectoryEventKind::SublimationExhaustion,
                tick_ns: 10,
            }]
        );

        let signed_zero = TrajectoryState {
            pack_ice_kg_m2: -0.0,
            ..state(0.0, 0.0, 0.0)
        };
        assert_eq!(
            event_driven_frost_hybrid(signed_zero, &[segment(1, 0.0, 0.0, 0.0)]),
            Err(TrajectoryFailure::DomainOrNonFinite)
        );
        assert_eq!(
            event_driven_frost_hybrid(
                TrajectoryState {
                    pack_ice_kg_m2: 0.3,
                    surface_frost_kg_m2: 0.3,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                },
                &[segment(1, 0.0, 0.0, 0.0)],
            ),
            Err(TrajectoryFailure::UnsupportedFrostInput)
        );
        let overflowing = [segment(u128::MAX, 0.0, 0.0, 0.0), segment(1, 0.0, 0.0, 0.0)];
        assert_eq!(
            released_ordered_trajectory(state(0.6, 0.0, 0.0), &overflowing),
            Err(TrajectoryFailure::TickOverflow)
        );
    }

    #[test]
    fn sign_changing_energy_path_is_deterministic_and_replays_exactly() {
        let path = [
            segment(5, 0.001, 0.01, -5_000.0),
            segment(5, -0.0005, 0.0, 50_000.0),
        ];
        let first =
            released_ordered_trajectory(state(0.6, 0.02, 10_000.0), &path).expect("first replay");
        let second =
            released_ordered_trajectory(state(0.6, 0.02, 10_000.0), &path).expect("second replay");
        assert_eq!(first, second);
        assert_closed(&first);
    }
}
