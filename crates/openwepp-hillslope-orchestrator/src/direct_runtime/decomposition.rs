use super::{
    DIRECT_AUDIT, DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT,
    DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT, DirectDayFrame, DirectRuntimeError,
    validate_finite, validate_nonnegative_direct_m,
};

const DECOMPOSITION_TEMP_OFFSET_C: f64 = 6.1;
const DECOMPOSITION_TEMP_ACTIVE_UPPER_C: f64 = 49.2;
const DECOMPOSITION_TEMP_T2: f64 = 1528.81;
const STANDING_RAIN_SATURATION_M: f64 = 0.004;

impl DirectDayFrame {
    pub fn run_r5c_decomposition_phase(
        &mut self,
    ) -> Result<DirectDecompositionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let decomposition = self.compute_r5c_decomposition()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.decomposition_inputs = DirectDecompositionInputs::from_frame(self)?;
        self.decomposition = decomposition;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.decomposition_downstream_operands =
            DirectDecompositionDownstreamOperands::from(decomposition);
        DIRECT_AUDIT.record_downstream_operand_production();

        let decomposition_shadow_projection = DirectDecompositionShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            active_context: decomposition.active_context,
            active_action: decomposition.active_action,
            surface_litter_input_kg_m2: decomposition.surface_litter_input_kg_m2,
            surface_residue_kg_m2: self.decomposition_downstream_operands.surface_residue_kg_m2,
            root_residue_kg_m2: self.decomposition_downstream_operands.root_residue_kg_m2,
            residue_depth_m: self.decomposition_downstream_operands.residue_depth_m,
            environment_index: self.decomposition_downstream_operands.environment_index,
            surface_decay_factor: self.decomposition_downstream_operands.surface_decay_factor,
            root_decay_factor: self.decomposition_downstream_operands.root_decay_factor,
        };
        self.decomposition_shadow_projection = Some(decomposition_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectDecompositionSpanReport {
            phase_count: DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            decomposition_shadow_projection,
        })
    }

    pub fn run_r5c_residue_partition_phase(
        &mut self,
    ) -> Result<DirectResiduePartitionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let residue_partition = self.compute_r5c_residue_partition()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.residue_partition_inputs = DirectResiduePartitionInputs::from_frame(self)?;
        self.residue_partition = residue_partition;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.residue_partition_downstream_operands =
            DirectResiduePartitionDownstreamOperands::from(residue_partition);
        DIRECT_AUDIT.record_downstream_operand_production();

        let residue_partition_shadow_projection = DirectResiduePartitionShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            standing_residue_kg_m2: self
                .residue_partition_downstream_operands
                .standing_residue_kg_m2,
            flat_residue_kg_m2: self
                .residue_partition_downstream_operands
                .flat_residue_kg_m2,
            buried_residue_kg_m2: self
                .residue_partition_downstream_operands
                .buried_residue_kg_m2,
            root_residue_kg_m2: self
                .residue_partition_downstream_operands
                .root_residue_kg_m2,
            total_residue_kg_m2: self
                .residue_partition_downstream_operands
                .total_residue_kg_m2,
            cover_fraction: self.residue_partition_downstream_operands.cover_fraction,
        };
        self.residue_partition_shadow_projection = Some(residue_partition_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectResiduePartitionSpanReport {
            phase_count: DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            residue_partition_shadow_projection,
        })
    }

    fn compute_r5c_decomposition(&self) -> Result<DirectDecompositionState, DirectRuntimeError> {
        if self.storage_bounds_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5B storage bounds phase",
            });
        }
        let inputs = DirectDecompositionInputs::from_frame(self)?;
        inputs.compute_state()
    }

    fn compute_r5c_residue_partition(
        &self,
    ) -> Result<DirectResiduePartitionState, DirectRuntimeError> {
        if self.decomposition_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5C decomposition transition",
            });
        }

        let inputs = DirectResiduePartitionInputs::from_frame(self)?;
        let flat_residue_kg_m2 = inputs.flat_residue_offset_kg_m2
            + self.decomposition_downstream_operands.surface_residue_kg_m2;
        validate_nonnegative_direct_m("residue_partition.flat_residue_kg_m2", flat_residue_kg_m2)?;

        let root_residue_kg_m2 = self.decomposition_downstream_operands.root_residue_kg_m2;
        validate_nonnegative_direct_m("residue_partition.root_residue_kg_m2", root_residue_kg_m2)?;

        let total_residue_kg_m2 = inputs.standing_residue_kg_m2
            + flat_residue_kg_m2
            + inputs.buried_residue_kg_m2
            + root_residue_kg_m2;
        validate_nonnegative_direct_m(
            "residue_partition.total_residue_kg_m2",
            total_residue_kg_m2,
        )?;

        let downstream = &self.decomposition_downstream_operands;
        let interrill_cover_fraction = residue_ground_cover_fraction(
            downstream.residue_cover_factor,
            downstream.interrill_ground_residue_kg_m2,
        )?;
        let rill_cover_fraction = residue_ground_cover_fraction(
            downstream.residue_cover_factor,
            downstream.rill_ground_residue_kg_m2,
        )?;

        // `covcal.for:176`: the published composite is the legacy
        // `rescov` area-weighted blend of the two covers. The weight is
        // fail-closed to [0, 1] at the input boundary (`validate`), so no
        // silent canonicalization happens here.
        let weight = inputs.rescov_interrill_weight;
        let composite_cover_fraction =
            weight * interrill_cover_fraction + (1.0 - weight) * rill_cover_fraction;

        Ok(DirectResiduePartitionState {
            standing_residue_kg_m2: inputs.standing_residue_kg_m2,
            interrill_cover_fraction,
            rill_cover_fraction,
            flat_residue_kg_m2,
            buried_residue_kg_m2: inputs.buried_residue_kg_m2,
            root_residue_kg_m2,
            total_residue_kg_m2,
            cover_fraction: composite_cover_fraction,
        })
    }
}

fn decomposition_temperature_factor(
    inputs: DirectDecompositionInputs,
) -> Result<f64, DirectRuntimeError> {
    let tave = f64::midpoint(inputs.temperature_max_c, inputs.temperature_min_c);
    validate_finite("decomposition.average_temperature_c", tave)?;
    let tmpfac =
        if tave <= -DECOMPOSITION_TEMP_OFFSET_C || tave >= DECOMPOSITION_TEMP_ACTIVE_UPPER_C {
            0.0
        } else {
            let t1 = (tave + DECOMPOSITION_TEMP_OFFSET_C).powi(2);
            let numerator = t1 * (2.0 * DECOMPOSITION_TEMP_T2 - t1);
            let denominator = DECOMPOSITION_TEMP_T2.powi(2);
            validate_finite("decomposition.temperature_factor_numerator", numerator)?;
            numerator / denominator
        };
    validate_unit_fraction("decomposition.temperature_factor", tmpfac)?;
    Ok(tmpfac)
}

fn standing_surface_water_factor(
    inputs: DirectDecompositionInputs,
) -> Result<f64, DirectRuntimeError> {
    let tave = f64::midpoint(inputs.temperature_max_c, inputs.temperature_min_c);
    validate_finite("decomposition.average_temperature_c", tave)?;
    let swatfc = if tave <= 0.0 {
        0.0
    } else if inputs.precipitation_m < STANDING_RAIN_SATURATION_M {
        inputs.precipitation_m / STANDING_RAIN_SATURATION_M
    } else {
        1.0
    };
    validate_unit_fraction("decomposition.surface_water_factor", swatfc)?;
    Ok(swatfc)
}

fn decay_factor(
    field: &'static str,
    environment_index: f64,
    decomposition_rate: f64,
) -> Result<f64, DirectRuntimeError> {
    let exponent = -environment_index * decomposition_rate;
    validate_finite(field, exponent)?;
    let factor = exponent.exp();
    validate_unit_fraction(field, factor)?;
    Ok(factor)
}

fn apply_decomposition_action(
    inputs: DirectDecompositionInputs,
    mut surface_residue_kg_m2: f64,
    mut root_residue_kg_m2: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    match inputs.active_context {
        DirectDecompositionActiveContext::AnnualOrFallow { .. } => match inputs.active_action {
            DirectDecompositionAction::Burn => {
                surface_residue_kg_m2 *= 1.0 - inputs.burn_surface_fraction;
            }
            DirectDecompositionAction::Remove => {
                surface_residue_kg_m2 *= 1.0 - inputs.remove_surface_fraction;
            }
            DirectDecompositionAction::Cut => {
                let transfer = surface_residue_kg_m2 * inputs.cut_transfer_fraction;
                validate_nonnegative_direct_m("decomposition.cut_transfer_kg_m2", transfer)?;
                surface_residue_kg_m2 -= transfer;
                root_residue_kg_m2 += transfer;
            }
            DirectDecompositionAction::None
            | DirectDecompositionAction::Herbicide
            | DirectDecompositionAction::Silage => {}
            DirectDecompositionAction::Grazing => {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "decomposition.active_action",
                });
            }
        },
        DirectDecompositionActiveContext::Perennial { .. } => match inputs.active_action {
            DirectDecompositionAction::Grazing => {
                surface_residue_kg_m2 *= 1.0 - inputs.grazing_digest_fraction;
            }
            DirectDecompositionAction::None | DirectDecompositionAction::Cut => {}
            DirectDecompositionAction::Herbicide
            | DirectDecompositionAction::Burn
            | DirectDecompositionAction::Silage
            | DirectDecompositionAction::Remove => {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "decomposition.active_action",
                });
            }
        },
        DirectDecompositionActiveContext::Inactive
        | DirectDecompositionActiveContext::Missing
        | DirectDecompositionActiveContext::Ambiguous => {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "decomposition.active_context",
            });
        }
    }

    validate_finite("decomposition.surface_residue_kg_m2", surface_residue_kg_m2)?;
    validate_finite("decomposition.root_residue_kg_m2", root_residue_kg_m2)?;
    Ok((surface_residue_kg_m2, root_residue_kg_m2))
}

/// Ground-pool action rules (`decomp.for`): burn/remove/graze fractions
/// apply to `rigrm`/`rilrm`; **Cut ADDS the cut material to both ground
/// pools** (`decomp.for:689-693` — `rilrm/rigrm/rmogt += tmpvr4`). Our
/// pool topology has no standing mat, so the cut-mass basis is the
/// surface-pool transfer (`surface·cut_transfer_fraction`) — a labeled
/// mapping; the ground-pool RULE (addition to both) is source-true.
fn apply_ground_pool_action(
    inputs: DirectDecompositionInputs,
    mut pool_kg_m2: f64,
    cut_mass_kg_m2: f64,
) -> Result<f64, DirectRuntimeError> {
    match inputs.active_action {
        DirectDecompositionAction::Burn => {
            pool_kg_m2 *= 1.0 - inputs.burn_surface_fraction;
        }
        DirectDecompositionAction::Remove => {
            pool_kg_m2 *= 1.0 - inputs.remove_surface_fraction;
        }
        DirectDecompositionAction::Grazing => {
            pool_kg_m2 *= 1.0 - inputs.grazing_digest_fraction;
        }
        DirectDecompositionAction::Cut => {
            pool_kg_m2 += cut_mass_kg_m2;
        }
        DirectDecompositionAction::None
        | DirectDecompositionAction::Herbicide
        | DirectDecompositionAction::Silage => {}
    }
    validate_nonnegative_direct_m("decomposition.ground_pool_kg_m2", pool_kg_m2)?;
    Ok(pool_kg_m2)
}

/// `covcal.for:160-176`: cover from a ground pool —
/// `1 − exp(−cf·mass)`, clamped to `[0, 0.999]`. The standing-mat
/// `strcov` term is 0 (the standing pool is not yet modeled; the term is
/// additive-only, so its absence is conservative in the fail-direction
/// of the `GAP-SED-009` defect).
pub fn residue_ground_cover_fraction(
    cover_factor: f64,
    ground_residue_kg_m2: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_finite("residue_cover.cover_factor", cover_factor)?;
    validate_nonnegative_direct_m("residue_cover.ground_residue_kg_m2", ground_residue_kg_m2)?;
    if cover_factor < 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "residue_cover.cover_factor",
        });
    }
    let cover = 1.0 - (-cover_factor * ground_residue_kg_m2).exp();
    Ok(cover.clamp(0.0, 0.999))
}

fn validate_unit_fraction(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else if value < 0.0 {
        Err(DirectRuntimeError::NegativeDirectValue { field })
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "restart-authority-evidence", derive(serde::Serialize))]
#[cfg_attr(
    feature = "restart-authority-evidence",
    serde(rename_all = "snake_case")
)]
pub enum DirectDecompositionActiveContext {
    Inactive,
    AnnualOrFallow {
        active_slot_index: u16,
        active_crop_slot_index: u16,
        runtime_day_of_year: u16,
    },
    Perennial {
        active_slot_index: u16,
        active_crop_slot_index: u16,
        runtime_day_of_year: u16,
    },
    Missing,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "restart-authority-evidence", derive(serde::Serialize))]
#[cfg_attr(
    feature = "restart-authority-evidence",
    serde(rename_all = "snake_case")
)]
pub enum DirectDecompositionAction {
    None,
    Herbicide,
    Burn,
    Silage,
    Cut,
    Remove,
    Grazing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "restart-authority-evidence", derive(serde::Serialize))]
pub struct DirectDecompositionInputs {
    pub active_context: DirectDecompositionActiveContext,
    pub active_action: DirectDecompositionAction,
    pub residue_type_selector: f64,
    pub surface_residue_seed_kg_m2: f64,
    /// Ground-cover authority (E.5 `GAP-SED-009` closure): the interrill
    /// and rill ground-residue pools (`init1.for:295-297` lineage — day-0
    /// back-derived from the declared IC covers; day-N carried from the
    /// prior day's state) and the residue-type cover factor
    /// (`cf(iresd)`), the `covcal.for` operands.
    pub interrill_ground_seed_kg_m2: f64,
    pub rill_ground_seed_kg_m2: f64,
    pub residue_cover_factor: f64,
    pub root_residue_seed_kg_m2: f64,
    pub surface_litter_input_kg_m2: f64,
    pub residue_depth_conversion_m_per_kg_m2: f64,
    pub temperature_max_c: f64,
    pub temperature_min_c: f64,
    pub precipitation_m: f64,
    pub water_stress_fraction: f64,
    pub surface_decomposition_rate: f64,
    pub root_decomposition_rate: f64,
    pub burn_surface_fraction: f64,
    pub remove_surface_fraction: f64,
    pub cut_transfer_fraction: f64,
    pub grazing_digest_fraction: f64,
}

impl DirectDecompositionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_context: DirectDecompositionActiveContext::Inactive,
            active_action: DirectDecompositionAction::None,
            residue_type_selector: 0.0,
            surface_residue_seed_kg_m2: 0.0,
            interrill_ground_seed_kg_m2: 0.0,
            rill_ground_seed_kg_m2: 0.0,
            residue_cover_factor: 0.0,
            root_residue_seed_kg_m2: 0.0,
            surface_litter_input_kg_m2: 0.0,
            residue_depth_conversion_m_per_kg_m2: 0.0,
            temperature_max_c: 0.0,
            temperature_min_c: 0.0,
            precipitation_m: 0.0,
            water_stress_fraction: 0.0,
            surface_decomposition_rate: 0.0,
            root_decomposition_rate: 0.0,
            burn_surface_fraction: 0.0,
            remove_surface_fraction: 0.0,
            cut_transfer_fraction: 0.0,
            grazing_digest_fraction: 0.0,
        }
    }

    fn from_frame(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        frame.decomposition_inputs.validate()?;
        Ok(frame.decomposition_inputs)
    }

    fn validate(self) -> Result<(), DirectRuntimeError> {
        self.validate_active_context()?;
        self.validate_action_domain()?;
        self.validate_pool_and_rate_domain()
    }

    pub fn compute_state(self) -> Result<DirectDecompositionState, DirectRuntimeError> {
        self.validate()?;

        if self.active_context == DirectDecompositionActiveContext::Inactive {
            return Ok(DirectDecompositionState::inactive(self));
        }

        let temperature_factor = decomposition_temperature_factor(self)?;
        let surface_water_factor = standing_surface_water_factor(self)?;
        let flat_water_factor = self.water_stress_fraction;
        let environmental_index = temperature_factor.min(flat_water_factor);
        validate_unit_fraction("decomposition.environment_index", environmental_index)?;

        let surface_decay_factor = decay_factor(
            "decomposition.surface_decay_factor",
            environmental_index,
            self.surface_decomposition_rate,
        )?;
        let root_decay_factor = decay_factor(
            "decomposition.root_decay_factor",
            environmental_index,
            self.root_decomposition_rate,
        )?;

        let surface_before_decay =
            self.surface_residue_seed_kg_m2 + self.surface_litter_input_kg_m2;
        validate_nonnegative_direct_m(
            "decomposition.surface_before_decay_kg_m2",
            surface_before_decay,
        )?;
        let surface_after_decay = surface_before_decay * surface_decay_factor;
        let root_after_decay = self.root_residue_seed_kg_m2 * root_decay_factor;
        let (surface_residue_kg_m2, root_residue_kg_m2) =
            apply_decomposition_action(self, surface_after_decay, root_after_decay)?;
        // Ground pools (`decomp.for` applies the identical decay law to
        // `rigrm`/`rilrm`; surface-litter fall lands on interrill and
        // rill areas alike): litter + decay + the ground-affecting
        // actions (Burn/Remove/Grazing fractions; Cut ADDS the cut
        // mass to both pools, `decomp.for:689-693`).
        let ground_cut_mass_kg_m2 = if self.active_action == DirectDecompositionAction::Cut {
            surface_after_decay * self.cut_transfer_fraction
        } else {
            0.0
        };
        let interrill_ground_residue_kg_m2 = apply_ground_pool_action(
            self,
            (self.interrill_ground_seed_kg_m2 + self.surface_litter_input_kg_m2)
                * surface_decay_factor,
            ground_cut_mass_kg_m2,
        )?;
        let rill_ground_residue_kg_m2 = apply_ground_pool_action(
            self,
            (self.rill_ground_seed_kg_m2 + self.surface_litter_input_kg_m2) * surface_decay_factor,
            ground_cut_mass_kg_m2,
        )?;
        let residue_depth_m = surface_residue_kg_m2 * self.residue_depth_conversion_m_per_kg_m2;

        validate_nonnegative_direct_m(
            "decomposition.surface_residue_kg_m2",
            surface_residue_kg_m2,
        )?;
        validate_nonnegative_direct_m("decomposition.root_residue_kg_m2", root_residue_kg_m2)?;
        validate_nonnegative_direct_m("decomposition.residue_depth_m", residue_depth_m)?;

        Ok(DirectDecompositionState {
            active_context: self.active_context,
            active_action: self.active_action,
            residue_type_selector: self.residue_type_selector,
            surface_residue_seed_kg_m2: self.surface_residue_seed_kg_m2,
            root_residue_seed_kg_m2: self.root_residue_seed_kg_m2,
            surface_litter_input_kg_m2: self.surface_litter_input_kg_m2,
            residue_depth_conversion_m_per_kg_m2: self.residue_depth_conversion_m_per_kg_m2,
            temperature_factor,
            surface_water_factor,
            flat_water_factor,
            environment_index: environmental_index,
            surface_decay_factor,
            root_decay_factor,
            surface_residue_kg_m2,
            root_residue_kg_m2,
            interrill_ground_residue_kg_m2,
            rill_ground_residue_kg_m2,
            residue_cover_factor: self.residue_cover_factor,
            residue_depth_m,
        })
    }

    fn validate_active_context(self) -> Result<(), DirectRuntimeError> {
        match self.active_context {
            DirectDecompositionActiveContext::Inactive => {
                if self.active_action == DirectDecompositionAction::None {
                    Ok(())
                } else {
                    Err(DirectRuntimeError::DirectDomainViolation {
                        field: "decomposition.active_action",
                    })
                }
            }
            DirectDecompositionActiveContext::AnnualOrFallow {
                runtime_day_of_year,
                ..
            }
            | DirectDecompositionActiveContext::Perennial {
                runtime_day_of_year,
                ..
            } => {
                if (1..=366).contains(&runtime_day_of_year) {
                    Ok(())
                } else {
                    Err(DirectRuntimeError::DirectDomainViolation {
                        field: "decomposition.runtime_day_of_year",
                    })
                }
            }
            DirectDecompositionActiveContext::Missing
            | DirectDecompositionActiveContext::Ambiguous => {
                Err(DirectRuntimeError::DirectDomainViolation {
                    field: "decomposition.active_context",
                })
            }
        }
    }

    fn validate_action_domain(self) -> Result<(), DirectRuntimeError> {
        validate_unit_fraction(
            "decomposition.burn_surface_fraction",
            self.burn_surface_fraction,
        )?;
        validate_unit_fraction(
            "decomposition.remove_surface_fraction",
            self.remove_surface_fraction,
        )?;
        validate_unit_fraction(
            "decomposition.cut_transfer_fraction",
            self.cut_transfer_fraction,
        )?;
        validate_unit_fraction(
            "decomposition.grazing_digest_fraction",
            self.grazing_digest_fraction,
        )
    }

    fn validate_pool_and_rate_domain(self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m("decomposition.iresd_seed", self.residue_type_selector)?;
        validate_nonnegative_direct_m(
            "decomposition.surface_residue_seed_kg_m2",
            self.surface_residue_seed_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.root_residue_seed_kg_m2",
            self.root_residue_seed_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.surface_litter_input_kg_m2",
            self.surface_litter_input_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.interrill_ground_seed_kg_m2",
            self.interrill_ground_seed_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.rill_ground_seed_kg_m2",
            self.rill_ground_seed_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.residue_cover_factor",
            self.residue_cover_factor,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.residue_depth_conversion_m_per_kg_m2",
            self.residue_depth_conversion_m_per_kg_m2,
        )?;
        validate_finite("decomposition.temperature_max_c", self.temperature_max_c)?;
        validate_finite("decomposition.temperature_min_c", self.temperature_min_c)?;
        validate_nonnegative_direct_m("decomposition.precipitation_m", self.precipitation_m)?;
        validate_unit_fraction(
            "decomposition.water_stress_fraction",
            self.water_stress_fraction,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.surface_decomposition_rate",
            self.surface_decomposition_rate,
        )?;
        validate_nonnegative_direct_m(
            "decomposition.root_decomposition_rate",
            self.root_decomposition_rate,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDecompositionState {
    pub active_context: DirectDecompositionActiveContext,
    pub active_action: DirectDecompositionAction,
    pub residue_type_selector: f64,
    pub surface_residue_seed_kg_m2: f64,
    pub root_residue_seed_kg_m2: f64,
    pub surface_litter_input_kg_m2: f64,
    pub residue_depth_conversion_m_per_kg_m2: f64,
    pub temperature_factor: f64,
    pub surface_water_factor: f64,
    pub flat_water_factor: f64,
    pub environment_index: f64,
    pub surface_decay_factor: f64,
    pub root_decay_factor: f64,
    pub surface_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    /// Evolved ground pools + the cover factor (`covcal` operands).
    pub interrill_ground_residue_kg_m2: f64,
    pub rill_ground_residue_kg_m2: f64,
    pub residue_cover_factor: f64,
    pub residue_depth_m: f64,
}

impl DirectDecompositionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_context: DirectDecompositionActiveContext::Inactive,
            active_action: DirectDecompositionAction::None,
            residue_type_selector: 0.0,
            surface_residue_seed_kg_m2: 0.0,
            root_residue_seed_kg_m2: 0.0,
            surface_litter_input_kg_m2: 0.0,
            residue_depth_conversion_m_per_kg_m2: 0.0,
            temperature_factor: 0.0,
            surface_water_factor: 0.0,
            flat_water_factor: 0.0,
            environment_index: 0.0,
            surface_decay_factor: 1.0,
            root_decay_factor: 1.0,
            surface_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            interrill_ground_residue_kg_m2: 0.0,
            rill_ground_residue_kg_m2: 0.0,
            residue_cover_factor: 0.0,
            residue_depth_m: 0.0,
        }
    }

    #[must_use]
    fn inactive(inputs: DirectDecompositionInputs) -> Self {
        Self {
            active_context: inputs.active_context,
            active_action: inputs.active_action,
            residue_type_selector: inputs.residue_type_selector,
            surface_residue_seed_kg_m2: inputs.surface_residue_seed_kg_m2,
            root_residue_seed_kg_m2: inputs.root_residue_seed_kg_m2,
            surface_litter_input_kg_m2: inputs.surface_litter_input_kg_m2,
            residue_depth_conversion_m_per_kg_m2: inputs.residue_depth_conversion_m_per_kg_m2,
            temperature_factor: 0.0,
            surface_water_factor: 0.0,
            flat_water_factor: inputs.water_stress_fraction,
            environment_index: 0.0,
            surface_decay_factor: 1.0,
            root_decay_factor: 1.0,
            surface_residue_kg_m2: inputs.surface_residue_seed_kg_m2,
            root_residue_kg_m2: inputs.root_residue_seed_kg_m2,
            interrill_ground_residue_kg_m2: inputs.interrill_ground_seed_kg_m2,
            rill_ground_residue_kg_m2: inputs.rill_ground_seed_kg_m2,
            residue_cover_factor: inputs.residue_cover_factor,
            residue_depth_m: inputs.surface_residue_seed_kg_m2
                * inputs.residue_depth_conversion_m_per_kg_m2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDecompositionDownstreamOperands {
    pub active_context: DirectDecompositionActiveContext,
    pub active_action: DirectDecompositionAction,
    pub residue_type_selector: f64,
    pub surface_litter_input_kg_m2: f64,
    pub surface_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    pub interrill_ground_residue_kg_m2: f64,
    pub rill_ground_residue_kg_m2: f64,
    pub residue_cover_factor: f64,
    pub residue_depth_m: f64,
    pub temperature_factor: f64,
    pub surface_water_factor: f64,
    pub flat_water_factor: f64,
    pub environment_index: f64,
    pub surface_decay_factor: f64,
    pub root_decay_factor: f64,
}

impl DirectDecompositionDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            interrill_ground_residue_kg_m2: 0.0,
            rill_ground_residue_kg_m2: 0.0,
            residue_cover_factor: 0.0,
            active_context: DirectDecompositionActiveContext::Inactive,
            active_action: DirectDecompositionAction::None,
            residue_type_selector: 0.0,
            surface_litter_input_kg_m2: 0.0,
            surface_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            residue_depth_m: 0.0,
            temperature_factor: 0.0,
            surface_water_factor: 0.0,
            flat_water_factor: 0.0,
            environment_index: 0.0,
            surface_decay_factor: 1.0,
            root_decay_factor: 1.0,
        }
    }
}

impl From<DirectDecompositionState> for DirectDecompositionDownstreamOperands {
    fn from(state: DirectDecompositionState) -> Self {
        Self {
            active_context: state.active_context,
            active_action: state.active_action,
            residue_type_selector: state.residue_type_selector,
            surface_litter_input_kg_m2: state.surface_litter_input_kg_m2,
            surface_residue_kg_m2: state.surface_residue_kg_m2,
            root_residue_kg_m2: state.root_residue_kg_m2,
            interrill_ground_residue_kg_m2: state.interrill_ground_residue_kg_m2,
            rill_ground_residue_kg_m2: state.rill_ground_residue_kg_m2,
            residue_cover_factor: state.residue_cover_factor,
            residue_depth_m: state.residue_depth_m,
            temperature_factor: state.temperature_factor,
            surface_water_factor: state.surface_water_factor,
            flat_water_factor: state.flat_water_factor,
            environment_index: state.environment_index,
            surface_decay_factor: state.surface_decay_factor,
            root_decay_factor: state.root_decay_factor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDecompositionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub active_context: DirectDecompositionActiveContext,
    pub active_action: DirectDecompositionAction,
    pub surface_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    pub surface_litter_input_kg_m2: f64,
    pub residue_depth_m: f64,
    pub environment_index: f64,
    pub surface_decay_factor: f64,
    pub root_decay_factor: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDecompositionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub decomposition_shadow_projection: DirectDecompositionShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "restart-authority-evidence", derive(serde::Serialize))]
pub struct DirectResiduePartitionInputs {
    pub standing_residue_kg_m2: f64,
    pub flat_residue_offset_kg_m2: f64,
    pub buried_residue_kg_m2: f64,
    pub cover_fraction: f64,
    /// `covcal.for:176` `rescov` interrill area weight
    /// `(rspace − width)/rspace`; the composite cover is
    /// `w·inrcov + (1−w)·rilcov`.
    pub rescov_interrill_weight: f64,
}

impl DirectResiduePartitionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            standing_residue_kg_m2: 0.0,
            flat_residue_offset_kg_m2: 0.0,
            buried_residue_kg_m2: 0.0,
            cover_fraction: 0.0,
            rescov_interrill_weight: 0.0,
        }
    }

    fn from_frame(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        frame.residue_partition_inputs.validate()?;
        Ok(frame.residue_partition_inputs)
    }

    fn validate(self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "residue_partition.standing_residue_kg_m2",
            self.standing_residue_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "residue_partition.flat_residue_offset_kg_m2",
            self.flat_residue_offset_kg_m2,
        )?;
        validate_nonnegative_direct_m(
            "residue_partition.buried_residue_kg_m2",
            self.buried_residue_kg_m2,
        )?;
        validate_unit_fraction("residue_partition.cover_fraction", self.cover_fraction)?;
        validate_unit_fraction(
            "residue_partition.rescov_interrill_weight",
            self.rescov_interrill_weight,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectResiduePartitionState {
    pub standing_residue_kg_m2: f64,
    /// `covcal.for` covers from the evolved ground pools (E.5
    /// `GAP-SED-009` closure): the erosion interrill/rill operands.
    pub interrill_cover_fraction: f64,
    pub rill_cover_fraction: f64,
    pub flat_residue_kg_m2: f64,
    pub buried_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    pub total_residue_kg_m2: f64,
    pub cover_fraction: f64,
}

impl DirectResiduePartitionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            interrill_cover_fraction: 0.0,
            rill_cover_fraction: 0.0,
            standing_residue_kg_m2: 0.0,
            flat_residue_kg_m2: 0.0,
            buried_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            total_residue_kg_m2: 0.0,
            cover_fraction: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectResiduePartitionDownstreamOperands {
    pub standing_residue_kg_m2: f64,
    pub flat_residue_kg_m2: f64,
    pub buried_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    pub total_residue_kg_m2: f64,
    pub cover_fraction: f64,
}

impl DirectResiduePartitionDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            standing_residue_kg_m2: 0.0,
            flat_residue_kg_m2: 0.0,
            buried_residue_kg_m2: 0.0,
            root_residue_kg_m2: 0.0,
            total_residue_kg_m2: 0.0,
            cover_fraction: 0.0,
        }
    }
}

impl From<DirectResiduePartitionState> for DirectResiduePartitionDownstreamOperands {
    fn from(state: DirectResiduePartitionState) -> Self {
        Self {
            standing_residue_kg_m2: state.standing_residue_kg_m2,
            flat_residue_kg_m2: state.flat_residue_kg_m2,
            buried_residue_kg_m2: state.buried_residue_kg_m2,
            root_residue_kg_m2: state.root_residue_kg_m2,
            total_residue_kg_m2: state.total_residue_kg_m2,
            cover_fraction: state.cover_fraction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectResiduePartitionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub standing_residue_kg_m2: f64,
    pub flat_residue_kg_m2: f64,
    pub buried_residue_kg_m2: f64,
    pub root_residue_kg_m2: f64,
    pub total_residue_kg_m2: f64,
    pub cover_fraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectResiduePartitionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub residue_partition_shadow_projection: DirectResiduePartitionShadowProjection,
}
