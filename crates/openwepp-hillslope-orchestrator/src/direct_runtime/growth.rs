use crate::constants::{
    PL_GROWTH_ANNUAL_LAI_A, PL_GROWTH_ANNUAL_LAI_B, PL_GROWTH_CANCOV_MAX, PL_GROWTH_DDM_SCALE,
    PL_GROWTH_GDMAX_MONTH_DAY_STARTS, PL_GROWTH_GDMAX_MONTH_LENGTHS, PL_GROWTH_GDMAX_YEAR_END_DAY,
    PL_GROWTH_PAR_LAI_OFFSET, PL_GROWTH_PAR_RAD_SCALE, PL_GROWTH_PERENNIAL_LAI_A,
    PL_GROWTH_PERENNIAL_LAI_B, PL_GROWTH_ROOT_DEPTH_CURVE_A, PL_GROWTH_ROOT_DEPTH_CURVE_B,
    WB11_ZERO_THRESHOLD,
};

use super::{
    DIRECT_AUDIT, DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT,
    DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT, DirectDayFrame, DirectRuntimeError,
    validate_finite, validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r5d_annual_growth_phase(
        &mut self,
    ) -> Result<DirectGrowthSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let growth = self.compute_r5d_annual_growth()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.annual_growth_inputs = DirectGrowthInputs::from_frame_annual(self)?;
        self.annual_growth = growth;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.annual_growth_downstream_operands = DirectGrowthDownstreamOperands::from(growth);
        DIRECT_AUDIT.record_downstream_operand_production();

        self.apply_growth_downstream_to_r4n(self.annual_growth_downstream_operands)?;
        let growth_shadow_projection = DirectGrowthShadowProjection::from_operands(
            self.lane_index,
            self.day_index,
            self.annual_growth_downstream_operands,
        );
        self.annual_growth_shadow_projection = Some(growth_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectGrowthSpanReport {
            phase_count: DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            growth_shadow_projection,
        })
    }

    pub fn run_r5d_perennial_growth_phase(
        &mut self,
    ) -> Result<DirectGrowthSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let growth = self.compute_r5d_perennial_growth()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.perennial_growth_inputs = DirectGrowthInputs::from_frame_perennial(self)?;
        self.perennial_growth = growth;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.perennial_growth_downstream_operands = DirectGrowthDownstreamOperands::from(growth);
        DIRECT_AUDIT.record_downstream_operand_production();

        self.apply_growth_downstream_to_r4n(self.perennial_growth_downstream_operands)?;
        let growth_shadow_projection = DirectGrowthShadowProjection::from_operands(
            self.lane_index,
            self.day_index,
            self.perennial_growth_downstream_operands,
        );
        self.perennial_growth_shadow_projection = Some(growth_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectGrowthSpanReport {
            phase_count: DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            growth_shadow_projection,
        })
    }

    fn compute_r5d_annual_growth(&self) -> Result<DirectGrowthState, DirectRuntimeError> {
        if self.residue_partition_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5C residue partition transition",
            });
        }
        let inputs = DirectGrowthInputs::from_frame_annual(self)?;
        inputs.compute_for_management_class(DirectGrowthManagementClass::AnnualOrFallow)
    }

    fn compute_r5d_perennial_growth(&self) -> Result<DirectGrowthState, DirectRuntimeError> {
        if self.residue_partition_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5C residue partition transition",
            });
        }
        if self.annual_growth_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5D annual growth transition",
            });
        }
        let inputs = DirectGrowthInputs::from_frame_perennial(self)?;
        inputs.compute_for_management_class(DirectGrowthManagementClass::Perennial)
    }

    fn apply_growth_downstream_to_r4n(
        &mut self,
        operands: DirectGrowthDownstreamOperands,
    ) -> Result<(), DirectRuntimeError> {
        self.evapotranspiration_compute_inputs
            .growth_context_required = true;
        if operands.active_context.is_active() && operands.active_action.consumes_plant_state() {
            self.evapotranspiration_compute_inputs.et_demand_m = operands.et_demand_m;
            self.evapotranspiration_compute_inputs.leaf_area_index =
                operands.state_after.leaf_area_index;
            self.evapotranspiration_compute_inputs.canopy_cover_fraction =
                operands.state_after.canopy_cover_fraction;
            self.evapotranspiration_compute_inputs.root_depth_m = operands.state_after.root_depth_m;
            self.evapotranspiration_compute_inputs
                .residue_interception_m = operands.residue_interception_m;
            self.evapotranspiration_compute_inputs.plant_tolerance = operands.plant_tolerance;
            validate_finite(
                "growth.downstream_evapotranspiration_et_demand_m",
                self.evapotranspiration_compute_inputs.et_demand_m,
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectGrowthManagementClass {
    AnnualOrFallow,
    Perennial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectGrowthActiveContext {
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

impl DirectGrowthActiveContext {
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::AnnualOrFallow { .. } | Self::Perennial { .. })
    }

    fn runtime_day_of_year(self) -> Result<u16, DirectRuntimeError> {
        match self {
            Self::AnnualOrFallow {
                runtime_day_of_year,
                ..
            }
            | Self::Perennial {
                runtime_day_of_year,
                ..
            } => Ok(runtime_day_of_year),
            Self::Inactive => Err(DirectRuntimeError::DirectDomainViolation {
                field: "growth.runtime_day_of_year",
            }),
            Self::Missing | Self::Ambiguous => Err(DirectRuntimeError::DirectDomainViolation {
                field: "growth.active_context",
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectGrowthAction {
    None,
    PrePlantSkip,
    PlantingReset,
    HarvestReset,
    StopReset,
    Cut,
    Grazing,
}

impl DirectGrowthAction {
    #[must_use]
    pub const fn consumes_plant_state(self) -> bool {
        matches!(
            self,
            Self::None
                | Self::PlantingReset
                | Self::HarvestReset
                | Self::StopReset
                | Self::Cut
                | Self::Grazing
        )
    }

    const fn is_reset(self) -> bool {
        matches!(
            self,
            Self::PlantingReset | Self::HarvestReset | Self::StopReset
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthStateSurface {
    pub sumgdd: f64,
    pub live_biomass_kg_m2: f64,
    pub interception_live_biomass_kg_m2: f64,
    pub canopy_cover_fraction: f64,
    pub leaf_area_index: f64,
    pub root_mass_kg_m2: f64,
    pub root_depth_m: f64,
    pub harvest_index: f64,
}

impl DirectGrowthStateSurface {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            sumgdd: 0.0,
            live_biomass_kg_m2: 0.0,
            interception_live_biomass_kg_m2: 0.0,
            canopy_cover_fraction: 0.0,
            leaf_area_index: 0.0,
            root_mass_kg_m2: 0.0,
            root_depth_m: 0.0,
            harvest_index: 0.0,
        }
    }

    fn validate(self, field_root: &'static str) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(field_root, self.sumgdd)?;
        validate_nonnegative_direct_m(field_root, self.live_biomass_kg_m2)?;
        validate_nonnegative_direct_m(field_root, self.interception_live_biomass_kg_m2)?;
        validate_between(
            "growth.canopy_cover_fraction",
            self.canopy_cover_fraction,
            0.0,
            PL_GROWTH_CANCOV_MAX,
        )?;
        validate_nonnegative_direct_m(field_root, self.leaf_area_index)?;
        validate_nonnegative_direct_m(field_root, self.root_mass_kg_m2)?;
        validate_nonnegative_direct_m(field_root, self.root_depth_m)?;
        validate_between("growth.harvest_index", self.harvest_index, 0.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthInputs {
    pub active_context: DirectGrowthActiveContext,
    pub active_action: DirectGrowthAction,
    pub state_before: DirectGrowthStateSurface,
    pub planting_day: u16,
    pub harvest_day: u16,
    pub stop_day: u16,
    pub water_stress: f64,
    pub temperature_max_c: f64,
    pub temperature_min_c: f64,
    pub radiation_mj_m2: f64,
    pub monthly_temperature_max_c: [f64; 12],
    pub monthly_temperature_min_c: [f64; 12],
    pub soil_depth_m: f64,
    pub btemp: f64,
    pub otemp: f64,
    pub gddmax: f64,
    pub dlai: f64,
    pub dropfc: f64,
    pub decfct: f64,
    pub spriod: f64,
    pub bb: f64,
    pub beinp: f64,
    pub extnct: f64,
    pub hi: f64,
    pub xmxlai: f64,
    pub rsr: f64,
    pub rtmmax: f64,
    pub rdmax: f64,
    pub et_demand_m: f64,
    pub residue_interception_m: f64,
    pub plant_tolerance: f64,
}

impl DirectGrowthInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_context: DirectGrowthActiveContext::Inactive,
            active_action: DirectGrowthAction::None,
            state_before: DirectGrowthStateSurface::zero(),
            planting_day: 1,
            harvest_day: 1,
            stop_day: 0,
            water_stress: 1.0,
            temperature_max_c: 0.0,
            temperature_min_c: 0.0,
            radiation_mj_m2: 0.0,
            monthly_temperature_max_c: [0.0; 12],
            monthly_temperature_min_c: [0.0; 12],
            soil_depth_m: 0.0,
            btemp: 0.0,
            otemp: 0.0,
            gddmax: 0.0,
            dlai: 0.0,
            dropfc: 0.0,
            decfct: 0.0,
            spriod: 0.0,
            bb: 0.0,
            beinp: 0.0,
            extnct: 0.0,
            hi: 0.0,
            xmxlai: 0.0,
            rsr: 0.0,
            rtmmax: 0.0,
            rdmax: 0.0,
            et_demand_m: 0.0,
            residue_interception_m: 0.0,
            plant_tolerance: 0.0,
        }
    }

    fn from_frame_annual(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        frame
            .annual_growth_inputs
            .validate_for_management_class(DirectGrowthManagementClass::AnnualOrFallow)?;
        Ok(frame.annual_growth_inputs)
    }

    fn from_frame_perennial(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        frame
            .perennial_growth_inputs
            .validate_for_management_class(DirectGrowthManagementClass::Perennial)?;
        Ok(frame.perennial_growth_inputs)
    }

    pub fn compute_annual_or_fallow(self) -> Result<DirectGrowthState, DirectRuntimeError> {
        self.compute_for_management_class(DirectGrowthManagementClass::AnnualOrFallow)
    }

    pub fn compute_perennial(self) -> Result<DirectGrowthState, DirectRuntimeError> {
        self.compute_for_management_class(DirectGrowthManagementClass::Perennial)
    }

    fn validate_for_management_class(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<(), DirectRuntimeError> {
        self.validate_context_for_management_class(management_class)?;
        self.state_before.validate("growth.state_before")?;
        self.validate_schedule_domain(management_class)?;
        if self.active_context.is_active() && self.active_action.requires_equation_inputs() {
            self.validate_equation_inputs(management_class)?;
        } else if self.active_context.is_active() {
            self.validate_downstream_inputs()?;
        }
        Ok(())
    }

    fn validate_context_for_management_class(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<(), DirectRuntimeError> {
        match (management_class, self.active_context) {
            (
                DirectGrowthManagementClass::AnnualOrFallow
                | DirectGrowthManagementClass::Perennial,
                DirectGrowthActiveContext::Inactive,
            ) => {
                if self.active_action == DirectGrowthAction::None {
                    Ok(())
                } else {
                    Err(DirectRuntimeError::DirectDomainViolation {
                        field: "growth.active_action",
                    })
                }
            }
            (
                DirectGrowthManagementClass::AnnualOrFallow,
                DirectGrowthActiveContext::AnnualOrFallow {
                    runtime_day_of_year,
                    ..
                },
            )
            | (
                DirectGrowthManagementClass::Perennial,
                DirectGrowthActiveContext::Perennial {
                    runtime_day_of_year,
                    ..
                },
            ) => validate_runtime_day(runtime_day_of_year),
            (
                DirectGrowthManagementClass::AnnualOrFallow,
                DirectGrowthActiveContext::Perennial { .. },
            )
            | (
                DirectGrowthManagementClass::Perennial,
                DirectGrowthActiveContext::AnnualOrFallow { .. },
            ) => Err(DirectRuntimeError::DirectDomainViolation {
                field: "growth.active_context",
            }),
            (_, DirectGrowthActiveContext::Missing | DirectGrowthActiveContext::Ambiguous) => {
                Err(DirectRuntimeError::DirectDomainViolation {
                    field: "growth.active_context",
                })
            }
        }
    }

    fn validate_schedule_domain(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<(), DirectRuntimeError> {
        match management_class {
            DirectGrowthManagementClass::AnnualOrFallow => {
                validate_runtime_day(self.planting_day)?;
                validate_runtime_day(self.harvest_day)?;
                match self.active_action {
                    DirectGrowthAction::None | DirectGrowthAction::Cut => {
                        if self.active_context.is_active()
                            && !self.is_annual_active_growth_window()?
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.annual_active_window",
                            });
                        }
                    }
                    DirectGrowthAction::PrePlantSkip => {
                        if self.active_context.is_active()
                            && self.is_annual_active_growth_window()?
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.active_action",
                            });
                        }
                    }
                    DirectGrowthAction::PlantingReset => {
                        if self.active_context.is_active()
                            && self.active_context.runtime_day_of_year()? != self.planting_day
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.planting_day",
                            });
                        }
                    }
                    DirectGrowthAction::HarvestReset => {
                        if self.active_context.is_active()
                            && self.active_context.runtime_day_of_year()? != self.harvest_day
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.harvest_day",
                            });
                        }
                    }
                    DirectGrowthAction::StopReset | DirectGrowthAction::Grazing => {
                        return Err(DirectRuntimeError::DirectDomainViolation {
                            field: "growth.active_action",
                        });
                    }
                }
            }
            DirectGrowthManagementClass::Perennial => {
                validate_optional_runtime_day(self.planting_day)?;
                validate_optional_runtime_day(self.harvest_day)?;
                validate_optional_runtime_day(self.stop_day)?;
                match self.active_action {
                    DirectGrowthAction::None
                    | DirectGrowthAction::Cut
                    | DirectGrowthAction::Grazing => {}
                    DirectGrowthAction::PlantingReset => {
                        if self.active_context.is_active()
                            && self.planting_day != 0
                            && self.active_context.runtime_day_of_year()? != self.planting_day
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.planting_day",
                            });
                        }
                    }
                    DirectGrowthAction::StopReset => {
                        if self.active_context.is_active()
                            && (self.stop_day == 0
                                || self.active_context.runtime_day_of_year()? != self.stop_day)
                        {
                            return Err(DirectRuntimeError::DirectDomainViolation {
                                field: "growth.stop_day",
                            });
                        }
                    }
                    DirectGrowthAction::HarvestReset | DirectGrowthAction::PrePlantSkip => {
                        return Err(DirectRuntimeError::DirectDomainViolation {
                            field: "growth.active_action",
                        });
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn validate_equation_inputs(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<(), DirectRuntimeError> {
        self.validate_downstream_inputs()?;
        validate_between("growth.water_stress", self.water_stress, 0.0, 1.0)?;
        validate_finite("growth.temperature_max_c", self.temperature_max_c)?;
        validate_finite("growth.temperature_min_c", self.temperature_min_c)?;
        validate_nonnegative_direct_m("growth.radiation_mj_m2", self.radiation_mj_m2)?;
        validate_positive("growth.soil_depth_m", self.soil_depth_m)?;
        validate_finite("growth.btemp", self.btemp)?;
        validate_finite("growth.otemp", self.otemp)?;
        if self.otemp <= self.btemp {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "growth.otemp",
            });
        }
        validate_nonnegative_direct_m("growth.gddmax", self.gddmax)?;
        validate_between("growth.dlai", self.dlai, f64::EPSILON, 1.0)?;
        validate_between("growth.dropfc", self.dropfc, 0.0, 1.0)?;
        validate_between("growth.decfct", self.decfct, 0.0, 1.0)?;
        validate_nonnegative_direct_m("growth.spriod", self.spriod)?;
        validate_nonnegative_direct_m("growth.bb", self.bb)?;
        validate_nonnegative_direct_m("growth.beinp", self.beinp)?;
        validate_nonnegative_direct_m("growth.extnct", self.extnct)?;
        validate_between("growth.hi", self.hi, 0.0, 1.0)?;
        validate_nonnegative_direct_m("growth.xmxlai", self.xmxlai)?;
        validate_nonnegative_direct_m("growth.rsr", self.rsr)?;
        if management_class == DirectGrowthManagementClass::Perennial {
            validate_positive("growth.rtmmax", self.rtmmax)?;
        } else {
            validate_nonnegative_direct_m("growth.rtmmax", self.rtmmax)?;
        }
        validate_positive("growth.rdmax", self.rdmax)?;
        if self.gddmax <= 0.0 {
            for value in self.monthly_temperature_max_c {
                validate_finite("growth.monthly_temperature_max_c", value)?;
            }
            for value in self.monthly_temperature_min_c {
                validate_finite("growth.monthly_temperature_min_c", value)?;
            }
        }
        Ok(())
    }

    fn validate_downstream_inputs(self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m("growth.et_demand_m", self.et_demand_m)?;
        validate_nonnegative_direct_m(
            "growth.residue_interception_m",
            self.residue_interception_m,
        )?;
        validate_finite("growth.plant_tolerance", self.plant_tolerance)
    }

    fn is_annual_active_growth_window(self) -> Result<bool, DirectRuntimeError> {
        let day = self.active_context.runtime_day_of_year()?;
        Ok(if self.harvest_day > self.planting_day {
            day >= self.planting_day && day <= self.harvest_day
        } else {
            day >= self.planting_day || day <= self.harvest_day
        })
    }

    fn compute_for_management_class(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<DirectGrowthState, DirectRuntimeError> {
        self.validate_for_management_class(management_class)?;

        let state_after = if self.active_context == DirectGrowthActiveContext::Inactive
            || self.active_action == DirectGrowthAction::PrePlantSkip
        {
            self.state_before
        } else if self.active_action.is_reset() {
            DirectGrowthStateSurface::zero()
        } else {
            self.compute_equation_growth_state(management_class)?
        };
        state_after.validate("growth.state_after")?;

        let mut state = DirectGrowthState {
            active_context: self.active_context,
            active_action: self.active_action,
            state_before: self.state_before,
            state_after,
            water_stress: self.water_stress,
            temperature_stress: 0.0,
            regulation_factor: 0.0,
            gdd: 0.0,
            gddmax_effective: 0.0,
            fphu: 0.0,
            par: 0.0,
            daily_biomass_increment_kg_m2: 0.0,
            senescence_biomass_decline_fraction: 0.0,
            senescence_canopy_decline_fraction: 0.0,
            et_demand_m: self.et_demand_m,
            residue_interception_m: self.residue_interception_m,
            plant_tolerance: self.plant_tolerance,
        };

        if self.active_context.is_active() && self.active_action.requires_equation_inputs() {
            state = self.compute_equation_diagnostics(management_class, state)?;
        }
        Ok(state)
    }

    #[allow(clippy::too_many_lines)]
    fn compute_equation_growth_state(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<DirectGrowthStateSurface, DirectRuntimeError> {
        let tave = f64::midpoint(self.temperature_max_c, self.temperature_min_c);
        validate_finite("growth.average_temperature_c", tave)?;
        let gdd = (tave - self.btemp).max(0.0);
        let gddmax_effective = self.resolve_effective_gddmax(management_class)?;
        let sumgdd_before = if self.state_before.sumgdd <= 0.0
            && self.state_before.leaf_area_index > 0.0
            && self.xmxlai > 0.0
        {
            (gddmax_effective * self.state_before.leaf_area_index / self.xmxlai)
                .min(gddmax_effective)
        } else {
            self.state_before.sumgdd
        };
        let sumgdd_next = (sumgdd_before + gdd).min(gddmax_effective);
        let fphu = (sumgdd_next / gddmax_effective).clamp(0.0, 1.0);

        let temp_ratio = (gdd / (self.otemp - self.btemp)).min(1.0);
        let temstr = (std::f64::consts::FRAC_PI_2 * temp_ratio)
            .sin()
            .clamp(0.0, 1.0);
        let reg = self.water_stress.min(temstr);

        let par = PL_GROWTH_PAR_RAD_SCALE
            * self.radiation_mj_m2
            * (1.0
                - (-self.extnct * (self.state_before.leaf_area_index + PL_GROWTH_PAR_LAI_OFFSET))
                    .exp());
        validate_nonnegative_direct_m("growth.par", par)?;
        let ddm = PL_GROWTH_DDM_SCALE * self.beinp * par;
        validate_nonnegative_direct_m("growth.daily_biomass_increment_kg_m2", ddm)?;

        let biomass_increment = ddm * reg;
        let mut vdmt_next = self.state_before.live_biomass_kg_m2 + biomass_increment;
        let mut interception_live_biomass_next =
            self.state_before.interception_live_biomass_kg_m2 + biomass_increment;
        if fphu >= self.dlai && self.spriod > 0.0 {
            let biomass_decline = (1.0 - self.dropfc) / self.spriod;
            validate_between(
                "growth.senescence_biomass_decline_fraction",
                biomass_decline,
                0.0,
                1.0,
            )?;
            vdmt_next *= 1.0 - biomass_decline;
            interception_live_biomass_next *= 1.0 - biomass_decline;
        }
        vdmt_next = vdmt_next.max(0.0);
        interception_live_biomass_next = interception_live_biomass_next.max(0.0);

        let hufh_denom = fphu + (6.5 - 10.0 * fphu).exp();
        validate_positive("growth.harvest_index_denominator", hufh_denom)?;
        let mut hia_next = self.hi * (fphu / hufh_denom);
        let water_stress_adjustment = if (0.3..0.9).contains(&fphu) {
            (std::f64::consts::FRAC_PI_2 * (fphu - 0.3) / 0.3).sin()
        } else {
            0.0
        };
        hia_next -= self.hi
            * (1.0 - 1.0 / (1.0 + 0.01 * water_stress_adjustment * (0.9 - self.water_stress)));
        hia_next = hia_next.clamp(0.0, self.hi);

        let canopy_biomass = if management_class == DirectGrowthManagementClass::Perennial {
            vdmt_next
        } else {
            vdmt_next * (1.0 - hia_next)
        };
        let cancov_raw = 1.0 - (-self.bb * canopy_biomass).exp();
        validate_finite("growth.canopy_cover_raw", cancov_raw)?;
        let mut cancov_next = cancov_raw.clamp(0.0, PL_GROWTH_CANCOV_MAX);
        if fphu >= self.dlai && self.spriod > 0.0 {
            let canopy_decline = (1.0 - self.decfct) / self.spriod;
            validate_between(
                "growth.senescence_canopy_decline_fraction",
                canopy_decline,
                0.0,
                1.0,
            )?;
            cancov_next = (cancov_next * (1.0 - canopy_decline)).clamp(0.0, PL_GROWTH_CANCOV_MAX);
        }

        let lai_next = if management_class == DirectGrowthManagementClass::Perennial {
            let denom = vdmt_next
                + PL_GROWTH_PERENNIAL_LAI_A * (-PL_GROWTH_PERENNIAL_LAI_B * vdmt_next).exp();
            validate_positive("growth.perennial_lai_denominator", denom)?;
            self.xmxlai * vdmt_next / denom
        } else {
            let vegetative_biomass = vdmt_next * (1.0 - hia_next);
            let denom = vegetative_biomass
                + PL_GROWTH_ANNUAL_LAI_A * (-PL_GROWTH_ANNUAL_LAI_B * vegetative_biomass).exp();
            validate_positive("growth.annual_lai_denominator", denom)?;
            self.xmxlai * vegetative_biomass / denom
        };
        validate_nonnegative_direct_m("growth.leaf_area_index", lai_next)?;

        let rtmass_unclamped = self.state_before.root_mass_kg_m2
            + (vdmt_next - self.state_before.live_biomass_kg_m2) * self.rsr;
        let rtmass_next = if management_class == DirectGrowthManagementClass::Perennial {
            rtmass_unclamped.clamp(0.0, self.rtmmax)
        } else {
            rtmass_unclamped.max(0.0)
        };

        let rtd_floor = self.rdmax
            * 0.5
            * (1.0
                + (PL_GROWTH_ROOT_DEPTH_CURVE_A * fphu / self.dlai - PL_GROWTH_ROOT_DEPTH_CURVE_B)
                    .sin());
        let rtd_upper = self.rdmax.min(self.soil_depth_m);
        validate_positive("growth.root_depth_upper_bound_m", rtd_upper)?;
        let rtd_candidate = if management_class == DirectGrowthManagementClass::Perennial {
            let growth_increment =
                ((rtmass_next - self.state_before.root_mass_kg_m2) / self.rtmmax) * self.rdmax;
            (self.state_before.root_depth_m + growth_increment).max(rtd_floor)
        } else {
            rtd_floor
        };
        validate_nonnegative_direct_m("growth.root_depth_candidate_m", rtd_candidate)?;
        let rtd_next = rtd_candidate.min(rtd_upper);

        DirectGrowthStateSurface {
            sumgdd: sumgdd_next,
            live_biomass_kg_m2: vdmt_next,
            interception_live_biomass_kg_m2: interception_live_biomass_next,
            canopy_cover_fraction: cancov_next,
            leaf_area_index: lai_next,
            root_mass_kg_m2: rtmass_next,
            root_depth_m: rtd_next,
            harvest_index: hia_next,
        }
        .tap_validate("growth.state_after")
    }

    fn compute_equation_diagnostics(
        self,
        management_class: DirectGrowthManagementClass,
        mut state: DirectGrowthState,
    ) -> Result<DirectGrowthState, DirectRuntimeError> {
        let tave = f64::midpoint(self.temperature_max_c, self.temperature_min_c);
        let gdd = (tave - self.btemp).max(0.0);
        let gddmax_effective = self.resolve_effective_gddmax(management_class)?;
        let fphu = (state.state_after.sumgdd / gddmax_effective).clamp(0.0, 1.0);
        let temp_ratio = (gdd / (self.otemp - self.btemp)).min(1.0);
        let temstr = (std::f64::consts::FRAC_PI_2 * temp_ratio)
            .sin()
            .clamp(0.0, 1.0);
        let par = PL_GROWTH_PAR_RAD_SCALE
            * self.radiation_mj_m2
            * (1.0
                - (-self.extnct * (self.state_before.leaf_area_index + PL_GROWTH_PAR_LAI_OFFSET))
                    .exp());
        let biomass_decline = if fphu >= self.dlai && self.spriod > 0.0 {
            (1.0 - self.dropfc) / self.spriod
        } else {
            0.0
        };
        let canopy_decline = if fphu >= self.dlai && self.spriod > 0.0 {
            (1.0 - self.decfct) / self.spriod
        } else {
            0.0
        };

        state.temperature_stress = temstr;
        state.regulation_factor = self.water_stress.min(temstr);
        state.gdd = gdd;
        state.gddmax_effective = gddmax_effective;
        state.fphu = fphu;
        state.par = par;
        state.daily_biomass_increment_kg_m2 = PL_GROWTH_DDM_SCALE * self.beinp * par;
        state.senescence_biomass_decline_fraction = biomass_decline;
        state.senescence_canopy_decline_fraction = canopy_decline;
        Ok(state)
    }

    fn resolve_effective_gddmax(
        self,
        management_class: DirectGrowthManagementClass,
    ) -> Result<f64, DirectRuntimeError> {
        if self.gddmax > 0.0 {
            return Ok(self.gddmax);
        }

        let resolved = if management_class == DirectGrowthManagementClass::Perennial {
            self.legacy_gdmax_from_monthly(1, PL_GROWTH_GDMAX_YEAR_END_DAY)?
        } else if self.harvest_day > self.planting_day {
            self.legacy_gdmax_from_monthly(
                usize::from(self.planting_day),
                usize::from(self.harvest_day),
            )?
        } else {
            self.legacy_gdmax_from_monthly(
                usize::from(self.planting_day),
                PL_GROWTH_GDMAX_YEAR_END_DAY,
            )? + self.legacy_gdmax_from_monthly(1, usize::from(self.harvest_day))?
        };
        validate_positive("growth.gddmax_effective", resolved)?;
        Ok(resolved)
    }

    fn legacy_gdmax_from_monthly(
        self,
        start_day: usize,
        end_day: usize,
    ) -> Result<f64, DirectRuntimeError> {
        if start_day == 0 || end_day == 0 || start_day > end_day || end_day > 366 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "growth.gddmax_day_window",
            });
        }

        let start_month = legacy_gdmax_month_for_day(start_day).ok_or(
            DirectRuntimeError::DirectDomainViolation {
                field: "growth.gddmax_start_month",
            },
        )?;
        let end_month = legacy_gdmax_month_for_day(end_day).ok_or(
            DirectRuntimeError::DirectDomainViolation {
                field: "growth.gddmax_end_month",
            },
        )?;

        let start_days = PL_GROWTH_GDMAX_MONTH_DAY_STARTS[start_month] - start_day + 1;
        let end_days = end_day - PL_GROWTH_GDMAX_MONTH_DAY_STARTS[end_month - 1];
        let mut sumgd = 0.0;

        let start_tave = f64::midpoint(
            self.monthly_temperature_max_c[start_month - 1],
            self.monthly_temperature_min_c[start_month - 1],
        );
        if start_tave > self.btemp {
            sumgd += (start_tave - self.btemp) * day_count_to_f64(start_days)?;
        }

        for month in (start_month + 1)..end_month {
            let tave = f64::midpoint(
                self.monthly_temperature_max_c[month - 1],
                self.monthly_temperature_min_c[month - 1],
            );
            if tave > self.btemp {
                sumgd += (tave - self.btemp)
                    * day_count_to_f64(PL_GROWTH_GDMAX_MONTH_LENGTHS[month - 1])?;
            }
        }

        let end_tave = f64::midpoint(
            self.monthly_temperature_max_c[end_month - 1],
            self.monthly_temperature_min_c[end_month - 1],
        );
        if end_tave > self.btemp {
            sumgd += (end_tave - self.btemp) * day_count_to_f64(end_days)?;
        }
        validate_finite("growth.gddmax_effective", sumgd)?;
        Ok(sumgd)
    }
}

trait DirectGrowthActionExt {
    fn requires_equation_inputs(self) -> bool;
}

impl DirectGrowthActionExt for DirectGrowthAction {
    fn requires_equation_inputs(self) -> bool {
        matches!(self, Self::None | Self::Cut | Self::Grazing)
    }
}

trait ValidateGrowthSurface {
    fn tap_validate(self, field_root: &'static str) -> Result<Self, DirectRuntimeError>
    where
        Self: Sized;
}

impl ValidateGrowthSurface for DirectGrowthStateSurface {
    fn tap_validate(self, field_root: &'static str) -> Result<Self, DirectRuntimeError> {
        self.validate(field_root)?;
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthState {
    pub active_context: DirectGrowthActiveContext,
    pub active_action: DirectGrowthAction,
    pub state_before: DirectGrowthStateSurface,
    pub state_after: DirectGrowthStateSurface,
    pub water_stress: f64,
    pub temperature_stress: f64,
    pub regulation_factor: f64,
    pub gdd: f64,
    pub gddmax_effective: f64,
    pub fphu: f64,
    pub par: f64,
    pub daily_biomass_increment_kg_m2: f64,
    pub senescence_biomass_decline_fraction: f64,
    pub senescence_canopy_decline_fraction: f64,
    pub et_demand_m: f64,
    pub residue_interception_m: f64,
    pub plant_tolerance: f64,
}

impl DirectGrowthState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_context: DirectGrowthActiveContext::Inactive,
            active_action: DirectGrowthAction::None,
            state_before: DirectGrowthStateSurface::zero(),
            state_after: DirectGrowthStateSurface::zero(),
            water_stress: 1.0,
            temperature_stress: 0.0,
            regulation_factor: 0.0,
            gdd: 0.0,
            gddmax_effective: 0.0,
            fphu: 0.0,
            par: 0.0,
            daily_biomass_increment_kg_m2: 0.0,
            senescence_biomass_decline_fraction: 0.0,
            senescence_canopy_decline_fraction: 0.0,
            et_demand_m: 0.0,
            residue_interception_m: 0.0,
            plant_tolerance: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthDownstreamOperands {
    pub active_context: DirectGrowthActiveContext,
    pub active_action: DirectGrowthAction,
    pub state_after: DirectGrowthStateSurface,
    pub root_depth_m: f64,
    pub leaf_area_index: f64,
    pub canopy_cover_fraction: f64,
    pub water_stress: f64,
    pub et_demand_m: f64,
    pub residue_interception_m: f64,
    pub plant_tolerance: f64,
}

impl DirectGrowthDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_context: DirectGrowthActiveContext::Inactive,
            active_action: DirectGrowthAction::None,
            state_after: DirectGrowthStateSurface::zero(),
            root_depth_m: 0.0,
            leaf_area_index: 0.0,
            canopy_cover_fraction: 0.0,
            water_stress: 1.0,
            et_demand_m: 0.0,
            residue_interception_m: 0.0,
            plant_tolerance: 0.0,
        }
    }
}

impl From<DirectGrowthState> for DirectGrowthDownstreamOperands {
    fn from(state: DirectGrowthState) -> Self {
        Self {
            active_context: state.active_context,
            active_action: state.active_action,
            state_after: state.state_after,
            root_depth_m: state.state_after.root_depth_m,
            leaf_area_index: state.state_after.leaf_area_index,
            canopy_cover_fraction: state.state_after.canopy_cover_fraction,
            water_stress: state.water_stress,
            et_demand_m: state.et_demand_m,
            residue_interception_m: state.residue_interception_m,
            plant_tolerance: state.plant_tolerance,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub active_context: DirectGrowthActiveContext,
    pub active_action: DirectGrowthAction,
    pub state_after: DirectGrowthStateSurface,
    pub root_depth_m: f64,
    pub leaf_area_index: f64,
    pub canopy_cover_fraction: f64,
    pub water_stress: f64,
    pub et_demand_m: f64,
    pub residue_interception_m: f64,
    pub plant_tolerance: f64,
}

impl DirectGrowthShadowProjection {
    #[must_use]
    pub const fn from_operands(
        lane_index: usize,
        day_index: usize,
        operands: DirectGrowthDownstreamOperands,
    ) -> Self {
        Self {
            lane_index,
            day_index,
            active_context: operands.active_context,
            active_action: operands.active_action,
            state_after: operands.state_after,
            root_depth_m: operands.root_depth_m,
            leaf_area_index: operands.leaf_area_index,
            canopy_cover_fraction: operands.canopy_cover_fraction,
            water_stress: operands.water_stress,
            et_demand_m: operands.et_demand_m,
            residue_interception_m: operands.residue_interception_m,
            plant_tolerance: operands.plant_tolerance,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGrowthSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub growth_shadow_projection: DirectGrowthShadowProjection,
}

fn validate_runtime_day(day: u16) -> Result<(), DirectRuntimeError> {
    if (1..=366).contains(&day) {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "growth.runtime_day_of_year",
        })
    }
}

fn validate_optional_runtime_day(day: u16) -> Result<(), DirectRuntimeError> {
    if day == 0 || (1..=366).contains(&day) {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "growth.optional_day_of_year",
        })
    }
}

fn validate_positive(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn validate_between(
    field: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value >= minimum - WB11_ZERO_THRESHOLD && value <= maximum + WB11_ZERO_THRESHOLD {
        Ok(())
    } else if value < minimum {
        Err(DirectRuntimeError::NegativeDirectValue { field })
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn legacy_gdmax_month_for_day(day: usize) -> Option<usize> {
    if day == 0 || day > PL_GROWTH_GDMAX_MONTH_DAY_STARTS[12] {
        return None;
    }
    (1..=12).find(|month| day <= PL_GROWTH_GDMAX_MONTH_DAY_STARTS[*month])
}

fn day_count_to_f64(day_count: usize) -> Result<f64, DirectRuntimeError> {
    let day_count_u16 =
        u16::try_from(day_count).map_err(|_| DirectRuntimeError::DirectDomainViolation {
            field: "growth.gddmax_day_count",
        })?;
    Ok(f64::from(day_count_u16))
}
