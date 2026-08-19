use openwepp_hillslope_orchestrator::{
    DirectLaneTransferLedger, DirectRunTransferDownstreamOperands, DirectTransferBuffers,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HexF64;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransferRestartError {
    #[error("{field} must be finite")]
    NonFinite { field: &'static str },
    #[error("{field} must be finite and nonnegative")]
    Nonnegative { field: &'static str },
    #[error("lane count does not fit the runtime usize")]
    LaneCountWidth,
}

fn finite(field: &'static str, value: &HexF64) -> Result<f64, TransferRestartError> {
    let value = value.to_f64();
    value
        .is_finite()
        .then_some(value)
        .ok_or(TransferRestartError::NonFinite { field })
}
fn nonnegative(field: &'static str, value: &HexF64) -> Result<f64, TransferRestartError> {
    let value = finite(field, value)?;
    (value >= 0.0)
        .then_some(value)
        .ok_or(TransferRestartError::Nonnegative { field })
}
fn encode24(values: [f64; 24]) -> [HexF64; 24] {
    values.map(HexF64::from_f64)
}
fn decode24(field: &'static str, values: &[HexF64; 24]) -> Result<[f64; 24], TransferRestartError> {
    let decoded: Result<Vec<_>, _> = values.iter().map(|v| nonnegative(field, v)).collect();
    decoded?
        .try_into()
        .map_err(|_| unreachable!("fixed array length"))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectTransferBuffersRestartV1 {
    pub surface_carry_m: [HexF64; 24],
    pub surface_hourly_weights: [HexF64; 24],
    pub lateral_carry_m: [HexF64; 24],
    pub upstream_flow_m: HexF64,
    pub subsurface_input_m: HexF64,
}
impl DirectTransferBuffersRestartV1 {
    pub fn project(value: &DirectTransferBuffers) -> Self {
        let DirectTransferBuffers {
            surface_carry_m,
            surface_hourly_weights,
            lateral_carry_m,
            upstream_flow_m,
            subsurface_input_m,
        } = *value;
        Self {
            surface_carry_m: encode24(surface_carry_m),
            surface_hourly_weights: encode24(surface_hourly_weights),
            lateral_carry_m: encode24(lateral_carry_m),
            upstream_flow_m: HexF64::from_f64(upstream_flow_m),
            subsurface_input_m: HexF64::from_f64(subsurface_input_m),
        }
    }
    pub fn restore(&self) -> Result<DirectTransferBuffers, TransferRestartError> {
        Ok(DirectTransferBuffers {
            surface_carry_m: decode24("surface_carry_m", &self.surface_carry_m)?,
            surface_hourly_weights: decode24(
                "surface_hourly_weights",
                &self.surface_hourly_weights,
            )?,
            lateral_carry_m: decode24("lateral_carry_m", &self.lateral_carry_m)?,
            upstream_flow_m: nonnegative("upstream_flow_m", &self.upstream_flow_m)?,
            subsurface_input_m: nonnegative("subsurface_input_m", &self.subsurface_input_m)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectLaneTransferLedgerRestartV1 {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: HexF64,
    pub area_m2: HexF64,
    pub outgoing_surface_m: HexF64,
    pub outgoing_lateral_m: HexF64,
    pub received_surface_m: HexF64,
    pub received_lateral_m: HexF64,
    pub net_transfer_m: HexF64,
}
impl DirectLaneTransferLedgerRestartV1 {
    pub fn project(value: &DirectLaneTransferLedger) -> Self {
        let DirectLaneTransferLedger {
            lane_id,
            upstream_lane_id,
            downstream_lane_id,
            upstream_area_ratio,
            area_m2,
            outgoing_surface_m,
            outgoing_lateral_m,
            received_surface_m,
            received_lateral_m,
            net_transfer_m,
        } = *value;
        Self {
            lane_id,
            upstream_lane_id,
            downstream_lane_id,
            upstream_area_ratio: HexF64::from_f64(upstream_area_ratio),
            area_m2: HexF64::from_f64(area_m2),
            outgoing_surface_m: HexF64::from_f64(outgoing_surface_m),
            outgoing_lateral_m: HexF64::from_f64(outgoing_lateral_m),
            received_surface_m: HexF64::from_f64(received_surface_m),
            received_lateral_m: HexF64::from_f64(received_lateral_m),
            net_transfer_m: HexF64::from_f64(net_transfer_m),
        }
    }
    pub fn restore(&self) -> Result<DirectLaneTransferLedger, TransferRestartError> {
        Ok(DirectLaneTransferLedger {
            lane_id: self.lane_id,
            upstream_lane_id: self.upstream_lane_id,
            downstream_lane_id: self.downstream_lane_id,
            upstream_area_ratio: nonnegative("upstream_area_ratio", &self.upstream_area_ratio)?,
            area_m2: nonnegative("area_m2", &self.area_m2)?,
            outgoing_surface_m: nonnegative("outgoing_surface_m", &self.outgoing_surface_m)?,
            outgoing_lateral_m: nonnegative("outgoing_lateral_m", &self.outgoing_lateral_m)?,
            received_surface_m: nonnegative("received_surface_m", &self.received_surface_m)?,
            received_lateral_m: nonnegative("received_lateral_m", &self.received_lateral_m)?,
            net_transfer_m: finite("net_transfer_m", &self.net_transfer_m)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectRunTransferDownstreamOperandsRestartV1 {
    pub lane_count: u64,
    pub outlet_lane_id: u32,
    pub total_outgoing_surface_m: HexF64,
    pub total_outgoing_lateral_m: HexF64,
    pub total_received_surface_m: HexF64,
    pub total_received_lateral_m: HexF64,
    pub total_net_transfer_m: HexF64,
}
impl DirectRunTransferDownstreamOperandsRestartV1 {
    pub fn project(value: &DirectRunTransferDownstreamOperands) -> Self {
        let DirectRunTransferDownstreamOperands {
            lane_count,
            outlet_lane_id,
            total_outgoing_surface_m,
            total_outgoing_lateral_m,
            total_received_surface_m,
            total_received_lateral_m,
            total_net_transfer_m,
        } = *value;
        Self {
            lane_count: lane_count as u64,
            outlet_lane_id,
            total_outgoing_surface_m: HexF64::from_f64(total_outgoing_surface_m),
            total_outgoing_lateral_m: HexF64::from_f64(total_outgoing_lateral_m),
            total_received_surface_m: HexF64::from_f64(total_received_surface_m),
            total_received_lateral_m: HexF64::from_f64(total_received_lateral_m),
            total_net_transfer_m: HexF64::from_f64(total_net_transfer_m),
        }
    }
    pub fn restore(&self) -> Result<DirectRunTransferDownstreamOperands, TransferRestartError> {
        Ok(DirectRunTransferDownstreamOperands {
            lane_count: usize::try_from(self.lane_count)
                .map_err(|_| TransferRestartError::LaneCountWidth)?,
            outlet_lane_id: self.outlet_lane_id,
            total_outgoing_surface_m: nonnegative(
                "total_outgoing_surface_m",
                &self.total_outgoing_surface_m,
            )?,
            total_outgoing_lateral_m: nonnegative(
                "total_outgoing_lateral_m",
                &self.total_outgoing_lateral_m,
            )?,
            total_received_surface_m: nonnegative(
                "total_received_surface_m",
                &self.total_received_surface_m,
            )?,
            total_received_lateral_m: nonnegative(
                "total_received_lateral_m",
                &self.total_received_lateral_m,
            )?,
            total_net_transfer_m: finite("total_net_transfer_m", &self.total_net_transfer_m)?,
        })
    }
}
