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
    climate::{ClimateFile, ClimateMetadata, ClimateMonthlyStats},
    frost::FrostParseOutput,
    irrigation_depletion::{IrrigationDepletionFile, IrrigationPeriodData},
    irrigation_fixeddate::{FixedDateEvent, FixedDateIrrigationFile},
    management::{
        InitialScenarioData, ManagementParseOutput, PlantScenarioData, YearlyAnnualExtension,
        YearlyCroplandBranch, YearlyScenarioData,
    },
    slope::{SlopePoint, SlopeProfile},
    snow::SnowParseOutput,
    soil::{DisturbedPolicy, SoilDatver, SoilProfile},
};
use openwepp_kernel_contract::{
    BoundaryError, BoundarySymbol, BoundaryValue, ClimateForcingSymbolSurface,
    ClimateForcingSymbolSurfaceError,
};

use crate::constants::{
    PL_GROWTH_ANNUAL_LAI_A, PL_GROWTH_ANNUAL_LAI_B, PL_GROWTH_CANCOV_MAX,
    PL_GROWTH_PERENNIAL_LAI_A, PL_GROWTH_PERENNIAL_LAI_B,
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
    MissingCorrectedLayerNormalizationInput {
        ofe_index: usize,
        layer_index: usize,
        field: &'static str,
    },
    CorrectedLayerNormalizationUnavailable {
        ofe_index: usize,
    },
    CorrectedLayerMappingIncomplete {
        ofe_index: usize,
        layer_index: usize,
        layer_top_depth_mm: f64,
        layer_bottom_depth_mm: f64,
        covered_depth_mm: f64,
    },
    NonFiniteProfileFcTailContribution {
        ofe_index: usize,
        value_mm: f64,
    },
    NegativeProfileFcTailContribution {
        ofe_index: usize,
        value_mm: f64,
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
            Self::MissingSoilOfe
            | Self::MissingSoilLayer
            | Self::MissingThetaResidual
            | Self::MissingThetaFieldCapacity
            | Self::NonFiniteProfileDepth { .. }
            | Self::NonPositiveProfileDepth { .. }
            | Self::NonFiniteTopLayerDepth { .. }
            | Self::NonPositiveTopLayerDepth { .. }
            | Self::NonFiniteThetaResidual { .. }
            | Self::NonFiniteThetaFieldCapacity { .. }
            | Self::SoilOfeCountMismatch { .. }
            | Self::SoilOfeCountOutOfRange { .. } => self.soil_core_code(),
            Self::SoilLayerCountMismatch { .. }
            | Self::SoilLayerCountOutOfRange { .. }
            | Self::NonFiniteLayerDepth { .. }
            | Self::NonPositiveLayerDepth { .. }
            | Self::NonMonotoneLayerDepth { .. }
            | Self::MissingSaturatedConductivity { .. }
            | Self::NonFiniteSaturatedConductivity { .. }
            | Self::NonPositiveSaturatedConductivity { .. } => self.soil_layer_code(),
            Self::MissingCorrectedLayerNormalizationInput { .. }
            | Self::CorrectedLayerNormalizationUnavailable { .. }
            | Self::CorrectedLayerMappingIncomplete { .. }
            | Self::NonFiniteProfileFcTailContribution { .. }
            | Self::NegativeProfileFcTailContribution { .. } => self.soil_corrected_code(),
            Self::MissingSlopeOfe
            | Self::SlopeOfeCountMismatch { .. }
            | Self::SlopeOfeCountOutOfRange { .. }
            | Self::SlopePointCountMismatch { .. }
            | Self::SlopePointCountOutOfRange { .. }
            | Self::InsufficientSlopePoints { .. }
            | Self::NonFiniteSlopeLength { .. }
            | Self::NonPositiveSlopeLength { .. } => self.slope_shape_code(),
            Self::NonFiniteXinput { .. }
            | Self::NonFiniteSlpinp { .. }
            | Self::NonMonotoneXinput { .. }
            | Self::NonFiniteDerivedAverageSlope { .. }
            | Self::NonPositiveDerivedAverageSlope { .. }
            | Self::NonFiniteDerivedSlopeLength { .. }
            | Self::NonPositiveDerivedSlopeLength { .. } => self.slope_numeric_code(),
            Self::ManagementTopologyCountMismatch { .. }
            | Self::ManagementScheduleSlotCountMismatch { .. }
            | Self::ManagementScheduleSlotArityMismatch { .. }
            | Self::ManagementInitialReferenceOutOfRange { .. }
            | Self::ManagementYearlyReferenceOutOfRange { .. }
            | Self::ManagementScheduleOfeIndexOutOfRange { .. } => self.management_code(),
            Self::UnsupportedPlLanduse { .. }
            | Self::UnsupportedPlManagementOption { .. }
            | Self::NonFinitePlProjectionField { .. }
            | Self::PlProjectionCountOutOfRange { .. }
            | Self::PlProjectionDayOutOfDomain { .. }
            | Self::PlAnnualExtensionMismatch { .. }
            | Self::PlProjectionCardinalityInvalid { .. }
            | Self::PlGrazingWindowOutOfDomain { .. }
            | Self::PlProjectionFieldOutOfDomain { .. }
            | Self::PlProjectionUnsupportedPayloadCombination { .. } => self.pl_projection_code(),
            Self::NonFiniteSnowControl { .. }
            | Self::SnowControlOutOfDomain { .. }
            | Self::NonFiniteFrostControl { .. }
            | Self::FrostControlOutOfDomain { .. }
            | Self::MissingIrrigationScheduleField { .. }
            | Self::NonFiniteIrrigationScheduleField { .. }
            | Self::IrrigationScheduleFieldOutOfDomain { .. }
            | Self::IrrigationScheduleCountOutOfRange { .. } => self.snow_frost_irrigation_code(),
        }
    }

    const fn soil_core_code(&self) -> &'static str {
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
            Self::SoilOfeCountMismatch { .. } => "HS-RUNTIME-E-026",
            Self::SoilOfeCountOutOfRange { .. } => "HS-RUNTIME-E-027",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn soil_layer_code(&self) -> &'static str {
        match self {
            Self::SoilLayerCountMismatch { .. } => "HS-RUNTIME-E-028",
            Self::SoilLayerCountOutOfRange { .. } => "HS-RUNTIME-E-029",
            Self::NonFiniteLayerDepth { .. } => "HS-RUNTIME-E-030",
            Self::NonPositiveLayerDepth { .. } => "HS-RUNTIME-E-031",
            Self::NonMonotoneLayerDepth { .. } => "HS-RUNTIME-E-032",
            Self::MissingSaturatedConductivity { .. } => "HS-RUNTIME-E-033",
            Self::NonFiniteSaturatedConductivity { .. } => "HS-RUNTIME-E-034",
            Self::NonPositiveSaturatedConductivity { .. } => "HS-RUNTIME-E-035",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn soil_corrected_code(&self) -> &'static str {
        match self {
            Self::MissingCorrectedLayerNormalizationInput { .. } => "HS-RUNTIME-E-060",
            Self::CorrectedLayerNormalizationUnavailable { .. } => "HS-RUNTIME-E-061",
            Self::CorrectedLayerMappingIncomplete { .. } => "HS-RUNTIME-E-062",
            Self::NonFiniteProfileFcTailContribution { .. } => "HS-RUNTIME-E-063",
            Self::NegativeProfileFcTailContribution { .. } => "HS-RUNTIME-E-064",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn slope_shape_code(&self) -> &'static str {
        match self {
            Self::MissingSlopeOfe => "HS-RUNTIME-E-011",
            Self::SlopeOfeCountMismatch { .. } => "HS-RUNTIME-E-012",
            Self::SlopeOfeCountOutOfRange { .. } => "HS-RUNTIME-E-013",
            Self::SlopePointCountMismatch { .. } => "HS-RUNTIME-E-014",
            Self::SlopePointCountOutOfRange { .. } => "HS-RUNTIME-E-015",
            Self::InsufficientSlopePoints { .. } => "HS-RUNTIME-E-016",
            Self::NonFiniteSlopeLength { .. } => "HS-RUNTIME-E-017",
            Self::NonPositiveSlopeLength { .. } => "HS-RUNTIME-E-018",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn slope_numeric_code(&self) -> &'static str {
        match self {
            Self::NonFiniteXinput { .. } => "HS-RUNTIME-E-019",
            Self::NonFiniteSlpinp { .. } => "HS-RUNTIME-E-020",
            Self::NonMonotoneXinput { .. } => "HS-RUNTIME-E-021",
            Self::NonFiniteDerivedAverageSlope { .. } => "HS-RUNTIME-E-022",
            Self::NonPositiveDerivedAverageSlope { .. } => "HS-RUNTIME-E-023",
            Self::NonFiniteDerivedSlopeLength { .. } => "HS-RUNTIME-E-024",
            Self::NonPositiveDerivedSlopeLength { .. } => "HS-RUNTIME-E-025",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn management_code(&self) -> &'static str {
        match self {
            Self::ManagementTopologyCountMismatch { .. } => "HS-RUNTIME-E-036",
            Self::ManagementScheduleSlotCountMismatch { .. } => "HS-RUNTIME-E-037",
            Self::ManagementScheduleSlotArityMismatch { .. } => "HS-RUNTIME-E-038",
            Self::ManagementInitialReferenceOutOfRange { .. } => "HS-RUNTIME-E-039",
            Self::ManagementYearlyReferenceOutOfRange { .. } => "HS-RUNTIME-E-040",
            Self::ManagementScheduleOfeIndexOutOfRange { .. } => "HS-RUNTIME-E-045",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn pl_projection_code(&self) -> &'static str {
        match self {
            Self::UnsupportedPlLanduse { .. } => "HS-RUNTIME-E-041",
            Self::UnsupportedPlManagementOption { .. } => "HS-RUNTIME-E-042",
            Self::NonFinitePlProjectionField { .. } => "HS-RUNTIME-E-043",
            Self::PlProjectionCountOutOfRange { .. } => "HS-RUNTIME-E-044",
            Self::PlProjectionDayOutOfDomain { .. } => "HS-RUNTIME-E-046",
            Self::PlAnnualExtensionMismatch { .. } => "HS-RUNTIME-E-047",
            Self::PlProjectionCardinalityInvalid { .. } => "HS-RUNTIME-E-048",
            Self::PlGrazingWindowOutOfDomain { .. } => "HS-RUNTIME-E-049",
            Self::PlProjectionFieldOutOfDomain { .. } => "HS-RUNTIME-E-050",
            Self::PlProjectionUnsupportedPayloadCombination { .. } => "HS-RUNTIME-E-051",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    const fn snow_frost_irrigation_code(&self) -> &'static str {
        match self {
            Self::NonFiniteSnowControl { .. } => "HS-RUNTIME-E-052",
            Self::SnowControlOutOfDomain { .. } => "HS-RUNTIME-E-053",
            Self::NonFiniteFrostControl { .. } => "HS-RUNTIME-E-054",
            Self::FrostControlOutOfDomain { .. } => "HS-RUNTIME-E-055",
            Self::MissingIrrigationScheduleField { .. } => "HS-RUNTIME-E-056",
            Self::NonFiniteIrrigationScheduleField { .. } => "HS-RUNTIME-E-057",
            Self::IrrigationScheduleFieldOutOfDomain { .. } => "HS-RUNTIME-E-058",
            Self::IrrigationScheduleCountOutOfRange { .. } => "HS-RUNTIME-E-059",
            _ => panic!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }
}

impl HillslopeRuntimeInputError {
    fn fmt_soil_core(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSoilOfe => {
                write!(f, "{}: soil profile contains no OFE blocks", self.code())
            }
            Self::MissingSoilLayer => {
                write!(f, "{}: primary OFE contains no soil layers", self.code())
            }
            Self::MissingThetaResidual => write!(
                f,
                "{}: primary soil layer missing required theta source for thetdr (theta_r_rosetta or wp_measured)",
                self.code()
            ),
            Self::MissingThetaFieldCapacity => write!(
                f,
                "{}: primary soil layer missing required theta source for thetfc (fc_rosetta or fc_measured)",
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_soil_layer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_soil_corrected(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCorrectedLayerNormalizationInput {
                ofe_index,
                layer_index,
                field,
            } => write!(
                f,
                "{}: soil OFE {} layer {} missing corrected-lineage normalization input field {}",
                self.code(),
                ofe_index,
                layer_index,
                field
            ),
            Self::CorrectedLayerNormalizationUnavailable { ofe_index } => write!(
                f,
                "{}: soil OFE {} cannot derive normalized corrected-layer lineage for authoritative FC/WP projection",
                self.code(),
                ofe_index
            ),
            Self::CorrectedLayerMappingIncomplete {
                ofe_index,
                layer_index,
                layer_top_depth_mm,
                layer_bottom_depth_mm,
                covered_depth_mm,
            } => write!(
                f,
                "{}: soil OFE {} layer {} corrected-lineage mapping coverage incomplete ({}..{} mm, covered {} mm)",
                self.code(),
                ofe_index,
                layer_index,
                layer_top_depth_mm,
                layer_bottom_depth_mm,
                covered_depth_mm
            ),
            Self::NonFiniteProfileFcTailContribution { ofe_index, value_mm } => write!(
                f,
                "{}: soil OFE {} produced non-finite ProfileFC tail contribution {} mm",
                self.code(),
                ofe_index,
                value_mm
            ),
            Self::NegativeProfileFcTailContribution { ofe_index, value_mm } => write!(
                f,
                "{}: soil OFE {} produced negative ProfileFC tail contribution {} mm",
                self.code(),
                ofe_index,
                value_mm
            ),
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_slope_shape(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_slope_numeric(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_management(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_pl_projection(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlLanduse { .. }
            | Self::UnsupportedPlManagementOption { .. }
            | Self::NonFinitePlProjectionField { .. }
            | Self::PlProjectionCountOutOfRange { .. }
            | Self::PlProjectionDayOutOfDomain { .. }
            | Self::PlAnnualExtensionMismatch { .. } => self.fmt_pl_projection_shape(f),
            Self::PlProjectionCardinalityInvalid { .. }
            | Self::PlGrazingWindowOutOfDomain { .. }
            | Self::PlProjectionFieldOutOfDomain { .. }
            | Self::PlProjectionUnsupportedPayloadCombination { .. } => {
                self.fmt_pl_projection_payload(f)
            }
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_pl_projection_shape(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_pl_projection_payload(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }

    fn fmt_snow_frost_irrigation(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => unreachable!("routed HillslopeRuntimeInputError family mismatch"),
        }
    }
}

impl fmt::Display for HillslopeRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSoilOfe
            | Self::MissingSoilLayer
            | Self::MissingThetaResidual
            | Self::MissingThetaFieldCapacity
            | Self::NonFiniteProfileDepth { .. }
            | Self::NonPositiveProfileDepth { .. }
            | Self::NonFiniteTopLayerDepth { .. }
            | Self::NonPositiveTopLayerDepth { .. }
            | Self::NonFiniteThetaResidual { .. }
            | Self::NonFiniteThetaFieldCapacity { .. }
            | Self::SoilOfeCountMismatch { .. }
            | Self::SoilOfeCountOutOfRange { .. } => self.fmt_soil_core(f),
            Self::SoilLayerCountMismatch { .. }
            | Self::SoilLayerCountOutOfRange { .. }
            | Self::NonFiniteLayerDepth { .. }
            | Self::NonPositiveLayerDepth { .. }
            | Self::NonMonotoneLayerDepth { .. }
            | Self::MissingSaturatedConductivity { .. }
            | Self::NonFiniteSaturatedConductivity { .. }
            | Self::NonPositiveSaturatedConductivity { .. } => self.fmt_soil_layer(f),
            Self::MissingCorrectedLayerNormalizationInput { .. }
            | Self::CorrectedLayerNormalizationUnavailable { .. }
            | Self::CorrectedLayerMappingIncomplete { .. }
            | Self::NonFiniteProfileFcTailContribution { .. }
            | Self::NegativeProfileFcTailContribution { .. } => self.fmt_soil_corrected(f),
            Self::MissingSlopeOfe
            | Self::SlopeOfeCountMismatch { .. }
            | Self::SlopeOfeCountOutOfRange { .. }
            | Self::SlopePointCountMismatch { .. }
            | Self::SlopePointCountOutOfRange { .. }
            | Self::InsufficientSlopePoints { .. }
            | Self::NonFiniteSlopeLength { .. }
            | Self::NonPositiveSlopeLength { .. } => self.fmt_slope_shape(f),
            Self::NonFiniteXinput { .. }
            | Self::NonFiniteSlpinp { .. }
            | Self::NonMonotoneXinput { .. }
            | Self::NonFiniteDerivedAverageSlope { .. }
            | Self::NonPositiveDerivedAverageSlope { .. }
            | Self::NonFiniteDerivedSlopeLength { .. }
            | Self::NonPositiveDerivedSlopeLength { .. } => self.fmt_slope_numeric(f),
            Self::ManagementTopologyCountMismatch { .. }
            | Self::ManagementScheduleSlotCountMismatch { .. }
            | Self::ManagementScheduleSlotArityMismatch { .. }
            | Self::ManagementInitialReferenceOutOfRange { .. }
            | Self::ManagementYearlyReferenceOutOfRange { .. }
            | Self::ManagementScheduleOfeIndexOutOfRange { .. } => self.fmt_management(f),
            Self::UnsupportedPlLanduse { .. }
            | Self::UnsupportedPlManagementOption { .. }
            | Self::NonFinitePlProjectionField { .. }
            | Self::PlProjectionCountOutOfRange { .. }
            | Self::PlProjectionDayOutOfDomain { .. }
            | Self::PlAnnualExtensionMismatch { .. }
            | Self::PlProjectionCardinalityInvalid { .. }
            | Self::PlGrazingWindowOutOfDomain { .. }
            | Self::PlProjectionFieldOutOfDomain { .. }
            | Self::PlProjectionUnsupportedPayloadCombination { .. } => self.fmt_pl_projection(f),
            Self::NonFiniteSnowControl { .. }
            | Self::SnowControlOutOfDomain { .. }
            | Self::NonFiniteFrostControl { .. }
            | Self::FrostControlOutOfDomain { .. }
            | Self::MissingIrrigationScheduleField { .. }
            | Self::NonFiniteIrrigationScheduleField { .. }
            | Self::IrrigationScheduleFieldOutOfDomain { .. }
            | Self::IrrigationScheduleCountOutOfRange { .. } => {
                self.fmt_snow_frost_irrigation(f)
            }
        }
    }
}

impl Error for HillslopeRuntimeInputError {}
