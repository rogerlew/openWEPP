const LEGACY_COE_MODEL_ID: &str = "legacy_coe";
const COE_SHORTWAVE_ALBEDO_MODEL_ID: &str = "coe_shortwave_albedo_v1";
const COE_WINTER_THAW_STATE_LOSS_MODEL_ID: &str = "coe_winter_thaw_state_loss_v1";
const COE_LIQUID_HOLDING_CAPACITY_MODEL_ID: &str = "coe_liquid_holding_capacity_v1";
const COE_OPEN_SUBLIMATION_STAGE_A_MODEL_ID: &str = "coe_open_sublimation_stage_a_v1";
const COE_OPEN_SUBLIMATION_STAGE_B_MODEL_ID: &str = "coe_open_sublimation_stage_b_v1";
const BROCK2000_ALBEDO_MODEL_ID: &str = "brock2000_temperature_age_v1";
const SNOW_ALBEDO_MIN: f64 = 0.0;
const SNOW_ALBEDO_MAX_BROCK2000: f64 = 0.85;
const SNOW_ALBEDO_FRESH_RESET_WATER_EQUIV_M: f64 = 0.001;
const SNOW_ALBEDO_ACTIVE_WATER_EQUIV_EPS_M: f64 = 1.0e-9;
const BROCK2000_LOG_DOMAIN_MIN_C_DAY: f64 = 1.0e-6;
const BROCK2000_DEEP_INTERCEPT: f64 = 0.713;
const BROCK2000_DEEP_LOG_COEFFICIENT: f64 = 0.112;
const BROCK2000_SHALLOW_ADDEND: f64 = 0.442;
const BROCK2000_SHALLOW_DECAY_PER_C_DAY: f64 = 0.058;
const BROCK2000_DEPTH_TRANSITION_SCALE_M: f64 = 0.024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowMeltModel {
    LegacyCoe,
    CoeShortwaveAlbedoV1,
    CoeWinterThawStateLossV1,
    CoeLiquidHoldingCapacityV1,
    CoeOpenSublimationStageAV1,
    CoeOpenSublimationStageBV1,
}

impl SnowMeltModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::LegacyCoe => LEGACY_COE_MODEL_ID,
            Self::CoeShortwaveAlbedoV1 => COE_SHORTWAVE_ALBEDO_MODEL_ID,
            Self::CoeWinterThawStateLossV1 => COE_WINTER_THAW_STATE_LOSS_MODEL_ID,
            Self::CoeLiquidHoldingCapacityV1 => COE_LIQUID_HOLDING_CAPACITY_MODEL_ID,
            Self::CoeOpenSublimationStageAV1 => COE_OPEN_SUBLIMATION_STAGE_A_MODEL_ID,
            Self::CoeOpenSublimationStageBV1 => COE_OPEN_SUBLIMATION_STAGE_B_MODEL_ID,
        }
    }

    #[must_use]
    pub const fn requires_snow_albedo_state(self) -> bool {
        matches!(self, Self::CoeShortwaveAlbedoV1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowAlbedoModel {
    Brock2000TemperatureAgeV1,
}

impl SnowAlbedoModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Brock2000TemperatureAgeV1 => BROCK2000_ALBEDO_MODEL_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowAlbedoState {
    pub model: SnowAlbedoModel,
    pub albedo: f64,
    pub accumulated_positive_temperature_c_day: f64,
}

impl SnowAlbedoState {
    /// Validate a persisted albedo state before it is carried across typed runtime state.
    pub fn validate(self) -> Result<(), SnowAlbedoError> {
        validate_previous_state(self.model, self)
    }

    #[must_use]
    pub fn shortwave_absorbed_fraction(self) -> f64 {
        1.0 - self.albedo
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowAlbedoUpdateInputs {
    pub melt_model: SnowMeltModel,
    pub albedo_model: Option<SnowAlbedoModel>,
    pub previous_state: Option<SnowAlbedoState>,
    pub snow_water_equivalent_m: f64,
    pub fresh_snow_water_equivalent_m: f64,
    pub positive_temperature_c_day_increment: f64,
    pub underlying_surface_albedo: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowAlbedoUpdateOutcome {
    pub active: bool,
    pub state: Option<SnowAlbedoState>,
    pub melt_model_id: &'static str,
    pub albedo_model_id: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SnowAlbedoError {
    MissingRequiredAlbedoModel,
    MissingRequiredAlbedoState,
    AlbedoModelMismatch {
        expected: SnowAlbedoModel,
        actual: SnowAlbedoModel,
    },
    NonFiniteInput {
        symbol: &'static str,
        value: f64,
    },
    OutOfRangeInput {
        symbol: &'static str,
        value: f64,
        minimum: f64,
        maximum: f64,
    },
}

impl fmt::Display for SnowAlbedoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredAlbedoModel => {
                write!(f, "missing required opt-in snow albedo model id")
            }
            Self::MissingRequiredAlbedoState => {
                write!(f, "missing required opt-in snow albedo state")
            }
            Self::AlbedoModelMismatch { expected, actual } => write!(
                f,
                "snow albedo model mismatch: expected {}, got {}",
                expected.id(),
                actual.id()
            ),
            Self::NonFiniteInput { symbol, value } => {
                write!(f, "non-finite snow albedo input {symbol}={value}")
            }
            Self::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "snow albedo input {symbol}={value} outside [{minimum}, {maximum}]"
            ),
        }
    }
}

impl Error for SnowAlbedoError {}

#[cfg(test)]
mod cqr_row5_snow_albedo_tests {
    use super::*;

    #[test]
    fn snow_albedo_error_display_covers_all_variants() {
        let cases = [
            (
                SnowAlbedoError::MissingRequiredAlbedoModel,
                "missing required opt-in snow albedo model id",
            ),
            (
                SnowAlbedoError::MissingRequiredAlbedoState,
                "missing required opt-in snow albedo state",
            ),
            (
                SnowAlbedoError::AlbedoModelMismatch {
                    expected: SnowAlbedoModel::Brock2000TemperatureAgeV1,
                    actual: SnowAlbedoModel::Brock2000TemperatureAgeV1,
                },
                "snow albedo model mismatch",
            ),
            (
                SnowAlbedoError::NonFiniteInput {
                    symbol: "row5.albedo",
                    value: f64::NAN,
                },
                "non-finite snow albedo input row5.albedo",
            ),
            (
                SnowAlbedoError::OutOfRangeInput {
                    symbol: "row5.albedo",
                    value: 2.0,
                    minimum: 0.0,
                    maximum: 1.0,
                },
                "snow albedo input row5.albedo=2 outside [0, 1]",
            ),
        ];

        for (error, expected_fragment) in cases {
            assert!(
                error.to_string().contains(expected_fragment),
                "{error:?} rendered as {error}"
            );
        }
    }
}

pub fn update_snow_albedo_state(
    inputs: SnowAlbedoUpdateInputs,
) -> Result<SnowAlbedoUpdateOutcome, SnowAlbedoError> {
    if !inputs.melt_model.requires_snow_albedo_state() {
        return Ok(SnowAlbedoUpdateOutcome {
            active: false,
            state: None,
            melt_model_id: inputs.melt_model.id(),
            albedo_model_id: None,
        });
    }

    validate_nonnegative("snow_water_equivalent_m", inputs.snow_water_equivalent_m)?;
    validate_nonnegative(
        "fresh_snow_water_equivalent_m",
        inputs.fresh_snow_water_equivalent_m,
    )?;
    validate_nonnegative(
        "positive_temperature_c_day_increment",
        inputs.positive_temperature_c_day_increment,
    )?;
    validate_range(
        "underlying_surface_albedo",
        inputs.underlying_surface_albedo,
        SNOW_ALBEDO_MIN,
        1.0,
    )?;

    let albedo_model = inputs
        .albedo_model
        .ok_or(SnowAlbedoError::MissingRequiredAlbedoModel)?;
    if albedo_model != SnowAlbedoModel::Brock2000TemperatureAgeV1 {
        return Err(SnowAlbedoError::AlbedoModelMismatch {
            expected: SnowAlbedoModel::Brock2000TemperatureAgeV1,
            actual: albedo_model,
        });
    }

    let effective_snow_water_equivalent_m = inputs
        .snow_water_equivalent_m
        .max(inputs.fresh_snow_water_equivalent_m);
    if effective_snow_water_equivalent_m <= SNOW_ALBEDO_ACTIVE_WATER_EQUIV_EPS_M {
        return Ok(SnowAlbedoUpdateOutcome {
            active: false,
            state: None,
            melt_model_id: inputs.melt_model.id(),
            albedo_model_id: Some(albedo_model.id()),
        });
    }

    let accumulated_positive_temperature_c_day =
        if inputs.fresh_snow_water_equivalent_m >= SNOW_ALBEDO_FRESH_RESET_WATER_EQUIV_M {
            inputs.positive_temperature_c_day_increment
        } else {
            let previous_state = inputs
                .previous_state
                .ok_or(SnowAlbedoError::MissingRequiredAlbedoState)?;
            validate_previous_state(albedo_model, previous_state)?;
            previous_state.accumulated_positive_temperature_c_day
                + inputs.positive_temperature_c_day_increment
        };

    let albedo = brock2000_temperature_age_albedo(
        accumulated_positive_temperature_c_day,
        effective_snow_water_equivalent_m,
        inputs.underlying_surface_albedo,
    );

    Ok(SnowAlbedoUpdateOutcome {
        active: true,
        state: Some(SnowAlbedoState {
            model: albedo_model,
            albedo,
            accumulated_positive_temperature_c_day,
        }),
        melt_model_id: inputs.melt_model.id(),
        albedo_model_id: Some(albedo_model.id()),
    })
}

fn validate_previous_state(
    expected_model: SnowAlbedoModel,
    state: SnowAlbedoState,
) -> Result<(), SnowAlbedoError> {
    if state.model != expected_model {
        return Err(SnowAlbedoError::AlbedoModelMismatch {
            expected: expected_model,
            actual: state.model,
        });
    }
    validate_range(
        "snow_albedo",
        state.albedo,
        SNOW_ALBEDO_MIN,
        SNOW_ALBEDO_MAX_BROCK2000,
    )?;
    validate_nonnegative(
        "snow_albedo_accumulated_positive_temperature_c_day",
        state.accumulated_positive_temperature_c_day,
    )
}

fn brock2000_temperature_age_albedo(
    accumulated_positive_temperature_c_day: f64,
    snow_water_equivalent_m: f64,
    underlying_surface_albedo: f64,
) -> f64 {
    let ta_for_log = accumulated_positive_temperature_c_day.max(BROCK2000_LOG_DOMAIN_MIN_C_DAY);
    let alpha_deep =
        BROCK2000_DEEP_INTERCEPT - BROCK2000_DEEP_LOG_COEFFICIENT * ta_for_log.log10();
    let alpha_shallow = underlying_surface_albedo
        + BROCK2000_SHALLOW_ADDEND
            * (-BROCK2000_SHALLOW_DECAY_PER_C_DAY * accumulated_positive_temperature_c_day).exp();
    let shallow_weight = (-snow_water_equivalent_m / BROCK2000_DEPTH_TRANSITION_SCALE_M).exp();
    let alpha = (1.0 - shallow_weight) * alpha_deep + shallow_weight * alpha_shallow;
    alpha.clamp(SNOW_ALBEDO_MIN, SNOW_ALBEDO_MAX_BROCK2000)
}

fn validate_nonnegative(symbol: &'static str, value: f64) -> Result<(), SnowAlbedoError> {
    validate_range(symbol, value, 0.0, f64::INFINITY)
}

fn validate_range(
    symbol: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> Result<(), SnowAlbedoError> {
    if !value.is_finite() {
        return Err(SnowAlbedoError::NonFiniteInput { symbol, value });
    }
    if value < minimum || value > maximum {
        return Err(SnowAlbedoError::OutOfRangeInput {
            symbol,
            value,
            minimum,
            maximum,
        });
    }
    Ok(())
}
