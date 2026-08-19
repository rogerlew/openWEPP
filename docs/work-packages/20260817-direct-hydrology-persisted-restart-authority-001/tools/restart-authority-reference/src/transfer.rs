use openwepp_hillslope_orchestrator::{
    DirectLaneTransferLedger, DirectRunTransferDownstreamOperands, DirectTransferBuffers,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{HexF64, LaneCount, WireLaneId, WirePrimitiveError};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransferRestartError {
    #[error("{field} must be finite")]
    NonFinite { field: &'static str },
    #[error("{field} must be finite and nonnegative")]
    Nonnegative { field: &'static str },
    #[error("lane count does not fit the runtime usize")]
    LaneCountWidth,
    #[error(transparent)]
    Wire(#[from] WirePrimitiveError),
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
    pub lane_id: WireLaneId,
    pub upstream_lane_id: WireLaneId,
    pub downstream_lane_id: WireLaneId,
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
            lane_id: WireLaneId(lane_id),
            upstream_lane_id: WireLaneId(upstream_lane_id),
            downstream_lane_id: WireLaneId(downstream_lane_id),
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
            lane_id: self.lane_id.0,
            upstream_lane_id: self.upstream_lane_id.0,
            downstream_lane_id: self.downstream_lane_id.0,
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
    pub lane_count: LaneCount,
    pub outlet_lane_id: WireLaneId,
    pub total_outgoing_surface_m: HexF64,
    pub total_outgoing_lateral_m: HexF64,
    pub total_received_surface_m: HexF64,
    pub total_received_lateral_m: HexF64,
    pub total_net_transfer_m: HexF64,
}
impl DirectRunTransferDownstreamOperandsRestartV1 {
    pub fn project(
        value: &DirectRunTransferDownstreamOperands,
    ) -> Result<Self, TransferRestartError> {
        let DirectRunTransferDownstreamOperands {
            lane_count,
            outlet_lane_id,
            total_outgoing_surface_m,
            total_outgoing_lateral_m,
            total_received_surface_m,
            total_received_lateral_m,
            total_net_transfer_m,
        } = *value;
        let lane_count =
            u32::try_from(lane_count).map_err(|_| TransferRestartError::LaneCountWidth)?;
        Ok(Self {
            lane_count: LaneCount::try_new(lane_count)?,
            outlet_lane_id: WireLaneId(outlet_lane_id),
            total_outgoing_surface_m: HexF64::from_f64(total_outgoing_surface_m),
            total_outgoing_lateral_m: HexF64::from_f64(total_outgoing_lateral_m),
            total_received_surface_m: HexF64::from_f64(total_received_surface_m),
            total_received_lateral_m: HexF64::from_f64(total_received_lateral_m),
            total_net_transfer_m: HexF64::from_f64(total_net_transfer_m),
        })
    }
    pub fn restore(&self) -> Result<DirectRunTransferDownstreamOperands, TransferRestartError> {
        Ok(DirectRunTransferDownstreamOperands {
            lane_count: self.lane_count.get() as usize,
            outlet_lane_id: self.outlet_lane_id.0,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_transfer_types_round_trip_every_field_bit_exactly() {
        let buffers = DirectTransferBuffers {
            surface_carry_m: std::array::from_fn(|i| i as f64 / 1000.0),
            surface_hourly_weights: [1.0 / 24.0; 24],
            lateral_carry_m: std::array::from_fn(|i| (23 - i) as f64 / 2000.0),
            upstream_flow_m: -0.0,
            subsurface_input_m: 0.04,
        };
        let dto = DirectTransferBuffersRestartV1::project(&buffers);
        assert_eq!(
            DirectTransferBuffersRestartV1::project(&dto.restore().expect("valid buffers")),
            dto
        );
        let ledger = DirectLaneTransferLedger {
            lane_id: 2,
            upstream_lane_id: 1,
            downstream_lane_id: 3,
            upstream_area_ratio: 0.75,
            area_m2: 120.0,
            outgoing_surface_m: 0.1,
            outgoing_lateral_m: 0.2,
            received_surface_m: -0.0,
            received_lateral_m: 0.3,
            net_transfer_m: -0.1,
        };
        let dto = DirectLaneTransferLedgerRestartV1::project(&ledger);
        assert_eq!(
            DirectLaneTransferLedgerRestartV1::project(&dto.restore().expect("valid ledger")),
            dto
        );
        let run = DirectRunTransferDownstreamOperands {
            lane_count: 3,
            outlet_lane_id: 3,
            total_outgoing_surface_m: 0.1,
            total_outgoing_lateral_m: 0.2,
            total_received_surface_m: -0.0,
            total_received_lateral_m: 0.3,
            total_net_transfer_m: -0.1,
        };
        let dto =
            DirectRunTransferDownstreamOperandsRestartV1::project(&run).expect("valid projection");
        assert_eq!(
            DirectRunTransferDownstreamOperandsRestartV1::project(
                &dto.restore().expect("valid run")
            )
            .expect("valid projection"),
            dto
        );
    }
    #[test]
    fn transfer_domain_and_lane_count_poisons_reject() {
        let mut buffers = DirectTransferBuffersRestartV1::project(&DirectTransferBuffers::zero());
        buffers.surface_carry_m[7] = HexF64::from_f64(-0.1);
        assert_eq!(
            buffers.restore(),
            Err(TransferRestartError::Nonnegative {
                field: "surface_carry_m"
            })
        );
        let zero = DirectRunTransferDownstreamOperands::zero();
        assert_eq!(
            DirectRunTransferDownstreamOperandsRestartV1::project(&zero),
            Err(TransferRestartError::Wire(WirePrimitiveError::ZeroCount {
                kind: "lane"
            }))
        );
    }
}
