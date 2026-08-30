const SNOW_DENSITY_LEGACY_MODEL_ID: &str = "legacy_wepp";
const SNOW_DENSITY_COMPACTION_MODEL_ID: &str = "physics_bulk_density_compaction_v1";
const SNOW_DENSITY_SPRING_DENSIFICATION_MODEL_ID: &str = "physics_bulk_spring_densification_v1";
const SNOW_DENSITY_SHALLOW_GUARD_MODEL_ID: &str = "physics_bulk_shallow_guard_v1";
const SNOW_DENSITY_CLIMATE_CLASS_MODEL_ID: &str = "physics_bulk_climate_class_density_v1";
const SNOW_DENSITY_MULTILAYER_MODEL_ID: &str = "physics_bulk_multilayer_density_v1";
const SNOW_DENSITY_SHALLOW_GUARD_DEPTH_THRESHOLD_M: f64 = 0.25;
const SNOW_DENSITY_RHO_WATER_KG_M3: f64 = 1_000.0;
const SNOW_DENSITY_ZERO_MASS_KG_M2: f64 = 1.0e-9;
const SNOW_DENSITY_DAILY_COMPACTION_STEPS: u8 = 24;
const SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const SNOW_DENSITY_DIAGNOSTIC_CLOSURE_TOLERANCE_KG_M3: f64 = 1.0e-9;
const SNOW_DENSITY_MULTILAYER_MAX_LAYERS: usize = 16;
pub const STURM1995_CDM_CRITICAL_TEMPERATURE_C: f64 = 10.0;
pub const STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH: f64 = 30.0;
pub const STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH: f64 = 125.0;
pub const STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY: f64 = 2.0;
pub const STURM1995_LOW_WIND_MAX_M_S: f64 = 0.5;
pub const STURM1995_HIGH_WIND_MIN_M_S: f64 = 2.0;

pub(crate) fn snow_density_layer_has_resolved_mass(mass_swe_m: f64) -> bool {
    openwepp_unit_boundary::conversions::snow_water_equivalent_meters_to_area_mass_kg_m2(mass_swe_m)
        > SNOW_DENSITY_ZERO_MASS_KG_M2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowDensityModel {
    LegacyWepp,
    PhysicsBulkDensityCompactionV1,
    PhysicsBulkSpringDensificationV1,
    PhysicsBulkShallowGuardV1,
    PhysicsBulkClimateClassDensityV1,
    PhysicsBulkMultilayerDensityV1,
}

impl SnowDensityModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::LegacyWepp => SNOW_DENSITY_LEGACY_MODEL_ID,
            Self::PhysicsBulkDensityCompactionV1 => SNOW_DENSITY_COMPACTION_MODEL_ID,
            Self::PhysicsBulkSpringDensificationV1 => SNOW_DENSITY_SPRING_DENSIFICATION_MODEL_ID,
            Self::PhysicsBulkShallowGuardV1 => SNOW_DENSITY_SHALLOW_GUARD_MODEL_ID,
            Self::PhysicsBulkClimateClassDensityV1 => SNOW_DENSITY_CLIMATE_CLASS_MODEL_ID,
            Self::PhysicsBulkMultilayerDensityV1 => SNOW_DENSITY_MULTILAYER_MODEL_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowClimateClass {
    Tundra,
    Taiga,
    Alpine,
    Maritime,
    Prairie,
    Ephemeral,
}

impl SnowClimateClass {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Tundra => "tundra",
            Self::Taiga => "taiga",
            Self::Alpine => "alpine",
            Self::Maritime => "maritime",
            Self::Prairie => "prairie",
            Self::Ephemeral => "ephemeral",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sturm1995ClimateNormals {
    pub cooling_degree_month_c: f64,
    pub snowfall_precipitation_rate_mm_day: f64,
    pub winter_wind_m_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sturm1995ClimateClassAssignmentError {
    NonFiniteInput {
        symbol: &'static str,
        value: f64,
    },
    NegativeInput {
        symbol: &'static str,
        value: f64,
    },
    AmbiguousWindThreshold {
        wind_m_s: f64,
        low_max_m_s: f64,
        high_min_m_s: f64,
    },
    RareClassCombination {
        cooling_degree_month_c: f64,
        snowfall_precipitation_rate_mm_day: f64,
    },
}

impl std::fmt::Display for Sturm1995ClimateClassAssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput { symbol, value } => {
                write!(f, "non-finite Sturm 1995 climate normal {symbol}={value}")
            }
            Self::NegativeInput { symbol, value } => {
                write!(f, "negative Sturm 1995 climate normal {symbol}={value}")
            }
            Self::AmbiguousWindThreshold {
                wind_m_s,
                low_max_m_s,
                high_min_m_s,
            } => write!(
                f,
                "Sturm 1995 wind normal {wind_m_s} m/s is inside unresolved ({low_max_m_s}, {high_min_m_s}) m/s bracket"
            ),
            Self::RareClassCombination {
                cooling_degree_month_c,
                snowfall_precipitation_rate_mm_day,
            } => write!(
                f,
                "Sturm 1995 rare deep-snow branch has no six-class runtime label: CDM={cooling_degree_month_c}, SPR={snowfall_precipitation_rate_mm_day}"
            ),
        }
    }
}

impl std::error::Error for Sturm1995ClimateClassAssignmentError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sturm1995WindBranch {
    Low,
    High,
}

pub fn sturm1995_climate_class_from_normals(
    normals: Sturm1995ClimateNormals,
) -> Result<SnowClimateClass, Sturm1995ClimateClassAssignmentError> {
    validate_sturm1995_normal("cooling_degree_month_c", normals.cooling_degree_month_c)?;
    validate_sturm1995_normal(
        "snowfall_precipitation_rate_mm_day",
        normals.snowfall_precipitation_rate_mm_day,
    )?;
    validate_sturm1995_normal("winter_wind_m_s", normals.winter_wind_m_s)?;

    if normals.cooling_degree_month_c < STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH {
        return Ok(SnowClimateClass::Ephemeral);
    }
    let low_temperature =
        normals.cooling_degree_month_c >= STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH;
    let high_precipitation =
        normals.snowfall_precipitation_rate_mm_day >= STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY;

    if low_temperature && high_precipitation {
        return Err(Sturm1995ClimateClassAssignmentError::RareClassCombination {
            cooling_degree_month_c: normals.cooling_degree_month_c,
            snowfall_precipitation_rate_mm_day: normals.snowfall_precipitation_rate_mm_day,
        });
    }
    if !low_temperature && high_precipitation {
        return Ok(SnowClimateClass::Maritime);
    }

    match (
        low_temperature,
        sturm1995_wind_branch(normals.winter_wind_m_s)?,
    ) {
        (true, Sturm1995WindBranch::Low) => Ok(SnowClimateClass::Taiga),
        (true, Sturm1995WindBranch::High) => Ok(SnowClimateClass::Tundra),
        (false, Sturm1995WindBranch::Low) => Ok(SnowClimateClass::Alpine),
        (false, Sturm1995WindBranch::High) => Ok(SnowClimateClass::Prairie),
    }
}

fn validate_sturm1995_normal(
    symbol: &'static str,
    value: f64,
) -> Result<(), Sturm1995ClimateClassAssignmentError> {
    if !value.is_finite() {
        return Err(Sturm1995ClimateClassAssignmentError::NonFiniteInput { symbol, value });
    }
    if value < 0.0 {
        return Err(Sturm1995ClimateClassAssignmentError::NegativeInput { symbol, value });
    }
    Ok(())
}

fn sturm1995_wind_branch(
    wind_m_s: f64,
) -> Result<Sturm1995WindBranch, Sturm1995ClimateClassAssignmentError> {
    if wind_m_s <= STURM1995_LOW_WIND_MAX_M_S {
        Ok(Sturm1995WindBranch::Low)
    } else if wind_m_s >= STURM1995_HIGH_WIND_MIN_M_S {
        Ok(Sturm1995WindBranch::High)
    } else {
        Err(
            Sturm1995ClimateClassAssignmentError::AmbiguousWindThreshold {
                wind_m_s,
                low_max_m_s: STURM1995_LOW_WIND_MAX_M_S,
                high_min_m_s: STURM1995_HIGH_WIND_MIN_M_S,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sturm2010DensityParameters {
    pub max_density_g_cm3: f64,
    pub initial_density_g_cm3: f64,
    pub depth_densification_per_cm: f64,
    pub day_densification_per_day: f64,
}

#[must_use]
pub const fn sturm2010_density_parameters_for_class(
    class: SnowClimateClass,
) -> Option<Sturm2010DensityParameters> {
    match class {
        SnowClimateClass::Alpine => Some(Sturm2010DensityParameters {
            max_density_g_cm3: 0.5975,
            initial_density_g_cm3: 0.2237,
            depth_densification_per_cm: 0.0012,
            day_densification_per_day: 0.0038,
        }),
        SnowClimateClass::Maritime => Some(Sturm2010DensityParameters {
            max_density_g_cm3: 0.5979,
            initial_density_g_cm3: 0.2578,
            depth_densification_per_cm: 0.0010,
            day_densification_per_day: 0.0038,
        }),
        SnowClimateClass::Prairie => Some(Sturm2010DensityParameters {
            max_density_g_cm3: 0.5940,
            initial_density_g_cm3: 0.2332,
            depth_densification_per_cm: 0.0016,
            day_densification_per_day: 0.0031,
        }),
        SnowClimateClass::Tundra => Some(Sturm2010DensityParameters {
            max_density_g_cm3: 0.3630,
            initial_density_g_cm3: 0.2425,
            depth_densification_per_cm: 0.0029,
            day_densification_per_day: 0.0049,
        }),
        SnowClimateClass::Taiga => Some(Sturm2010DensityParameters {
            max_density_g_cm3: 0.2170,
            initial_density_g_cm3: 0.2170,
            depth_densification_per_cm: 0.0,
            day_densification_per_day: 0.0,
        }),
        SnowClimateClass::Ephemeral => None,
    }
}

pub fn sturm2010_bulk_density_kg_m3(
    class: SnowClimateClass,
    snow_depth_m: f64,
    sturm_day_of_year: f64,
) -> Result<f64, SnowDensityError> {
    density_validate_nonnegative("sturm2010_snow_depth_m", snow_depth_m)?;
    density_validate_positive("sturm2010_day_of_year", sturm_day_of_year)?;
    if sturm_day_of_year > 366.0 {
        return Err(SnowDensityError::OutOfRangeInput {
            symbol: "sturm2010_day_of_year",
            value: sturm_day_of_year,
            minimum: Some(f64::MIN_POSITIVE),
            maximum: Some(366.0),
        });
    }
    let parameters = sturm2010_density_parameters_for_class(class)
        .ok_or(SnowDensityError::MissingClimateClassDensityParameters { class: class.id() })?;
    let depth_cm = snow_depth_m * 100.0;
    let exponent = -parameters.depth_densification_per_cm * depth_cm
        - parameters.day_densification_per_day * sturm_day_of_year;
    let density_g_cm3 = (parameters.max_density_g_cm3 - parameters.initial_density_g_cm3)
        * (1.0 - exponent.exp())
        + parameters.initial_density_g_cm3;
    Ok(density_g_cm3 * SNOW_DENSITY_RHO_WATER_KG_M3)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowDensityCompactionConstants {
    pub new_snow_density_min_kg_m3: f64,
    pub new_snow_density_max_kg_m3: f64,
    pub new_snow_density_base_kg_m3: f64,
    pub new_snow_density_temperature_threshold_c: f64,
    pub new_snow_density_temperature_coefficient: f64,
    pub dry_compaction_max_density_kg_m3: f64,
    pub dry_compaction_swe_max_kg_m2: f64,
    pub wet_compaction_max_density_kg_m3: f64,
    pub wet_compaction_half_saturation_ratio: f64,
    pub dry_compaction_multiplier: f64,
    pub wet_compaction_multiplier: f64,
    pub wet_compaction_substeps_per_day: u8,
    pub compaction_rate_cos_amplitude: f64,
    pub compaction_rate_offset: f64,
    pub ptm_rate_per_hour: f64,
    pub ptm_density_threshold_kg_m3: f64,
    pub ptm_density_decay_m3_per_kg: f64,
    pub ptm_temperature_decay_per_c: f64,
    pub poc_rate_per_hour: f64,
    pub poc_temperature_decay_per_c: f64,
    pub poc_density_decay: f64,
    pub shallow_compaction_guard_depth_threshold_m: f64,
}

#[must_use]
pub const fn snow_density_compaction_v1_constants() -> SnowDensityCompactionConstants {
    SnowDensityCompactionConstants {
        new_snow_density_min_kg_m3: 75.0,
        new_snow_density_max_kg_m3: 250.0,
        new_snow_density_base_kg_m3: 75.0,
        new_snow_density_temperature_threshold_c: -15.0,
        new_snow_density_temperature_coefficient: 1.7,
        dry_compaction_max_density_kg_m3: 550.0,
        dry_compaction_swe_max_kg_m2: 2_000.0,
        wet_compaction_max_density_kg_m3: 550.0,
        wet_compaction_half_saturation_ratio: 0.4,
        dry_compaction_multiplier: 4.0,
        wet_compaction_multiplier: 2.0,
        wet_compaction_substeps_per_day: 1,
        compaction_rate_cos_amplitude: 23.5,
        compaction_rate_offset: 24.5,
        ptm_rate_per_hour: 0.01,
        ptm_density_threshold_kg_m3: 100.0,
        ptm_density_decay_m3_per_kg: 0.046,
        ptm_temperature_decay_per_c: 0.04,
        poc_rate_per_hour: 0.026,
        poc_temperature_decay_per_c: 0.08,
        poc_density_decay: 21.0,
        shallow_compaction_guard_depth_threshold_m: 0.0,
    }
}

#[must_use]
pub const fn snow_density_spring_densification_v1_constants() -> SnowDensityCompactionConstants {
    SnowDensityCompactionConstants {
        wet_compaction_substeps_per_day: SNOW_DENSITY_DAILY_COMPACTION_STEPS,
        ..snow_density_compaction_v1_constants()
    }
}

#[must_use]
pub const fn snow_density_shallow_guard_v1_constants() -> SnowDensityCompactionConstants {
    SnowDensityCompactionConstants {
        shallow_compaction_guard_depth_threshold_m: SNOW_DENSITY_SHALLOW_GUARD_DEPTH_THRESHOLD_M,
        ..snow_density_compaction_v1_constants()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnowDensityRuntimeInputs {
    pub model: SnowDensityModel,
    pub prior_swe_m: f64,
    pub prior_depth_m: f64,
    pub prior_density_kg_m3: f64,
    pub prior_settle_day_count: f64,
    pub prior_layers: Vec<DirectSnowLayerState>,
    pub boundary_swe_after_m: f64,
    pub boundary_depth_after_m: f64,
    pub boundary_density_after_kg_m3: f64,
    pub snow_input_m: f64,
    pub liquid_for_compaction_m: f64,
    pub mean_air_temperature_c: f64,
    pub runtime_density_cap_kg_m3: f64,
    pub sturm_climate_class: Option<SnowClimateClass>,
    pub sturm_day_of_year: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnowDensityRuntimeOutcome {
    pub model: SnowDensityModel,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub max_abs_swe_identity_residual_m: f64,
    pub max_abs_unbounded_swe_residual_m: f64,
    pub sturm_density_form_fallback_used: bool,
    pub density_process_diagnostics: SnowDensityProcessDiagnostics,
    pub layers_after: Vec<DirectSnowLayerState>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SnowDensityProcessDiagnostics {
    pub applicable: bool,
    pub fresh_snow_density_available: bool,
    pub initial_density_kg_m3: f64,
    pub initial_snow_mass_kg_m2: f64,
    pub liquid_for_compaction_mass_kg_m2: f64,
    pub compaction_temperature_c: f64,
    pub snow_input_mass_kg_m2: f64,
    pub snow_input_depth_m: f64,
    pub fresh_snow_density_kg_m3: f64,
    pub fresh_snow_mixing_delta_kg_m3: f64,
    pub wet_compaction_delta_kg_m3: f64,
    pub destructive_metamorphism_delta_kg_m3: f64,
    pub overburden_compaction_delta_kg_m3: f64,
    pub structural_projection_delta_kg_m3: f64,
    pub climate_fallback_used: bool,
    pub climate_fallback_delta_kg_m3: f64,
    pub internal_cap_delta_kg_m3: f64,
    pub runtime_cap_delta_kg_m3: f64,
    pub downstream_stage3_delta_kg_m3: f64,
    pub final_density_kg_m3: f64,
    pub closure_residual_kg_m3: f64,
}

impl SnowDensityProcessDiagnostics {
    fn close_at_density(&mut self, final_density_kg_m3: f64) -> Result<(), SnowDensityError> {
        self.final_density_kg_m3 = final_density_kg_m3;
        self.closure_residual_kg_m3 = final_density_kg_m3
            - self.initial_density_kg_m3
            - self.fresh_snow_mixing_delta_kg_m3
            - self.wet_compaction_delta_kg_m3
            - self.destructive_metamorphism_delta_kg_m3
            - self.overburden_compaction_delta_kg_m3
            - self.structural_projection_delta_kg_m3
            - self.climate_fallback_delta_kg_m3
            - self.internal_cap_delta_kg_m3
            - self.runtime_cap_delta_kg_m3
            - self.downstream_stage3_delta_kg_m3;
        self.validate_closure()
    }

    fn validate_closure(&self) -> Result<(), SnowDensityError> {
        for (symbol, value) in [
            (
                "density_process_initial_density_kg_m3",
                self.initial_density_kg_m3,
            ),
            (
                "density_process_initial_snow_mass_kg_m2",
                self.initial_snow_mass_kg_m2,
            ),
            (
                "density_process_liquid_for_compaction_mass_kg_m2",
                self.liquid_for_compaction_mass_kg_m2,
            ),
            (
                "density_process_compaction_temperature_c",
                self.compaction_temperature_c,
            ),
            (
                "density_process_snow_input_mass_kg_m2",
                self.snow_input_mass_kg_m2,
            ),
            (
                "density_process_snow_input_depth_m",
                self.snow_input_depth_m,
            ),
            (
                "density_process_fresh_snow_density_kg_m3",
                self.fresh_snow_density_kg_m3,
            ),
            (
                "density_process_fresh_snow_mixing_delta_kg_m3",
                self.fresh_snow_mixing_delta_kg_m3,
            ),
            (
                "density_process_wet_compaction_delta_kg_m3",
                self.wet_compaction_delta_kg_m3,
            ),
            (
                "density_process_destructive_metamorphism_delta_kg_m3",
                self.destructive_metamorphism_delta_kg_m3,
            ),
            (
                "density_process_overburden_compaction_delta_kg_m3",
                self.overburden_compaction_delta_kg_m3,
            ),
            (
                "density_process_structural_projection_delta_kg_m3",
                self.structural_projection_delta_kg_m3,
            ),
            (
                "density_process_climate_fallback_delta_kg_m3",
                self.climate_fallback_delta_kg_m3,
            ),
            (
                "density_process_internal_cap_delta_kg_m3",
                self.internal_cap_delta_kg_m3,
            ),
            (
                "density_process_runtime_cap_delta_kg_m3",
                self.runtime_cap_delta_kg_m3,
            ),
            (
                "density_process_downstream_stage3_delta_kg_m3",
                self.downstream_stage3_delta_kg_m3,
            ),
            (
                "density_process_final_density_kg_m3",
                self.final_density_kg_m3,
            ),
            (
                "snow_density_process_closure_residual_kg_m3",
                self.closure_residual_kg_m3,
            ),
        ] {
            density_validate_finite(symbol, value)?;
        }
        if self.closure_residual_kg_m3.abs() > SNOW_DENSITY_DIAGNOSTIC_CLOSURE_TOLERANCE_KG_M3 {
            return Err(SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3: self.closure_residual_kg_m3,
                tolerance_kg_m3: SNOW_DENSITY_DIAGNOSTIC_CLOSURE_TOLERANCE_KG_M3,
            });
        }
        Ok(())
    }

    pub(crate) fn apply_downstream_stage3_density(
        &mut self,
        density_kg_m3: f64,
    ) -> Result<(), SnowDensityError> {
        if !self.applicable {
            return Ok(());
        }
        density_validate_finite("downstream_stage3_density_kg_m3", density_kg_m3)?;
        self.downstream_stage3_delta_kg_m3 = density_kg_m3 - self.final_density_kg_m3;
        self.close_at_density(density_kg_m3)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SnowDensityError {
    NonFiniteInput {
        symbol: &'static str,
        value: f64,
    },
    OutOfRangeInput {
        symbol: &'static str,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    MissingClimateClassAssignment {
        model: &'static str,
    },
    MissingSturmDayOfYear {
        model: &'static str,
    },
    MissingClimateClassDensityParameters {
        class: &'static str,
    },
    LayerAggregateMismatch {
        symbol: &'static str,
        value: f64,
        expected: f64,
    },
    DiagnosticClosureViolation {
        residual_kg_m3: f64,
        tolerance_kg_m3: f64,
    },
}

impl std::fmt::Display for SnowDensityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput { symbol, value } => {
                write!(f, "non-finite snow-density input {symbol}={value}")
            }
            Self::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "snow-density input {symbol}={value} outside [{minimum:?}, {maximum:?}]"
            ),
            Self::MissingClimateClassAssignment { model } => write!(
                f,
                "snow-density model {model} requires forcing-derived Sturm climate class"
            ),
            Self::MissingSturmDayOfYear { model } => write!(
                f,
                "snow-density model {model} requires Sturm density day-of-year"
            ),
            Self::MissingClimateClassDensityParameters { class } => write!(
                f,
                "missing Sturm 2010 density parameters for snow climate class {class}"
            ),
            Self::LayerAggregateMismatch {
                symbol,
                value,
                expected,
            } => write!(
                f,
                "snow-density layer aggregate {symbol}={value} does not match expected {expected}"
            ),
            Self::DiagnosticClosureViolation {
                residual_kg_m3,
                tolerance_kg_m3,
            } => write!(
                f,
                "snow-density diagnostic closure residual {residual_kg_m3} kg m^-3 exceeds {tolerance_kg_m3} kg m^-3"
            ),
        }
    }
}

impl std::error::Error for SnowDensityError {}

pub fn update_snow_density_runtime_state(
    inputs: &SnowDensityRuntimeInputs,
) -> Result<SnowDensityRuntimeOutcome, SnowDensityError> {
    validate_snow_density_runtime_inputs(inputs)?;

    if inputs.model == SnowDensityModel::LegacyWepp {
        let outcome = SnowDensityRuntimeOutcome {
            model: inputs.model,
            runtime_swe_after_m: inputs.boundary_swe_after_m,
            runtime_depth_after_m: inputs.boundary_depth_after_m,
            runtime_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
            coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
            max_abs_swe_identity_residual_m: 0.0,
            max_abs_unbounded_swe_residual_m: 0.0,
            sturm_density_form_fallback_used: false,
            density_process_diagnostics: SnowDensityProcessDiagnostics::default(),
            layers_after: Vec::new(),
        };
        return Ok(outcome);
    }

    if inputs.model == SnowDensityModel::PhysicsBulkMultilayerDensityV1 {
        let outcome = update_multilayer_snow_density_runtime_state(inputs)?;
        return Ok(outcome);
    }

    let constants = snow_density_constants_for_model(inputs.model);
    let (mut state, mut diagnostics) = apply_bulk_accumulation_and_compaction(inputs, constants)?;
    if !diagnostics.applicable {
        let outcome = snow_free_density_outcome(inputs);
        return Ok(outcome);
    }
    let density_before_fallback = state.density_kg_m3;
    let sturm_density_form_fallback_used = apply_sturm2010_density_form_fallback(
        &mut state,
        inputs.model,
        inputs.sturm_climate_class,
        inputs.sturm_day_of_year,
    )?;
    diagnostics.climate_fallback_used = sturm_density_form_fallback_used;
    diagnostics.climate_fallback_delta_kg_m3 = state.density_kg_m3 - density_before_fallback;

    let unbounded_swe_m = state.mass_kg_m2 / SNOW_DENSITY_RHO_WATER_KG_M3;
    let density_before_structure = state.density_kg_m3;
    state.mass_kg_m2 = inputs.boundary_swe_after_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    if state.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        state = CoeBoundDensityState::default();
    } else if state.density_kg_m3 <= 0.0 {
        state.density_kg_m3 = constants.new_snow_density_min_kg_m3;
    }
    diagnostics.structural_projection_delta_kg_m3 = state.density_kg_m3 - density_before_structure;
    let density_before_runtime_cap = state.density_kg_m3;
    state.density_kg_m3 = state.density_kg_m3.min(inputs.runtime_density_cap_kg_m3);
    diagnostics.runtime_cap_delta_kg_m3 = state.density_kg_m3 - density_before_runtime_cap;
    let runtime_swe_after_m = state.mass_kg_m2 / SNOW_DENSITY_RHO_WATER_KG_M3;
    let identity_residual_m = runtime_swe_after_m - inputs.boundary_swe_after_m;
    diagnostics.close_at_density(state.density_kg_m3)?;

    let outcome = SnowDensityRuntimeOutcome {
        model: inputs.model,
        runtime_swe_after_m,
        runtime_depth_after_m: state.depth_m(),
        runtime_density_after_kg_m3: state.density_kg_m3,
        coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
        coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
        max_abs_swe_identity_residual_m: identity_residual_m.abs(),
        max_abs_unbounded_swe_residual_m: (unbounded_swe_m - inputs.boundary_swe_after_m).abs(),
        sturm_density_form_fallback_used,
        density_process_diagnostics: diagnostics,
        layers_after: Vec::new(),
    };
    Ok(outcome)
}

fn snow_free_density_outcome(inputs: &SnowDensityRuntimeInputs) -> SnowDensityRuntimeOutcome {
    SnowDensityRuntimeOutcome {
        model: inputs.model,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
        coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
        max_abs_swe_identity_residual_m: inputs.boundary_swe_after_m.abs(),
        max_abs_unbounded_swe_residual_m: inputs.boundary_swe_after_m.abs(),
        sturm_density_form_fallback_used: false,
        density_process_diagnostics: SnowDensityProcessDiagnostics::default(),
        layers_after: Vec::new(),
    }
}

fn apply_bulk_accumulation_and_compaction(
    inputs: &SnowDensityRuntimeInputs,
    constants: SnowDensityCompactionConstants,
) -> Result<(CoeBoundDensityState, SnowDensityProcessDiagnostics), SnowDensityError> {
    let mut state = initialize_bulk_density_state(inputs, constants);
    let mut diagnostics = SnowDensityProcessDiagnostics {
        applicable: state.mass_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2
            || inputs.snow_input_m > 0.0
            || inputs.boundary_swe_after_m > 0.0,
        initial_density_kg_m3: state.density_kg_m3,
        initial_snow_mass_kg_m2: state.mass_kg_m2,
        liquid_for_compaction_mass_kg_m2: inputs.liquid_for_compaction_m
            * SNOW_DENSITY_RHO_WATER_KG_M3,
        compaction_temperature_c: inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        ..SnowDensityProcessDiagnostics::default()
    };
    if !diagnostics.applicable {
        return Ok((state, SnowDensityProcessDiagnostics::default()));
    }
    apply_bulk_fresh_snow(&mut state, &mut diagnostics, inputs, constants)?;
    let compaction = apply_daily_compaction(
        &mut state,
        inputs.liquid_for_compaction_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        constants,
    );
    apply_compaction_attribution(&mut diagnostics, compaction);
    Ok((state, diagnostics))
}

fn initialize_bulk_density_state(
    inputs: &SnowDensityRuntimeInputs,
    constants: SnowDensityCompactionConstants,
) -> CoeBoundDensityState {
    let mut state = CoeBoundDensityState {
        mass_kg_m2: inputs.prior_swe_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        density_kg_m3: inputs.prior_density_kg_m3,
    };
    if state.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        state = CoeBoundDensityState::default();
    } else if state.density_kg_m3 <= 0.0 && inputs.prior_depth_m > 0.0 {
        state.density_kg_m3 = state.mass_kg_m2 / inputs.prior_depth_m;
    } else if state.density_kg_m3 <= 0.0 {
        state.density_kg_m3 = constants.new_snow_density_min_kg_m3;
    }
    state
}

fn apply_bulk_fresh_snow(
    state: &mut CoeBoundDensityState,
    diagnostics: &mut SnowDensityProcessDiagnostics,
    inputs: &SnowDensityRuntimeInputs,
    constants: SnowDensityCompactionConstants,
) -> Result<(), SnowDensityError> {
    let snow_input_kg_m2 = inputs.snow_input_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    diagnostics.snow_input_mass_kg_m2 = snow_input_kg_m2;
    if snow_input_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        return Ok(());
    }
    diagnostics.fresh_snow_density_available = true;
    diagnostics.fresh_snow_density_kg_m3 =
        fresh_snow_density_kg_m3(inputs.mean_air_temperature_c, constants)?;
    diagnostics.snow_input_depth_m = snow_input_kg_m2 / diagnostics.fresh_snow_density_kg_m3;
    let density_before_fresh = state.density_kg_m3;
    add_fresh_snow(
        state,
        snow_input_kg_m2,
        inputs.mean_air_temperature_c,
        constants,
    )?;
    diagnostics.fresh_snow_mixing_delta_kg_m3 = state.density_kg_m3 - density_before_fresh;
    Ok(())
}

fn validate_snow_density_runtime_inputs(
    inputs: &SnowDensityRuntimeInputs,
) -> Result<(), SnowDensityError> {
    density_validate_nonnegative("prior_swe_m", inputs.prior_swe_m)?;
    density_validate_nonnegative("prior_depth_m", inputs.prior_depth_m)?;
    density_validate_nonnegative("prior_density_kg_m3", inputs.prior_density_kg_m3)?;
    density_validate_nonnegative("prior_settle_day_count", inputs.prior_settle_day_count)?;
    density_validate_nonnegative("boundary_swe_after_m", inputs.boundary_swe_after_m)?;
    density_validate_nonnegative("boundary_depth_after_m", inputs.boundary_depth_after_m)?;
    density_validate_nonnegative(
        "boundary_density_after_kg_m3",
        inputs.boundary_density_after_kg_m3,
    )?;
    density_validate_nonnegative("snow_input_m", inputs.snow_input_m)?;
    density_validate_nonnegative("liquid_for_compaction_m", inputs.liquid_for_compaction_m)?;
    density_validate_finite("mean_air_temperature_c", inputs.mean_air_temperature_c)?;
    density_validate_positive(
        "runtime_density_cap_kg_m3",
        inputs.runtime_density_cap_kg_m3,
    )
}

const fn snow_density_constants_for_model(
    model: SnowDensityModel,
) -> SnowDensityCompactionConstants {
    match model {
        SnowDensityModel::LegacyWepp
        | SnowDensityModel::PhysicsBulkDensityCompactionV1
        | SnowDensityModel::PhysicsBulkClimateClassDensityV1
        | SnowDensityModel::PhysicsBulkMultilayerDensityV1 => {
            snow_density_compaction_v1_constants()
        }
        SnowDensityModel::PhysicsBulkSpringDensificationV1 => {
            snow_density_spring_densification_v1_constants()
        }
        SnowDensityModel::PhysicsBulkShallowGuardV1 => snow_density_shallow_guard_v1_constants(),
    }
}

fn apply_sturm2010_density_form_fallback(
    state: &mut CoeBoundDensityState,
    model: SnowDensityModel,
    sturm_climate_class: Option<SnowClimateClass>,
    sturm_day_of_year: Option<f64>,
) -> Result<bool, SnowDensityError> {
    if model != SnowDensityModel::PhysicsBulkClimateClassDensityV1 {
        return Ok(false);
    }
    let class = sturm_climate_class
        .ok_or(SnowDensityError::MissingClimateClassAssignment { model: model.id() })?;
    if class == SnowClimateClass::Ephemeral {
        return Ok(false);
    }
    let day =
        sturm_day_of_year.ok_or(SnowDensityError::MissingSturmDayOfYear { model: model.id() })?;
    let target_density_kg_m3 = sturm2010_bulk_density_kg_m3(class, state.depth_m(), day)?;
    if state.mass_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        state.density_kg_m3 = target_density_kg_m3;
    }
    Ok(true)
}

fn update_multilayer_snow_density_runtime_state(
    inputs: &SnowDensityRuntimeInputs,
) -> Result<SnowDensityRuntimeOutcome, SnowDensityError> {
    let constants = snow_density_constants_for_model(inputs.model);
    let mut layers = initialize_multilayer_density_state(inputs, constants)?;
    let mut diagnostics = SnowDensityProcessDiagnostics {
        applicable: !layers.is_empty()
            || inputs.snow_input_m > 0.0
            || inputs.boundary_swe_after_m > 0.0,
        initial_density_kg_m3: multilayer_bulk_density_kg_m3(&layers),
        initial_snow_mass_kg_m2: multilayer_mass_kg_m2(&layers),
        liquid_for_compaction_mass_kg_m2: inputs.liquid_for_compaction_m
            * SNOW_DENSITY_RHO_WATER_KG_M3,
        compaction_temperature_c: inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        ..SnowDensityProcessDiagnostics::default()
    };
    if !diagnostics.applicable {
        return Ok(snow_free_density_outcome(inputs));
    }
    increment_multilayer_settle_clock(&mut layers);

    let snow_input_kg_m2 = inputs.snow_input_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    diagnostics.snow_input_mass_kg_m2 = snow_input_kg_m2;
    if snow_input_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        diagnostics.fresh_snow_density_available = true;
        diagnostics.fresh_snow_density_kg_m3 =
            fresh_snow_density_kg_m3(inputs.mean_air_temperature_c, constants)?;
        diagnostics.snow_input_depth_m = snow_input_kg_m2 / diagnostics.fresh_snow_density_kg_m3;
        let density_before_fresh = multilayer_bulk_density_kg_m3(&layers);
        layers.insert(
            0,
            SnowDensityLayerWorkState {
                mass_kg_m2: snow_input_kg_m2,
                density_kg_m3: diagnostics.fresh_snow_density_kg_m3,
                settle_day_count: 1.0,
                temperature_c: inputs.mean_air_temperature_c.min(0.0),
                liquid_water_m: 0.0,
                cold_content_j_m2: 0.0,
                refrozen_liquid_m: 0.0,
            },
        );
        diagnostics.fresh_snow_mixing_delta_kg_m3 =
            multilayer_bulk_density_kg_m3(&layers) - density_before_fresh;
    }

    let compaction = apply_multilayer_daily_compaction(
        &mut layers,
        inputs.liquid_for_compaction_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        constants,
    );
    apply_compaction_attribution(&mut diagnostics, compaction);
    let unbounded_swe_m = multilayer_mass_kg_m2(&layers) / SNOW_DENSITY_RHO_WATER_KG_M3;
    let density_before_structure = multilayer_bulk_density_kg_m3(&layers);
    apply_multilayer_boundary_mass(
        &mut layers,
        inputs.boundary_swe_after_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.boundary_density_after_kg_m3,
        constants,
    );
    merge_multilayer_bottom_layers(&mut layers);
    diagnostics.structural_projection_delta_kg_m3 =
        multilayer_bulk_density_kg_m3(&layers) - density_before_structure;
    let density_before_runtime_cap = multilayer_bulk_density_kg_m3(&layers);
    cap_multilayer_density(&mut layers, inputs.runtime_density_cap_kg_m3);
    diagnostics.runtime_cap_delta_kg_m3 =
        multilayer_bulk_density_kg_m3(&layers) - density_before_runtime_cap;

    let runtime_swe_after_m = multilayer_mass_kg_m2(&layers) / SNOW_DENSITY_RHO_WATER_KG_M3;
    let runtime_depth_after_m = multilayer_depth_m(&layers);
    let mut runtime_density_after_kg_m3 = if runtime_swe_after_m
        > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M
        && runtime_depth_after_m > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M
    {
        (runtime_swe_after_m * SNOW_DENSITY_RHO_WATER_KG_M3) / runtime_depth_after_m
    } else {
        0.0
    };
    runtime_density_after_kg_m3 = runtime_density_after_kg_m3.min(inputs.runtime_density_cap_kg_m3);
    let identity_residual_m = runtime_swe_after_m - inputs.boundary_swe_after_m;
    diagnostics.close_at_density(runtime_density_after_kg_m3)?;

    Ok(SnowDensityRuntimeOutcome {
        model: inputs.model,
        runtime_swe_after_m,
        runtime_depth_after_m,
        runtime_density_after_kg_m3,
        coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
        coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
        max_abs_swe_identity_residual_m: identity_residual_m.abs(),
        max_abs_unbounded_swe_residual_m: (unbounded_swe_m - inputs.boundary_swe_after_m).abs(),
        sturm_density_form_fallback_used: false,
        density_process_diagnostics: diagnostics,
        layers_after: layers
            .into_iter()
            .map(SnowDensityLayerWorkState::into_direct_state)
            .collect(),
    })
}

#[derive(Debug, Clone, Copy, Default)]
struct CoeBoundDensityState {
    mass_kg_m2: f64,
    density_kg_m3: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct SnowDensityLayerWorkState {
    mass_kg_m2: f64,
    density_kg_m3: f64,
    settle_day_count: f64,
    temperature_c: f64,
    liquid_water_m: f64,
    cold_content_j_m2: f64,
    refrozen_liquid_m: f64,
}

impl SnowDensityLayerWorkState {
    fn depth_m(self) -> f64 {
        if self.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 || self.density_kg_m3 <= 0.0 {
            0.0
        } else {
            self.mass_kg_m2 / self.density_kg_m3
        }
    }

    fn into_direct_state(self) -> DirectSnowLayerState {
        DirectSnowLayerState::new(
            self.mass_kg_m2 / SNOW_DENSITY_RHO_WATER_KG_M3,
            self.depth_m(),
            self.density_kg_m3,
            self.settle_day_count,
        )
        .with_stage3_thermal_liquid_state(
            self.temperature_c,
            self.liquid_water_m,
            self.cold_content_j_m2,
            self.refrozen_liquid_m,
        )
    }
}

fn initialize_multilayer_density_state(
    inputs: &SnowDensityRuntimeInputs,
    constants: SnowDensityCompactionConstants,
) -> Result<Vec<SnowDensityLayerWorkState>, SnowDensityError> {
    if !inputs.prior_layers.is_empty() {
        let mut layers = Vec::with_capacity(inputs.prior_layers.len());
        let mut swe_sum_m = 0.0;
        let mut depth_sum_m = 0.0;
        for layer in &inputs.prior_layers {
            density_validate_nonnegative("prior_layers.mass_swe_m", layer.mass_swe_m)?;
            density_validate_nonnegative("prior_layers.thickness_m", layer.thickness_m)?;
            density_validate_nonnegative("prior_layers.density_kg_m3", layer.density_kg_m3)?;
            density_validate_nonnegative("prior_layers.settle_day_count", layer.settle_day_count)?;
            if !snow_density_layer_has_resolved_mass(layer.mass_swe_m) {
                continue;
            }
            let density = if layer.density_kg_m3 > 0.0 {
                layer.density_kg_m3
            } else if layer.thickness_m > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M {
                layer.mass_swe_m * SNOW_DENSITY_RHO_WATER_KG_M3 / layer.thickness_m
            } else {
                constants.new_snow_density_min_kg_m3
            };
            swe_sum_m += layer.mass_swe_m;
            depth_sum_m += layer.thickness_m;
            layers.push(SnowDensityLayerWorkState {
                mass_kg_m2: layer.mass_swe_m * SNOW_DENSITY_RHO_WATER_KG_M3,
                density_kg_m3: density,
                settle_day_count: layer.settle_day_count,
                temperature_c: layer.temperature_c,
                liquid_water_m: layer.liquid_water_m,
                cold_content_j_m2: layer.cold_content_j_m2,
                refrozen_liquid_m: layer.refrozen_liquid_m,
            });
        }
        if (swe_sum_m - inputs.prior_swe_m).abs() > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M {
            return Err(SnowDensityError::LayerAggregateMismatch {
                symbol: "prior_layers.mass_swe_m",
                value: swe_sum_m,
                expected: inputs.prior_swe_m,
            });
        }
        if (depth_sum_m - inputs.prior_depth_m).abs() > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M {
            return Err(SnowDensityError::LayerAggregateMismatch {
                symbol: "prior_layers.thickness_m",
                value: depth_sum_m,
                expected: inputs.prior_depth_m,
            });
        }
        return Ok(layers);
    }

    if inputs.prior_swe_m <= SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M {
        return Ok(Vec::new());
    }
    let density = if inputs.prior_density_kg_m3 > 0.0 {
        inputs.prior_density_kg_m3
    } else if inputs.prior_depth_m > SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M {
        inputs.prior_swe_m * SNOW_DENSITY_RHO_WATER_KG_M3 / inputs.prior_depth_m
    } else {
        constants.new_snow_density_min_kg_m3
    };
    Ok(vec![SnowDensityLayerWorkState {
        mass_kg_m2: inputs.prior_swe_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        density_kg_m3: density,
        settle_day_count: inputs.prior_settle_day_count,
        temperature_c: 0.0,
        liquid_water_m: 0.0,
        cold_content_j_m2: 0.0,
        refrozen_liquid_m: 0.0,
    }])
}

fn increment_multilayer_settle_clock(layers: &mut [SnowDensityLayerWorkState]) {
    for layer in layers {
        layer.settle_day_count += 1.0;
    }
}

fn apply_multilayer_daily_compaction(
    layers: &mut [SnowDensityLayerWorkState],
    liquid_for_compaction_kg_m2: f64,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
) -> DailyCompactionDiagnostics {
    let mut diagnostics = DailyCompactionDiagnostics::default();
    if layers.is_empty() {
        return diagnostics;
    }
    let total_mass_kg_m2 = multilayer_mass_kg_m2(layers);
    if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2
        && total_mass_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2
    {
        for layer_index in 0..layers.len() {
            let liquid_for_layer_kg_m2 =
                liquid_for_compaction_kg_m2 * layers[layer_index].mass_kg_m2 / total_mass_kg_m2;
            let bulk_before = multilayer_bulk_density_kg_m3(layers);
            let density_before = layers[layer_index].density_kg_m3;
            let wet = {
                let layer = &mut layers[layer_index];
                let mut state = layer.as_coe_bound_state();
                let increment =
                    apply_wet_compaction(&mut state, liquid_for_layer_kg_m2, constants, 1.0);
                layer.density_kg_m3 = state.density_kg_m3;
                increment
            };
            let bulk_after = multilayer_bulk_density_kg_m3(layers);
            let bulk_uncapped = multilayer_bulk_density_with_layer_density(
                layers,
                layer_index,
                density_before + wet.destructive_raw,
            );
            diagnostics.wet_raw += bulk_uncapped - bulk_before;
            diagnostics.wet_applied += bulk_after - bulk_before;
        }
    }
    for _ in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
        let mut overburden_kg_m2 = 0.0;
        for layer_index in 0..layers.len() {
            let bulk_before = multilayer_bulk_density_kg_m3(layers);
            let density_before = layers[layer_index].density_kg_m3;
            let dry = {
                let layer = &mut layers[layer_index];
                let mut state = layer.as_coe_bound_state();
                let increment = apply_time_compaction_scaled_with_overburden(
                    &mut state,
                    overburden_kg_m2,
                    snow_temperature_c,
                    constants,
                    1.0,
                );
                layer.density_kg_m3 = state.density_kg_m3;
                increment
            };
            let bulk_after = multilayer_bulk_density_kg_m3(layers);
            let raw_total = dry.destructive_raw + dry.overburden_raw;
            let bulk_uncapped = multilayer_bulk_density_with_layer_density(
                layers,
                layer_index,
                density_before + raw_total,
            );
            diagnostics.accumulate_dry_bulk(dry, bulk_before, bulk_after, bulk_uncapped);
            overburden_kg_m2 += layers[layer_index].mass_kg_m2;
        }
    }
    diagnostics
}

impl SnowDensityLayerWorkState {
    fn as_coe_bound_state(self) -> CoeBoundDensityState {
        CoeBoundDensityState {
            mass_kg_m2: self.mass_kg_m2,
            density_kg_m3: self.density_kg_m3,
        }
    }
}

fn apply_multilayer_boundary_mass(
    layers: &mut Vec<SnowDensityLayerWorkState>,
    target_mass_kg_m2: f64,
    boundary_density_kg_m3: f64,
    constants: SnowDensityCompactionConstants,
) {
    if target_mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        layers.clear();
        return;
    }
    let mut current_mass_kg_m2 = multilayer_mass_kg_m2(layers);
    if current_mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        let density = if boundary_density_kg_m3 > 0.0 {
            boundary_density_kg_m3
        } else {
            constants.new_snow_density_min_kg_m3
        };
        layers.push(SnowDensityLayerWorkState {
            mass_kg_m2: target_mass_kg_m2,
            density_kg_m3: density,
            settle_day_count: 1.0,
            temperature_c: 0.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 0.0,
            refrozen_liquid_m: 0.0,
        });
        return;
    }

    if current_mass_kg_m2 > target_mass_kg_m2 {
        let mut mass_to_remove = current_mass_kg_m2 - target_mass_kg_m2;
        while mass_to_remove > SNOW_DENSITY_ZERO_MASS_KG_M2 && !layers.is_empty() {
            if layers[0].mass_kg_m2 <= mass_to_remove + SNOW_DENSITY_ZERO_MASS_KG_M2 {
                mass_to_remove -= layers[0].mass_kg_m2;
                layers.remove(0);
            } else {
                let old_mass = layers[0].mass_kg_m2;
                layers[0].mass_kg_m2 -= mass_to_remove;
                let scale = if old_mass > SNOW_DENSITY_ZERO_MASS_KG_M2 {
                    (layers[0].mass_kg_m2 / old_mass).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                layers[0].liquid_water_m *= scale;
                layers[0].cold_content_j_m2 *= scale;
                layers[0].refrozen_liquid_m *= scale;
                mass_to_remove = 0.0;
            }
        }
        return;
    }

    let mass_to_add = target_mass_kg_m2 - current_mass_kg_m2;
    if mass_to_add > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        if let Some(surface_layer) = layers.first_mut() {
            surface_layer.mass_kg_m2 += mass_to_add;
        } else {
            layers.push(SnowDensityLayerWorkState {
                mass_kg_m2: mass_to_add,
                density_kg_m3: constants.new_snow_density_min_kg_m3,
                settle_day_count: 1.0,
                temperature_c: 0.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 0.0,
                refrozen_liquid_m: 0.0,
            });
        }
    }
    current_mass_kg_m2 = multilayer_mass_kg_m2(layers);
    if current_mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        layers.clear();
    }
}

fn merge_multilayer_bottom_layers(layers: &mut Vec<SnowDensityLayerWorkState>) {
    while layers.len() > SNOW_DENSITY_MULTILAYER_MAX_LAYERS {
        let Some(bottom) = layers.pop() else {
            return;
        };
        let Some(above_bottom) = layers.last_mut() else {
            layers.push(bottom);
            return;
        };
        let mass = above_bottom.mass_kg_m2 + bottom.mass_kg_m2;
        let depth = above_bottom.depth_m() + bottom.depth_m();
        let settle = if mass > SNOW_DENSITY_ZERO_MASS_KG_M2 {
            (above_bottom.settle_day_count * above_bottom.mass_kg_m2
                + bottom.settle_day_count * bottom.mass_kg_m2)
                / mass
        } else {
            0.0
        };
        let temperature_c = if mass > SNOW_DENSITY_ZERO_MASS_KG_M2 {
            (above_bottom.temperature_c * above_bottom.mass_kg_m2
                + bottom.temperature_c * bottom.mass_kg_m2)
                / mass
        } else {
            0.0
        };
        let liquid_water_m = above_bottom.liquid_water_m + bottom.liquid_water_m;
        let cold_content_j_m2 = above_bottom.cold_content_j_m2 + bottom.cold_content_j_m2;
        let refrozen_liquid_m = above_bottom.refrozen_liquid_m + bottom.refrozen_liquid_m;
        above_bottom.mass_kg_m2 = mass;
        above_bottom.density_kg_m3 = if depth > 0.0 { mass / depth } else { 0.0 };
        above_bottom.settle_day_count = settle;
        above_bottom.temperature_c = temperature_c;
        above_bottom.liquid_water_m = liquid_water_m;
        above_bottom.cold_content_j_m2 = cold_content_j_m2;
        above_bottom.refrozen_liquid_m = refrozen_liquid_m;
    }
}

fn cap_multilayer_density(layers: &mut [SnowDensityLayerWorkState], density_cap_kg_m3: f64) {
    for layer in layers {
        layer.density_kg_m3 = layer.density_kg_m3.min(density_cap_kg_m3);
    }
}

fn multilayer_mass_kg_m2(layers: &[SnowDensityLayerWorkState]) -> f64 {
    layers.iter().map(|layer| layer.mass_kg_m2).sum()
}

fn multilayer_depth_m(layers: &[SnowDensityLayerWorkState]) -> f64 {
    layers.iter().map(|layer| layer.depth_m()).sum()
}

fn multilayer_bulk_density_kg_m3(layers: &[SnowDensityLayerWorkState]) -> f64 {
    let mass_kg_m2 = multilayer_mass_kg_m2(layers);
    let depth_m = multilayer_depth_m(layers);
    if mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2
        || depth_m <= SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M
    {
        0.0
    } else {
        mass_kg_m2 / depth_m
    }
}

fn multilayer_bulk_density_with_layer_density(
    layers: &[SnowDensityLayerWorkState],
    layer_index: usize,
    replacement_density_kg_m3: f64,
) -> f64 {
    let mass_kg_m2 = multilayer_mass_kg_m2(layers);
    let depth_m = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            if index == layer_index && replacement_density_kg_m3 > 0.0 {
                layer.mass_kg_m2 / replacement_density_kg_m3
            } else {
                layer.depth_m()
            }
        })
        .sum::<f64>();
    if mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2
        || depth_m <= SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M
    {
        0.0
    } else {
        mass_kg_m2 / depth_m
    }
}

fn add_fresh_snow(
    state: &mut CoeBoundDensityState,
    snow_input_kg_m2: f64,
    air_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
) -> Result<(), SnowDensityError> {
    let fresh_density = fresh_snow_density_kg_m3(air_temperature_c, constants)?;
    let new_depth_m = state.depth_m() + snow_input_kg_m2 / fresh_density;
    state.mass_kg_m2 += snow_input_kg_m2;
    state.density_kg_m3 = if new_depth_m > 0.0 {
        state.mass_kg_m2 / new_depth_m
    } else {
        0.0
    };
    Ok(())
}

fn fresh_snow_density_kg_m3(
    temp_air_c: f64,
    constants: SnowDensityCompactionConstants,
) -> Result<f64, SnowDensityError> {
    density_validate_finite("fresh_snow_air_temperature_c", temp_air_c)?;
    let density = if temp_air_c <= constants.new_snow_density_temperature_threshold_c {
        constants.new_snow_density_base_kg_m3
    } else {
        constants.new_snow_density_base_kg_m3
            + constants.new_snow_density_temperature_coefficient
                * (temp_air_c - constants.new_snow_density_temperature_threshold_c).powf(1.5)
    };
    Ok(density.clamp(
        constants.new_snow_density_min_kg_m3,
        constants.new_snow_density_max_kg_m3,
    ))
}

#[derive(Debug, Clone, Copy, Default)]
struct DensityCompactionIncrement {
    destructive_raw: f64,
    overburden_raw: f64,
    applied: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct DailyCompactionDiagnostics {
    wet_raw: f64,
    wet_applied: f64,
    destructive_raw: f64,
    overburden_raw: f64,
    dry_applied: f64,
}

impl DailyCompactionDiagnostics {
    fn accumulate_dry(&mut self, increment: DensityCompactionIncrement) {
        self.destructive_raw += increment.destructive_raw;
        self.overburden_raw += increment.overburden_raw;
        self.dry_applied += increment.applied;
    }

    fn accumulate_dry_bulk(
        &mut self,
        increment: DensityCompactionIncrement,
        bulk_before_kg_m3: f64,
        bulk_after_kg_m3: f64,
        bulk_uncapped_kg_m3: f64,
    ) {
        let raw_total = increment.destructive_raw + increment.overburden_raw;
        let uncapped_bulk_delta = bulk_uncapped_kg_m3 - bulk_before_kg_m3;
        if raw_total > 0.0 {
            self.destructive_raw += uncapped_bulk_delta * increment.destructive_raw / raw_total;
            self.overburden_raw += uncapped_bulk_delta * increment.overburden_raw / raw_total;
        }
        self.dry_applied += bulk_after_kg_m3 - bulk_before_kg_m3;
    }
}

fn apply_compaction_attribution(
    diagnostics: &mut SnowDensityProcessDiagnostics,
    compaction: DailyCompactionDiagnostics,
) {
    diagnostics.wet_compaction_delta_kg_m3 = compaction.wet_raw;
    diagnostics.destructive_metamorphism_delta_kg_m3 = compaction.destructive_raw;
    diagnostics.overburden_compaction_delta_kg_m3 = compaction.overburden_raw;
    diagnostics.internal_cap_delta_kg_m3 += compaction.wet_applied - compaction.wet_raw
        + compaction.dry_applied
        - compaction.destructive_raw
        - compaction.overburden_raw;
}

fn apply_daily_compaction(
    state: &mut CoeBoundDensityState,
    liquid_for_compaction_kg_m2: f64,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
) -> DailyCompactionDiagnostics {
    let mut diagnostics = DailyCompactionDiagnostics::default();
    let shallow_guard_factor = shallow_compaction_guard_factor(state.depth_m(), constants);
    let wet_substeps = constants.wet_compaction_substeps_per_day.max(1);
    if wet_substeps == 1 {
        if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
            let wet = apply_wet_compaction(
                state,
                liquid_for_compaction_kg_m2,
                constants,
                shallow_guard_factor,
            );
            diagnostics.wet_raw += wet.destructive_raw;
            diagnostics.wet_applied += wet.applied;
        }
        for _ in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
            let dry = apply_time_compaction_scaled_with_overburden(
                state,
                state.mass_kg_m2,
                snow_temperature_c,
                constants,
                shallow_guard_factor,
            );
            diagnostics.accumulate_dry(dry);
        }
        return diagnostics;
    }

    let liquid_per_step = liquid_for_compaction_kg_m2 / f64::from(wet_substeps);
    if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        let wet = apply_wet_compaction(
            state,
            liquid_for_compaction_kg_m2,
            constants,
            shallow_guard_factor,
        );
        diagnostics.wet_raw += wet.destructive_raw;
        diagnostics.wet_applied += wet.applied;
    }
    for step in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
        let wet_step = step < wet_substeps && liquid_per_step > SNOW_DENSITY_ZERO_MASS_KG_M2;
        let dry = apply_time_compaction_scaled(
            state,
            snow_temperature_c,
            constants,
            if wet_step {
                constants.wet_compaction_multiplier
            } else {
                1.0
            } * shallow_guard_factor,
        );
        diagnostics.accumulate_dry(dry);
    }
    diagnostics
}

fn shallow_compaction_guard_factor(
    pre_compaction_depth_m: f64,
    constants: SnowDensityCompactionConstants,
) -> f64 {
    let threshold_m = constants.shallow_compaction_guard_depth_threshold_m;
    if threshold_m <= 0.0 || pre_compaction_depth_m >= threshold_m {
        return 1.0;
    }
    (pre_compaction_depth_m / threshold_m).clamp(0.0, 1.0)
}

fn apply_time_compaction_scaled(
    state: &mut CoeBoundDensityState,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
    multiplier_scale: f64,
) -> DensityCompactionIncrement {
    apply_time_compaction_scaled_with_overburden(
        state,
        state.mass_kg_m2,
        snow_temperature_c,
        constants,
        multiplier_scale,
    )
}

fn apply_time_compaction_scaled_with_overburden(
    state: &mut CoeBoundDensityState,
    overburden_kg_m2: f64,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
    multiplier_scale: f64,
) -> DensityCompactionIncrement {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return DensityCompactionIncrement::default();
    }
    let overburden = overburden_kg_m2.max(0.0);
    let rate = if overburden >= constants.dry_compaction_swe_max_kg_m2 {
        1.0
    } else {
        constants.compaction_rate_cos_amplitude
            * (std::f64::consts::PI * overburden / constants.dry_compaction_swe_max_kg_m2).cos()
            + constants.compaction_rate_offset
    };
    let c11 = if density < constants.ptm_density_threshold_kg_m3 {
        1.0
    } else {
        (-constants.ptm_density_decay_m3_per_kg * (density - constants.ptm_density_threshold_kg_m3))
            .exp()
    };
    let freeze_minus_snow_temp = -snow_temperature_c.min(0.0);
    let destructive_metamorphism = constants.ptm_rate_per_hour
        * c11
        * (-constants.ptm_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        / rate;
    let overburden_compaction = constants.poc_rate_per_hour
        * (-constants.poc_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        * overburden
        * (-constants.poc_density_decay * (density / SNOW_DENSITY_RHO_WATER_KG_M3)).exp()
        / rate;
    let destructive_raw_delta =
        constants.dry_compaction_multiplier * multiplier_scale * destructive_metamorphism * density;
    let overburden_raw_delta =
        constants.dry_compaction_multiplier * multiplier_scale * overburden_compaction * density;
    let combined_raw_delta = constants.dry_compaction_multiplier
        * multiplier_scale
        * (destructive_metamorphism + overburden_compaction)
        * density;
    state.density_kg_m3 =
        (density + combined_raw_delta).min(constants.dry_compaction_max_density_kg_m3);
    DensityCompactionIncrement {
        destructive_raw: destructive_raw_delta,
        overburden_raw: overburden_raw_delta,
        applied: state.density_kg_m3 - density,
    }
}

fn apply_wet_compaction(
    state: &mut CoeBoundDensityState,
    liquid_added_kg_m2: f64,
    constants: SnowDensityCompactionConstants,
    shallow_guard_factor: f64,
) -> DensityCompactionIncrement {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.wet_compaction_max_density_kg_m3 {
        return DensityCompactionIncrement::default();
    }
    if state.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        return DensityCompactionIncrement::default();
    }
    let h2o_added_ratio = liquid_added_kg_m2 / state.mass_kg_m2;
    if h2o_added_ratio <= 1.0e-6 {
        return DensityCompactionIncrement::default();
    }
    let density_delta = constants.wet_compaction_multiplier
        * shallow_guard_factor
        * (constants.wet_compaction_max_density_kg_m3 - density)
        / (1.0 + constants.wet_compaction_half_saturation_ratio / h2o_added_ratio);
    state.density_kg_m3 = (density + density_delta).min(constants.wet_compaction_max_density_kg_m3);
    DensityCompactionIncrement {
        destructive_raw: density_delta,
        overburden_raw: 0.0,
        applied: state.density_kg_m3 - density,
    }
}

impl CoeBoundDensityState {
    fn depth_m(self) -> f64 {
        if self.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 || self.density_kg_m3 <= 0.0 {
            0.0
        } else {
            self.mass_kg_m2 / self.density_kg_m3
        }
    }

    fn observed_density_kg_m3(self) -> f64 {
        let depth = self.depth_m();
        if depth <= 0.0 {
            0.0
        } else {
            self.mass_kg_m2 / depth
        }
    }
}

fn density_validate_finite(symbol: &'static str, value: f64) -> Result<(), SnowDensityError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SnowDensityError::NonFiniteInput { symbol, value })
    }
}

fn density_validate_nonnegative(symbol: &'static str, value: f64) -> Result<(), SnowDensityError> {
    density_validate_finite(symbol, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(SnowDensityError::OutOfRangeInput {
            symbol,
            value,
            minimum: Some(0.0),
            maximum: None,
        })
    }
}

fn density_validate_positive(symbol: &'static str, value: f64) -> Result<(), SnowDensityError> {
    density_validate_finite(symbol, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(SnowDensityError::OutOfRangeInput {
            symbol,
            value,
            minimum: Some(f64::MIN_POSITIVE),
            maximum: None,
        })
    }
}

#[cfg(test)]
mod cqr_row5_snow_density_tests {
    use super::*;

    fn layer(mass_kg_m2: f64, density_kg_m3: f64) -> SnowDensityLayerWorkState {
        SnowDensityLayerWorkState {
            mass_kg_m2,
            density_kg_m3,
            settle_day_count: 3.0,
            temperature_c: -2.0,
            liquid_water_m: 0.02,
            cold_content_j_m2: 50.0,
            refrozen_liquid_m: 0.01,
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "actual={actual} expected={expected}"
        );
    }

    fn density_inputs(model: SnowDensityModel) -> SnowDensityRuntimeInputs {
        SnowDensityRuntimeInputs {
            model,
            prior_swe_m: 0.12,
            prior_depth_m: 0.6,
            prior_density_kg_m3: 200.0,
            prior_settle_day_count: 8.0,
            prior_layers: Vec::new(),
            boundary_swe_after_m: 0.14,
            boundary_depth_after_m: 0.7,
            boundary_density_after_kg_m3: 200.0,
            snow_input_m: 0.02,
            liquid_for_compaction_m: 0.004,
            mean_air_temperature_c: -5.0,
            runtime_density_cap_kg_m3: 522.0,
            sturm_climate_class: None,
            sturm_day_of_year: None,
        }
    }

    #[test]
    fn eb04v_isolated_fresh_and_wet_vectors_match_independent_equations() {
        let constants = snow_density_compaction_v1_constants();
        let mut fresh_state = CoeBoundDensityState {
            mass_kg_m2: 100.0,
            density_kg_m3: 200.0,
        };
        let air_temperature_c: f64 = -5.0;
        let fresh_density = 75.0 + 1.7 * (air_temperature_c + 15.0).powf(1.5);
        let expected_mixed_density = 120.0 / (100.0 / 200.0 + 20.0 / fresh_density);
        add_fresh_snow(&mut fresh_state, 20.0, air_temperature_c, constants)
            .unwrap_or_else(|error| panic!("fresh-snow update failed: {error}"));
        assert_close(fresh_state.density_kg_m3, expected_mixed_density);

        let mut wet_state = CoeBoundDensityState {
            mass_kg_m2: 100.0,
            density_kg_m3: 200.0,
        };
        let expected_wet_delta = 2.0 * (550.0 - 200.0) / (1.0 + 0.4 / 0.1);
        let wet = apply_wet_compaction(&mut wet_state, 10.0, constants, 1.0);
        assert_close(wet.destructive_raw, expected_wet_delta);
        assert_close(wet.applied, expected_wet_delta);
        assert_close(wet_state.density_kg_m3, 200.0 + expected_wet_delta);
    }

    #[test]
    fn eb04v_isolated_dry_vectors_separate_ptm_and_poc() {
        let mut destructive_constants = snow_density_compaction_v1_constants();
        destructive_constants.poc_rate_per_hour = 0.0;
        let mut destructive_state = CoeBoundDensityState {
            mass_kg_m2: 100.0,
            density_kg_m3: 200.0,
        };
        let destructive = apply_time_compaction_scaled_with_overburden(
            &mut destructive_state,
            100.0,
            -5.0,
            destructive_constants,
            1.0,
        );
        assert!(destructive.destructive_raw > 0.0);
        assert_close(destructive.overburden_raw, 0.0);
        assert_close(destructive.applied, destructive.destructive_raw);

        let mut overburden_constants = snow_density_compaction_v1_constants();
        overburden_constants.ptm_rate_per_hour = 0.0;
        let mut overburden_state = CoeBoundDensityState {
            mass_kg_m2: 100.0,
            density_kg_m3: 200.0,
        };
        let overburden = apply_time_compaction_scaled_with_overburden(
            &mut overburden_state,
            100.0,
            -5.0,
            overburden_constants,
            1.0,
        );
        assert_close(overburden.destructive_raw, 0.0);
        assert!(overburden.overburden_raw > 0.0);
        assert_close(overburden.applied, overburden.overburden_raw);
    }

    #[test]
    fn eb04v_multilayer_internal_cap_is_exact_in_bulk_density_space() {
        let mut constants = snow_density_compaction_v1_constants();
        constants.ptm_rate_per_hour = 0.0;
        constants.poc_rate_per_hour = 0.0;
        let mut layers = vec![layer(25.0, 500.0), layer(75.0, 450.0)];
        let initial = multilayer_bulk_density_kg_m3(&layers);
        let compaction = apply_multilayer_daily_compaction(&mut layers, 100.0, -1.0, constants);
        let final_density = multilayer_bulk_density_kg_m3(&layers);

        let bulk_density = |surface_density: f64, lower_density: f64| {
            100.0 / (25.0 / surface_density + 75.0 / lower_density)
        };
        let wet_ratio = 1.0;
        let surface_raw_density = 500.0
            + constants.wet_compaction_multiplier
                * (constants.wet_compaction_max_density_kg_m3 - 500.0)
                / (1.0 + constants.wet_compaction_half_saturation_ratio / wet_ratio);
        let lower_raw_density = 450.0
            + constants.wet_compaction_multiplier
                * (constants.wet_compaction_max_density_kg_m3 - 450.0)
                / (1.0 + constants.wet_compaction_half_saturation_ratio / wet_ratio);
        let after_surface_cap = bulk_density(constants.wet_compaction_max_density_kg_m3, 450.0);
        let expected_uncapped_wet_delta = bulk_density(surface_raw_density, 450.0) - initial
            + bulk_density(
                constants.wet_compaction_max_density_kg_m3,
                lower_raw_density,
            )
            - after_surface_cap;
        let expected_final = bulk_density(
            constants.wet_compaction_max_density_kg_m3,
            constants.wet_compaction_max_density_kg_m3,
        );
        let expected_internal_cap = expected_final - initial - expected_uncapped_wet_delta;

        let mut ledger = SnowDensityProcessDiagnostics::default();
        apply_compaction_attribution(&mut ledger, compaction);
        let reconstructed_delta = ledger.wet_compaction_delta_kg_m3
            + ledger.destructive_metamorphism_delta_kg_m3
            + ledger.overburden_compaction_delta_kg_m3
            + ledger.internal_cap_delta_kg_m3;
        assert!(ledger.internal_cap_delta_kg_m3 < 0.0);
        assert!((final_density - initial - reconstructed_delta).abs() <= 1.0e-9);
        assert_close(
            ledger.wet_compaction_delta_kg_m3,
            expected_uncapped_wet_delta,
        );
        assert_close(final_density, expected_final);
        assert_close(ledger.internal_cap_delta_kg_m3, expected_internal_cap);
        assert_close(ledger.destructive_metamorphism_delta_kg_m3, 0.0);
        assert_close(ledger.overburden_compaction_delta_kg_m3, 0.0);
    }

    #[test]
    fn eb04v_isolated_structural_fallback_and_stage3_vectors_are_explicit() {
        let mut structural = density_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1);
        structural.prior_swe_m = 0.275;
        structural.prior_depth_m = 0.5;
        structural.prior_density_kg_m3 = 550.0;
        structural.boundary_swe_after_m = 0.0;
        structural.snow_input_m = 0.0;
        structural.liquid_for_compaction_m = 0.0;
        let structural_outcome = update_snow_density_runtime_state(&structural)
            .unwrap_or_else(|error| panic!("structural update failed: {error}"));
        let structural_ledger = structural_outcome.density_process_diagnostics;
        assert_close(structural_ledger.structural_projection_delta_kg_m3, -550.0);
        assert_close(structural_ledger.final_density_kg_m3, 0.0);

        let mut fallback = density_inputs(SnowDensityModel::PhysicsBulkClimateClassDensityV1);
        fallback.snow_input_m = 0.0;
        fallback.liquid_for_compaction_m = 0.0;
        fallback.sturm_climate_class = Some(SnowClimateClass::Alpine);
        fallback.sturm_day_of_year = Some(120.0);
        let fallback_outcome = update_snow_density_runtime_state(&fallback)
            .unwrap_or_else(|error| panic!("fallback update failed: {error}"));
        assert!(
            fallback_outcome
                .density_process_diagnostics
                .climate_fallback_used
        );
        assert!(
            fallback_outcome
                .density_process_diagnostics
                .climate_fallback_delta_kg_m3
                .abs()
                > 1.0e-9
        );

        let mut stage3 = SnowDensityProcessDiagnostics {
            applicable: true,
            initial_density_kg_m3: 100.0,
            fresh_snow_mixing_delta_kg_m3: 30.0,
            ..SnowDensityProcessDiagnostics::default()
        };
        stage3
            .close_at_density(130.0)
            .unwrap_or_else(|error| panic!("initial close failed: {error}"));
        stage3
            .apply_downstream_stage3_density(120.0)
            .unwrap_or_else(|error| panic!("Stage-3 close failed: {error}"));
        assert_close(stage3.downstream_stage3_delta_kg_m3, -10.0);
        assert_close(stage3.final_density_kg_m3, 120.0);
    }

    #[test]
    fn eb04v_inapplicable_and_invalid_ledgers_fail_closed_without_aliasing() {
        let legacy =
            update_snow_density_runtime_state(&density_inputs(SnowDensityModel::LegacyWepp))
                .unwrap_or_else(|error| panic!("legacy update failed: {error}"));
        let mut legacy_ledger = legacy.density_process_diagnostics;
        legacy_ledger
            .apply_downstream_stage3_density(300.0)
            .unwrap_or_else(|error| panic!("legacy Stage-3 adjustment failed: {error}"));
        assert_eq!(legacy_ledger, SnowDensityProcessDiagnostics::default());

        let invalid = SnowDensityProcessDiagnostics {
            applicable: true,
            closure_residual_kg_m3: 2.0e-9,
            ..SnowDensityProcessDiagnostics::default()
        };
        assert!(matches!(
            invalid.validate_closure(),
            Err(SnowDensityError::DiagnosticClosureViolation { .. })
        ));

        let mut nonfinite = SnowDensityProcessDiagnostics {
            applicable: true,
            ..SnowDensityProcessDiagnostics::default()
        };
        assert!(matches!(
            nonfinite.apply_downstream_stage3_density(f64::NAN),
            Err(SnowDensityError::NonFiniteInput { .. })
        ));

        let nonfinite_driver = SnowDensityProcessDiagnostics {
            applicable: true,
            liquid_for_compaction_mass_kg_m2: f64::INFINITY,
            ..SnowDensityProcessDiagnostics::default()
        };
        assert!(matches!(
            nonfinite_driver.validate_closure(),
            Err(SnowDensityError::NonFiniteInput {
                symbol: "density_process_liquid_for_compaction_mass_kg_m2",
                ..
            })
        ));

        let mut overflow = density_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1);
        overflow.liquid_for_compaction_m = 1.0e308;
        assert!(matches!(
            update_snow_density_runtime_state(&overflow),
            Err(SnowDensityError::NonFiniteInput {
                symbol: "density_process_liquid_for_compaction_mass_kg_m2",
                ..
            })
        ));
    }

    #[test]
    fn eb04v_omitted_process_aliases_fail_the_closure_tolerance() {
        let increments = [3.0, 5.0, 7.0, 11.0, -2.0, 13.0, -17.0, -19.0, 23.0];
        let final_density = 100.0 + increments.iter().sum::<f64>();
        for omitted in 0..increments.len() {
            let wrong_residual = final_density
                - 100.0
                - increments
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != omitted)
                    .map(|(_, value)| value)
                    .sum::<f64>();
            assert!(wrong_residual.abs() > SNOW_DENSITY_DIAGNOSTIC_CLOSURE_TOLERANCE_KG_M3);
        }
    }

    #[test]
    fn eb04v_bulk_ledger_closes_and_keeps_fresh_density_direct() {
        let outcome = update_snow_density_runtime_state(&density_inputs(
            SnowDensityModel::PhysicsBulkDensityCompactionV1,
        ))
        .unwrap_or_else(|error| panic!("density update failed: {error}"));
        let ledger = outcome.density_process_diagnostics;

        assert!(ledger.applicable);
        assert!(ledger.fresh_snow_density_available);
        assert!(ledger.snow_input_depth_m > 0.0);
        assert!(ledger.wet_compaction_delta_kg_m3 > 0.0);
        assert!(ledger.destructive_metamorphism_delta_kg_m3 > 0.0);
        assert!(ledger.overburden_compaction_delta_kg_m3 > 0.0);
        assert!((ledger.fresh_snow_density_kg_m3 - ledger.final_density_kg_m3).abs() > 1.0e-6);
        assert!(ledger.closure_residual_kg_m3.abs() <= 1.0e-9);
    }

    #[test]
    fn eb04v_multilayer_ledger_separates_surface_metamorphism_and_loaded_overburden() {
        let mut inputs = density_inputs(SnowDensityModel::PhysicsBulkMultilayerDensityV1);
        inputs.prior_layers = vec![
            DirectSnowLayerState::new(0.04, 0.25, 160.0, 2.0),
            DirectSnowLayerState::new(0.08, 0.32, 250.0, 12.0),
        ];
        inputs.prior_depth_m = 0.57;
        let outcome = update_snow_density_runtime_state(&inputs)
            .unwrap_or_else(|error| panic!("density update failed: {error}"));
        let ledger = outcome.density_process_diagnostics;

        assert!(ledger.destructive_metamorphism_delta_kg_m3 > 0.0);
        assert!(ledger.overburden_compaction_delta_kg_m3 > 0.0);
        assert!(ledger.wet_compaction_delta_kg_m3 > 0.0);
        assert!(ledger.closure_residual_kg_m3.abs() <= 1.0e-9);
    }

    #[test]
    fn eb04v_runtime_cap_and_snow_free_inapplicability_are_explicit() {
        let mut capped = density_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1);
        capped.prior_swe_m = 0.27;
        capped.prior_depth_m = 0.5;
        capped.prior_density_kg_m3 = 540.0;
        capped.boundary_swe_after_m = 0.27;
        capped.snow_input_m = 0.0;
        capped.liquid_for_compaction_m = 0.0;
        let capped_outcome = update_snow_density_runtime_state(&capped)
            .unwrap_or_else(|error| panic!("density update failed: {error}"));
        assert!(
            capped_outcome
                .density_process_diagnostics
                .runtime_cap_delta_kg_m3
                < 0.0
        );
        assert!(
            capped_outcome
                .density_process_diagnostics
                .closure_residual_kg_m3
                .abs()
                <= 1.0e-9
        );

        let mut empty = density_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1);
        empty.prior_swe_m = 0.0;
        empty.prior_depth_m = 0.0;
        empty.prior_density_kg_m3 = 0.0;
        empty.boundary_swe_after_m = 0.0;
        empty.boundary_depth_after_m = 0.0;
        empty.boundary_density_after_kg_m3 = 0.0;
        empty.snow_input_m = 0.0;
        empty.liquid_for_compaction_m = 0.0;
        let empty_outcome = update_snow_density_runtime_state(&empty)
            .unwrap_or_else(|error| panic!("density update failed: {error}"));
        assert!(!empty_outcome.density_process_diagnostics.applicable);
        assert!(
            !empty_outcome
                .density_process_diagnostics
                .fresh_snow_density_available
        );
        assert_eq!(
            empty_outcome.density_process_diagnostics,
            SnowDensityProcessDiagnostics::default()
        );

        empty.model = SnowDensityModel::PhysicsBulkClimateClassDensityV1;
        empty.sturm_climate_class = None;
        empty.sturm_day_of_year = None;
        let empty_climate_outcome = update_snow_density_runtime_state(&empty)
            .unwrap_or_else(|error| panic!("snow-free climate update failed: {error}"));
        assert!(!empty_climate_outcome.sturm_density_form_fallback_used);
        assert_eq!(
            empty_climate_outcome.density_process_diagnostics,
            SnowDensityProcessDiagnostics::default()
        );
    }

    #[test]
    fn eb04d_layer_lifecycle_uses_mass_units_not_meter_residual_tolerance() {
        let mass_boundary_swe_m = SNOW_DENSITY_ZERO_MASS_KG_M2 / SNOW_DENSITY_RHO_WATER_KG_M3;
        let just_below = f64::from_bits(mass_boundary_swe_m.to_bits() - 1);
        let just_above = f64::from_bits(mass_boundary_swe_m.to_bits() + 1);

        assert!(!snow_density_layer_has_resolved_mass(just_below));
        assert!(!snow_density_layer_has_resolved_mass(mass_boundary_swe_m));
        assert!(snow_density_layer_has_resolved_mass(just_above));
        let captured_fragment_swe_m = 5.260_584_353_128_359e-10;
        assert!(snow_density_layer_has_resolved_mass(
            captured_fragment_swe_m
        ));
        assert!(captured_fragment_swe_m <= SNOW_DENSITY_LAYER_CLOSURE_TOLERANCE_M);
    }

    #[test]
    fn multilayer_boundary_mass_covers_clear_create_trim_and_add_paths() {
        let constants = snow_density_compaction_v1_constants();

        let mut layers = vec![layer(10.0, 100.0)];
        apply_multilayer_boundary_mass(&mut layers, 0.0, 0.0, constants);
        assert!(layers.is_empty());

        apply_multilayer_boundary_mass(&mut layers, 12.0, 180.0, constants);
        assert_eq!(layers.len(), 1);
        assert_close(layers[0].mass_kg_m2, 12.0);
        assert_close(layers[0].density_kg_m3, 180.0);

        layers.clear();
        apply_multilayer_boundary_mass(&mut layers, 8.0, 0.0, constants);
        assert_close(
            layers[0].density_kg_m3,
            constants.new_snow_density_min_kg_m3,
        );

        layers = vec![layer(10.0, 100.0), layer(10.0, 200.0)];
        apply_multilayer_boundary_mass(&mut layers, 15.0, 0.0, constants);
        assert_eq!(layers.len(), 2);
        assert!((layers[0].mass_kg_m2 - 5.0).abs() <= 1.0e-12);
        assert!((layers[0].liquid_water_m - 0.01).abs() <= 1.0e-12);
        assert!((layers[0].cold_content_j_m2 - 25.0).abs() <= 1.0e-12);
        assert!((layers[0].refrozen_liquid_m - 0.005).abs() <= 1.0e-12);

        apply_multilayer_boundary_mass(&mut layers, 5.0, 0.0, constants);
        assert_eq!(layers.len(), 1);
        assert!((multilayer_mass_kg_m2(&layers) - 5.0).abs() <= 1.0e-12);

        apply_multilayer_boundary_mass(&mut layers, 20.0, 0.0, constants);
        assert_eq!(layers.len(), 1);
        assert!((layers[0].mass_kg_m2 - 20.0).abs() <= 1.0e-12);
    }

}
