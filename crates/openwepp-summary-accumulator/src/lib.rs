#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Typed summary accumulator kernel for deterministic daily/monthly/yearly/EOS rollups.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::mem;

use openwepp_comparator_metadata::{
    ComparatorTierRoutingError, ComparatorTierRoutingMetadata, ComparatorTierRoutingRequest,
    route_comparator_tier_metadata,
};
use openwepp_sim_contract::status::{SimulationPhase, SimulationStatus, StatusError};

/// Message id emitted when a daily window rollup is produced.
pub const SUMMARY_DAILY_MESSAGE_ID: &str = "SUMACC-DAILY-001";
/// Message id emitted when a monthly window rollup is produced.
pub const SUMMARY_MONTHLY_MESSAGE_ID: &str = "SUMACC-MONTHLY-001";
/// Message id emitted when a yearly window rollup is produced.
pub const SUMMARY_YEARLY_MESSAGE_ID: &str = "SUMACC-YEARLY-001";
/// Message id emitted when an EOS window rollup is produced.
pub const SUMMARY_EOS_MESSAGE_ID: &str = "SUMACC-EOS-001";

/// Summary accumulation windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SummaryWindow {
    Daily,
    Monthly,
    Yearly,
    EndOfSimulation,
}

impl SummaryWindow {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::EndOfSimulation => "end_of_simulation",
        }
    }
}

/// Calendar day key for summary accumulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CalendarDay {
    year: i32,
    month: u8,
    day: u8,
}

impl CalendarDay {
    /// Construct a validated calendar day.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, SummaryAccumulatorError> {
        if !(1..=12).contains(&month) {
            return Err(SummaryAccumulatorError::InvalidDate { year, month, day });
        }

        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return Err(SummaryAccumulatorError::InvalidDate { year, month, day });
        }

        Ok(Self { year, month, day })
    }

    #[must_use]
    pub const fn year(self) -> i32 {
        self.year
    }

    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }
}

/// Typed key for a summary window output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SummaryWindowKey {
    Daily(CalendarDay),
    Monthly { year: i32, month: u8 },
    Yearly { year: i32 },
    EndOfSimulation,
}

/// Deterministic scalar surface used by accumulation inputs and outputs.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SummaryScalarSurface {
    scalars: BTreeMap<String, f64>,
}

impl SummaryScalarSurface {
    /// Build a scalar surface from an iterator of `(symbol, value)` pairs.
    pub fn from_pairs<I, S>(pairs: I) -> Result<Self, SummaryAccumulatorError>
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        let mut scalars = BTreeMap::new();

        for (symbol, value) in pairs {
            let symbol = symbol.into();
            validate_symbol(symbol.as_str())?;
            validate_finite(symbol.as_str(), value)?;

            if scalars.insert(symbol.clone(), value).is_some() {
                return Err(SummaryAccumulatorError::DuplicateSymbol { symbol });
            }
        }

        if scalars.is_empty() {
            return Err(SummaryAccumulatorError::EmptyScalarSurface);
        }

        Ok(Self { scalars })
    }

    /// Build a scalar surface from a pre-constructed map.
    pub fn from_map(scalars: BTreeMap<String, f64>) -> Result<Self, SummaryAccumulatorError> {
        validate_scalar_map(&scalars)?;
        Ok(Self { scalars })
    }

    #[must_use]
    pub fn as_map(&self) -> &BTreeMap<String, f64> {
        &self.scalars
    }

    #[must_use]
    pub fn value(&self, symbol: &str) -> Option<f64> {
        self.scalars.get(symbol).copied()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scalars.is_empty()
    }

    fn add_assign(&mut self, delta: &Self) {
        for (symbol, value) in &delta.scalars {
            let entry = self.scalars.entry(symbol.clone()).or_insert(0.0);
            *entry += value;
        }
    }
}

/// One emitted summary rollup.
#[derive(Debug, Clone, PartialEq)]
pub struct SummaryRollup {
    pub window: SummaryWindow,
    pub key: SummaryWindowKey,
    pub totals: SummaryScalarSurface,
    pub status: SimulationStatus,
    pub comparator_metadata: ComparatorTierRoutingMetadata,
}

/// Output from a single accumulation step or finalize call.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SummaryAccumulatorStepOutcome {
    pub emitted_rollups: Vec<SummaryRollup>,
}

impl SummaryAccumulatorStepOutcome {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.emitted_rollups.is_empty()
    }
}

/// Summary accumulation kernel state machine.
#[derive(Debug, Clone, PartialEq)]
pub struct SummaryAccumulator {
    routing_metadata: ComparatorTierRoutingMetadata,
    current_day: Option<CalendarDay>,
    current_month: Option<(i32, u8)>,
    current_year: Option<i32>,
    daily_totals: SummaryScalarSurface,
    monthly_totals: SummaryScalarSurface,
    yearly_totals: SummaryScalarSurface,
    eos_totals: SummaryScalarSurface,
    sample_count: u64,
}

impl SummaryAccumulator {
    /// Construct a new summary accumulator with explicit comparator tier routing metadata.
    pub fn new(
        routing_request: ComparatorTierRoutingRequest,
    ) -> Result<Self, SummaryAccumulatorError> {
        let routing_metadata = route_comparator_tier_metadata(routing_request)?;
        Ok(Self {
            routing_metadata,
            current_day: None,
            current_month: None,
            current_year: None,
            daily_totals: SummaryScalarSurface::default(),
            monthly_totals: SummaryScalarSurface::default(),
            yearly_totals: SummaryScalarSurface::default(),
            eos_totals: SummaryScalarSurface::default(),
            sample_count: 0,
        })
    }

    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }

    #[must_use]
    pub const fn current_day(&self) -> Option<CalendarDay> {
        self.current_day
    }

    #[must_use]
    pub const fn routing_metadata(&self) -> ComparatorTierRoutingMetadata {
        self.routing_metadata
    }

    /// Accumulate one typed daily input surface.
    ///
    /// When the incoming day advances past the active day, this call emits the
    /// completed daily rollup; if the month and/or year also changes, monthly
    /// and yearly rollups are emitted in deterministic order.
    pub fn accumulate_day(
        &mut self,
        day: CalendarDay,
        delta: SummaryScalarSurface,
    ) -> Result<SummaryAccumulatorStepOutcome, SummaryAccumulatorError> {
        if delta.is_empty() {
            return Err(SummaryAccumulatorError::EmptyScalarSurface);
        }

        let mut emitted_rollups = Vec::new();

        match self.current_day {
            None => {
                self.current_day = Some(day);
                self.current_month = Some((day.year(), day.month()));
                self.current_year = Some(day.year());
            }
            Some(previous_day) => {
                if day < previous_day {
                    return Err(SummaryAccumulatorError::NonMonotonicDate {
                        previous: previous_day,
                        incoming: day,
                    });
                }

                if day != previous_day {
                    emitted_rollups.push(self.take_daily_rollup(previous_day)?);
                    self.current_day = Some(day);

                    if day.year() != previous_day.year() || day.month() != previous_day.month() {
                        let (month_year, month) = self.current_month.ok_or(
                            SummaryAccumulatorError::WindowStateMissing {
                                window: SummaryWindow::Monthly,
                            },
                        )?;
                        emitted_rollups.push(self.take_monthly_rollup(month_year, month)?);
                        self.current_month = Some((day.year(), day.month()));
                    }

                    if day.year() != previous_day.year() {
                        let year = self.current_year.ok_or(
                            SummaryAccumulatorError::WindowStateMissing {
                                window: SummaryWindow::Yearly,
                            },
                        )?;
                        emitted_rollups.push(self.take_yearly_rollup(year)?);
                        self.current_year = Some(day.year());
                    }
                }
            }
        }

        self.daily_totals.add_assign(&delta);
        self.monthly_totals.add_assign(&delta);
        self.yearly_totals.add_assign(&delta);
        self.eos_totals.add_assign(&delta);
        self.sample_count = self.sample_count.saturating_add(1);

        Ok(SummaryAccumulatorStepOutcome { emitted_rollups })
    }

    /// Flush the active day/month/year windows and emit EOS totals.
    pub fn finalize(&mut self) -> Result<SummaryAccumulatorStepOutcome, SummaryAccumulatorError> {
        if self.sample_count == 0 {
            return Err(SummaryAccumulatorError::FinalizeWithoutSamples);
        }

        let current_day = self
            .current_day
            .ok_or(SummaryAccumulatorError::WindowStateMissing {
                window: SummaryWindow::Daily,
            })?;
        let (month_year, month) =
            self.current_month
                .ok_or(SummaryAccumulatorError::WindowStateMissing {
                    window: SummaryWindow::Monthly,
                })?;
        let year = self
            .current_year
            .ok_or(SummaryAccumulatorError::WindowStateMissing {
                window: SummaryWindow::Yearly,
            })?;

        let emitted_rollups = vec![
            self.take_daily_rollup(current_day)?,
            self.take_monthly_rollup(month_year, month)?,
            self.take_yearly_rollup(year)?,
            self.take_eos_rollup()?,
        ];

        self.reset_state();

        Ok(SummaryAccumulatorStepOutcome { emitted_rollups })
    }

    fn take_daily_rollup(
        &mut self,
        day: CalendarDay,
    ) -> Result<SummaryRollup, SummaryAccumulatorError> {
        build_rollup(
            SummaryWindow::Daily,
            SummaryWindowKey::Daily(day),
            SUMMARY_DAILY_MESSAGE_ID,
            self.routing_metadata,
            &mut self.daily_totals,
        )
    }

    fn take_monthly_rollup(
        &mut self,
        year: i32,
        month: u8,
    ) -> Result<SummaryRollup, SummaryAccumulatorError> {
        build_rollup(
            SummaryWindow::Monthly,
            SummaryWindowKey::Monthly { year, month },
            SUMMARY_MONTHLY_MESSAGE_ID,
            self.routing_metadata,
            &mut self.monthly_totals,
        )
    }

    fn take_yearly_rollup(&mut self, year: i32) -> Result<SummaryRollup, SummaryAccumulatorError> {
        build_rollup(
            SummaryWindow::Yearly,
            SummaryWindowKey::Yearly { year },
            SUMMARY_YEARLY_MESSAGE_ID,
            self.routing_metadata,
            &mut self.yearly_totals,
        )
    }

    fn take_eos_rollup(&mut self) -> Result<SummaryRollup, SummaryAccumulatorError> {
        build_rollup(
            SummaryWindow::EndOfSimulation,
            SummaryWindowKey::EndOfSimulation,
            SUMMARY_EOS_MESSAGE_ID,
            self.routing_metadata,
            &mut self.eos_totals,
        )
    }

    fn reset_state(&mut self) {
        self.current_day = None;
        self.current_month = None;
        self.current_year = None;
        self.daily_totals = SummaryScalarSurface::default();
        self.monthly_totals = SummaryScalarSurface::default();
        self.yearly_totals = SummaryScalarSurface::default();
        self.eos_totals = SummaryScalarSurface::default();
        self.sample_count = 0;
    }
}

fn build_rollup(
    window: SummaryWindow,
    key: SummaryWindowKey,
    message_id: &str,
    comparator_metadata: ComparatorTierRoutingMetadata,
    totals: &mut SummaryScalarSurface,
) -> Result<SummaryRollup, SummaryAccumulatorError> {
    if totals.is_empty() {
        return Err(SummaryAccumulatorError::WindowTotalsMissing { window });
    }

    let status = SimulationStatus::ok(SimulationPhase::SummaryAccumulator, message_id)?;
    let totals = mem::take(totals);

    Ok(SummaryRollup {
        window,
        key,
        totals,
        status,
        comparator_metadata,
    })
}

/// Errors produced by summary accumulation input validation and window handling.
#[derive(Debug, Clone, PartialEq)]
pub enum SummaryAccumulatorError {
    InvalidDate {
        year: i32,
        month: u8,
        day: u8,
    },
    EmptyScalarSurface,
    EmptySymbol,
    DuplicateSymbol {
        symbol: String,
    },
    NonFiniteInput {
        symbol: String,
        value: f64,
    },
    NonMonotonicDate {
        previous: CalendarDay,
        incoming: CalendarDay,
    },
    WindowStateMissing {
        window: SummaryWindow,
    },
    WindowTotalsMissing {
        window: SummaryWindow,
    },
    FinalizeWithoutSamples,
    Status(StatusError),
    ComparatorMetadata(ComparatorTierRoutingError),
}

impl fmt::Display for SummaryAccumulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDate { year, month, day } => {
                write!(
                    f,
                    "invalid calendar date: year={year}, month={month}, day={day}"
                )
            }
            Self::EmptyScalarSurface => f.write_str("summary scalar surface must not be empty"),
            Self::EmptySymbol => f.write_str("summary scalar symbol must not be empty"),
            Self::DuplicateSymbol { symbol } => {
                write!(f, "duplicate summary scalar symbol: {symbol}")
            }
            Self::NonFiniteInput { symbol, value } => {
                write!(f, "non-finite scalar value for symbol {symbol}: {value}")
            }
            Self::NonMonotonicDate { previous, incoming } => {
                write!(
                    f,
                    "non-monotonic day sequence: previous={:?}, incoming={:?}",
                    previous, incoming
                )
            }
            Self::WindowStateMissing { window } => {
                write!(f, "window state missing for {}", window.as_str())
            }
            Self::WindowTotalsMissing { window } => {
                write!(f, "window totals missing for {}", window.as_str())
            }
            Self::FinalizeWithoutSamples => {
                f.write_str("cannot finalize summary accumulator with zero samples")
            }
            Self::Status(source) => write!(f, "status construction failed: {source}"),
            Self::ComparatorMetadata(source) => {
                write!(f, "comparator metadata routing failed: {source}")
            }
        }
    }
}

impl Error for SummaryAccumulatorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::ComparatorMetadata(source) => Some(source),
            _ => None,
        }
    }
}

impl From<StatusError> for SummaryAccumulatorError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<ComparatorTierRoutingError> for SummaryAccumulatorError {
    fn from(value: ComparatorTierRoutingError) -> Self {
        Self::ComparatorMetadata(value)
    }
}

fn validate_scalar_map(scalars: &BTreeMap<String, f64>) -> Result<(), SummaryAccumulatorError> {
    if scalars.is_empty() {
        return Err(SummaryAccumulatorError::EmptyScalarSurface);
    }

    for (symbol, value) in scalars {
        validate_symbol(symbol.as_str())?;
        validate_finite(symbol.as_str(), *value)?;
    }

    Ok(())
}

fn validate_symbol(symbol: &str) -> Result<(), SummaryAccumulatorError> {
    if symbol.trim().is_empty() {
        return Err(SummaryAccumulatorError::EmptySymbol);
    }

    Ok(())
}

fn validate_finite(symbol: &str, value: f64) -> Result<(), SummaryAccumulatorError> {
    if !value.is_finite() {
        return Err(SummaryAccumulatorError::NonFiniteInput {
            symbol: symbol.to_string(),
            value,
        });
    }

    Ok(())
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use openwepp_comparator_metadata::{
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
        ComparatorSurfaceClass,
    };

    use super::*;

    fn surface(pairs: &[(&str, f64)]) -> SummaryScalarSurface {
        SummaryScalarSurface::from_pairs(pairs.iter().copied()).expect("valid scalar surface")
    }

    fn accumulator() -> SummaryAccumulator {
        SummaryAccumulator::new(ComparatorTierRoutingRequest::single_ofe_daily())
            .expect("valid routing metadata")
    }

    fn expect_symbol(surface: &SummaryScalarSurface, symbol: &str, expected: f64) {
        assert_eq!(surface.value(symbol), Some(expected));
    }

    #[test]
    fn emits_deterministic_rollups_across_day_month_year_and_eos() {
        let day_1 = CalendarDay::new(2026, 12, 30).expect("valid day");
        let day_2 = CalendarDay::new(2026, 12, 31).expect("valid day");
        let day_3 = CalendarDay::new(2027, 1, 1).expect("valid day");

        let mut accumulator = accumulator();

        let first = accumulator
            .accumulate_day(day_1, surface(&[("runoff", 1.0), ("sed", 2.0)]))
            .expect("first day accepted");
        assert!(first.is_empty());

        let second = accumulator
            .accumulate_day(day_2, surface(&[("runoff", 2.0), ("sed", 3.0)]))
            .expect("second day accepted");
        assert_eq!(second.emitted_rollups.len(), 1);
        assert_eq!(second.emitted_rollups[0].window, SummaryWindow::Daily);
        assert_eq!(
            second.emitted_rollups[0].key,
            SummaryWindowKey::Daily(day_1)
        );
        expect_symbol(&second.emitted_rollups[0].totals, "runoff", 1.0);
        expect_symbol(&second.emitted_rollups[0].totals, "sed", 2.0);

        let third = accumulator
            .accumulate_day(day_3, surface(&[("runoff", 4.0), ("sed", 5.0)]))
            .expect("third day accepted");
        assert_eq!(third.emitted_rollups.len(), 3);

        let daily = &third.emitted_rollups[0];
        assert_eq!(daily.window, SummaryWindow::Daily);
        assert_eq!(daily.key, SummaryWindowKey::Daily(day_2));
        assert_eq!(daily.status.phase(), SimulationPhase::SummaryAccumulator);
        assert_eq!(daily.status.message_id(), SUMMARY_DAILY_MESSAGE_ID);
        assert_eq!(
            daily.comparator_metadata.surface_class,
            ComparatorSurfaceClass::SingleOfeDailyWaterBalance
        );
        assert_eq!(
            daily.comparator_metadata.confidence_tier,
            ComparatorConfidenceTier::HigherConfidence
        );
        assert_eq!(
            daily.comparator_metadata.message_id,
            COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
        );
        expect_symbol(&daily.totals, "runoff", 2.0);
        expect_symbol(&daily.totals, "sed", 3.0);

        let monthly = &third.emitted_rollups[1];
        assert_eq!(monthly.window, SummaryWindow::Monthly);
        assert_eq!(
            monthly.key,
            SummaryWindowKey::Monthly {
                year: 2026,
                month: 12
            }
        );
        expect_symbol(&monthly.totals, "runoff", 3.0);
        expect_symbol(&monthly.totals, "sed", 5.0);
        assert_eq!(monthly.status.message_id(), SUMMARY_MONTHLY_MESSAGE_ID);

        let yearly = &third.emitted_rollups[2];
        assert_eq!(yearly.window, SummaryWindow::Yearly);
        assert_eq!(yearly.key, SummaryWindowKey::Yearly { year: 2026 });
        expect_symbol(&yearly.totals, "runoff", 3.0);
        expect_symbol(&yearly.totals, "sed", 5.0);
        assert_eq!(yearly.status.message_id(), SUMMARY_YEARLY_MESSAGE_ID);

        let final_outcome = accumulator.finalize().expect("finalize succeeds");
        assert_eq!(final_outcome.emitted_rollups.len(), 4);

        let eos_daily = &final_outcome.emitted_rollups[0];
        assert_eq!(eos_daily.window, SummaryWindow::Daily);
        assert_eq!(eos_daily.key, SummaryWindowKey::Daily(day_3));
        expect_symbol(&eos_daily.totals, "runoff", 4.0);
        expect_symbol(&eos_daily.totals, "sed", 5.0);

        let eos_monthly = &final_outcome.emitted_rollups[1];
        assert_eq!(eos_monthly.window, SummaryWindow::Monthly);
        assert_eq!(
            eos_monthly.key,
            SummaryWindowKey::Monthly {
                year: 2027,
                month: 1
            }
        );
        expect_symbol(&eos_monthly.totals, "runoff", 4.0);
        expect_symbol(&eos_monthly.totals, "sed", 5.0);

        let eos_yearly = &final_outcome.emitted_rollups[2];
        assert_eq!(eos_yearly.window, SummaryWindow::Yearly);
        assert_eq!(eos_yearly.key, SummaryWindowKey::Yearly { year: 2027 });
        expect_symbol(&eos_yearly.totals, "runoff", 4.0);
        expect_symbol(&eos_yearly.totals, "sed", 5.0);

        let eos = &final_outcome.emitted_rollups[3];
        assert_eq!(eos.window, SummaryWindow::EndOfSimulation);
        assert_eq!(eos.key, SummaryWindowKey::EndOfSimulation);
        expect_symbol(&eos.totals, "runoff", 7.0);
        expect_symbol(&eos.totals, "sed", 10.0);
        assert_eq!(eos.status.message_id(), SUMMARY_EOS_MESSAGE_ID);

        assert_eq!(accumulator.sample_count(), 0);
        assert_eq!(accumulator.current_day(), None);
    }

    #[test]
    fn accumulates_multiple_samples_within_same_day_before_rollup() {
        let day_1 = CalendarDay::new(2026, 5, 12).expect("valid day");
        let day_2 = CalendarDay::new(2026, 5, 13).expect("valid day");

        let mut accumulator = accumulator();

        let first = accumulator
            .accumulate_day(day_1, surface(&[("runoff", 1.5)]))
            .expect("first sample accepted");
        assert!(first.is_empty());

        let second = accumulator
            .accumulate_day(day_1, surface(&[("runoff", 2.5)]))
            .expect("second sample accepted");
        assert!(second.is_empty());

        let third = accumulator
            .accumulate_day(day_2, surface(&[("runoff", 1.0)]))
            .expect("day change accepted");

        assert_eq!(third.emitted_rollups.len(), 1);
        assert_eq!(third.emitted_rollups[0].window, SummaryWindow::Daily);
        assert_eq!(third.emitted_rollups[0].key, SummaryWindowKey::Daily(day_1));
        expect_symbol(&third.emitted_rollups[0].totals, "runoff", 4.0);
        assert_eq!(accumulator.sample_count(), 3);
    }

    #[test]
    fn rejects_non_monotonic_days() {
        let mut accumulator = accumulator();
        let day_2 = CalendarDay::new(2026, 1, 2).expect("valid day");
        let day_1 = CalendarDay::new(2026, 1, 1).expect("valid day");

        accumulator
            .accumulate_day(day_2, surface(&[("runoff", 1.0)]))
            .expect("first day accepted");

        let error = accumulator
            .accumulate_day(day_1, surface(&[("runoff", 1.0)]))
            .expect_err("non-monotonic day must error");

        assert!(matches!(
            error,
            SummaryAccumulatorError::NonMonotonicDate { .. }
        ));
    }

    #[test]
    fn rejects_invalid_calendar_dates() {
        let error = CalendarDay::new(2026, 2, 30).expect_err("invalid day");
        assert!(matches!(error, SummaryAccumulatorError::InvalidDate { .. }));

        let error = CalendarDay::new(2026, 0, 10).expect_err("invalid month");
        assert!(matches!(error, SummaryAccumulatorError::InvalidDate { .. }));
    }

    #[test]
    fn rejects_non_finite_scalar_inputs() {
        let error = SummaryScalarSurface::from_pairs([("runoff", f64::NAN)])
            .expect_err("non-finite scalar should fail");

        assert!(matches!(
            error,
            SummaryAccumulatorError::NonFiniteInput { .. }
        ));
    }

    #[test]
    fn rejects_empty_scalar_surfaces() {
        let mut accumulator = accumulator();
        let day = CalendarDay::new(2026, 1, 1).expect("valid day");

        let error = accumulator
            .accumulate_day(day, SummaryScalarSurface::default())
            .expect_err("empty surface should fail");

        assert_eq!(error, SummaryAccumulatorError::EmptyScalarSurface);
    }

    #[test]
    fn rejects_duplicate_symbol_in_pair_builder() {
        let error = SummaryScalarSurface::from_pairs([("runoff", 1.0), ("runoff", 2.0)])
            .expect_err("duplicate symbols should fail");

        assert!(matches!(
            error,
            SummaryAccumulatorError::DuplicateSymbol { .. }
        ));
    }

    #[test]
    fn finalize_without_samples_is_rejected() {
        let mut accumulator = accumulator();
        let error = accumulator
            .finalize()
            .expect_err("finalize without samples should fail");

        assert_eq!(error, SummaryAccumulatorError::FinalizeWithoutSamples);
    }

    #[test]
    fn invalid_comparator_routing_is_rejected_at_construction() {
        let error = SummaryAccumulator::new(ComparatorTierRoutingRequest::new(
            ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
            None,
        ))
        .expect_err("invalid routing request should fail");

        assert!(matches!(
            error,
            SummaryAccumulatorError::ComparatorMetadata(
                ComparatorTierRoutingError::MissingRequiredMetadata { .. }
            )
        ));
    }
}
