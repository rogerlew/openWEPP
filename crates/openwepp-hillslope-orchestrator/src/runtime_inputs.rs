use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_climate_runtime_adapter::{
    SharedClimateDailyForcing as HillslopeClimateDailyForcing,
    SharedClimateRuntimeInputError as ClimateRuntimeInputError,
    SharedClimateRuntimeRequest as SharedHillslopeClimateRuntimeRequest,
    build_climate_runtime_request, select_day_forcing,
};
use openwepp_input_contract::parsers::{
    climate::ClimateFile,
    frost::FrostParseOutput,
    irrigation_depletion::{IrrigationDepletionFile, IrrigationPeriodData},
    irrigation_fixeddate::{FixedDateEvent, FixedDateIrrigationFile},
    management::{
        InitialScenarioData, ManagementParseOutput, PlantScenarioData, YearlyAnnualExtension,
        YearlyCroplandBranch, YearlyScenarioData,
    },
    slope::{SlopePoint, SlopeProfile},
    snow::SnowParseOutput,
    soil::SoilProfile,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, ClimateForcingSymbolSurface, ClimateForcingSymbolSurfaceError,
};

use crate::HillslopeWritebackSurface;

/// Typed errors for parser-to-hillslope runtime surface adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopeRuntimeInputError {
    MissingSoilOfe,
    MissingSoilLayer,
    MissingThetaResidual,
    MissingThetaFieldCapacity,
    NonFiniteProfileDepth {
        value_mm: f64,
    },
    NonPositiveProfileDepth {
        value_mm: f64,
    },
    NonFiniteTopLayerDepth {
        value_mm: f64,
    },
    NonPositiveTopLayerDepth {
        value_mm: f64,
    },
    NonFiniteThetaResidual {
        value: f64,
    },
    NonFiniteThetaFieldCapacity {
        value: f64,
    },
    SoilOfeCountMismatch {
        declared_ofe_count: usize,
        observed_ofes: usize,
    },
    SoilOfeCountOutOfRange {
        value: usize,
    },
    SoilLayerCountMismatch {
        ofe_index: usize,
        declared_nsl: usize,
        observed_layers: usize,
    },
    SoilLayerCountOutOfRange {
        ofe_index: usize,
        value: usize,
    },
    NonFiniteLayerDepth {
        ofe_index: usize,
        layer_index: usize,
        value_mm: f64,
    },
    NonPositiveLayerDepth {
        ofe_index: usize,
        layer_index: usize,
        value_mm: f64,
    },
    NonMonotoneLayerDepth {
        ofe_index: usize,
        upper_layer_index: usize,
        upper_depth_mm: f64,
        lower_layer_index: usize,
        lower_depth_mm: f64,
    },
    MissingSaturatedConductivity {
        ofe_index: usize,
        layer_index: usize,
    },
    NonFiniteSaturatedConductivity {
        ofe_index: usize,
        layer_index: usize,
        value_mm_h: f64,
    },
    NonPositiveSaturatedConductivity {
        ofe_index: usize,
        layer_index: usize,
        value_mm_h: f64,
    },
    MissingSlopeOfe,
    SlopeOfeCountMismatch {
        declared_ofe_count: usize,
        observed_ofes: usize,
    },
    SlopeOfeCountOutOfRange {
        value: usize,
    },
    SlopePointCountMismatch {
        ofe_index: usize,
        declared_nslpts: usize,
        observed_points: usize,
    },
    SlopePointCountOutOfRange {
        ofe_index: usize,
        value: usize,
    },
    InsufficientSlopePoints {
        ofe_index: usize,
        observed_points: usize,
    },
    NonFiniteSlopeLength {
        ofe_index: usize,
        value_m: f64,
    },
    NonPositiveSlopeLength {
        ofe_index: usize,
        value_m: f64,
    },
    NonFiniteXinput {
        ofe_index: usize,
        point_index: usize,
        value: f64,
    },
    NonFiniteSlpinp {
        ofe_index: usize,
        point_index: usize,
        value: f64,
    },
    NonMonotoneXinput {
        ofe_index: usize,
        left_point_index: usize,
        left_value: f64,
        right_point_index: usize,
        right_value: f64,
    },
    NonFiniteDerivedAverageSlope {
        ofe_index: usize,
        value: f64,
    },
    NonPositiveDerivedAverageSlope {
        ofe_index: usize,
        value: f64,
    },
    NonFiniteDerivedSlopeLength {
        ofe_index: usize,
        value_m: f64,
    },
    NonPositiveDerivedSlopeLength {
        ofe_index: usize,
        value_m: f64,
    },
    ManagementTopologyCountMismatch {
        expected_ofes: usize,
        schedule_initial_refs: usize,
    },
    ManagementScheduleSlotCountMismatch {
        expected_slots: usize,
        observed_slots: usize,
    },
    ManagementScheduleSlotArityMismatch {
        slot_index: usize,
        crop_slots: usize,
        yearly_refs: usize,
    },
    ManagementScheduleOfeIndexOutOfRange {
        slot_index: usize,
        ofe_index: usize,
        max_ofe_index: usize,
    },
    ManagementInitialReferenceOutOfRange {
        ofe_index: usize,
        initial_ref: usize,
        max_initial_ref: usize,
    },
    ManagementYearlyReferenceOutOfRange {
        slot_index: usize,
        crop_slot_index: usize,
        yearly_ref: usize,
        max_yearly_ref: usize,
    },
    UnsupportedPlLanduse {
        section: &'static str,
        value: usize,
    },
    UnsupportedPlManagementOption {
        field: &'static str,
        value: usize,
        allowed: &'static str,
    },
    NonFinitePlProjectionField {
        field: &'static str,
        slot_index: usize,
        crop_slot_index: usize,
        value: f64,
    },
    PlProjectionCountOutOfRange {
        field: &'static str,
        value: usize,
    },
    PlProjectionDayOutOfDomain {
        field: &'static str,
        slot_index: usize,
        crop_slot_index: usize,
        value: usize,
        allowed: &'static str,
    },
    PlAnnualExtensionMismatch {
        slot_index: usize,
        crop_slot_index: usize,
        resmgt: usize,
        expected: &'static str,
        observed: &'static str,
    },
    PlProjectionCardinalityInvalid {
        field: &'static str,
        slot_index: usize,
        crop_slot_index: usize,
        value: usize,
        expected: &'static str,
    },
    PlGrazingWindowOutOfDomain {
        slot_index: usize,
        crop_slot_index: usize,
        cycle_index: usize,
        gday: usize,
        gend: usize,
    },
    PlProjectionFieldOutOfDomain {
        field: &'static str,
        slot_index: usize,
        crop_slot_index: usize,
        value: f64,
        allowed: &'static str,
    },
    PlProjectionUnsupportedPayloadCombination {
        field: &'static str,
        slot_index: usize,
        crop_slot_index: usize,
        reason: &'static str,
    },
    NonFiniteSnowControl {
        field: &'static str,
        value: f64,
    },
    SnowControlOutOfDomain {
        field: &'static str,
        value: f64,
        allowed: &'static str,
    },
    NonFiniteFrostControl {
        field: &'static str,
        value: f64,
    },
    FrostControlOutOfDomain {
        field: &'static str,
        value: f64,
        allowed: &'static str,
    },
    MissingIrrigationScheduleField {
        field: &'static str,
    },
    NonFiniteIrrigationScheduleField {
        field: &'static str,
        value: f64,
    },
    IrrigationScheduleFieldOutOfDomain {
        field: &'static str,
        value: f64,
        allowed: &'static str,
    },
    IrrigationScheduleCountOutOfRange {
        field: &'static str,
        value: usize,
    },
}

impl HillslopeRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingSoilOfe => "HS-RUNTIME-E-001",
            Self::MissingSoilLayer => "HS-RUNTIME-E-002",
            Self::MissingThetaResidual => "HS-RUNTIME-E-003",
            Self::MissingThetaFieldCapacity => "HS-RUNTIME-E-004",
            Self::NonFiniteProfileDepth { .. } => "HS-RUNTIME-E-005",
            Self::NonPositiveProfileDepth { .. } => "HS-RUNTIME-E-006",
            Self::NonFiniteTopLayerDepth { .. } => "HS-RUNTIME-E-007",
            Self::NonPositiveTopLayerDepth { .. } => "HS-RUNTIME-E-008",
            Self::NonFiniteThetaResidual { .. } => "HS-RUNTIME-E-009",
            Self::NonFiniteThetaFieldCapacity { .. } => "HS-RUNTIME-E-010",
            Self::MissingSlopeOfe => "HS-RUNTIME-E-011",
            Self::SlopeOfeCountMismatch { .. } => "HS-RUNTIME-E-012",
            Self::SlopeOfeCountOutOfRange { .. } => "HS-RUNTIME-E-013",
            Self::SlopePointCountMismatch { .. } => "HS-RUNTIME-E-014",
            Self::SlopePointCountOutOfRange { .. } => "HS-RUNTIME-E-015",
            Self::InsufficientSlopePoints { .. } => "HS-RUNTIME-E-016",
            Self::NonFiniteSlopeLength { .. } => "HS-RUNTIME-E-017",
            Self::NonPositiveSlopeLength { .. } => "HS-RUNTIME-E-018",
            Self::NonFiniteXinput { .. } => "HS-RUNTIME-E-019",
            Self::NonFiniteSlpinp { .. } => "HS-RUNTIME-E-020",
            Self::NonMonotoneXinput { .. } => "HS-RUNTIME-E-021",
            Self::NonFiniteDerivedAverageSlope { .. } => "HS-RUNTIME-E-022",
            Self::NonPositiveDerivedAverageSlope { .. } => "HS-RUNTIME-E-023",
            Self::NonFiniteDerivedSlopeLength { .. } => "HS-RUNTIME-E-024",
            Self::NonPositiveDerivedSlopeLength { .. } => "HS-RUNTIME-E-025",
            Self::SoilOfeCountMismatch { .. } => "HS-RUNTIME-E-026",
            Self::SoilOfeCountOutOfRange { .. } => "HS-RUNTIME-E-027",
            Self::SoilLayerCountMismatch { .. } => "HS-RUNTIME-E-028",
            Self::SoilLayerCountOutOfRange { .. } => "HS-RUNTIME-E-029",
            Self::NonFiniteLayerDepth { .. } => "HS-RUNTIME-E-030",
            Self::NonPositiveLayerDepth { .. } => "HS-RUNTIME-E-031",
            Self::NonMonotoneLayerDepth { .. } => "HS-RUNTIME-E-032",
            Self::MissingSaturatedConductivity { .. } => "HS-RUNTIME-E-033",
            Self::NonFiniteSaturatedConductivity { .. } => "HS-RUNTIME-E-034",
            Self::NonPositiveSaturatedConductivity { .. } => "HS-RUNTIME-E-035",
            Self::ManagementTopologyCountMismatch { .. } => "HS-RUNTIME-E-036",
            Self::ManagementScheduleSlotCountMismatch { .. } => "HS-RUNTIME-E-037",
            Self::ManagementScheduleSlotArityMismatch { .. } => "HS-RUNTIME-E-038",
            Self::ManagementInitialReferenceOutOfRange { .. } => "HS-RUNTIME-E-039",
            Self::ManagementYearlyReferenceOutOfRange { .. } => "HS-RUNTIME-E-040",
            Self::UnsupportedPlLanduse { .. } => "HS-RUNTIME-E-041",
            Self::UnsupportedPlManagementOption { .. } => "HS-RUNTIME-E-042",
            Self::NonFinitePlProjectionField { .. } => "HS-RUNTIME-E-043",
            Self::PlProjectionCountOutOfRange { .. } => "HS-RUNTIME-E-044",
            Self::ManagementScheduleOfeIndexOutOfRange { .. } => "HS-RUNTIME-E-045",
            Self::PlProjectionDayOutOfDomain { .. } => "HS-RUNTIME-E-046",
            Self::PlAnnualExtensionMismatch { .. } => "HS-RUNTIME-E-047",
            Self::PlProjectionCardinalityInvalid { .. } => "HS-RUNTIME-E-048",
            Self::PlGrazingWindowOutOfDomain { .. } => "HS-RUNTIME-E-049",
            Self::PlProjectionFieldOutOfDomain { .. } => "HS-RUNTIME-E-050",
            Self::PlProjectionUnsupportedPayloadCombination { .. } => "HS-RUNTIME-E-051",
            Self::NonFiniteSnowControl { .. } => "HS-RUNTIME-E-052",
            Self::SnowControlOutOfDomain { .. } => "HS-RUNTIME-E-053",
            Self::NonFiniteFrostControl { .. } => "HS-RUNTIME-E-054",
            Self::FrostControlOutOfDomain { .. } => "HS-RUNTIME-E-055",
            Self::MissingIrrigationScheduleField { .. } => "HS-RUNTIME-E-056",
            Self::NonFiniteIrrigationScheduleField { .. } => "HS-RUNTIME-E-057",
            Self::IrrigationScheduleFieldOutOfDomain { .. } => "HS-RUNTIME-E-058",
            Self::IrrigationScheduleCountOutOfRange { .. } => "HS-RUNTIME-E-059",
        }
    }
}

impl fmt::Display for HillslopeRuntimeInputError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSoilOfe => {
                write!(f, "{}: soil profile contains no OFE blocks", self.code())
            }
            Self::MissingSoilLayer => {
                write!(f, "{}: primary OFE contains no soil layers", self.code())
            }
            Self::MissingThetaResidual => write!(
                f,
                "{}: primary soil layer missing required theta_r_rosetta (thetdr)",
                self.code()
            ),
            Self::MissingThetaFieldCapacity => write!(
                f,
                "{}: primary soil layer missing required fc_rosetta (thetfc)",
                self.code()
            ),
            Self::NonFiniteProfileDepth { value_mm } => write!(
                f,
                "{}: non-finite soil profile depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonPositiveProfileDepth { value_mm } => write!(
                f,
                "{}: non-positive soil profile depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonFiniteTopLayerDepth { value_mm } => write!(
                f,
                "{}: non-finite top-layer depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonPositiveTopLayerDepth { value_mm } => write!(
                f,
                "{}: non-positive top-layer depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonFiniteThetaResidual { value } => {
                write!(f, "{}: non-finite thetdr value {}", self.code(), value)
            }
            Self::NonFiniteThetaFieldCapacity { value } => {
                write!(f, "{}: non-finite thetfc value {}", self.code(), value)
            }
            Self::SoilOfeCountMismatch {
                declared_ofe_count,
                observed_ofes,
            } => write!(
                f,
                "{}: soil ntemp {} does not match observed OFE blocks {}",
                self.code(),
                declared_ofe_count,
                observed_ofes
            ),
            Self::SoilOfeCountOutOfRange { value } => write!(
                f,
                "{}: soil OFE count {} exceeds lossless conversion range",
                self.code(),
                value
            ),
            Self::SoilLayerCountMismatch {
                ofe_index,
                declared_nsl,
                observed_layers,
            } => write!(
                f,
                "{}: soil OFE {} declares nsl={} but contains {} layer rows",
                self.code(),
                ofe_index,
                declared_nsl,
                observed_layers
            ),
            Self::SoilLayerCountOutOfRange { ofe_index, value } => write!(
                f,
                "{}: soil OFE {} nsl {} exceeds lossless conversion range",
                self.code(),
                ofe_index,
                value
            ),
            Self::NonFiniteLayerDepth {
                ofe_index,
                layer_index,
                value_mm,
            } => write!(
                f,
                "{}: soil OFE {} layer {} has non-finite depth_mm {}",
                self.code(),
                ofe_index,
                layer_index,
                value_mm
            ),
            Self::NonPositiveLayerDepth {
                ofe_index,
                layer_index,
                value_mm,
            } => write!(
                f,
                "{}: soil OFE {} layer {} has non-positive depth_mm {}",
                self.code(),
                ofe_index,
                layer_index,
                value_mm
            ),
            Self::NonMonotoneLayerDepth {
                ofe_index,
                upper_layer_index,
                upper_depth_mm,
                lower_layer_index,
                lower_depth_mm,
            } => write!(
                f,
                "{}: soil OFE {} layer depth must increase strictly (layer {}={}mm -> layer {}={}mm)",
                self.code(),
                ofe_index,
                upper_layer_index,
                upper_depth_mm,
                lower_layer_index,
                lower_depth_mm
            ),
            Self::MissingSaturatedConductivity {
                ofe_index,
                layer_index,
            } => write!(
                f,
                "{}: soil OFE {} layer {} missing required ksat (ssc) value",
                self.code(),
                ofe_index,
                layer_index
            ),
            Self::NonFiniteSaturatedConductivity {
                ofe_index,
                layer_index,
                value_mm_h,
            } => write!(
                f,
                "{}: soil OFE {} layer {} has non-finite ksat_mm_h {}",
                self.code(),
                ofe_index,
                layer_index,
                value_mm_h
            ),
            Self::NonPositiveSaturatedConductivity {
                ofe_index,
                layer_index,
                value_mm_h,
            } => write!(
                f,
                "{}: soil OFE {} layer {} has non-positive ksat_mm_h {}",
                self.code(),
                ofe_index,
                layer_index,
                value_mm_h
            ),
            Self::MissingSlopeOfe => {
                write!(f, "{}: slope profile contains no OFE blocks", self.code())
            }
            Self::SlopeOfeCountMismatch {
                declared_ofe_count,
                observed_ofes,
            } => write!(
                f,
                "{}: slope ofe_count {} does not match observed OFE blocks {}",
                self.code(),
                declared_ofe_count,
                observed_ofes
            ),
            Self::SlopeOfeCountOutOfRange { value } => write!(
                f,
                "{}: slope OFE count {} exceeds lossless conversion range",
                self.code(),
                value
            ),
            Self::SlopePointCountMismatch {
                ofe_index,
                declared_nslpts,
                observed_points,
            } => write!(
                f,
                "{}: OFE {} declares nslpts={} but contains {} points",
                self.code(),
                ofe_index,
                declared_nslpts,
                observed_points
            ),
            Self::SlopePointCountOutOfRange { ofe_index, value } => write!(
                f,
                "{}: OFE {} nslpts {} exceeds lossless conversion range",
                self.code(),
                ofe_index,
                value
            ),
            Self::InsufficientSlopePoints {
                ofe_index,
                observed_points,
            } => write!(
                f,
                "{}: OFE {} requires at least 2 slope points; observed {}",
                self.code(),
                ofe_index,
                observed_points
            ),
            Self::NonFiniteSlopeLength { ofe_index, value_m } => write!(
                f,
                "{}: OFE {} has non-finite slplen value {}",
                self.code(),
                ofe_index,
                value_m
            ),
            Self::NonPositiveSlopeLength { ofe_index, value_m } => write!(
                f,
                "{}: OFE {} has non-positive slplen value {}",
                self.code(),
                ofe_index,
                value_m
            ),
            Self::NonFiniteXinput {
                ofe_index,
                point_index,
                value,
            } => write!(
                f,
                "{}: OFE {} point {} has non-finite xinput {}",
                self.code(),
                ofe_index,
                point_index,
                value
            ),
            Self::NonFiniteSlpinp {
                ofe_index,
                point_index,
                value,
            } => write!(
                f,
                "{}: OFE {} point {} has non-finite slpinp {}",
                self.code(),
                ofe_index,
                point_index,
                value
            ),
            Self::NonMonotoneXinput {
                ofe_index,
                left_point_index,
                left_value,
                right_point_index,
                right_value,
            } => write!(
                f,
                "{}: OFE {} xinput must be monotonic non-decreasing (point {}={} -> point {}={})",
                self.code(),
                ofe_index,
                left_point_index,
                left_value,
                right_point_index,
                right_value
            ),
            Self::NonFiniteDerivedAverageSlope { ofe_index, value } => write!(
                f,
                "{}: OFE {} derived avgslp is non-finite ({})",
                self.code(),
                ofe_index,
                value
            ),
            Self::NonPositiveDerivedAverageSlope { ofe_index, value } => write!(
                f,
                "{}: OFE {} derived avgslp must be > 0, observed {}",
                self.code(),
                ofe_index,
                value
            ),
            Self::NonFiniteDerivedSlopeLength { ofe_index, value_m } => write!(
                f,
                "{}: OFE {} derived slope length (terminal xinput) is non-finite ({})",
                self.code(),
                ofe_index,
                value_m
            ),
            Self::NonPositiveDerivedSlopeLength { ofe_index, value_m } => write!(
                f,
                "{}: OFE {} derived slope length (terminal xinput) must be > 0, observed {}",
                self.code(),
                ofe_index,
                value_m
            ),
            Self::ManagementTopologyCountMismatch {
                expected_ofes,
                schedule_initial_refs,
            } => write!(
                f,
                "{}: management topology count {} does not match schedule OFE initial-ref count {}",
                self.code(),
                expected_ofes,
                schedule_initial_refs
            ),
            Self::ManagementScheduleSlotCountMismatch {
                expected_slots,
                observed_slots,
            } => write!(
                f,
                "{}: management schedule slot count mismatch: expected {}, observed {}",
                self.code(),
                expected_slots,
                observed_slots
            ),
            Self::ManagementScheduleSlotArityMismatch {
                slot_index,
                crop_slots,
                yearly_refs,
            } => write!(
                f,
                "{}: management slot {} crop-slot arity mismatch: crop_slots={}, yearly_refs={}",
                self.code(),
                slot_index,
                crop_slots,
                yearly_refs
            ),
            Self::ManagementScheduleOfeIndexOutOfRange {
                slot_index,
                ofe_index,
                max_ofe_index,
            } => write!(
                f,
                "{}: management slot {} OFE index {} exceeds max {}",
                self.code(),
                slot_index,
                ofe_index,
                max_ofe_index
            ),
            Self::ManagementInitialReferenceOutOfRange {
                ofe_index,
                initial_ref,
                max_initial_ref,
            } => write!(
                f,
                "{}: OFE {} initial reference {} exceeds max {}",
                self.code(),
                ofe_index,
                initial_ref,
                max_initial_ref
            ),
            Self::ManagementYearlyReferenceOutOfRange {
                slot_index,
                crop_slot_index,
                yearly_ref,
                max_yearly_ref,
            } => write!(
                f,
                "{}: slot {} crop-slot {} yearly reference {} exceeds max {}",
                self.code(),
                slot_index,
                crop_slot_index,
                yearly_ref,
                max_yearly_ref
            ),
            Self::UnsupportedPlLanduse { section, value } => write!(
                f,
                "{}: unsupported PL landuse {} in {}",
                self.code(),
                value,
                section
            ),
            Self::UnsupportedPlManagementOption {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: unsupported PL management option {}={} (allowed {})",
                self.code(),
                field,
                value,
                allowed
            ),
            Self::NonFinitePlProjectionField {
                field,
                slot_index,
                crop_slot_index,
                value,
            } => write!(
                f,
                "{}: non-finite PL projection field {} at slot {} crop-slot {} ({})",
                self.code(),
                field,
                slot_index,
                crop_slot_index,
                value
            ),
            Self::PlProjectionCountOutOfRange { field, value } => write!(
                f,
                "{}: PL projection count {}={} exceeds lossless conversion range",
                self.code(),
                field,
                value
            ),
            Self::PlProjectionDayOutOfDomain {
                field,
                slot_index,
                crop_slot_index,
                value,
                allowed,
            } => write!(
                f,
                "{}: PL projection day field {} at slot {} crop-slot {} has invalid value {} (allowed {})",
                self.code(),
                field,
                slot_index,
                crop_slot_index,
                value,
                allowed
            ),
            Self::PlAnnualExtensionMismatch {
                slot_index,
                crop_slot_index,
                resmgt,
                expected,
                observed,
            } => write!(
                f,
                "{}: annual extension mismatch at slot {} crop-slot {} for resmgt {} (expected {}, observed {})",
                self.code(),
                slot_index,
                crop_slot_index,
                resmgt,
                expected,
                observed
            ),
            Self::PlProjectionCardinalityInvalid {
                field,
                slot_index,
                crop_slot_index,
                value,
                expected,
            } => write!(
                f,
                "{}: invalid cardinality for {} at slot {} crop-slot {} (value {}, expected {})",
                self.code(),
                field,
                slot_index,
                crop_slot_index,
                value,
                expected
            ),
            Self::PlGrazingWindowOutOfDomain {
                slot_index,
                crop_slot_index,
                cycle_index,
                gday,
                gend,
            } => write!(
                f,
                "{}: invalid grazing window at slot {} crop-slot {} cycle {} (gday {} must be < gend {})",
                self.code(),
                slot_index,
                crop_slot_index,
                cycle_index,
                gday,
                gend
            ),
            Self::PlProjectionFieldOutOfDomain {
                field,
                slot_index,
                crop_slot_index,
                value,
                allowed,
            } => write!(
                f,
                "{}: PL projection field {} at slot {} crop-slot {} is out of domain ({}, allowed {})",
                self.code(),
                field,
                slot_index,
                crop_slot_index,
                value,
                allowed
            ),
            Self::PlProjectionUnsupportedPayloadCombination {
                field,
                slot_index,
                crop_slot_index,
                reason,
            } => write!(
                f,
                "{}: unsupported PL payload combination for {} at slot {} crop-slot {} ({})",
                self.code(),
                field,
                slot_index,
                crop_slot_index,
                reason
            ),
            Self::NonFiniteSnowControl { field, value } => write!(
                f,
                "{}: non-finite snow control {}={}",
                self.code(),
                field,
                value
            ),
            Self::SnowControlOutOfDomain {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: snow control {} is out of domain ({}, allowed {})",
                self.code(),
                field,
                value,
                allowed
            ),
            Self::NonFiniteFrostControl { field, value } => write!(
                f,
                "{}: non-finite frost control {}={}",
                self.code(),
                field,
                value
            ),
            Self::FrostControlOutOfDomain {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: frost control {} is out of domain ({}, allowed {})",
                self.code(),
                field,
                value,
                allowed
            ),
            Self::MissingIrrigationScheduleField { field } => write!(
                f,
                "{}: missing required irrigation schedule field {}",
                self.code(),
                field
            ),
            Self::NonFiniteIrrigationScheduleField { field, value } => write!(
                f,
                "{}: non-finite irrigation schedule field {}={}",
                self.code(),
                field,
                value
            ),
            Self::IrrigationScheduleFieldOutOfDomain {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: irrigation schedule field {} is out of domain ({}, allowed {})",
                self.code(),
                field,
                value,
                allowed
            ),
            Self::IrrigationScheduleCountOutOfRange { field, value } => write!(
                f,
                "{}: irrigation schedule count {}={} exceeds lossless conversion range",
                self.code(),
                field,
                value
            ),
        }
    }
}

impl Error for HillslopeRuntimeInputError {}

/// Typed hillslope climate runtime request with precomputed boundary alias
/// projections for forcing series surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeClimateRuntimeRequest {
    shared: SharedHillslopeClimateRuntimeRequest,
    day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
}

/// Typed PL management runtime projection surfaces (`PL-MAN-SEAM-001`).
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopePlRuntimeSurfaces {
    pub pl_schedule_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub pl_growth_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub pl_decomp_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl HillslopePlRuntimeSurfaces {
    #[must_use]
    pub fn merged_state_surface(&self) -> BTreeMap<BoundarySymbol, BoundaryValue> {
        let mut merged = BTreeMap::new();
        merged.extend(self.pl_schedule_surface.clone());
        merged.extend(self.pl_growth_surface.clone());
        merged.extend(self.pl_decomp_surface.clone());
        merged
    }
}

/// Build strict typed PL runtime projection surfaces from parsed management
/// input (`PL-MAN-SEAM-001`).
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` for unsupported branches, dangling
/// references, non-finite required controls, or schedule closure violations.
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_pl_runtime_surfaces_from_management(
    management: &ManagementParseOutput,
) -> Result<HillslopePlRuntimeSurfaces, HillslopeRuntimeInputError> {
    if management.topology_count != management.schedule.ofe_initial_refs.len() {
        return Err(
            HillslopeRuntimeInputError::ManagementTopologyCountMismatch {
                expected_ofes: management.topology_count,
                schedule_initial_refs: management.schedule.ofe_initial_refs.len(),
            },
        );
    }

    let expected_slots = management
        .schedule
        .rotation_repeats
        .checked_mul(management.schedule.rotation_years)
        .and_then(|value| value.checked_mul(management.topology_count))
        .ok_or(HillslopeRuntimeInputError::PlProjectionCountOutOfRange {
            field: "schedule.expected_slots",
            value: usize::MAX,
        })?;

    if management.schedule.slots.len() != expected_slots {
        return Err(
            HillslopeRuntimeInputError::ManagementScheduleSlotCountMismatch {
                expected_slots,
                observed_slots: management.schedule.slots.len(),
            },
        );
    }

    let mut pl_schedule_surface = BTreeMap::new();
    let mut pl_growth_surface = BTreeMap::new();
    let mut pl_decomp_surface = BTreeMap::new();

    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_nofe"),
        BoundaryValue::scalar(usize_to_f64("pl_schedule_nofe", management.topology_count)?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_repeats"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_repeats",
            management.schedule.rotation_repeats,
        )?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_years"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_years",
            management.schedule.rotation_years,
        )?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_count"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_slot_count",
            management.schedule.slots.len(),
        )?),
    );
    // Explicit scheduler preconditions from PL02 contract/baseline ordering.
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(1.0),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_growth_after_decomp"),
        BoundaryValue::scalar(1.0),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(1.0),
    );
    for (symbol, value) in [
        ("sumgdd", 0.0),
        ("vdmt", 0.0),
        ("cancov", 0.0),
        ("lai", 0.0),
        ("rtmass", 0.0),
        ("rtd", 0.0),
        ("hia", 0.0),
    ] {
        pl_growth_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    for (ofe_position, initial_ref) in management.schedule.ofe_initial_refs.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        if *initial_ref == 0 || *initial_ref > management.registries.initials.len() {
            return Err(
                HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                    ofe_index,
                    initial_ref: *initial_ref,
                    max_initial_ref: management.registries.initials.len(),
                },
            );
        }
        let initial = &management.registries.initials[*initial_ref - 1];
        if initial.meta.landuse != 1 {
            return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: initial.meta.landuse,
            });
        }
        let InitialScenarioData::Cropland(initial_data) = &initial.data;

        pl_schedule_surface.insert(
            pl_schedule_ofe_symbol("initial_ref", ofe_index),
            BoundaryValue::scalar(usize_to_f64("initial_ref", *initial_ref)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_ofe_symbol("lanuse", ofe_index),
            BoundaryValue::scalar(usize_to_f64("lanuse", initial.meta.landuse)?),
        );

        pl_growth_surface.insert(
            pl_growth_ofe_symbol("imngmt_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("imngmt_seed", initial_data.imngmt)?),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("rtyp_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("rtyp_seed", initial_data.rtyp)?),
        );

        let sumrtm = initial_data.terminal_line[0];
        if !sumrtm.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "sumrtm_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value: sumrtm,
            });
        }
        let sumsrm = initial_data.terminal_line[1];
        if !sumsrm.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "sumsrm_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value: sumsrm,
            });
        }
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("iresd_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("iresd_seed", initial_data.iresd)?),
        );
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("sumrtm_seed", ofe_index),
            BoundaryValue::scalar(sumrtm),
        );
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("sumsrm_seed", ofe_index),
            BoundaryValue::scalar(sumsrm),
        );

        if let Some(understory) = initial_data.understory_line {
            let usinrcol = understory[0];
            let usrilcol = understory[1];
            if !usinrcol.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                    field: "usinrcol_seed",
                    slot_index: 0,
                    crop_slot_index: 0,
                    value: usinrcol,
                });
            }
            if !usrilcol.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                    field: "usrilcol_seed",
                    slot_index: 0,
                    crop_slot_index: 0,
                    value: usrilcol,
                });
            }
            pl_decomp_surface.insert(
                pl_decomp_ofe_symbol("usinrcol_seed", ofe_index),
                BoundaryValue::scalar(usinrcol),
            );
            pl_decomp_surface.insert(
                pl_decomp_ofe_symbol("usrilcol_seed", ofe_index),
                BoundaryValue::scalar(usrilcol),
            );
        }

        if ofe_index == 1 {
            pl_schedule_surface.insert(
                BoundarySymbol::from("lanuse"),
                BoundaryValue::scalar(usize_to_f64("lanuse", initial.meta.landuse)?),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("imngmt_seed"),
                BoundaryValue::scalar(usize_to_f64("imngmt_seed", initial_data.imngmt)?),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("iresd_seed"),
                BoundaryValue::scalar(usize_to_f64("iresd_seed", initial_data.iresd)?),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("sumrtm_seed"),
                BoundaryValue::scalar(sumrtm),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("sumsrm_seed"),
                BoundaryValue::scalar(sumsrm),
            );
        }
    }

    for (slot_position, slot) in management.schedule.slots.iter().enumerate() {
        let slot_index = slot_position + 1;
        if slot.crop_slots != slot.yearly_refs.len() {
            return Err(
                HillslopeRuntimeInputError::ManagementScheduleSlotArityMismatch {
                    slot_index,
                    crop_slots: slot.crop_slots,
                    yearly_refs: slot.yearly_refs.len(),
                },
            );
        }

        let ofe_index = slot.ofe_index + 1;
        if ofe_index == 0 || ofe_index > management.topology_count {
            return Err(
                HillslopeRuntimeInputError::ManagementScheduleOfeIndexOutOfRange {
                    slot_index,
                    ofe_index,
                    max_ofe_index: management.topology_count,
                },
            );
        }

        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("rotation_index", slot_index),
            BoundaryValue::scalar(usize_to_f64("rotation_index", slot.rotation_index + 1)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("year_in_rotation", slot_index),
            BoundaryValue::scalar(usize_to_f64("year_in_rotation", slot.year_in_rotation + 1)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("ofe_index", slot_index),
            BoundaryValue::scalar(usize_to_f64("ofe_index", ofe_index)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("crop_slots", slot_index),
            BoundaryValue::scalar(usize_to_f64("crop_slots", slot.crop_slots)?),
        );

        for (crop_slot_position, yearly_ref) in slot.yearly_refs.iter().enumerate() {
            let crop_slot_index = crop_slot_position + 1;
            if *yearly_ref == 0 || *yearly_ref > management.registries.yearlies.len() {
                return Err(
                    HillslopeRuntimeInputError::ManagementYearlyReferenceOutOfRange {
                        slot_index,
                        crop_slot_index,
                        yearly_ref: *yearly_ref,
                        max_yearly_ref: management.registries.yearlies.len(),
                    },
                );
            }
            let yearly = &management.registries.yearlies[*yearly_ref - 1];
            if yearly.meta.landuse != 1 {
                return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
                    section: "yearly",
                    value: yearly.meta.landuse,
                });
            }
            let YearlyScenarioData::Cropland(cropland) = &yearly.data;

            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("yearly_ref", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("yearly_ref", *yearly_ref)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("lanuse", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("lanuse", yearly.meta.landuse)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("itype", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("tilseq", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("tilseq", cropland.tilseq)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("conset", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("conset", cropland.conset)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("drset", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("drset", cropland.drset)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
            );

            pl_growth_surface.insert(
                pl_growth_slot_crop_symbol("itype", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
            );
            pl_growth_surface.insert(
                pl_growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
            );

            if cropland.itype == 0 || cropland.itype > management.registries.plants.len() {
                return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                    field: "itype",
                    slot_index,
                    crop_slot_index,
                    value: usize_to_f64("itype", cropland.itype)?,
                    allowed: "1..=plant_scenario_count",
                });
            }
            let plant = &management.registries.plants[cropland.itype - 1];
            let PlantScenarioData::Cropland(plant_cropland) = &plant.data;
            project_growth_equation_symbols(
                &mut pl_growth_surface,
                slot_index,
                crop_slot_index,
                plant_cropland,
            )?;
            project_decomposition_equation_symbols(
                &mut pl_decomp_surface,
                slot_index,
                crop_slot_index,
                plant_cropland,
            )?;
            if slot_index == 1 && crop_slot_index == 1 {
                project_primary_growth_equation_aliases(&mut pl_growth_surface, plant_cropland)?;
                project_primary_decomposition_equation_aliases(
                    &mut pl_decomp_surface,
                    plant_cropland,
                )?;
            }

            match &cropland.branch {
                YearlyCroplandBranch::AnnualOrFallow(annual) => {
                    if !annual.rw.is_finite() {
                        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                            field: "rw",
                            slot_index,
                            crop_slot_index,
                            value: annual.rw,
                        });
                    }
                    let jdharv = validate_projection_day(
                        "jdharv",
                        slot_index,
                        crop_slot_index,
                        annual.jdharv,
                        false,
                    )?;
                    let jdplt = validate_projection_day(
                        "jdplt",
                        slot_index,
                        crop_slot_index,
                        annual.jdplt,
                        false,
                    )?;
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index),
                        BoundaryValue::scalar(annual.rw),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("resmgt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                    );
                    let annual_extension_projection = project_annual_extension_controls(
                        slot_index,
                        crop_slot_index,
                        annual.resmgt,
                        annual.extension.as_ref(),
                    )?;
                    project_annual_extension_symbols(
                        &mut pl_decomp_surface,
                        slot_index,
                        crop_slot_index,
                        &annual_extension_projection,
                    )?;

                    if slot_index == 1 && crop_slot_index == 1 {
                        pl_growth_surface.insert(
                            BoundarySymbol::from("itype"),
                            BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("imngmt"),
                            BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("jdharv"),
                            BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("jdplt"),
                            BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                        );
                        pl_growth_surface
                            .insert(BoundarySymbol::from("rw"), BoundaryValue::scalar(annual.rw));
                        pl_decomp_surface.insert(
                            BoundarySymbol::from("resmgt"),
                            BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                        );
                        project_primary_annual_extension_aliases(
                            &mut pl_decomp_surface,
                            &annual_extension_projection,
                        )?;
                    }
                }
                YearlyCroplandBranch::Perennial(perennial) => {
                    if perennial.mgtopt > 3 {
                        return Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                            field: "mgtopt",
                            value: perennial.mgtopt,
                            allowed: "1..3",
                        });
                    }
                    if !perennial.rw.is_finite() {
                        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                            field: "rw",
                            slot_index,
                            crop_slot_index,
                            value: perennial.rw,
                        });
                    }
                    let jdharv = validate_projection_day(
                        "jdharv",
                        slot_index,
                        crop_slot_index,
                        perennial.jdharv,
                        true,
                    )?;
                    let jdplt = validate_projection_day(
                        "jdplt",
                        slot_index,
                        crop_slot_index,
                        perennial.jdplt,
                        true,
                    )?;
                    let jdstop = validate_projection_day(
                        "jdstop",
                        slot_index,
                        crop_slot_index,
                        perennial.jdstop,
                        true,
                    )?;
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdstop", jdstop)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index),
                        BoundaryValue::scalar(perennial.rw),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("mgtopt", perennial.mgtopt)?),
                    );

                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("mgtopt", perennial.mgtopt)?),
                    );
                    let (ncut, ncycle) = match perennial.mgtopt {
                        1 => {
                            if perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                                        field: "ncut",
                                        slot_index,
                                        crop_slot_index,
                                        value: 0,
                                        expected: ">=1 for mgtopt=1",
                                    },
                                );
                            }
                            if !perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "grazing_cycles",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=1 requires empty grazing_cycles",
                                    },
                                );
                            }

                            project_perennial_cutday_symbols(
                                &mut pl_decomp_surface,
                                slot_index,
                                crop_slot_index,
                                perennial,
                            )?;
                            (perennial.cut_days.len(), 0)
                        }
                        2 => {
                            if perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                                        field: "ncycle",
                                        slot_index,
                                        crop_slot_index,
                                        value: 0,
                                        expected: ">=1 for mgtopt=2",
                                    },
                                );
                            }
                            if !perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "cut_days",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=2 requires empty cut_days",
                                    },
                                );
                            }

                            project_perennial_grazing_cycle_symbols(
                                &mut pl_decomp_surface,
                                slot_index,
                                crop_slot_index,
                                perennial,
                            )?;
                            (0, perennial.grazing_cycles.len())
                        }
                        3 => {
                            if !perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "cut_days",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=3 requires empty cut_days",
                                    },
                                );
                            }
                            if !perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "grazing_cycles",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=3 requires empty grazing_cycles",
                                    },
                                );
                            }
                            (0, 0)
                        }
                        _ => {
                            return Err(
                                HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                                    field: "mgtopt",
                                    value: perennial.mgtopt,
                                    allowed: "1..3",
                                },
                            );
                        }
                    };
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("ncut", ncut)?),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("ncycle", ncycle)?),
                    );
                }
            }
        }
    }

    Ok(HillslopePlRuntimeSurfaces {
        pl_schedule_surface,
        pl_growth_surface,
        pl_decomp_surface,
    })
}

/// Build one merged state surface from strict PL management projection
/// sub-surfaces.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when PL management projection fails.
pub fn build_hillslope_runtime_surface_from_management(
    management: &ManagementParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(management)?;
    Ok(HillslopeWritebackSurface {
        state_surface: pl_surfaces.merged_state_surface(),
        flux_surface: BTreeMap::new(),
    })
}

/// Build an orchestrator-owned hillslope runtime surface from parsed soil input.
///
/// This seam is strict by design: missing runtime-critical fields fail
/// explicitly instead of defaulting.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required parser outputs are
/// missing or non-finite.
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_runtime_surface_from_soil(
    soil: &SoilProfile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let primary_ofe = soil
        .ofes
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilOfe)?;
    let primary_top_layer = primary_ofe
        .layers
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?;

    let primary_profile_depth_mm = primary_ofe
        .layers
        .last()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?
        .depth_mm;
    if !primary_profile_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteProfileDepth {
            value_mm: primary_profile_depth_mm,
        });
    }
    if primary_profile_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveProfileDepth {
            value_mm: primary_profile_depth_mm,
        });
    }

    let primary_top_layer_depth_mm = primary_top_layer.depth_mm;
    if !primary_top_layer_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteTopLayerDepth {
            value_mm: primary_top_layer_depth_mm,
        });
    }
    if primary_top_layer_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveTopLayerDepth {
            value_mm: primary_top_layer_depth_mm,
        });
    }

    let primary_thetdr = primary_top_layer
        .theta_r_rosetta
        .ok_or(HillslopeRuntimeInputError::MissingThetaResidual)?;
    if !primary_thetdr.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual {
            value: primary_thetdr,
        });
    }

    let primary_thetfc = primary_top_layer
        .fc_rosetta
        .ok_or(HillslopeRuntimeInputError::MissingThetaFieldCapacity)?;
    if !primary_thetfc.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
            value: primary_thetfc,
        });
    }

    if soil.ntemp != soil.ofes.len() {
        return Err(HillslopeRuntimeInputError::SoilOfeCountMismatch {
            declared_ofe_count: soil.ntemp,
            observed_ofes: soil.ofes.len(),
        });
    }
    let ntemp = u32::try_from(soil.ntemp)
        .map_err(|_| HillslopeRuntimeInputError::SoilOfeCountOutOfRange { value: soil.ntemp })?;

    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("ntemp"),
        BoundaryValue::scalar(f64::from(ntemp)),
    );

    for (ofe_position, ofe) in soil.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        if ofe.nsl != ofe.layers.len() {
            return Err(HillslopeRuntimeInputError::SoilLayerCountMismatch {
                ofe_index,
                declared_nsl: ofe.nsl,
                observed_layers: ofe.layers.len(),
            });
        }
        let nsl = u32::try_from(ofe.nsl).map_err(|_| {
            HillslopeRuntimeInputError::SoilLayerCountOutOfRange {
                ofe_index,
                value: ofe.nsl,
            }
        })?;
        state_surface.insert(
            soil_ofe_symbol("nsl", ofe_index),
            BoundaryValue::scalar(f64::from(nsl)),
        );

        let mut previous_depth_mm = 0.0_f64;
        for (layer_position, layer) in ofe.layers.iter().enumerate() {
            let layer_index = layer_position + 1;
            let layer_depth_mm = layer.depth_mm;
            if !layer_depth_mm.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteLayerDepth {
                    ofe_index,
                    layer_index,
                    value_mm: layer_depth_mm,
                });
            }
            if layer_depth_mm <= 0.0 {
                return Err(HillslopeRuntimeInputError::NonPositiveLayerDepth {
                    ofe_index,
                    layer_index,
                    value_mm: layer_depth_mm,
                });
            }
            if layer_depth_mm <= previous_depth_mm {
                return Err(HillslopeRuntimeInputError::NonMonotoneLayerDepth {
                    ofe_index,
                    upper_layer_index: layer_index.saturating_sub(1),
                    upper_depth_mm: previous_depth_mm,
                    lower_layer_index: layer_index,
                    lower_depth_mm: layer_depth_mm,
                });
            }

            let layer_dg_m = (layer_depth_mm - previous_depth_mm) / 1_000.0;
            let layer_solthk_m = layer_depth_mm / 1_000.0;

            let layer_thetdr = layer
                .theta_r_rosetta
                .ok_or(HillslopeRuntimeInputError::MissingThetaResidual)?;
            if !layer_thetdr.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual {
                    value: layer_thetdr,
                });
            }

            let layer_thetfc = layer
                .fc_rosetta
                .ok_or(HillslopeRuntimeInputError::MissingThetaFieldCapacity)?;
            if !layer_thetfc.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
                    value: layer_thetfc,
                });
            }

            let layer_ksat_mm_h = layer.ksat_mm_h.ok_or(
                HillslopeRuntimeInputError::MissingSaturatedConductivity {
                    ofe_index,
                    layer_index,
                },
            )?;
            if !layer_ksat_mm_h.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteSaturatedConductivity {
                    ofe_index,
                    layer_index,
                    value_mm_h: layer_ksat_mm_h,
                });
            }
            if layer_ksat_mm_h <= 0.0 {
                return Err(
                    HillslopeRuntimeInputError::NonPositiveSaturatedConductivity {
                        ofe_index,
                        layer_index,
                        value_mm_h: layer_ksat_mm_h,
                    },
                );
            }
            let layer_ssc_m_s = layer_ksat_mm_h / 3.6e6;

            state_surface.insert(
                soil_ofe_layer_symbol("solthk", ofe_index, layer_index),
                BoundaryValue::scalar(layer_solthk_m),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("dg", ofe_index, layer_index),
                BoundaryValue::scalar(layer_dg_m),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("thetdr", ofe_index, layer_index),
                BoundaryValue::scalar(layer_thetdr),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("thetfc", ofe_index, layer_index),
                BoundaryValue::scalar(layer_thetfc),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("ssc", ofe_index, layer_index),
                BoundaryValue::scalar(layer_ssc_m_s),
            );

            if ofe_index == 1 {
                state_surface.insert(
                    soil_primary_layer_symbol("solthk", layer_index),
                    BoundaryValue::scalar(layer_solthk_m),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("dg", layer_index),
                    BoundaryValue::scalar(layer_dg_m),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("thetdr", layer_index),
                    BoundaryValue::scalar(layer_thetdr),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("thetfc", layer_index),
                    BoundaryValue::scalar(layer_thetfc),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("ssc", layer_index),
                    BoundaryValue::scalar(layer_ssc_m_s),
                );

                if layer_index == 1 {
                    state_surface.insert(
                        BoundarySymbol::from("dg"),
                        BoundaryValue::scalar(layer_dg_m),
                    );
                    state_surface.insert(
                        BoundarySymbol::from("thetdr"),
                        BoundaryValue::scalar(layer_thetdr),
                    );
                    state_surface.insert(
                        BoundarySymbol::from("thetfc"),
                        BoundaryValue::scalar(layer_thetfc),
                    );
                    state_surface.insert(
                        BoundarySymbol::from("ssc"),
                        BoundaryValue::scalar(layer_ssc_m_s),
                    );
                }
            }

            previous_depth_mm = layer_depth_mm;
        }

        if let Some(last_layer) = ofe.layers.last() {
            state_surface.insert(
                soil_ofe_symbol("solthk", ofe_index),
                BoundaryValue::scalar(last_layer.depth_mm / 1_000.0),
            );
        }

        if ofe_index == 1 {
            state_surface.insert(
                BoundarySymbol::from("nsl"),
                BoundaryValue::scalar(f64::from(nsl)),
            );
            state_surface.insert(
                BoundarySymbol::from("solthk"),
                BoundaryValue::scalar(primary_profile_depth_mm / 1_000.0),
            );
            state_surface.insert(
                BoundarySymbol::from("salb"),
                BoundaryValue::scalar(ofe.salb),
            );
        }
    }

    Ok(HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    })
}

/// Build an orchestrator-owned hillslope runtime surface from parsed slope
/// input.
///
/// Canonical slope symbols are projected with explicit index-qualified runtime
/// keys while preserving first-OFE aliases for continuity:
/// - per-OFE: `ofe{idx}_{nslpts|slplen|avgslp}`
/// - per-point: `ofe{idx}_{xinput|slpinp}_{point:04}`
/// - first-OFE aliases: `nslpts`, `slplen`, `avgslp`,
///   `{xinput|slpinp}_{point:04}`
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required slope parser outputs are
/// missing, inconsistent, non-finite, or violate runtime guard policy.
pub fn build_hillslope_runtime_surface_from_slope(
    slope: &SlopeProfile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    validate_slope_profile_shape(slope)?;

    let mut state_surface = BTreeMap::new();
    let ofe_count = u32::try_from(slope.ofe_count).map_err(|_| {
        HillslopeRuntimeInputError::SlopeOfeCountOutOfRange {
            value: slope.ofe_count,
        }
    })?;
    state_surface.insert(
        BoundarySymbol::from("nelem"),
        BoundaryValue::scalar(f64::from(ofe_count)),
    );
    state_surface.insert(
        BoundarySymbol::from("nwsofe"),
        BoundaryValue::scalar(f64::from(ofe_count)),
    );

    for (ofe_position, ofe) in slope.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        validate_slope_ofe_shape(ofe_index, ofe.nslpts, ofe.points.len())?;
        validate_slope_points(ofe_index, &ofe.points)?;

        let nslpts = u32::try_from(ofe.nslpts).map_err(|_| {
            HillslopeRuntimeInputError::SlopePointCountOutOfRange {
                ofe_index,
                value: ofe.nslpts,
            }
        })?;

        let slplen = ofe.slplen;
        if !slplen.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteSlopeLength {
                ofe_index,
                value_m: slplen,
            });
        }
        if slplen <= 0.0 {
            return Err(HillslopeRuntimeInputError::NonPositiveSlopeLength {
                ofe_index,
                value_m: slplen,
            });
        }

        let avgslp = derive_avgslp(ofe_index, &ofe.points)?;
        state_surface.insert(
            slope_ofe_symbol("nslpts", ofe_index),
            BoundaryValue::scalar(f64::from(nslpts)),
        );
        state_surface.insert(
            slope_ofe_symbol("slplen", ofe_index),
            BoundaryValue::scalar(slplen),
        );
        state_surface.insert(
            slope_ofe_symbol("avgslp", ofe_index),
            BoundaryValue::scalar(avgslp),
        );

        for (point_position, point) in ofe.points.iter().enumerate() {
            let point_index = point_position + 1;
            state_surface.insert(
                slope_ofe_point_symbol("xinput", ofe_index, point_index),
                BoundaryValue::scalar(point.xinput),
            );
            state_surface.insert(
                slope_ofe_point_symbol("slpinp", ofe_index, point_index),
                BoundaryValue::scalar(point.slpinp),
            );
        }

        if ofe_index == 1 {
            state_surface.insert(
                BoundarySymbol::from("nslpts"),
                BoundaryValue::scalar(f64::from(nslpts)),
            );
            state_surface.insert(
                BoundarySymbol::from("slplen"),
                BoundaryValue::scalar(slplen),
            );
            state_surface.insert(
                BoundarySymbol::from("avgslp"),
                BoundaryValue::scalar(avgslp),
            );

            for (point_position, point) in ofe.points.iter().enumerate() {
                let point_index = point_position + 1;
                state_surface.insert(
                    slope_primary_point_symbol("xinput", point_index),
                    BoundaryValue::scalar(point.xinput),
                );
                state_surface.insert(
                    slope_primary_point_symbol("slpinp", point_index),
                    BoundaryValue::scalar(point.slpinp),
                );
            }
        }
    }

    Ok(HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    })
}

/// Build a hillslope-owned climate runtime request from parser output
/// (`HS-CLIM-SEAM-001`).
///
/// Runtime policy at this seam enforces `datver=0.0` override (`iclig=0`) or
/// `datver>=4.0` (`iclig=1`).
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate parser output violates
/// runtime seam policy or numeric invariants.
pub fn build_hillslope_climate_runtime_request(
    climate: &ClimateFile,
) -> Result<HillslopeClimateRuntimeRequest, ClimateRuntimeInputError> {
    let shared = build_climate_runtime_request(climate)?;
    let mut day_symbol_surfaces = Vec::with_capacity(shared.daily_forcing.len());
    for forcing in &shared.daily_forcing {
        day_symbol_surfaces.push(build_hillslope_series_surface(forcing)?);
    }

    Ok(HillslopeClimateRuntimeRequest {
        shared,
        day_symbol_surfaces,
    })
}

/// Seed a hillslope runtime writeback surface with one climate forcing record.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when requested day index is invalid or
/// climate forcing cannot be losslessly projected onto runtime symbols.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_climate(
    runtime_surface: &mut HillslopeWritebackSurface,
    climate: &HillslopeClimateRuntimeRequest,
    day_index: usize,
) -> Result<(), ClimateRuntimeInputError> {
    let forcing = select_day_forcing(&climate.shared, day_index)?;
    let day_symbols = climate.day_symbol_surfaces.get(day_index).ok_or(
        ClimateRuntimeInputError::DayIndexOutOfRange {
            day_index,
            available: climate.day_symbol_surfaces.len(),
        },
    )?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("datver"),
        BoundaryValue::scalar(climate.shared.datver),
    );
    state_surface.insert(
        BoundarySymbol::from("iclig"),
        BoundaryValue::scalar(f64::from(climate.shared.iclig)),
    );
    state_surface.insert(
        BoundarySymbol::from("itemp"),
        BoundaryValue::scalar(f64::from(climate.shared.itemp)),
    );
    state_surface.insert(
        BoundarySymbol::from("ibrkpt"),
        BoundaryValue::scalar(f64::from(climate.shared.ibrkpt)),
    );
    state_surface.insert(
        BoundarySymbol::from("iwind"),
        BoundaryValue::scalar(f64::from(climate.shared.iwind)),
    );

    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                BoundaryValue::scalar(day.prcp),
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                BoundaryValue::scalar(day.stmdur),
            );
            state_surface.insert(
                BoundarySymbol::from("timep"),
                BoundaryValue::scalar(day.timep),
            );
            state_surface.insert(BoundarySymbol::from("ip"), BoundaryValue::scalar(day.ip));
            let ninten = u32::try_from(day.ninten).map_err(|_| {
                ClimateRuntimeInputError::BreakpointCountOutOfRange { value: day.ninten }
            })?;
            state_surface.insert(
                BoundarySymbol::from("ninten"),
                BoundaryValue::scalar(f64::from(ninten)),
            );
            state_surface.insert(
                BoundarySymbol::from("avrint"),
                BoundaryValue::scalar(day.avrint),
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                BoundaryValue::scalar(day.mxint),
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                BoundaryValue::scalar(day.tmax),
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                BoundaryValue::scalar(day.tmin),
            );
            state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(day.rad));
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                BoundaryValue::scalar(day.vwind),
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                BoundaryValue::scalar(day.wind),
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                BoundaryValue::scalar(day.tdpt),
            );
            insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
            insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
        }
        HillslopeClimateDailyForcing::Breakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("stmstr"),
                BoundaryValue::scalar(day.stmstr),
            );
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                BoundaryValue::scalar(day.prcp),
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                BoundaryValue::scalar(day.stmdur),
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                BoundaryValue::scalar(day.mxint),
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                BoundaryValue::scalar(day.tmax),
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                BoundaryValue::scalar(day.tmin),
            );
            state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(day.rad));
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                BoundaryValue::scalar(day.vwind),
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                BoundaryValue::scalar(day.wind),
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                BoundaryValue::scalar(day.tdpt),
            );

            let nbrkpt = u32::try_from(day.nbrkpt).map_err(|_| {
                ClimateRuntimeInputError::BreakpointCountOutOfRange { value: day.nbrkpt }
            })?;
            state_surface.insert(
                BoundarySymbol::from("nbrkpt"),
                BoundaryValue::scalar(f64::from(nbrkpt)),
            );

            insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
            insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
        }
    }

    Ok(())
}

/// Build a hillslope runtime surface from climate parser output and selected day
/// index.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate runtime request projection
/// fails.
pub fn build_hillslope_runtime_surface_from_climate(
    climate: &ClimateFile,
    day_index: usize,
) -> Result<HillslopeWritebackSurface, ClimateRuntimeInputError> {
    let request = build_hillslope_climate_runtime_request(climate)?;
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_climate(&mut surface, &request, day_index)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from parsed snow-control input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when snow controls are non-finite or
/// outside required CLIM05 domains.
pub fn build_hillslope_runtime_surface_from_snow(
    snow: &SnowParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_snow(&mut surface, snow)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from parsed frost-control input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when frost controls are non-finite or
/// outside required CLIM06 domains.
pub fn build_hillslope_runtime_surface_from_frost(
    frost: &FrostParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_frost(&mut surface, frost)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from parsed depletion-scheduled irrigation
/// sidecar input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required depletion scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
pub fn build_hillslope_runtime_surface_from_irrigation_depletion(
    depletion: &IrrigationDepletionFile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_irrigation_depletion(&mut surface, depletion)?;
    Ok(surface)
}

/// Seed parsed depletion-scheduled irrigation symbols into an existing
/// hillslope runtime surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required depletion scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_irrigation_depletion(
    runtime_surface: &mut HillslopeWritebackSurface,
    depletion: &IrrigationDepletionFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let system_type = match depletion.system_type {
        openwepp_input_contract::parsers::irrigation_depletion::IrrigationSystemType::Sprinkler => {
            1.0
        }
        openwepp_input_contract::parsers::irrigation_depletion::IrrigationSystemType::Furrow => 2.0,
    };

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.element_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.depletion.element_count",
            depletion.element_count,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.system_type"),
        BoundaryValue::scalar(system_type),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.schedule_type"),
        BoundaryValue::scalar(f64::from(depletion.schedule_type)),
    );

    let min_depth =
        validate_irrigation_finite("irrigation.depletion.min_depth_m", depletion.min_depth_m)?;
    if min_depth < 0.0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.depletion.min_depth_m",
                value: min_depth,
                allowed: ">= 0.0",
            },
        );
    }
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.min_depth_m"),
        BoundaryValue::scalar(min_depth),
    );
    if let Some(max_depth_m) = depletion.max_depth_m {
        let max_depth =
            validate_irrigation_finite("irrigation.depletion.max_depth_m", max_depth_m)?;
        if max_depth < min_depth {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.max_depth_m",
                    value: max_depth,
                    allowed: ">= irrigation.depletion.min_depth_m",
                },
            );
        }
        state_surface.insert(
            BoundarySymbol::from("irrigation.depletion.max_depth_m"),
            BoundaryValue::scalar(max_depth),
        );
    }

    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.depletion.period_count",
            depletion.periods.len(),
        )?),
    );

    for (period_position, period) in depletion.periods.iter().enumerate() {
        let period_index = period_position + 1;
        if period.element_id == 0 {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.period_####.element_id",
                    value: 0.0,
                    allowed: ">= 1",
                },
            );
        }
        state_surface.insert(
            irrigation_depletion_period_symbol(period_index, "element_id"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.depletion.period_####.element_id",
                period.element_id,
            )?),
        );

        let depletion_trigger = validate_irrigation_finite(
            "irrigation.depletion.period_####.depletion_trigger_ratio",
            period.depletion_trigger_ratio,
        )?;
        if !(0.0..=1.0).contains(&depletion_trigger) {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.period_####.depletion_trigger_ratio",
                    value: depletion_trigger,
                    allowed: "[0.0,1.0]",
                },
            );
        }
        state_surface.insert(
            irrigation_depletion_period_symbol(period_index, "depletion_trigger_ratio"),
            BoundaryValue::scalar(depletion_trigger),
        );

        for (field, value) in [
            ("start_doy", period.start_doy),
            ("start_year", period.start_year),
            ("end_doy", period.end_doy),
            ("end_year", period.end_year),
        ] {
            if value < 0 {
                return Err(
                    HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                        field: "irrigation.depletion.period_####.date",
                        value: f64::from(value),
                        allowed: ">= 0",
                    },
                );
            }
            state_surface.insert(
                irrigation_depletion_period_symbol(period_index, field),
                BoundaryValue::scalar(f64::from(value)),
            );
        }

        match &period.data {
            IrrigationPeriodData::Sprinkler(record) => {
                let rate = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_rate_m_per_s",
                    record.rate_m_per_s,
                )?;
                if rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_rate_m_per_s",
                            value: rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let depth_ratio = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_depth_ratio",
                    record.depth_ratio,
                )?;
                if depth_ratio < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_depth_ratio",
                            value: depth_ratio,
                            allowed: ">= 0.0",
                        },
                    );
                }
                let nozzle = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_nozzle_factor",
                    record.nozzle_factor,
                )?;
                if nozzle <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_nozzle_factor",
                            value: nozzle,
                            allowed: "> 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_rate_m_per_s"),
                    BoundaryValue::scalar(rate),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_depth_ratio"),
                    BoundaryValue::scalar(depth_ratio),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_nozzle_factor"),
                    BoundaryValue::scalar(nozzle),
                );
            }
            IrrigationPeriodData::Furrow(record) => {
                if record.end_element_id == 0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_end_element_id",
                            value: 0.0,
                            allowed: ">= 1",
                        },
                    );
                }
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_end_element_id"),
                    BoundaryValue::scalar(irrigation_count_to_f64(
                        "irrigation.depletion.period_####.furrow_end_element_id",
                        record.end_element_id,
                    )?),
                );

                let supply_rate = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_supply_rate_m3_per_s",
                    record.supply_rate_m3_per_s,
                )?;
                if supply_rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_supply_rate_m3_per_s",
                            value: supply_rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let supply_duration = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_supply_duration_s",
                    record.supply_duration_s,
                )?;
                if supply_duration <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_supply_duration_s",
                            value: supply_duration,
                            allowed: "> 0.0",
                        },
                    );
                }
                let fill_ratio = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_fill_ratio",
                    record.fill_ratio,
                )?;
                if fill_ratio < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_fill_ratio",
                            value: fill_ratio,
                            allowed: ">= 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_supply_rate_m3_per_s"),
                    BoundaryValue::scalar(supply_rate),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_supply_duration_s"),
                    BoundaryValue::scalar(supply_duration),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_surge_code"),
                    BoundaryValue::scalar(f64::from(record.surge_code)),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_fill_ratio"),
                    BoundaryValue::scalar(fill_ratio),
                );
            }
        }
    }

    Ok(())
}

/// Build a hillslope runtime surface from parsed fixed-date irrigation sidecar
/// input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required fixed-date scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
pub fn build_hillslope_runtime_surface_from_irrigation_fixeddate(
    fixeddate: &FixedDateIrrigationFile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_irrigation_fixeddate(&mut surface, fixeddate)?;
    Ok(surface)
}

/// Seed parsed fixed-date irrigation symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required fixed-date scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_irrigation_fixeddate(
    runtime_surface: &mut HillslopeWritebackSurface,
    fixeddate: &FixedDateIrrigationFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let datver = validate_irrigation_finite("irrigation.fixeddate.datver", fixeddate.datver)?;
    if datver <= 0.0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.datver",
                value: datver,
                allowed: "> 0.0",
            },
        );
    }

    if fixeddate.itemp == 0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.ofe_count",
                value: 0.0,
                allowed: ">= 1",
            },
        );
    }
    if fixeddate.initial_records.len() != fixeddate.itemp {
        let observed = irrigation_count_to_f64(
            "irrigation.fixeddate.initial_records",
            fixeddate.initial_records.len(),
        )?;
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.initial_records",
                value: observed,
                allowed: "== irrigation.fixeddate.ofe_count",
            },
        );
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.datver"),
        BoundaryValue::scalar(datver),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.ofe_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.ofe_count",
            fixeddate.itemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.system_type"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.system_type",
            fixeddate.jtemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.schedule_type"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.schedule_type",
            fixeddate.ktemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.event_count",
            fixeddate.events.len(),
        )?),
    );

    let mut expected_ofe = 1usize;
    let mut active_dates = fixeddate.initial_records.clone();
    for (event_position, event) in fixeddate.events.iter().enumerate() {
        let event_index = event_position + 1;
        let expected_ofe_f64 =
            irrigation_count_to_f64("irrigation.fixeddate.event_####.ofe_id", expected_ofe)?;
        let schedule = active_dates.get(expected_ofe - 1).ok_or(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.event_####.ofe_id",
                value: expected_ofe_f64,
                allowed: "1..=irrigation.fixeddate.ofe_count",
            },
        )?;

        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "ofe_id"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.ofe_id",
                schedule.ofeflg,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "day"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.day",
                schedule.irday,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "year"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.year",
                schedule.iryr,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "schedule_termination_flag"),
            BoundaryValue::scalar(if schedule.schedule_termination_flag {
                1.0
            } else {
                0.0
            }),
        );

        match event {
            FixedDateEvent::Sprinkler(sprinkler) => {
                let rate = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_rate_m_per_s",
                    sprinkler.irint,
                )?;
                if rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_rate_m_per_s",
                            value: rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let depth = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_depth_m",
                    sprinkler.irdept,
                )?;
                if depth < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_depth_m",
                            value: depth,
                            allowed: ">= 0.0",
                        },
                    );
                }
                let nozzle = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_nozzle_factor",
                    sprinkler.nozzle,
                )?;
                if nozzle <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_nozzle_factor",
                            value: nozzle,
                            allowed: "> 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_rate_m_per_s"),
                    BoundaryValue::scalar(rate),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_depth_m"),
                    BoundaryValue::scalar(depth),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_nozzle_factor"),
                    BoundaryValue::scalar(nozzle),
                );
            }
            FixedDateEvent::Furrow(furrow) => {
                let surges = irrigation_count_to_f64(
                    "irrigation.fixeddate.event_####.furrow_surges",
                    furrow.surges,
                )?;
                let mut total_duration = 0.0_f64;
                let mut total_volume = 0.0_f64;
                for surge in &furrow.rows {
                    let supply_rate = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_supply_rate_m3_per_s",
                        surge.qspply,
                    )?;
                    if supply_rate <= 0.0 {
                        return Err(
                            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                field: "irrigation.fixeddate.event_####.furrow_supply_rate_m3_per_s",
                                value: supply_rate,
                                allowed: "> 0.0",
                            },
                        );
                    }
                    let start_s = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_start_s",
                        surge.tstart,
                    )?;
                    let end_s = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_end_s",
                        surge.tend,
                    )?;
                    if end_s < start_s {
                        return Err(
                            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                field: "irrigation.fixeddate.event_####.furrow_end_s",
                                value: end_s,
                                allowed: ">= irrigation.fixeddate.event_####.furrow_start_s",
                            },
                        );
                    }
                    if let Some(tdepl) = surge.tdepl {
                        let depletion_tail = validate_irrigation_finite(
                            "irrigation.fixeddate.event_####.furrow_tdepl_s",
                            tdepl,
                        )?;
                        if depletion_tail < 0.0 {
                            return Err(
                                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                    field: "irrigation.fixeddate.event_####.furrow_tdepl_s",
                                    value: depletion_tail,
                                    allowed: ">= 0.0",
                                },
                            );
                        }
                        total_duration += depletion_tail;
                    }
                    let active_duration = end_s - start_s;
                    total_duration += active_duration;
                    total_volume += supply_rate * active_duration;
                }

                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_surges"),
                    BoundaryValue::scalar(surges),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_total_duration_s"),
                    BoundaryValue::scalar(total_duration),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_total_supply_volume_m3"),
                    BoundaryValue::scalar(total_volume),
                );
            }
        }

        match event {
            FixedDateEvent::Sprinkler(event) => {
                active_dates[expected_ofe - 1] = event.next_record.clone();
            }
            FixedDateEvent::Furrow(event) => {
                active_dates[expected_ofe - 1] = event.next_record.clone();
            }
        }
        expected_ofe += 1;
        if expected_ofe > fixeddate.itemp {
            expected_ofe = 1;
        }
    }

    Ok(())
}

/// Seed parsed snow-control runtime symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when parsed snow controls are
/// non-finite or violate required CLIM05 domains.
pub fn seed_hillslope_runtime_surface_from_snow(
    runtime_surface: &mut HillslopeWritebackSurface,
    snow: &SnowParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    let rst = validate_snow_control_finite("snow.options.rst", snow.rst)?;
    let newsnw = validate_snow_control_finite("snow.options.newsnw", snow.newsnw)?;
    let ssd = validate_snow_control_finite("snow.options.ssd", snow.ssd)?;

    if newsnw <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "> 0.0",
        });
    }
    if ssd <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.ssd",
            value: ssd,
            allowed: "> 0.0",
        });
    }
    if newsnw > ssd {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "<= snow.options.ssd",
        });
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("snow.options.rst"),
        BoundaryValue::scalar(rst),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(newsnw),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.ssd"),
        BoundaryValue::scalar(ssd),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(if snow.sidecar_present { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.0),
    );

    Ok(())
}

/// Seed parsed frost-control runtime symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when parsed frost controls are
/// non-finite or violate required CLIM06 domains.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_frost(
    runtime_surface: &mut HillslopeWritebackSurface,
    frost: &FrostParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    let wint_red = f64::from(frost.wint_red);
    let fine_top = f64::from(frost.fine_top);
    let fine_bot = f64::from(frost.fine_bot);
    let ksnowf = validate_frost_control_finite("frost.options.ksnowf", frost.ksnowf)?;
    let kresf = validate_frost_control_finite("frost.options.kresf", frost.kresf)?;
    let ksoilf = validate_frost_control_finite("frost.options.ksoilf", frost.ksoilf)?;
    let kfactor1 = validate_frost_control_finite("frost.options.kfactor1", frost.kfactor1)?;
    let kfactor2 = validate_frost_control_finite("frost.options.kfactor2", frost.kfactor2)?;
    let kfactor3 = validate_frost_control_finite("frost.options.kfactor3", frost.kfactor3)?;

    if frost.wint_red != 0 && frost.wint_red != 1 {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.wintRed",
            value: wint_red,
            allowed: "{0,1}",
        });
    }
    if !(1..=10).contains(&frost.fine_top) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineTop",
            value: fine_top,
            allowed: "integer [1,10]",
        });
    }
    if !(1..=10).contains(&frost.fine_bot) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineBot",
            value: fine_bot,
            allowed: "integer [1,10]",
        });
    }
    for (field, value) in [
        ("frost.options.ksnowf", ksnowf),
        ("frost.options.kresf", kresf),
        ("frost.options.ksoilf", ksoilf),
    ] {
        if !(0.1..=10.0).contains(&value) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real [0.1,10.0]",
            });
        }
    }
    for (field, value) in [
        ("frost.options.kfactor1", kfactor1),
        ("frost.options.kfactor2", kfactor2),
        ("frost.options.kfactor3", kfactor3),
    ] {
        if !(value > 0.0 && value <= 1.0) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real (0.0,1.0]",
            });
        }
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(wint_red),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineTop"),
        BoundaryValue::scalar(fine_top),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineBot"),
        BoundaryValue::scalar(fine_bot),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksnowf"),
        BoundaryValue::scalar(ksnowf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kresf"),
        BoundaryValue::scalar(kresf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksoilf"),
        BoundaryValue::scalar(ksoilf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(kfactor1),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(kfactor2),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(kfactor3),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(if frost.frost_file_present { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dfrost"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dthaw"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_nft"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_ws_frz"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_infcap_frz"),
        BoundaryValue::scalar(0.0),
    );

    Ok(())
}

fn insert_common_day_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    day: i32,
    mon: i32,
    year: i32,
) {
    surface.insert(
        BoundarySymbol::from("day"),
        BoundaryValue::scalar(f64::from(day)),
    );
    surface.insert(
        BoundarySymbol::from("mon"),
        BoundaryValue::scalar(f64::from(mon)),
    );
    surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(f64::from(year)),
    );
}

fn validate_snow_control_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteSnowControl { field, value });
    }
    Ok(value)
}

fn validate_frost_control_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteFrostControl { field, value });
    }
    Ok(value)
}

fn validate_slope_profile_shape(slope: &SlopeProfile) -> Result<(), HillslopeRuntimeInputError> {
    if slope.ofes.is_empty() {
        return Err(HillslopeRuntimeInputError::MissingSlopeOfe);
    }

    if slope.ofe_count != slope.ofes.len() {
        return Err(HillslopeRuntimeInputError::SlopeOfeCountMismatch {
            declared_ofe_count: slope.ofe_count,
            observed_ofes: slope.ofes.len(),
        });
    }

    Ok(())
}

fn validate_slope_ofe_shape(
    ofe_index: usize,
    declared_nslpts: usize,
    observed_points: usize,
) -> Result<(), HillslopeRuntimeInputError> {
    if declared_nslpts != observed_points {
        return Err(HillslopeRuntimeInputError::SlopePointCountMismatch {
            ofe_index,
            declared_nslpts,
            observed_points,
        });
    }

    if observed_points < 2 {
        return Err(HillslopeRuntimeInputError::InsufficientSlopePoints {
            ofe_index,
            observed_points,
        });
    }

    Ok(())
}

fn validate_slope_points(
    ofe_index: usize,
    points: &[SlopePoint],
) -> Result<(), HillslopeRuntimeInputError> {
    for (point_position, point) in points.iter().enumerate() {
        let point_index = point_position + 1;
        if !point.xinput.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteXinput {
                ofe_index,
                point_index,
                value: point.xinput,
            });
        }

        if !point.slpinp.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteSlpinp {
                ofe_index,
                point_index,
                value: point.slpinp,
            });
        }
    }

    for (segment_position, window) in points.windows(2).enumerate() {
        let left_point_index = segment_position + 1;
        let right_point_index = segment_position + 2;
        let left_value = window[0].xinput;
        let right_value = window[1].xinput;
        if right_value < left_value {
            return Err(HillslopeRuntimeInputError::NonMonotoneXinput {
                ofe_index,
                left_point_index,
                left_value,
                right_point_index,
                right_value,
            });
        }
    }

    Ok(())
}

fn derive_avgslp(
    ofe_index: usize,
    points: &[SlopePoint],
) -> Result<f64, HillslopeRuntimeInputError> {
    let slen = points.last().map(|point| point.xinput).ok_or(
        HillslopeRuntimeInputError::InsufficientSlopePoints {
            ofe_index,
            observed_points: 0,
        },
    )?;
    if !slen.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteDerivedSlopeLength {
            ofe_index,
            value_m: slen,
        });
    }
    if slen <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveDerivedSlopeLength {
            ofe_index,
            value_m: slen,
        });
    }

    let mut top_elevation = 0.0;
    for pair in points.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        top_elevation += (right.xinput - left.xinput) * (left.slpinp + right.slpinp) / 2.0;
    }

    let avgslp = top_elevation / slen;
    if !avgslp.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteDerivedAverageSlope {
            ofe_index,
            value: avgslp,
        });
    }
    if avgslp <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
            ofe_index,
            value: avgslp,
        });
    }

    Ok(avgslp)
}

fn usize_to_f64(field: &'static str, value: usize) -> Result<f64, HillslopeRuntimeInputError> {
    let value_u32 = u32::try_from(value)
        .map_err(|_| HillslopeRuntimeInputError::PlProjectionCountOutOfRange { field, value })?;
    Ok(f64::from(value_u32))
}

fn irrigation_count_to_f64(
    field: &'static str,
    value: usize,
) -> Result<f64, HillslopeRuntimeInputError> {
    let value_u32 = u32::try_from(value).map_err(|_| {
        HillslopeRuntimeInputError::IrrigationScheduleCountOutOfRange { field, value }
    })?;
    Ok(f64::from(value_u32))
}

fn project_growth_equation_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in growth_equation_parameter_values(slot_index, crop_slot_index, plant)? {
        surface.insert(
            pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    Ok(())
}

fn project_primary_growth_equation_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in growth_equation_parameter_values(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_decomposition_equation_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in
        decomposition_equation_parameter_values(slot_index, crop_slot_index, plant)?
    {
        surface.insert(
            pl_decomp_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    Ok(())
}

fn project_primary_decomposition_equation_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in decomposition_equation_parameter_values(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn growth_equation_parameter_values(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<[(&'static str, f64); 15], HillslopeRuntimeInputError> {
    let bb =
        validate_projection_non_negative("bb", slot_index, crop_slot_index, plant.canopy_line[0])?;
    let beinp = validate_projection_non_negative(
        "beinp",
        slot_index,
        crop_slot_index,
        plant.canopy_line[2],
    )?;
    let btemp =
        validate_projection_finite("btemp", slot_index, crop_slot_index, plant.canopy_line[3])?;
    let decfct =
        validate_projection_fraction("decfct", slot_index, crop_slot_index, plant.canopy_line[8])?;

    let dlai =
        validate_projection_fraction("dlai", slot_index, crop_slot_index, plant.growth_line[0])?;
    if dlai <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "dlai",
            slot_index,
            crop_slot_index,
            value: dlai,
            allowed: ">0.0 and <=1.0",
        });
    }
    let dropfc =
        validate_projection_fraction("dropfc", slot_index, crop_slot_index, plant.growth_line[1])?;
    let extnct = validate_projection_non_negative(
        "extnct",
        slot_index,
        crop_slot_index,
        plant.growth_line[2],
    )?;
    let gddmax =
        validate_projection_positive("gddmax", slot_index, crop_slot_index, plant.growth_line[5])?;
    let hi = validate_projection_fraction("hi", slot_index, crop_slot_index, plant.growth_line[6])?;

    let otemp =
        validate_projection_finite("otemp", slot_index, crop_slot_index, plant.residue_line[2])?;
    if otemp <= btemp {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "otemp",
            slot_index,
            crop_slot_index,
            value: otemp,
            allowed: "> btemp",
        });
    }
    let rdmax =
        validate_projection_positive("rdmax", slot_index, crop_slot_index, plant.residue_line[5])?;
    let rsr = validate_projection_non_negative(
        "rsr",
        slot_index,
        crop_slot_index,
        plant.residue_line[6],
    )?;
    let rtmmax = validate_projection_non_negative(
        "rtmmax",
        slot_index,
        crop_slot_index,
        plant.residue_line[7],
    )?;
    let spriod = validate_projection_non_negative(
        "spriod",
        slot_index,
        crop_slot_index,
        plant.residue_line[8],
    )?;
    let xmxlai = validate_projection_non_negative(
        "xmxlai",
        slot_index,
        crop_slot_index,
        plant.terminal_line[1],
    )?;

    Ok([
        ("btemp", btemp),
        ("otemp", otemp),
        ("gddmax", gddmax),
        ("dlai", dlai),
        ("dropfc", dropfc),
        ("decfct", decfct),
        ("spriod", spriod),
        ("bb", bb),
        ("beinp", beinp),
        ("extnct", extnct),
        ("hi", hi),
        ("xmxlai", xmxlai),
        ("rsr", rsr),
        ("rtmmax", rtmmax),
        ("rdmax", rdmax),
    ])
}

fn decomposition_equation_parameter_values(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<[(&'static str, f64); 2], HillslopeRuntimeInputError> {
    let annual_decay_rate =
        validate_projection_positive("oratea", slot_index, crop_slot_index, plant.residue_line[0])?;
    let root_decay_rate =
        validate_projection_positive("orater", slot_index, crop_slot_index, plant.residue_line[1])?;
    Ok([("oratea", annual_decay_rate), ("orater", root_decay_rate)])
}

fn validate_projection_finite(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    Ok(value)
}

fn validate_projection_non_negative(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    let value = validate_projection_finite(field, slot_index, crop_slot_index, value)?;
    if value < 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: ">=0.0",
        });
    }
    Ok(value)
}

#[derive(Debug, Clone, PartialEq)]
struct AnnualExtensionProjection {
    jdherb: usize,
    jdburn: usize,
    jdslge: usize,
    jdcut: usize,
    jdmove: usize,
    fbrnag: f64,
    fbrnog: f64,
    frcut: f64,
    frmove: f64,
}

impl AnnualExtensionProjection {
    const fn zeroed() -> Self {
        Self {
            jdherb: 0,
            jdburn: 0,
            jdslge: 0,
            jdcut: 0,
            jdmove: 0,
            fbrnag: 0.0,
            fbrnog: 0.0,
            frcut: 0.0,
            frmove: 0.0,
        }
    }
}

fn validate_projection_day(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: usize,
    allow_zero: bool,
) -> Result<usize, HillslopeRuntimeInputError> {
    if (allow_zero && value == 0) || (1..=366).contains(&value) {
        return Ok(value);
    }

    Err(HillslopeRuntimeInputError::PlProjectionDayOutOfDomain {
        field,
        slot_index,
        crop_slot_index,
        value,
        allowed: if allow_zero { "0 or 1..366" } else { "1..366" },
    })
}

fn validate_projection_fraction(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: "0.0..=1.0",
        });
    }
    Ok(value)
}

fn validate_projection_positive(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    if value <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: ">0.0",
        });
    }
    Ok(value)
}

fn annual_extension_variant_name(extension: Option<&YearlyAnnualExtension>) -> &'static str {
    match extension {
        Some(YearlyAnnualExtension::Herbicide { .. }) => "herbicide",
        Some(YearlyAnnualExtension::Burn { .. }) => "burn",
        Some(YearlyAnnualExtension::Silage { .. }) => "silage",
        Some(YearlyAnnualExtension::Cut { .. }) => "cut",
        Some(YearlyAnnualExtension::Remove { .. }) => "remove",
        None => "none",
    }
}

fn annual_extension_mismatch(
    slot_index: usize,
    crop_slot_index: usize,
    resmgt: usize,
    expected: &'static str,
    observed: Option<&YearlyAnnualExtension>,
) -> HillslopeRuntimeInputError {
    HillslopeRuntimeInputError::PlAnnualExtensionMismatch {
        slot_index,
        crop_slot_index,
        resmgt,
        expected,
        observed: annual_extension_variant_name(observed),
    }
}

#[allow(clippy::too_many_lines)]
fn project_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    resmgt: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let mut projection = AnnualExtensionProjection::zeroed();
    match resmgt {
        1 => match extension {
            Some(YearlyAnnualExtension::Herbicide { jdherb }) => {
                projection.jdherb =
                    validate_projection_day("jdherb", slot_index, crop_slot_index, *jdherb, false)?;
            }
            _ => {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "herbicide",
                    extension,
                ));
            }
        },
        2 => match extension {
            Some(YearlyAnnualExtension::Burn {
                jdburn,
                fbmag,
                fbrnog,
            }) => {
                projection.jdburn =
                    validate_projection_day("jdburn", slot_index, crop_slot_index, *jdburn, false)?;
                projection.fbrnag =
                    validate_projection_fraction("fbrnag", slot_index, crop_slot_index, *fbmag)?;
                projection.fbrnog =
                    validate_projection_fraction("fbrnog", slot_index, crop_slot_index, *fbrnog)?;
            }
            _ => {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "burn",
                    extension,
                ));
            }
        },
        3 => match extension {
            Some(YearlyAnnualExtension::Silage { jdslge }) => {
                projection.jdslge =
                    validate_projection_day("jdslge", slot_index, crop_slot_index, *jdslge, false)?;
            }
            _ => {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "silage",
                    extension,
                ));
            }
        },
        4 => match extension {
            Some(YearlyAnnualExtension::Cut { jdcut, frcut }) => {
                projection.jdcut =
                    validate_projection_day("jdcut", slot_index, crop_slot_index, *jdcut, false)?;
                projection.frcut =
                    validate_projection_fraction("frcut", slot_index, crop_slot_index, *frcut)?;
            }
            _ => {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "cut",
                    extension,
                ));
            }
        },
        5 => match extension {
            Some(YearlyAnnualExtension::Remove { jdmove, frmove }) => {
                projection.jdmove =
                    validate_projection_day("jdmove", slot_index, crop_slot_index, *jdmove, false)?;
                projection.frmove =
                    validate_projection_fraction("frmove", slot_index, crop_slot_index, *frmove)?;
            }
            _ => {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "remove",
                    extension,
                ));
            }
        },
        6 => {
            if extension.is_some() {
                return Err(annual_extension_mismatch(
                    slot_index,
                    crop_slot_index,
                    resmgt,
                    "none",
                    extension,
                ));
            }
        }
        7 => {
            return Err(
                HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                    field: "resmgt",
                    slot_index,
                    crop_slot_index,
                    reason: "resmgt=7 annual-cut payload is not represented by runtime projection",
                },
            );
        }
        _ => {
            return Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                field: "resmgt",
                value: resmgt,
                allowed: "1..7",
            });
        }
    }
    Ok(projection)
}

fn project_annual_extension_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    projection: &AnnualExtensionProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        pl_decomp_slot_crop_symbol("jdherb", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdherb", projection.jdherb)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdburn", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdburn", projection.jdburn)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdslge", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdslge", projection.jdslge)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdcut", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdcut", projection.jdcut)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdmove", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdmove", projection.jdmove)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("fbrnag", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.fbrnag),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("fbrnog", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.fbrnog),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("frcut", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.frcut),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("frmove", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.frmove),
    );
    Ok(())
}

fn project_primary_annual_extension_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    projection: &AnnualExtensionProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        BoundarySymbol::from("jdherb"),
        BoundaryValue::scalar(usize_to_f64("jdherb", projection.jdherb)?),
    );
    surface.insert(
        BoundarySymbol::from("jdburn"),
        BoundaryValue::scalar(usize_to_f64("jdburn", projection.jdburn)?),
    );
    surface.insert(
        BoundarySymbol::from("jdslge"),
        BoundaryValue::scalar(usize_to_f64("jdslge", projection.jdslge)?),
    );
    surface.insert(
        BoundarySymbol::from("jdcut"),
        BoundaryValue::scalar(usize_to_f64("jdcut", projection.jdcut)?),
    );
    surface.insert(
        BoundarySymbol::from("jdmove"),
        BoundaryValue::scalar(usize_to_f64("jdmove", projection.jdmove)?),
    );
    surface.insert(
        BoundarySymbol::from("fbrnag"),
        BoundaryValue::scalar(projection.fbrnag),
    );
    surface.insert(
        BoundarySymbol::from("fbrnog"),
        BoundaryValue::scalar(projection.fbrnog),
    );
    surface.insert(
        BoundarySymbol::from("frcut"),
        BoundaryValue::scalar(projection.frcut),
    );
    surface.insert(
        BoundarySymbol::from("frmove"),
        BoundaryValue::scalar(projection.frmove),
    );
    Ok(())
}

fn project_perennial_cutday_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &openwepp_input_contract::parsers::management::YearlyPerennialData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (position, cutday) in perennial.cut_days.iter().enumerate() {
        let cut_index = position + 1;
        let day = validate_projection_day("cutday", slot_index, crop_slot_index, *cutday, false)?;
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("cutday", slot_index, crop_slot_index, cut_index),
            BoundaryValue::scalar(usize_to_f64("cutday", day)?),
        );
    }
    Ok(())
}

fn project_perennial_grazing_cycle_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &openwepp_input_contract::parsers::management::YearlyPerennialData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (position, cycle) in perennial.grazing_cycles.iter().enumerate() {
        let cycle_index = position + 1;
        let gday = validate_projection_day("gday", slot_index, crop_slot_index, cycle.gday, false)?;
        let gend = validate_projection_day("gend", slot_index, crop_slot_index, cycle.gend, false)?;
        if gday >= gend {
            return Err(HillslopeRuntimeInputError::PlGrazingWindowOutOfDomain {
                slot_index,
                crop_slot_index,
                cycle_index,
                gday,
                gend,
            });
        }

        let animal =
            validate_projection_positive("animal", slot_index, crop_slot_index, cycle.animal)?;
        let bodywt =
            validate_projection_positive("bodywt", slot_index, crop_slot_index, cycle.bodywt)?;
        let area = validate_projection_positive("area", slot_index, crop_slot_index, cycle.area)?;
        let digest =
            validate_projection_fraction("digest", slot_index, crop_slot_index, cycle.digest)?;

        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("gday", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(usize_to_f64("gday", gday)?),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("gend", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(usize_to_f64("gend", gend)?),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("animal", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(animal),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("bodywt", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(bodywt),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("area", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(area),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("digest", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(digest),
        );
    }
    Ok(())
}

fn pl_schedule_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_schedule_ofe{ofe_index}_{root}"))
}

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_schedule_slot_{slot_index:04}_{root}"))
}

fn pl_schedule_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_growth_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_{root}"))
}

fn pl_growth_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_decomp_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_decomp_ofe{ofe_index}_{root}"))
}

fn pl_decomp_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_decomp_slot_crop_indexed_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
    index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_{index:04}"
    ))
}

fn slope_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("ofe{ofe_index}_{root}"))
}

fn slope_ofe_point_symbol(root: &str, ofe_index: usize, point_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("ofe{ofe_index}_{root}_{point_index:04}"))
}

fn slope_primary_point_symbol(root: &str, point_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("{root}_{point_index:04}"))
}

fn soil_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("ofe{ofe_index}_{root}"))
}

fn soil_ofe_layer_symbol(root: &str, ofe_index: usize, layer_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("ofe{ofe_index}_{root}_{layer_index:04}"))
}

fn soil_primary_layer_symbol(root: &str, layer_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("{root}_{layer_index:04}"))
}

fn irrigation_depletion_period_symbol(period_index: usize, field: &str) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "irrigation.depletion.period_{period_index:04}.{field}"
    ))
}

fn irrigation_fixeddate_event_symbol(event_index: usize, field: &str) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "irrigation.fixeddate.event_{event_index:04}.{field}"
    ))
}

fn validate_irrigation_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteIrrigationScheduleField { field, value });
    }
    Ok(value)
}

fn build_hillslope_series_surface(
    forcing: &HillslopeClimateDailyForcing,
) -> Result<ClimateForcingSymbolSurface, ClimateRuntimeInputError> {
    let point_count = forcing_series_point_count(forcing);
    ClimateForcingSymbolSurface::hillslope(point_count)
        .map_err(|error| map_surface_build_error(&error))
}

fn forcing_series_point_count(forcing: &HillslopeClimateDailyForcing) -> usize {
    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => day.timem.len(),
        HillslopeClimateDailyForcing::Breakpoint(day) => day.timem.len(),
    }
}

fn map_surface_build_error(error: &ClimateForcingSymbolSurfaceError) -> ClimateRuntimeInputError {
    match error {
        ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
            count,
            supported_max,
        } => ClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
            value: *count,
            max: *supported_max,
        },
    }
}

fn insert_series_values(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    symbols: &[BoundarySymbol],
    values: &[f64],
) {
    debug_assert_eq!(symbols.len(), values.len());
    for (symbol, value) in symbols.iter().zip(values.iter()) {
        surface.insert(symbol.clone(), BoundaryValue::scalar(*value));
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::{
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
        management::{
            ParseMode as ManagementParseMode, YearlyCroplandBranch, YearlyPerennialData,
            YearlyScenarioData, parse_management_from_str,
        },
        slope::{SlopeParserOptions, parse_slope_str},
        soil::{ParserMode, SoilParserOptions, parse_soil},
    };
    use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

    use super::{
        ClimateRuntimeInputError, HillslopeRuntimeInputError,
        build_hillslope_pl_runtime_surfaces_from_management,
        build_hillslope_runtime_surface_from_climate,
        build_hillslope_runtime_surface_from_management,
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    };

    const VALID_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/strict_valid.cli");
    const LEGACY_DATVER_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
    const SINGLE_STORM_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/single_storm_itemp2.cli");
    const BREAKPOINT_OVERFLOW_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/breakpoint_overflow_51.cli");
    const WC1_BREAKPOINT_STMSTR_NONZERO: &str = include_str!(
        "../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli"
    );
    const WC1_BREAKPOINT_NBRKPT_42: &str = include_str!(
        "../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli"
    );
    const WC1_CANOGA_DAY1: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
    const WC1_CANOGA_STMDUR_CAP: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
    const SLOPE_STRICT_VALID_CANONICAL: &str =
        include_str!("../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const VALID_9002: &str = include_str!("../../../tests/fixtures/infile/soil/valid_9002.sol");
    const VALID_97_5: &str = include_str!("../../../tests/fixtures/infile/soil/valid_97_5.sol");
    const MANAGEMENT_CANONICAL_NONZERO_98_4: &str = include_str!(
        "../../../tests/fixtures/infile/management/canonical_cropland_nonzero_98_4.man"
    );

    fn build_breakpoint_fixture(nbrkpt: usize) -> String {
        let mut climate = format!(
            "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n45.0 -120.0 1000.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n1 1 2000 {nbrkpt} 11.0 1.0 180.0 2.0 170.0 -2.0\n"
        );
        if nbrkpt == 0 {
            return climate;
        }
        let denom_u32 = u32::try_from((nbrkpt - 1).max(1))
            .expect("breakpoint fixture helper expects small cardinalities");
        let denom = f64::from(denom_u32);
        for index in 0..nbrkpt {
            let idx_u32 = u32::try_from(index)
                .expect("breakpoint fixture helper expects small cardinalities");
            let idx = f64::from(idx_u32);
            let timem = (24.0 * idx) / denom;
            let pptcum = (120.0 * idx) / denom;
            writeln!(&mut climate, "{timem:.4} {pptcum:.3}")
                .expect("writing synthetic breakpoint fixture should succeed");
        }
        climate
    }

    #[test]
    fn soil_runtime_surface_contains_canonical_state_symbols() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solthk = surface
            .state_surface
            .get(&BoundarySymbol::from("solthk"))
            .expect("solthk should be present")
            .as_f64();
        let dg = surface
            .state_surface
            .get(&BoundarySymbol::from("dg"))
            .expect("dg should be present")
            .as_f64();
        let thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        let nsl = surface
            .state_surface
            .get(&BoundarySymbol::from("nsl"))
            .expect("nsl should be present")
            .as_f64();
        let ssc = surface
            .state_surface
            .get(&BoundarySymbol::from("ssc"))
            .expect("ssc should be present")
            .as_f64();
        let dg_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("dg_0002"))
            .expect("dg_0002 should be present")
            .as_f64();
        let solthk_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("solthk_0002"))
            .expect("solthk_0002 should be present")
            .as_f64();
        let ssc_layer2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ssc_0002"))
            .expect("ssc_0002 should be present")
            .as_f64();

        assert!((solthk - 0.25).abs() < 1e-12);
        assert!((dg - 0.1).abs() < 1e-12);
        assert!((thetdr - 0.05).abs() < 1e-12);
        assert!((thetfc - 0.31).abs() < 1e-12);
        assert!((nsl - 2.0).abs() < 1e-12);
        assert!((ssc - (15.0 / 3.6e6)).abs() < 1e-12);
        assert!((dg_layer2 - 0.15).abs() < 1e-12);
        assert!((solthk_layer2 - 0.25).abs() < 1e-12);
        assert!((ssc_layer2 - (8.0 / 3.6e6)).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_theta_fields() {
        let soil = parse_soil(VALID_97_5, SoilParserOptions::default())
            .expect("97.5 soil fixture should parse");

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing theta fields must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-003");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingThetaResidual
        ));
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_saturated_conductivity() {
        let mut soil = parse_soil(VALID_9002, SoilParserOptions::default())
            .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[0].ksat_mm_h = None;

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing ksat must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-033");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingSaturatedConductivity {
                ofe_index: 1,
                layer_index: 1
            }
        ));
    }

    #[test]
    fn slope_runtime_surface_contains_canonical_state_symbols() {
        let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");
        let surface = build_hillslope_runtime_surface_from_slope(&slope)
            .expect("slope runtime surface should build");

        let nelem = surface
            .state_surface
            .get(&BoundarySymbol::from("nelem"))
            .expect("nelem should be present")
            .as_f64();
        let slplen = surface
            .state_surface
            .get(&BoundarySymbol::from("slplen"))
            .expect("slplen should be present")
            .as_f64();
        let nslpts = surface
            .state_surface
            .get(&BoundarySymbol::from("nslpts"))
            .expect("nslpts should be present")
            .as_f64();
        let avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("avgslp"))
            .expect("avgslp should be present")
            .as_f64();
        let xinput_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("xinput_0002"))
            .expect("xinput_0002 should be present")
            .as_f64();
        let slpinp_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("slpinp_0002"))
            .expect("slpinp_0002 should be present")
            .as_f64();
        let ofe2_avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("ofe2_avgslp"))
            .expect("ofe2_avgslp should be present")
            .as_f64();

        assert!((nelem - 2.0).abs() < 1e-12);
        assert!((slplen - 60.0).abs() < 1e-12);
        assert!((nslpts - 3.0).abs() < 1e-12);
        assert!((avgslp - 0.058).abs() < 1e-12);
        assert!((xinput_2 - 0.6).abs() < 1e-12);
        assert!((slpinp_2 - 0.08).abs() < 1e-12);
        assert!((ofe2_avgslp - 0.0425).abs() < 1e-12);
    }

    #[test]
    fn slope_runtime_surface_rejects_non_positive_derived_avgslp() {
        let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");

        for point in &mut slope.ofes[0].points {
            point.slpinp = 0.0;
        }

        let error = build_hillslope_runtime_surface_from_slope(&slope)
            .expect_err("non-positive derived avgslp must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-023");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
                ofe_index: 1,
                value
            } if value.abs() < 1e-12
        ));
    }

    #[test]
    fn management_runtime_surfaces_project_required_pl_controls_and_seeds() {
        let management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("pl runtime surface projection should succeed");
        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("merged pl runtime state surface should build");

        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("lanuse")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("itype")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("pl_order_decomp_before_soil")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdharv")),
            Some(&BoundaryValue::scalar(288.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdplt")),
            Some(&BoundaryValue::scalar(130.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("resmgt")),
            Some(&BoundaryValue::scalar(6.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumrtm_seed")),
            Some(&BoundaryValue::scalar(0.50003))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumsrm_seed")),
            Some(&BoundaryValue::scalar(0.19997))
        );

        assert_eq!(
            pl_surfaces.pl_schedule_surface.get(&BoundarySymbol::from(
                "pl_schedule_slot_0001_crop_0001_itype"
            )),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_resmgt"
            )),
            Some(&BoundaryValue::scalar(6.0))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_out_of_range_initial_reference() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.schedule.ofe_initial_refs[0] = 0;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("out-of-range initial reference must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-039");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                ofe_index: 1,
                initial_ref: 0,
                max_initial_ref: 1
            }
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_unsupported_pl_landuse() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.registries.initials[0].meta.landuse = 2;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("unsupported landuse must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-041");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: 2
            }
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_non_finite_row_width() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        match &mut cropland.branch {
            YearlyCroplandBranch::AnnualOrFallow(annual) => annual.rw = f64::NAN,
            YearlyCroplandBranch::Perennial(_) => panic!("fixture should use annual branch"),
        }

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("non-finite row width must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-043");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "rw",
                slot_index: 1,
                crop_slot_index: 1,
                value,
            } if value.is_nan()
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_unsupported_perennial_option() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 288,
            jdplt: 130,
            jdstop: 0,
            rw: 0.762,
            mgtopt: 4,
            cut_days: Vec::new(),
            grazing_cycles: Vec::new(),
        });

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("unsupported perennial mgtopt must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-042");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                field: "mgtopt",
                value: 4,
                allowed: "1..3",
            }
        ));
    }

    #[test]
    fn climate_runtime_surface_contains_canonical_daily_symbols() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("climate runtime surface should build");

        let datver = surface
            .state_surface
            .get(&BoundarySymbol::from("datver"))
            .expect("datver should exist")
            .as_f64();
        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        let ninten = surface
            .state_surface
            .get(&BoundarySymbol::from("ninten"))
            .expect("ninten should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let intsty_first = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0001"))
            .expect("intsty_0001 should exist")
            .as_f64();

        assert!((datver - 5.3).abs() < 1e-12);
        assert!((iclig - 1.0).abs() < 1e-12);
        assert!((prcp - 0.01).abs() < 1e-12);
        assert!((stmdur - 7_200.0).abs() < 1e-12);
        assert!((ip - 2.1).abs() < 1e-12);
        assert!(ninten >= 2.0);
        assert!(timem_first.abs() < 1e-12);
        assert!(intsty_first.is_finite());
    }

    #[test]
    fn breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint() {
        let climate =
            parse_climate_from_str(WC1_BREAKPOINT_STMSTR_NONZERO, ClimateParserMode::Strict)
                .expect("curated wc1 breakpoint fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("breakpoint runtime surface should build");

        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("stmstr"))
            .expect("stmstr should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("mxint"))
            .expect("mxint should exist")
            .as_f64();
        let timem_1 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0002"))
            .expect("timem_0002 should exist")
            .as_f64();
        let intsty_5 = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0005"))
            .expect("intsty_0005 should exist")
            .as_f64();

        let times_h = [4.8667_f64, 17.2667, 19.4333, 21.3667, 23.9833];
        let pptcum_mm = [0.0_f64, 2.01, 4.02, 6.04, 7.35];
        let expected_stmdur = (times_h[4] - times_h[0]) * 3_600.0;
        let expected_timem_2 = (times_h[1] - times_h[0]) * 3_600.0;
        let mut expected_mxint: f64 = 0.0;
        for index in 1..times_h.len() {
            let drain_m = (pptcum_mm[index] - pptcum_mm[index - 1]) * 0.001;
            let delta_time_s = (times_h[index] - times_h[index - 1]) * 3_600.0;
            expected_mxint = expected_mxint.max(drain_m / delta_time_s);
        }

        assert!((stmstr - 4.8667).abs() < 1e-12);
        assert!((prcp - 0.00735).abs() < 1e-12);
        assert!((stmdur - expected_stmdur).abs() < 1e-6);
        assert!((mxint - expected_mxint).abs() < 1e-12);
        assert!(timem_1.abs() < 1e-12);
        assert!((timem_2 - expected_timem_2).abs() < 1e-6);
        assert!(intsty_5.abs() < 1e-12);
    }

    #[test]
    fn breakpoint_runtime_surface_supports_curated_wc1_42_point_event_shape() {
        let climate = parse_climate_from_str(WC1_BREAKPOINT_NBRKPT_42, ClimateParserMode::Strict)
            .expect("42-point wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("42-point breakpoint surface should build");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_last = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0042"))
            .expect("timem_0042 should exist")
            .as_f64();
        let intsty_last = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0042"))
            .expect("intsty_0042 should exist")
            .as_f64();

        assert!((nbrkpt - 42.0).abs() < 1e-12);
        assert!(timem_first.abs() < 1e-12);
        assert!(timem_last > timem_first);
        assert!(intsty_last.abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("runtime seam should accept 1500 breakpoint rows");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        assert!((nbrkpt - 1_500.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override()
    {
        let climate = parse_climate_from_str(
            &build_breakpoint_fixture(1_501),
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should allow >1500 breakpoint rows with explicit override");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject >1500 breakpoint rows");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: 1_501,
                max: 1_500
            }
        ));
    }

    #[test]
    fn climate_runtime_surface_supports_explicit_datver_zero_override() {
        let climate = parse_climate_from_str(LEGACY_DATVER_CLIMATE, ClimateParserMode::Strict)
            .expect("legacy datver fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("seam should accept explicit datver=0.0 override");

        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist for datver override")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist for datver override")
            .as_f64();
        assert!((iclig - 0.0).abs() < 1e-12);
        assert!((ip - 2.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_applies_timep_floor_for_wet_nonconstant_events() {
        let climate = parse_climate_from_str(WC1_CANOGA_DAY1, ClimateParserMode::Strict)
            .expect("wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("wc1 runtime surface should build");

        let timep = surface
            .state_surface
            .get(&BoundarySymbol::from("timep"))
            .expect("timep should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((timep - 0.01).abs() < 1e-12);
        assert!((ip - 2.94).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_caps_storm_duration_to_23_999_hours() {
        let climate = parse_climate_from_str(WC1_CANOGA_STMDUR_CAP, ClimateParserMode::Strict)
            .expect("wc1 duration-cap fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("duration-cap fixture should build runtime surface");

        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((stmdur - (23.999 * 3_600.0)).abs() < 1e-9);
        assert!((ip - 22.589).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_pre4_nonzero_datver_branch() {
        let mut climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        climate.datver = 3.9;

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("pre-4 nonzero branch must be rejected");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-001");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedDatver { datver } if (datver - 3.9).abs() < 1e-12
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_single_storm_even_in_compat_parser_mode() {
        let climate = parse_climate_from_str(
            SINGLE_STORM_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: true,
                allow_breakpoint_cardinality_override: false,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should accept itemp=2 when explicitly enabled");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject single-storm itemp=2");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-002");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedItemp { itemp: 2 }
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_duplicate_breakpoint_times() {
        let mut climate = parse_climate_from_str(
            BREAKPOINT_OVERFLOW_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("breakpoint fixture should parse in compatibility mode");

        let day = climate
            .daily_records
            .first_mut()
            .expect("one breakpoint day expected");
        match day {
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
                let first_timem = record
                    .breakpoints
                    .first()
                    .expect("first breakpoint point should exist")
                    .timem;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .timem = first_timem;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("duplicate breakpoint timem must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_negative_breakpoint_drain() {
        let mut climate = parse_climate_from_str(
            BREAKPOINT_OVERFLOW_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("breakpoint fixture should parse in compatibility mode");

        let day = climate
            .daily_records
            .first_mut()
            .expect("one breakpoint day expected");
        match day {
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
                record
                    .breakpoints
                    .first_mut()
                    .expect("first breakpoint point should exist")
                    .pptcum = 0.02;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .pptcum = 0.01;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("negative breakpoint drain must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-006");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::NegativeField {
                field: "drain",
                value
            } if value < 0.0
        ));
    }
}
