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
        Self::try_new(value_kpa * 1_000.0)
    }

    /// Raw value in pascals.
    #[must_use]
    pub const fn as_pascals(self) -> f64 {
        self.0
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
    validate_turbulent_inputs(inputs)?;
    if inputs.wind_speed.as_meters_per_second() == 0.0 {
        return Ok(TurbulentFluxes {
            sensible_heat: EnergyFluxWattsPerSquareMeter::try_new(0.0)?,
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(0.0)?,
            mass_flux: MassFluxKilogramsPerSquareMeterSecond::try_new(0.0)?,
            iterations: 0,
            obukhov_length_m: None,
        });
    }

    let state = TurbulentState::initial(inputs)?;
    if (state.air_potential_temperature_k - state.surface_temperature_k).abs() <= f64::EPSILON {
        return state.finish(0, None);
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
        CALORIE_TO_JOULE * (0.024_928 + 0.001_76 * temperature_kelvin) / 0.001,
    )
}

fn iterate_turbulent_fluxes(
    inputs: TurbulentFluxInputs,
    mut state: TurbulentState,
) -> Result<TurbulentFluxes, MeteorologyError> {
    let mut obukhov_length = f64::INFINITY;
    let mut last_delta = f64::INFINITY;

    for iteration in 1..=inputs.options.max_iterations {
        let last_obukhov_length = obukhov_length;
        let stability_buoyancy = state.sensible_heat_w_m2
            / (state.air_potential_temperature_k * SPECIFIC_HEAT_AIR_J_KG_K)
            + 0.61 * state.mass_flux_kg_m2_s;
        if stability_buoyancy == 0.0 {
            return state.finish(iteration, None);
        }

        obukhov_length = state.friction_velocity_m_s.powi(3) * state.air_density_kg_m3
            / (VON_KARMAN * GRAVITY_M_S2 * stability_buoyancy);
        if !obukhov_length.is_finite() || obukhov_length == 0.0 {
            return state.finish(iteration, None);
        }

        state.recompute_with_obukhov_length(inputs, obukhov_length);
        last_delta = last_obukhov_length - obukhov_length;
        if last_delta.abs() <= inputs.options.convergence_tolerance
            || (last_delta / obukhov_length).abs() <= inputs.options.convergence_tolerance
        {
            return state.finish(iteration, Some(obukhov_length));
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
    log_momentum: f64,
    log_sensible: f64,
    log_latent: f64,
    specific_humidity_air: f64,
    specific_humidity_surface: f64,
    air_potential_temperature_k: f64,
    surface_temperature_k: f64,
    air_density_kg_m3: f64,
    friction_velocity_m_s: f64,
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
            log_momentum,
            log_sensible,
            log_latent,
            specific_humidity_air,
            specific_humidity_surface,
            air_potential_temperature_k,
            surface_temperature_k,
            air_density_kg_m3,
            friction_velocity_m_s: 0.0,
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
        _momentum_correction: f64,
        sensible_correction: f64,
        latent_correction: f64,
    ) {
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
    ) -> Result<TurbulentFluxes, MeteorologyError> {
        Ok(TurbulentFluxes {
            sensible_heat: EnergyFluxWattsPerSquareMeter::try_new(self.sensible_heat_w_m2)?,
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(
                self.latent_heat_j_kg * self.mass_flux_kg_m2_s,
            )?,
            mass_flux: MassFluxKilogramsPerSquareMeterSecond::try_new(self.mass_flux_kg_m2_s)?,
            iterations,
            obukhov_length_m,
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
}
