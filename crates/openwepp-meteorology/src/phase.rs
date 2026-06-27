use openwepp_unit_boundary::{FractionUnitInterval, TemperatureCelsius};

use crate::error::MeteorologyError;
use crate::psychrometrics::{
    DiffusivitySquareMetersPerSecond, ThermalConductivityWattsPerMeterKelvin,
    VaporDensityKilogramsPerCubicMeter, actual_vapor_pressure_from_relative_humidity_kpa,
    celsius_to_kelvin, latent_heat_for_hydrometeor_temperature,
    molecular_diffusivity_water_vapor_in_air, saturation_vapor_pressure_auto_kpa,
    thermal_conductivity_air, vapor_density_from_pressure_and_temperature,
};

/// Harder-Pomeroy logistic coefficient set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseTimescale {
    /// Fifteen-minute coefficients.
    FifteenMinute,
    /// Hourly coefficients.
    Hourly,
    /// Daily coefficients.
    Daily,
}

impl PhaseTimescale {
    const fn coefficients(self) -> PhaseCoefficients {
        match self {
            Self::FifteenMinute => PhaseCoefficients {
                b: 2.630_06,
                c: 0.093_36,
            },
            Self::Hourly => PhaseCoefficients {
                b: 2.502_86,
                c: 0.125_006,
            },
            Self::Daily => PhaseCoefficients {
                b: 2.798_56,
                c: 0.249_292,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PhaseCoefficients {
    b: f64,
    c: f64,
}

/// Candidate hydrometeor-temperature solver controls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HydrometeorSolverOptions {
    /// Maximum fixed-point iterations.
    pub max_iterations: usize,
    /// Absolute Celsius convergence tolerance.
    pub tolerance_c: f64,
}

impl Default for HydrometeorSolverOptions {
    fn default() -> Self {
        Self {
            max_iterations: 200,
            tolerance_c: 1.0e-8,
        }
    }
}

impl HydrometeorSolverOptions {
    /// Construct solver options with domain checks.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `max_iterations` is zero or `tolerance_c` is
    /// non-finite/non-positive.
    pub fn try_new(max_iterations: usize, tolerance_c: f64) -> Result<Self, MeteorologyError> {
        if max_iterations == 0 {
            return Err(MeteorologyError::InvalidSolverOptions {
                quantity: "max_iterations",
                value: 0.0,
            });
        }
        if !tolerance_c.is_finite() || tolerance_c <= 0.0 {
            return Err(MeteorologyError::InvalidSolverOptions {
                quantity: "tolerance_c",
                value: tolerance_c,
            });
        }
        Ok(Self {
            max_iterations,
            tolerance_c,
        })
    }
}

/// Harder-Pomeroy hydrometeor-temperature solution and diagnostics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HydrometeorTemperatureSolution {
    /// Candidate hydrometeor temperature.
    pub temperature: TemperatureCelsius,
    /// Fixed-point iterations used.
    pub iterations: usize,
    /// Actual free-air vapor density.
    pub air_vapor_density: VaporDensityKilogramsPerCubicMeter,
    /// Saturation vapor density at the solved hydrometeor temperature.
    pub saturation_vapor_density: VaporDensityKilogramsPerCubicMeter,
    /// Last absolute Celsius update.
    pub last_delta_c: f64,
}

/// Candidate rain/snow fractions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrecipitationPhaseFractions {
    /// Rain fraction in `[0, 1]`.
    pub rain_fraction: FractionUnitInterval,
    /// Snow fraction in `[0, 1]`.
    pub snow_fraction: FractionUnitInterval,
}

/// Candidate phase estimate from air temperature and humidity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrecipitationPhaseEstimate {
    /// Solved hydrometeor temperature and diagnostics.
    pub hydrometeor_temperature: HydrometeorTemperatureSolution,
    /// Harder-Pomeroy rain/snow fractions for the requested timescale.
    pub fractions: PrecipitationPhaseFractions,
}

/// Solve Harder-Pomeroy hydrometeor temperature with default solver options.
///
/// # Errors
///
/// Returns a typed error on invalid primitives or solver non-convergence.
pub fn hydrometeor_temperature_from_relative_humidity(
    air_temperature: TemperatureCelsius,
    relative_humidity: FractionUnitInterval,
) -> Result<HydrometeorTemperatureSolution, MeteorologyError> {
    hydrometeor_temperature_from_relative_humidity_with_options(
        air_temperature,
        relative_humidity,
        HydrometeorSolverOptions::default(),
    )
}

/// Solve Harder-Pomeroy hydrometeor temperature with explicit solver options.
///
/// # Errors
///
/// Returns a typed error on invalid primitives, invalid options, or solver
/// non-convergence.
pub fn hydrometeor_temperature_from_relative_humidity_with_options(
    air_temperature: TemperatureCelsius,
    relative_humidity: FractionUnitInterval,
    options: HydrometeorSolverOptions,
) -> Result<HydrometeorTemperatureSolution, MeteorologyError> {
    HydrometeorSolverOptions::try_new(options.max_iterations, options.tolerance_c)?;

    let air_temperature_k = celsius_to_kelvin(air_temperature)?;
    let air_vapor_pressure =
        actual_vapor_pressure_from_relative_humidity_kpa(air_temperature, relative_humidity)?;
    let air_vapor_density =
        vapor_density_from_pressure_and_temperature(air_vapor_pressure, air_temperature)?;
    let diffusivity = molecular_diffusivity_water_vapor_in_air(air_temperature)?;
    let conductivity = thermal_conductivity_air(air_temperature)?;

    let mut hydrometeor_temperature_c = air_temperature.as_celsius();
    let mut last_delta_c = f64::INFINITY;
    let mut saturation_density =
        vapor_density_at_hydrometeor_temperature(hydrometeor_temperature_c)?;

    for iteration in 1..=options.max_iterations {
        let next_temperature_c = next_hydrometeor_temperature_c(
            air_temperature_k,
            hydrometeor_temperature_c,
            air_vapor_density,
            saturation_density,
            diffusivity,
            conductivity,
        )?;
        last_delta_c = (next_temperature_c - hydrometeor_temperature_c).abs();
        let next_temperature = TemperatureCelsius::try_new(next_temperature_c)?;
        saturation_density = vapor_density_at_hydrometeor_temperature(next_temperature_c)?;

        if last_delta_c <= options.tolerance_c {
            return Ok(HydrometeorTemperatureSolution {
                temperature: next_temperature,
                iterations: iteration,
                air_vapor_density,
                saturation_vapor_density: saturation_density,
                last_delta_c,
            });
        }

        hydrometeor_temperature_c = next_temperature_c;
    }

    Err(MeteorologyError::SolverDidNotConverge {
        iterations: options.max_iterations,
        last_temperature_c: hydrometeor_temperature_c,
        last_delta_c,
    })
}

/// Compute Harder-Pomeroy rain/snow fractions from hydrometeor temperature.
///
/// # Errors
///
/// Returns a typed error when computed fractions leave the unit interval.
pub fn rainfall_fraction_for_hydrometeor_temperature(
    hydrometeor_temperature: TemperatureCelsius,
    timescale: PhaseTimescale,
) -> Result<PrecipitationPhaseFractions, MeteorologyError> {
    let coefficients = timescale.coefficients();
    let rain =
        1.0 / (1.0 + coefficients.b * coefficients.c.powf(hydrometeor_temperature.as_celsius()));
    let rain_fraction = FractionUnitInterval::try_new(rain)?;
    let snow_fraction = FractionUnitInterval::try_new(1.0 - rain_fraction.as_fraction())?;
    Ok(PrecipitationPhaseFractions {
        rain_fraction,
        snow_fraction,
    })
}

/// Estimate candidate Harder-Pomeroy phase fractions from air temperature and RH.
///
/// # Errors
///
/// Returns a typed error on hydrometeor solver or fraction failures.
pub fn harder_pomeroy_phase_from_relative_humidity(
    air_temperature: TemperatureCelsius,
    relative_humidity: FractionUnitInterval,
    timescale: PhaseTimescale,
) -> Result<PrecipitationPhaseEstimate, MeteorologyError> {
    let hydrometeor_temperature =
        hydrometeor_temperature_from_relative_humidity(air_temperature, relative_humidity)?;
    let fractions = rainfall_fraction_for_hydrometeor_temperature(
        hydrometeor_temperature.temperature,
        timescale,
    )?;
    Ok(PrecipitationPhaseEstimate {
        hydrometeor_temperature,
        fractions,
    })
}

fn vapor_density_at_hydrometeor_temperature(
    hydrometeor_temperature_c: f64,
) -> Result<VaporDensityKilogramsPerCubicMeter, MeteorologyError> {
    let temperature = TemperatureCelsius::try_new(hydrometeor_temperature_c)?;
    let saturation_pressure = saturation_vapor_pressure_auto_kpa(temperature)?;
    vapor_density_from_pressure_and_temperature(saturation_pressure, temperature)
}

fn next_hydrometeor_temperature_c(
    air_temperature_k: f64,
    hydrometeor_temperature_c: f64,
    air_vapor_density: VaporDensityKilogramsPerCubicMeter,
    saturation_vapor_density: VaporDensityKilogramsPerCubicMeter,
    diffusivity: DiffusivitySquareMetersPerSecond,
    conductivity: ThermalConductivityWattsPerMeterKelvin,
) -> Result<f64, MeteorologyError> {
    let hydrometeor_temperature = TemperatureCelsius::try_new(hydrometeor_temperature_c)?;
    let latent_heat =
        latent_heat_for_hydrometeor_temperature(hydrometeor_temperature)?.as_joules_per_kilogram();
    let next_temperature_k = air_temperature_k
        + diffusivity.as_square_meters_per_second() / conductivity.as_watts_per_meter_kelvin()
            * latent_heat
            * (air_vapor_density.as_kilograms_per_cubic_meter()
                - saturation_vapor_density.as_kilograms_per_cubic_meter());
    let next_temperature_c = next_temperature_k - 273.15;
    celsius_to_kelvin(TemperatureCelsius::try_new(next_temperature_c)?)?;
    Ok(next_temperature_c)
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

    #[test]
    fn saturated_air_identity_holds_for_hydrometeor_temperature() {
        let air = TemperatureCelsius::try_new(0.0).expect("valid air temperature");
        let rh = FractionUnitInterval::try_new(1.0).expect("valid RH");
        let solution =
            hydrometeor_temperature_from_relative_humidity(air, rh).expect("valid solution");

        assert_eq!(solution.iterations, 1);
        assert_close(solution.temperature.as_celsius(), 0.0, 1.0e-12);
        assert_close(solution.last_delta_c, 0.0, 1.0e-12);
    }

    #[test]
    fn unsaturated_air_cools_hydrometeor_temperature() {
        for (air_c, rh, expected_ti_c) in [
            (0.0, 0.5, -3.229_846_367_476),
            (5.0, 0.7, 2.679_146_160_092_045_6),
            (-5.0, 0.8, -5.963_503_925_215_093),
            (-10.0, 0.5, -11.796_751_637_522_05),
            (2.0, 0.6, -0.898_958_740_860_052_8),
        ] {
            let solution = hydrometeor_temperature_from_relative_humidity(
                TemperatureCelsius::try_new(air_c).expect("valid air temperature"),
                FractionUnitInterval::try_new(rh).expect("valid RH"),
            )
            .expect("valid solution");

            assert!(
                solution.temperature.as_celsius() < air_c,
                "unsaturated hydrometeor temperature should cool below air"
            );
            assert_close(solution.temperature.as_celsius(), expected_ti_c, 1.0e-6);
        }
    }

    #[test]
    fn rain_fraction_is_monotonic_and_closes_by_timescale() {
        for timescale in [
            PhaseTimescale::FifteenMinute,
            PhaseTimescale::Hourly,
            PhaseTimescale::Daily,
        ] {
            let cold = rainfall_fraction_for_hydrometeor_temperature(
                TemperatureCelsius::try_new(-2.0).expect("valid temperature"),
                timescale,
            )
            .expect("valid cold fraction");
            let neutral = rainfall_fraction_for_hydrometeor_temperature(
                TemperatureCelsius::try_new(0.0).expect("valid temperature"),
                timescale,
            )
            .expect("valid neutral fraction");
            let warm = rainfall_fraction_for_hydrometeor_temperature(
                TemperatureCelsius::try_new(2.0).expect("valid temperature"),
                timescale,
            )
            .expect("valid warm fraction");

            assert!(cold.rain_fraction.as_fraction() < neutral.rain_fraction.as_fraction());
            assert!(neutral.rain_fraction.as_fraction() < warm.rain_fraction.as_fraction());
            for fractions in [cold, neutral, warm] {
                assert_close(
                    fractions.rain_fraction.as_fraction() + fractions.snow_fraction.as_fraction(),
                    1.0,
                    1.0e-12,
                );
            }
        }
    }

    #[test]
    fn hourly_fraction_reference_values_are_stable() {
        for (temperature_c, expected_rain) in [
            (-2.0, 0.006_204_718_602_135_303),
            (0.0, 0.285_481_006_948_607_73),
            (2.0, 0.962_361_149_032_52),
        ] {
            let fractions = rainfall_fraction_for_hydrometeor_temperature(
                TemperatureCelsius::try_new(temperature_c).expect("valid temperature"),
                PhaseTimescale::Hourly,
            )
            .expect("valid fraction");
            assert_close(
                fractions.rain_fraction.as_fraction(),
                expected_rain,
                1.0e-12,
            );
        }
    }

    #[test]
    fn solver_reports_non_convergence() {
        let air = TemperatureCelsius::try_new(5.0).expect("valid air temperature");
        let rh = FractionUnitInterval::try_new(0.7).expect("valid RH");
        let options = HydrometeorSolverOptions::try_new(1, 1.0e-12).expect("valid options");
        assert!(matches!(
            hydrometeor_temperature_from_relative_humidity_with_options(air, rh, options),
            Err(MeteorologyError::SolverDidNotConverge { iterations: 1, .. })
        ));
    }
}
