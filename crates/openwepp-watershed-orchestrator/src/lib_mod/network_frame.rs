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
            | Self::InvalidGroundwaterAuthority { .. } => None,
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
    pub sediment_state: RoutedChannelSedimentState,
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

        let runoff_volume_m3 = dispatch_ids
            .channel_ids
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
        let channel_baseflow_m3 = dispatch_ids
            .channel_ids
            .iter()
            .filter_map(|node_id| self.routed_channels.get(node_id))
            .map(|state| state.channel_baseflow_m3)
            .sum::<f64>();
        let channel_loss_m3 = dispatch_ids
            .channel_ids
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

        Ok(WatershedPublicationFrame {
            sim_day_index: i32::try_from(report.dispatch_report.steps.len().max(1))
                .unwrap_or(i32::MAX),
            element_id: first_i32_or_default(&dispatch_ids.channel_ids, 1),
            channel_id: first_i32_or_default(&dispatch_ids.channel_ids, 1),
            runoff_volume_m3,
            peak_discharge_m3_s: first_channel_peak(
                &self.routed_channels,
                &dispatch_ids.channel_ids,
            ),
            sediment_yield_kg: dispatch_ids
                .channel_ids
                .iter()
                .filter_map(|node_id| self.routed_channels.get(node_id))
                .map(|state| state.sediment_yield_kg)
                .sum::<f64>(),
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
    impoundment_ids: BTreeSet<u32>,
    contributor_hillslopes: BTreeSet<u32>,
}

fn collect_dispatch_ids_from_steps(steps: &[DispatchStep]) -> TypedDispatchIds {
    let mut channel_ids = BTreeSet::new();
    let mut impoundment_ids = BTreeSet::new();
    let mut contributor_hillslopes = BTreeSet::new();

    for step in steps {
        match step.node.kind {
            TopologyNodeKind::Channel => {
                channel_ids.insert(step.node.id);
            }
            TopologyNodeKind::Impoundment => {
                impoundment_ids.insert(step.node.id);
            }
            TopologyNodeKind::Hillslope => {}
        }
        contributor_hillslopes.extend(step.contributor_hillslopes.iter().copied());
    }

    TypedDispatchIds {
        channel_ids,
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
