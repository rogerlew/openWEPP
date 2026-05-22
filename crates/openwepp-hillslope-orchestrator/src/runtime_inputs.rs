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
    management::{
        InitialScenarioData, ManagementParseOutput, YearlyCroplandBranch, YearlyScenarioData,
    },
    slope::{SlopePoint, SlopeProfile},
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
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", annual.jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", annual.jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index),
                        BoundaryValue::scalar(annual.rw),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("resmgt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                    );

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
                            BoundaryValue::scalar(usize_to_f64("jdharv", annual.jdharv)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("jdplt"),
                            BoundaryValue::scalar(usize_to_f64("jdplt", annual.jdplt)?),
                        );
                        pl_growth_surface
                            .insert(BoundarySymbol::from("rw"), BoundaryValue::scalar(annual.rw));
                        pl_decomp_surface.insert(
                            BoundarySymbol::from("resmgt"),
                            BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                        );
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
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", perennial.jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", perennial.jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdstop", perennial.jdstop)?),
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
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("ncut", perennial.cut_days.len())?),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64(
                            "ncycle",
                            perennial.grazing_cycles.len(),
                        )?),
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
