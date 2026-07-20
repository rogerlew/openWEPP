//! Contract-governed plant phenology process kernels.
//!
//! The initial kernel implements the generalized Growing Season Index (GSI)
//! from Jolly, Nemani, and Running (2005). It deliberately stops at the
//! dimensionless foliar-phenology signal and the contract-authorized native
//! forest realization of that signal into daily canopy and foliar mass state.

#![deny(unsafe_code)]

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

/// Number of daily instantaneous-GSI samples in the published moving window.
pub const GSI_WINDOW_DAYS: usize = 21;

/// Diagnostic onset/offset crossing used by Jolly et al. (2005).
///
/// This is not a production canopy switch.
pub const GSI_DIAGNOSTIC_THRESHOLD: f64 = 0.5;

/// Existing openWEPP finite canopy-cover ceiling.
pub const FOREST_CANOPY_COVER_CAP: f64 = 0.999;

/// Explicit native-forest authority for realizing a GSI value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForestCanopyParameters {
    pub gsi: GsiParameters,
    pub summer_foliar_biomass_kg_m2: f64,
    pub maximum_leaf_area_index: f64,
    pub evergreen_fraction: f64,
    pub structural_canopy_cover_fraction: f64,
    pub structural_biomass_kg_m2: f64,
    pub canopy_cover_coefficient_m2_kg: f64,
}

impl ForestCanopyParameters {
    fn validate(self) -> Result<(), ForestCanopyError> {
        self.gsi.validate().map_err(ForestCanopyError::Gsi)?;
        validate_positive_finite_forest(
            "summer_foliar_biomass_kg_m2",
            self.summer_foliar_biomass_kg_m2,
        )?;
        validate_positive_finite_forest("maximum_leaf_area_index", self.maximum_leaf_area_index)?;
        validate_unit_interval("evergreen_fraction", self.evergreen_fraction)
            .map_err(ForestCanopyError::Gsi)?;
        validate_non_negative_finite_forest(
            "structural_canopy_cover_fraction",
            self.structural_canopy_cover_fraction,
        )?;
        if self.structural_canopy_cover_fraction > FOREST_CANOPY_COVER_CAP {
            return Err(ForestCanopyError::OutOfDomain {
                field: "structural_canopy_cover_fraction",
            });
        }
        validate_non_negative_finite_forest(
            "structural_biomass_kg_m2",
            self.structural_biomass_kg_m2,
        )?;
        validate_positive_finite_forest(
            "canopy_cover_coefficient_m2_kg",
            self.canopy_cover_coefficient_m2_kg,
        )
    }
}

/// One native-forest canopy realization and its exact daily foliar ledger.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForestCanopyRealization {
    pub growing_season_index: f64,
    pub foliar_activity_fraction: f64,
    pub previous_foliar_biomass_kg_m2: f64,
    pub evergreen_foliar_biomass_kg_m2: f64,
    pub deciduous_foliar_biomass_kg_m2: f64,
    pub live_foliar_biomass_kg_m2: f64,
    pub structural_biomass_kg_m2: f64,
    pub leaf_area_index: f64,
    pub canopy_cover_fraction: f64,
    pub leaf_on_allocation_kg_m2: f64,
    pub leaf_off_litter_kg_m2: f64,
}

/// Coupled moving-window and prior-foliar-mass state for one forest lane.
#[derive(Debug, Clone, PartialEq)]
pub struct ForestCanopyState {
    gsi: GsiState,
    previous_foliar_biomass_kg_m2: f64,
}

impl ForestCanopyState {
    /// Start from an explicit live foliar-mass boundary without synthetic GSI
    /// history.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the boundary is non-finite or negative.
    pub fn new(previous_foliar_biomass_kg_m2: f64) -> Result<Self, ForestCanopyError> {
        validate_non_negative_finite_forest(
            "previous_foliar_biomass_kg_m2",
            previous_foliar_biomass_kg_m2,
        )?;
        Ok(Self {
            gsi: GsiState::new(),
            previous_foliar_biomass_kg_m2,
        })
    }

    /// Advance the GSI window and realize its post-phenology canopy state.
    /// Mutation is atomic: all validation and realization complete first.
    ///
    /// # Errors
    ///
    /// Returns a typed GSI, authority, state, or closure error.
    pub fn advance(
        &mut self,
        parameters: ForestCanopyParameters,
        forcing: GsiDailyForcing,
    ) -> Result<ForestCanopyDailyResult, ForestCanopyError> {
        parameters.validate()?;
        let mut next_gsi = self.gsi.clone();
        let gsi = next_gsi
            .advance(parameters.gsi, forcing)
            .map_err(ForestCanopyError::Gsi)?;
        let canopy = realize_forest_canopy(
            parameters,
            gsi.growing_season_index,
            self.previous_foliar_biomass_kg_m2,
        )?;
        self.gsi = next_gsi;
        self.previous_foliar_biomass_kg_m2 = canopy.live_foliar_biomass_kg_m2;
        Ok(ForestCanopyDailyResult { gsi, canopy })
    }

    #[must_use]
    pub fn gsi_state(&self) -> &GsiState {
        &self.gsi
    }

    #[must_use]
    pub fn previous_foliar_biomass_kg_m2(&self) -> f64 {
        self.previous_foliar_biomass_kg_m2
    }
}

/// One coupled GSI and native-forest canopy update.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForestCanopyDailyResult {
    pub gsi: GsiDailyResult,
    pub canopy: ForestCanopyRealization,
}

/// Realize an already-computed trailing GSI value into native forest state.
///
/// # Errors
///
/// Returns a typed error for invalid authority, GSI, prior state, computed
/// state, or daily foliar-mass closure.
pub fn realize_forest_canopy(
    parameters: ForestCanopyParameters,
    growing_season_index: f64,
    previous_foliar_biomass_kg_m2: f64,
) -> Result<ForestCanopyRealization, ForestCanopyError> {
    parameters.validate()?;
    validate_unit_interval("growing_season_index", growing_season_index)
        .map_err(ForestCanopyError::Gsi)?;
    validate_non_negative_finite_forest(
        "previous_foliar_biomass_kg_m2",
        previous_foliar_biomass_kg_m2,
    )?;

    let deciduous_fraction = 1.0 - parameters.evergreen_fraction;
    let foliar_activity_fraction =
        parameters.evergreen_fraction + deciduous_fraction * growing_season_index;
    let evergreen_foliar_biomass_kg_m2 =
        parameters.summer_foliar_biomass_kg_m2 * parameters.evergreen_fraction;
    let deciduous_foliar_biomass_kg_m2 =
        parameters.summer_foliar_biomass_kg_m2 * deciduous_fraction * growing_season_index;
    let live_foliar_biomass_kg_m2 = evergreen_foliar_biomass_kg_m2 + deciduous_foliar_biomass_kg_m2;
    let leaf_area_index = parameters.maximum_leaf_area_index * foliar_activity_fraction;
    let foliar_canopy_cover =
        1.0 - (-parameters.canopy_cover_coefficient_m2_kg * live_foliar_biomass_kg_m2).exp();
    let canopy_cover_fraction = parameters
        .structural_canopy_cover_fraction
        .max(foliar_canopy_cover)
        .min(FOREST_CANOPY_COVER_CAP);
    let change = live_foliar_biomass_kg_m2 - previous_foliar_biomass_kg_m2;
    let leaf_on_allocation_kg_m2 = change.max(0.0);
    let leaf_off_litter_kg_m2 = (-change).max(0.0);

    for (field, value) in [
        ("foliar_activity_fraction", foliar_activity_fraction),
        (
            "evergreen_foliar_biomass_kg_m2",
            evergreen_foliar_biomass_kg_m2,
        ),
        (
            "deciduous_foliar_biomass_kg_m2",
            deciduous_foliar_biomass_kg_m2,
        ),
        ("live_foliar_biomass_kg_m2", live_foliar_biomass_kg_m2),
        ("leaf_area_index", leaf_area_index),
        ("canopy_cover_fraction", canopy_cover_fraction),
        ("leaf_on_allocation_kg_m2", leaf_on_allocation_kg_m2),
        ("leaf_off_litter_kg_m2", leaf_off_litter_kg_m2),
    ] {
        validate_non_negative_finite_forest(field, value)?;
    }
    let reconstructed =
        previous_foliar_biomass_kg_m2 + leaf_on_allocation_kg_m2 - leaf_off_litter_kg_m2;
    let closure_tolerance = 16.0 * f64::EPSILON * live_foliar_biomass_kg_m2.abs().max(1.0);
    if (reconstructed - live_foliar_biomass_kg_m2).abs() > closure_tolerance {
        return Err(ForestCanopyError::MassClosure);
    }

    Ok(ForestCanopyRealization {
        growing_season_index,
        foliar_activity_fraction,
        previous_foliar_biomass_kg_m2,
        evergreen_foliar_biomass_kg_m2,
        deciduous_foliar_biomass_kg_m2,
        live_foliar_biomass_kg_m2,
        structural_biomass_kg_m2: parameters.structural_biomass_kg_m2,
        leaf_area_index,
        canopy_cover_fraction,
        leaf_on_allocation_kg_m2,
        leaf_off_litter_kg_m2,
    })
}

/// Parameters for the generalized GSI constraint indicators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsiParameters {
    pub minimum_temperature_inactive_c: f64,
    pub minimum_temperature_unconstrained_c: f64,
    pub vapor_pressure_deficit_unconstrained_pa: f64,
    pub vapor_pressure_deficit_inactive_pa: f64,
    pub photoperiod_inactive_hours: f64,
    pub photoperiod_unconstrained_hours: f64,
}

impl GsiParameters {
    /// Generalized parameterization used across the 2005 paper's test sites.
    #[must_use]
    pub const fn generalized() -> Self {
        Self {
            minimum_temperature_inactive_c: -2.0,
            minimum_temperature_unconstrained_c: 5.0,
            vapor_pressure_deficit_unconstrained_pa: 900.0,
            vapor_pressure_deficit_inactive_pa: 4_100.0,
            photoperiod_inactive_hours: 10.0,
            photoperiod_unconstrained_hours: 11.0,
        }
    }

    fn validate(self) -> Result<(), GsiError> {
        validate_threshold_pair(
            "minimum_temperature_c",
            self.minimum_temperature_inactive_c,
            self.minimum_temperature_unconstrained_c,
        )?;
        validate_threshold_pair(
            "vapor_pressure_deficit_pa",
            self.vapor_pressure_deficit_unconstrained_pa,
            self.vapor_pressure_deficit_inactive_pa,
        )?;
        validate_threshold_pair(
            "photoperiod_hours",
            self.photoperiod_inactive_hours,
            self.photoperiod_unconstrained_hours,
        )?;
        if self.photoperiod_inactive_hours < 0.0 || self.photoperiod_unconstrained_hours > 24.0 {
            return Err(GsiError::ThresholdOutOfDomain {
                field: "photoperiod_hours",
            });
        }
        if self.vapor_pressure_deficit_unconstrained_pa < 0.0 {
            return Err(GsiError::ThresholdOutOfDomain {
                field: "vapor_pressure_deficit_pa",
            });
        }
        Ok(())
    }
}

impl Default for GsiParameters {
    fn default() -> Self {
        Self::generalized()
    }
}

/// Year-aware calendar key for one GSI forcing day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GsiDate {
    pub year: i32,
    pub ordinal_day: u16,
}

impl GsiDate {
    fn validate(self) -> Result<(), GsiError> {
        let maximum_ordinal_day = days_in_year(self.year);
        if self.ordinal_day == 0 || self.ordinal_day > maximum_ordinal_day {
            return Err(GsiError::InvalidCalendarDate {
                year: self.year,
                ordinal_day: self.ordinal_day,
                maximum_ordinal_day,
            });
        }
        Ok(())
    }

    fn next(self) -> Result<Self, GsiError> {
        self.validate()?;
        if self.ordinal_day < days_in_year(self.year) {
            Ok(Self {
                year: self.year,
                ordinal_day: self.ordinal_day + 1,
            })
        } else {
            Ok(Self {
                year: self
                    .year
                    .checked_add(1)
                    .ok_or(GsiError::CalendarYearOverflow)?,
                ordinal_day: 1,
            })
        }
    }
}

/// Daily meteorological, location, and calendar forcing for the GSI kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsiDailyForcing {
    pub minimum_temperature_c: f64,
    pub vapor_pressure_deficit_pa: f64,
    pub latitude_degrees: f64,
    pub date: GsiDate,
}

impl GsiDailyForcing {
    fn validate(self) -> Result<(), GsiError> {
        validate_finite("minimum_temperature_c", self.minimum_temperature_c)?;
        validate_finite("vapor_pressure_deficit_pa", self.vapor_pressure_deficit_pa)?;
        validate_finite("latitude_degrees", self.latitude_degrees)?;
        if self.vapor_pressure_deficit_pa < 0.0 {
            return Err(GsiError::NegativeVaporPressureDeficit);
        }
        if !(-90.0..=90.0).contains(&self.latitude_degrees) {
            return Err(GsiError::LatitudeOutOfRange);
        }
        self.date.validate()?;
        Ok(())
    }
}

/// The three daily constraints and their instantaneous product.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsiDailyIndicators {
    pub minimum_temperature: f64,
    pub vapor_pressure_deficit: f64,
    pub photoperiod: f64,
    pub instantaneous_gsi: f64,
    pub photoperiod_hours: f64,
}

/// Result after admitting one daily forcing value to the moving window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GsiDailyResult {
    pub indicators: GsiDailyIndicators,
    pub growing_season_index: f64,
    pub sample_count: usize,
}

/// Exact trailing-window state for the generalized GSI.
#[derive(Debug, Clone, PartialEq)]
pub struct GsiState {
    history: VecDeque<f64>,
    last_date: Option<GsiDate>,
}

impl GsiState {
    /// Construct an empty state. The first output averages one real sample;
    /// the history is not prefilled with synthetic zeros.
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(GSI_WINDOW_DAYS),
            last_date: None,
        }
    }

    /// Restore an exact trailing history, oldest sample first.
    ///
    /// # Errors
    ///
    /// Returns [`GsiError::HistoryTooLong`] above 21 samples or a typed value
    /// error when any retained sample is non-finite or outside `[0,1]`.
    pub fn try_from_history(history: &[f64], last_date: Option<GsiDate>) -> Result<Self, GsiError> {
        if history.len() > GSI_WINDOW_DAYS {
            return Err(GsiError::HistoryTooLong);
        }
        if history.is_empty() != last_date.is_none() {
            return Err(GsiError::HistoryAnchorMismatch);
        }
        if let Some(date) = last_date {
            date.validate()?;
        }
        for &value in history {
            validate_unit_interval("gsi_history", value)?;
        }
        Ok(Self {
            history: history.iter().copied().collect(),
            last_date,
        })
    }

    /// Number of real daily samples currently retained.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.history.len()
    }

    /// Retained history in chronological order, oldest first.
    #[must_use]
    pub fn history(&self) -> Vec<f64> {
        self.history.iter().copied().collect()
    }

    /// Calendar date associated with the newest retained sample.
    #[must_use]
    pub fn last_date(&self) -> Option<GsiDate> {
        self.last_date
    }

    /// Evaluate and admit one day, returning the exact trailing arithmetic mean.
    ///
    /// # Errors
    ///
    /// Returns a typed parameter, forcing, photoperiod, or state error before
    /// admitting the daily value when any input violates the GSI contract.
    pub fn advance(
        &mut self,
        parameters: GsiParameters,
        forcing: GsiDailyForcing,
    ) -> Result<GsiDailyResult, GsiError> {
        let indicators = daily_indicators(parameters, forcing)?;
        if let Some(last_date) = self.last_date {
            let expected = last_date.next()?;
            if forcing.date != expected {
                return Err(GsiError::NonConsecutiveDate {
                    expected,
                    actual: forcing.date,
                });
            }
        }
        if self.history.len() == GSI_WINDOW_DAYS {
            self.history.pop_front();
        }
        self.history.push_back(indicators.instantaneous_gsi);
        self.last_date = Some(forcing.date);
        let growing_season_index = arithmetic_mean(&self.history)?;
        Ok(GsiDailyResult {
            indicators,
            growing_season_index,
            sample_count: self.history.len(),
        })
    }
}

impl Default for GsiState {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluate the daily indicators without changing moving-window state.
///
/// # Errors
///
/// Returns a typed error for unordered/out-of-domain parameters, invalid daily
/// forcing, or a non-finite/out-of-domain computed result.
pub fn daily_indicators(
    parameters: GsiParameters,
    forcing: GsiDailyForcing,
) -> Result<GsiDailyIndicators, GsiError> {
    parameters.validate()?;
    forcing.validate()?;
    let photoperiod_hours = photoperiod_hours(forcing.latitude_degrees, forcing.date.ordinal_day)?;
    let minimum_temperature = increasing_indicator(
        forcing.minimum_temperature_c,
        parameters.minimum_temperature_inactive_c,
        parameters.minimum_temperature_unconstrained_c,
    );
    let vapor_pressure_deficit = 1.0
        - increasing_indicator(
            forcing.vapor_pressure_deficit_pa,
            parameters.vapor_pressure_deficit_unconstrained_pa,
            parameters.vapor_pressure_deficit_inactive_pa,
        );
    let photoperiod = increasing_indicator(
        photoperiod_hours,
        parameters.photoperiod_inactive_hours,
        parameters.photoperiod_unconstrained_hours,
    );
    let instantaneous_gsi = minimum_temperature * vapor_pressure_deficit * photoperiod;
    validate_unit_interval("instantaneous_gsi", instantaneous_gsi)?;
    Ok(GsiDailyIndicators {
        minimum_temperature,
        vapor_pressure_deficit,
        photoperiod,
        instantaneous_gsi,
        photoperiod_hours,
    })
}

fn days_in_year(year: i32) -> u16 {
    if year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0) {
        366
    } else {
        365
    }
}

/// Maximum possible daylight duration using FAO-56 solar geometry.
///
/// # Errors
///
/// Returns a typed error for non-finite or out-of-range latitude, ordinal days
/// outside `1..=366`, or a non-finite/out-of-domain computed duration.
pub fn photoperiod_hours(latitude_degrees: f64, ordinal_day: u16) -> Result<f64, GsiError> {
    validate_finite("latitude_degrees", latitude_degrees)?;
    if !(-90.0..=90.0).contains(&latitude_degrees) {
        return Err(GsiError::LatitudeOutOfRange);
    }
    if !(1..=366).contains(&ordinal_day) {
        return Err(GsiError::OrdinalDayOutOfRange);
    }
    let latitude_radians = latitude_degrees.to_radians();
    let day = f64::from(ordinal_day);
    let solar_declination = 0.409 * ((2.0 * std::f64::consts::PI * day / 365.0) - 1.39).sin();
    let sunset_cosine = -latitude_radians.tan() * solar_declination.tan();
    let sunset_hour_angle = sunset_cosine.clamp(-1.0, 1.0).acos();
    let hours = 24.0 * sunset_hour_angle / std::f64::consts::PI;
    if !hours.is_finite() || !(0.0..=24.0).contains(&hours) {
        return Err(GsiError::PhotoperiodOutOfDomain);
    }
    Ok(hours)
}

fn increasing_indicator(value: f64, lower: f64, upper: f64) -> f64 {
    if value <= lower {
        0.0
    } else if value >= upper {
        1.0
    } else {
        (value - lower) / (upper - lower)
    }
}

fn arithmetic_mean(history: &VecDeque<f64>) -> Result<f64, GsiError> {
    if history.is_empty() {
        return Err(GsiError::EmptyHistory);
    }
    let count = u32::try_from(history.len()).map_err(|_| GsiError::HistoryTooLong)?;
    let mean = history.iter().sum::<f64>() / f64::from(count);
    validate_unit_interval("growing_season_index", mean)?;
    Ok(mean)
}

fn validate_threshold_pair(field: &'static str, lower: f64, upper: f64) -> Result<(), GsiError> {
    validate_finite(field, lower)?;
    validate_finite(field, upper)?;
    if lower >= upper {
        Err(GsiError::ThresholdOrder { field })
    } else {
        Ok(())
    }
}

fn validate_finite(field: &'static str, value: f64) -> Result<(), GsiError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(GsiError::NonFinite { field })
    }
}

fn validate_unit_interval(field: &'static str, value: f64) -> Result<(), GsiError> {
    validate_finite(field, value)?;
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(GsiError::UnitIntervalViolation { field })
    }
}

fn validate_non_negative_finite_forest(
    field: &'static str,
    value: f64,
) -> Result<(), ForestCanopyError> {
    if !value.is_finite() {
        return Err(ForestCanopyError::NonFinite { field });
    }
    if value < 0.0 {
        return Err(ForestCanopyError::OutOfDomain { field });
    }
    Ok(())
}

fn validate_positive_finite_forest(
    field: &'static str,
    value: f64,
) -> Result<(), ForestCanopyError> {
    validate_non_negative_finite_forest(field, value)?;
    if value == 0.0 {
        return Err(ForestCanopyError::OutOfDomain { field });
    }
    Ok(())
}

/// Typed native-forest canopy authority, state, and closure failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ForestCanopyError {
    Gsi(GsiError),
    NonFinite { field: &'static str },
    OutOfDomain { field: &'static str },
    MassClosure,
}

impl fmt::Display for ForestCanopyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gsi(source) => write!(formatter, "invalid GSI authority or forcing: {source}"),
            Self::NonFinite { field } => write!(formatter, "{field} must be finite"),
            Self::OutOfDomain { field } => write!(formatter, "{field} is outside its domain"),
            Self::MassClosure => formatter.write_str("daily foliar mass ledger does not close"),
        }
    }
}

impl Error for ForestCanopyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Gsi(source) => Some(source),
            Self::NonFinite { .. } | Self::OutOfDomain { .. } | Self::MassClosure => None,
        }
    }
}

/// Typed GSI parameter, forcing, and state failures.
#[derive(Debug, Clone, PartialEq)]
pub enum GsiError {
    NonFinite {
        field: &'static str,
    },
    NegativeVaporPressureDeficit,
    LatitudeOutOfRange,
    OrdinalDayOutOfRange,
    InvalidCalendarDate {
        year: i32,
        ordinal_day: u16,
        maximum_ordinal_day: u16,
    },
    NonConsecutiveDate {
        expected: GsiDate,
        actual: GsiDate,
    },
    CalendarYearOverflow,
    ThresholdOrder {
        field: &'static str,
    },
    ThresholdOutOfDomain {
        field: &'static str,
    },
    PhotoperiodOutOfDomain,
    HistoryTooLong,
    HistoryAnchorMismatch,
    EmptyHistory,
    UnitIntervalViolation {
        field: &'static str,
    },
}

impl fmt::Display for GsiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field } => write!(formatter, "{field} must be finite"),
            Self::NegativeVaporPressureDeficit => {
                formatter.write_str("vapor_pressure_deficit_pa must be nonnegative")
            }
            Self::LatitudeOutOfRange => {
                formatter.write_str("latitude_degrees must be within [-90, 90]")
            }
            Self::OrdinalDayOutOfRange => {
                formatter.write_str("ordinal_day must be within [1, 366]")
            }
            Self::InvalidCalendarDate {
                year,
                ordinal_day,
                maximum_ordinal_day,
            } => write!(
                formatter,
                "ordinal_day {ordinal_day} is invalid for year {year}; maximum is {maximum_ordinal_day}"
            ),
            Self::NonConsecutiveDate { expected, actual } => write!(
                formatter,
                "GSI forcing date must be consecutive: expected {}-{:03}, received {}-{:03}",
                expected.year, expected.ordinal_day, actual.year, actual.ordinal_day
            ),
            Self::CalendarYearOverflow => {
                formatter.write_str("GSI calendar year overflow while advancing date")
            }
            Self::ThresholdOrder { field } => {
                write!(
                    formatter,
                    "{field} lower threshold must be less than upper threshold"
                )
            }
            Self::ThresholdOutOfDomain { field } => {
                write!(
                    formatter,
                    "{field} thresholds are outside the physical domain"
                )
            }
            Self::PhotoperiodOutOfDomain => {
                formatter.write_str("computed photoperiod must be finite and within [0, 24] hours")
            }
            Self::HistoryTooLong => formatter.write_str("GSI history exceeds 21 samples"),
            Self::HistoryAnchorMismatch => formatter.write_str(
                "nonempty GSI history requires its newest date and empty history forbids one",
            ),
            Self::EmptyHistory => formatter.write_str("GSI history is empty"),
            Self::UnitIntervalViolation { field } => {
                write!(formatter, "{field} must be within [0, 1]")
            }
        }
    }
}

impl Error for GsiError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn forcing(
        minimum_temperature_c: f64,
        vapor_pressure_deficit_pa: f64,
        latitude_degrees: f64,
        ordinal_day: u16,
    ) -> GsiDailyForcing {
        dated_forcing(
            minimum_temperature_c,
            vapor_pressure_deficit_pa,
            latitude_degrees,
            2027,
            ordinal_day,
        )
    }

    fn dated_forcing(
        minimum_temperature_c: f64,
        vapor_pressure_deficit_pa: f64,
        latitude_degrees: f64,
        year: i32,
        ordinal_day: u16,
    ) -> GsiDailyForcing {
        GsiDailyForcing {
            minimum_temperature_c,
            vapor_pressure_deficit_pa,
            latitude_degrees,
            date: GsiDate { year, ordinal_day },
        }
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }

    #[test]
    fn published_indicator_breakpoints_and_interiors_are_exact() {
        let parameters = GsiParameters::generalized();
        let cold = daily_indicators(parameters, forcing(-2.0, 900.0, 0.0, 80))
            .expect("valid cold breakpoint");
        let warm = daily_indicators(parameters, forcing(5.0, 900.0, 0.0, 80))
            .expect("valid warm breakpoint");
        let middle = daily_indicators(parameters, forcing(1.5, 2_500.0, 0.0, 80))
            .expect("valid interior values");
        let dry = daily_indicators(parameters, forcing(5.0, 4_100.0, 0.0, 80))
            .expect("valid dry breakpoint");

        assert_eq!(cold.minimum_temperature.to_bits(), 0.0_f64.to_bits());
        assert_eq!(warm.minimum_temperature.to_bits(), 1.0_f64.to_bits());
        assert_eq!(cold.vapor_pressure_deficit.to_bits(), 1.0_f64.to_bits());
        assert_eq!(dry.vapor_pressure_deficit.to_bits(), 0.0_f64.to_bits());
        assert_eq!(middle.minimum_temperature.to_bits(), 0.5_f64.to_bits());
        assert_eq!(middle.vapor_pressure_deficit.to_bits(), 0.5_f64.to_bits());
        assert_eq!(middle.photoperiod.to_bits(), 1.0_f64.to_bits());
        assert_eq!(middle.instantaneous_gsi.to_bits(), 0.25_f64.to_bits());
    }

    #[test]
    fn independent_three_constraint_vector_matches_published_equations() {
        let indicators = daily_indicators(
            GsiParameters::generalized(),
            forcing(1.5, 2_500.0, 45.0, 55),
        )
        .expect("valid three-constraint vector");

        assert_close(indicators.minimum_temperature, 0.5, 1.0e-15);
        assert_close(indicators.vapor_pressure_deficit, 0.5, 1.0e-15);
        assert_close(
            indicators.photoperiod_hours,
            10.638_946_133_296_711,
            1.0e-12,
        );
        assert_close(indicators.photoperiod, 0.638_946_133_296_711_2, 1.0e-12);
        assert_close(
            indicators.instantaneous_gsi,
            0.159_736_533_324_177_8,
            1.0e-12,
        );
    }

    #[test]
    fn photoperiod_indicator_breakpoints_are_directly_exercised() {
        let parameters = GsiParameters::generalized();
        assert_eq!(
            increasing_indicator(
                10.0,
                parameters.photoperiod_inactive_hours,
                parameters.photoperiod_unconstrained_hours
            )
            .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            increasing_indicator(
                11.0,
                parameters.photoperiod_inactive_hours,
                parameters.photoperiod_unconstrained_hours
            )
            .to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(
            increasing_indicator(
                10.5,
                parameters.photoperiod_inactive_hours,
                parameters.photoperiod_unconstrained_hours
            )
            .to_bits(),
            0.5_f64.to_bits()
        );
    }

    #[test]
    fn fao56_daylight_vector_is_anchored_at_an_ordinary_latitude() {
        assert_close(
            photoperiod_hours(-20.0, 246).expect("valid FAO-56 vector"),
            11.665_591_945_584_73,
            1.0e-12,
        );
    }

    #[test]
    fn moving_window_has_explicit_warmup_transition_and_fifo_eviction() {
        let mut state = GsiState::new();
        let parameters = GsiParameters::generalized();
        let mut first_admission = None;
        let mut day_20 = None;
        for day in 1_u16..=20 {
            let target = f64::from(day) / 20.0;
            let result = state
                .advance(parameters, forcing(-2.0 + 7.0 * target, 900.0, 0.0, day))
                .expect("warm-up sample");
            if day == 1 {
                first_admission = Some(result);
            }
            day_20 = Some(result);
        }
        let first_admission = first_admission.expect("day 1 result");
        assert_eq!(first_admission.sample_count, 1);
        assert_close(first_admission.growing_season_index, 0.05, 1.0e-14);
        let day_20 = day_20.expect("day 20 result");
        assert_eq!(day_20.sample_count, 20);
        assert_close(day_20.growing_season_index, 10.5 / 20.0, 1.0e-14);

        let day_21 = state
            .advance(parameters, forcing(-2.0 + 7.0 * 0.25, 900.0, 0.0, 21))
            .expect("first full-window sample");
        assert_eq!(day_21.sample_count, GSI_WINDOW_DAYS);
        assert_close(day_21.growing_season_index, 10.75 / 21.0, 1.0e-14);

        let day_22 = state
            .advance(parameters, forcing(-2.0 + 7.0 * 0.75, 900.0, 0.0, 22))
            .expect("FIFO eviction sample");
        assert_eq!(day_22.sample_count, GSI_WINDOW_DAYS);
        assert_close(day_22.growing_season_index, 11.45 / 21.0, 1.0e-14);
        assert_close(state.history()[0], 0.10, 1.0e-14);
        assert_close(state.history()[20], 0.75, 1.0e-14);
        assert_eq!(
            state.last_date(),
            Some(GsiDate {
                year: 2027,
                ordinal_day: 22
            })
        );
    }

    #[test]
    fn signed_latitude_reverses_seasonal_phase() {
        let north_june = photoperiod_hours(45.0, 172).expect("north June");
        let north_december = photoperiod_hours(45.0, 355).expect("north December");
        let south_june = photoperiod_hours(-45.0, 172).expect("south June");
        let south_december = photoperiod_hours(-45.0, 355).expect("south December");

        assert!(north_june > north_december);
        assert!(south_june < south_december);
        assert_close(north_june, south_december, 0.05);
        assert_close(north_december, south_june, 0.05);
    }

    #[test]
    fn polar_day_and_night_are_finite_and_bounded() {
        assert_eq!(
            photoperiod_hours(90.0, 172)
                .expect("north polar day")
                .to_bits(),
            24.0_f64.to_bits()
        );
        assert_eq!(
            photoperiod_hours(90.0, 355)
                .expect("north polar night")
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            photoperiod_hours(-90.0, 172)
                .expect("south polar night")
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            photoperiod_hours(-90.0, 355)
                .expect("south polar day")
                .to_bits(),
            24.0_f64.to_bits()
        );
    }

    #[test]
    fn invalid_parameters_forcing_and_history_fail_typed() {
        let mut unordered = GsiParameters::generalized();
        unordered.minimum_temperature_unconstrained_c = -2.0;
        assert!(matches!(
            daily_indicators(unordered, forcing(1.0, 1_000.0, 45.0, 100)),
            Err(GsiError::ThresholdOrder {
                field: "minimum_temperature_c"
            })
        ));
        assert_eq!(
            daily_indicators(
                GsiParameters::generalized(),
                forcing(f64::NAN, 1_000.0, 45.0, 100)
            ),
            Err(GsiError::NonFinite {
                field: "minimum_temperature_c"
            })
        );
        assert_eq!(
            daily_indicators(GsiParameters::generalized(), forcing(1.0, -1.0, 45.0, 100)),
            Err(GsiError::NegativeVaporPressureDeficit)
        );
        assert_eq!(
            photoperiod_hours(90.1, 100),
            Err(GsiError::LatitudeOutOfRange)
        );
        assert_eq!(
            photoperiod_hours(45.0, 0),
            Err(GsiError::OrdinalDayOutOfRange)
        );
        assert_eq!(
            GsiState::try_from_history(
                &[0.0; GSI_WINDOW_DAYS + 1],
                Some(GsiDate {
                    year: 2027,
                    ordinal_day: 21
                })
            ),
            Err(GsiError::HistoryTooLong)
        );
        assert_eq!(
            GsiState::try_from_history(
                &[1.1],
                Some(GsiDate {
                    year: 2027,
                    ordinal_day: 1
                })
            ),
            Err(GsiError::UnitIntervalViolation {
                field: "gsi_history"
            })
        );
        assert_eq!(
            daily_indicators(
                GsiParameters::generalized(),
                dated_forcing(1.0, 1_000.0, 45.0, 2027, 366)
            ),
            Err(GsiError::InvalidCalendarDate {
                year: 2027,
                ordinal_day: 366,
                maximum_ordinal_day: 365,
            })
        );
        assert_eq!(
            GsiState::try_from_history(&[0.5], None),
            Err(GsiError::HistoryAnchorMismatch)
        );
        assert_eq!(
            GsiState::try_from_history(
                &[],
                Some(GsiDate {
                    year: 2027,
                    ordinal_day: 1
                })
            ),
            Err(GsiError::HistoryAnchorMismatch)
        );
    }

    #[test]
    fn state_rejects_repeated_skipped_and_reversed_dates_without_mutation() {
        let parameters = GsiParameters::generalized();
        for rejected_day in [100_u16, 102, 99] {
            let mut state = GsiState::new();
            state
                .advance(parameters, forcing(5.0, 900.0, 0.0, 100))
                .expect("anchor day");
            assert_eq!(
                state.advance(parameters, forcing(5.0, 900.0, 0.0, rejected_day)),
                Err(GsiError::NonConsecutiveDate {
                    expected: GsiDate {
                        year: 2027,
                        ordinal_day: 101
                    },
                    actual: GsiDate {
                        year: 2027,
                        ordinal_day: rejected_day
                    },
                })
            );
            assert_eq!(state.sample_count(), 1);
            assert_eq!(
                state.last_date(),
                Some(GsiDate {
                    year: 2027,
                    ordinal_day: 100
                })
            );
        }
    }

    #[test]
    fn state_accepts_common_and_leap_year_rollovers() {
        let parameters = GsiParameters::generalized();
        let mut common = GsiState::new();
        common
            .advance(parameters, dated_forcing(5.0, 900.0, 0.0, 2027, 365))
            .expect("common-year end");
        common
            .advance(parameters, dated_forcing(5.0, 900.0, 0.0, 2028, 1))
            .expect("common-year rollover");

        let mut leap = GsiState::new();
        leap.advance(parameters, dated_forcing(5.0, 900.0, 0.0, 2028, 366))
            .expect("leap-year end");
        leap.advance(parameters, dated_forcing(5.0, 900.0, 0.0, 2029, 1))
            .expect("leap-year rollover");
    }

    #[test]
    fn identical_replay_is_bit_identical() {
        let parameters = GsiParameters::generalized();
        let sequence = (1_u16..=60).map(|day| {
            forcing(
                -5.0 + f64::from(day) * 0.2,
                700.0 + f64::from(day) * 35.0,
                44.0,
                day,
            )
        });
        let mut first = GsiState::new();
        let mut second = GsiState::new();
        for day in sequence {
            let a = first.advance(parameters, day).expect("first replay");
            let b = second.advance(parameters, day).expect("second replay");
            assert_eq!(a, b);
            assert_eq!(
                a.growing_season_index.to_bits(),
                b.growing_season_index.to_bits()
            );
        }
        assert_eq!(first, second);
    }

    #[test]
    fn every_typed_error_has_stable_display_text() {
        let cases = [
            (
                GsiError::NonFinite { field: "forcing" },
                "forcing must be finite",
            ),
            (
                GsiError::NegativeVaporPressureDeficit,
                "vapor_pressure_deficit_pa must be nonnegative",
            ),
            (
                GsiError::LatitudeOutOfRange,
                "latitude_degrees must be within [-90, 90]",
            ),
            (
                GsiError::OrdinalDayOutOfRange,
                "ordinal_day must be within [1, 366]",
            ),
            (
                GsiError::InvalidCalendarDate {
                    year: 2027,
                    ordinal_day: 366,
                    maximum_ordinal_day: 365,
                },
                "ordinal_day 366 is invalid for year 2027; maximum is 365",
            ),
            (
                GsiError::NonConsecutiveDate {
                    expected: GsiDate {
                        year: 2027,
                        ordinal_day: 101,
                    },
                    actual: GsiDate {
                        year: 2027,
                        ordinal_day: 103,
                    },
                },
                "GSI forcing date must be consecutive: expected 2027-101, received 2027-103",
            ),
            (
                GsiError::CalendarYearOverflow,
                "GSI calendar year overflow while advancing date",
            ),
            (
                GsiError::ThresholdOrder { field: "threshold" },
                "threshold lower threshold must be less than upper threshold",
            ),
            (
                GsiError::ThresholdOutOfDomain { field: "threshold" },
                "threshold thresholds are outside the physical domain",
            ),
            (
                GsiError::PhotoperiodOutOfDomain,
                "computed photoperiod must be finite and within [0, 24] hours",
            ),
            (GsiError::HistoryTooLong, "GSI history exceeds 21 samples"),
            (
                GsiError::HistoryAnchorMismatch,
                "nonempty GSI history requires its newest date and empty history forbids one",
            ),
            (GsiError::EmptyHistory, "GSI history is empty"),
            (
                GsiError::UnitIntervalViolation { field: "indicator" },
                "indicator must be within [0, 1]",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
