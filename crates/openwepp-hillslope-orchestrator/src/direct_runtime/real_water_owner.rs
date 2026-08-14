//! Production direct-hydrology withdrawal authorization used by shadow owners.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    ResourceAmountBasis, ResourceOwnerId, ResourceRequest, TransactionId,
    authorize_proportionally_by,
};

use super::{DirectDayFrame, DirectRuntimeError};

const WATER_DENSITY_KG_M3: f64 = 1_000.0;

/// One canonically ranked withdrawal against a production day-frame layer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DirectLayerWithdrawalRequest {
    pub lane_index: usize,
    pub layer_index: usize,
    pub canonical_rank: usize,
    pub amount_kg_m2: f64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DirectLayerWithdrawalKey {
    lane_index: usize,
    layer_index: usize,
    canonical_rank: usize,
}

/// Authorize same-snapshot withdrawals from the freshly seeded production
/// day frames. Returned amounts retain input order; callers retain the richer
/// requester/resource identity outside this dependency-neutral owner kernel.
pub(crate) fn authorize_direct_layer_withdrawals(
    day_frames: &[DirectDayFrame],
    requests: &[DirectLayerWithdrawalRequest],
) -> Result<Vec<f64>, DirectRuntimeError> {
    let mut ranks = BTreeSet::new();
    let mut available = BTreeMap::new();
    for day_frame in day_frames {
        for (layer_index, layer) in day_frame.percolation_inputs.layers.iter().enumerate() {
            available.insert(
                (day_frame.lane_index, layer_index),
                layer.theta_m * WATER_DENSITY_KG_M3,
            );
        }
    }
    let owner_id = ResourceOwnerId::try_new("direct-hydrology-shadow-request").map_err(|_| {
        DirectRuntimeError::DirectDomainViolation {
            field: "real_water_owner.owner_identity",
        }
    })?;
    let mut canonical_requests = Vec::with_capacity(requests.len());
    for request in requests {
        if !request.amount_kg_m2.is_finite() || request.amount_kg_m2 < 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "real_water_owner.request_kg_m2",
            });
        }
        if !ranks.insert(request.canonical_rank) {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "real_water_owner.canonical_rank",
            });
        }
        let day_frame =
            day_frames
                .get(request.lane_index)
                .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index: request.lane_index,
                    lane_count: day_frames.len(),
                })?;
        if day_frame.lane_index != request.lane_index
            || day_frame.identity.lane_count != day_frames.len()
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "real_water_owner.lane_identity",
            });
        }
        if request.layer_index >= day_frame.percolation_inputs.layers.len() {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "real_water_owner.layer_index",
            });
        }
        canonical_requests.push(ResourceRequest {
            transaction_id: TransactionId(0),
            owner_id: owner_id.clone(),
            key: DirectLayerWithdrawalKey {
                lane_index: request.lane_index,
                layer_index: request.layer_index,
                canonical_rank: request.canonical_rank,
            },
            amount: request.amount_kg_m2,
            basis: ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
        });
    }
    authorize_proportionally_by(
        &canonical_requests,
        &available,
        ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
        |key| (key.lane_index, key.layer_index),
    )
    .map(|authorizations| {
        authorizations
            .into_iter()
            .map(|authorization| authorization.amount)
            .collect()
    })
    .map_err(|_| DirectRuntimeError::DirectDomainViolation {
        field: "real_water_owner.protocol",
    })
}
