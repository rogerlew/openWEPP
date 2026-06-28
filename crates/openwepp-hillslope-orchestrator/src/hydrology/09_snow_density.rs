const SNOW_DENSITY_LEGACY_MODEL_ID: &str = "legacy_wepp";
const SNOW_DENSITY_COMPACTION_MODEL_ID: &str = "physics_bulk_density_compaction_v1";
const SNOW_DENSITY_SPRING_DENSIFICATION_MODEL_ID: &str = "physics_bulk_spring_densification_v1";
const SNOW_DENSITY_SHALLOW_GUARD_MODEL_ID: &str = "physics_bulk_shallow_guard_v1";
const SNOW_DENSITY_SHALLOW_GUARD_DEPTH_THRESHOLD_M: f64 = 0.25;
const SNOW_DENSITY_RHO_WATER_KG_M3: f64 = 1_000.0;
const SNOW_DENSITY_ZERO_MASS_KG_M2: f64 = 1.0e-9;
const SNOW_DENSITY_DAILY_COMPACTION_STEPS: u8 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowDensityModel {
    LegacyWepp,
    PhysicsBulkDensityCompactionV1,
    PhysicsBulkSpringDensificationV1,
    PhysicsBulkShallowGuardV1,
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
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowDensityRuntimeInputs {
    pub model: SnowDensityModel,
    pub prior_swe_m: f64,
    pub prior_depth_m: f64,
    pub prior_density_kg_m3: f64,
    pub boundary_swe_after_m: f64,
    pub boundary_depth_after_m: f64,
    pub boundary_density_after_kg_m3: f64,
    pub snow_input_m: f64,
    pub liquid_for_compaction_m: f64,
    pub mean_air_temperature_c: f64,
    pub runtime_density_cap_kg_m3: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowDensityRuntimeOutcome {
    pub model: SnowDensityModel,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub max_abs_swe_identity_residual_m: f64,
    pub max_abs_unbounded_swe_residual_m: f64,
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
        }
    }
}

impl std::error::Error for SnowDensityError {}

pub fn update_snow_density_runtime_state(
    inputs: SnowDensityRuntimeInputs,
) -> Result<SnowDensityRuntimeOutcome, SnowDensityError> {
    density_validate_nonnegative("prior_swe_m", inputs.prior_swe_m)?;
    density_validate_nonnegative("prior_depth_m", inputs.prior_depth_m)?;
    density_validate_nonnegative("prior_density_kg_m3", inputs.prior_density_kg_m3)?;
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
        });
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
    })
}

const fn snow_density_constants_for_model(
    model: SnowDensityModel,
) -> SnowDensityCompactionConstants {
    match model {
        SnowDensityModel::LegacyWepp | SnowDensityModel::PhysicsBulkDensityCompactionV1 => {
            snow_density_compaction_v1_constants()
        }
        SnowDensityModel::PhysicsBulkSpringDensificationV1 => {
            snow_density_spring_densification_v1_constants()
        }
        SnowDensityModel::PhysicsBulkShallowGuardV1 => snow_density_shallow_guard_v1_constants(),
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CoeBoundDensityState {
    mass_kg_m2: f64,
    density_kg_m3: f64,
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
