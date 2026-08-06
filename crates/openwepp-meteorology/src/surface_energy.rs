//! Surface-agnostic energy-balance flux primitives.
//!
//! Sign convention follows SNOBAL/libsnobal: positive flux warms or adds mass
//! to the modeled surface; negative flux removes energy or vapor mass.

use openwepp_unit_boundary::{
    BoundaryError, FractionUnitInterval, LinearRateMetersPerSecond, TemperatureCelsius,
};

use crate::error::MeteorologyError;

const LIBSNOBAL_FREEZE_K: f64 = 273.16;
const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_32e-8;
const MOLAR_MASS_DRY_AIR_KG_PER_KMOL: f64 = 28.9644;
const MOLAR_MASS_WATER_KG_PER_KMOL: f64 = 18.0153;
const UNIVERSAL_GAS_CONSTANT_J_KMOL_K: f64 = 8.314_32e3;
const SPECIFIC_HEAT_AIR_J_KG_K: f64 = 1.005e3;
const SPECIFIC_HEAT_WATER_0C_J_KG_K: f64 = 4_217.7;
const GRAVITY_M_S2: f64 = 9.806_65;
const VON_KARMAN: f64 = 0.41;
const DRY_ADIABATIC_LAPSE_RATE_K_M: f64 = GRAVITY_M_S2 / SPECIFIC_HEAT_AIR_J_KG_K;
const SEA_LEVEL_PRESSURE_PA: f64 = 1.013_246e5;
const BOILING_POINT_K: f64 = 373.15;
const PAESCHKE: f64 = 7.35;
const BETA_STABLE: f64 = 5.2;
const BETA_UNSTABLE: f64 = 16.0;
const DEFAULT_TURBULENT_MAX_ITERATIONS: usize = 50;
const DEFAULT_TURBULENT_CONVERGENCE_TOLERANCE: f64 = 1.0e-5;
const CALORIE_TO_JOULE: f64 = 4.186_798_188;
const SNOWENERGY_DILLEY_INTERCEPT_W_M2: f64 = 59.38;
const SNOWENERGY_DILLEY_TEMPERATURE_COEFFICIENT_W_M2: f64 = 113.7;
const SNOWENERGY_DILLEY_TEMPERATURE_REFERENCE_K: f64 = 273.16;
const SNOWENERGY_DILLEY_WATER_COEFFICIENT_W_M2: f64 = 96.96;
const SNOWENERGY_DILLEY_WATER_REFERENCE_KG_M2: f64 = 25.0;
const SNOWENERGY_PRECIPITABLE_WATER_FACTOR: f64 = 4_650.0;
const SNOWENERGY_UNSWORTH_CLOUD_WEIGHT: f64 = 0.84;
const SNOWENERGY_CLEAR_CLOUD_INDEX: f64 = 0.80;
const SNOWENERGY_OVERCAST_CLOUD_INDEX: f64 = 0.15;
const SNOWENERGY_DIFFUSE_EXTINCTION_FACTOR: f64 = 1.6;
const SNOWENERGY_EXTRATERRESTRIAL_MIN_MJ_M2_DAY: f64 = 1.0e-9;
const SNOBAL_SNOW_CONDUCTIVITY_COEFFICIENT_CAL_M_S_K: f64 = 0.0077;
const SNOBAL_POROUS_LAYER_DIFFUSIVITY_SCALE_M2_S: f64 = 0.0001;

/// Signed energy flux in watts per square meter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyFluxWattsPerSquareMeter(f64);

impl EnergyFluxWattsPerSquareMeter {
    /// Construct a finite signed energy flux.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_w_m2` is not finite.
    pub fn try_new(value_w_m2: f64) -> Result<Self, MeteorologyError> {
        validate_finite("energy_flux_w_m2", value_w_m2)?;
        Ok(Self(value_w_m2))
    }

    /// Raw value in watts per square meter.
    #[must_use]
    pub const fn as_watts_per_square_meter(self) -> f64 {
        self.0
    }
}

/// Non-negative radiative flux in watts per square meter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadiativeFluxWattsPerSquareMeter(f64);

impl RadiativeFluxWattsPerSquareMeter {
    /// Construct a finite non-negative radiative flux.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_w_m2` is not finite or is negative.
    pub fn try_new(value_w_m2: f64) -> Result<Self, MeteorologyError> {
        validate_non_negative("radiative_flux_w_m2", value_w_m2)?;
        Ok(Self(value_w_m2))
    }

    /// Raw value in watts per square meter.
    #[must_use]
    pub const fn as_watts_per_square_meter(self) -> f64 {
        self.0
    }
}

/// Atmospheric or vapor pressure in pascals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressurePascals(f64);

impl PressurePascals {
    /// Construct a positive finite pressure.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_pa` is not finite or is not positive.
    pub fn try_new(value_pa: f64) -> Result<Self, MeteorologyError> {
        validate_positive("pressure_pa", value_pa)?;
        Ok(Self(value_pa))
    }

    /// Construct pressure from kilopascals.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the converted pressure is invalid.
    pub fn from_kilopascals(value_kpa: f64) -> Result<Self, MeteorologyError> {
        validate_finite("pressure_kpa", value_kpa)?;
        // UNIT-CONVERSION-ALLOW: mm_m_scale named Pa/kPa boundary conversion.
        Self::try_new(value_kpa * 1_000.0)
    }

    /// Raw value in pascals.
    #[must_use]
    pub const fn as_pascals(self) -> f64 {
        self.0
    }

    /// Raw value converted to kilopascals.
    #[must_use]
    pub const fn as_kilopascals(self) -> f64 {
        // UNIT-CONVERSION-ALLOW: mm_m_scale named Pa/kPa boundary conversion.
        self.0 / 1_000.0
    }
}

/// Positive distance or layer thickness in meters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositiveLengthMeters(f64);

impl PositiveLengthMeters {
    /// Construct a positive finite length.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_m` is not finite or is not positive.
    pub fn try_new(value_m: f64) -> Result<Self, MeteorologyError> {
        validate_positive("length_m", value_m)?;
        Ok(Self(value_m))
    }

    /// Raw value in meters.
    #[must_use]
    pub const fn as_meters(self) -> f64 {
        self.0
    }
}

/// Signed mass flux in kilograms per square meter per second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassFluxKilogramsPerSquareMeterSecond(f64);

impl MassFluxKilogramsPerSquareMeterSecond {
    /// Construct a finite signed mass flux.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_kg_m2_s` is not finite.
    pub fn try_new(value_kg_m2_s: f64) -> Result<Self, MeteorologyError> {
        validate_finite("mass_flux_kg_m2_s", value_kg_m2_s)?;
        Ok(Self(value_kg_m2_s))
    }

    /// Raw value in kilograms per square meter per second.
    #[must_use]
    pub const fn as_kilograms_per_square_meter_second(self) -> f64 {
        self.0
    }
}

/// Non-negative precipitation mass flux in kilograms per square meter per second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrecipitationMassFluxKilogramsPerSquareMeterSecond(f64);

impl PrecipitationMassFluxKilogramsPerSquareMeterSecond {
    /// Construct a finite non-negative precipitation mass flux.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_kg_m2_s` is not finite or is negative.
    pub fn try_new(value_kg_m2_s: f64) -> Result<Self, MeteorologyError> {
        validate_non_negative("precipitation_mass_flux_kg_m2_s", value_kg_m2_s)?;
        Ok(Self(value_kg_m2_s))
    }

    /// Raw value in kilograms per square meter per second.
    #[must_use]
    pub const fn as_kilograms_per_square_meter_second(self) -> f64 {
        self.0
    }
}

/// Thermal conductivity in watts per meter per kelvin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalConductivityWattsPerMeterKelvin(f64);

impl ThermalConductivityWattsPerMeterKelvin {
    /// Construct a positive finite thermal conductivity.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_w_m_k` is not finite or is not positive.
    pub fn try_new(value_w_m_k: f64) -> Result<Self, MeteorologyError> {
        validate_positive("thermal_conductivity_w_m_k", value_w_m_k)?;
        Ok(Self(value_w_m_k))
    }

    /// Raw value in watts per meter per kelvin.
    #[must_use]
    pub const fn as_watts_per_meter_kelvin(self) -> f64 {
        self.0
    }
}

/// Specific heat capacity in joules per kilogram per kelvin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificHeatCapacityJoulesPerKilogramKelvin(f64);

impl SpecificHeatCapacityJoulesPerKilogramKelvin {
    /// Construct a positive finite specific heat capacity.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_j_kg_k` is not finite or is not positive.
    pub fn try_new(value_j_kg_k: f64) -> Result<Self, MeteorologyError> {
        validate_positive("specific_heat_capacity_j_kg_k", value_j_kg_k)?;
        Ok(Self(value_j_kg_k))
    }

    /// Raw value in joules per kilogram per kelvin.
    #[must_use]
    pub const fn as_joules_per_kilogram_kelvin(self) -> f64 {
        self.0
    }
}

/// Latent heat in joules per kilogram.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatentHeatJoulesPerKilogram(f64);

impl LatentHeatJoulesPerKilogram {
    /// Construct a positive finite latent heat.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_j_kg` is not finite or is not positive.
    pub fn try_new(value_j_kg: f64) -> Result<Self, MeteorologyError> {
        validate_positive("latent_heat_j_kg", value_j_kg)?;
        Ok(Self(value_j_kg))
    }

    /// Raw value in joules per kilogram.
    #[must_use]
    pub const fn as_joules_per_kilogram(self) -> f64 {
        self.0
    }
}

/// Net all-wave radiation inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllWaveRadiationInputs {
    /// Incoming shortwave radiation.
    pub incoming_shortwave: RadiativeFluxWattsPerSquareMeter,
    /// Surface shortwave albedo.
    pub albedo: FractionUnitInterval,
    /// Incoming longwave radiation.
    pub incoming_longwave: RadiativeFluxWattsPerSquareMeter,
    /// Surface longwave emissivity.
    pub emissivity: FractionUnitInterval,
    /// Surface temperature.
    pub surface_temperature: TemperatureCelsius,
}

/// Canonical SC-SNOWENERGY-001 inputs for one hourly snow longwave evaluation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowLongwaveInputs {
    pub air_temperature: TemperatureCelsius,
    pub surface_temperature: TemperatureCelsius,
    pub actual_vapor_pressure: PressurePascals,
    pub daily_solar_radiation_mj_m2: f64,
    pub daily_extraterrestrial_radiation_mj_m2: f64,
    pub daylight: bool,
    pub canopy_cover: FractionUnitInterval,
}

/// Reconstruction operands for canonical atmospheric and sub-canopy longwave.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowLongwaveFluxes {
    pub cloud_fraction: FractionUnitInterval,
    pub sky_view_fraction: FractionUnitInterval,
    pub atmospheric_longwave: RadiativeFluxWattsPerSquareMeter,
    pub subcanopy_longwave: RadiativeFluxWattsPerSquareMeter,
    pub canopy_longwave: RadiativeFluxWattsPerSquareMeter,
    pub outgoing_longwave: RadiativeFluxWattsPerSquareMeter,
    pub net_longwave: EnergyFluxWattsPerSquareMeter,
}

/// Evaluate the Dilley-Unsworth atmospheric and FSM2-derived sub-canopy
/// longwave route selected by SC-SNOWENERGY-001.
///
/// # Errors
///
/// Returns a typed error for invalid radiation, polar night, a closed canopy,
/// or a derived emissivity outside the admitted physical domain.
pub fn snow_longwave_dilley_unsworth(
    inputs: SnowLongwaveInputs,
) -> Result<SnowLongwaveFluxes, MeteorologyError> {
    validate_non_negative(
        "daily_solar_radiation_mj_m2",
        inputs.daily_solar_radiation_mj_m2,
    )?;
    validate_non_negative(
        "daily_extraterrestrial_radiation_mj_m2",
        inputs.daily_extraterrestrial_radiation_mj_m2,
    )?;
    if !inputs.daylight
        || inputs.daily_extraterrestrial_radiation_mj_m2
            <= SNOWENERGY_EXTRATERRESTRIAL_MIN_MJ_M2_DAY
    {
        return Err(MeteorologyError::CloudForcingUnavailable);
    }
    let air_temperature_k = libsnobal_kelvin(inputs.air_temperature)?;
    let surface_temperature_k = libsnobal_kelvin(inputs.surface_temperature)?;
    let vapor_pressure_kpa = inputs.actual_vapor_pressure.as_kilopascals();
    let precipitable_water =
        SNOWENERGY_PRECIPITABLE_WATER_FACTOR * vapor_pressure_kpa / air_temperature_k;
    let clear_longwave = SNOWENERGY_DILLEY_INTERCEPT_W_M2
        + SNOWENERGY_DILLEY_TEMPERATURE_COEFFICIENT_W_M2
            * (air_temperature_k / SNOWENERGY_DILLEY_TEMPERATURE_REFERENCE_K).powi(6)
        + SNOWENERGY_DILLEY_WATER_COEFFICIENT_W_M2
            * (precipitable_water / SNOWENERGY_DILLEY_WATER_REFERENCE_KG_M2).sqrt();
    let blackbody_air = STEFAN_BOLTZMANN_W_M2_K4 * air_temperature_k.powi(4);
    let clear_emissivity = clear_longwave / blackbody_air;
    require_unit_interval_authority("clear_sky_emissivity", clear_emissivity)?;
    let clearness =
        inputs.daily_solar_radiation_mj_m2 / inputs.daily_extraterrestrial_radiation_mj_m2;
    let cloud = ((SNOWENERGY_CLEAR_CLOUD_INDEX - clearness)
        / (SNOWENERGY_CLEAR_CLOUD_INDEX - SNOWENERGY_OVERCAST_CLOUD_INDEX))
        .clamp(0.0, 1.0);
    let all_sky_emissivity = (1.0 - SNOWENERGY_UNSWORTH_CLOUD_WEIGHT * cloud) * clear_emissivity
        + SNOWENERGY_UNSWORTH_CLOUD_WEIGHT * cloud;
    require_unit_interval_authority("all_sky_emissivity", all_sky_emissivity)?;
    let atmospheric = all_sky_emissivity * blackbody_air;
    let cover = inputs.canopy_cover.as_fraction();
    if cover >= 1.0 {
        return Err(MeteorologyError::OutOfAuthority {
            quantity: "canopy_cover_fraction",
            value: cover,
            minimum: 0.0,
            maximum: 1.0,
        });
    }
    let sky_view = (1.0 - cover).powf(SNOWENERGY_DIFFUSE_EXTINCTION_FACTOR);
    let canopy_longwave = STEFAN_BOLTZMANN_W_M2_K4 * air_temperature_k.powi(4);
    let subcanopy = sky_view * atmospheric + (1.0 - sky_view) * canopy_longwave;
    let outgoing = STEFAN_BOLTZMANN_W_M2_K4 * surface_temperature_k.powi(4);
    Ok(SnowLongwaveFluxes {
        cloud_fraction: FractionUnitInterval::try_new(cloud)?,
        sky_view_fraction: FractionUnitInterval::try_new(sky_view)?,
        atmospheric_longwave: RadiativeFluxWattsPerSquareMeter::try_new(atmospheric)?,
        subcanopy_longwave: RadiativeFluxWattsPerSquareMeter::try_new(subcanopy)?,
        canopy_longwave: RadiativeFluxWattsPerSquareMeter::try_new(canopy_longwave)?,
        outgoing_longwave: RadiativeFluxWattsPerSquareMeter::try_new(outgoing)?,
        net_longwave: EnergyFluxWattsPerSquareMeter::try_new(subcanopy - outgoing)?,
    })
}

fn require_unit_interval_authority(
    quantity: &'static str,
    value: f64,
) -> Result<(), MeteorologyError> {
    validate_finite(quantity, value)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(MeteorologyError::OutOfAuthority {
            quantity,
            value,
            minimum: 0.0,
            maximum: 1.0,
        });
    }
    Ok(())
}

/// Monin-Obukhov turbulent-transfer solver controls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurbulentTransferOptions {
    /// Maximum Obukhov-length iterations.
    pub max_iterations: usize,
    /// Absolute and relative convergence tolerance for Obukhov length.
    pub convergence_tolerance: f64,
}

impl Default for TurbulentTransferOptions {
    fn default() -> Self {
        Self {
            max_iterations: DEFAULT_TURBULENT_MAX_ITERATIONS,
            convergence_tolerance: DEFAULT_TURBULENT_CONVERGENCE_TOLERANCE,
        }
    }
}

impl TurbulentTransferOptions {
    /// Construct solver options with domain checks.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the iteration count is zero or tolerance is
    /// not finite and positive.
    pub fn try_new(
        max_iterations: usize,
        convergence_tolerance: f64,
    ) -> Result<Self, MeteorologyError> {
        if max_iterations == 0 {
            return Err(MeteorologyError::InvalidSolverOptions {
                quantity: "turbulent_max_iterations",
                value: 0.0,
            });
        }
        if !convergence_tolerance.is_finite() || convergence_tolerance <= 0.0 {
            return Err(MeteorologyError::InvalidSolverOptions {
                quantity: "turbulent_convergence_tolerance",
                value: convergence_tolerance,
            });
        }
        Ok(Self {
            max_iterations,
            convergence_tolerance,
        })
    }
}

/// Inputs for Monin-Obukhov turbulent sensible and latent heat fluxes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurbulentFluxInputs {
    /// Atmospheric pressure.
    pub air_pressure: PressurePascals,
    /// Air temperature at `air_temperature_height`.
    pub air_temperature: TemperatureCelsius,
    /// Surface temperature.
    pub surface_temperature: TemperatureCelsius,
    /// Air vapor pressure at `vapor_pressure_height`.
    pub air_vapor_pressure: PressurePascals,
    /// Surface vapor pressure.
    pub surface_vapor_pressure: PressurePascals,
    /// Height of air temperature measurement above the surface.
    pub air_temperature_height: PositiveLengthMeters,
    /// Height of vapor pressure measurement above the surface.
    pub vapor_pressure_height: PositiveLengthMeters,
    /// Wind speed at `wind_speed_height`.
    pub wind_speed: LinearRateMetersPerSecond,
    /// Height of wind speed measurement above the surface.
    pub wind_speed_height: PositiveLengthMeters,
    /// Aerodynamic roughness length.
    pub roughness_length: PositiveLengthMeters,
    /// Solver controls.
    pub options: TurbulentTransferOptions,
}

/// Turbulent sensible heat, latent heat, and vapor mass fluxes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurbulentFluxes {
    /// Sensible heat flux.
    pub sensible_heat: EnergyFluxWattsPerSquareMeter,
    /// Latent heat flux.
    pub latent_heat: EnergyFluxWattsPerSquareMeter,
    /// Vapor mass flux.
    pub mass_flux: MassFluxKilogramsPerSquareMeterSecond,
    /// Iterations used by the stability solver.
    pub iterations: usize,
    /// Last Obukhov stability length, when non-neutral iteration was needed.
    pub obukhov_length_m: Option<f64>,
}

/// Exact successful termination of the Monin-Obukhov solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbulentTerminationStatus {
    ZeroWind,
    InitialPotentialTemperatureNeutral,
    IterativeZeroBuoyancy,
    IterativeInvalidObukhov,
    ConvergedStable,
    ConvergedUnstable,
}

impl TurbulentTerminationStatus {
    /// Stable schema identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::ZeroWind => "zero_wind",
            Self::InitialPotentialTemperatureNeutral => "initial_potential_temperature_neutral",
            Self::IterativeZeroBuoyancy => "iterative_zero_buoyancy",
            Self::IterativeInvalidObukhov => "iterative_invalid_obukhov",
            Self::ConvergedStable => "converged_stable",
            Self::ConvergedUnstable => "converged_unstable",
        }
    }
}

/// Stability classification attached to a successful turbulent solve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbulentStabilityClass {
    ZeroWind,
    Neutral,
    Stable,
    Unstable,
    IndeterminateObukhov,
}

impl TurbulentStabilityClass {
    /// Stable schema identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::ZeroWind => "zero_wind",
            Self::Neutral => "neutral",
            Self::Stable => "stable",
            Self::Unstable => "unstable",
            Self::IndeterminateObukhov => "indeterminate_obukhov",
        }
    }
}

/// Primitive state from the exact solver invocation that produced `fluxes`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurbulentFluxDiagnostics {
    pub fluxes: TurbulentFluxes,
    pub termination_status: TurbulentTerminationStatus,
    pub stability_class: TurbulentStabilityClass,
    pub momentum_stability_correction: f64,
    pub sensible_stability_correction: f64,
    pub latent_stability_correction: f64,
    pub displacement_height_m: Option<f64>,
    pub log_momentum: Option<f64>,
    pub log_sensible: Option<f64>,
    pub log_latent: Option<f64>,
    pub friction_velocity_m_s: f64,
    pub sensible_exchange_velocity_m_s: Option<f64>,
    pub latent_exchange_velocity_m_s: Option<f64>,
    pub air_density_kg_m3: Option<f64>,
    pub air_potential_temperature_k: Option<f64>,
    pub surface_temperature_k: Option<f64>,
    pub specific_humidity_air_kg_kg: Option<f64>,
    pub specific_humidity_surface_kg_kg: Option<f64>,
    pub latent_heat_j_kg: Option<f64>,
}

/// Precipitation advected-heat inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrecipitationAdvectedHeatInputs {
    /// Rain mass flux.
    pub rain_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond,
    /// Rain temperature.
    pub rain_temperature: TemperatureCelsius,
    /// Snowfall mass flux.
    pub snow_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond,
    /// Snowfall temperature.
    pub snow_temperature: TemperatureCelsius,
    /// Surface temperature receiving precipitation heat.
    pub surface_temperature: TemperatureCelsius,
}

/// Surface energy-balance terms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceEnergyBalanceTerms {
    /// Net all-wave radiation.
    pub net_radiation: EnergyFluxWattsPerSquareMeter,
    /// Turbulent sensible heat.
    pub sensible_heat: EnergyFluxWattsPerSquareMeter,
    /// Turbulent latent heat.
    pub latent_heat: EnergyFluxWattsPerSquareMeter,
    /// Ground/substrate conduction.
    pub conduction: EnergyFluxWattsPerSquareMeter,
    /// Precipitation advected heat.
    pub advected_heat: EnergyFluxWattsPerSquareMeter,
}

/// Net absorbed shortwave radiation.
///
/// # Errors
///
/// Returns a typed error if the computed flux is not finite.
pub fn net_shortwave_radiation(
    incoming_shortwave: RadiativeFluxWattsPerSquareMeter,
    albedo: FractionUnitInterval,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    EnergyFluxWattsPerSquareMeter::try_new(
        incoming_shortwave.as_watts_per_square_meter() * (1.0 - albedo.as_fraction()),
    )
}

/// Net longwave radiation using surface emissivity.
///
/// # Errors
///
/// Returns a typed error if the surface temperature is at/below absolute zero
/// or if the computed flux is not finite.
pub fn net_longwave_radiation(
    incoming_longwave: RadiativeFluxWattsPerSquareMeter,
    emissivity: FractionUnitInterval,
    surface_temperature: TemperatureCelsius,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    let surface_kelvin = libsnobal_kelvin(surface_temperature)?;
    EnergyFluxWattsPerSquareMeter::try_new(
        emissivity.as_fraction()
            * (incoming_longwave.as_watts_per_square_meter()
                - STEFAN_BOLTZMANN_W_M2_K4 * surface_kelvin.powi(4)),
    )
}

/// Net all-wave radiation: net shortwave plus net longwave.
///
/// # Errors
///
/// Returns a typed error when either component calculation fails.
pub fn net_all_wave_radiation(
    inputs: AllWaveRadiationInputs,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    let shortwave = net_shortwave_radiation(inputs.incoming_shortwave, inputs.albedo)?;
    let longwave = net_longwave_radiation(
        inputs.incoming_longwave,
        inputs.emissivity,
        inputs.surface_temperature,
    )?;
    EnergyFluxWattsPerSquareMeter::try_new(
        shortwave.as_watts_per_square_meter() + longwave.as_watts_per_square_meter(),
    )
}

/// Conductive heat exchange between a surface layer and substrate.
///
/// Positive values move heat toward the first/surface layer.
///
/// # Errors
///
/// Returns a typed error if the computed flux is not finite.
pub fn conductive_heat_flux(
    surface_conductivity: ThermalConductivityWattsPerMeterKelvin,
    substrate_conductivity: ThermalConductivityWattsPerMeterKelvin,
    surface_temperature: TemperatureCelsius,
    substrate_temperature: TemperatureCelsius,
    surface_layer_thickness: PositiveLengthMeters,
    substrate_layer_thickness: PositiveLengthMeters,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    let surface_kelvin = libsnobal_kelvin(surface_temperature)?;
    let substrate_kelvin = libsnobal_kelvin(substrate_temperature)?;
    let numerator = 2.0
        * surface_conductivity.as_watts_per_meter_kelvin()
        * substrate_conductivity.as_watts_per_meter_kelvin()
        * (substrate_kelvin - surface_kelvin);
    let denominator = substrate_conductivity.as_watts_per_meter_kelvin()
        * surface_layer_thickness.as_meters()
        + surface_conductivity.as_watts_per_meter_kelvin() * substrate_layer_thickness.as_meters();
    EnergyFluxWattsPerSquareMeter::try_new(numerator / denominator)
}

/// SNOBAL effective snow conductivity after Yen (1965) and Anderson (1976).
///
/// This is the exact `KTS` plus `efcon` formulation used by libsnobal:
/// dry snow conduction is augmented by saturated pore-vapor diffusion at the
/// supplied layer temperature and atmospheric pressure.
///
/// # Errors
///
/// Returns a typed error for non-positive density, invalid temperature or
/// pressure, saturation pressure at/above atmospheric pressure, or a
/// non-finite/non-positive effective conductivity.
pub fn snow_effective_thermal_conductivity_snobal(
    snow_density_kg_m3: f64,
    snow_temperature: TemperatureCelsius,
    air_pressure: PressurePascals,
) -> Result<ThermalConductivityWattsPerMeterKelvin, MeteorologyError> {
    validate_positive("snow_density_kg_m3", snow_density_kg_m3)?;
    let temperature_kelvin = libsnobal_kelvin(snow_temperature)?;
    let pressure_pa = air_pressure.as_pascals();
    let saturation_pressure_pa =
        saturation_vapor_pressure_snobal_pa(snow_temperature)?.as_pascals();
    if saturation_pressure_pa >= pressure_pa {
        return Err(MeteorologyError::OutOfAuthority {
            quantity: "snow_saturation_pressure_pa",
            value: saturation_pressure_pa,
            minimum: 0.0,
            maximum: pressure_pa,
        });
    }
    // UNIT-CONVERSION-ALLOW: mm_m_scale exact libsnobal KTS density ratio.
    let relative_density = snow_density_kg_m3 / 1_000.0;
    let dry_conductivity_w_m_k = CALORIE_TO_JOULE
        * SNOBAL_SNOW_CONDUCTIVITY_COEFFICIENT_CAL_M_S_K
        * relative_density
        * relative_density;
    let diffusivity_m2_s = 0.65
        * (SEA_LEVEL_PRESSURE_PA / pressure_pa)
        * (temperature_kelvin / LIBSNOBAL_FREEZE_K).powf(14.0)
        * SNOBAL_POROUS_LAYER_DIFFUSIVITY_SCALE_M2_S;
    let latent_heat_j_kg =
        latent_heat_for_surface_temperature(snow_temperature)?.as_joules_per_kilogram();
    let mixing_ratio = (MOLAR_MASS_WATER_KG_PER_KMOL / MOLAR_MASS_DRY_AIR_KG_PER_KMOL)
        * saturation_pressure_pa
        / (pressure_pa - saturation_pressure_pa);
    ThermalConductivityWattsPerMeterKelvin::try_new(
        dry_conductivity_w_m_k + latent_heat_j_kg * diffusivity_m2_s * mixing_ratio,
    )
}

/// Monin-Obukhov bulk-aerodynamic sensible, latent, and vapor mass fluxes.
///
/// # Errors
///
/// Returns a typed error when heights, pressures, temperatures, or solver
/// options are outside the supported domain, or when the stability iteration
/// does not converge.
pub fn turbulent_fluxes_monin_obukhov(
    inputs: TurbulentFluxInputs,
) -> Result<TurbulentFluxes, MeteorologyError> {
    turbulent_fluxes_monin_obukhov_with_diagnostics(inputs).map(|result| result.fluxes)
}

/// Monin-Obukhov fluxes plus the exact primitive solver lineage.
///
/// This calls the same private solver as [`turbulent_fluxes_monin_obukhov`].
///
/// # Errors
///
/// Returns the same typed errors as [`turbulent_fluxes_monin_obukhov`].
pub fn turbulent_fluxes_monin_obukhov_with_diagnostics(
    inputs: TurbulentFluxInputs,
) -> Result<TurbulentFluxDiagnostics, MeteorologyError> {
    validate_turbulent_inputs(inputs)?;
    if inputs.wind_speed.as_meters_per_second() == 0.0 {
        return Ok(TurbulentFluxDiagnostics {
            fluxes: TurbulentFluxes {
                sensible_heat: EnergyFluxWattsPerSquareMeter::try_new(0.0)?,
                latent_heat: EnergyFluxWattsPerSquareMeter::try_new(0.0)?,
                mass_flux: MassFluxKilogramsPerSquareMeterSecond::try_new(0.0)?,
                iterations: 0,
                obukhov_length_m: None,
            },
            termination_status: TurbulentTerminationStatus::ZeroWind,
            stability_class: TurbulentStabilityClass::ZeroWind,
            momentum_stability_correction: 0.0,
            sensible_stability_correction: 0.0,
            latent_stability_correction: 0.0,
            displacement_height_m: None,
            log_momentum: None,
            log_sensible: None,
            log_latent: None,
            friction_velocity_m_s: 0.0,
            sensible_exchange_velocity_m_s: None,
            latent_exchange_velocity_m_s: None,
            air_density_kg_m3: None,
            air_potential_temperature_k: None,
            surface_temperature_k: None,
            specific_humidity_air_kg_kg: None,
            specific_humidity_surface_kg_kg: None,
            latent_heat_j_kg: None,
        });
    }

    let state = TurbulentState::initial(inputs)?;
    if (state.air_potential_temperature_k - state.surface_temperature_k).abs() <= f64::EPSILON {
        return state.finish(
            0,
            None,
            TurbulentTerminationStatus::InitialPotentialTemperatureNeutral,
            TurbulentStabilityClass::Neutral,
        );
    }

    iterate_turbulent_fluxes(inputs, state)
}

/// Convert latent heat flux to vapor mass flux using the supplied latent heat.
///
/// # Errors
///
/// Returns a typed error if the computed mass flux is not finite.
pub fn mass_flux_from_latent_heat_flux(
    latent_heat_flux: EnergyFluxWattsPerSquareMeter,
    latent_heat: LatentHeatJoulesPerKilogram,
) -> Result<MassFluxKilogramsPerSquareMeterSecond, MeteorologyError> {
    MassFluxKilogramsPerSquareMeterSecond::try_new(
        latent_heat_flux.as_watts_per_square_meter() / latent_heat.as_joules_per_kilogram(),
    )
}

/// Convert vapor mass flux to latent heat flux using the supplied latent heat.
///
/// # Errors
///
/// Returns a typed error if the computed energy flux is not finite.
pub fn latent_heat_flux_from_mass_flux(
    mass_flux: MassFluxKilogramsPerSquareMeterSecond,
    latent_heat: LatentHeatJoulesPerKilogram,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    EnergyFluxWattsPerSquareMeter::try_new(
        mass_flux.as_kilograms_per_square_meter_second() * latent_heat.as_joules_per_kilogram(),
    )
}

/// Advected heat from rain and snowfall mass fluxes.
///
/// # Errors
///
/// Returns a typed error if temperature conversion or the output flux fails.
pub fn precipitation_advected_heat_flux(
    inputs: PrecipitationAdvectedHeatInputs,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    let rain_heat = heat_storage_flux(
        specific_heat_water(inputs.rain_temperature)?,
        inputs.rain_mass_flux.as_kilograms_per_square_meter_second(),
        inputs.rain_temperature.as_celsius() - inputs.surface_temperature.as_celsius(),
    );
    let snow_heat = heat_storage_flux(
        specific_heat_ice(inputs.snow_temperature)?,
        inputs.snow_mass_flux.as_kilograms_per_square_meter_second(),
        inputs.snow_temperature.as_celsius() - inputs.surface_temperature.as_celsius(),
    );
    EnergyFluxWattsPerSquareMeter::try_new(rain_heat + snow_heat)
}

/// Sum the surface energy-balance terms.
///
/// # Errors
///
/// Returns a typed error if the computed sum is not finite.
pub fn surface_energy_balance(
    terms: SurfaceEnergyBalanceTerms,
) -> Result<EnergyFluxWattsPerSquareMeter, MeteorologyError> {
    EnergyFluxWattsPerSquareMeter::try_new(
        terms.net_radiation.as_watts_per_square_meter()
            + terms.sensible_heat.as_watts_per_square_meter()
            + terms.latent_heat.as_watts_per_square_meter()
            + terms.conduction.as_watts_per_square_meter()
            + terms.advected_heat.as_watts_per_square_meter(),
    )
}

/// Saturation vapor pressure over ice at/below freezing and water above.
///
/// # Errors
///
/// Returns a typed error if temperature conversion or pressure construction
/// fails.
pub fn saturation_vapor_pressure_snobal_pa(
    temperature: TemperatureCelsius,
) -> Result<PressurePascals, MeteorologyError> {
    let temperature_kelvin = libsnobal_kelvin(temperature)?;
    if temperature.as_celsius() <= 0.0 {
        saturation_vapor_pressure_ice_pa(temperature_kelvin)
    } else {
        saturation_vapor_pressure_water_pa(temperature_kelvin)
    }
}

/// Latent heat of vaporization, or sublimation at/below freezing.
///
/// # Errors
///
/// Returns a typed error if the temperature or output latent heat is invalid.
pub fn latent_heat_for_surface_temperature(
    temperature: TemperatureCelsius,
) -> Result<LatentHeatJoulesPerKilogram, MeteorologyError> {
    let temperature_kelvin = libsnobal_kelvin(temperature)?;
    let value = latent_heat_vaporization_j_kg(temperature_kelvin);
    if temperature.as_celsius() <= 0.0 {
        LatentHeatJoulesPerKilogram::try_new(value + latent_heat_fusion_j_kg(temperature_kelvin))
    } else {
        LatentHeatJoulesPerKilogram::try_new(value)
    }
}

/// Specific heat of rain water.
///
/// # Errors
///
/// Returns a typed error if the temperature or output heat capacity is invalid.
pub fn specific_heat_water(
    temperature: TemperatureCelsius,
) -> Result<SpecificHeatCapacityJoulesPerKilogramKelvin, MeteorologyError> {
    let temperature_kelvin = libsnobal_kelvin(temperature)?;
    SpecificHeatCapacityJoulesPerKilogramKelvin::try_new(
        SPECIFIC_HEAT_WATER_0C_J_KG_K - 2.55 * (temperature_kelvin - LIBSNOBAL_FREEZE_K),
    )
}

/// Specific heat of ice/snow.
///
/// # Errors
///
/// Returns a typed error if the temperature or output heat capacity is invalid.
pub fn specific_heat_ice(
    temperature: TemperatureCelsius,
) -> Result<SpecificHeatCapacityJoulesPerKilogramKelvin, MeteorologyError> {
    let temperature_kelvin = libsnobal_kelvin(temperature)?;
    SpecificHeatCapacityJoulesPerKilogramKelvin::try_new(
        // UNIT-CONVERSION-ALLOW: mm_m_scale retained SNOBAL calorie-to-kilogram conversion.
        CALORIE_TO_JOULE * (0.024_928 + 0.001_76 * temperature_kelvin) / 0.001,
    )
}

fn iterate_turbulent_fluxes(
    inputs: TurbulentFluxInputs,
    mut state: TurbulentState,
) -> Result<TurbulentFluxDiagnostics, MeteorologyError> {
    let mut obukhov_length = f64::INFINITY;
    let mut last_delta = f64::INFINITY;

    for iteration in 1..=inputs.options.max_iterations {
        let last_obukhov_length = obukhov_length;
        let stability_buoyancy = state.sensible_heat_w_m2
            / (state.air_potential_temperature_k * SPECIFIC_HEAT_AIR_J_KG_K)
            + 0.61 * state.mass_flux_kg_m2_s;
        if stability_buoyancy == 0.0 {
            let stability_class = state.stability_class()?;
            return state.finish(
                iteration,
                None,
                TurbulentTerminationStatus::IterativeZeroBuoyancy,
                stability_class,
            );
        }

        obukhov_length = state.friction_velocity_m_s.powi(3) * state.air_density_kg_m3
            / (VON_KARMAN * GRAVITY_M_S2 * stability_buoyancy);
        if !obukhov_length.is_finite() || obukhov_length == 0.0 {
            return state.finish(
                iteration,
                None,
                TurbulentTerminationStatus::IterativeInvalidObukhov,
                TurbulentStabilityClass::IndeterminateObukhov,
            );
        }

        state.recompute_with_obukhov_length(inputs, obukhov_length);
        last_delta = last_obukhov_length - obukhov_length;
        if last_delta.abs() <= inputs.options.convergence_tolerance
            || (last_delta / obukhov_length).abs() <= inputs.options.convergence_tolerance
        {
            let (status, stability_class) = if obukhov_length.is_sign_positive() {
                (
                    TurbulentTerminationStatus::ConvergedStable,
                    TurbulentStabilityClass::Stable,
                )
            } else {
                (
                    TurbulentTerminationStatus::ConvergedUnstable,
                    TurbulentStabilityClass::Unstable,
                )
            };
            return state.finish(iteration, Some(obukhov_length), status, stability_class);
        }
    }

    Err(MeteorologyError::TurbulentTransferDidNotConverge {
        iterations: inputs.options.max_iterations,
        last_obukhov_length_m: obukhov_length,
        last_delta_m: last_delta,
    })
}

#[derive(Debug, Clone, Copy)]
struct TurbulentState {
    displacement_height_m: f64,
    log_momentum: f64,
    log_sensible: f64,
    log_latent: f64,
    specific_humidity_air: f64,
    specific_humidity_surface: f64,
    air_potential_temperature_k: f64,
    surface_temperature_k: f64,
    air_density_kg_m3: f64,
    friction_velocity_m_s: f64,
    momentum_stability_correction: f64,
    sensible_stability_correction: f64,
    latent_stability_correction: f64,
    sensible_exchange_velocity_m_s: f64,
    latent_exchange_velocity_m_s: f64,
    mass_flux_kg_m2_s: f64,
    sensible_heat_w_m2: f64,
    latent_heat_j_kg: f64,
}

impl TurbulentState {
    fn initial(inputs: TurbulentFluxInputs) -> Result<Self, MeteorologyError> {
        let displacement_height = displacement_height(inputs.roughness_length);
        let log_momentum = ((inputs.wind_speed_height.as_meters() - displacement_height)
            / inputs.roughness_length.as_meters())
        .ln();
        let log_sensible = ((inputs.air_temperature_height.as_meters() - displacement_height)
            / inputs.roughness_length.as_meters())
        .ln();
        let log_latent = ((inputs.vapor_pressure_height.as_meters() - displacement_height)
            / inputs.roughness_length.as_meters())
        .ln();
        let air_temperature_k = libsnobal_kelvin(inputs.air_temperature)?;
        let surface_temperature_k = libsnobal_kelvin(inputs.surface_temperature)?;
        let air_potential_temperature_k = air_temperature_k
            + DRY_ADIABATIC_LAPSE_RATE_K_M * inputs.air_temperature_height.as_meters();
        let air_density_kg_m3 =
            air_density(inputs, air_potential_temperature_k, surface_temperature_k);
        let specific_humidity_air = specific_humidity(
            inputs.air_vapor_pressure.as_pascals(),
            inputs.air_pressure.as_pascals(),
        );
        let specific_humidity_surface = specific_humidity(
            inputs.surface_vapor_pressure.as_pascals(),
            inputs.air_pressure.as_pascals(),
        );
        let latent_heat_j_kg = latent_heat_for_surface_temperature(inputs.surface_temperature)?
            .as_joules_per_kilogram();
        let mut state = Self {
            displacement_height_m: displacement_height,
            log_momentum,
            log_sensible,
            log_latent,
            specific_humidity_air,
            specific_humidity_surface,
            air_potential_temperature_k,
            surface_temperature_k,
            air_density_kg_m3,
            friction_velocity_m_s: 0.0,
            momentum_stability_correction: 0.0,
            sensible_stability_correction: 0.0,
            latent_stability_correction: 0.0,
            sensible_exchange_velocity_m_s: 0.0,
            latent_exchange_velocity_m_s: 0.0,
            mass_flux_kg_m2_s: 0.0,
            sensible_heat_w_m2: 0.0,
            latent_heat_j_kg,
        };
        state.recompute_neutral(inputs);
        Ok(state)
    }

    fn recompute_neutral(&mut self, inputs: TurbulentFluxInputs) {
        self.friction_velocity_m_s =
            VON_KARMAN * inputs.wind_speed.as_meters_per_second() / self.log_momentum;
        self.recompute_fluxes(0.0, 0.0, 0.0);
    }

    fn recompute_with_obukhov_length(&mut self, inputs: TurbulentFluxInputs, obukhov_length: f64) {
        let momentum_correction = psi(
            inputs.wind_speed_height.as_meters() / obukhov_length,
            PsiKind::Momentum,
        );
        self.friction_velocity_m_s = VON_KARMAN * inputs.wind_speed.as_meters_per_second()
            / (self.log_momentum - momentum_correction);
        self.recompute_fluxes(
            momentum_correction,
            psi(
                inputs.air_temperature_height.as_meters() / obukhov_length,
                PsiKind::Sensible,
            ),
            psi(
                inputs.vapor_pressure_height.as_meters() / obukhov_length,
                PsiKind::Latent,
            ),
        );
    }

    fn recompute_fluxes(
        &mut self,
        momentum_correction: f64,
        sensible_correction: f64,
        latent_correction: f64,
    ) {
        self.momentum_stability_correction = momentum_correction;
        self.sensible_stability_correction = sensible_correction;
        self.latent_stability_correction = latent_correction;
        self.sensible_exchange_velocity_m_s =
            VON_KARMAN * self.friction_velocity_m_s / (self.log_sensible - sensible_correction);
        self.latent_exchange_velocity_m_s =
            VON_KARMAN * self.friction_velocity_m_s / (self.log_latent - latent_correction);
        let factor = VON_KARMAN * self.friction_velocity_m_s * self.air_density_kg_m3;
        self.mass_flux_kg_m2_s = (self.specific_humidity_air - self.specific_humidity_surface)
            * factor
            / (self.log_latent - latent_correction);
        self.sensible_heat_w_m2 = (self.air_potential_temperature_k - self.surface_temperature_k)
            * factor
            * SPECIFIC_HEAT_AIR_J_KG_K
            / (self.log_sensible - sensible_correction);
    }

    fn finish(
        self,
        iterations: usize,
        obukhov_length_m: Option<f64>,
        termination_status: TurbulentTerminationStatus,
        stability_class: TurbulentStabilityClass,
    ) -> Result<TurbulentFluxDiagnostics, MeteorologyError> {
        let fluxes = TurbulentFluxes {
            sensible_heat: EnergyFluxWattsPerSquareMeter::try_new(self.sensible_heat_w_m2)?,
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(
                self.latent_heat_j_kg * self.mass_flux_kg_m2_s,
            )?,
            mass_flux: MassFluxKilogramsPerSquareMeterSecond::try_new(self.mass_flux_kg_m2_s)?,
            iterations,
            obukhov_length_m,
        };
        Ok(TurbulentFluxDiagnostics {
            fluxes,
            termination_status,
            stability_class,
            momentum_stability_correction: self.momentum_stability_correction,
            sensible_stability_correction: self.sensible_stability_correction,
            latent_stability_correction: self.latent_stability_correction,
            displacement_height_m: Some(self.displacement_height_m),
            log_momentum: Some(self.log_momentum),
            log_sensible: Some(self.log_sensible),
            log_latent: Some(self.log_latent),
            friction_velocity_m_s: self.friction_velocity_m_s,
            sensible_exchange_velocity_m_s: Some(self.sensible_exchange_velocity_m_s),
            latent_exchange_velocity_m_s: Some(self.latent_exchange_velocity_m_s),
            air_density_kg_m3: Some(self.air_density_kg_m3),
            air_potential_temperature_k: Some(self.air_potential_temperature_k),
            surface_temperature_k: Some(self.surface_temperature_k),
            specific_humidity_air_kg_kg: Some(self.specific_humidity_air),
            specific_humidity_surface_kg_kg: Some(self.specific_humidity_surface),
            latent_heat_j_kg: Some(self.latent_heat_j_kg),
        })
    }

    fn stability_class(&self) -> Result<TurbulentStabilityClass, MeteorologyError> {
        let corrections = [
            self.momentum_stability_correction,
            self.sensible_stability_correction,
            self.latent_stability_correction,
        ];
        if corrections.iter().any(|value| !value.is_finite()) {
            return Err(MeteorologyError::OutOfAuthority {
                quantity: "turbulent_stability_correction",
                value: f64::NAN,
                minimum: f64::NEG_INFINITY,
                maximum: f64::INFINITY,
            });
        }
        if corrections.iter().all(|value| *value == 0.0) {
            return Ok(TurbulentStabilityClass::Neutral);
        }
        if corrections.iter().all(|value| *value <= 0.0) {
            return Ok(TurbulentStabilityClass::Stable);
        }
        if corrections.iter().all(|value| *value >= 0.0) {
            return Ok(TurbulentStabilityClass::Unstable);
        }
        Err(MeteorologyError::OutOfAuthority {
            quantity: "turbulent_stability_correction",
            value: f64::NAN,
            minimum: 0.0,
            maximum: 0.0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum PsiKind {
    Momentum,
    Sensible,
    Latent,
}

fn psi(mut zeta: f64, kind: PsiKind) -> f64 {
    if zeta > 0.0 {
        if zeta > 1.0 {
            zeta = 1.0;
        }
        return -BETA_STABLE * zeta;
    }
    if zeta < 0.0 {
        let height_function = (1.0 - BETA_UNSTABLE * zeta).sqrt().sqrt();
        return match kind {
            PsiKind::Momentum => {
                2.0 * f64::midpoint(1.0, height_function).ln()
                    + f64::midpoint(1.0, height_function.powi(2)).ln()
                    - 2.0 * height_function.atan()
                    + std::f64::consts::FRAC_PI_2
            }
            PsiKind::Sensible | PsiKind::Latent => {
                2.0 * f64::midpoint(1.0, height_function.powi(2)).ln()
            }
        };
    }
    0.0
}

fn validate_turbulent_inputs(inputs: TurbulentFluxInputs) -> Result<(), MeteorologyError> {
    TurbulentTransferOptions::try_new(
        inputs.options.max_iterations,
        inputs.options.convergence_tolerance,
    )?;
    let pressure = inputs.air_pressure.as_pascals();
    for (quantity, vapor_pressure) in [
        (
            "air_vapor_pressure_pa",
            inputs.air_vapor_pressure.as_pascals(),
        ),
        (
            "surface_vapor_pressure_pa",
            inputs.surface_vapor_pressure.as_pascals(),
        ),
    ] {
        if vapor_pressure >= pressure {
            return Err(BoundaryError::AboveMaximum {
                boundary: quantity,
                value: vapor_pressure,
                maximum: pressure,
            }
            .into());
        }
    }

    let displacement_height = displacement_height(inputs.roughness_length);
    for (quantity, height) in [
        (
            "air_temperature_height_m",
            inputs.air_temperature_height.as_meters(),
        ),
        (
            "vapor_pressure_height_m",
            inputs.vapor_pressure_height.as_meters(),
        ),
        ("wind_speed_height_m", inputs.wind_speed_height.as_meters()),
    ] {
        if height <= inputs.roughness_length.as_meters() {
            return Err(BoundaryError::BelowMinimum {
                boundary: quantity,
                value: height,
                minimum: inputs.roughness_length.as_meters(),
            }
            .into());
        }
        if height <= displacement_height {
            return Err(BoundaryError::BelowMinimum {
                boundary: quantity,
                value: height,
                minimum: displacement_height,
            }
            .into());
        }
    }
    Ok(())
}

fn displacement_height(roughness_length: PositiveLengthMeters) -> f64 {
    2.0 * PAESCHKE * roughness_length.as_meters() / 3.0
}

fn air_density(
    inputs: TurbulentFluxInputs,
    air_potential_temperature_k: f64,
    surface_temperature_k: f64,
) -> f64 {
    gas_density(
        inputs.air_pressure.as_pascals(),
        MOLAR_MASS_DRY_AIR_KG_PER_KMOL,
        virtual_temperature(
            (air_potential_temperature_k * surface_temperature_k).sqrt(),
            (inputs.air_vapor_pressure.as_pascals() * inputs.surface_vapor_pressure.as_pascals())
                .sqrt(),
            inputs.air_pressure.as_pascals(),
        ),
    )
}

fn gas_density(pressure_pa: f64, molecular_mass_kg_per_kmol: f64, temperature_k: f64) -> f64 {
    pressure_pa * molecular_mass_kg_per_kmol / (UNIVERSAL_GAS_CONSTANT_J_KMOL_K * temperature_k)
}

fn virtual_temperature(temperature_k: f64, vapor_pressure_pa: f64, pressure_pa: f64) -> f64 {
    temperature_k
        / (1.0
            - (1.0 - MOLAR_MASS_WATER_KG_PER_KMOL / MOLAR_MASS_DRY_AIR_KG_PER_KMOL)
                * (vapor_pressure_pa / pressure_pa))
}

fn specific_humidity(vapor_pressure_pa: f64, pressure_pa: f64) -> f64 {
    vapor_pressure_pa * MOLAR_MASS_WATER_KG_PER_KMOL
        / (MOLAR_MASS_DRY_AIR_KG_PER_KMOL * pressure_pa
            + vapor_pressure_pa * (MOLAR_MASS_WATER_KG_PER_KMOL - MOLAR_MASS_DRY_AIR_KG_PER_KMOL))
}

fn saturation_vapor_pressure_ice_pa(
    temperature_kelvin: f64,
) -> Result<PressurePascals, MeteorologyError> {
    let log_10 = 10.0_f64.ln();
    let exponent = -9.09718 * ((LIBSNOBAL_FREEZE_K / temperature_kelvin) - 1.0)
        - 3.56654 * (LIBSNOBAL_FREEZE_K / temperature_kelvin).ln() / log_10
        + 0.876_793 * (1.0 - (temperature_kelvin / LIBSNOBAL_FREEZE_K))
        + 6.1071_f64.log10();
    // UNIT-CONVERSION-ALLOW: cm_m_scale retained SNOBAL millibar-to-pascal conversion.
    PressurePascals::try_new(10.0_f64.powf(exponent) * 100.0)
}

fn saturation_vapor_pressure_water_pa(
    temperature_kelvin: f64,
) -> Result<PressurePascals, MeteorologyError> {
    let log_10 = 10.0_f64.ln();
    let exponent = -7.90298 * (BOILING_POINT_K / temperature_kelvin - 1.0)
        + 5.02808 * (BOILING_POINT_K / temperature_kelvin).ln() / log_10
        - 1.3816e-7 * (10.0_f64.powf(11.344 * (1.0 - temperature_kelvin / BOILING_POINT_K)) - 1.0)
        + 8.1328e-3
            * (10.0_f64.powf(-3.49149 * (BOILING_POINT_K / temperature_kelvin - 1.0)) - 1.0)
        + SEA_LEVEL_PRESSURE_PA.log10();
    PressurePascals::try_new(10.0_f64.powf(exponent))
}

fn latent_heat_vaporization_j_kg(temperature_kelvin: f64) -> f64 {
    2.5e6 - 2.955_73e3 * (temperature_kelvin - LIBSNOBAL_FREEZE_K)
}

fn latent_heat_fusion_j_kg(temperature_kelvin: f64) -> f64 {
    3.336e5 + 1.6667e2 * (LIBSNOBAL_FREEZE_K - temperature_kelvin)
}

fn heat_storage_flux(
    specific_heat: SpecificHeatCapacityJoulesPerKilogramKelvin,
    mass_flux_kg_m2_s: f64,
    temperature_difference_k: f64,
) -> f64 {
    specific_heat.as_joules_per_kilogram_kelvin() * mass_flux_kg_m2_s * temperature_difference_k
}

fn libsnobal_kelvin(temperature: TemperatureCelsius) -> Result<f64, MeteorologyError> {
    let value_c = temperature.as_celsius();
    if value_c <= -LIBSNOBAL_FREEZE_K {
        return Err(MeteorologyError::BelowAbsoluteZero {
            quantity: "temperature_c",
            value_c,
        });
    }
    Ok(value_c + LIBSNOBAL_FREEZE_K)
}

fn validate_finite(boundary: &'static str, value: f64) -> Result<(), MeteorologyError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(BoundaryError::NonFinite { boundary, value }.into())
    }
}

fn validate_non_negative(boundary: &'static str, value: f64) -> Result<(), MeteorologyError> {
    validate_finite(boundary, value)?;
    if value < 0.0 {
        return Err(BoundaryError::BelowMinimum {
            boundary,
            value,
            minimum: 0.0,
        }
        .into());
    }
    Ok(())
}

fn validate_positive(boundary: &'static str, value: f64) -> Result<(), MeteorologyError> {
    validate_finite(boundary, value)?;
    if value <= 0.0 {
        return Err(MeteorologyError::NonPositive {
            quantity: boundary,
            value,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}"
        );
    }

    fn temp(value_c: f64) -> TemperatureCelsius {
        TemperatureCelsius::try_new(value_c).expect("valid temperature")
    }

    #[test]
    fn net_all_wave_radiation_closes_from_shortwave_and_longwave_terms() {
        let inputs = AllWaveRadiationInputs {
            incoming_shortwave: RadiativeFluxWattsPerSquareMeter::try_new(800.0)
                .expect("valid shortwave"),
            albedo: FractionUnitInterval::try_new(0.82).expect("valid albedo"),
            incoming_longwave: RadiativeFluxWattsPerSquareMeter::try_new(300.0)
                .expect("valid longwave"),
            emissivity: FractionUnitInterval::try_new(0.98).expect("valid emissivity"),
            surface_temperature: temp(-5.0),
        };

        let shortwave = net_shortwave_radiation(inputs.incoming_shortwave, inputs.albedo)
            .expect("valid shortwave");
        let longwave = net_longwave_radiation(
            inputs.incoming_longwave,
            inputs.emissivity,
            inputs.surface_temperature,
        )
        .expect("valid longwave");
        let all_wave = net_all_wave_radiation(inputs).expect("valid all-wave radiation");

        assert_close(shortwave.as_watts_per_square_meter(), 144.0, 1.0e-12);
        assert_close(longwave.as_watts_per_square_meter(), 0.0, 100.0);
        assert_close(
            all_wave.as_watts_per_square_meter(),
            shortwave.as_watts_per_square_meter() + longwave.as_watts_per_square_meter(),
            1.0e-12,
        );
    }

    #[test]
    fn conductive_heat_flux_uses_series_resistance_and_sign_to_surface() {
        let flux = conductive_heat_flux(
            ThermalConductivityWattsPerMeterKelvin::try_new(0.3).expect("valid k"),
            ThermalConductivityWattsPerMeterKelvin::try_new(1.65).expect("valid k"),
            temp(-2.0),
            temp(1.0),
            PositiveLengthMeters::try_new(0.1).expect("valid thickness"),
            PositiveLengthMeters::try_new(0.2).expect("valid thickness"),
        )
        .expect("valid conduction");

        assert_close(flux.as_watts_per_square_meter(), 13.2, 1.0e-12);
    }

    #[test]
    fn snobal_snow_conductivity_uses_density_temperature_and_pressure() {
        let sea_level = snow_effective_thermal_conductivity_snobal(
            300.0,
            temp(-5.0),
            PressurePascals::try_new(SEA_LEVEL_PRESSURE_PA).expect("valid pressure"),
        )
        .expect("valid effective snow conductivity")
        .as_watts_per_meter_kelvin();
        let high_elevation = snow_effective_thermal_conductivity_snobal(
            300.0,
            temp(-5.0),
            PressurePascals::try_new(80_000.0).expect("valid pressure"),
        )
        .expect("valid high-elevation effective conductivity")
        .as_watts_per_meter_kelvin();
        let denser = snow_effective_thermal_conductivity_snobal(
            500.0,
            temp(-5.0),
            PressurePascals::try_new(SEA_LEVEL_PRESSURE_PA).expect("valid pressure"),
        )
        .expect("valid dense-snow effective conductivity")
        .as_watts_per_meter_kelvin();

        assert!(sea_level > 0.0);
        assert_close(sea_level, 0.356_693_429_416_186_5, 1.0e-12);
        assert!(high_elevation > sea_level);
        assert!(denser > sea_level);
    }

    #[test]
    fn advected_heat_from_precipitation_matches_heat_storage_terms() {
        let flux = precipitation_advected_heat_flux(PrecipitationAdvectedHeatInputs {
            rain_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond::try_new(0.001)
                .expect("valid rain"),
            rain_temperature: temp(2.0),
            snow_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond::try_new(0.0005)
                .expect("valid snow"),
            snow_temperature: temp(-1.0),
            surface_temperature: temp(0.0),
        })
        .expect("valid advection");

        assert_close(
            flux.as_watts_per_square_meter(),
            7.370_274_231_920_216,
            1.0e-12,
        );
    }

    #[test]
    fn latent_heat_and_mass_flux_convert_bidirectionally() {
        let latent_heat =
            LatentHeatJoulesPerKilogram::try_new(2_835_000.0).expect("valid latent heat");
        let mass_flux =
            MassFluxKilogramsPerSquareMeterSecond::try_new(-1.0e-5).expect("valid mass flux");
        let energy = latent_heat_flux_from_mass_flux(mass_flux, latent_heat)
            .expect("valid latent heat flux");
        let round_trip =
            mass_flux_from_latent_heat_flux(energy, latent_heat).expect("valid mass flux");

        assert_close(energy.as_watts_per_square_meter(), -28.35, 1.0e-12);
        assert_close(
            round_trip.as_kilograms_per_square_meter_second(),
            -1.0e-5,
            1.0e-15,
        );
    }

    #[test]
    fn turbulent_fluxes_include_potential_temperature_height_correction() {
        let vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(0.0)).expect("valid saturation pressure");
        let fluxes = turbulent_fluxes_monin_obukhov(TurbulentFluxInputs {
            air_pressure: PressurePascals::try_new(85_000.0).expect("valid air pressure"),
            air_temperature: temp(0.0),
            surface_temperature: temp(0.0),
            air_vapor_pressure: vapor_pressure,
            surface_vapor_pressure: vapor_pressure,
            air_temperature_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            vapor_pressure_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            wind_speed: LinearRateMetersPerSecond::try_new(3.0).expect("valid wind"),
            wind_speed_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            roughness_length: PositiveLengthMeters::try_new(0.005).expect("valid roughness"),
            options: TurbulentTransferOptions::default(),
        })
        .expect("valid turbulent flux");

        assert_close(
            fluxes.sensible_heat.as_watts_per_square_meter(),
            0.298_608_287_488_169_8,
            1.0e-12,
        );
        assert_close(fluxes.latent_heat.as_watts_per_square_meter(), 0.0, 1.0e-12);
        assert_close(
            fluxes.mass_flux.as_kilograms_per_square_meter_second(),
            0.0,
            1.0e-18,
        );
        assert!(fluxes.iterations > 0);
    }

    #[test]
    fn turbulent_fluxes_apply_stability_and_close_latent_mass_coupling() {
        let air_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(1.0)).expect("valid saturation pressure");
        let surface_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(-2.0)).expect("valid saturation pressure");
        let fluxes = turbulent_fluxes_monin_obukhov(TurbulentFluxInputs {
            air_pressure: PressurePascals::try_new(85_000.0).expect("valid air pressure"),
            air_temperature: temp(1.0),
            surface_temperature: temp(-2.0),
            air_vapor_pressure,
            surface_vapor_pressure,
            air_temperature_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            vapor_pressure_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            wind_speed: LinearRateMetersPerSecond::try_new(4.0).expect("valid wind"),
            wind_speed_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            roughness_length: PositiveLengthMeters::try_new(0.005).expect("valid roughness"),
            options: TurbulentTransferOptions::default(),
        })
        .expect("valid turbulent flux");
        let latent_heat = latent_heat_for_surface_temperature(temp(-2.0))
            .expect("valid latent heat")
            .as_joules_per_kilogram();

        assert!(fluxes.iterations > 0);
        assert!(fluxes.obukhov_length_m.is_some());
        assert!(fluxes.sensible_heat.as_watts_per_square_meter() > 0.0);
        assert!(fluxes.latent_heat.as_watts_per_square_meter() > 0.0);
        assert_close(
            fluxes.latent_heat.as_watts_per_square_meter(),
            fluxes.mass_flux.as_kilograms_per_square_meter_second() * latent_heat,
            1.0e-12,
        );
    }

    #[test]
    fn turbulent_diagnostics_share_exact_flux_solver_results() {
        let air_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(-1.0)).expect("valid air vapor pressure");
        let surface_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(-6.0)).expect("valid surface vapor pressure");
        let base = TurbulentFluxInputs {
            air_pressure: PressurePascals::try_new(80_000.0).expect("valid pressure"),
            air_temperature: temp(-1.0),
            surface_temperature: temp(-6.0),
            air_vapor_pressure,
            surface_vapor_pressure,
            air_temperature_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            vapor_pressure_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            wind_speed: LinearRateMetersPerSecond::try_new(3.5).expect("valid wind"),
            wind_speed_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            roughness_length: PositiveLengthMeters::try_new(0.005).expect("valid roughness"),
            options: TurbulentTransferOptions::default(),
        };
        for (inputs, expected_status) in [
            (base, TurbulentTerminationStatus::ConvergedStable),
            (
                TurbulentFluxInputs {
                    wind_speed: LinearRateMetersPerSecond::try_new(0.0).expect("valid zero wind"),
                    ..base
                },
                TurbulentTerminationStatus::ZeroWind,
            ),
            (
                TurbulentFluxInputs {
                    air_temperature: temp(-10.0),
                    surface_temperature: temp(-2.0),
                    ..base
                },
                TurbulentTerminationStatus::ConvergedUnstable,
            ),
            (
                TurbulentFluxInputs {
                    air_temperature: temp(-2.0),
                    surface_temperature: temp(-2.0 + DRY_ADIABATIC_LAPSE_RATE_K_M * 5.0),
                    ..base
                },
                TurbulentTerminationStatus::InitialPotentialTemperatureNeutral,
            ),
        ] {
            let legacy = turbulent_fluxes_monin_obukhov(inputs).expect("legacy fluxes");
            let diagnostic =
                turbulent_fluxes_monin_obukhov_with_diagnostics(inputs).expect("diagnostic fluxes");
            assert_eq!(
                legacy.sensible_heat.as_watts_per_square_meter().to_bits(),
                diagnostic
                    .fluxes
                    .sensible_heat
                    .as_watts_per_square_meter()
                    .to_bits()
            );
            assert_eq!(
                legacy.latent_heat.as_watts_per_square_meter().to_bits(),
                diagnostic
                    .fluxes
                    .latent_heat
                    .as_watts_per_square_meter()
                    .to_bits()
            );
            assert_eq!(
                legacy
                    .mass_flux
                    .as_kilograms_per_square_meter_second()
                    .to_bits(),
                diagnostic
                    .fluxes
                    .mass_flux
                    .as_kilograms_per_square_meter_second()
                    .to_bits()
            );
            assert_eq!(legacy.iterations, diagnostic.fluxes.iterations);
            assert_eq!(
                legacy.obukhov_length_m.map(f64::to_bits),
                diagnostic.fluxes.obukhov_length_m.map(f64::to_bits)
            );
            assert_eq!(diagnostic.termination_status, expected_status);
        }

        let nonconvergent = TurbulentFluxInputs {
            options: TurbulentTransferOptions {
                max_iterations: 1,
                convergence_tolerance: f64::MIN_POSITIVE,
            },
            ..base
        };
        let legacy_error = turbulent_fluxes_monin_obukhov(nonconvergent)
            .expect_err("one iteration must not converge");
        let diagnostic_error = turbulent_fluxes_monin_obukhov_with_diagnostics(nonconvergent)
            .expect_err("diagnostic solver must share nonconvergence");
        assert_eq!(format!("{legacy_error:?}"), format!("{diagnostic_error:?}"));
    }

    #[test]
    fn turbulent_diagnostic_retains_rare_iterative_exit_taxonomy() {
        let air_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(-1.0)).expect("valid air vapor pressure");
        let surface_vapor_pressure =
            saturation_vapor_pressure_snobal_pa(temp(-6.0)).expect("valid surface vapor pressure");
        let inputs = TurbulentFluxInputs {
            air_pressure: PressurePascals::try_new(80_000.0).expect("valid pressure"),
            air_temperature: temp(-1.0),
            surface_temperature: temp(-6.0),
            air_vapor_pressure,
            surface_vapor_pressure,
            air_temperature_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            vapor_pressure_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            wind_speed: LinearRateMetersPerSecond::try_new(3.5).expect("valid wind"),
            wind_speed_height: PositiveLengthMeters::try_new(5.0).expect("valid height"),
            roughness_length: PositiveLengthMeters::try_new(0.005).expect("valid roughness"),
            options: TurbulentTransferOptions::default(),
        };

        let mut zero_buoyancy = TurbulentState::initial(inputs).expect("initial state");
        zero_buoyancy.sensible_heat_w_m2 = 0.0;
        zero_buoyancy.mass_flux_kg_m2_s = 0.0;
        let zero = iterate_turbulent_fluxes(inputs, zero_buoyancy)
            .expect("zero buoyancy is a successful exact exit");
        assert_eq!(
            zero.termination_status,
            TurbulentTerminationStatus::IterativeZeroBuoyancy
        );
        assert_eq!(zero.fluxes.iterations, 1);
        assert!(zero.fluxes.obukhov_length_m.is_none());

        let mut invalid_obukhov = TurbulentState::initial(inputs).expect("initial state");
        invalid_obukhov.friction_velocity_m_s = f64::MAX;
        let invalid = iterate_turbulent_fluxes(inputs, invalid_obukhov)
            .expect("invalid Obukhov is a retained successful exit");
        assert_eq!(
            invalid.termination_status,
            TurbulentTerminationStatus::IterativeInvalidObukhov
        );
        assert_eq!(
            invalid.stability_class,
            TurbulentStabilityClass::IndeterminateObukhov
        );
        assert!(invalid.fluxes.obukhov_length_m.is_none());
    }

    #[test]
    fn surface_energy_balance_sums_all_terms() {
        let terms = SurfaceEnergyBalanceTerms {
            net_radiation: EnergyFluxWattsPerSquareMeter::try_new(120.0)
                .expect("valid net radiation"),
            sensible_heat: EnergyFluxWattsPerSquareMeter::try_new(10.0).expect("valid sensible"),
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(-25.0).expect("valid latent"),
            conduction: EnergyFluxWattsPerSquareMeter::try_new(2.5).expect("valid conduction"),
            advected_heat: EnergyFluxWattsPerSquareMeter::try_new(0.5).expect("valid advection"),
        };

        let total = surface_energy_balance(terms).expect("valid balance");
        assert_close(total.as_watts_per_square_meter(), 108.0, 1.0e-12);
    }

    #[test]
    fn domain_errors_reject_nonphysical_inputs() {
        assert!(matches!(
            RadiativeFluxWattsPerSquareMeter::try_new(-1.0),
            Err(MeteorologyError::Boundary(
                BoundaryError::BelowMinimum { .. }
            ))
        ));

        let error = turbulent_fluxes_monin_obukhov(TurbulentFluxInputs {
            air_pressure: PressurePascals::try_new(85_000.0).expect("valid air pressure"),
            air_temperature: temp(1.0),
            surface_temperature: temp(-2.0),
            air_vapor_pressure: PressurePascals::try_new(500.0).expect("valid vp"),
            surface_vapor_pressure: PressurePascals::try_new(500.0).expect("valid vp"),
            air_temperature_height: PositiveLengthMeters::try_new(0.01).expect("valid height"),
            vapor_pressure_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            wind_speed: LinearRateMetersPerSecond::try_new(1.0).expect("valid wind"),
            wind_speed_height: PositiveLengthMeters::try_new(2.0).expect("valid height"),
            roughness_length: PositiveLengthMeters::try_new(0.005).expect("valid roughness"),
            options: TurbulentTransferOptions::default(),
        })
        .expect_err("height below displacement plane must fail");

        assert!(matches!(
            error,
            MeteorologyError::Boundary(BoundaryError::BelowMinimum { .. })
        ));
    }

    #[test]
    fn canonical_snow_longwave_preserves_complementary_view_factors() {
        let inputs = SnowLongwaveInputs {
            air_temperature: temp(-2.0),
            surface_temperature: temp(-8.0),
            actual_vapor_pressure: PressurePascals::from_kilopascals(0.5)
                .expect("valid vapor pressure"),
            daily_solar_radiation_mj_m2: 5.0,
            daily_extraterrestrial_radiation_mj_m2: 10.0,
            daylight: true,
            canopy_cover: FractionUnitInterval::try_new(0.5).expect("valid cover"),
        };
        let fluxes = snow_longwave_dilley_unsworth(inputs).expect("canonical longwave");
        let rejected_air_temperature_emission = snow_longwave_dilley_unsworth(SnowLongwaveInputs {
            surface_temperature: inputs.air_temperature,
            ..inputs
        })
        .expect("air-temperature rejection candidate");
        assert_close(
            fluxes.sky_view_fraction.as_fraction(),
            0.5_f64.powf(1.6),
            1.0e-12,
        );
        assert!(fluxes.atmospheric_longwave.as_watts_per_square_meter() > 0.0);
        assert!(fluxes.subcanopy_longwave.as_watts_per_square_meter() > 0.0);
        assert!(fluxes.net_longwave.as_watts_per_square_meter().is_finite());
        assert!(
            (fluxes.outgoing_longwave.as_watts_per_square_meter()
                - rejected_air_temperature_emission
                    .outgoing_longwave
                    .as_watts_per_square_meter())
            .abs()
                > 1.0
        );
    }

    #[test]
    fn frozen_surface_saturation_rejects_the_water_surface_candidate() {
        let temperature = temp(-8.0);
        let ice_pressure = saturation_vapor_pressure_snobal_pa(temperature)
            .expect("ice saturation")
            .as_pascals();
        let water_pressure = saturation_vapor_pressure_water_pa(
            libsnobal_kelvin(temperature).expect("kelvin conversion"),
        )
        .expect("water saturation rejection candidate")
        .as_pascals();

        assert!((ice_pressure - water_pressure).abs() > 1.0);
    }

    #[test]
    fn canonical_snow_longwave_fails_closed_at_polar_night() {
        let result = snow_longwave_dilley_unsworth(SnowLongwaveInputs {
            air_temperature: temp(-15.0),
            surface_temperature: temp(-15.0),
            actual_vapor_pressure: PressurePascals::from_kilopascals(0.2)
                .expect("valid vapor pressure"),
            daily_solar_radiation_mj_m2: 0.0,
            daily_extraterrestrial_radiation_mj_m2: 0.0,
            daylight: false,
            canopy_cover: FractionUnitInterval::try_new(0.2).expect("valid cover"),
        });
        assert!(matches!(
            result,
            Err(MeteorologyError::CloudForcingUnavailable)
        ));
    }
}
