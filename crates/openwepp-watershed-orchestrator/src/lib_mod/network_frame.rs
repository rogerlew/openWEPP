use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_input_contract::parsers::{
    chaninp::{ChaninpFile, ChaninpParseOutcome},
    slope::{DistanceMode, SlopeProfile},
    watershed_channel::{ChannelRatingCurve, WatershedChannelFile},
    watershed_impoundment::{ImpoundmentRecord, WatershedImpoundmentFile},
};
use openwepp_topology::{TopologyGraph, TopologyNodeKind};

use crate::runtime_inputs::WatershedRuntimeInputError;

use super::types::{DispatchStep, WatershedFrameExecutionReport};

const METERS_TO_FEET: f64 = 3.281;

/// Failures while building or projecting the typed watershed network frame.
#[derive(Debug)]
pub enum WatershedNetworkFrameError {
    ChaninpNotRuntimeReady {
        observed: ChaninpParseOutcome,
        chaninp_ipeak: i32,
        channel_ipeak: i32,
    },
    MissingChaninpOptions,
    ChannelIdOutOfRange {
        channel_id: usize,
    },
    ImpoundmentIdOutOfRange {
        impoundment_index: usize,
    },
    MissingSlopeProfile {
        channel_id: u32,
        slope_profile_count: usize,
    },
    RuntimeInput(WatershedRuntimeInputError),
    MissingRoutedChannelState {
        node_id: u32,
    },
    MissingRoutedImpoundmentState {
        node_id: u32,
    },
    InvalidGroundwaterAuthority {
        field: &'static str,
        value: f64,
    },
    InvalidTerminalPublication {
        node_id: u32,
        field: &'static str,
        value: f64,
    },
}

impl fmt::Display for WatershedNetworkFrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChaninpNotRuntimeReady {
                observed,
                chaninp_ipeak,
                channel_ipeak,
            } => write!(
                formatter,
                "WSHEDFRAME-E-001 chan.inp parse outcome {observed:?} is not runtime-ready for chaninp ipeak {chaninp_ipeak} and channel ipeak {channel_ipeak}"
            ),
            Self::MissingChaninpOptions => {
                write!(formatter, "WSHEDFRAME-E-002 chan.inp options are missing")
            }
            Self::ChannelIdOutOfRange { channel_id } => write!(
                formatter,
                "WSHEDFRAME-E-003 channel id {channel_id} exceeds typed frame range"
            ),
            Self::ImpoundmentIdOutOfRange { impoundment_index } => write!(
                formatter,
                "WSHEDFRAME-E-004 impoundment index {impoundment_index} exceeds typed frame range"
            ),
            Self::MissingSlopeProfile {
                channel_id,
                slope_profile_count,
            } => write!(
                formatter,
                "WSHEDFRAME-E-005 missing slope profile for channel {channel_id}; profile_count={slope_profile_count}"
            ),
            Self::RuntimeInput(source) => {
                write!(formatter, "WSHEDFRAME-E-006 runtime input failed: {source}")
            }
            Self::MissingRoutedChannelState { node_id } => write!(
                formatter,
                "WSHEDFRAME-E-007 missing routed channel state for node {node_id}"
            ),
            Self::MissingRoutedImpoundmentState { node_id } => write!(
                formatter,
                "WSHEDFRAME-E-008 missing routed impoundment state for node {node_id}"
            ),
            Self::InvalidGroundwaterAuthority { field, value } => write!(
                formatter,
                "WSHEDFRAME-E-009 invalid groundwater authority field {field}={value}"
            ),
            Self::InvalidTerminalPublication {
                node_id,
                field,
                value,
            } => write!(
                formatter,
                "WSHEDFRAME-E-010 invalid terminal publication for channel {node_id}: {field}={value}"
            ),
        }
    }
}

impl Error for WatershedNetworkFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RuntimeInput(source) => Some(source),
            Self::ChaninpNotRuntimeReady { .. }
            | Self::MissingChaninpOptions
            | Self::ChannelIdOutOfRange { .. }
            | Self::ImpoundmentIdOutOfRange { .. }
            | Self::MissingSlopeProfile { .. }
            | Self::MissingRoutedChannelState { .. }
            | Self::MissingRoutedImpoundmentState { .. }
            | Self::InvalidGroundwaterAuthority { .. }
            | Self::InvalidTerminalPublication { .. } => None,
        }
    }
}

impl From<WatershedRuntimeInputError> for WatershedNetworkFrameError {
    fn from(value: WatershedRuntimeInputError) -> Self {
        Self::RuntimeInput(value)
    }
}

/// Global channel-routing controls formerly stored in watershed symbol maps.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedRoutingGlobals {
    pub ipeak: i32,
    pub nchan: u32,
    pub dtchr_seconds: f64,
    pub ntchr: f64,
    pub nchnum: f64,
    pub cbase: f64,
    pub groundwater_baseflow: WatershedGroundwaterRoutingAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatershedGroundwaterRoutingAuthority {
    Disabled,
    LinearReservoir { baseflow_threshold_area_ha: f64 },
}

impl WatershedGroundwaterRoutingAuthority {
    #[must_use]
    pub const fn disabled() -> Self {
        Self::Disabled
    }

    /// # Errors
    ///
    /// Returns [`WatershedNetworkFrameError::InvalidGroundwaterAuthority`] if
    /// the threshold area is negative or non-finite.
    pub fn linear_reservoir(
        baseflow_threshold_area_ha: f64,
    ) -> Result<Self, WatershedNetworkFrameError> {
        if !baseflow_threshold_area_ha.is_finite() || baseflow_threshold_area_ha < 0.0 {
            return Err(WatershedNetworkFrameError::InvalidGroundwaterAuthority {
                field: "baseflow_threshold_area_ha",
                value: baseflow_threshold_area_ha,
            });
        }
        Ok(Self::LinearReservoir {
            baseflow_threshold_area_ha,
        })
    }

    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::LinearReservoir { .. })
    }
}

/// Typed channel rating-curve controls.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedChannelRatingCurveControl {
    pub rccoef: f64,
    pub rcexp: f64,
    pub rcoset: f64,
}

impl From<&ChannelRatingCurve> for WatershedChannelRatingCurveControl {
    fn from(value: &ChannelRatingCurve) -> Self {
        Self {
            rccoef: value.rccoef,
            rcexp: value.rcexp,
            rcoset: value.rcoset,
        }
    }
}

/// One typed channel segment-profile point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WatershedChannelSegmentPoint {
    pub x_m: f64,
    pub slope: f64,
    pub depth_a_ft: f64,
    pub depth_b_ft: f64,
    pub width_a_ft: f64,
    pub width_b_ft: f64,
}

/// Typed channel controls and segment scaffold for one channel node.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedChannelControlRecord {
    pub node_id: u32,
    pub ishape: i32,
    pub icntrl: i32,
    pub ienslp: i32,
    pub flgout: i32,
    pub chnz: f64,
    pub chnnbr: f64,
    pub chnn: f64,
    pub chnk: f64,
    pub chntcr: f64,
    pub chnedm: f64,
    pub chneds: f64,
    pub ctlslp: f64,
    pub ctlz: f64,
    pub ctln: f64,
    pub rating_curve: Option<WatershedChannelRatingCurveControl>,
    pub segment_points: Vec<WatershedChannelSegmentPoint>,
    pub ws20_case12_enabled: bool,
    pub ws21_case34_enabled: bool,
    pub crfrac: Vec<f64>,
}

/// Typed impoundment control record. The full parsed record is retained so the
/// direct WS12 kernel can derive the same coefficient families from parsed
/// impoundment input.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedImpoundmentControlRecord {
    pub node_id: u32,
    pub h: f64,
    pub hfull: f64,
    pub deltat: f64,
    pub qinf: f64,
    pub source_record: ImpoundmentRecord,
}

/// Typed hillslope contribution consumed by watershed routing.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeContribution {
    pub hillslope_id: u32,
    pub area_m2: Option<f64>,
    pub peak_runoff_m3_s: f64,
    pub duration_seconds: f64,
    pub generated_baseflow_m3: f64,
    pub groundwater_deep_seepage_m3: f64,
    pub total_detachment_kg: f64,
    pub total_deposition_kg: f64,
    pub sediment_concentration_kg_m3: Vec<f64>,
    pub particle_diameter_m: Vec<f64>,
    pub particle_flow_fraction: Vec<f64>,
    /// SC-INFILE-HBP-001 §3a minor-1 paired hourly surfaces (empty on
    /// minor-0 shards): hour-integrated exit runoff volume (m³) and
    /// exported sediment mass (kg) on a shared 24-slot time base.
    pub hourly_runoff_volume_m3: Vec<f64>,
    pub hourly_sediment_mass_kg: Vec<f64>,
}

impl HillslopeContribution {
    #[must_use]
    pub fn particle_class_count(&self) -> usize {
        self.sediment_concentration_kg_m3.len()
    }
}

/// WS11 routed wave state retained for downstream typed routing steps.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelWaveState {
    pub q1_m3_s: f64,
    pub qin_m3_s: f64,
    pub qlat_m3_s: f64,
    pub c0: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub c4: f64,
}

/// `SC-ROUTE-001#INV-ROUTE-015..016` routed water on the normalized channel
/// grid. Each vector has exactly `ntchr` entries and uses the same index.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelIntervalWaterState {
    pub dtchr_seconds: f64,
    pub qin_m3_s: Vec<f64>,
    pub qlat_total_m3_s: Vec<f64>,
    pub q1_m3_s: Vec<f64>,
    /// Diagnostic unrestricted interval flux residual. This is not the
    /// `SC-ROUTE-001#INV-ROUTE-021` hydraulic storage authority.
    pub storage_change_m3: Vec<f64>,
    pub initial_storage_m3: f64,
    pub final_storage_m3: f64,
}

/// Carried channel geometry for the active interval sediment lane.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelGeometryState {
    pub depth_a_points_ft: Vec<f64>,
    pub depth_b_points_ft: Vec<f64>,
    pub width_a_points_ft: Vec<f64>,
    pub width_b_points_ft: Vec<f64>,
    pub eroded_width_a_points_ft: Vec<f64>,
    pub eroded_width_b_points_ft: Vec<f64>,
}

/// Pinned hydraulic profile operands consumed by one active sediment interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutedChannelIntervalHydraulicState {
    pub qe_m3_s: f64,
    pub qt_m3_s: f64,
    pub qlat_total_m3_s: f64,
    pub leff_ft: f64,
    pub qu_top_cfs: f64,
    pub qlat_eff_cfs_per_ft: f64,
}

/// Explicit day-level tillage authority for carried `ishape=3` geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelTillageDayState {
    NoPrimaryTillage,
    PrimaryTillage,
}

/// Per-class mass ledger for one routed channel interval, in kilograms.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelIntervalClassLedger {
    pub inlet_kg: Vec<f64>,
    pub lateral_kg: Vec<f64>,
    pub detached_kg: Vec<f64>,
    pub deposited_kg: Vec<f64>,
    pub egress_kg: Vec<f64>,
    pub hydraulic: Option<RoutedChannelIntervalHydraulicState>,
    pub max_effective_shear_lb_ft2: f64,
    pub outlet_transport_capacity_kg_s: Vec<f64>,
}

/// `SC-ROUTE-001#INV-ROUTE-017..020` sediment and geometry state carried by
/// the real downstream channel consumer.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelIntervalSedimentState {
    pub particle_diameter_m: Vec<f64>,
    pub intervals: Vec<RoutedChannelIntervalClassLedger>,
    pub daily_inlet_kg: Vec<f64>,
    pub daily_lateral_kg: Vec<f64>,
    pub daily_detached_kg: Vec<f64>,
    pub daily_deposited_kg: Vec<f64>,
    pub daily_egress_kg: Vec<f64>,
    pub geometry_start: RoutedChannelGeometryState,
    pub geometry_end: RoutedChannelGeometryState,
}

/// WS18/WS20 routed sediment state retained for downstream typed routing steps.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RoutedChannelSedimentState {
    pub qsed_kg_s: f64,
    pub transport_capacity_kg_s: f64,
    pub particle_flow_fraction: Vec<f64>,
    pub particle_diameter_m: Vec<f64>,
    pub ws20_case1_segments: u32,
    pub ws20_case2_segments: u32,
    pub ws24_case2_detach_segments: u32,
    pub ws21_case3_segments: u32,
    pub ws21_case4_segments: u32,
    pub ws21_enddet_segments: u32,
}

/// Routed channel state after deterministic watershed dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedChannelState {
    pub node_id: u32,
    pub runoff_volume_m3: f64,
    pub channel_inflow_m3: f64,
    pub channel_outflow_m3: f64,
    pub channel_storage_m3: f64,
    pub peak_discharge_m3_s: f64,
    pub duration_seconds: f64,
    pub channel_baseflow_m3: f64,
    pub channel_loss_m3: f64,
    pub groundwater_deep_seepage_m3: f64,
    pub sediment_yield_kg: f64,
    pub wave_state: Option<RoutedChannelWaveState>,
    pub interval_water_state: Option<RoutedChannelIntervalWaterState>,
    pub sediment_state: RoutedChannelSedimentState,
    pub interval_sediment_state: Option<RoutedChannelIntervalSedimentState>,
}

/// Routed impoundment state after deterministic watershed dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedImpoundmentState {
    pub node_id: u32,
    pub outflow_volume_m3: f64,
    pub outflow_rate_m3_s: f64,
    pub duration_seconds: f64,
    pub hnext_m: f64,
}

/// Typed publication projection consumed by watershed output writers.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedPublicationFrame {
    pub year: i16,
    pub simulation_year: i16,
    pub sim_day_index: i32,
    pub julian: i16,
    pub month: i8,
    pub day_of_month: i8,
    pub water_year: i16,
    pub element_id: i32,
    pub channel_id: i32,
    pub runoff_volume_m3: f64,
    pub peak_discharge_m3_s: f64,
    pub sediment_yield_kg: f64,
    pub soluble_pollutant_kg: Option<f64>,
    pub particulate_pollutant_kg: Option<f64>,
    pub channel_inflow_m3: Option<f64>,
    pub channel_outflow_m3: Option<f64>,
    pub channel_storage_m3: Option<f64>,
    pub channel_baseflow_m3: Option<f64>,
    pub channel_loss_m3: Option<f64>,
    pub area_m2: Option<f64>,
    pub subsurface_runoff_volume_m3: Option<f64>,
    pub total_detachment_kg: f64,
    pub total_deposition_kg: f64,
    pub sediment_class_deposition_kg: Option<[f64; 5]>,
    pub sediment_volume_concentration_m3_m3: Option<f64>,
    pub precipitation_mm: Option<f64>,
    pub rain_melt_mm: Option<f64>,
    pub runoff_mm: Option<f64>,
    pub q_diagnostic_mm: Option<f64>,
    pub deep_percolation_mm: Option<f64>,
    pub lateral_flow_mm: Option<f64>,
    pub qofe_mm: Option<f64>,
    pub transpiration_mm: Option<f64>,
    pub evaporation_soil_mm: Option<f64>,
    pub evaporation_residue_mm: Option<f64>,
    pub upstream_q_mm: Option<f64>,
    pub subsurface_runon_mm: Option<f64>,
    pub total_soil_water_mm: Option<f64>,
    pub soil_water_total_mm: Option<f64>,
    pub profile_depth_mm: Option<f64>,
    pub profile_porosity_cap_mm: Option<f64>,
    pub profile_fc_store_mm: Option<f64>,
    pub profile_wp_store_mm: Option<f64>,
    pub interception_mm: Option<f64>,
    pub interception_storage_mm: Option<f64>,
    pub frozen_water_mm: Option<f64>,
    pub snow_water_mm: Option<f64>,
    pub tile_mm: Option<f64>,
    pub irrigation_mm: Option<f64>,
    pub baseflow_mm: Option<f64>,
    pub tsmf_fraction: Option<f64>,
    pub qrain_mm: Option<f64>,
    pub qsnow_mm: Option<f64>,
}

impl Default for WatershedPublicationFrame {
    fn default() -> Self {
        Self {
            year: 1,
            simulation_year: 1,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 1,
            element_id: 1,
            channel_id: 1,
            runoff_volume_m3: 0.0,
            peak_discharge_m3_s: 0.0,
            sediment_yield_kg: 0.0,
            soluble_pollutant_kg: None,
            particulate_pollutant_kg: None,
            channel_inflow_m3: None,
            channel_outflow_m3: None,
            channel_storage_m3: None,
            channel_baseflow_m3: None,
            channel_loss_m3: None,
            area_m2: None,
            subsurface_runoff_volume_m3: None,
            total_detachment_kg: 0.0,
            total_deposition_kg: 0.0,
            sediment_class_deposition_kg: None,
            sediment_volume_concentration_m3_m3: None,
            precipitation_mm: None,
            rain_melt_mm: None,
            runoff_mm: None,
            q_diagnostic_mm: None,
            deep_percolation_mm: None,
            lateral_flow_mm: None,
            qofe_mm: None,
            transpiration_mm: None,
            evaporation_soil_mm: None,
            evaporation_residue_mm: None,
            upstream_q_mm: None,
            subsurface_runon_mm: None,
            total_soil_water_mm: None,
            soil_water_total_mm: None,
            profile_depth_mm: None,
            profile_porosity_cap_mm: None,
            profile_fc_store_mm: None,
            profile_wp_store_mm: None,
            interception_mm: None,
            interception_storage_mm: None,
            frozen_water_mm: None,
            snow_water_mm: None,
            tile_mm: None,
            irrigation_mm: None,
            baseflow_mm: None,
            tsmf_fraction: None,
            qrain_mm: None,
            qsnow_mm: None,
        }
    }
}

/// Typed watershed routing frame. Parsed inputs are retained as private source
/// records for frame-native routing and publication.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedNetworkFrame {
    topology: TopologyGraph,
    chaninp_source: Option<ChaninpFile>,
    channel_source: WatershedChannelFile,
    slope_source: SlopeProfile,
    impoundment_source: WatershedImpoundmentFile,
    pub routing_globals: WatershedRoutingGlobals,
    pub channel_controls: BTreeMap<u32, WatershedChannelControlRecord>,
    pub impoundment_controls: BTreeMap<u32, WatershedImpoundmentControlRecord>,
    pub hillslope_contributions: BTreeMap<u32, HillslopeContribution>,
    pub routed_channels: BTreeMap<u32, RoutedChannelState>,
    pub channel_tillage_day_state: BTreeMap<u32, ChannelTillageDayState>,
    pub routed_impoundments: BTreeMap<u32, RoutedImpoundmentState>,
    pub publication_frame: Option<WatershedPublicationFrame>,
}

impl WatershedNetworkFrame {
    /// Build the typed network frame from validated watershed parser products.
    ///
    /// # Errors
    ///
    /// Returns `WatershedNetworkFrameError` when typed id conversion fails,
    /// required `chan.inp` runtime options are absent, or channel slope-profile
    /// mapping is incomplete.
    pub fn from_parsed_inputs(
        topology: TopologyGraph,
        chaninp: Option<ChaninpFile>,
        channel: WatershedChannelFile,
        slope: SlopeProfile,
        impoundment: WatershedImpoundmentFile,
        default_dtchr_seconds: f64,
        default_ntchr: f64,
    ) -> Result<Self, WatershedNetworkFrameError> {
        let routing_globals = build_routing_globals(
            chaninp.as_ref(),
            &channel,
            default_dtchr_seconds,
            default_ntchr,
        )?;
        let channel_controls = build_channel_controls(&channel, &slope)?;
        let impoundment_controls = build_impoundment_controls(&impoundment)?;

        Ok(Self {
            topology,
            chaninp_source: chaninp,
            channel_source: channel,
            slope_source: slope,
            impoundment_source: impoundment,
            routing_globals,
            channel_controls,
            impoundment_controls,
            hillslope_contributions: BTreeMap::new(),
            routed_channels: BTreeMap::new(),
            channel_tillage_day_state: BTreeMap::new(),
            routed_impoundments: BTreeMap::new(),
            publication_frame: None,
        })
    }

    #[must_use]
    pub fn topology(&self) -> &TopologyGraph {
        &self.topology
    }

    pub fn add_hillslope_contribution(&mut self, contribution: HillslopeContribution) {
        self.hillslope_contributions
            .insert(contribution.hillslope_id, contribution);
    }

    pub fn configure_groundwater_baseflow_routing(
        &mut self,
        authority: WatershedGroundwaterRoutingAuthority,
    ) {
        self.routing_globals.groundwater_baseflow = authority;
    }

    pub fn set_channel_tillage_day_state(
        &mut self,
        channel_id: u32,
        state: ChannelTillageDayState,
    ) {
        self.channel_tillage_day_state.insert(channel_id, state);
    }

    pub(crate) fn record_routed_channel_state(&mut self, state: RoutedChannelState) {
        self.routed_channels.insert(state.node_id, state);
    }

    pub(crate) fn record_routed_impoundment_state(&mut self, state: RoutedImpoundmentState) {
        self.routed_impoundments.insert(state.node_id, state);
    }

    /// Publish typed routed state from the frame-native dispatch report.
    ///
    /// # Errors
    ///
    /// Returns `WatershedNetworkFrameError` when a dispatch step lacks its
    /// corresponding routed-state operand.
    pub fn publish_typed_routing_report(
        &mut self,
        report: &WatershedFrameExecutionReport,
    ) -> Result<WatershedPublicationFrame, WatershedNetworkFrameError> {
        let dispatch_ids = collect_dispatch_ids_from_steps(&report.dispatch_report.steps);
        let publication_frame = self.build_typed_publication_frame(report, &dispatch_ids)?;
        self.publication_frame = Some(publication_frame.clone());
        Ok(publication_frame)
    }

    fn build_typed_publication_frame(
        &self,
        report: &WatershedFrameExecutionReport,
        dispatch_ids: &TypedDispatchIds,
    ) -> Result<WatershedPublicationFrame, WatershedNetworkFrameError> {
        for node_id in &dispatch_ids.channel_ids {
            if !self.routed_channels.contains_key(node_id) {
                return Err(WatershedNetworkFrameError::MissingRoutedChannelState {
                    node_id: *node_id,
                });
            }
        }
        for node_id in &dispatch_ids.impoundment_ids {
            if !self.routed_impoundments.contains_key(node_id) {
                return Err(WatershedNetworkFrameError::MissingRoutedImpoundmentState {
                    node_id: *node_id,
                });
            }
        }

        // INV-SYSTEM-036: public event yield is an outlet reduction on every
        // routing lane. Upstream dispatched channels remain diagnostics and
        // must not be counted again as watershed yield.
        let publication_channel_ids = &dispatch_ids.outlet_channel_ids;
        let runoff_volume_m3 = publication_channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.runoff_volume_m3)
            .sum::<f64>();
        let channel_inflow_m3 = dispatch_ids
            .channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_inflow_m3)
            .sum::<f64>();
        let channel_outflow_m3 = dispatch_ids
            .channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_outflow_m3)
            .sum::<f64>();
        let channel_storage_m3 = dispatch_ids
            .channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_storage_m3)
            .sum::<f64>();
        let channel_baseflow_m3 = publication_channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_baseflow_m3)
            .sum::<f64>();
        let channel_loss_m3 = publication_channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_loss_m3)
            .sum::<f64>();
        let area_m2 = sum_contributing_area_m2(&self.hillslope_contributions, dispatch_ids);
        let runoff_mm = area_m2.map(|area| runoff_volume_m3 / area * 1_000.0);
        let total_detachment_kg = dispatch_ids
            .contributor_hillslopes
            .iter()
            .filter_map(|hillslope_id| self.hillslope_contributions.get(hillslope_id))
            .map(|contribution| contribution.total_detachment_kg)
            .sum::<f64>();
        let total_deposition_kg = dispatch_ids
            .contributor_hillslopes
            .iter()
            .filter_map(|hillslope_id| self.hillslope_contributions.get(hillslope_id))
            .map(|contribution| contribution.total_deposition_kg)
            .sum::<f64>();
        let sediment_yield_kg = terminal_sediment_yield_kg(
            &self.routed_channels,
            &self.routed_impoundments,
            &self.hillslope_contributions,
            &report.dispatch_report.steps,
            publication_channel_ids,
            self.routing_globals.dtchr_seconds,
        )?;

        Ok(WatershedPublicationFrame {
            sim_day_index: i32::try_from(report.dispatch_report.steps.len().max(1))
                .unwrap_or(i32::MAX),
            element_id: first_i32_or_default(publication_channel_ids, 1),
            channel_id: first_i32_or_default(publication_channel_ids, 1),
            runoff_volume_m3,
            peak_discharge_m3_s: first_channel_peak(&self.routed_channels, publication_channel_ids),
            sediment_yield_kg,
            channel_inflow_m3: Some(channel_inflow_m3),
            channel_outflow_m3: Some(channel_outflow_m3),
            channel_storage_m3: Some(channel_storage_m3),
            channel_baseflow_m3: Some(channel_baseflow_m3),
            channel_loss_m3: Some(channel_loss_m3),
            area_m2,
            runoff_mm,
            total_detachment_kg,
            total_deposition_kg,
            ..WatershedPublicationFrame::default()
        })
    }
}

fn terminal_sediment_yield_kg(
    routed_channels: &BTreeMap<u32, RoutedChannelState>,
    routed_impoundments: &BTreeMap<u32, RoutedImpoundmentState>,
    contributions: &BTreeMap<u32, HillslopeContribution>,
    steps: &[DispatchStep],
    terminal_channel_ids: &BTreeSet<u32>,
    dtchr_seconds: f64,
) -> Result<f64, WatershedNetworkFrameError> {
    let channel_contributors = channel_contributor_ancestry(steps)?;
    let mut terminal_mass_kg = 0.0_f64;
    for node_id in terminal_channel_ids {
        let state = routed_channels
            .get(node_id)
            .ok_or(WatershedNetworkFrameError::MissingRoutedChannelState { node_id: *node_id })?;
        let mass_kg = if state.interval_sediment_state.is_some() {
            state.sediment_yield_kg
        } else {
            let step = steps
                .iter()
                .find(|step| {
                    step.node.kind == TopologyNodeKind::Channel && step.node.id == *node_id
                })
                .ok_or(WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id: *node_id,
                    field: "dispatch_step",
                    value: 0.0,
                })?;
            let contributor_ids = channel_contributors.get(node_id).ok_or(
                WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id: *node_id,
                    field: "channel_contributor_ancestry",
                    value: 0.0,
                },
            )?;
            let duration_s = direct_terminal_sediment_duration_s(
                *node_id,
                step,
                contributor_ids,
                contributions,
                routed_channels,
                routed_impoundments,
                dtchr_seconds,
            )?;
            state.sediment_state.qsed_kg_s * duration_s
        };
        if !mass_kg.is_finite() || mass_kg < 0.0 {
            return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id: *node_id,
                field: "sediment_mass_kg",
                value: mass_kg,
            });
        }
        terminal_mass_kg += mass_kg;
    }
    if !terminal_mass_kg.is_finite() || terminal_mass_kg < 0.0 {
        return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
            node_id: 0,
            field: "terminal_sediment_mass_sum_kg",
            value: terminal_mass_kg,
        });
    }
    Ok(terminal_mass_kg)
}

fn channel_contributor_ancestry(
    steps: &[DispatchStep],
) -> Result<BTreeMap<u32, BTreeSet<u32>>, WatershedNetworkFrameError> {
    let mut channel_contributors = BTreeMap::<u32, BTreeSet<u32>>::new();
    for step in steps {
        if step.node.kind != TopologyNodeKind::Channel {
            continue;
        }
        let mut contributors = step
            .contributor_hillslopes
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for dependency in step
            .dependency_nodes
            .iter()
            .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
        {
            let inherited = channel_contributors.get(&dependency.id).ok_or(
                WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id: step.node.id,
                    field: "dependency_contributor_ancestry",
                    value: f64::from(dependency.id),
                },
            )?;
            contributors.extend(inherited.iter().copied());
        }
        channel_contributors.insert(step.node.id, contributors);
    }
    Ok(channel_contributors)
}

fn direct_terminal_sediment_duration_s(
    node_id: u32,
    step: &DispatchStep,
    contributor_ids: &BTreeSet<u32>,
    contributions: &BTreeMap<u32, HillslopeContribution>,
    routed_channels: &BTreeMap<u32, RoutedChannelState>,
    routed_impoundments: &BTreeMap<u32, RoutedImpoundmentState>,
    dtchr_seconds: f64,
) -> Result<f64, WatershedNetworkFrameError> {
    let event_duration_s = direct_terminal_event_duration_s(
        node_id,
        step,
        contributions,
        routed_channels,
        routed_impoundments,
        dtchr_seconds,
    )?;
    if !event_duration_s.is_finite() || event_duration_s < 0.0 {
        return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
            node_id,
            field: "event_duration_s",
            value: event_duration_s,
        });
    }

    let mut summed_hourly_sediment_kg = [0.0_f64; 24];
    let mut hourly_resolved = false;
    for hillslope_id in contributor_ids {
        let contribution = contributions.get(hillslope_id).ok_or(
            WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id,
                field: "hillslope_contribution",
                value: f64::from(*hillslope_id),
            },
        )?;
        match (
            contribution.hourly_runoff_volume_m3.len(),
            contribution.hourly_sediment_mass_kg.len(),
        ) {
            (24, 24) => hourly_resolved = true,
            (0, 0) => continue,
            (runoff_count, sediment_count) => {
                let observed_count = runoff_count.max(sediment_count);
                return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id,
                    field: "hourly_pair_cardinality",
                    value: f64::from(u32::try_from(observed_count).unwrap_or(u32::MAX)),
                });
            }
        }
        for (sum, value) in summed_hourly_sediment_kg
            .iter_mut()
            .zip(&contribution.hourly_sediment_mass_kg)
        {
            if !value.is_finite() || *value < 0.0 {
                return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id,
                    field: "hourly_sediment_mass_kg",
                    value: *value,
                });
            }
            *sum += *value;
        }
    }
    if !hourly_resolved {
        return Ok(event_duration_s);
    }

    let first = summed_hourly_sediment_kg
        .iter()
        .position(|value| *value > 0.0);
    let last = summed_hourly_sediment_kg
        .iter()
        .rposition(|value| *value > 0.0);
    let (Some(first), Some(last)) = (first, last) else {
        return Ok(event_duration_s);
    };
    let slot_count = u32::try_from(last - first + 1).map_err(|_| {
        WatershedNetworkFrameError::InvalidTerminalPublication {
            node_id,
            field: "hourly_sediment_active_slot_count",
            value: 24.0,
        }
    })?;
    Ok(f64::from(slot_count) * 3_600.0)
}

fn direct_terminal_event_duration_s(
    node_id: u32,
    step: &DispatchStep,
    contributions: &BTreeMap<u32, HillslopeContribution>,
    routed_channels: &BTreeMap<u32, RoutedChannelState>,
    routed_impoundments: &BTreeMap<u32, RoutedImpoundmentState>,
    dtchr_seconds: f64,
) -> Result<f64, WatershedNetworkFrameError> {
    if !dtchr_seconds.is_finite() || dtchr_seconds <= 0.0 {
        return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
            node_id,
            field: "dtchr_seconds",
            value: dtchr_seconds,
        });
    }
    let mut event_duration_s = dtchr_seconds;
    for hillslope_id in &step.contributor_hillslopes {
        let contribution = contributions.get(hillslope_id).ok_or(
            WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id,
                field: "hillslope_contribution",
                value: f64::from(*hillslope_id),
            },
        )?;
        if !contribution.duration_seconds.is_finite() || contribution.duration_seconds < 0.0 {
            return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id,
                field: "hillslope_duration_seconds",
                value: contribution.duration_seconds,
            });
        }
        event_duration_s = event_duration_s.max(contribution.duration_seconds);
    }
    for dependency in &step.dependency_nodes {
        let duration_s = match dependency.kind {
            TopologyNodeKind::Channel => {
                routed_channels
                    .get(&dependency.id)
                    .ok_or(WatershedNetworkFrameError::MissingRoutedChannelState {
                        node_id: dependency.id,
                    })?
                    .duration_seconds
            }
            TopologyNodeKind::Impoundment => {
                routed_impoundments
                    .get(&dependency.id)
                    .ok_or(WatershedNetworkFrameError::MissingRoutedImpoundmentState {
                        node_id: dependency.id,
                    })?
                    .duration_seconds
            }
            TopologyNodeKind::Hillslope => {
                return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    node_id,
                    field: "dependency_kind",
                    value: -1.0,
                });
            }
        };
        if !duration_s.is_finite() || duration_s < 0.0 {
            return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id,
                field: "dependency_duration_seconds",
                value: duration_s,
            });
        }
        event_duration_s = event_duration_s.max(duration_s);
    }
    if !event_duration_s.is_finite() || event_duration_s <= 0.0 {
        return Err(WatershedNetworkFrameError::InvalidTerminalPublication {
            node_id,
            field: "direct_event_duration_s",
            value: event_duration_s,
        });
    }
    Ok(event_duration_s)
}

fn sum_contributing_area_m2(
    contributions: &BTreeMap<u32, HillslopeContribution>,
    dispatch_ids: &TypedDispatchIds,
) -> Option<f64> {
    let mut area_m2 = 0.0_f64;
    let mut observed = false;
    for hillslope_id in &dispatch_ids.contributor_hillslopes {
        let contribution = contributions.get(hillslope_id)?;
        let area = contribution.area_m2?;
        if !area.is_finite() || area <= 0.0 {
            return None;
        }
        area_m2 += area;
        observed = true;
    }
    (observed && area_m2.is_finite() && area_m2 > 0.0).then_some(area_m2)
}

struct TypedDispatchIds {
    channel_ids: BTreeSet<u32>,
    outlet_channel_ids: BTreeSet<u32>,
    impoundment_ids: BTreeSet<u32>,
    contributor_hillslopes: BTreeSet<u32>,
}

fn collect_dispatch_ids_from_steps(steps: &[DispatchStep]) -> TypedDispatchIds {
    let mut channel_ids = BTreeSet::new();
    let mut dependency_channel_ids = BTreeSet::new();
    let mut impoundment_ids = BTreeSet::new();
    let mut contributor_hillslopes = BTreeSet::new();
    let consumed_impoundment_ids = steps
        .iter()
        .flat_map(|step| &step.dependency_nodes)
        .filter(|dependency| dependency.kind == TopologyNodeKind::Impoundment)
        .map(|dependency| dependency.id)
        .collect::<BTreeSet<_>>();

    for step in steps {
        match step.node.kind {
            TopologyNodeKind::Channel => {
                channel_ids.insert(step.node.id);
                dependency_channel_ids.extend(
                    step.dependency_nodes
                        .iter()
                        .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
                        .map(|dependency| dependency.id),
                );
            }
            TopologyNodeKind::Impoundment => {
                impoundment_ids.insert(step.node.id);
                if consumed_impoundment_ids.contains(&step.node.id) {
                    // INV-SYSTEM-036: a channel above an impoundment is
                    // internal when routing continues beyond that
                    // impoundment. Retain the channel-oriented proxy only
                    // when the impoundment itself is a topology terminal.
                    dependency_channel_ids.extend(
                        step.dependency_nodes
                            .iter()
                            .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
                            .map(|dependency| dependency.id),
                    );
                }
            }
            TopologyNodeKind::Hillslope => {}
        }
        contributor_hillslopes.extend(step.contributor_hillslopes.iter().copied());
    }

    let outlet_channel_ids = channel_ids
        .difference(&dependency_channel_ids)
        .copied()
        .collect();

    TypedDispatchIds {
        channel_ids,
        outlet_channel_ids,
        impoundment_ids,
        contributor_hillslopes,
    }
}

fn first_i32_or_default(values: &BTreeSet<u32>, default: i32) -> i32 {
    values
        .iter()
        .next()
        .copied()
        .and_then(|value| i32::try_from(value).ok())
        .unwrap_or(default)
}

fn first_channel_peak(
    routed_channels: &BTreeMap<u32, RoutedChannelState>,
    channel_ids: &BTreeSet<u32>,
) -> f64 {
    channel_ids
        .iter()
        .next()
        .and_then(|node_id| routed_channels.get(node_id))
        .map_or(0.0, |state| state.peak_discharge_m3_s)
}

fn build_routing_globals(
    chaninp: Option<&ChaninpFile>,
    channel: &WatershedChannelFile,
    default_dtchr_seconds: f64,
    default_ntchr: f64,
) -> Result<WatershedRoutingGlobals, WatershedNetworkFrameError> {
    if let Some(chaninp) = chaninp {
        if chaninp.ipeak != channel.ipeak {
            return Err(WatershedNetworkFrameError::ChaninpNotRuntimeReady {
                observed: chaninp.parse_outcome,
                chaninp_ipeak: chaninp.ipeak,
                channel_ipeak: channel.ipeak,
            });
        }
        match chaninp.parse_outcome {
            ChaninpParseOutcome::NotApplicable => {
                if channel.ipeak > 2 {
                    return Err(WatershedNetworkFrameError::ChaninpNotRuntimeReady {
                        observed: chaninp.parse_outcome,
                        chaninp_ipeak: chaninp.ipeak,
                        channel_ipeak: channel.ipeak,
                    });
                }
                let nchan = u32::try_from(channel.nchan).map_err(|_| {
                    WatershedNetworkFrameError::ChannelIdOutOfRange {
                        channel_id: channel.nchan,
                    }
                })?;
                return Ok(WatershedRoutingGlobals {
                    ipeak: channel.ipeak,
                    nchan,
                    dtchr_seconds: default_dtchr_seconds,
                    ntchr: default_ntchr,
                    nchnum: 0.0,
                    cbase: 0.0,
                    groundwater_baseflow: WatershedGroundwaterRoutingAuthority::disabled(),
                });
            }
            ChaninpParseOutcome::ParsedBranch
            | ChaninpParseOutcome::DefaultedCompat
            | ChaninpParseOutcome::OpenErrorCollapsedCompat => {}
        }
        let Some(options) = chaninp.options.as_ref() else {
            return Err(WatershedNetworkFrameError::MissingChaninpOptions);
        };
        let nchan = u32::try_from(chaninp.nchan).map_err(|_| {
            WatershedNetworkFrameError::ChannelIdOutOfRange {
                channel_id: chaninp.nchan,
            }
        })?;
        return Ok(WatershedRoutingGlobals {
            ipeak: chaninp.ipeak,
            nchan,
            dtchr_seconds: f64::from(options.dtchr_norm_s),
            ntchr: f64::from(options.ntchr),
            nchnum: f64::from(options.nchnum_norm),
            cbase: options.cbase_m3_s_m2,
            groundwater_baseflow: WatershedGroundwaterRoutingAuthority::disabled(),
        });
    }

    let nchan = u32::try_from(channel.nchan).map_err(|_| {
        WatershedNetworkFrameError::ChannelIdOutOfRange {
            channel_id: channel.nchan,
        }
    })?;
    Ok(WatershedRoutingGlobals {
        ipeak: channel.ipeak,
        nchan,
        dtchr_seconds: default_dtchr_seconds,
        ntchr: default_ntchr,
        nchnum: 0.0,
        cbase: 0.0,
        groundwater_baseflow: WatershedGroundwaterRoutingAuthority::disabled(),
    })
}

fn build_channel_controls(
    channel: &WatershedChannelFile,
    slope: &SlopeProfile,
) -> Result<BTreeMap<u32, WatershedChannelControlRecord>, WatershedNetworkFrameError> {
    let mut controls = BTreeMap::new();

    for definition in &channel.channels {
        let node_id = u32::try_from(definition.channel_id).map_err(|_| {
            WatershedNetworkFrameError::ChannelIdOutOfRange {
                channel_id: definition.channel_id,
            }
        })?;
        let Some(slope_index) = definition.channel_id.checked_sub(1) else {
            return Err(WatershedNetworkFrameError::MissingSlopeProfile {
                channel_id: node_id,
                slope_profile_count: slope.ofes.len(),
            });
        };
        let Some(ofe) = slope.ofes.get(slope_index) else {
            return Err(WatershedNetworkFrameError::MissingSlopeProfile {
                channel_id: node_id,
                slope_profile_count: slope.ofes.len(),
            });
        };

        let width_ft = ofe.fwidth * METERS_TO_FEET;
        let depth_ft = definition.chnedm * METERS_TO_FEET;
        let segment_points = ofe
            .points
            .iter()
            .map(|point| {
                let x_m = match ofe.distance_mode {
                    DistanceMode::Absolute => point.xinput,
                    DistanceMode::Normalized => point.xinput * ofe.slplen,
                };
                WatershedChannelSegmentPoint {
                    x_m,
                    slope: point.slpinp,
                    depth_a_ft: depth_ft,
                    depth_b_ft: depth_ft,
                    width_a_ft: width_ft,
                    width_b_ft: width_ft,
                }
            })
            .collect::<Vec<_>>();

        controls.insert(
            node_id,
            WatershedChannelControlRecord {
                node_id,
                ishape: definition.ishape,
                icntrl: definition.icntrl,
                ienslp: definition.ienslp,
                flgout: definition.flgout,
                chnz: definition.chnz,
                chnnbr: definition.chnnbr,
                chnn: definition.chnn,
                chnk: definition.chnk,
                chntcr: definition.chntcr,
                chnedm: definition.chnedm,
                chneds: definition.chneds,
                ctlslp: definition.ctlslp_effective,
                ctlz: definition.ctlz_effective,
                ctln: definition.ctln_effective,
                rating_curve: definition
                    .rating_curve
                    .as_ref()
                    .map(WatershedChannelRatingCurveControl::from),
                segment_points,
                ws20_case12_enabled: false,
                ws21_case34_enabled: false,
                crfrac: Vec::new(),
            },
        );
    }

    Ok(controls)
}

fn build_impoundment_controls(
    impoundment: &WatershedImpoundmentFile,
) -> Result<BTreeMap<u32, WatershedImpoundmentControlRecord>, WatershedNetworkFrameError> {
    let mut controls = BTreeMap::new();

    for (index, record) in impoundment.items.iter().enumerate() {
        let node_id = u32::try_from(index + 1).map_err(|_| {
            WatershedNetworkFrameError::ImpoundmentIdOutOfRange {
                impoundment_index: index,
            }
        })?;
        controls.insert(
            node_id,
            WatershedImpoundmentControlRecord {
                node_id,
                h: record.h,
                hfull: record.hfull,
                deltat: record.deltat,
                qinf: record.qinf,
                source_record: record.clone(),
            },
        );
    }

    Ok(controls)
}

#[cfg(test)]
mod network_frame_tests {
    use openwepp_input_contract::parsers::{
        chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
        slope::{SlopeParserOptions, parse_slope_str},
        watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
        watershed_impoundment::{
            WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
        },
    };
    use openwepp_sim_contract::status::{SimulationPhase, SimulationStatus};
    use openwepp_topology::{TopologyNodeKey, parse_topology_fixture_str};

    use super::*;
    use crate::lib_mod::types::{WatershedDispatchReport, WatershedFrameExecutionReport};

    const TEST_TOPOLOGY: &str = r"
HILLSLOPES 2
CHANNELS 1
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 1 0 0 I 0 0 0
";
    const TEST_CHANINP: &str =
        include_str!("../../../../tests/fixtures/infile/chaninp/strict_valid.chaninp");
    const TEST_SLOPE: &str =
        include_str!("../../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const TEST_CHANNEL: &str = include_str!(
        "../../../../tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn"
    );
    const TEST_IMPOUNDMENT: &str = include_str!(
        "../../../../tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"
    );

    fn ok_status() -> SimulationStatus {
        SimulationStatus::ok(SimulationPhase::WatershedKernel, "WSHED-W11D-TEST-OK")
            .expect("test status should build")
    }

    fn channel_step(
        node_id: u32,
        dependency_nodes: Vec<TopologyNodeKey>,
        contributor_hillslopes: Vec<u32>,
    ) -> DispatchStep {
        DispatchStep {
            sequence_index: usize::try_from(node_id).expect("test node id fits usize"),
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, node_id),
            dependency_nodes,
            contributor_hillslopes,
            status: ok_status(),
        }
    }

    fn direct_channel_state(node_id: u32, qsed_kg_s: f64) -> RoutedChannelState {
        RoutedChannelState {
            node_id,
            runoff_volume_m3: 0.0,
            channel_inflow_m3: 0.0,
            channel_outflow_m3: 0.0,
            channel_storage_m3: 0.0,
            peak_discharge_m3_s: 0.0,
            duration_seconds: 600.0,
            channel_baseflow_m3: 0.0,
            channel_loss_m3: 0.0,
            groundwater_deep_seepage_m3: 0.0,
            sediment_yield_kg: qsed_kg_s,
            wave_state: None,
            interval_water_state: None,
            sediment_state: RoutedChannelSedimentState {
                qsed_kg_s,
                ..RoutedChannelSedimentState::default()
            },
            interval_sediment_state: None,
        }
    }

    fn hourly_contribution(
        hillslope_id: u32,
        hourly_sediment_mass_kg: Vec<f64>,
    ) -> HillslopeContribution {
        HillslopeContribution {
            hillslope_id,
            area_m2: Some(1.0),
            peak_runoff_m3_s: 0.0,
            duration_seconds: 0.0,
            generated_baseflow_m3: 0.0,
            groundwater_deep_seepage_m3: 0.0,
            total_detachment_kg: hourly_sediment_mass_kg.iter().sum(),
            total_deposition_kg: 0.0,
            sediment_concentration_kg_m3: vec![0.0],
            particle_diameter_m: vec![0.001],
            particle_flow_fraction: vec![1.0],
            hourly_runoff_volume_m3: vec![0.0; 24],
            hourly_sediment_mass_kg,
        }
    }

    fn parsed_test_frame() -> WatershedNetworkFrame {
        let topology = parse_topology_fixture_str(TEST_TOPOLOGY).expect("valid test topology");
        let channel =
            parse_watershed_channel_from_str(TEST_CHANNEL, WatershedChannelParseOptions::default())
                .expect("valid channel fixture");
        let chaninp = parse_chaninp_from_str(
            TEST_CHANINP,
            ChaninpParseOptions::strict(channel.ipeak, 2),
            &BTreeSet::from([4, 5]),
        )
        .expect("valid chaninp fixture");
        let slope =
            parse_slope_str(TEST_SLOPE, SlopeParserOptions::strict()).expect("valid slope fixture");
        let impoundment = parse_watershed_impoundment_from_str(
            TEST_IMPOUNDMENT,
            WatershedImpoundmentParseOptions::strict(),
        )
        .expect("valid impoundment fixture");
        WatershedNetworkFrame::from_parsed_inputs(
            topology,
            Some(chaninp),
            channel,
            slope,
            impoundment,
            3_600.0,
            24.0,
        )
        .expect("valid parsed test frame")
    }

    fn execution_report(steps: Vec<DispatchStep>) -> WatershedFrameExecutionReport {
        WatershedFrameExecutionReport {
            dispatch_report: WatershedDispatchReport {
                precondition_status: ok_status(),
                dispatch_status: ok_status(),
                steps,
                diagnostics: Vec::new(),
            },
            step_reports: Vec::new(),
        }
    }

    #[test]
    fn wshedw11d_terminal_selector_and_extensive_sediment_sum_exclude_internal_channel() {
        let steps = vec![
            channel_step(1, Vec::new(), vec![1]),
            channel_step(
                2,
                vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 1)],
                Vec::new(),
            ),
            channel_step(3, Vec::new(), vec![3]),
            DispatchStep {
                sequence_index: 4,
                node: TopologyNodeKey::new(TopologyNodeKind::Impoundment, 1),
                dependency_nodes: vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 3)],
                contributor_hillslopes: Vec::new(),
                status: ok_status(),
            },
        ];
        let dispatch_ids = collect_dispatch_ids_from_steps(&steps);
        assert_eq!(dispatch_ids.channel_ids, BTreeSet::from([1, 2, 3]));
        assert_eq!(dispatch_ids.outlet_channel_ids, BTreeSet::from([2, 3]));

        let mut first_hourly = vec![0.0; 24];
        first_hourly[10] = 240.0;
        let mut third_hourly = vec![0.0; 24];
        third_hourly[8] = 60.0;
        third_hourly[9] = 60.0;
        let contributions = BTreeMap::from([
            (1, hourly_contribution(1, first_hourly)),
            (3, hourly_contribution(3, third_hourly)),
        ]);
        let routed_channels = BTreeMap::from([
            (1, direct_channel_state(1, 240.0 / 3_600.0)),
            (2, direct_channel_state(2, 240.0 / 3_600.0)),
            (3, direct_channel_state(3, 120.0 / 7_200.0)),
        ]);
        let mass_kg = terminal_sediment_yield_kg(
            &routed_channels,
            &BTreeMap::new(),
            &contributions,
            &steps,
            &dispatch_ids.outlet_channel_ids,
            600.0,
        )
        .expect("terminal extensive sediment reduction should close");
        assert!((mass_kg - 360.0).abs() <= 1.0e-12);
    }

    #[test]
    fn wshedw11d_terminal_selector_follows_serial_impoundment_path() {
        let steps = vec![
            channel_step(1, Vec::new(), vec![1]),
            DispatchStep {
                sequence_index: 1,
                node: TopologyNodeKey::new(TopologyNodeKind::Impoundment, 9),
                dependency_nodes: vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 1)],
                contributor_hillslopes: Vec::new(),
                status: ok_status(),
            },
            channel_step(
                2,
                vec![TopologyNodeKey::new(TopologyNodeKind::Impoundment, 9)],
                vec![2],
            ),
        ];
        let dispatch_ids = collect_dispatch_ids_from_steps(&steps);
        assert_eq!(dispatch_ids.channel_ids, BTreeSet::from([1, 2]));
        assert_eq!(dispatch_ids.outlet_channel_ids, BTreeSet::from([2]));

        let mut upstream_hourly = vec![0.0; 24];
        upstream_hourly[4] = 240.0;
        let mut downstream_hourly = vec![0.0; 24];
        downstream_hourly[10] = 60.0;
        downstream_hourly[11] = 60.0;
        let contributions = BTreeMap::from([
            (1, hourly_contribution(1, upstream_hourly)),
            (2, hourly_contribution(2, downstream_hourly)),
        ]);
        let routed_channels = BTreeMap::from([
            (1, direct_channel_state(1, 240.0 / 3_600.0)),
            (2, direct_channel_state(2, 120.0 / 7_200.0)),
        ]);
        let routed_impoundments = BTreeMap::from([(
            9,
            RoutedImpoundmentState {
                node_id: 9,
                outflow_volume_m3: 10.0,
                outflow_rate_m3_s: 0.1,
                duration_seconds: 100.0,
                hnext_m: 0.2,
            },
        )]);
        let mass_kg = terminal_sediment_yield_kg(
            &routed_channels,
            &routed_impoundments,
            &contributions,
            &steps,
            &dispatch_ids.outlet_channel_ids,
            600.0,
        )
        .expect("post-impoundment terminal mass should publish without upstream aliasing");
        assert!((mass_kg - 120.0).abs() <= 1.0e-12);
    }

    #[test]
    fn duration_contract_valid_maxima_and_dtchr_floor() {
        let dtchr_only_duration_s = direct_terminal_event_duration_s(
            3,
            &channel_step(3, Vec::new(), Vec::new()),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            600.0,
        )
        .expect("dtchr-only duration");
        assert!((dtchr_only_duration_s - 600.0).abs() <= f64::EPSILON);

        for (step, contributions, channels, impoundments, expected) in [
            (
                channel_step(4, Vec::new(), vec![1]),
                BTreeMap::from([(1, hourly_contribution(1, vec![0.0; 24]))]),
                BTreeMap::new(),
                BTreeMap::new(),
                1_800.0,
            ),
            (
                channel_step(
                    5,
                    vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 7)],
                    Vec::new(),
                ),
                BTreeMap::new(),
                BTreeMap::from([(7, direct_channel_state(7, 0.0))]),
                BTreeMap::new(),
                1_200.0,
            ),
            (
                channel_step(
                    6,
                    vec![TopologyNodeKey::new(TopologyNodeKind::Impoundment, 9)],
                    Vec::new(),
                ),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::from([(
                    9,
                    RoutedImpoundmentState {
                        node_id: 9,
                        outflow_volume_m3: 0.0,
                        outflow_rate_m3_s: 0.0,
                        duration_seconds: 2_400.0,
                        hnext_m: 0.0,
                    },
                )]),
                2_400.0,
            ),
        ] {
            let mut contributions = contributions;
            let mut channels = channels;
            if let Some(contribution) = contributions.get_mut(&1) {
                contribution.duration_seconds = 1_800.0;
            }
            if let Some(channel) = channels.get_mut(&7) {
                channel.duration_seconds = 1_200.0;
            }
            let duration_s = direct_terminal_event_duration_s(
                step.node.id,
                &step,
                &contributions,
                &channels,
                &impoundments,
                600.0,
            )
            .expect("each duration source can dominate independently");
            assert!((duration_s - expected).abs() <= f64::EPSILON);
        }
    }

    #[test]
    fn duration_contract_reachable_input_guards() {
        let empty = channel_step(2, Vec::new(), Vec::new());
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
            assert!(matches!(
                direct_terminal_event_duration_s(
                    2,
                    &empty,
                    &BTreeMap::new(),
                    &BTreeMap::new(),
                    &BTreeMap::new(),
                    value,
                ),
                Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    field: "dtchr_seconds",
                    ..
                })
            ));
        }
        let contributor_step = channel_step(2, Vec::new(), vec![1]);
        assert!(matches!(
            direct_terminal_event_duration_s(
                2,
                &contributor_step,
                &BTreeMap::new(),
                &BTreeMap::new(),
                &BTreeMap::new(),
                600.0,
            ),
            Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                field: "hillslope_contribution",
                ..
            })
        ));
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            let mut contribution = hourly_contribution(1, vec![0.0; 24]);
            contribution.duration_seconds = value;
            assert!(matches!(
                direct_terminal_event_duration_s(
                    2,
                    &contributor_step,
                    &BTreeMap::from([(1, contribution)]),
                    &BTreeMap::new(),
                    &BTreeMap::new(),
                    600.0,
                ),
                Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    field: "hillslope_duration_seconds",
                    ..
                })
            ));
        }
    }

    #[test]
    fn duration_contract_reachable_dependency_guards() {
        for kind in [TopologyNodeKind::Channel, TopologyNodeKind::Impoundment] {
            let step = channel_step(2, vec![TopologyNodeKey::new(kind, 7)], Vec::new());
            let error = direct_terminal_event_duration_s(
                2,
                &step,
                &BTreeMap::new(),
                &BTreeMap::new(),
                &BTreeMap::new(),
                600.0,
            );
            assert!(matches!(
                (kind, error),
                (
                    TopologyNodeKind::Channel,
                    Err(WatershedNetworkFrameError::MissingRoutedChannelState { .. })
                ) | (
                    TopologyNodeKind::Impoundment,
                    Err(WatershedNetworkFrameError::MissingRoutedImpoundmentState { .. })
                )
            ));
        }
        let channel_dependency = channel_step(
            2,
            vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 7)],
            Vec::new(),
        );
        let impoundment_dependency = channel_step(
            2,
            vec![TopologyNodeKey::new(TopologyNodeKind::Impoundment, 9)],
            Vec::new(),
        );
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            let mut channel = direct_channel_state(7, 0.0);
            channel.duration_seconds = value;
            assert!(matches!(
                direct_terminal_event_duration_s(
                    2,
                    &channel_dependency,
                    &BTreeMap::new(),
                    &BTreeMap::from([(7, channel)]),
                    &BTreeMap::new(),
                    600.0,
                ),
                Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    field: "dependency_duration_seconds",
                    ..
                })
            ));
            let impoundment = RoutedImpoundmentState {
                node_id: 9,
                outflow_volume_m3: 0.0,
                outflow_rate_m3_s: 0.0,
                duration_seconds: value,
                hnext_m: 0.0,
            };
            assert!(matches!(
                direct_terminal_event_duration_s(
                    2,
                    &impoundment_dependency,
                    &BTreeMap::new(),
                    &BTreeMap::new(),
                    &BTreeMap::from([(9, impoundment)]),
                    600.0,
                ),
                Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    field: "dependency_duration_seconds",
                    ..
                })
            ));
        }
        let hillslope_dependency = channel_step(
            2,
            vec![TopologyNodeKey::new(TopologyNodeKind::Hillslope, 1)],
            Vec::new(),
        );
        assert!(matches!(
            direct_terminal_event_duration_s(
                2,
                &hillslope_dependency,
                &BTreeMap::new(),
                &BTreeMap::new(),
                &BTreeMap::new(),
                600.0,
            ),
            Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                field: "dependency_kind",
                ..
            })
        ));
    }

    #[test]
    fn sediment_duration_contract_covers_hourly_fallback_and_guards() {
        let step = channel_step(2, Vec::new(), vec![1]);
        let mut contribution = hourly_contribution(1, vec![0.0; 24]);
        contribution.duration_seconds = 1_800.0;
        contribution.hourly_sediment_mass_kg[3] = 1.0;
        contribution.hourly_sediment_mass_kg[5] = 1.0;
        let contributions = BTreeMap::from([(1, contribution.clone())]);
        let hourly_duration_s = direct_terminal_sediment_duration_s(
            2,
            &step,
            &BTreeSet::from([1]),
            &contributions,
            &BTreeMap::new(),
            &BTreeMap::new(),
            600.0,
        )
        .expect("hourly active span");
        assert!((hourly_duration_s - 10_800.0).abs() <= f64::EPSILON);
        contribution.hourly_sediment_mass_kg.fill(0.0);
        let fallback_duration_s = direct_terminal_sediment_duration_s(
            2,
            &step,
            &BTreeSet::from([1]),
            &BTreeMap::from([(1, contribution.clone())]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            600.0,
        )
        .expect("zero hourly sediment falls back to event duration");
        assert!((fallback_duration_s - 1_800.0).abs() <= f64::EPSILON);
        contribution.hourly_runoff_volume_m3.pop();
        assert!(matches!(
            direct_terminal_sediment_duration_s(
                2,
                &step,
                &BTreeSet::from([1]),
                &BTreeMap::from([(1, contribution.clone())]),
                &BTreeMap::new(),
                &BTreeMap::new(),
                600.0,
            ),
            Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                field: "hourly_pair_cardinality",
                ..
            })
        ));
        contribution.hourly_runoff_volume_m3.push(0.0);
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            let mut invalid = contribution.clone();
            invalid.hourly_sediment_mass_kg[0] = value;
            assert!(matches!(
                direct_terminal_sediment_duration_s(
                    2,
                    &step,
                    &BTreeSet::from([1]),
                    &BTreeMap::from([(1, invalid)]),
                    &BTreeMap::new(),
                    &BTreeMap::new(),
                    600.0,
                ),
                Err(WatershedNetworkFrameError::InvalidTerminalPublication {
                    field: "hourly_sediment_mass_kg",
                    ..
                })
            ));
        }
    }

    #[test]
    fn terminal_publication_contract_covers_wrapper_success_and_missing_states() {
        let channel = channel_step(1, Vec::new(), vec![1]);
        let impoundment = DispatchStep {
            sequence_index: 2,
            node: TopologyNodeKey::new(TopologyNodeKind::Impoundment, 1),
            dependency_nodes: vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 1)],
            contributor_hillslopes: Vec::new(),
            status: ok_status(),
        };
        let report = execution_report(vec![channel, impoundment]);
        let mut frame = parsed_test_frame();
        let _topology = frame.topology();
        frame.add_hillslope_contribution(hourly_contribution(1, vec![0.0; 24]));
        frame.configure_groundwater_baseflow_routing(
            WatershedGroundwaterRoutingAuthority::linear_reservoir(2.0)
                .expect("valid groundwater authority"),
        );
        frame.set_channel_tillage_day_state(1, ChannelTillageDayState::PrimaryTillage);
        frame.record_routed_channel_state(direct_channel_state(1, 0.0));
        frame.record_routed_impoundment_state(RoutedImpoundmentState {
            node_id: 1,
            outflow_volume_m3: 0.0,
            outflow_rate_m3_s: 0.0,
            duration_seconds: 600.0,
            hnext_m: 0.0,
        });
        let published = frame
            .publish_typed_routing_report(&report)
            .expect("complete routed state publishes");
        assert_eq!(published.element_id, 1);
        assert_eq!(frame.publication_frame, Some(published));

        let mut missing_channel = parsed_test_frame();
        assert!(matches!(
            missing_channel.publish_typed_routing_report(&report),
            Err(WatershedNetworkFrameError::MissingRoutedChannelState { node_id: 1 })
        ));
        missing_channel.record_routed_channel_state(direct_channel_state(1, 0.0));
        assert!(matches!(
            missing_channel.publish_typed_routing_report(&report),
            Err(WatershedNetworkFrameError::MissingRoutedImpoundmentState { node_id: 1 })
        ));
    }

    #[test]
    fn contributing_area_contract_covers_complete_and_rejected_operands() {
        let ids = TypedDispatchIds {
            channel_ids: BTreeSet::new(),
            outlet_channel_ids: BTreeSet::new(),
            impoundment_ids: BTreeSet::new(),
            contributor_hillslopes: BTreeSet::from([1, 2]),
        };
        let mut first = hourly_contribution(1, vec![0.0; 24]);
        first.area_m2 = Some(2.0);
        let mut second = hourly_contribution(2, vec![0.0; 24]);
        second.area_m2 = Some(3.0);
        let mut contributions = BTreeMap::from([(1, first), (2, second)]);
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), Some(5.0));
        contributions.get_mut(&2).expect("second").area_m2 = None;
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), None);
        contributions.get_mut(&2).expect("second").area_m2 = Some(-1.0);
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), None);
        contributions.get_mut(&2).expect("second").area_m2 = Some(0.0);
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), None);
        contributions.get_mut(&2).expect("second").area_m2 = Some(f64::NAN);
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), None);
        contributions.get_mut(&2).expect("second").area_m2 = Some(f64::INFINITY);
        assert_eq!(sum_contributing_area_m2(&contributions, &ids), None);
        assert_eq!(sum_contributing_area_m2(&BTreeMap::new(), &ids), None);
        let empty_ids = TypedDispatchIds {
            channel_ids: BTreeSet::new(),
            outlet_channel_ids: BTreeSet::new(),
            impoundment_ids: BTreeSet::new(),
            contributor_hillslopes: BTreeSet::new(),
        };
        assert_eq!(sum_contributing_area_m2(&BTreeMap::new(), &empty_ids), None);
    }

    #[test]
    fn routing_global_contract_covers_authorized_branches_and_failures() {
        let frame = parsed_test_frame();
        let channel = frame.channel_source.clone();
        let chaninp = frame.chaninp_source.clone().expect("parsed chaninp");
        let parsed = build_routing_globals(Some(&chaninp), &channel, 3_600.0, 24.0)
            .expect("parsed routing globals");
        assert!((parsed.dtchr_seconds - 600.0).abs() <= f64::EPSILON);
        let absent = build_routing_globals(None, &channel, 3_600.0, 24.0)
            .expect("absent sidecar uses explicit defaults");
        assert!((absent.dtchr_seconds - 3_600.0).abs() <= f64::EPSILON);

        let mut mismatch = chaninp.clone();
        mismatch.ipeak += 1;
        assert!(matches!(
            build_routing_globals(Some(&mismatch), &channel, 3_600.0, 24.0),
            Err(WatershedNetworkFrameError::ChaninpNotRuntimeReady { .. })
        ));
        let mut no_options = chaninp.clone();
        no_options.options = None;
        assert!(matches!(
            build_routing_globals(Some(&no_options), &channel, 3_600.0, 24.0),
            Err(WatershedNetworkFrameError::MissingChaninpOptions)
        ));
        let mut not_applicable = chaninp.clone();
        not_applicable.parse_outcome = ChaninpParseOutcome::NotApplicable;
        assert!(matches!(
            build_routing_globals(Some(&not_applicable), &channel, 3_600.0, 24.0),
            Err(WatershedNetworkFrameError::ChaninpNotRuntimeReady { .. })
        ));
        let mut low_peak_channel = channel.clone();
        low_peak_channel.ipeak = 2;
        not_applicable.ipeak = 2;
        let defaults =
            build_routing_globals(Some(&not_applicable), &low_peak_channel, 3_600.0, 24.0)
                .expect("not-applicable low-peak branch");
        assert!(defaults.nchnum.abs() <= f64::EPSILON);
        let mut oversized_channel = channel.clone();
        oversized_channel.nchan = usize::MAX;
        assert!(matches!(
            build_routing_globals(None, &oversized_channel, 3_600.0, 24.0),
            Err(WatershedNetworkFrameError::ChannelIdOutOfRange { .. })
        ));
        let mut oversized_chaninp = chaninp;
        oversized_chaninp.nchan = usize::MAX;
        assert!(matches!(
            build_routing_globals(Some(&oversized_chaninp), &channel, 3_600.0, 24.0),
            Err(WatershedNetworkFrameError::ChannelIdOutOfRange { .. })
        ));
    }

    #[test]
    fn small_surface_contracts_cover_conversions_helpers_and_error_sources() {
        let disabled = WatershedGroundwaterRoutingAuthority::disabled();
        assert!(!disabled.is_enabled());
        assert!(
            WatershedGroundwaterRoutingAuthority::linear_reservoir(0.0)
                .expect("zero threshold is valid")
                .is_enabled()
        );
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            assert!(matches!(
                WatershedGroundwaterRoutingAuthority::linear_reservoir(value),
                Err(WatershedNetworkFrameError::InvalidGroundwaterAuthority { .. })
            ));
        }
        let curve = ChannelRatingCurve {
            rccoef: 1.0,
            rcexp: 2.0,
            rcoset: 3.0,
        };
        assert_eq!(
            WatershedChannelRatingCurveControl::from(&curve),
            WatershedChannelRatingCurveControl {
                rccoef: 1.0,
                rcexp: 2.0,
                rcoset: 3.0,
            }
        );
        assert_eq!(first_i32_or_default(&BTreeSet::new(), 9), 9);
        assert_eq!(first_i32_or_default(&BTreeSet::from([4]), 9), 4);
        assert_eq!(first_i32_or_default(&BTreeSet::from([u32::MAX]), 9), 9);
        assert!(first_channel_peak(&BTreeMap::new(), &BTreeSet::new()).abs() <= f64::EPSILON);
        let mut state = direct_channel_state(4, 0.0);
        state.peak_discharge_m3_s = 2.5;
        let peak = first_channel_peak(&BTreeMap::from([(4, state)]), &BTreeSet::from([4]));
        assert!((peak - 2.5).abs() <= f64::EPSILON);
        assert!(WatershedPublicationFrame::default().runoff_volume_m3.abs() <= f64::EPSILON);

        let runtime = WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: "h".to_owned(),
            value: f64::NAN,
        };
        let wrapped = WatershedNetworkFrameError::from(runtime);
        assert!(wrapped.source().is_some());
        assert!(
            WatershedNetworkFrameError::MissingChaninpOptions
                .source()
                .is_none()
        );
        assert!(wrapped.to_string().contains("WSHEDFRAME-E-006"));
    }
}
