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
const SNOW_DENSITY_MULTILAYER_MAX_LAYERS: usize = 16;
pub const STURM1995_CDM_CRITICAL_TEMPERATURE_C: f64 = 10.0;
pub const STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH: f64 = 30.0;
pub const STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH: f64 = 125.0;
pub const STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY: f64 = 2.0;
pub const STURM1995_LOW_WIND_MAX_M_S: f64 = 0.5;
pub const STURM1995_HIGH_WIND_MIN_M_S: f64 = 2.0;

fn snow_density_layer_has_resolved_mass(mass_swe_m: f64) -> bool {
    openwepp_unit_boundary::conversions::snow_water_equivalent_meters_to_area_mass_kg_m2(
        mass_swe_m,
    ) > SNOW_DENSITY_ZERO_MASS_KG_M2
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
            Self::PhysicsBulkSpringDensificationV1 => {
                SNOW_DENSITY_SPRING_DENSIFICATION_MODEL_ID
            }
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
    validate_sturm1995_normal(
        "cooling_degree_month_c",
        normals.cooling_degree_month_c,
    )?;
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

    match (low_temperature, sturm1995_wind_branch(normals.winter_wind_m_s)?) {
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
        Err(Sturm1995ClimateClassAssignmentError::AmbiguousWindThreshold {
            wind_m_s,
            low_max_m_s: STURM1995_LOW_WIND_MAX_M_S,
            high_min_m_s: STURM1995_HIGH_WIND_MIN_M_S,
        })
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
    let parameters = sturm2010_density_parameters_for_class(class).ok_or(
        SnowDensityError::MissingClimateClassDensityParameters { class: class.id() },
    )?;
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
    pub layers_after: Vec<DirectSnowLayerState>,
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
        }
    }
}

impl std::error::Error for SnowDensityError {}

pub fn update_snow_density_runtime_state(
    inputs: &SnowDensityRuntimeInputs,
) -> Result<SnowDensityRuntimeOutcome, SnowDensityError> {
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
    density_validate_positive("runtime_density_cap_kg_m3", inputs.runtime_density_cap_kg_m3)?;

    if inputs.model == SnowDensityModel::LegacyWepp {
        return Ok(SnowDensityRuntimeOutcome {
            model: inputs.model,
            runtime_swe_after_m: inputs.boundary_swe_after_m,
            runtime_depth_after_m: inputs.boundary_depth_after_m,
            runtime_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
            coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
            max_abs_swe_identity_residual_m: 0.0,
            max_abs_unbounded_swe_residual_m: 0.0,
            sturm_density_form_fallback_used: false,
            layers_after: Vec::new(),
        });
    }

    if inputs.model == SnowDensityModel::PhysicsBulkMultilayerDensityV1 {
        return update_multilayer_snow_density_runtime_state(inputs);
    }

    let constants = snow_density_constants_for_model(inputs.model);
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

    let snow_input_kg_m2 = inputs.snow_input_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    if snow_input_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        add_fresh_snow(
            &mut state,
            snow_input_kg_m2,
            inputs.mean_air_temperature_c,
            constants,
        )?;
    }

    apply_daily_compaction(
        &mut state,
        inputs.liquid_for_compaction_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        constants,
    );
    let sturm_density_form_fallback_used = apply_sturm2010_density_form_fallback(
        &mut state,
        inputs.model,
        inputs.sturm_climate_class,
        inputs.sturm_day_of_year,
    )?;

    let unbounded_swe_m = state.mass_kg_m2 / SNOW_DENSITY_RHO_WATER_KG_M3;
    state.mass_kg_m2 = inputs.boundary_swe_after_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    if state.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        state = CoeBoundDensityState::default();
    } else if state.density_kg_m3 <= 0.0 {
        state.density_kg_m3 = constants.new_snow_density_min_kg_m3;
    }
    state.density_kg_m3 = state.density_kg_m3.min(inputs.runtime_density_cap_kg_m3);
    let runtime_swe_after_m = state.mass_kg_m2 / SNOW_DENSITY_RHO_WATER_KG_M3;
    let identity_residual_m = runtime_swe_after_m - inputs.boundary_swe_after_m;

    Ok(SnowDensityRuntimeOutcome {
        model: inputs.model,
        runtime_swe_after_m,
        runtime_depth_after_m: state.depth_m(),
        runtime_density_after_kg_m3: state.density_kg_m3,
        coe_boundary_depth_after_m: inputs.boundary_depth_after_m,
        coe_boundary_density_after_kg_m3: inputs.boundary_density_after_kg_m3,
        max_abs_swe_identity_residual_m: identity_residual_m.abs(),
        max_abs_unbounded_swe_residual_m: (unbounded_swe_m - inputs.boundary_swe_after_m).abs(),
        sturm_density_form_fallback_used,
        layers_after: Vec::new(),
    })
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
    let class = sturm_climate_class.ok_or(SnowDensityError::MissingClimateClassAssignment {
        model: model.id(),
    })?;
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
    increment_multilayer_settle_clock(&mut layers);

    let snow_input_kg_m2 = inputs.snow_input_m * SNOW_DENSITY_RHO_WATER_KG_M3;
    if snow_input_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        layers.insert(
            0,
            SnowDensityLayerWorkState {
                mass_kg_m2: snow_input_kg_m2,
                density_kg_m3: fresh_snow_density_kg_m3(inputs.mean_air_temperature_c, constants)?,
                settle_day_count: 1.0,
                temperature_c: inputs.mean_air_temperature_c.min(0.0),
                liquid_water_m: 0.0,
                cold_content_j_m2: 0.0,
                refrozen_liquid_m: 0.0,
            },
        );
    }

    apply_multilayer_daily_compaction(
        &mut layers,
        inputs.liquid_for_compaction_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.mean_air_temperature_c.clamp(-30.0, 0.0),
        constants,
    );
    let unbounded_swe_m = multilayer_mass_kg_m2(&layers) / SNOW_DENSITY_RHO_WATER_KG_M3;
    apply_multilayer_boundary_mass(
        &mut layers,
        inputs.boundary_swe_after_m * SNOW_DENSITY_RHO_WATER_KG_M3,
        inputs.boundary_density_after_kg_m3,
        constants,
    );
    merge_multilayer_bottom_layers(&mut layers);
    cap_multilayer_density(&mut layers, inputs.runtime_density_cap_kg_m3);

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
    runtime_density_after_kg_m3 =
        runtime_density_after_kg_m3.min(inputs.runtime_density_cap_kg_m3);
    let identity_residual_m = runtime_swe_after_m - inputs.boundary_swe_after_m;

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
) {
    if layers.is_empty() {
        return;
    }
    let total_mass_kg_m2 = multilayer_mass_kg_m2(layers);
    if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2
        && total_mass_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2
    {
        for layer in layers.iter_mut() {
            let liquid_for_layer_kg_m2 =
                liquid_for_compaction_kg_m2 * layer.mass_kg_m2 / total_mass_kg_m2;
            let mut state = layer.as_coe_bound_state();
            apply_wet_compaction(&mut state, liquid_for_layer_kg_m2, constants, 1.0);
            layer.density_kg_m3 = state.density_kg_m3;
        }
    }

    for _ in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
        let mut overburden_kg_m2 = 0.0;
        for layer in layers.iter_mut() {
            let mut state = layer.as_coe_bound_state();
            apply_time_compaction_scaled_with_overburden(
                &mut state,
                overburden_kg_m2,
                snow_temperature_c,
                constants,
                1.0,
            );
            layer.density_kg_m3 = state.density_kg_m3;
            overburden_kg_m2 += layer.mass_kg_m2;
        }
    }
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

fn apply_time_compaction(
    state: &mut CoeBoundDensityState,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
    shallow_guard_factor: f64,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return;
    }
    let swe = state.mass_kg_m2;
    let rate = if swe >= constants.dry_compaction_swe_max_kg_m2 {
        1.0
    } else {
        constants.compaction_rate_cos_amplitude
            * (std::f64::consts::PI * swe / constants.dry_compaction_swe_max_kg_m2).cos()
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
        * swe
        * (-constants.poc_density_decay * (density / SNOW_DENSITY_RHO_WATER_KG_M3)).exp()
        / rate;
    state.density_kg_m3 = (density
        + constants.dry_compaction_multiplier
            * shallow_guard_factor
            * (destructive_metamorphism + overburden_compaction)
            * density)
        .min(constants.dry_compaction_max_density_kg_m3);
}

fn apply_daily_compaction(
    state: &mut CoeBoundDensityState,
    liquid_for_compaction_kg_m2: f64,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
) {
    let shallow_guard_factor = shallow_compaction_guard_factor(state.depth_m(), constants);
    let wet_substeps = constants.wet_compaction_substeps_per_day.max(1);
    if wet_substeps == 1 {
        if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
            apply_wet_compaction(
                state,
                liquid_for_compaction_kg_m2,
                constants,
                shallow_guard_factor,
            );
        }
        for _ in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
            apply_time_compaction(state, snow_temperature_c, constants, shallow_guard_factor);
        }
        return;
    }

    let liquid_per_step = liquid_for_compaction_kg_m2 / f64::from(wet_substeps);
    if liquid_for_compaction_kg_m2 > SNOW_DENSITY_ZERO_MASS_KG_M2 {
        apply_wet_compaction(
            state,
            liquid_for_compaction_kg_m2,
            constants,
            shallow_guard_factor,
        );
    }
    for step in 0..SNOW_DENSITY_DAILY_COMPACTION_STEPS {
        let wet_step = step < wet_substeps && liquid_per_step > SNOW_DENSITY_ZERO_MASS_KG_M2;
        apply_time_compaction_scaled(
            state,
            snow_temperature_c,
            constants,
            if wet_step {
                constants.wet_compaction_multiplier
            } else {
                1.0
            } * shallow_guard_factor,
        );
    }
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
) {
    apply_time_compaction_scaled_with_overburden(
        state,
        state.mass_kg_m2,
        snow_temperature_c,
        constants,
        multiplier_scale,
    );
}

fn apply_time_compaction_scaled_with_overburden(
    state: &mut CoeBoundDensityState,
    overburden_kg_m2: f64,
    snow_temperature_c: f64,
    constants: SnowDensityCompactionConstants,
    multiplier_scale: f64,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return;
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
    state.density_kg_m3 = (density
        + constants.dry_compaction_multiplier
            * multiplier_scale
            * (destructive_metamorphism + overburden_compaction)
            * density)
        .min(constants.dry_compaction_max_density_kg_m3);
}

fn apply_wet_compaction(
    state: &mut CoeBoundDensityState,
    liquid_added_kg_m2: f64,
    constants: SnowDensityCompactionConstants,
    shallow_guard_factor: f64,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.wet_compaction_max_density_kg_m3 {
        return;
    }
    if state.mass_kg_m2 <= SNOW_DENSITY_ZERO_MASS_KG_M2 {
        return;
    }
    let h2o_added_ratio = liquid_added_kg_m2 / state.mass_kg_m2;
    if h2o_added_ratio <= 1.0e-6 {
        return;
    }
    let density_delta = constants.wet_compaction_multiplier
        * shallow_guard_factor
        * (constants.wet_compaction_max_density_kg_m3 - density)
        / (1.0 + constants.wet_compaction_half_saturation_ratio / h2o_added_ratio);
    state.density_kg_m3 = (density + density_delta).min(constants.wet_compaction_max_density_kg_m3);
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

fn density_validate_nonnegative(
    symbol: &'static str,
    value: f64,
) -> Result<(), SnowDensityError> {
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

    #[test]
    fn eb04d_layer_lifecycle_uses_mass_units_not_meter_residual_tolerance() {
        let mass_boundary_swe_m =
            SNOW_DENSITY_ZERO_MASS_KG_M2 / SNOW_DENSITY_RHO_WATER_KG_M3;
        let just_below = f64::from_bits(mass_boundary_swe_m.to_bits() - 1);
        let just_above = f64::from_bits(mass_boundary_swe_m.to_bits() + 1);

        assert!(!snow_density_layer_has_resolved_mass(just_below));
        assert!(!snow_density_layer_has_resolved_mass(mass_boundary_swe_m));
        assert!(snow_density_layer_has_resolved_mass(just_above));
        let captured_fragment_swe_m = 5.260_584_353_128_359e-10;
        assert!(snow_density_layer_has_resolved_mass(captured_fragment_swe_m));
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
